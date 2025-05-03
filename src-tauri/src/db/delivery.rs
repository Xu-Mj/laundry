use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::types::chrono::{DateTime, FixedOffset};
use sqlx::{FromRow, QueryBuilder, Row, Sqlite, Transaction};
use tauri::State;

use crate::db::{Curd, PageParams, PageResult, Validator};
use crate::error::{Error, Result};
use crate::state::AppState;
use crate::utils;
use crate::utils::request::Request;

use super::order_clothes::OrderCloth;
use super::user::User;

// Delivery status constants
const DELIVERY_STATUS_PENDING: &str = "00"; // 待派送
// const DELIVERY_STATUS_DELIVERING: &str = "01"; // 派送中
const DELIVERY_STATUS_COMPLETED: &str = "02"; // 已完成
const DELIVERY_STATUS_CANCELED: &str = "03"; // 已取消

// Cloth status constants for delivery
const CLOTH_STATUS_INITIAL: &str = "01"; // 初始状态（洗护中）
const CLOTH_STATUS_WASHED: &str = "02"; // 已洗完
const CLOTH_STATUS_DELIVERED: &str = "03"; // 已派送

// Notice template ID for delivery
// const DELIVERY_NOTICE_TEMPLATE_ID: i64 = 2; // This should match your actual template ID

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Delivery {
    /// 派送ID
    pub delivery_id: Option<i64>,
    /// 所属商店ID
    pub store_id: Option<i64>,
    /// 用户ID
    pub user_id: Option<i64>,
    /// 订单ID (可能包含多个，用逗号分隔)
    pub order_id: Option<String>,
    /// 衣物ID (可能包含多个，用逗号分隔)
    pub cloth_id: Option<String>,
    /// 派送地址
    pub address: Option<String>,
    /// 派送时间
    pub dispatch_time: Option<DateTime<FixedOffset>>,
    /// 完成时间
    pub complete_time: Option<DateTime<FixedOffset>>,
    /// 备注
    pub remark: Option<String>,
    /// 派送状态（00待派送 01派送中 02已完成 03已取消）
    pub delivery_status: Option<String>,
    /// 创建时间
    pub create_time: Option<DateTime<FixedOffset>>,
    /// 更新时间
    pub update_time: Option<DateTime<FixedOffset>>,

    pub need_sync: bool,
}

impl FromRow<'_, SqliteRow> for Delivery {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Delivery {
            delivery_id: row.try_get("delivery_id").unwrap_or_default(),
            store_id: row.try_get("store_id").unwrap_or_default(),
            user_id: row.try_get("user_id").unwrap_or_default(),
            order_id: row.try_get("order_id").unwrap_or_default(),
            cloth_id: row.try_get("cloth_id").unwrap_or_default(),
            address: row.try_get("address").unwrap_or_default(),
            dispatch_time: row.try_get("dispatch_time").unwrap_or_default(),
            complete_time: row.try_get("complete_time").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
            delivery_status: row.try_get("delivery_status").unwrap_or_default(),
            create_time: row.try_get("create_time").unwrap_or_default(),
            update_time: row.try_get("update_time").unwrap_or_default(),
            need_sync: false,
        })
    }
}

// Validation for Delivery struct
impl Validator for Delivery {
    fn validate(&self) -> Result<()> {
        if self.user_id.is_none() {
            return Err(Error::bad_request("用户ID不能为空"));
        }

        if self.cloth_id.is_none() || self.cloth_id.as_ref().unwrap().is_empty() {
            return Err(Error::bad_request("衣物ID不能为空"));
        }

        if self.address.is_none() || self.address.as_ref().unwrap().is_empty() {
            return Err(Error::bad_request("派送地址不能为空"));
        }

        if self.dispatch_time.is_none() {
            return Err(Error::bad_request("派送时间不能为空"));
        }

        Ok(())
    }
}

const QUERY_SQL: &str = "SELECT d.* FROM deliveries d WHERE 1=1";
const BY_ID_SQL: &str = "SELECT * FROM deliveries WHERE delivery_id = ?";
const COUNT_SQL: &str = "SELECT COUNT(*) FROM deliveries d WHERE 1=1";
const DELETE_BATCH_SQL: &str = "DELETE FROM deliveries WHERE delivery_id IN (";

impl Curd for Delivery {
    const COUNT_SQL: &'static str = COUNT_SQL;
    const QUERY_SQL: &'static str = QUERY_SQL;
    const BY_ID_SQL: &'static str = BY_ID_SQL;
    const DELETE_BATCH_SQL: &'static str = DELETE_BATCH_SQL;
    const ORDER_SQL: Option<&'static str> = Some(" ORDER BY create_time DESC");

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, Sqlite>) {
        if let Some(store_id) = self.store_id {
            builder.push(" AND d.store_id = ").push_bind(store_id);
        }

        if let Some(user_id) = self.user_id {
            builder.push(" AND d.user_id = ").push_bind(user_id);
        }

        if let Some(status) = &self.delivery_status {
            if !status.is_empty() {
                builder.push(" AND d.delivery_status = ").push_bind(status);
            }
        }
    }
}

impl Delivery {
    pub async fn create(&self, tx: &mut Transaction<'_, Sqlite>) -> Result<Self> {
        let now = utils::get_now();
        let delivery = sqlx::query_as::<_, Self>(
            "INSERT INTO deliveries (
                store_id, user_id, order_id, cloth_id, address,
                dispatch_time, remark, delivery_status, create_time, update_time
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *",
        )
        .bind(&self.store_id)
        .bind(&self.user_id)
        .bind(&self.order_id)
        .bind(&self.cloth_id)
        .bind(&self.address)
        .bind(&self.dispatch_time)
        .bind(&self.remark)
        .bind(DELIVERY_STATUS_PENDING) // Initial status is always pending
        .bind(now)
        .bind(now)
        .fetch_one(&mut **tx)
        .await?;

        Ok(delivery)
    }

    pub async fn update_status(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        status: &str,
    ) -> Result<bool> {
        let now = utils::get_now();
        let mut query_builder = QueryBuilder::new("UPDATE deliveries SET delivery_status = ");
        query_builder.push_bind(status);
        query_builder.push(", update_time = ");
        query_builder.push_bind(now);

        // If status is completed, set complete_time
        if status == DELIVERY_STATUS_COMPLETED {
            query_builder.push(", complete_time = ");
            query_builder.push_bind(now);
        }

        query_builder.push(" WHERE delivery_id = ");
        query_builder.push_bind(self.delivery_id);

        let result = query_builder.build().execute(&mut **tx).await?;
        Ok(result.rows_affected() > 0)
    }

    // Send notification to user about delivery
    // async fn send_delivery_notification(
    //     &self,
    //     tx: &mut Transaction<'_, Sqlite>,
    //     state: &State<'_, AppState>,
    // ) -> Result<()> {
    //     if let (Some(user_id), Some(dispatch_time)) = (self.user_id, self.dispatch_time) {
    //         // Get user phone number
    //         let user = User::get_by_id(&state.pool, user_id).await?
    //             .ok_or_else(|| Error::not_found("User not found"))?;

    //         if let Some(phone) = user.phonenumber {
    //             if !phone.is_empty() {
    //                 // Format the dispatch date for the message
    //                 let dispatch_date = dispatch_time.format("%Y-%m-%d").to_string();

    //                 // Create args for notification template
    //                 let mut args = std::collections::HashMap::new();
    //                 args.insert("date".to_string(), dispatch_date);
    //                 args.insert("address".to_string(), self.address.clone().unwrap_or_default());

    //                 // Create and save notification record
    //                 let notice = NoticeRecord {
    //                     store_id: self.store_id,
    //                     temp_id: Some(DELIVERY_NOTICE_TEMPLATE_ID),
    //                     send_to: Some(phone),
    //                     args: Some(serde_json::to_string(&args).unwrap_or_default()),
    //                     ..Default::default()
    //                 };

    //                 // Insert notification record
    //                 sqlx::query(
    //                     "INSERT INTO notice_records
    //                     (store_id, temp_id, send_to, status, args, create_time)
    //                     VALUES (?, ?, ?, ?, ?, ?)"
    //                 )
    //                 .bind(notice.store_id)
    //                 .bind(notice.temp_id)
    //                 .bind(notice.send_to)
    //                 .bind("0") // Pending status
    //                 .bind(notice.args)
    //                 .bind(utils::get_now())
    //                 .execute(&mut **tx)
    //                 .await?;
    //             }
    //         }
    //     }

    //     Ok(())
    // }

    // Process a delivery request from start to finish
    pub async fn process_delivery(&mut self, state: &State<'_, AppState>) -> Result<Self> {
        // Validate the delivery request
        self.validate()?;

        // Start transaction
        let mut tx = state.pool.begin().await?;

        // Set store ID if not provided
        if self.store_id.is_none() {
            self.store_id = Some(utils::get_user_id(state).await?);
        }
        // todo send delivery data to server
        let delivery = self.create_request(&state).await?;
        // Create delivery record
        let delivery = delivery.create(&mut tx).await?;

        let cloth_ids = delivery
            .cloth_id
            .clone()
            .ok_or(Error::bad_request("衣物ID不能为空"))?;
        let ids: Vec<i64> = cloth_ids
            .split(',')
            .filter_map(|id| id.parse::<i64>().ok())
            .collect();

        if ids.is_empty() {
            return Err(Error::bad_request("衣物列表不能为空"));
        }

        // Verify the status of all clothes to make sure they're either washing or washed
        let clothes = OrderCloth::get_by_ids(&state.pool, &ids).await?;

        for cloth in &clothes {
            let status = cloth.clothing_status.as_deref().unwrap_or("");
            if status != CLOTH_STATUS_INITIAL && status != CLOTH_STATUS_WASHED {
                return Err(Error::bad_request(format!(
                    "衣物 {} 状态不符合派送条件，只能派送正在洗护中和已洗完的衣物",
                    cloth.cloth_id.unwrap_or(0)
                )));
            }
        }

        // Update clothes status
        if !OrderCloth::update_cloth_status_delivery(&mut tx, &ids, CLOTH_STATUS_DELIVERED).await? {
            return Err(Error::internal("衣物状态更新失败"));
        }

        // update user address if needed
        if self.need_sync {
            User::update_address(
                &mut tx,
                delivery.store_id.unwrap(),
                delivery.user_id.unwrap(),
                delivery.address.as_ref().unwrap(),
            )
            .await?;
        }

        // Commit transaction
        tx.commit().await?;

        Ok(delivery)
    }

    // Complete a delivery
    pub async fn complete_delivery(state: &State<'_, AppState>, delivery_id: i64) -> Result<bool> {
        let mut tx = state.pool.begin().await?;

        // Get delivery by ID
        let delivery = Self::get_by_id(&state.pool, delivery_id)
            .await?
            .ok_or(Error::not_found("Delivery not found"))?;

        // Update status to completed
        let success = delivery
            .update_status(&mut tx, DELIVERY_STATUS_COMPLETED)
            .await?;

        let cloth_ids = delivery
            .cloth_id
            .clone()
            .ok_or(Error::bad_request("衣物ID不能为空"))?;
        let ids: Vec<i64> = cloth_ids
            .split(',')
            .filter_map(|id| id.parse::<i64>().ok())
            .collect();

        if ids.is_empty() {
            return Err(Error::bad_request("衣物列表不能为空"));
        }
        if !OrderCloth::update_cloth_status_delivery(&mut tx, &ids, DELIVERY_STATUS_COMPLETED)
            .await?
        {
            return Err(Error::internal("衣物状态更新失败"));
        }

        // sync delivery data to server

        // Commit transaction
        tx.commit().await?;

        Ok(success)
    }

    // Cancel a delivery
    pub async fn cancel_delivery(state: &State<'_, AppState>, delivery_id: i64) -> Result<bool> {
        let mut tx = state.pool.begin().await?;

        // Get delivery by ID
        let delivery = Self::get_by_id(&state.pool, delivery_id)
            .await?
            .ok_or_else(|| Error::not_found("Delivery not found"))?;

        // Update status to canceled
        let success = delivery
            .update_status(&mut tx, DELIVERY_STATUS_CANCELED)
            .await?;

        // Revert clothes status (optional, depends on your business logic)
        // Here we would implement code to restore the previous status of clothes

        // Commit transaction
        tx.commit().await?;

        Ok(success)
    }
}

impl Request for Delivery {
    const URL: &'static str = "/delivery";
}

// Tauri commands
#[tauri::command]
pub async fn create_delivery(
    state: State<'_, AppState>,
    mut delivery: Delivery,
) -> Result<Delivery> {
    let store_id = utils::get_user_id(&state).await?;
    delivery.store_id = Some(store_id);
    delivery.process_delivery(&state).await
}

#[tauri::command]
pub async fn complete_delivery(state: State<'_, AppState>, delivery_id: i64) -> Result<bool> {
    Delivery::complete_delivery(&state, delivery_id).await
}

#[tauri::command]
pub async fn cancel_delivery(state: State<'_, AppState>, delivery_id: i64) -> Result<bool> {
    Delivery::cancel_delivery(&state, delivery_id).await
}

#[tauri::command]
pub async fn get_delivery_by_id(
    state: State<'_, AppState>,
    delivery_id: i64,
) -> Result<Option<Delivery>> {
    Delivery::get_by_id(&state.pool, delivery_id).await
}

#[tauri::command]
pub async fn list_deliveries(
    state: State<'_, AppState>,
    page_params: PageParams,
    mut delivery: Delivery,
) -> Result<PageResult<Delivery>> {
    let store_id = utils::get_user_id(&state).await?;
    delivery.store_id = Some(store_id);
    delivery.get_list(&state.pool, page_params).await
}
