use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{
    FromRow, Pool, QueryBuilder, Row, Sqlite, Transaction,
    types::chrono::{DateTime, FixedOffset},
};
use tauri::State;

use crate::db::coupon_orders::CouponOrder;
use crate::db::payments::Payment;
use crate::db::user_coupons::UserCoupon;
use crate::db::{Curd, PageParams, PageResult, Validator};
use crate::error::{Error, ErrorKind, Result};
use crate::state::AppState;
use crate::utils;
use crate::utils::chrono_serde::{deserialize_date, serialize_date};

use super::user::User;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Coupon {
    pub coupon_id: Option<i64>,           // Coupon ID
    pub store_id: Option<i64>,            // Coupon ID
    pub coupon_number: Option<String>,    // Unique Coupon Number
    pub coupon_type: Option<String>,      // Coupon Type (e.g., '000')
    pub coupon_title: Option<String>,     // Coupon Title
    pub coupon_value: Option<f64>,        // Coupon Value
    pub min_spend: Option<f64>,           // Minimum Spend
    pub customer_invalid: Option<String>, // Whether the coupon is invalid for the customer
    pub customer_sale_total: Option<i32>, // Total sales associated with the customer
    pub customer_sale_count: Option<i32>, // Total count of coupons used by the customer
    #[serde(
        deserialize_with = "deserialize_date",
        serialize_with = "serialize_date"
    )]
    pub valid_from: Option<DateTime<FixedOffset>>, // Start date of validity
    #[serde(
        deserialize_with = "deserialize_date",
        serialize_with = "serialize_date"
    )]
    pub valid_to: Option<DateTime<FixedOffset>>, // End date of validity
    pub auto_delay: Option<String>,       // Whether the coupon is auto-delayed
    pub usage_value: Option<f64>,         // Usage value
    pub usage_limit: Option<f64>,         // Usage limit
    pub del_flag: Option<String>,         // Delete flag
    pub applicable_category: Option<String>, // Applicable categories
    pub applicable_style: Option<String>, // Applicable styles
    pub applicable_cloths: Option<String>, // Applicable cloths
    pub status: Option<String>,           // Coupon status (e.g., '0' for active, '1' for inactive)
    pub remark: Option<String>,           // Additional remarks
    pub desc: Option<String>,             // Additional description
}

impl FromRow<'_, SqliteRow> for Coupon {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            coupon_id: row.try_get("coupon_id").unwrap_or_default(),
            store_id: row.try_get("store_id").unwrap_or_default(),
            coupon_number: row.try_get("coupon_number").unwrap_or_default(),
            coupon_title: row.try_get("coupon_title").unwrap_or_default(),
            coupon_type: row.try_get("coupon_type").unwrap_or_default(),
            coupon_value: row.try_get("coupon_value").unwrap_or_default(),
            min_spend: row.try_get("min_spend").unwrap_or_default(),
            customer_invalid: row.try_get("customer_invalid").unwrap_or_default(),
            customer_sale_total: row.try_get("customer_sale_total").unwrap_or_default(),
            customer_sale_count: row.try_get("customer_sale_count").unwrap_or_default(),
            valid_from: row.try_get("valid_from").unwrap_or_default(),
            valid_to: row.try_get("valid_to").unwrap_or_default(),
            auto_delay: row.try_get("auto_delay").unwrap_or_default(),
            usage_value: row.try_get("usage_value").unwrap_or_default(),
            usage_limit: row.try_get("usage_limit").unwrap_or_default(),
            del_flag: row.try_get("del_flag").unwrap_or_default(),
            applicable_category: row.try_get("applicable_category").unwrap_or_default(),
            applicable_style: row.try_get("applicable_style").unwrap_or_default(),
            applicable_cloths: row.try_get("applicable_cloths").unwrap_or_default(),
            status: row.try_get("status").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
            desc: row.try_get("desc").unwrap_or_default(),
        })
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CouponBuyReq {
    pub coupons: Vec<CouponIdCount>,
    pub user_id: i64,
    pub payment_method: String,
    #[allow(dead_code)]
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CouponIdCount {
    pub coupon_id: i64,
    pub count: i32,
}

impl Validator for Coupon {
    fn validate(&self) -> Result<()> {
        if self.coupon_type.is_none() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "coupon_type is required",
            ));
        }

        if self.coupon_title.is_none() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "coupon_title is required",
            ));
        }

        if self.coupon_value.is_none() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "coupon_value is required",
            ));
        }

        if self.valid_from.is_none() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "valid_from is required",
            ));
        }

        if self.valid_to.is_none() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "valid_to is required",
            ));
        }

        if let (Some(valid_from), Some(valid_to)) = (&self.valid_from, &self.valid_to) {
            if valid_from > valid_to {
                return Err(Error::with_details(
                    ErrorKind::BadRequest,
                    "Valid from date cannot be later than valid to date.",
                ));
            }
        }

        Ok(())
    }
}

impl Coupon {
    fn new4sale(
        status: impl ToString,
        del_flag: impl ToString,
        title: Option<String>,
        tp: Option<String>,
    ) -> Self {
        Self {
            status: Some(status.to_string()),
            del_flag: Some(del_flag.to_string()),
            coupon_title: title,
            coupon_type: tp,
            ..Default::default()
        }
    }
}

impl Curd for Coupon {
    const COUNT_SQL: &'static str = "SELECT COUNT(*) FROM coupons WHERE del_flag = '0' ";
    const QUERY_SQL: &'static str = "SELECT * FROM coupons WHERE del_flag = '0' ";
    const BY_ID_SQL: &'static str = "SELECT * FROM coupons WHERE del_flag = '0' AND coupon_id = ?";
    const DELETE_BATCH_SQL: &'static str = "UPDATE coupons SET del_flag = '2' WHERE coupon_id IN (";

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, Sqlite>) {
        if let Some(coupon_number) = &self.coupon_number {
            builder
                .push(" AND coupon_number LIKE ")
                .push_bind(format!("%{}%", coupon_number));
        }

        if let Some(coupon_name) = &self.coupon_title {
            builder
                .push(" AND coupon_title LIKE ")
                .push_bind(format!("%{}%", coupon_name));
        }

        if let Some(coupon_type) = &self.coupon_type {
            builder.push(" AND coupon_type = ").push_bind(coupon_type);
        }

        if let Some(status) = &self.status {
            builder.push(" AND status = ").push_bind(status);
        }

        if let Some(store_id) = &self.store_id {
            builder.push(" AND store_id = ").push_bind(store_id);
        }

        if let Some(del_flag) = &self.del_flag {
            builder.push(" AND del_flag = ").push_bind(del_flag);
        }
    }
}

impl Coupon {
    pub async fn add(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<Self> {
        let query = r#"
        INSERT INTO coupons (store_id, coupon_number, coupon_type, coupon_title, desc, coupon_value, min_spend, customer_invalid,
                            customer_sale_total, customer_sale_count, valid_from, valid_to, auto_delay, usage_value,
                            usage_limit, del_flag, applicable_category, applicable_style, applicable_cloths, status, remark)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *
    "#;

        tracing::debug!("coupon add: {:?}", self);
        let result = sqlx::query_as(query)
            .bind(&self.store_id)
            .bind(&self.coupon_number)
            .bind(&self.coupon_type)
            .bind(&self.coupon_title)
            .bind(&self.desc)
            .bind(self.coupon_value)
            .bind(self.min_spend)
            .bind(&self.customer_invalid)
            .bind(self.customer_sale_total)
            .bind(self.customer_sale_count)
            .bind(self.valid_from)
            .bind(self.valid_to)
            .bind(&self.auto_delay)
            .bind(self.usage_value)
            .bind(self.usage_limit)
            .bind(&self.del_flag)
            .bind(&self.applicable_category)
            .bind(&self.applicable_style)
            .bind(&self.applicable_cloths)
            .bind(&self.status)
            .bind(&self.remark)
            .fetch_one(&mut **tr)
            .await?;

        Ok(result)
    }

    pub async fn list4sale(&self, pool: &Pool<Sqlite>) -> Result<Vec<Self>> {
        let mut builder = QueryBuilder::<Sqlite>::new(
            "SELECT * FROM coupons WHERE
                 customer_sale_count != 0
                 AND customer_sale_total != 0
                 AND ",
        );

        builder.push_bind(utils::get_now());
        builder.push(" BETWEEN valid_from AND valid_to ");

        self.apply_filters(&mut builder);

        let result = builder.build_query_as().fetch_all(pool).await?;
        Ok(result)
    }

    pub async fn update(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<bool> {
        // Ensure coupon_id is present
        if self.coupon_id.is_none() {
            return Err(Error::bad_request("coupon_id is required for update"));
        }

        let query = r#"
        UPDATE coupons SET 
            store_id = ?,
            coupon_number = ?,
            coupon_type = ?,
            coupon_title = ?,
            coupon_value = ?,
            min_spend = ?,
            customer_invalid = ?,
            customer_sale_total = ?,
            customer_sale_count = ?,
            valid_from = ?,
            valid_to = ?,
            auto_delay = ?,
            usage_value = ?,
            usage_limit = ?,
            applicable_category = ?,
            applicable_style = ?,
            applicable_cloths = ?,
            status = ?,
            remark = ?,
            desc = ?
        WHERE coupon_id = ?
        "#;

        let result = sqlx::query(query)
            .bind(&self.store_id)
            .bind(&self.coupon_number)
            .bind(&self.coupon_type)
            .bind(&self.coupon_title)
            .bind(self.coupon_value)
            .bind(self.min_spend)
            .bind(&self.customer_invalid)
            .bind(self.customer_sale_total)
            .bind(self.customer_sale_count)
            .bind(self.valid_from)
            .bind(self.valid_to)
            .bind(&self.auto_delay)
            .bind(self.usage_value)
            .bind(self.usage_limit)
            .bind(&self.applicable_category)
            .bind(&self.applicable_style)
            .bind(&self.applicable_cloths)
            .bind(&self.status)
            .bind(&self.remark)
            .bind(&self.desc)
            .bind(self.coupon_id)
            .execute(&mut **tr)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // Add this new method to check expiring coupons
    #[allow(dead_code)]
    pub async fn check_expiring_coupons(pool: &Pool<Sqlite>) -> Result<()> {
        let two_weeks_later = Utc::now() + chrono::Duration::weeks(2);

        // Query expiring coupons of type '000'
        let expiring_coupons = sqlx::query_as::<_, Self>(
            r#"
            SELECT c.* 
            FROM coupons c
            WHERE c.coupon_type = '000'
            AND c.status = '0'
            AND c.del_flag = '0'
            AND c.valid_to <= ?
            "#,
        )
        .bind(two_weeks_later)
        .fetch_all(pool)
        .await?;

        for coupon in expiring_coupons {
            // Query users with valid balance
            let users_with_balance: Vec<User> = sqlx::query_as(
                r#"
                SELECT uc.user_id, uc.available_value as balance, u.phone_number, u.nick_name
                FROM user_coupons uc
                JOIN users u ON u.user_id = uc.user_id
                WHERE uc.coupon_id = ?
                AND uc.status = '0'
                AND uc.available_value > 0
                "#,
            )
            .bind(coupon.coupon_id)
            .fetch_all(pool)
            .await?;

            // Send notifications to users
            for user in users_with_balance {
                let message = format!(
                    "尊敬的{}，您的{}卡券（余额：{}元）即将于{}到期，请及时使用。",
                    user.nick_name.unwrap_or_default(),
                    coupon.coupon_title.as_deref().unwrap_or_default(),
                    user.balance,
                    coupon.valid_to.unwrap().format("%Y-%m-%d")
                );

                tracing::debug!("Sending message to user: {}", message);
                // Spawn async task to send message
                // if let Some(tel) = user.phonenumber  {

                //     tokio::spawn(async move {
                //         utils::send_sms(tel, message).await;
                //     });
                // }
            }
        }

        Ok(())
    }
}

impl Coupon {
    pub async fn insert(&mut self, tr: &mut Transaction<'_, Sqlite>) -> Result<Self> {
        // gen number
        let number = utils::gen_code(self.coupon_title.clone().unwrap());
        self.coupon_number = Some(format!("{number}-{}", Utc::now().timestamp_subsec_millis()));
        self.del_flag = Some("0".to_string());

        Ok(self.add(tr).await?)
    }

    pub async fn update_coupon(&mut self, pool: &Pool<Sqlite>) -> Result<bool> {
        self.validate()?;
        if self.coupon_id.is_none() {
            return Err(Error::bad_request("coupon_id is required"));
        }

        // query if there is a coupon which has sold already
        let is_exist =
            UserCoupon::exists_by_coupon_id(pool, self.store_id.unwrap(), self.coupon_id.unwrap())
                .await?;

        let mut tr = pool.begin().await?;
        if is_exist {
            // set old coupon to expired
            let mut old_coupon = Self::get_by_id(pool, self.coupon_id.unwrap())
                .await?
                .unwrap();
            old_coupon.status = Some("2".to_string());
            if !old_coupon.update(&mut tr).await? {
                return Err(Error::internal("Failed to update old coupon"));
            }

            // gen new coupon number
            let number = utils::gen_code(old_coupon.coupon_title.as_ref().unwrap());
            self.coupon_number = Some(format!("{number}-{}", Utc::now().timestamp_subsec_millis()));
            self.add(&mut tr).await?;
            tr.commit().await?;
            return Ok(true);
        }
        let result = self.update(&mut tr).await?;

        tr.commit().await?;

        Ok(result)
    }

    async fn check_coupon(
        &mut self,
        tr: &mut Transaction<'_, Sqlite>,
        info: &CouponIdCount,
    ) -> Result<()> {
        if self.customer_sale_total != Some(-1) && self.customer_sale_count != Some(-1) {
            if info.count > self.customer_sale_total.unwrap()
                || info.count > self.customer_sale_count.unwrap()
            {
                return Err(Error::bad_request("单用户购买数量超过购买限制"));
            }
        } else if self.customer_sale_total != Some(-1) {
            if info.count > self.customer_sale_total.unwrap() {
                return Err(Error::bad_request("库存不足"));
            }
            // update customer_sale_total
            self.customer_sale_total = Some(self.customer_sale_total.unwrap() - info.count);
        } else if self.customer_sale_count != Some(-1) {
            if info.count > self.customer_sale_count.unwrap() {
                return Err(Error::bad_request("超出单用户购买数量限制"));
            }
            // update customer_sale_count
            self.customer_sale_count = Some(self.customer_sale_count.unwrap() - info.count);
        }
        self.update(tr).await?;
        Ok(())
    }

    async fn send_message(user_id: i64, coupon_names: String) {
        // todo send message to user
        // todo record send information to db
        tracing::debug!("send notice to user: {} -- {}", user_id, coupon_names)
    }

    async fn create_user_coupon(
        tr: &mut Transaction<'_, Sqlite>,
        req: &CouponBuyReq,
        coupon: &Coupon,
        info: &CouponIdCount,
    ) -> Result<i64> {
        let now = utils::get_now();
        let user_coupon = UserCoupon {
            uc_id: None,
            store_id: coupon.store_id,
            user_id: Some(req.user_id),
            coupon_id: coupon.coupon_id,
            create_time: Some(now),
            obtain_at: Some(now),
            available_value: Some(coupon.usage_value.unwrap() * info.count as f64),
            uc_count: Some(info.count),
            pay_id: None,
            uc_type: Some("01".to_string()),
            status: Some("0".to_string()),
            remark: coupon.remark.clone(),
            coupon: None,
        };

        user_coupon.validate()?;

        // insert into db
        let result = user_coupon.create(tr).await?;
        Ok(result.uc_id.unwrap())
    }

    pub async fn gift(pool: &Pool<Sqlite>, coupon_buy_req: CouponBuyReq) -> Result<()> {
        let mut tr = pool.begin().await?;
        let mut coupon_names = vec![];
        for info in &coupon_buy_req.coupons {
            // query coupon by coupon_id
            let mut coupon = Self::get_by_id(pool, info.coupon_id)
                .await?
                .ok_or(Error::not_found("Coupon not found"))?;

            // check coupon status and reduce available_value if necessary
            coupon.check_coupon(&mut tr, &info).await?;

            Self::create_user_coupon(&mut tr, &coupon_buy_req, &coupon, &info).await?;

            // concat coupon name
            coupon_names.push(format!("{} x {}", coupon.coupon_title.unwrap(), info.count));
        }
        tr.commit().await?;

        // send message to user
        tokio::spawn(async move {
            Self::send_message(
                coupon_buy_req.user_id,
                format!("Your coupon has been issued: {}", coupon_names.join(", ")),
            )
            .await;
        });
        Ok(())
    }

    pub async fn buy(
        pool: &Pool<Sqlite>,
        store_id: i64,
        coupon_buy_req: CouponBuyReq,
    ) -> Result<()> {
        let mut uc_ids = Vec::with_capacity(coupon_buy_req.coupons.len());
        let mut total_amount = 0.0;
        let mut amount = 0.0;

        let mut tr = pool.begin().await?;

        for info in &coupon_buy_req.coupons {
            // query coupon by coupon_id
            let mut coupon = Self::get_by_id(pool, info.coupon_id)
                .await?
                .ok_or(Error::not_found("Coupon not found"))?;

            // check coupon status and reduce available_value if necessary
            coupon.check_coupon(&mut tr, &info).await?;

            let uc_id = Self::create_user_coupon(&mut tr, &coupon_buy_req, &coupon, &info).await?;

            // concat coupon name
            uc_ids.push(uc_id);

            total_amount += coupon.coupon_value.unwrap() * info.count as f64;
            amount += coupon.coupon_value.unwrap() * info.count as f64;
        }

        // create order
        CouponOrder::new_with_now(
            store_id,
            uc_ids
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(","),
        )
        .create(&mut tr)
        .await?;

        // create payment

        let payment = Payment {
            pay_id: Some(uuid::Uuid::new_v4().to_string()),
            pay_number: Some(format!("KQ-{}", Utc::now().timestamp_millis())),
            order_type: Some(0.to_string()),
            total_amount: Some(total_amount),
            payment_amount: Some(amount),
            payment_status: Some("01".to_string()),
            payment_method: Some(coupon_buy_req.payment_method),
            order_status: Some("01".to_string()),
            create_time: Some(utils::get_timestamp()),
            store_id: Some(store_id),
            ..Default::default()
        };
        let payment = payment.create_payment(&mut tr).await?;

        // update user coupon's payment_id
        UserCoupon::update_pay_id(&mut tr, store_id, &uc_ids, payment.pay_id.as_ref().unwrap())
            .await?;
        tr.commit().await?;
        Ok(())
    }
}

#[tauri::command]
pub async fn get_coupon_list(
    state: State<'_, AppState>,
    page_params: PageParams,
    coupon: Coupon,
) -> Result<PageResult<Coupon>> {
    coupon.get_list(&state.pool, page_params).await
}

#[tauri::command]
pub async fn get_coupons4sale(
    state: State<'_, AppState>,
    title: Option<String>,
    tp: Option<String>,
) -> Result<Vec<Coupon>> {
    let mut coupon = Coupon::new4sale("0", "0", title, tp);
    let store_id = utils::get_user_id(&state).await?;
    coupon.store_id = Some(store_id);
    coupon.list4sale(&state.pool).await
}

#[tauri::command]
pub async fn get_coupon_by_id(state: State<'_, AppState>, id: i64) -> Result<Option<Coupon>> {
    Coupon::get_by_id(&state.pool, id).await
}

#[tauri::command]
pub async fn buy_coupons(state: State<'_, AppState>, coupon_buy_req: CouponBuyReq) -> Result<()> {
    let store_id = utils::get_user_id(&state).await?;
    Coupon::buy(&state.pool, store_id, coupon_buy_req).await
}

#[tauri::command]
pub async fn gift_coupons(state: State<'_, AppState>, coupon_buy_req: CouponBuyReq) -> Result<()> {
    Coupon::gift(&state.pool, coupon_buy_req).await
}

#[tauri::command]
pub async fn add_coupon(state: State<'_, AppState>, mut coupon: Coupon) -> Result<Coupon> {
    let store_id = utils::get_user_id(&state).await?;
    coupon.store_id = Some(store_id);

    let mut tr = state.pool.begin().await?;
    coupon.validate()?;

    let result = coupon.insert(&mut tr).await?;
    tr.commit().await?;
    Ok(result)
}

#[tauri::command]
pub async fn update_coupon(state: State<'_, AppState>, mut coupon: Coupon) -> Result<bool> {
    let store_id = utils::get_user_id(&state).await?;
    coupon.store_id = Some(store_id);
    coupon.update_coupon(&state.pool).await
}

#[tauri::command]
pub async fn delete_coupons(state: State<'_, AppState>, ids: Vec<i64>) -> Result<bool> {
    let mut tr = state.pool.begin().await?;
    let result = Coupon::delete_batch(&mut tr, &ids).await?;
    tr.commit().await?;
    Ok(result)
}
