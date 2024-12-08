use crate::db::orders::Order;
use crate::db::user::User;
use crate::db::{AppState, Curd, PageParams, PageResult};
use crate::error::{Error, Result};
use crate::utils;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::types::chrono::{DateTime, FixedOffset};
use sqlx::{FromRow, Pool, QueryBuilder, Row, Sqlite, Transaction};
use tauri::State;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct NoticeTemp {
    pub temp_id: Option<i64>,                       // 自增主键，可以为 None
    pub temp_name: Option<String>,                  // 模板名称
    pub notice_method: Option<String>,              // 通知方式
    pub content: Option<String>,                    // 通知内容
    pub create_time: Option<DateTime<FixedOffset>>, // 创建时间，可以为 None
    pub temp_type: Option<String>,                  // 模板类型
    pub remark: Option<String>,                     // 备注，可以为 None
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct NoticeRecord {
    pub notice_id: Option<i64>,                     // 自增主键，可以为 None
    pub user_id: i64,                               // 用户 ID
    pub order_number: Option<String>,               // 订单号，可以为 None
    pub username: Option<String>,                   // 订单号，可以为 None
    pub phonenumber: Option<String>,                // 订单号，可以为 None
    pub notice_method: Option<String>,              // 通知方式
    pub notice_type: Option<String>,                // 通知类型
    pub notice_time: Option<DateTime<FixedOffset>>, // 通知时间，可以为 None
    pub title: Option<String>,                      // 通知标题
    pub content: Option<String>,                    // 通知内容
    pub result: Option<String>,                     // 通知结果，可以为 None
    pub remark: Option<String>,                     // 备注，可以为 None
}

impl FromRow<'_, SqliteRow> for NoticeTemp {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            temp_id: row.try_get("temp_id").unwrap_or_default(),
            temp_name: row.try_get("temp_name").unwrap_or_default(),
            notice_method: row.try_get("notice_method").unwrap_or_default(),
            content: row.try_get("content").unwrap_or_default(),
            create_time: row.try_get("create_time").unwrap_or_default(),
            temp_type: row.try_get("temp_type").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
        })
    }
}

impl FromRow<'_, SqliteRow> for NoticeRecord {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            notice_id: row.try_get("notice_id").unwrap_or_default(),
            user_id: row.try_get("user_id").unwrap_or_default(),
            order_number: row.try_get("order_number").unwrap_or_default(),
            username: row.try_get("username").unwrap_or_default(),
            phonenumber: row.try_get("phonenumber").unwrap_or_default(),
            notice_method: row.try_get("notice_method").unwrap_or_default(),
            notice_type: row.try_get("notice_type").unwrap_or_default(),
            notice_time: row.try_get("notice_time").unwrap_or_default(),
            title: row.try_get("title").unwrap_or_default(),
            content: row.try_get("content").unwrap_or_default(),
            result: row.try_get("result").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
        })
    }
}

impl Curd for NoticeTemp {
    const COUNT_SQL: &'static str = "SELECT COUNT(1) FROM notice_temp WHERE 1=1";
    const QUERY_SQL: &'static str = "SELECT * FROM notice_temp WHERE 1=1 ";
    const BY_ID_SQL: &'static str = "SELECT * FROM notice_temp WHERE temp_id = ?";
    const DELETE_BATCH_SQL: &'static str = "DELETE FROM notice_temp WHERE temp_id IN ( ";

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, Sqlite>) {
        if let Some(temp_name) = &self.temp_name {
            builder
                .push(" AND temp_name LIKE ")
                .push_bind(format!("%{}%", temp_name));
        }

        if let Some(notice_method) = &self.notice_method {
            builder
                .push(" AND notice_method = ")
                .push_bind(notice_method);
        }

        if let Some(temp_type) = &self.temp_type {
            builder.push(" AND temp_type = ").push_bind(temp_type);
        }
    }
}

const QUERY_RECORD_SQL: &str = "
        SELECT n.notice_id,
               n.user_id,
               n.order_number,
               n.notice_method,
               n.notice_type,
               n.notice_time,
               n.title,
               n.content,
               n.result,
               n.remark,
               u.nick_name,
               u.user_name,
               u.phonenumber FROM notice_record n
        LEFT JOIN users u on n.user_id = u.user_id ";

impl Curd for NoticeRecord {
    const COUNT_SQL: &'static str = "SELECT COUNT(1) FROM notice_record WHERE 1=1";
    const QUERY_SQL: &'static str = QUERY_RECORD_SQL;
    const BY_ID_SQL: &'static str = "SELECT n.notice_id,
               n.user_id,
               n.order_number,
               n.notice_method,
               n.notice_type,
               n.notice_time,
               n.title,
               n.content,
               n.result,
               n.remark,
               u.nick_name,
               u.user_name,
               u.phonenumber FROM notice_record n
        LEFT JOIN users u on n.user_id = u.user_idWHERE notice_id = ?";
    const DELETE_BATCH_SQL: &'static str = "DELETE FROM notice_record WHERE notice_id IN ( ";
    const ORDER_SQL: Option<&'static str> = Some("ORDER BY notice_time DESC");

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, Sqlite>) {
        if let Some(ref order_number) = self.order_number {
            builder.push(" AND order_number = ").push_bind(order_number);
        }

        if let Some(ref notice_method) = self.notice_method {
            builder
                .push(" AND notice_method = ")
                .push_bind(notice_method);
        }

        if let Some(ref notice_type) = self.notice_type {
            builder.push(" AND notice_type = ").push_bind(notice_type);
        }

        if let Some(ref result) = self.result {
            builder.push(" AND result = ").push_bind(result);
        }

        if let Some(ref username) = self.username {
            builder
                .push(" AND u.user_name LIKE ")
                .push_bind(format!("%{username}%"));
        }

        if let Some(phonenumber) = &self.phonenumber {
            builder
                .push(" AND u.phonenumber LIKE ")
                .push_bind(format!("%{phonenumber}%"));
        }
    }
}

impl NoticeTemp {
    // insert
    pub async fn create(self, pool: &Pool<Sqlite>) -> Result<Self> {
        let result = sqlx::query_as("INSERT INTO notice_temp (temp_name, notice_method, content, temp_type, remark) VALUES (?, ?, ?, ?, ?)")
            .bind(self.temp_name)
            .bind(self.notice_method)
            .bind(self.content)
            .bind(self.temp_type)
            .bind(self.remark)
            .fetch_one(pool)
            .await?;
        Ok(result)
    }

    // update
    pub async fn update(self, pool: &Pool<Sqlite>) -> Result<bool> {
        let result = sqlx::query("UPDATE notice_temp SET temp_name = ?, notice_method = ?, content = ?, temp_type = ?, remark = ? WHERE temp_id = ?")
            .bind(self.temp_name)
            .bind(self.notice_method)
            .bind(self.content)
            .bind(self.temp_type)
            .bind(self.remark)
            .bind(self.temp_id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

impl NoticeRecord {
    // insert
    pub async fn create(self, pool: &Pool<Sqlite>) -> Result<Self> {
        let result = sqlx::query_as("INSERT INTO notice_record (user_id, order_number, notice_method, notice_type, notice_time, title, content, result, remark) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(self.user_id)
            .bind(self.order_number)
            .bind(self.notice_method)
            .bind(self.notice_type)
            .bind(self.notice_time)
            .bind(self.title)
            .bind(self.content)
            .bind(self.result)
            .bind(self.remark)
            .fetch_one(pool)
            .await?;
        Ok(result)
    }

    // 批量插入
    pub async fn insert_batch(
        tx: &mut Transaction<'_, Sqlite>,
        records: &[NoticeRecord],
    ) -> Result<u64> {
        if records.is_empty() {
            return Ok(0);
        }

        // Initialize the QueryBuilder
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            "INSERT INTO notice_record (user_id, order_number, notice_method, notice_type, notice_time, title, content, result, remark) ",
        );

        let now = utils::get_now();
        // Begin values clause
        query_builder.push_values(records.into_iter(), |mut builder, record| {
            builder
                .push_bind(&record.user_id) // Bind user_id
                .push_bind(&record.order_number) // Bind order_number
                .push_bind(&record.notice_method) // Bind notice_method
                .push_bind(&record.notice_type) // Bind notice_type
                .push_bind(&now) // Bind current timestamp
                .push_bind(&record.title) // Bind title
                .push_bind(&record.content) // Bind content
                .push_bind(&record.result) // Bind result
                .push_bind(&record.remark); // Bind remark
        });

        // Execute the query
        let rows_affected = query_builder
            .build()
            .execute(&mut **tx)
            .await?
            .rows_affected();

        Ok(rows_affected)
    }

    // update
    pub async fn update(self, pool: &Pool<Sqlite>) -> Result<bool> {
        let result = sqlx::query("UPDATE notice_record SET user_id = ?, order_number = ?, notice_method = ?, notice_type = ?, notice_time = ?, title = ?, content = ?, result = ?, remark = ? WHERE notice_id = ?")
            .bind(self.user_id)
            .bind(self.order_number)
            .bind(self.notice_method)
            .bind(self.notice_type)
            .bind(self.notice_time)
            .bind(self.title)
            .bind(self.content)
            .bind(self.result)
            .bind(self.remark)
            .bind(self.notice_id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // delete all
    pub async fn delete_all(pool: &Pool<Sqlite>) -> Result<u64> {
        let result = sqlx::query("DELETE FROM notice_record")
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_old_records(pool: &Pool<Sqlite>, days: i64) -> Result<u64> {
        let rows_affected = sqlx::query(
            "DELETE FROM notice_record
         WHERE notice_time < datetime('now', '-' || ? || ' days')",
        )
        .bind(days) // Bind the number of days
        .execute(pool)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }
}

impl NoticeRecord {
    fn from_temp_with_user_id(temp: NoticeTemp, user_id: i64, order_num: Option<String>) -> Self {
        NoticeRecord {
            user_id,
            order_number: order_num,
            notice_method: temp.notice_method,
            notice_type: temp.temp_type,
            notice_time: Some(utils::get_now()),
            title: temp.temp_name,
            content: temp.content,
            result: Some("2".to_string()),
            ..Default::default()
        }
    }
}

const TEMP_TYPE_PICKUP: &str = "0";
const TEMP_TYPE_AD: &str = "1";
const TEMP_TYPE_OTHER: &str = "2";

impl NoticeTemp {
    pub async fn send_notice(pool: &Pool<Sqlite>, temp_id: i64, user_ids: &[i64]) -> Result<u64> {
        let mut tr = pool.begin().await?;
        // select temp info by temp_id
        let mut temp = Self::get_by_id(pool, temp_id)
            .await?
            .ok_or(Error::not_found("temp not found"))?;

        // select users by user ids
        let users = User::list_by_ids(pool, user_ids).await?;

        let mut records = Vec::with_capacity(users.len());

        let content = temp.content.clone().unwrap_or_default();

        // pickup notice only remain
        // select orders that wait to pickup by user ids
        let orders = Order::select_list_with_wait_to_pick_with_user_ids(pool, user_ids).await?;

        // generate notice records
        orders.into_iter().for_each(|order| {
            // get user by user_id in order
            let user = users
                .iter()
                .filter(|user| user.user_id == order.user_id)
                .next();
            if user.is_none() {
                return;
            }
            let user = user.unwrap();

            // replace variant in template
            temp.content = Some(Self::create_content(&content, user, &order));

            // generate notice record
            let record = NoticeRecord::from_temp_with_user_id(
                temp.clone(),
                user.user_id.unwrap_or_default(),
                order.order_number,
            );
            records.push(record);
        });

        NoticeRecord::insert_batch(&mut tr, &records).await?;

        // todo send notice
        tr.commit().await?;
        Ok(0)
    }

    fn create_content(template: &str, user: &User, order: &Order) -> String {
        let mut result = template.to_string();
        let regex = Regex::new(r"《(.*?)》").unwrap();

        for caps in regex.captures_iter(template) {
            match &caps[1] {
                "会员姓名" => {
                    let tel = user.phonenumber.clone().unwrap_or_default();
                    let phone_tail = &tel[7..];
                    result = result.replace(
                        &format!("《{}》", &caps[1]),
                        &format!(
                            "{}{}",
                            user.nick_name.clone().unwrap_or_default(),
                            phone_tail
                        ),
                    );
                }
                "取件码" => {
                    result = result.replace(
                        &format!("《{}》", &caps[1]),
                        &order.pickup_code.clone().unwrap_or_default(),
                    );
                }
                _ => {}
            }
        }
        result
    }
}

#[tauri::command]
pub async fn get_temp_pagination(
    state: State<'_, AppState>,
    page_params: PageParams,
    temp: NoticeTemp,
) -> Result<PageResult<NoticeTemp>> {
    temp.get_list(&state.0, page_params).await
}

#[tauri::command]
pub async fn get_temp_by_id(state: State<'_, AppState>, id: i64) -> Result<Option<NoticeTemp>> {
    NoticeTemp::get_by_id(&state.0, id).await
}

#[tauri::command]
pub async fn create_temp(state: State<'_, AppState>, temp: NoticeTemp) -> Result<NoticeTemp> {
    temp.create(&state.0).await
}

#[tauri::command]
pub async fn update_temp(state: State<'_, AppState>, temp: NoticeTemp) -> Result<bool> {
    temp.update(&state.0).await
}

#[tauri::command]
pub async fn delete_temp(state: State<'_, AppState>, ids: Vec<i64>) -> Result<bool> {
    if ids.contains(&1) {
        return Err(Error::bad_request("不允许删除取衣通知模板！"));
    }
    let mut tx = state.0.begin().await?;
    let result = NoticeTemp::delete_batch(&mut tx, &ids).await?;
    tx.commit().await?;
    Ok(result)
}

#[tauri::command]
pub async fn get_notice_record_pagination(
    state: State<'_, AppState>,
    page_params: PageParams,
    record: NoticeRecord,
) -> Result<PageResult<NoticeRecord>> {
    record.get_list(&state.0, page_params).await
}

#[tauri::command]
pub async fn delete_all_record(state: State<'_, AppState>) -> Result<bool> {
    let count = NoticeRecord::delete_all(&state.0).await?;
    Ok(count > 0)
}

#[tauri::command]
pub async fn delete_old_record(state: State<'_, AppState>, days: i64) -> Result<bool> {
    let count = NoticeRecord::delete_old_records(&state.0, days).await?;
    Ok(count > 0)
}

#[tauri::command]
pub async fn send_notice(
    state: State<'_, AppState>,
    temp_id: i64,
    user_ids: Vec<i64>,
) -> Result<bool> {
    let count = NoticeTemp::send_notice(&state.0, temp_id, &user_ids).await?;
    Ok(count == user_ids.len() as u64)
}
