use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::types::chrono::{DateTime, FixedOffset, Local};
use sqlx::{FromRow, Pool, QueryBuilder, Row, Sqlite, Transaction};
use std::collections::HashMap;
use tauri::State;

use crate::db::cloth_sequence::ClothSequence;
use crate::db::clothing::Clothing;
use crate::db::drying_rack::DryingRack;
use crate::db::notice_temp::NoticeRecord;
use crate::db::order_pictures::OrderPicture;
use crate::db::orders::Order;
use crate::db::tags::Tag;
use crate::db::{PageParams, PageResult, Validator};
use crate::error::{Error, ErrorKind, Result};
use crate::state::AppState;
use crate::utils;

const CLOTH_STATUS_HANGED: &str = "02";
const CLOTH_STATUS_PICKED: &str = "00";
// const CLOTH_STATUS_WASHED: &str = "02";
const CLOTH_STATUS_DELIVERED: &str = "03";
const ORDER_STATUS_COMPLETED: &str = "04";
const ORDER_STATUS_PAID: &str = "00";
// const CLOTH_STATUS_DELIVERY: &str = "01";
// const CLOTH_STATUS_EXPRESS: &str = "02";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OrderCloth {
    pub cloth_id: Option<i64>,
    pub store_id: Option<i64>,
    pub order_id: Option<i64>,
    pub clothing_id: Option<i64>,
    pub clothing_category: Option<String>,
    pub category_id: Option<i64>,
    pub clothing_style: Option<String>,
    pub style_id: Option<i64>,
    pub clothing_color: Option<i64>,
    pub clothing_flaw: Option<String>,
    pub estimate: Option<String>,
    pub clothing_brand: Option<i64>,
    pub service_type: Option<String>,
    pub service_requirement: Option<String>,
    pub before_pics: Option<String>,
    pub after_pics: Option<String>,
    pub notes: Option<String>,
    pub process_markup: Option<f64>,
    pub price_value: Option<f64>,
    pub hang_type: Option<String>,
    pub hang_location_code: Option<i64>,
    pub hanger_number: Option<i32>,
    pub hanger_name: Option<String>,
    pub hang_cloth_code: Option<String>,
    pub hang_remark: Option<String>,
    pub create_time: Option<DateTime<FixedOffset>>,
    pub pickup_time: Option<DateTime<FixedOffset>>,
    pub pickup_method: Option<String>,
    pub clothing_status: Option<String>,
    pub remark: Option<String>,
    pub cloth_info: Option<Clothing>,
}

// todo check the remark attribute if necessary
impl FromRow<'_, SqliteRow> for OrderCloth {
    fn from_row(row: &SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        let clothing_id: Option<i64> = row.try_get("clothing_id").unwrap_or_default();
        let mut cloth_info = None;
        if clothing_id.is_some() {
            cloth_info = Some(Clothing::from_row(row)?);
        }
        Ok(OrderCloth {
            cloth_id: row.try_get("cloth_id").unwrap_or_default(),
            store_id: row.try_get("store_id").unwrap_or_default(),
            order_id: row.try_get("order_id").unwrap_or_default(),
            clothing_id: row.try_get("clothing_id").unwrap_or_default(),
            clothing_category: row.try_get("clothing_category").unwrap_or_default(),
            category_id: row.try_get("category_id").unwrap_or_default(),
            clothing_style: row.try_get("clothing_style").unwrap_or_default(),
            style_id: row.try_get("style_id").unwrap_or_default(),
            clothing_color: row.try_get("clothing_color").unwrap_or_default(),
            clothing_flaw: row.try_get("clothing_flaw").unwrap_or_default(),
            estimate: row.try_get("estimate").unwrap_or_default(),
            clothing_brand: row.try_get("clothing_brand").unwrap_or_default(),
            service_type: row.try_get("service_type").unwrap_or_default(),
            service_requirement: row.try_get("service_requirement").unwrap_or_default(),
            before_pics: row.try_get("before_pics").unwrap_or_default(),
            after_pics: row.try_get("after_pics").unwrap_or_default(),
            notes: row.try_get("notes").unwrap_or_default(),
            process_markup: row.try_get("process_markup").unwrap_or_default(),
            price_value: row.try_get("price_value").unwrap_or_default(),
            hang_type: row.try_get("hang_type").unwrap_or_default(),
            hang_location_code: row.try_get("hang_location_code").unwrap_or_default(),
            hanger_name: row.try_get("hanger_name").unwrap_or_default(),
            hanger_number: row.try_get("hanger_number").unwrap_or_default(),
            hang_cloth_code: row.try_get("hang_cloth_code").unwrap_or_default(),
            hang_remark: row.try_get("hang_remark").unwrap_or_default(),
            create_time: row.try_get("create_time").unwrap_or_default(),
            pickup_time: row.try_get("pickup_time").unwrap_or_default(),
            pickup_method: row.try_get("pickup_method").unwrap_or_default(),
            clothing_status: row.try_get("clothing_status").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
            cloth_info,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HangReq {
    pub cloth_id: i64,
    pub hang_location_id: i64,
    pub hanger_number: i32,
    pub hang_remark: Option<String>,
}

/// 所有的字段全部标记为Option，然后提供一个validate，在insert操作时调用。
impl Validator for OrderCloth {
    fn validate(&self) -> Result<()> {
        if self.clothing_id.is_none() {
            return Err(Error::bad_request("clothing_id is required"));
        }

        if self.category_id.is_none() {
            return Err(Error::bad_request("clothing_category is required"));
        }

        if self.style_id.is_none() {
            return Err(Error::bad_request("clothing_style is required"));
        }

        if self.service_type.is_none() {
            return Err(Error::bad_request("service_type is required"));
        }

        if self.hang_type.is_none() {
            return Err(Error::bad_request("hang_type is required"));
        }

        if self.price_value.is_none() {
            return Err(Error::bad_request("price_value is required"));
        }
        Ok(())
    }
}

const SQL: &str = "SELECT
               oc.*,
               c.id as clothing_id,
               c.clothing_number,
               c.title,
               c.clothing_base_price,
               c.clothing_min_price,
               c.order_num,
               c.clothing_degree,
               d.name as hanger_name,
               ct.category_name as clothing_category,
               st.style_name as clothing_style
        FROM order_clothes oc
                 left join clothing c on oc.clothing_id = c.id
                 left join drying_rack d on oc.hang_location_code = d.id
                 left join clothing_categories ct on c.category_id = ct.category_id 
                 left join clothing_styles st on c.style_id = st.style_id";

impl OrderCloth {
    pub async fn add(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<Self> {
        let cloth = sqlx::query_as(
            "INSERT INTO order_clothes
        (clothing_id, store_id, category_id, style_id, clothing_color,
         clothing_flaw, estimate, clothing_brand, service_type, service_requirement,
         before_pics, after_pics, notes, process_markup, price_value,
        hang_type, hang_location_code, hanger_number, hang_cloth_code, hang_remark,
        create_time, pickup_time, pickup_method, clothing_status, remark)
         VALUES
         (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
         RETURNING *",
        )
        .bind(&self.clothing_id)
        .bind(&self.store_id)
        .bind(&self.category_id)
        .bind(&self.style_id)
        .bind(&self.clothing_color)
        .bind(&self.clothing_flaw)
        .bind(&self.estimate)
        .bind(&self.clothing_brand)
        .bind(&self.service_type)
        .bind(&self.service_requirement)
        .bind(&self.before_pics)
        .bind(&self.after_pics)
        .bind(&self.notes)
        .bind(&self.process_markup)
        .bind(&self.price_value)
        .bind(&self.hang_type)
        .bind(&self.hang_location_code)
        .bind(&self.hanger_number)
        .bind(&self.hang_cloth_code)
        .bind(&self.hang_remark)
        .bind(&self.create_time)
        .bind(&self.pickup_time)
        .bind(&self.pickup_method)
        .bind(&self.clothing_status)
        .bind(&self.remark)
        .fetch_one(&mut **tr)
        .await?;
        Ok(cloth)
    }

    pub async fn get_by_id(pool: &Pool<Sqlite>, id: i64) -> Result<Option<Self>> {
        let cloth = sqlx::query_as(&format!("{} WHERE cloth_id = ?", SQL))
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(cloth)
    }

    /// only need basic information, no necessary to query clothing or hangers info
    pub async fn get_by_ids(pool: &Pool<Sqlite>, id: &[i64]) -> Result<Vec<Self>> {
        let mut builder =
            sqlx::QueryBuilder::new("SELECT * FROM order_clothes WHERE cloth_id IN (");

        // bind ids
        for (i, id) in id.iter().enumerate() {
            if i > 0 {
                builder.push(", ");
            }
            builder.push_bind(id);
        }
        builder.push(")");

        let clothes = builder.build_query_as().fetch_all(pool).await?;
        Ok(clothes)
    }

    pub async fn get_by_cloth_code(pool: &Pool<Sqlite>, cloth_code: &str) -> Result<Option<Self>> {
        let cloth = sqlx::query_as::<_, Self>(&format!("{} WHERE oc.hang_cloth_code = ?", SQL))
            .bind(cloth_code)
            .fetch_optional(pool)
            .await?;
        Ok(cloth)
    }

    pub async fn get_by_order_id(pool: &Pool<Sqlite>, order_id: i64) -> Result<Vec<Self>> {
        let cloth = sqlx::query_as::<_, Self>(&format!("{} WHERE oc.order_id = ?", SQL))
            .bind(order_id)
            .fetch_all(pool)
            .await?;
        Ok(cloth)
    }

    pub async fn get_by_order_id_with_tx(
        tx: &mut Transaction<'_, Sqlite>,
        order_id: i64,
    ) -> Result<Vec<Self>> {
        let cloth = sqlx::query_as::<_, Self>(&format!("{} WHERE oc.order_id = ?", SQL))
            .bind(order_id)
            .fetch_all(&mut **tx)
            .await?;
        Ok(cloth)
    }

    pub async fn get_by_user_id(
        pool: &Pool<Sqlite>,
        user_id: i64,
        page_params: PageParams,
    ) -> Result<Vec<Self>> {
        let mut builder = QueryBuilder::<Sqlite>::new(&format!(
            "{SQL} WHERE order_id in (select order_id from orders where user_id = "
        ));

        builder.push_bind(user_id);
        builder.push(")");

        builder.push(" LIMIT ").push_bind(page_params.page_size);
        builder
            .push(" OFFSET ")
            .push_bind(page_params.page_size * (page_params.page - 1));

        let cloth = builder.build_query_as().fetch_all(pool).await?;
        Ok(cloth)
    }

    async fn count_by_user_id(pool: &Pool<Sqlite>, user_id: i64) -> Result<u64> {
        let count =
            sqlx::query_scalar::<_, u64>("SELECT COUNT(1) FROM order_clothes WHERE order_id in (select order_id from orders where user_id = ?)")
                .bind(user_id)
                .fetch_one(pool)
                .await?;
        Ok(count)
    }

    pub async fn query_hanger_number(
        pool: &Pool<Sqlite>,
        store_id: i64,
        hang_location_code: i64,
    ) -> Result<Vec<i32>> {
        let numbers = sqlx::query_scalar(
            "SELECT hanger_number FROM order_clothes WHERE store_id = ? AND hang_location_code = ? AND clothing_status = '01'",
        )
        .bind(store_id)
        .bind(hang_location_code)
        .fetch_all(pool)
        .await?;
        Ok(numbers)
    }

    pub async fn refound_by_order_id(
        tr: &mut Transaction<'_, Sqlite>,
        order_id: i64,
    ) -> Result<bool> {
        let result = sqlx::query("UPDATE order_clothes SET clothing_status = '03', pickup_method = '00' WHERE order_id = ?")
            .bind(order_id)
            .execute(&mut **tr)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn update_order_id(
        tr: &mut Transaction<'_, Sqlite>,
        order_id: i64,
        clothes_id: &[i64],
    ) -> Result<bool> {
        if clothes_id.is_empty() {
            return Ok(true);
        }

        let mut builder = QueryBuilder::new("UPDATE order_clothes SET order_id =");
        builder.push_bind(order_id);
        builder.push(" WHERE cloth_id IN (");

        for (i, id) in clothes_id.iter().enumerate() {
            if i > 0 {
                builder.push(",");
            }
            builder.push_bind(id);
        }
        builder.push(")");

        let result = builder.build().execute(&mut **tr).await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn update(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<bool> {
        // 确保有 cloth_id
        let cloth_id = self.cloth_id.ok_or(Error::bad_request("缺少衣物ID"))?;

        // 执行更新
        let result = sqlx::query(
            r#"
            UPDATE order_clothes SET
                order_id = ?,
                clothing_id = ?,
                clothing_category = ?,
                clothing_style = ?,
                clothing_color = ?,
                clothing_flaw = ?,
                estimate = ?,
                clothing_brand = ?,
                service_type = ?,
                service_requirement = ?,
                before_pics = ?,
                after_pics = ?,
                notes = ?,
                process_markup = ?,
                price_value = ?,
                hang_type = ?,
                hang_location_code = ?,
                hanger_number = ?,
                hang_cloth_code = ?,
                hang_remark = ?,
                pickup_time = ?,
                pickup_method = ?,
                clothing_status = ?
            WHERE cloth_id = ?
            "#,
        )
        .bind(&self.order_id)
        .bind(&self.clothing_id)
        .bind(&self.clothing_category)
        .bind(&self.clothing_style)
        .bind(self.clothing_color)
        .bind(&self.clothing_flaw)
        .bind(&self.estimate)
        .bind(self.clothing_brand)
        .bind(&self.service_type)
        .bind(&self.service_requirement)
        .bind(&self.before_pics)
        .bind(&self.after_pics)
        .bind(&self.notes)
        .bind(self.process_markup)
        .bind(self.price_value)
        .bind(&self.hang_type)
        .bind(&self.hang_location_code)
        .bind(self.hanger_number)
        .bind(&self.hang_cloth_code)
        .bind(&self.hang_remark)
        .bind(&self.pickup_time)
        .bind(&self.pickup_method)
        .bind(&self.clothing_status)
        .bind(cloth_id)
        .execute(&mut **tr)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_batch(tr: &mut Transaction<'_, Sqlite>, ids: &[i64]) -> Result<u64> {
        let mut builder = sqlx::QueryBuilder::new("DELETE FROM order_clothes WHERE cloth_id IN (");

        ids.iter().enumerate().for_each(|(i, id)| {
            if i > 0 {
                builder.push(",");
            }
            builder.push_bind(id);
        });

        builder.push(")");

        let result = builder.build().execute(&mut **tr).await?;
        Ok(result.rows_affected())
    }

    pub async fn update_cloth_status_delivery(
        tr: &mut Transaction<'_, Sqlite>,
        cloth_ids: &[i64],
        status: &str,
    ) -> Result<bool> {
        if cloth_ids.is_empty() {
            return Ok(false);
        }

        let mut query_builder = QueryBuilder::new("UPDATE order_clothes SET clothing_status = ");
        query_builder.push_bind(status);

        // Add pickup time if the status is for delivery
        if status == CLOTH_STATUS_DELIVERED {
            query_builder.push(", pickup_time = ");
            query_builder.push_bind(utils::get_now());
            query_builder.push(", pickup_method = ");
            query_builder.push_bind("delivery"); // Set pickup method to delivery
        }

        query_builder.push(" WHERE cloth_id IN (");

        cloth_ids.iter().enumerate().for_each(|(i, id)| {
            if i > 0 {
                query_builder.push(", ");
            }
            query_builder.push_bind(id);
        });

        query_builder.push(")");

        let result = query_builder.build().execute(&mut **tr).await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn get_delivery_eligible_clothes(
        pool: &Pool<Sqlite>,
        store_id: i64,
        user_id: Option<i64>,
    ) -> Result<Vec<Self>> {
        // Modified to include clothes with status '01' (washing) or '02' (washed)
        let status_clause = "oc.clothing_status IN ('01', '02')";

        if let Some(uid) = user_id {
            sqlx::query_as::<_, Self>(&format!(
                "{} 
                LEFT JOIN orders o ON oc.order_id = o.order_id 
                WHERE {} AND oc.store_id = ? AND o.user_id = ?",
                SQL, status_clause
            ))
            .bind(store_id)
            .bind(uid)
            .fetch_all(pool)
            .await
        } else {
            sqlx::query_as::<_, Self>(&format!(
                "{} WHERE {} AND oc.store_id = ?",
                SQL, status_clause
            ))
            .bind(store_id)
            .fetch_all(pool)
            .await
        }
        .map_err(|e| Error::internal(format!("Failed to get delivery eligible clothes: {}", e)))
    }
}

impl OrderCloth {
    async fn generate_clothing_number(
        &self,
        // pool: &Pool<Sqlite>,
        tr: &mut Transaction<'_, Sqlite>,
    ) -> Result<String> {
        // 获取当前序列号
        let sequence = ClothSequence::get_latest_sequence(tr).await?;
        let sequence = if let Some(mut seq) = sequence {
            seq.sequence_number += 1;
            seq.update(tr).await?;
            seq
        } else {
            // 如果序列号为空，则创建新的序列号
            let mut new_sequence = ClothSequence {
                id: 0,
                date: Local::now().date_naive(),
                sequence_number: 1,
            };
            new_sequence.add(tr).await?;
            new_sequence
        };

        // 获取当前日期并格式化为 "yyMMdd"
        let today_date = utils::get_now().format("%y%m%d").to_string();

        // 生成最终编码
        let clothing_number = format!("{}{}", today_date, sequence.sequence_number);

        Ok(clothing_number)
    }

    pub async fn insert_order_cloth(&mut self, pool: &Pool<Sqlite>) -> Result<Self> {
        let mut tr = pool.begin().await?; // 开启事务

        // 设置时间
        self.create_time = Some(utils::get_now());

        // 设置衣物状态为洗护中
        self.clothing_status = Some("01".to_string());

        // 生成衣物编号
        self.hang_cloth_code = Some(self.generate_clothing_number(&mut tr).await?);

        // 清空取走时间
        self.pickup_time = None;

        // 生成衣挂位置
        let drying_rack = DryingRack::get_position(
            pool,
            self.store_id.unwrap(),
            self.hang_type.clone().unwrap_or("01".to_string()),
        )
        .await?;

        if !drying_rack.update(&mut tr).await? {
            return Err(Error::internal("Failed to update drying rack"));
        }

        self.hanger_number = drying_rack.position.map(|x| x - 1);
        self.hang_location_code = drying_rack.id;

        // 标签处理
        let mut tag_ids = Vec::with_capacity(5);
        if let Some(brand) = self.clothing_brand {
            tag_ids.push(brand);
        }
        if let Some(color) = self.clothing_color {
            tag_ids.push(color);
        }
        if let Some(flaw) = &self.clothing_flaw {
            if !flaw.is_empty() {
                tag_ids.extend(
                    flaw.split(",")
                        .map(|id| id.parse::<i64>().unwrap_or_default()),
                );
            }
        }
        if let Some(estimate) = &self.estimate {
            if !estimate.is_empty() {
                tag_ids.extend(
                    estimate
                        .split(",")
                        .map(|id| id.parse::<i64>().unwrap_or_default()),
                );
            }
        }

        // 增加标签引用次数
        if !tag_ids.is_empty() {
            // 增加标签引用次数的方法
            Tag::increment_ref_num(&mut tr, &tag_ids).await?;
        }

        // 增加衣物引用计数
        if let Some(id) = self.clothing_id {
            if !Clothing::increment_ref_num(&mut tr, id).await? {
                return Err(Error::internal("Failed to increment clothing ref num"));
            }
        }

        // 插入数据
        let result = self.add(&mut tr).await?;

        // 提交事务
        tr.commit().await?;

        Ok(result)
    }

    // 辅助方法：从图片列表中删除指定的图片 ID，保持格式正确
    fn remove_picture_id(pics: &str, picture_id_str: &str) -> String {
        pics.split(',')
            .filter(|&pic| pic != picture_id_str) // 过滤掉要删除的图片 ID
            .collect::<Vec<&str>>()
            .join(",") // 用逗号重新拼接
    }

    pub(crate) async fn update_picture(
        tr: &mut Transaction<'_, Sqlite>,
        mut cloth: OrderCloth,
        picture_id: i64,
    ) -> Result<bool> {
        let picture_id_str = picture_id.to_string();

        // 处理 before_pics 或 after_pics
        if let Some(ref before_pics) = cloth.before_pics {
            if before_pics.contains(&picture_id_str) {
                // 如果图片 ID 存在于 before_pics 中，移除它
                cloth.before_pics = Some(Self::remove_picture_id(before_pics, &picture_id_str));
            }
        } else if let Some(ref after_pics) = cloth.after_pics {
            if after_pics.contains(&picture_id_str) {
                // 如果图片 ID 存在于 after_pics 中，移除它
                cloth.after_pics = Some(Self::remove_picture_id(after_pics, &picture_id_str));
            }
        }

        cloth.update(tr).await
    }

    async fn delete_pics(tr: &mut Transaction<'_, Sqlite>, pics: &str) -> Result<()> {
        let ids: Vec<i64> = pics
            .split(',')
            .filter_map(|s| s.parse::<i64>().ok()) // 解析 ids
            .collect();

        // get pictures from db
        let pictures = OrderPicture::get_by_ids(tr, &ids).await?;

        // delete from fs
        for picture in pictures {
            if let Some(path) = picture.picture_path {
                if let Err(err) = std::fs::remove_file(&path) {
                    log::error!("Failed to delete file {}: {}", path, err);
                }
            }
        }

        // 删除数据库中的记录
        OrderPicture::delete_batch(tr, &ids).await?;
        Ok(())
    }

    // 删除图片的方法
    async fn delete_pictures(tr: &mut Transaction<'_, Sqlite>, cloth: &OrderCloth) -> Result<()> {
        // 删除 beforePics 中的图片
        if let Some(pics) = &cloth.before_pics {
            Self::delete_pics(tr, pics).await?;
        }

        // 删除 afterPics 中的图片
        if let Some(pics) = &cloth.after_pics {
            Self::delete_pics(tr, pics).await?;
        }

        Ok(())
    }

    pub async fn delete_by_ids(pool: &Pool<Sqlite>, ids: &[i64]) -> Result<u64> {
        // query clothes by ids
        let clothes = OrderCloth::get_by_ids(pool, ids).await?;
        let mut tr = pool.begin().await?;
        // delete pictures in clothes
        for cloth in clothes {
            Self::delete_pictures(&mut tr, &cloth).await?;
        }
        let result = OrderCloth::delete_batch(&mut tr, ids).await?;
        tr.commit().await?;
        Ok(result)
    }

    pub async fn hang_cloth(state: &State<'_, AppState>, hang_req: HangReq) -> Result<()> {
        let store_id = utils::get_user_id(&state).await?;
        let pool = &state.pool;
        let mut tr = pool.begin().await?;

        // query cloth information by id
        let mut cloth = OrderCloth::get_by_id(pool, hang_req.cloth_id)
            .await?
            .ok_or(Error::with_kind(ErrorKind::NotFound))?;

        if cloth.order_id.is_none() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "cloth has no order information",
            ));
        }

        // update cloth status
        cloth.clothing_status = Some(CLOTH_STATUS_HANGED.to_string());
        cloth.hang_location_code = Some(hang_req.hang_location_id);
        cloth.hanger_number = Some(hang_req.hanger_number);
        cloth.hang_remark = hang_req.hang_remark;

        if !cloth.update(&mut tr).await? {
            return Err(Error::with_details(
                ErrorKind::InternalServer,
                "update cloth information failed",
            ));
        }

        // query order information
        let mut order = Order::get_by_id(pool, store_id, cloth.order_id.unwrap())
            .await?
            .ok_or(Error::with_details(ErrorKind::NotFound, "order not found"))?;

        // check if all clothes are hanged
        let is_all_hanged = OrderCloth::get_by_order_id(pool, order.order_id.unwrap())
            .await?
            .iter()
            .filter(|c| c.cloth_id != cloth.cloth_id)
            .all(|c| c.clothing_status == Some(CLOTH_STATUS_HANGED.to_string()));

        // handle order status and message sending
        if order.pickup_code.is_none() {
            order.pickup_code = Some(Self::generate_pickup_code(pool).await?);
        }

        if is_all_hanged {
            order.status = Some(CLOTH_STATUS_HANGED.to_string());
        }

        // Self::update_order_and_notify(&mut tr, state, &mut order, is_all_hanged).await?;
        if let Err(err) =
            Self::update_order_and_notify(&mut tr, state, &mut order, is_all_hanged).await
        {
            match err.kind() {
                // 这些错误类型需要返回给前端，但不应导致事务回滚
                ErrorKind::SmsNotSubscribed | ErrorKind::SmsRemainShort => {
                    tracing::warn!("SMS service error: {:?}", err);
                    // 提交事务以确保其他操作成功
                    tr.commit().await?;
                    // 然后返回错误给前端
                    return Err(err);
                }
                // 其他错误类型仍然返回错误
                _ => return Err(err),
            }
        }

        tr.commit().await?;
        Ok(())
    }

    /// Generates a unique pickup code.
    async fn generate_pickup_code(pool: &Pool<Sqlite>) -> Result<String> {
        let mut code = utils::gen_random_number();
        while Order::check_pickup_code(pool, code.to_string())
            .await?
            .is_some()
        {
            code = utils::gen_random_number();
        }
        Ok(code.to_string())
    }

    /// Updates the order status and sends a notification if necessary.
    async fn update_order_and_notify(
        tr: &mut Transaction<'_, Sqlite>,
        state: &State<'_, AppState>,
        order: &mut Order,
        is_all_hanged: bool,
    ) -> Result<()> {
        if !order.update(tr).await? {
            return Err(Error::with_details(
                ErrorKind::InternalServer,
                "update order information failed",
            ));
        }

        if is_all_hanged {
            if let Some(tel) = &order.phonenumber {
                if let Some(code) = &order.pickup_code {
                    Self::send_pickup_msg(
                        tr,
                        state,
                        code,
                        tel,
                        &order.order_number.as_ref().unwrap_or(&String::new()),
                        order.user_id.unwrap_or_default(),
                    )
                    .await?;
                }
            }
        }

        Ok(())
    }

    async fn send_pickup_msg(
        tx: &mut Transaction<'_, Sqlite>,
        state: &State<'_, AppState>,
        code: &str,
        tel: &str,
        order_num: &str,
        user_id: i64,
    ) -> Result<()> {
        let token = state
            .token
            .lock()
            .await
            .clone()
            .ok_or(Error::account_or_pwd())?;
        let token_str = token.token.clone();
        let store_id = token.user.id.unwrap();

        // release mutex
        drop(token);

        // 直接构建参数，调用专有接口
        let param = HashMap::from([("code".to_string(), code.to_string())]);

        let body = SendSmsRequest {
            temp_id: 0, // 不再需要模板ID，服务端会处理
            store_id,
            phone: tel.to_string(),
            args: Some(param),
        };

        tracing::info!("send hangup sms request: {:?}", body);

        // 调用专有接口发送短信
        let result = match state
            .http_client
            .post("/sms/hangup", body, Some(&token_str))
            .await
        {
            Ok(res) => {
                if res {
                    String::from("0")
                } else {
                    String::from("1")
                }
            }
            Err(err) => {
                // 检查是否是短信服务相关错误
                match err.kind() {
                    ErrorKind::SmsNotSubscribed => {
                        tracing::warn!("SMS service not subscribed: {:?}", err);
                        // 返回特定错误，而不是简单忽略
                        return Err(Error::with_details(
                            ErrorKind::SmsNotSubscribed,
                            "短信服务未订阅，请先订阅短信服务",
                        ));
                    }
                    ErrorKind::SmsRemainShort => {
                        tracing::warn!("SMS remaining count is low: {:?}", err);
                        // 返回特定错误，而不是简单忽略
                        return Err(Error::with_details(
                            ErrorKind::SmsRemainShort,
                            "短信余量不足，请及时充值",
                        ));
                    }
                    _ => {
                        tracing::error!("send sms failed: {:?}", err);
                    }
                }
                String::from("1")
            }
        };

        // 无论短信是否发送成功，都创建通知记录
        let mut record = NoticeRecord {
            notice_id: None,
            user_id,
            order_number: Some(order_num.to_string()),
            notice_method: Some("0".to_string()),
            notice_type: Some("0".to_string()),
            title: Some("取衣通知".to_string()),
            content: Some(format!("取件码：{}", code)),
            result: Some(result),
            ..Default::default()
        };

        // 尝试创建记录，但即使失败也不影响主流程
        if let Err(e) = record.create(tx).await {
            tracing::error!("Failed to create notice record: {:?}", e);
        }

        Ok(())
    }

    /// clothes may be in different orders
    pub async fn pickup(pool: &Pool<Sqlite>, store_id: i64, ids: &[i64]) -> Result<()> {
        let mut tr = pool.begin().await?;
        // query clothes by ids and change status
        let mut clothes = Self::get_by_ids(pool, ids).await?;
        for cloth in clothes.iter_mut() {
            cloth.clothing_status = Some(CLOTH_STATUS_PICKED.to_string());
            cloth.pickup_time = Some(utils::get_now());
            cloth.pickup_method = Some(CLOTH_STATUS_PICKED.to_string());

            // update cloth
            if !cloth.update(&mut tr).await? {
                return Err(Error::internal("update cloth information failed"));
            }

            // update rack remain count
            let mut rack = DryingRack::get_by_id(pool, cloth.hang_location_code.unwrap())
                .await?
                .ok_or(Error::with_details(ErrorKind::NotFound, "rack not found"))?;
            rack.remaining_capacity = rack.remaining_capacity.and_then(|i| Some(i - 1));
            if !rack.update(&mut tr).await? {
                return Err(Error::internal("update rack information failed"));
            }
        }

        // collect order ids
        let order_ids: Vec<i64> = clothes.iter().filter_map(|c| c.order_id).collect();

        // update order status to pickup if all clothes are picked
        for order_id in order_ids {
            let clothes = OrderCloth::get_by_order_id_with_tx(&mut tr, order_id).await?;
            if clothes
                .iter()
                .filter(|c| c.clothing_status == Some(CLOTH_STATUS_PICKED.to_string()))
                .count()
                == clothes.len()
            {
                let order = Order::get_by_id(pool, store_id, order_id).await?;
                if let Some(mut order) = order {
                    // update order status to complete if it was paid already
                    if order.payment_status == Some(ORDER_STATUS_PAID.to_string()) {
                        order.status = Some(ORDER_STATUS_COMPLETED.to_string());
                        if !order.update(&mut tr).await? {
                            return Err(Error::with_details(
                                ErrorKind::InternalServer,
                                "update order information failed",
                            ));
                        }
                    }
                } else {
                    // log warn
                    tracing::warn!("order: {order_id} not found");
                }
            }
        }

        tr.commit().await?;

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct SendSmsRequest {
    pub temp_id: i64,
    pub store_id: i64,
    pub phone: String,
    pub args: Option<HashMap<String, String>>,
}

#[tauri::command]
pub async fn list_order_clothes_history(
    state: State<'_, AppState>,
    user_id: i64,
    page_params: PageParams,
) -> Result<PageResult<OrderCloth>> {
    let rows = OrderCloth::get_by_user_id(&state.pool, user_id, page_params).await?;
    let total = OrderCloth::count_by_user_id(&state.pool, user_id).await?;
    Ok(PageResult { rows, total })
}

#[tauri::command]
pub async fn list_order_clothes(
    state: State<'_, AppState>,
    order_id: i64,
) -> Result<Vec<OrderCloth>> {
    OrderCloth::get_by_order_id(&state.pool, order_id).await
}

#[tauri::command]
pub async fn get_order_cloth_by_id(
    state: State<'_, AppState>,
    cloth_id: i64,
) -> Result<Option<OrderCloth>> {
    OrderCloth::get_by_id(&state.pool, cloth_id).await
}

#[tauri::command]
pub async fn get_order_cloth_by_code(
    state: State<'_, AppState>,
    code: String,
) -> Result<Option<OrderCloth>> {
    OrderCloth::get_by_cloth_code(&state.pool, &code).await
}

#[tauri::command]
pub async fn update_order_cloth(state: State<'_, AppState>, cloth: OrderCloth) -> Result<bool> {
    let mut tr = state.pool.begin().await?;
    let result = cloth.update(&mut tr).await?;
    tr.commit().await?;
    Ok(result)
}

#[tauri::command]
pub async fn add_order_cloth(
    state: State<'_, AppState>,
    mut order_cloth: OrderCloth,
) -> Result<OrderCloth> {
    let store_id = utils::get_user_id(&state).await?;
    order_cloth.store_id = Some(store_id);
    order_cloth.insert_order_cloth(&state.pool).await
}

#[tauri::command]
pub async fn remove_pic_from_order_cloth(
    state: State<'_, AppState>,
    cloth_id: i64,
    pic_id: i64,
) -> Result<()> {
    OrderPicture::delete_by_id(&state.pool, pic_id, cloth_id).await?;
    Ok(())
}

#[tauri::command]
pub async fn pickup_order_cloth(state: State<'_, AppState>, clothes_id: Vec<i64>) -> Result<()> {
    let store_id = utils::get_user_id(&state).await?;
    OrderCloth::pickup(&state.pool, store_id, &clothes_id).await
}

#[tauri::command]
pub async fn hang_order_cloth(state: State<'_, AppState>, hang_req: HangReq) -> Result<()> {
    OrderCloth::hang_cloth(&state, hang_req).await
}

#[tauri::command]
pub async fn delete_order_cloth_by_ids(state: State<'_, AppState>, ids: Vec<i64>) -> Result<u64> {
    OrderCloth::delete_by_ids(&state.pool, &ids).await
}

/// 上传衣物图片
#[tauri::command]
pub async fn upload_cloth_pic(
    state: State<'_, AppState>,
    filename: String,
    cloth_id: i64,
    is_pre: bool,
) -> Result<Option<i64>> {
    let pool = &state.pool;
    // query cloth information by cloth id
    let mut cloth = OrderCloth::get_by_id(pool, cloth_id)
        .await?
        .ok_or(Error::not_found("衣物不存在"))?;

    let mut tx = pool.begin().await?;
    // insert into database for pic
    let picture = OrderPicture::new_with_path(filename)
        .insert(&mut tx)
        .await?;

    if is_pre {
        if let Some(pic_id) = picture.picture_id {
            if let Some(ref mut pics) = cloth.before_pics {
                // 根据逗号进行切割，然后将新的picture id push进去
                pics.push(',');
                pics.push_str(&pic_id.to_string());
            } else {
                cloth.before_pics = Some(pic_id.to_string());
            }
        }
    } else {
        if let Some(pic_id) = picture.picture_id {
            if let Some(ref mut pics) = cloth.after_pics {
                // 根据逗号进行切割，然后将新的picture id push进去
                pics.push(',');
                pics.push_str(&pic_id.to_string());
            } else {
                cloth.after_pics = Some(pic_id.to_string());
            }
        }
    }

    if !cloth.update(&mut tx).await? {
        return Err(Error::internal("衣物照片信息更新失败"));
    }
    tx.commit().await?;
    Ok(picture.picture_id)
}

#[tauri::command]
pub async fn list_delivery_eligible_clothes(
    state: State<'_, AppState>,
    user_id: Option<i64>,
) -> Result<Vec<OrderCloth>> {
    let store_id = utils::get_user_id(&state).await?;
    OrderCloth::get_delivery_eligible_clothes(&state.pool, store_id, user_id).await
}

#[tauri::command]
pub async fn deliver_clothes(state: State<'_, AppState>, cloth_ids: Vec<i64>) -> Result<bool> {
    let mut tx = state.pool.begin().await?;

    // Update clothes status to delivered
    let success =
        OrderCloth::update_cloth_status_delivery(&mut tx, &cloth_ids, CLOTH_STATUS_DELIVERED)
            .await?;

    tx.commit().await?;
    Ok(success)
}

// Add this new Tauri command to retrieve multiple clothes by IDs
#[tauri::command]
pub async fn get_order_cloths_by_ids(
    state: State<'_, AppState>,
    cloth_ids: Vec<i64>,
) -> Result<Vec<OrderCloth>> {
    let store_id = utils::get_user_id(&state).await?;

    // Get clothes by IDs
    let clothes = OrderCloth::get_by_ids(&state.pool, &cloth_ids).await?;

    // Filter by store ID if needed
    let filtered_clothes = clothes
        .into_iter()
        .filter(|cloth| cloth.store_id.unwrap_or_default() == store_id)
        .collect();

    Ok(filtered_clothes)
}
