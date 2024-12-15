use crate::db::adjust_price::OrderClothAdjust;
use crate::db::cloth_price::ClothPrice;
use crate::db::configs::Config;
use crate::db::expenditure::Expenditure;
use crate::db::order_clothes::OrderCloth;
use crate::db::payments::Payment;
use crate::db::user::User;
use crate::db::user_coupons::UserCoupon;
use crate::db::{AppState, Curd, PageParams, PageResult};
use crate::error::{Error, ErrorKind, Result};
use crate::utils;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{
    types::chrono::{DateTime, FixedOffset, NaiveDate, Utc},
    FromRow, Pool, QueryBuilder, Row, Sqlite, Transaction,
};
use std::cmp::Ordering;

const PAY_STATUS_NOT_PAID: &str = "01";
const PAY_STATUS_REFUND: &str = "05";
const ORDER_STATUS_REFUND: &str = "06";
const CLOTH_STATUS_REFUND: &str = "03";
const PAY_STATUS_PAID: &str = "00";
const NORMAL_ORDER: &str = "00";
const CLOTHING_STATUS_PICKED_UP: &str = "00";
// const REWASH_ORDER: &str = "02";
const STATUS_LAUNDRY: &str = "01";
const STATUS_COMPLETED: &str = "04";
const NORMAL_ALARM: &str = "00";
const BUSINESS_MAIN: &str = "00";
const DESIRE_COMPLETE_TIME_KEY: &str = "desire_complete_time";
const STORAGE_CARD_NUMBER: &str = "000";
// const ONCE_CARD_NUMBER: &str = "002";
const OFF_CARD_NUMBER: &str = "003";
const SUB_CARD_NUMBER: &str = "004";
const SERVICE_TYPE_EMERGENCY: &str = "001";
const SERVICE_TYPE_SINGLE_WASH: &str = "002";

const PAYMENT_METHOD_MEITUAN: &str = "03";
const PAYMENT_METHOD_DOUYIN: &str = "04";
const PAYMENT_METHOD_COMBINATION_ALIPAY_COUPON: &str = "18";
const PAYMENT_METHOD_COMBINATION_WECHAT_COUPON: &str = "28";
const PAYMENT_METHOD_COMBINATION_CASH_COUPON: &str = "58";

const DEFAULT_DESIRE_DAYS: i64 = 7;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Order {
    pub order_id: Option<i64>,
    /// for validate
    pub cloth_ids: Option<Vec<i64>>,
    pub cloth_codes: Option<Vec<String>>,
    pub order_number: Option<String>,
    pub business_type: Option<String>,
    pub user_id: Option<i64>,
    pub price_id: Option<i64>,
    pub desire_complete_time: Option<NaiveDate>,
    pub cost_time_alarm: Option<String>,
    pub pickup_code: Option<String>,
    pub complete_time: Option<DateTime<FixedOffset>>,
    pub delivery_mode: Option<String>,
    pub source: Option<String>,
    pub status: Option<String>,
    pub payment_status: Option<String>,
    pub remark: Option<String>,
    pub order_type: Option<String>,
    pub create_time: Option<DateTime<FixedOffset>>,
    pub update_time: Option<DateTime<FixedOffset>>,

    /// Fields from sys_user
    pub nick_name: Option<String>,
    pub phonenumber: Option<String>,

    /// adjust price info
    pub adjust: Option<OrderClothAdjust>,
    // payment
    pub payment: Option<Payment>,

    pub payment_bonus_type: Option<String>,
    // coupons count,
    pub payment_bonus_count: Option<f64>,
    pub diff_price: Option<f64>,

    pub payment_amount: Option<f64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PaymentReq {
    pub payment: Option<Payment>,
    pub orders: Option<Vec<Order>>,
    pub time_based: Option<Vec<TimeBasedCoupon>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct TimeBasedCoupon {
    pub uc_id: i64,
    pub count: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefundInfoResp {
    pub user: Option<User>,
    pub payment: Option<Payment>,
    pub user_coupons: Vec<UserCoupon>, // List in Java is equivalent to Vec in Rust
}

impl FromRow<'_, SqliteRow> for Order {
    fn from_row(row: &SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        let payment = Payment::from_row(row)?;
        let order_id = row.try_get("order_id").unwrap_or_default();
        let order_number = row.try_get("order_number").unwrap_or_default();
        let business_type = row.try_get("business_type").unwrap_or_default();
        let user_id = row.try_get("user_id").unwrap_or_default();
        let price_id = row.try_get("price_id").unwrap_or_default();
        let desire_complete_time = row.try_get("desire_complete_time").unwrap_or_default();
        let cost_time_alarm = row.try_get("cost_time_alarm").unwrap_or_default();
        let pickup_code = row.try_get("pickup_code").unwrap_or_default();
        let complete_time = row.try_get("complete_time").unwrap_or_default();
        let delivery_mode = row.try_get("delivery_mode").unwrap_or_default();
        let source = row.try_get("source").unwrap_or_default();
        let status = row.try_get("status").unwrap_or_default();
        let payment_status = row.try_get("payment_status").unwrap_or_default();
        let remark = row.try_get("remark").unwrap_or_default();
        let order_type = row.try_get("order_type").unwrap_or_default();
        let create_time = row.try_get("create_time").unwrap_or_default();
        let update_time = row.try_get("update_time").unwrap_or_default();

        // user information
        let nick_name = row.try_get("nick_name").unwrap_or_default();
        let phonenumber = row.try_get("phonenumber").unwrap_or_default();

        // adjust data
        let mut adjust = None;
        let adjust_id: Option<i64> = row.try_get("adjust_id").unwrap_or_default();
        if adjust_id.is_some() {
            let adjust_value_add = row.try_get("adjust_value_add").unwrap_or_default();
            let adjust_value_sub = row.try_get("adjust_value_sub").unwrap_or_default();
            let adjust_total = row.try_get("adjust_total").unwrap_or_default();
            let adjust_remark = row.try_get("adjust_remark").unwrap_or_default();
            adjust = Some(OrderClothAdjust {
                adjust_id,
                order_id: Some(order_id),
                adjust_value_add,
                adjust_value_sub,
                adjust_total,
                remark: adjust_remark,
            })
        }
        Ok(Order {
            order_id: Some(order_id),
            order_number,
            business_type,
            user_id,
            price_id,
            desire_complete_time,
            cost_time_alarm,
            pickup_code,
            complete_time,
            delivery_mode,
            source,
            status,
            payment_status,
            remark,
            order_type,
            create_time,
            update_time,
            cloth_ids: None,
            cloth_codes: None,
            nick_name,
            phonenumber,
            adjust,
            payment: Some(payment),
            payment_bonus_type: None,
            payment_bonus_count: None,
            diff_price: None,
            payment_amount: None,
        })
    }
}

const SQL: &str = "SELECT
 o.*,
 u.nick_name,
 u.phonenumber,
 a.adjust_id,
 a.adjust_value_add,
 a.adjust_value_sub,
 a.adjust_total,
 a.remark as adjust_remark
FROM orders o
LEFT JOIN users u ON o.user_id = u.user_id
LEFT JOIN order_clothes_adjust a ON o.order_id = a.order_id
";

impl Order {
    // Insert a new Order into the database
    pub async fn create(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<Order> {
        let result = sqlx::query_as(
            "INSERT INTO orders
        (order_number, business_type, user_id, price_id, desire_complete_time, cost_time_alarm, pickup_code, complete_time, delivery_mode, source, status, payment_status, remark, order_type, create_time, update_time)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING *"
        )
            .bind(&self.order_number)
            .bind(&self.business_type)
            .bind(self.user_id)
            .bind(self.price_id)
            .bind(self.desire_complete_time)
            .bind(&self.cost_time_alarm)
            .bind(&self.pickup_code)
            .bind(self.complete_time)
            .bind(&self.delivery_mode)
            .bind(&self.source)
            .bind(&self.status)
            .bind(&self.payment_status)
            .bind(&self.remark)
            .bind(&self.order_type)
            .bind(self.create_time)
            .bind(self.update_time)
            .fetch_one(&mut **tr)
            .await?;

        Ok(result)
    }

    // Retrieve a SysOrder by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, order_id: i64) -> Result<Option<Self>> {
        let result = sqlx::query_as::<_, Order>(&format!("{SQL} WHERE o.order_id = ?"))
            .bind(order_id)
            .fetch_optional(pool)
            .await?;
        Ok(result)
    }

    // Update an existing SysOrder
    pub async fn check_pickup_code(
        pool: &Pool<Sqlite>,
        pickup_code: String,
    ) -> Result<Option<i64>> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT order_id FROM orders WHERE pickup_code = ? AND status <> '02' LIMIT 1",
        )
        .bind(pickup_code)
        .fetch_optional(pool)
        .await?;
        Ok(result)
    }

    fn apply_filters<'a>(&'a self, query_builder: &mut QueryBuilder<'a, Sqlite>) {
        self.order_number
            .as_ref()
            .filter(|n| !n.is_empty())
            .map(|n| {
                query_builder
                    .push(" AND o.order_number LIKE ")
                    .push_bind(format!("%{}%", n))
            });

        self.phonenumber
            .as_ref()
            .filter(|p| !p.is_empty())
            .map(|p| {
                query_builder
                    .push(" AND u.phonenumber LIKE ")
                    .push_bind(format!("%{}%", p))
            });

        self.business_type
            .as_ref()
            .filter(|business_type| !business_type.is_empty())
            .map(|business_type| {
                query_builder
                    .push(" AND o.business_type = ")
                    .push_bind(business_type);
            });

        self.delivery_mode
            .as_ref()
            .filter(|delivery_mode| !delivery_mode.is_empty())
            .map(|delivery_mode| {
                query_builder
                    .push(" AND o.delivery_mode = ")
                    .push_bind(delivery_mode);
            });

        if let Some(source) = &self.source {
            if !source.is_empty() {
                query_builder.push(" AND o.source = ").push_bind(source);
            }
        }

        if let Some(status) = &self.status {
            if !status.is_empty() {
                query_builder.push(" AND o.status = ").push_bind(status);
            }
        }

        self.payment_status
            .as_ref()
            .filter(|p| !p.is_empty())
            .map(|p| {
                query_builder.push(" AND o.payment_status = ").push_bind(p);
            });

        self.order_type.as_ref().filter(|t| !t.is_empty()).map(|t| {
            query_builder.push(" AND o.order_type = ").push_bind(t);
        });

        self.pickup_code
            .as_ref()
            .filter(|code| !code.is_empty())
            .map(|code| {
                query_builder
                    .push(" AND o.pickup_code LIKE ")
                    .push_bind(format!("%{}%", code));
            });

        if let Some(user_id) = &self.user_id {
            query_builder.push(" AND o.user_id = ").push_bind(user_id);
        }

        self.cost_time_alarm
            .as_ref()
            .filter(|cost_time_alarm| !cost_time_alarm.is_empty())
            .map(|cost_time_alarm| {
                query_builder
                    .push(" AND o.cost_time_alarm = ")
                    .push_bind(cost_time_alarm);
            });

        if let Some(price_id) = &self.price_id {
            query_builder.push(" AND o.price_id = ").push_bind(price_id);
        }

        self.remark
            .as_ref()
            .filter(|remark| !remark.is_empty())
            .map(|remark| {
                query_builder
                    .push(" AND o.remark LIKE ")
                    .push_bind(format!("%{}%", remark));
            });
    }

    async fn count(&self, pool: &Pool<Sqlite>) -> Result<u64> {
        let mut query_builder =
            QueryBuilder::<Sqlite>::new("SELECT COUNT(*) FROM orders o WHERE 1=1");
        self.apply_filters(&mut query_builder);
        let query = query_builder.build_query_scalar::<u64>();
        Ok(query.fetch_one(pool).await?)
    }

    async fn build_query(
        &self,
        pool: &Pool<Sqlite>,
        condition_prefix: &str,
        page_params: Option<PageParams>,
    ) -> Result<Vec<Order>> {
        let mut query_builder = QueryBuilder::<Sqlite>::new(SQL);

        query_builder.push(condition_prefix);

        self.apply_filters(&mut query_builder);

        if let Some(param) = page_params {
            query_builder.push(" LIMIT ").push_bind(param.page_size);
            query_builder
                .push(" OFFSET ")
                .push_bind((param.page - 1) * param.page_size);
        }

        let orders = query_builder
            .build_query_as::<Order>()
            .fetch_all(pool)
            .await?;
        Ok(orders)
    }

    pub async fn list(
        &self,
        pool: &Pool<Sqlite>,
        page_params: PageParams,
    ) -> Result<PageResult<Order>> {
        let rows = self
            .build_query(pool, " WHERE 1=1 ", Some(page_params))
            .await?;
        let total = self.count(pool).await?;
        Ok(PageResult { total, rows })
    }

    pub async fn list_4_home(&self, pool: &Pool<Sqlite>) -> Result<Vec<Order>> {
        self.build_query(pool, " WHERE o.status != '04' AND o.status != '05' ", None)
            .await
    }

    pub async fn update(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<bool> {
        let mut query_builder = QueryBuilder::<Sqlite>::new("UPDATE orders SET");
        let mut has_updated = false;

        // update fields by condition
        if let Some(order_number) = &self.order_number {
            query_builder
                .push(" order_number = ")
                .push_bind(order_number);
            has_updated = true;
        }

        if let Some(business_type) = &self.business_type {
            if has_updated {
                query_builder.push(", ");
            }
            query_builder
                .push(" business_type = ")
                .push_bind(business_type);
            has_updated = true;
        }

        if let Some(user_id) = &self.user_id {
            if has_updated {
                query_builder.push(", ");
            }
            query_builder.push(" user_id = ").push_bind(user_id);
            has_updated = true;
        }

        if let Some(price_id) = &self.price_id {
            if has_updated {
                query_builder.push(", ");
            }
            query_builder.push(" price_id = ").push_bind(price_id);
            has_updated = true;
        }

        if let Some(desire_complete_time) = &self.desire_complete_time {
            if has_updated {
                query_builder.push(", ");
            }
            query_builder
                .push(" desire_complete_time = ")
                .push_bind(desire_complete_time);
            has_updated = true;
        }

        if let Some(delivery_mode) = &self.delivery_mode {
            if has_updated {
                query_builder.push(", ");
            }
            query_builder
                .push(" delivery_mode = ")
                .push_bind(delivery_mode);
            has_updated = true;
        }

        if let Some(source) = &self.source {
            if has_updated {
                query_builder.push(", ");
            }
            query_builder.push(" source = ").push_bind(source);
            has_updated = true;
        }

        if let Some(status) = &self.status {
            if has_updated {
                query_builder.push(", ");
            }
            query_builder.push(" status = ").push_bind(status);
            has_updated = true;
        }

        if let Some(payment_status) = &self.payment_status {
            if has_updated {
                query_builder.push(", ");
            }
            query_builder
                .push(" payment_status = ")
                .push_bind(payment_status);
            has_updated = true;
        }

        if let Some(order_type) = &self.order_type {
            if has_updated {
                query_builder.push(", ");
            }
            query_builder.push(" order_type = ").push_bind(order_type);
            has_updated = true;
        }

        if let Some(pickup_code) = &self.pickup_code {
            if has_updated {
                query_builder.push(", ");
            }
            query_builder.push(" pickup_code = ").push_bind(pickup_code);
            has_updated = true;
        }

        if let Some(remark) = &self.remark {
            if has_updated {
                query_builder.push(", ");
            }
            query_builder.push(" remark = ").push_bind(remark);
            has_updated = true;
        }

        if has_updated {
            query_builder
                .push(" ,update_time = ")
                .push_bind(utils::get_now());
            query_builder
                .push(" WHERE order_id = ")
                .push_bind(&self.order_id);
            let result = query_builder.build().execute(&mut **tr).await?;
            return Ok(result.rows_affected() > 0);
        }
        Ok(false)
    }

    pub async fn delete_batch(tr: &mut Transaction<'_, Sqlite>, ids: &[i64]) -> Result<bool> {
        let mut builder = QueryBuilder::new("DELETE FROM orders WHERE order_id IN ( ");

        ids.iter().enumerate().for_each(|(i, id)| {
            if i > 0 {
                builder.push(", ");
            }
            builder.push_bind(id);
        });
        builder.push(")");

        let result = builder.build().execute(&mut **tr).await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn select_list_with_wait_to_pick_with_user_ids<'a>(
        pool: &Pool<Sqlite>,
        user_ids: &[i64],
    ) -> Result<Vec<Self>> {
        let mut builder =
            QueryBuilder::new(&format!("{SQL} WHERE o.status = '02' AND o.user_id IN ("));

        // 添加占位符
        user_ids.iter().enumerate().for_each(|(i, id)| {
            if i > 0 {
                builder.push(", ");
            }
            builder.push_bind(id);
        });

        // 关闭括号
        builder.push(")");

        // 执行查询
        let result = builder.build_query_as().fetch_all(pool).await?;

        Ok(result)
    }
}

const ORDER_NUMBER_PREFIX: &str = "XYFW-";

impl Order {
    pub async fn add_order(&mut self, pool: &Pool<Sqlite>) -> Result<Order> {
        // validate
        if self.cloth_ids.is_none() || self.cloth_ids.as_ref().unwrap().is_empty() {
            return Err(Error::bad_request("cloth_ids is empty"));
        }

        let now = Utc::now();
        let mut tr = pool.begin().await?;

        // get desire_complete_time from configuration
        let config = Config::get_config_by_key(pool, DESIRE_COMPLETE_TIME_KEY).await?;
        let days = config.map_or(DEFAULT_DESIRE_DAYS, |c| {
            c.config_value
                .unwrap_or(DEFAULT_DESIRE_DAYS.to_string())
                .parse::<i64>()
                .unwrap_or(DEFAULT_DESIRE_DAYS)
        });
        let desire_complete_time = now.naive_local() + chrono::Duration::days(days);
        self.desire_complete_time = Some(desire_complete_time.date());

        // gen number
        self.order_number = Some(format!("{}{}", ORDER_NUMBER_PREFIX, now.timestamp_millis()));

        self.initial();

        // insert into db
        let order = self.create(&mut tr).await?;

        // save adjust data to db
        if let Some(adjust) = self.adjust.as_mut() {
            adjust.order_id = order.order_id;
            adjust.create(&mut tr).await?;
        }

        // update clothes order_id
        if let Some(ids) = &self.cloth_ids {
            if !OrderCloth::update_order_id(&mut tr, order.order_id.unwrap_or_default(), ids)
                .await?
            {
                tr.rollback().await?;
                return Err(Error::internal("update clothes failed"));
            }
        }
        tr.commit().await?;
        Ok(order)
    }

    fn initial(&mut self) {
        self.payment_status = Some(PAY_STATUS_NOT_PAID.to_string());
        self.order_type = Some(NORMAL_ORDER.to_string());
        self.status = Some(STATUS_LAUNDRY.to_string());
        self.cost_time_alarm = Some(NORMAL_ALARM.to_string());
        self.business_type = Some(BUSINESS_MAIN.to_string());
        self.create_time = Some(utils::get_now());
    }

    pub async fn pay(pool: &Pool<Sqlite>, mut payment_req: PaymentReq) -> Result<()> {
        let mut tr = pool.begin().await?;
        // 获取多个订单
        let orders = payment_req
            .orders
            .take()
            .ok_or(Error::bad_request("orders can not be empty"))?;
        let mut is_time_based = false;

        let mut payment = payment_req
            .payment
            .take()
            // .as_mut()
            .ok_or(Error::bad_request("payment can not be empty"))?;

        // 获取支付信息和卡券信息
        let mut user_coupons: Vec<UserCoupon> = if let Some(uc_id) = &payment.uc_id {
            let ids: Vec<i64> = uc_id
                .split(',')
                .filter_map(|id| id.parse::<i64>().ok())
                .collect();
            let coupons = UserCoupon::find_by_uc_ids(pool, &ids).await?;
            if coupons.len() != ids.len() {
                return Err(Error::bad_request("卡券信息不正确，存在未入库的卡券"));
            }
            coupons
        } else if let Some(time_based) = &payment_req.time_based {
            // 如果使用了次卡
            let ids: Vec<i64> = time_based.iter().map(|t| t.uc_id).collect();
            let coupons = UserCoupon::find_by_uc_ids(pool, &ids).await?;

            if coupons.len() != ids.len() {
                return Err(Error::bad_request("卡券信息不正确，存在未入库的卡券"));
            }
            is_time_based = true;
            coupons
        } else {
            vec![]
        };

        for order in orders.iter() {
            if let Some(order_id) = order.order_id {
                // 校验订单状态
                let mut existing_order = Self::get_by_id(pool, order_id)
                    .await?
                    .ok_or(Error::bad_request("order is not exist"))?;

                if existing_order.payment_status.as_deref() == Some(PAY_STATUS_PAID) {
                    return Err(Error::bad_request("order is paid already"));
                }

                // 查询订单衣物信息
                let clothes = OrderCloth::get_by_order_id(pool, order_id).await?;

                // 计算订单总价
                let order_total_amount =
                    Self::cal_total_price(pool, &mut existing_order, &clothes).await?;

                // 应用卡券或储值卡
                if !user_coupons.is_empty() {
                    if is_time_based {
                        Self::validate_time_based_on_payment(
                            &mut tr,
                            &mut payment_req,
                            &mut user_coupons,
                            &clothes,
                        )
                        .await?;
                    } else {
                        Self::validate_and_apply_coupons(
                            &mut tr,
                            &mut payment,
                            &mut user_coupons,
                            order_total_amount,
                        )
                        .await?;
                        // if !is_storage_card && !user_coupons.is_empty() {
                        //     return Err(Error::bad_request("存在不符合使用规则的优惠券"));
                        // }
                    }
                }

                // 如果所有衣物状态为 "已取件"，更新订单状态为 "已完成"
                if clothes.iter().all(|cloth| {
                    cloth.clothing_status.as_deref() == Some(CLOTHING_STATUS_PICKED_UP)
                }) {
                    existing_order.status = Some(STATUS_COMPLETED.to_string());
                }

                // update order payment status to PAID
                existing_order.payment_status = Some(PAY_STATUS_PAID.to_string());
                if !existing_order.update(&mut tr).await? {
                    return Err(Error::internal("update order failed"));
                }

                // create payment record
                payment.order_status = Some("01".to_string());
                payment.payment_status = Some("01".to_string());
                payment.pay_number = existing_order.order_number;
                payment.create_payment(&mut tr).await?;
            }
        }

        if let Some(order) = orders.first() {
            if !User::increase_points(
                &mut tr,
                order.user_id.unwrap_or_default(),
                payment.payment_amount.unwrap_or_default() as i64,
            )
            .await?
            {
                return Err(Error::internal("更新用户积分失败"));
            }
        }
        tr.commit().await?;
        Ok(())
    }

    async fn validate_and_apply_coupons(
        tr: &mut Transaction<'_, Sqlite>,
        payment: &mut Payment,
        user_coupons: &mut Vec<UserCoupon>,
        mut total_amount: f64,
    ) -> Result<bool> {
        if user_coupons.is_empty() {
            return Ok(true);
        }

        // 校验卡券是否过期
        for coupon in user_coupons.iter() {
            let coupon = coupon
                .coupon
                .as_ref()
                .ok_or(Error::internal("get coupon failed"))?;
            if Some(utils::get_now()) > coupon.valid_to {
                return Err(Error::bad_request(format!(
                    "Coupon {} is expired",
                    coupon.coupon_title.clone().unwrap_or_default()
                )));
            }
        }

        // 校验卡券类型一致性
        let (storage_cards, other_coupons): (Vec<_>, Vec<_>) =
            user_coupons.iter_mut().partition(|coupon| {
                coupon.coupon.as_ref().map_or(false, |c| {
                    c.coupon_type.as_deref() == Some(STORAGE_CARD_NUMBER)
                })
            });

        if !storage_cards.is_empty() && !other_coupons.is_empty() {
            return Err(Error::bad_request(
                "Cannot combine storage cards with other coupon types",
            ));
        }

        // 储值卡处理逻辑
        if !storage_cards.is_empty() {
            // 按余额升序排序储值卡
            let mut sorted_storage_cards = storage_cards
                .into_iter()
                .filter(|card| card.available_value.unwrap_or(0.0) > 0.0)
                .collect::<Vec<_>>();
            sorted_storage_cards
                .sort_by(|a, b| a.available_value.partial_cmp(&b.available_value).unwrap());

            for storage_card in sorted_storage_cards {
                let available_value = storage_card.available_value.unwrap_or(0.0);

                if available_value >= total_amount {
                    // 使用储值卡支付足够金额
                    storage_card.available_value = Some(available_value - total_amount);
                    payment.payment_method = Some("06".to_string());
                    payment.payment_amount_vip = Some(total_amount);
                    // update user coupon
                    if !storage_card.update(tr).await? {
                        return Err(Error::internal("update user coupon failed"));
                    }
                    return Ok(true);
                } else {
                    // 使用储值卡支付部分金额
                    total_amount -= available_value;
                    storage_card.available_value = Some(0.0);
                    // update user coupon
                    if !storage_card.update(tr).await? {
                        return Err(Error::internal("update user coupon failed"));
                    }
                }
            }
            return Ok(true);
        }

        // 校验非储值卡优惠券数量
        if other_coupons.len() > 1 {
            return Err(Error::bad_request("Only one coupon can be used at a time"));
        }

        // 处理单张优惠券
        if let Some(user_coupon) = other_coupons.into_iter().next() {
            let coupon = user_coupon
                .coupon
                .as_ref()
                .ok_or(Error::internal("Invalid coupon data"))?;

            if coupon.coupon_type.as_deref() == Some(OFF_CARD_NUMBER)
                || coupon.coupon_type.as_deref() == Some(SUB_CARD_NUMBER)
            {
                // sub count
                let uc_count = user_coupon
                    .uc_count
                    .as_mut()
                    .ok_or(Error::internal("get user coupon count failed"))?;
                if *uc_count > 0 {
                    *uc_count -= 1;
                } else {
                    return Err(Error::bad_request("coupon count is not enough"));
                }

                // 校验卡券最低消费
                if let Some(min_spend) = coupon.min_spend {
                    if total_amount < min_spend {
                        return Err(Error::bad_request(format!(
                            "Minimum spend not met for coupon {}",
                            coupon.coupon_title.clone().unwrap_or_default()
                        )));
                    }
                }

                // 计算优惠金额
                let total_amount_decimal = Decimal::from_f64(total_amount).unwrap_or_default();
                let result = if coupon.coupon_type.as_deref() == Some(OFF_CARD_NUMBER) {
                    if coupon.usage_value.is_none() || coupon.usage_limit.is_none() {
                        return Err(Error::internal("get coupon usage value failed"));
                    }
                    // 折扣券逻辑
                    let discount = Decimal::from_f64(1.0).unwrap_or_default()
                        - Decimal::from_f64(coupon.usage_value.unwrap_or_default())
                            .unwrap_or_default()
                            / Decimal::from_f64(100.0).unwrap_or_default();
                    let discounted = (total_amount_decimal * discount)
                        .round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero);
                    let usage_limit = Decimal::from_f64(coupon.usage_limit.unwrap_or_default())
                        .unwrap_or_default();
                    let result = if discounted > usage_limit {
                        usage_limit
                    } else {
                        discounted
                    };
                    total_amount_decimal - result
                } else {
                    // 满减券逻辑
                    let usage_value = Decimal::from_f64(coupon.usage_value.unwrap_or_default())
                        .unwrap_or_default();
                    (total_amount_decimal - usage_value)
                        .round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero)
                };

                // 更新金额和校验逻辑
                total_amount = result.to_f64().unwrap_or_default();
                payment.payment_amount_vip =
                    Some((total_amount_decimal - result).to_f64().unwrap_or_default());

                // 校验最终金额
                if (total_amount - payment.payment_amount.unwrap_or_default()).abs() > f64::EPSILON
                {
                    return Err(Error::internal(
                        "提交的最终金额(优惠后的金额)与订单最终金额(优惠后的金额)不一致",
                    ));
                }

                // update user coupon
                if !user_coupon.update(tr).await? {
                    return Err(Error::internal("update user coupon failed"));
                }
            }
        }

        Ok(true)
    }

    async fn validate_time_based_on_payment(
        tr: &mut Transaction<'_, Sqlite>,
        payment: &mut PaymentReq,
        user_coupons: &mut Vec<UserCoupon>,
        clothes: &[OrderCloth],
    ) -> Result<()> {
        // Get cloth count
        let mut cloth_count = clothes.len();
        let mut used_coupon_ids = Vec::with_capacity(cloth_count);
        let mut total_used_coupons = 0;

        // Map `uc_id` to the corresponding `TimeBasedCoupon`
        if let Some(time_based) = payment.time_based.as_mut() {
            for coupon in user_coupons.iter_mut() {
                if let Some(uc_id) = coupon.uc_id {
                    // Get corresponding `TimeBasedCoupon`
                    let time_based_coupon = time_based
                        .iter_mut()
                        .filter(|t| t.uc_id == uc_id)
                        .next()
                        .ok_or(Error::internal("time based card information error"))?;
                    let remaining_uses = coupon.available_value.unwrap_or_default() as usize; // Remaining uses in the system
                    let requested_uses = time_based_coupon.count as usize; // Uses requested by the user

                    // Ensure user does not request more uses than available
                    let usable_count = remaining_uses.min(requested_uses);

                    if usable_count == 0 {
                        continue;
                    }

                    if usable_count >= cloth_count {
                        // If the coupon can cover all clothes, deduct its value and update the database
                        coupon.available_value =
                            coupon.available_value.map(|v| v - cloth_count as f64);
                        time_based_coupon.count -= cloth_count as i32;

                        // Update the coupon in the database
                        if !coupon.update(tr).await? {
                            return Err(Error::internal("update user coupon failed"));
                        }

                        used_coupon_ids.push(uc_id);
                        total_used_coupons += cloth_count;

                        break;
                    } else {
                        // If the coupon cannot cover all clothes, use it up and continue to the next coupon
                        cloth_count -= usable_count;
                        coupon.available_value = Some(0f64); // This coupon is fully consumed
                        time_based_coupon.count -= usable_count as i32;

                        // Update the coupon in the database
                        if !coupon.update(tr).await? {
                            return Err(Error::internal("update user coupon failed"));
                        }
                        used_coupon_ids.push(uc_id);
                        total_used_coupons += usable_count;
                    }
                }
            }

            if cloth_count > 0 {
                return Err(Error::bad_request("not enough coupon"));
            }

            // Update the `PaymentReq` object
            if let Some(payment) = payment.payment.as_mut() {
                payment.uc_id = Some(
                    used_coupon_ids
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(","),
                );
                payment.payment_amount_vip = Some(total_used_coupons as f64);
            }
        }
        Ok(())
    }

    fn cal_payment_method(order: &mut Order, payment: &mut Payment) {
        let payment_method = payment.payment_method.clone();

        if Some(PAYMENT_METHOD_COMBINATION_ALIPAY_COUPON) == payment_method.as_deref()
            || Some(PAYMENT_METHOD_COMBINATION_CASH_COUPON) == payment_method.as_deref()
            || Some(PAYMENT_METHOD_COMBINATION_WECHAT_COUPON) == payment_method.as_deref()
        {
            order.payment_bonus_count = Some(payment.payment_amount_vip.unwrap_or_default());
            order.diff_price = Some(payment.payment_amount.unwrap_or_default());
        } else if Some(PAYMENT_METHOD_MEITUAN) == payment_method.as_deref()
            || Some(PAYMENT_METHOD_DOUYIN) == payment_method.as_deref()
        {
        } else {
            order.payment_bonus_count = Some(payment.payment_amount_vip.unwrap_or_default());
            order.diff_price = Some(payment.payment_amount.unwrap_or_default());
        }

        order.payment_bonus_type = payment_method;

        order.payment_amount = payment.total_amount;
    }

    async fn cal_total_price(
        pool: &Pool<Sqlite>,
        order: &mut Order,
        clothes: &[OrderCloth],
    ) -> Result<f64> {
        // calculate total price by clothes
        let mut price = clothes
            .iter()
            .map(|cloth| {
                if let Some(mut base_price) = cloth.price_value {
                    base_price += cloth.process_markup.unwrap_or_default();

                    if Some(SERVICE_TYPE_EMERGENCY) == cloth.service_type.as_deref() {
                        Decimal::from_f64(base_price).unwrap_or_default() * dec!(2.0)
                    } else if Some(SERVICE_TYPE_SINGLE_WASH) == cloth.service_type.as_deref() {
                        Decimal::from_f64(base_price).unwrap_or_default() * dec!(1.5)
                    } else {
                        Decimal::from_f64(base_price).unwrap_or_default()
                    }
                } else {
                    Decimal::ZERO
                }
            })
            .sum::<Decimal>();

        if let Some(price_id) = order.price_id {
            // query price obj
            let cloth_price =
                ClothPrice::get_by_id(pool, price_id)
                    .await?
                    .ok_or(Error::with_details(
                        ErrorKind::NotFound,
                        "cloth price not found",
                    ))?;

            if let Some(price_value) = cloth_price.price_value {
                price = Decimal::from_f64(price_value).unwrap_or_default();
            } else if let Some(price_discount) = cloth_price.price_discount {
                // Apply discount to price
                let discount = Decimal::from_f64(price_discount).unwrap_or_default() / dec!(100);
                let discount_value = price * discount;
                price -= discount_value.round_dp(2);
            } else {
                return Err(Error::internal("Price tag configuration error"));
            }
        }

        // Process adjustments
        if let Some(adjust) = &order.adjust {
            if let Some(adjust_total) = adjust.adjust_total {
                price = Decimal::from_f64(adjust_total).unwrap_or_default();
            } else {
                price += Decimal::from_f64(adjust.adjust_value_add.unwrap_or_default())
                    .unwrap_or_default();
                price -= Decimal::from_f64(adjust.adjust_value_sub.unwrap_or_default())
                    .unwrap_or_default();
            }
        }

        // Ensure the price is non-negative
        Ok(price.to_f64().unwrap_or_default().max(0.0))
    }

    pub async fn query_list(
        &self,
        pool: &Pool<Sqlite>,
        page_params: PageParams,
    ) -> Result<PageResult<Order>> {
        let mut result = self.list(pool, page_params).await?;
        // combine payment and clothes information
        for mut order in result.rows.iter_mut() {
            if let Some(id) = order.order_id {
                // query clothes by order id
                let clothes = OrderCloth::get_by_order_id(pool, id).await?;

                // extract cloth codes
                let cloth_codes: Vec<String> = clothes
                    .iter()
                    .filter_map(|c| c.hang_cloth_code.clone())
                    .collect();
                order.cloth_codes = Some(cloth_codes);
                if let Some(mut payment) = Payment::get_by_order_id(pool, id).await? {
                    Self::cal_payment_method(&mut order, &mut payment);
                } else {
                    // calculate total
                    order.payment_amount =
                        Some(Self::cal_total_price(pool, &mut order, &clothes).await?);
                }
            }
        }

        Ok(result)
    }

    pub async fn query_list4home(&self, pool: &Pool<Sqlite>) -> Result<Vec<Self>> {
        let mut list = self.list_4_home(pool).await?;

        for order in list.iter_mut() {
            order.payment = Payment::get_by_order_id(pool, order.order_id.unwrap()).await?;
        }

        Ok(list)
    }

    pub async fn refund(pool: &Pool<Sqlite>, exp: Expenditure) -> Result<()> {
        if exp.order_id.is_none() {
            return Err(Error::bad_request("order_id is required"));
        }
        let mut order = Order::get_by_id(pool, exp.order_id.unwrap())
            .await?
            .ok_or(Error::not_found("order not found"))?;

        // check order's payment status
        if let Some(status) = &order.payment_status {
            if status == PAY_STATUS_REFUND {
                return Err(Error::bad_request("订单已经退单，请勿重复退单"));
            }
        }

        // update order status to refund
        order.payment_status = Some(PAY_STATUS_REFUND.to_string());
        // update order complete time
        order.complete_time = Some(utils::get_now());

        let mut tx = pool.begin().await?;

        // update clothes status to refund
        if !OrderCloth::update_status_by_order_id(
            &mut tx,
            order.order_id.unwrap(),
            CLOTH_STATUS_REFUND,
        )
        .await?
        {
            return Err(Error::internal("update clothes status failed"));
        }

        // select payment record
        let payment = Payment::get_by_order_id(pool, order.order_id.unwrap()).await?;
        if payment.is_none() {
            order.status = Some(PAY_STATUS_REFUND.to_string());
            if !order.update(&mut tx).await? {
                return Err(Error::internal("update order failed"));
            }
            tx.commit().await?;
            return Ok(());
        } else {
            order.status = Some(ORDER_STATUS_REFUND.to_string());
            if !order.update(&mut tx).await? {
                return Err(Error::internal("update order failed"));
            }
        }

        let payment = payment.unwrap();
        // generate expenditure recorde
        if payment.payment_amount_mv.is_some() && payment.payment_amount_mv.unwrap() > 0. {
            exp.create(&mut tx).await?;
        }

        // check if user used coupon
        let uc_id = &payment.uc_id;
        if uc_id.is_none() {
            tx.commit().await?;
            return Ok(());
        }

        // get the uc id list
        let uc_ids: Vec<i64> = uc_id
            .clone()
            .unwrap()
            .split(",")
            .map(|s| s.parse::<i64>().unwrap_or_default())
            .collect();

        // query coupon information
        let mut user_coupons = UserCoupon::find_by_uc_ids(pool, &uc_ids).await?;
        if user_coupons.len() != uc_ids.len() {
            return Err(Error::internal("卡券信息不正确，无法退还"));
        }

        // Check if there are storage card type coupons
        let is_type_000 = user_coupons.iter().any(|coupon| {
            coupon.coupon.is_some()
                && coupon.coupon.as_ref().unwrap().coupon_type.as_deref()
                    == Some(STORAGE_CARD_NUMBER)
        });

        if is_type_000 {
            // Sort storage cards by available value (ascending)
            let mut storage_cards: Vec<_> = user_coupons
                .iter_mut()
                .filter(|coupon| {
                    coupon.coupon.is_some()
                        && coupon.coupon.as_ref().unwrap().coupon_type.as_deref()
                            == Some(STORAGE_CARD_NUMBER)
                })
                .collect();

            storage_cards.sort_by(|a, b| {
                a.available_value
                    .partial_cmp(&b.available_value)
                    .unwrap_or(Ordering::Equal)
            });

            // Total amount used in this order
            let total_amount = payment.payment_amount_vip;
            if total_amount.is_none() {
                tx.commit().await?;
                return Ok(());
            }
            let mut total_amount = total_amount.unwrap();

            for user_coupon in storage_cards {
                if let Some(coupon) = &user_coupon.coupon {
                    // Original total value of the storage card
                    // todo check the uc count is correct
                    let old_value = coupon.usage_value.unwrap_or_default()
                        * user_coupon.uc_count.unwrap_or_default() as f64;
                    // Amount used in this order for this storage card
                    let used_value = old_value - user_coupon.available_value.unwrap_or_default();

                    if total_amount > used_value {
                        // Refund the full value
                        user_coupon.available_value = Some(old_value);
                        if !user_coupon.update(&mut tx).await? {
                            return Err(Error::internal("退还储值卡失败"));
                        }

                        total_amount -= used_value;
                    } else {
                        // Refund only part of the value
                        user_coupon.available_value =
                            user_coupon.available_value.map(|v| v + total_amount);
                        if !user_coupon.update(&mut tx).await? {
                            return Err(Error::internal("退还储值卡失败"));
                        }
                        break;
                    }
                }
            }
        } else {
            if let Some(user_coupon) = user_coupons.first_mut() {
                // Refund usage count for non-storage card coupons
                user_coupon.uc_count = user_coupon.uc_count.map(|c| c + 1);
                if !user_coupon.update(&mut tx).await? {
                    return Err(Error::internal("退还优惠券失败"));
                }
            }
        }

        // refund user points
        if payment.payment_amount.is_some() && payment.payment_amount.unwrap() > 0. {
            if !User::decrease_points(
                &mut tx,
                order.user_id.unwrap_or_default(),
                payment.payment_amount.unwrap_or_default() as i64,
            )
            .await?
            {
                return Err(Error::internal("退还积分失败"));
            }
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn get_refund_info(
        pool: &Pool<Sqlite>,
        order_id: i64,
        user_id: i64,
    ) -> Result<RefundInfoResp> {
        let mut resp = RefundInfoResp::default();
        // query user information
        let user = User::get_by_id(pool, user_id)
            .await?
            .ok_or(Error::not_found("用户不存在"))?;

        resp.user = Some(user);

        // query payment information by order id
        let payment = Payment::get_by_order_id(pool, order_id).await?;
        if payment.is_none() {
            return Ok(resp);
        }

        let payment = payment.unwrap();
        let mut user_coupons = vec![];
        // query used coupons in this payment
        if let Some(uc_id) = &payment.uc_id {
            let uc_ids: Vec<i64> = uc_id
                .split(",")
                .map(|s| s.parse::<i64>().unwrap_or_default())
                .collect();
            user_coupons = UserCoupon::find_by_uc_ids(pool, &uc_ids).await?;
        }

        resp.payment = Some(payment);
        resp.user_coupons = user_coupons;

        Ok(resp)
    }

    pub async fn delete_orders(pool: &Pool<Sqlite>, ids: &[i64]) -> Result<()> {
        let mut tx = pool.begin().await?;
        for order_id in ids {
            // delete related order clothes
            let cloth_ids: Vec<i64> = OrderCloth::get_by_order_id(pool, *order_id)
                .await?
                .into_iter()
                .filter_map(|c| c.cloth_id)
                .collect();
            OrderCloth::delete_batch(&mut tx, &cloth_ids).await?;

            // delete adjust-price data
            if !OrderClothAdjust::delete(&mut tx, *order_id).await? {
                return Err(Error::internal("删除订单调整价格数据失败"));
            }
        }

        // delete order by order ids
        if !Order::delete_batch(&mut tx, ids).await? {
            return Err(Error::internal("删除订单数据失败"));
        }

        tx.commit().await?;

        Ok(())
    }
}

#[tauri::command]
pub async fn create_order(state: tauri::State<'_, AppState>, mut order: Order) -> Result<Order> {
    Ok(order.add_order(&state.0).await?)
}

#[tauri::command]
pub async fn get_orders_pagination(
    state: tauri::State<'_, AppState>,
    page_params: PageParams,
    order: Order,
) -> Result<PageResult<Order>> {
    order.query_list(&state.0, page_params).await
}

#[tauri::command]
pub async fn get_orders4home(
    state: tauri::State<'_, AppState>,
    order: Order,
) -> Result<Vec<Order>> {
    order.query_list4home(&state.0).await
}

#[tauri::command]
pub async fn get_order_by_id(state: tauri::State<'_, AppState>, id: i64) -> Result<Option<Order>> {
    Order::get_by_id(&state.0, id).await
}

#[tauri::command]
pub async fn update_order(state: tauri::State<'_, AppState>, order: Order) -> Result<bool> {
    let mut tr = state.0.begin().await?;

    let result = order.update(&mut tr).await?;

    tr.commit().await?;

    Ok(result)
}

#[tauri::command]
pub async fn delete_orders(state: tauri::State<'_, AppState>, ids: Vec<i64>) -> Result<()> {
    Order::delete_orders(&state.0, &ids).await
}

/// update adjust data
#[tauri::command]
pub async fn update_adjust(state: tauri::State<'_, AppState>, order: Order) -> Result<bool> {
    if let Some(adjust) = order.adjust {
        if !adjust.upsert(&state.0).await? {
            return Err(Error::with_details(
                ErrorKind::InternalServer,
                "update adjust data failed",
            ));
        }
    }

    Ok(true)
}

#[tauri::command]
pub async fn pay_order(state: tauri::State<'_, AppState>, req: PaymentReq) -> Result<()> {
    Order::pay(&state.0, req).await
}

#[tauri::command]
pub async fn get_refund_info(
    state: tauri::State<'_, AppState>,
    order_id: i64,
    user_id: i64,
) -> Result<RefundInfoResp> {
    Order::get_refund_info(&state.0, order_id, user_id).await
}

#[tauri::command]
pub async fn refund_order(state: tauri::State<'_, AppState>, exp: Expenditure) -> Result<()> {
    Order::refund(&state.0, exp).await
}
