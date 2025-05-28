use std::cmp::Ordering;

use chrono::Local;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{
    FromRow, Pool, QueryBuilder, Row, Sqlite, Transaction,
    types::chrono::{DateTime, FixedOffset, NaiveDate, Utc},
};
use tauri::{AppHandle, Manager, Runtime};

use crate::constants::{
    ClothStatus, CouponType, OrderStatus, PaymentMethod, PaymentStatus, ServiceRequirmentType,
};
use crate::db::adjust_price::OrderClothAdjust;
use crate::db::cloth_price::ClothPrice;
use crate::db::configs::Config;
use crate::db::order_clothes::OrderCloth;
use crate::db::payments::Payment;
use crate::db::user::User;
use crate::db::user_coupons::UserCoupon;
use crate::db::{Curd, PageParams, PageResult};
use crate::error::{Error, ErrorKind, Result};
use crate::qrcode_payments::QrcodePayment;
use crate::state::AppState;
use crate::utils;
use crate::utils::chrono_serde::deserialize_date;
use crate::utils::request::Request;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

// const PAY_STATUS_NOT_PAID: &str = "01";
// const PAY_STATUS_REFUND: &str = "05";
// const ORDER_STATUS_REFUND: &str = "06";
// const CLOTH_STATUS_REFUND: &str = "03";
const PAY_STATUS_PAID: &str = "00";
const NORMAL_ORDER: &str = "00";
// const CLOTHING_STATUS_PICKED_UP: &str = "00";
// const REWASH_ORDER: &str = "02";
// const STATUS_LAUNDRY: &str = "01";
// const STATUS_COMPLETED: &str = "04";
const NORMAL_ALARM: &str = "00";
const WARNING_ALARM: &str = "01";
const OVERDUE_ALARM: &str = "02";
const BUSINESS_MAIN: &str = "00";
const DESIRE_COMPLETE_TIME_KEY: &str = "desire_complete_time";
// const STORAGE_CARD_NUMBER: &str = "000";
// const ONCE_CARD_NUMBER: &str = "002";
// const OFF_CARD_NUMBER: &str = "003";
// const SUB_CARD_NUMBER: &str = "004";
// const DISCOUNT_CARD_NUMBER: &str = "005";
// const SERVICE_TYPE_EMERGENCY: &str = "001";
// const SERVICE_TYPE_SINGLE_WASH: &str = "002";

// const PAYMENT_METHOD_TIME_BASED: &str = "07";
// const PAYMENT_METHOD_MEITUAN: &str = "03";
// const PAYMENT_METHOD_DOUYIN: &str = "04";
// const PAYMENT_METHOD_DOCOUPON: &str = "06";
// const PAYMENT_METHOD_DISCOUNT: &str = "08";
// const PAYMENT_METHOD_COMBINATION_ALIPAY_COUPON: &str = "16";
// const PAYMENT_METHOD_COMBINATION_WECHAT_COUPON: &str = "26";
// const PAYMENT_METHOD_COMBINATION_CASH_COUPON: &str = "56";

const DEFAULT_DESIRE_DAYS: i64 = 7;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Order {
    pub order_id: Option<i64>,
    /// for validate
    pub cloth_ids: Option<Vec<String>>,
    pub cloth_codes: Option<Vec<String>>,
    pub order_number: Option<String>,
    pub business_type: Option<String>,
    pub user_id: Option<i64>,
    pub price_ids: Option<Vec<i64>>,
    pub desire_complete_time: Option<NaiveDate>,
    pub cost_time_alarm: Option<String>,
    pub pickup_code: Option<String>,
    pub complete_time: Option<DateTime<FixedOffset>>,
    pub delivery_mode: Option<String>,
    pub source: Option<String>,
    pub status: Option<OrderStatus>,
    pub payment_status: Option<PaymentStatus>,
    pub remark: Option<String>,
    pub order_type: Option<String>,
    pub create_time: Option<DateTime<FixedOffset>>,
    pub update_time: Option<DateTime<FixedOffset>>,
    pub store_id: Option<i64>, // 商家ID，用于数据隔离

    /// Fields from sys_user
    pub nick_name: Option<String>,
    pub phonenumber: Option<String>,

    /// adjust price info
    pub adjust: Option<OrderClothAdjust>,
    // payment
    pub payment: Option<Payment>,

    pub payment_bonus_type: Option<PaymentMethod>,
    // coupons count,
    pub payment_bonus_count: Option<f64>,
    pub diff_price: Option<f64>,

    pub payment_amount: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OrderFromServer {
    pub order: Order,
    pub order_clothes: Vec<OrderCloth>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub enum PaymentReqMethod {
    #[default]
    Alipay,
    Wechat,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PaymentReq {
    pub payment: Option<Payment>,
    pub orders: Option<Vec<Order>>,
    pub time_based: Option<Vec<TimeBasedCoupon>>,
    // 扫码支付相关字段
    pub auth_code: Option<String>,              // 支付授权码
    pub store_id: Option<i64>,                  // 商家ID
    pub subject: Option<String>,                // 订单标题
    pub payment_type: Option<PaymentReqMethod>, // 支付类型：alipay/wechat
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
        // 创建Payment对象，使用别名字段
        let payment = Payment {
            pay_id: row.try_get("pay_id").unwrap_or_default(),
            pay_number: row.try_get("pay_number").unwrap_or_default(),
            order_type: row.try_get("p_order_type").unwrap_or_default(),
            total_amount: row.try_get("total_amount").unwrap_or_default(),
            payment_amount: row.try_get("payment_amount").unwrap_or_default(),
            payment_amount_vip: row.try_get("payment_amount_vip").unwrap_or_default(),
            payment_amount_mv: row.try_get("payment_amount_mv").unwrap_or_default(),
            payment_status: row.try_get("p_payment_status").unwrap_or_default(),
            payment_method: row.try_get("payment_method").unwrap_or_default(),
            transaction_id: row.try_get("transaction_id").unwrap_or_default(),
            uc_order_id: row.try_get("uc_order_id").unwrap_or_default(),
            uc_id: row.try_get("uc_id").unwrap_or_default(),
            order_status: row.try_get("order_status").unwrap_or_default(),
            create_time: row.try_get("p_create_time").unwrap_or_default(),
            update_time: row.try_get("p_update_time").unwrap_or_default(),
            store_id: row.try_get("p_store_id").unwrap_or_default(),
            refund_reason: row.try_get("refund_reason").unwrap_or_default(),
        };

        let order_id = row.try_get("order_id").unwrap_or_default();

        // adjust data
        let mut adjust = None;
        let adjust_id: Option<i64> = row.try_get("adjust_id").unwrap_or_default();
        if adjust_id.is_some() {
            adjust = Some(OrderClothAdjust {
                adjust_id,
                order_id,
                adjust_value_add: row.try_get("adjust_value_add").unwrap_or_default(),
                adjust_value_sub: row.try_get("adjust_value_sub").unwrap_or_default(),
                adjust_total: row.try_get("adjust_total").unwrap_or_default(),
                remark: row.try_get("adjust_remark").unwrap_or_default(),
            })
        }

        // Parse price_ids from the concatenated string
        let price_ids: Option<String> = row.try_get("price_ids").unwrap_or_default();
        let price_ids = price_ids.map(|s| {
            s.split(',')
                .filter_map(|id| id.parse::<i64>().ok())
                .collect::<Vec<i64>>()
        });

        Ok(Order {
            order_id,
            store_id: row.try_get("store_id").unwrap_or_default(),
            order_number: row.try_get("order_number").unwrap_or_default(),
            business_type: row.try_get("business_type").unwrap_or_default(),
            user_id: row.try_get("user_id").unwrap_or_default(),
            price_ids,
            desire_complete_time: row.try_get("desire_complete_time").unwrap_or_default(),
            cost_time_alarm: row.try_get("cost_time_alarm").unwrap_or_default(),
            pickup_code: row.try_get("pickup_code").unwrap_or_default(),
            complete_time: row.try_get("complete_time").unwrap_or_default(),
            delivery_mode: row.try_get("delivery_mode").unwrap_or_default(),
            source: row.try_get("source").unwrap_or_default(),
            status: row.try_get("status").unwrap_or_default(),
            payment_status: row.try_get("payment_status").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
            order_type: row.try_get("order_type").unwrap_or_default(),
            create_time: row.try_get("create_time").unwrap_or_default(),
            update_time: row.try_get("update_time").unwrap_or_default(),
            nick_name: row.try_get("nick_name").unwrap_or_default(),
            phonenumber: row.try_get("phonenumber").unwrap_or_default(),
            adjust,
            payment: Some(payment),
            cloth_ids: None,
            cloth_codes: None,
            payment_bonus_type: None,
            payment_bonus_count: None,
            diff_price: None,
            payment_amount: None,
        })
    }
}

const SQL: &str = "SELECT
 o.*,
 GROUP_CONCAT(opr.price_id) as price_ids,
 p.pay_id,
 p.pay_number,
 p.order_type as p_order_type,
 p.total_amount,
 p.payment_amount,
 p.payment_amount_vip,
 p.payment_amount_mv,
 p.payment_status as p_payment_status,
 p.payment_method,
 p.transaction_id,
 p.uc_order_id,
 p.uc_id,
 p.order_status,
 p.create_time as p_create_time,
 p.update_time as p_update_time,
 p.store_id as p_store_id,
 p.refund_reason,
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
LEFT JOIN payments p ON o.order_id = p.uc_order_id
LEFT JOIN order_price_relations opr ON o.order_id = opr.order_id";

const SQL_BY_CLOTHING_NAME: &str = "SELECT
    o.*,
    GROUP_CONCAT(opr.price_id) as price_ids,
    p.pay_id,
    p.pay_number,
    p.order_type as p_order_type,
    p.total_amount,
    p.payment_amount,
    p.payment_amount_vip,
    p.payment_amount_mv,
    p.payment_status as p_payment_status,
    p.payment_method,
    p.transaction_id,
    p.uc_order_id,
    p.uc_id,
    p.order_status,
    p.create_time as p_create_time,
    p.update_time as p_update_time,
    p.store_id as p_store_id,
    p.refund_reason,
    u.nick_name,
    u.phonenumber,
    a.adjust_id,
    a.adjust_value_add,
    a.adjust_value_sub,
    a.adjust_total,
    a.remark as adjust_remark
FROM orders o
    INNER JOIN users u ON o.user_id = u.user_id
    LEFT JOIN order_clothes_adjust a ON o.order_id = a.order_id
    LEFT JOIN payments p ON o.order_id = p.uc_order_id
    INNER JOIN order_clothes oc ON o.order_id = oc.order_id
    INNER JOIN clothing c ON oc.clothing_id = c.id 
    LEFT JOIN order_price_relations opr ON o.order_id = opr.order_id";

impl Order {
    // Insert a new Order into the database
    pub async fn create(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<Self> {
        let result: Self = sqlx::query_as(
            "INSERT INTO orders
        (order_id, order_number, business_type, store_id, user_id, desire_complete_time, cost_time_alarm,
         pickup_code, complete_time, delivery_mode, source, status, payment_status,
         remark, order_type, create_time, update_time)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING *",
        )
        .bind(&self.order_id)
        .bind(&self.order_number)
        .bind(&self.business_type)
        .bind(self.store_id)
        .bind(self.user_id)
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

        // Insert price relations
        if let Some(price_ids) = &self.price_ids {
            for price_id in price_ids {
                sqlx::query(
                    "INSERT INTO order_price_relations (order_id, price_id, create_time)
                    VALUES (?, ?, ?)",
                )
                .bind(result.order_id)
                .bind(price_id)
                .bind(utils::get_now())
                .execute(&mut **tr)
                .await?;
            }
        }

        Ok(result)
    }

    // Retrieve a SysOrder by ID
    pub async fn get_by_id(
        pool: &Pool<Sqlite>,
        store_id: i64,
        order_id: i64,
    ) -> Result<Option<Self>> {
        let result = sqlx::query_as::<_, Order>(&format!(
            "{SQL} WHERE o.store_id = ? AND o.order_id = ? GROUP BY o.order_id"
        ))
        .bind(store_id)
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
        self.store_id
            .map(|n| query_builder.push(" AND o.store_id = ").push_bind(n));

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
            query_builder.push(" AND o.status = ").push_bind(status);
        }

        self.payment_status.as_ref().map(|p| {
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

        // if let Some(price_ids) = &self.price_ids {
        //     query_builder.push(" AND o.price_ids IN (").push_bind(price_ids.join(",")).push(")");
        // }

        self.nick_name
            .as_ref()
            .filter(|nick_name| !nick_name.is_empty())
            .map(|nick_name| {
                query_builder
                    .push(" AND u.nick_name LIKE ")
                    .push_bind(format!("%{}%", nick_name));
            });

        self.remark
            .as_ref()
            .filter(|remark| !remark.is_empty())
            .map(|remark| {
                query_builder
                    .push(" AND o.remark LIKE ")
                    .push_bind(format!("%{}%", remark));
            });
    }

    async fn count(&self, pool: &Pool<Sqlite>, page_params: Option<PageParams>) -> Result<u64> {
        let mut builder = QueryBuilder::new(
            "SELECT COUNT(*) FROM orders o
                    LEFT JOIN users u ON o.user_id = u.user_id
                    LEFT JOIN order_clothes_adjust a ON o.order_id = a.order_id WHERE 1=1",
        );
        self.apply_filters(&mut builder);
        if let Some(page_params) = &page_params {
            if let Some(param) = &page_params.params {
                if let Some(begin_time) = param.get("startTime") {
                    builder.push(" AND strftime('%Y%m%d', o.create_time) >= strftime('%Y%m%d', ");
                    builder.push_bind(begin_time);
                    builder.push(") ");
                }
            }
        }

        if let Some(page_params) = &page_params {
            if let Some(param) = &page_params.params {
                if let Some(end_time) = param.get("endTime") {
                    builder.push(" AND strftime('%Y%m%d', o.create_time) <= strftime('%Y%m%d', ");
                    builder.push_bind(end_time);
                    builder.push(") ");
                }
            }
        }
        let query = builder.build_query_scalar::<u64>();
        Ok(query.fetch_optional(pool).await?.unwrap_or_default())
    }

    async fn build_query(
        &self,
        pool: &Pool<Sqlite>,
        condition_prefix: &str,
        page_params: Option<PageParams>,
    ) -> Result<Vec<Order>> {
        let mut builder = QueryBuilder::new(SQL);

        builder.push(condition_prefix);

        self.apply_filters(&mut builder);
        if let Some(page_params) = &page_params {
            if let Some(param) = &page_params.params {
                if let Some(begin_time) = param.get("startTime") {
                    builder.push(" AND strftime('%Y%m%d', o.create_time) >= strftime('%Y%m%d', ");
                    builder.push_bind(begin_time);
                    builder.push(") ");
                }
            }
        }

        if let Some(page_params) = &page_params {
            if let Some(param) = &page_params.params {
                if let Some(end_time) = param.get("endTime") {
                    builder.push(" AND strftime('%Y%m%d', o.create_time) <= strftime('%Y%m%d', ");
                    builder.push_bind(end_time);
                    builder.push(") ");
                }
            }
        }

        builder.push(" GROUP BY o.order_id");

        // sort
        builder.push(" ORDER BY o.create_time DESC");

        if let Some(param) = &page_params {
            builder.push(" LIMIT ").push_bind(param.page_size);
            builder
                .push(" OFFSET ")
                .push_bind((param.page - 1) * param.page_size);
        }

        let orders = builder.build_query_as().fetch_all(pool).await?;
        Ok(orders)
    }

    pub async fn list(
        &self,
        pool: &Pool<Sqlite>,
        page_params: PageParams,
    ) -> Result<PageResult<Order>> {
        let rows = self
            .build_query(pool, " WHERE 1=1 ", Some(page_params.clone()))
            .await?;
        let total = self.count(pool, Some(page_params)).await?;
        Ok(PageResult { total, rows })
    }

    pub async fn list_4_home(&self, pool: &Pool<Sqlite>) -> Result<Vec<Order>> {
        self.build_query(pool, " WHERE o.status != '04' AND o.status != '05' ", None)
            .await
    }

    pub async fn list_by_cloth_name(
        pool: &Pool<Sqlite>,
        store_id: i64,
        page_params: &PageParams,
        query: &OrderQuery,
    ) -> Result<Vec<Self>> {
        let mut builder = QueryBuilder::new(SQL_BY_CLOTHING_NAME);
        builder.push(" WHERE o.store_id = ").push_bind(store_id);
        builder.push(" AND o.user_id = ").push_bind(query.user_id);
        if let Some(cloth_name) = &query.cloth_name {
            builder
                .push(" AND c.title LIKE ")
                .push_bind(format!("%{}%", cloth_name));
        }
        if let Some(start_time) = &query.start_time {
            builder.push(" AND o.create_time >= ").push_bind(start_time);
        }

        if let Some(end_time) = &query.end_time {
            builder.push(" AND o.create_time <= ").push_bind(end_time);
        }

        builder.push(" GROUP BY o.order_id");
        builder.push(" ORDER BY o.create_time DESC");
        builder.push(" LIMIT ").push_bind(page_params.page_size);
        builder
            .push(" OFFSET ")
            .push_bind((page_params.page - 1) * page_params.page_size);

        let orders = builder.build_query_as().fetch_all(pool).await?;
        Ok(orders)
    }

    pub async fn count_by_cloth_name(
        pool: &Pool<Sqlite>,
        store_id: i64,
        query: &OrderQuery,
    ) -> Result<u64> {
        let mut builder =
            QueryBuilder::new(format!("SELECT COUNT(*) FROM ({SQL_BY_CLOTHING_NAME}"));
        builder.push(" WHERE o.store_id = ").push_bind(store_id);
        builder.push(" AND o.user_id = ").push_bind(query.user_id);
        if let Some(cloth_name) = &query.cloth_name {
            builder
                .push(" AND c.title LIKE ")
                .push_bind(format!("%{}%", cloth_name));
        }
        if let Some(start_time) = &query.start_time {
            builder.push(" AND o.create_time >= ").push_bind(start_time);
        }

        if let Some(end_time) = &query.end_time {
            builder.push(" AND o.create_time <= ").push_bind(end_time);
        }

        builder.push(" GROUP BY o.order_id");
        builder.push(")");

        let count = builder.build_query_scalar().fetch_one(pool).await?;
        Ok(count)
    }

    pub async fn update(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<bool> {
        let result = sqlx::query(
            r#"
            UPDATE orders SET
                store_id = ?,
                order_number = ?,
                business_type = ?,
                user_id = ?,
                desire_complete_time = ?,
                cost_time_alarm = ?,
                pickup_code = ?,
                complete_time = ?,
                delivery_mode = ?,
                source = ?,
                status = ?,
                payment_status = ?,
                remark = ?,
                order_type = ?,
                update_time = ?
            WHERE order_id = ?
            "#,
        )
        .bind(&self.store_id)
        .bind(&self.order_number)
        .bind(&self.business_type)
        .bind(&self.user_id)
        .bind(&self.desire_complete_time)
        .bind(&self.cost_time_alarm)
        .bind(&self.pickup_code)
        .bind(&self.complete_time)
        .bind(&self.delivery_mode)
        .bind(&self.source)
        .bind(&self.status)
        .bind(&self.payment_status)
        .bind(&self.remark)
        .bind(&self.order_type)
        .bind(utils::get_timestamp())
        .bind(&self.order_id)
        .execute(&mut **tr)
        .await?;

        // Update price relations
        if let Some(order_id) = self.order_id {
            // Delete existing relations
            sqlx::query("DELETE FROM order_price_relations WHERE order_id = ?")
                .bind(order_id)
                .execute(&mut **tr)
                .await?;

            // Insert new relations
            if let Some(price_ids) = &self.price_ids {
                for price_id in price_ids {
                    sqlx::query(
                        "INSERT INTO order_price_relations (order_id, price_id, create_time)
                        VALUES (?, ?, ?)",
                    )
                    .bind(order_id)
                    .bind(price_id)
                    .bind(utils::get_now())
                    .execute(&mut **tr)
                    .await?;
                }
            }
        }

        Ok(result.rows_affected() > 0)
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

    pub async fn select_list_with_wait_to_pick_with_user_ids(
        pool: &Pool<Sqlite>,
        store_id: i64,
        user_ids: &[i64],
    ) -> Result<Vec<Self>> {
        let mut builder = QueryBuilder::new(&format!(
            "{SQL} WHERE o.store_id = {store_id} AND o.status = '02' AND o.user_id IN ("
        ));

        // 添加占位符
        user_ids.iter().enumerate().for_each(|(i, id)| {
            if i > 0 {
                builder.push(", ");
            }
            builder.push_bind(id);
        });

        // 关闭括号
        builder.push(")");

        builder.push("GROUP BY o.order_id");

        // 执行查询
        let result = builder.build_query_as().fetch_all(pool).await?;

        Ok(result)
    }

    pub async fn query_count_by_status(
        pool: &Pool<Sqlite>,
        store_id: i64,
        status: &str,
    ) -> Result<i64> {
        let result =
            sqlx::query_scalar("SELECT COUNT(1) FROM orders WHERE store_id = ? AND status = ?")
                .bind(store_id)
                .bind(status)
                .fetch_one(pool)
                .await?;

        Ok(result)
    }

    pub async fn count_by_source(
        pool: &Pool<Sqlite>,
        store_id: i64,
        year: Option<i32>,
        month: Option<u32>,
    ) -> Result<Vec<SourceDistribution>> {
        let mut query =
            String::from("SELECT source, COUNT(1) AS count FROM orders WHERE store_id = ?");

        if let (Some(year), Some(month)) = (year, month) {
            query.push_str(&format!(
                " AND strftime('%Y', create_time) = '{}' AND strftime('%m', create_time) = '{:02}'",
                year, month
            ));
        }

        query.push_str(" GROUP BY source");

        let result = sqlx::query_as(&query)
            .bind(store_id)
            .fetch_all(pool)
            .await?;

        Ok(result)
    }

    pub async fn total(pool: &Pool<Sqlite>, store_id: i64) -> Result<i64> {
        let count = sqlx::query_scalar("SELECT COUNT(1) FROM orders WHERE store_id =?")
            .bind(store_id)
            .fetch_one(pool)
            .await?;

        Ok(count)
    }

    /// 检查订单时效并更新预警状态
    pub async fn check_time_warning(pool: &Pool<Sqlite>, store_id: i64) -> Result<()> {
        let now = Local::now();
        let warning_threshold = now + chrono::Duration::days(1);

        // 查询所有未完成且需要检查的订单
        let orders = sqlx::query_as::<_, Order>(
            "SELECT * FROM orders 
            WHERE store_id = ? 
            AND status NOT IN ('04', '05', '06') 
            AND desire_complete_time IS NOT NULL",
        )
        .bind(store_id)
        .fetch_all(pool)
        .await?;

        let mut tx = pool.begin().await?;

        for mut order in orders {
            if let Some(desire_time) = order.desire_complete_time {
                // 将 NaiveDate 转换为 DateTime<Utc>
                let desire_datetime = desire_time
                    .and_hms_opt(0, 0, 0)
                    .map(|ndt| {
                        DateTime::<Local>::from_naive_utc_and_offset(ndt, now.offset().clone())
                    })
                    .unwrap_or_else(|| now);

                let current_alarm = order.cost_time_alarm.as_deref().unwrap_or(NORMAL_ALARM);
                let new_alarm = if desire_datetime < now {
                    // 已超时
                    OVERDUE_ALARM
                } else if desire_datetime <= warning_threshold {
                    // 即将到期
                    WARNING_ALARM
                } else {
                    // 正常
                    NORMAL_ALARM
                };

                // 如果预警状态发生变化，则更新
                if current_alarm != new_alarm {
                    order.cost_time_alarm = Some(new_alarm.to_string());
                    if !order.update(&mut tx).await? {
                        return Err(Error::internal("更新订单预警状态失败"));
                    }

                    // 记录预警日志
                    match new_alarm {
                        OVERDUE_ALARM => {
                            tracing::warn!(
                                "订单 {} 已超时，期望完成时间: {}",
                                order.order_number.unwrap_or_default(),
                                desire_time
                            );
                        }
                        WARNING_ALARM => {
                            tracing::info!(
                                "订单 {} 即将到期，期望完成时间: {}",
                                order.order_number.unwrap_or_default(),
                                desire_time
                            );
                        }
                        _ => {
                            tracing::info!(
                                "订单 {} 恢复正常状态，期望完成时间: {}",
                                order.order_number.unwrap_or_default(),
                                desire_time
                            );
                        }
                    }
                }
            }
        }

        tx.commit().await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow, Default)]
pub struct SourceDistribution {
    pub source: String,
    pub count: i64,
}

const ORDER_NUMBER_PREFIX: &str = "XYFW-";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct OrderWithCloth {
    pub order: Order,
    pub clothes: Vec<OrderCloth>,
}

impl Request for OrderWithCloth {
    const URL: &'static str = "/orders";
}

impl Order {
    pub async fn add_order(&mut self, state: &tauri::State<'_, AppState>) -> Result<Order> {
        let pool = &state.pool;
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

        let cloth_ids = self
            .cloth_ids
            .as_deref()
            .ok_or(Error::bad_request("cloth_ids is empty"))?;

        // add to server
        let clothes = OrderCloth::get_by_ids(pool, &cloth_ids).await?;
        let order_with_cloth = OrderWithCloth {
            order: self.clone(),
            clothes,
        };
        let order_with_cloth = order_with_cloth.create_request(state).await?;
        self.order_id = order_with_cloth.order.order_id;
        // insert into db
        let order = self.create(&mut tr).await?;

        // save adjust data to db
        if let Some(adjust) = self.adjust.as_mut() {
            adjust.order_id = order.order_id;
            adjust.create(&mut tr).await?;
        }

        let order_id = order.order_id.unwrap_or_default();

        if order.price_ids.is_some() && !order.price_ids.as_ref().unwrap().is_empty() {
            // increment ref_num
            if !ClothPrice::increment_ref_num(&mut tr, &order.price_ids.as_ref().unwrap()).await? {
                return Err(Error::internal("increment ref_num failed"));
            }
        }

        // update clothes order_id
        if !OrderCloth::update_order_id(&mut tr, order_id, &cloth_ids).await? {
            return Err(Error::internal("update clothes failed"));
        }

        tr.commit().await?;
        Ok(order)
    }

    async fn udpate_order(&mut self, state: &tauri::State<'_, AppState>) -> Result<bool> {
        let pool = &state.pool;
        let mut tx = pool.begin().await?;

        // update to server
        let cloth_ids = self
            .cloth_ids
            .as_deref()
            .ok_or(Error::bad_request("cloth_ids is empty"))?;

        // add to server
        let clothes = OrderCloth::get_by_ids(pool, &cloth_ids).await?;
        let order_with_cloth = OrderWithCloth {
            order: self.clone(),
            clothes,
        };

        if !order_with_cloth.update_request(state).await? {
            return Err(Error::internal("update order failed"));
        }

        let res = self.update(&mut tx).await?;

        // save adjust data to db
        if let Some(adjust) = self.adjust.as_mut() {
            adjust.order_id = self.order_id;
            adjust.upsert(&mut tx).await?;
        }

        let order_id = self.order_id.unwrap_or_default();
        // update clothes order_id
        if !OrderCloth::update_order_id(&mut tx, order_id, &cloth_ids).await? {
            return Err(Error::internal("update clothes failed"));
        }
        tx.commit().await?;
        Ok(res)
    }

    fn initial(&mut self) {
        self.payment_status = Some(PaymentStatus::Unpaid);
        self.order_type = Some(NORMAL_ORDER.to_string());
        self.status = Some(OrderStatus::Processing);
        self.cost_time_alarm = Some(NORMAL_ALARM.to_string());
        self.business_type = Some(BUSINESS_MAIN.to_string());
        self.create_time = Some(utils::get_now());
    }

    pub async fn pay(
        state: &tauri::State<'_, AppState>,
        store_id: i64,
        mut payment_req: PaymentReq,
    ) -> Result<()> {
        let pool = &state.pool;
        let mut tr = pool.begin().await?;
        // 获取多个订单
        let orders = payment_req
            .orders
            .take()
            .ok_or(Error::bad_request("订单不能为空"))?;
        let mut is_time_based = false;

        let mut payment = payment_req
            .payment
            .take()
            .ok_or(Error::bad_request("支付信息不能为空"))?;

        // set store_id
        payment.store_id = Some(store_id);

        // 获取支付信息和卡券信息
        let mut user_coupons: Vec<UserCoupon> = if let Some(uc_id) = &payment.uc_id {
            let ids: Vec<i64> = uc_id
                .split(',')
                .filter_map(|id| id.parse::<i64>().ok())
                .collect();
            let coupons = UserCoupon::find_by_uc_ids(pool, store_id, &ids).await?;
            if coupons.len() != ids.len() {
                return Err(Error::bad_request("卡券信息不正确，存在未入库的卡券"));
            }
            coupons
        } else if let Some(time_based) = &payment_req.time_based {
            tracing::debug!("使用了次卡进行支付: {:?}", time_based);
            // 如果使用了次卡
            let ids: Vec<i64> = time_based.iter().map(|t| t.uc_id).collect();
            let coupons = UserCoupon::find_by_uc_ids(pool, store_id, &ids).await?;

            if coupons.len() != ids.len() {
                return Err(Error::bad_request("卡券信息不正确，存在未入库的卡券"));
            }
            is_time_based = true;
            payment.uc_id = Some(
                ids.iter()
                    .map(|&id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            );
            payment.payment_amount_vip = Some((ids.len()) as f64);
            coupons
        } else {
            vec![]
        };

        // 检查是否使用扫码支付
        let is_qr_code_payment = payment_req.auth_code.is_some();
        let auth_code = payment_req.auth_code.take();
        let subject = payment_req.subject.take();
        let payment_type = payment_req.payment_type.take().unwrap_or_default();

        // 处理订单信息
        let mut order_ids = Vec::new();
        let mut order_numbers = Vec::new();
        let mut total_payment_amount = 0.0;
        let mut user_id = None;

        for order in orders.iter() {
            if let Some(order_id) = order.order_id {
                // 校验订单状态
                let mut existing_order = Self::get_by_id(pool, store_id, order_id)
                    .await?
                    .ok_or(Error::bad_request("订单不存在"))?;

                if existing_order.payment_status == Some(PaymentStatus::Paid) {
                    return Err(Error::bad_request("订单已支付"));
                }

                // 查询订单衣物信息
                let clothes = OrderCloth::get_by_order_id(pool, order_id).await?;

                // 计算订单总价
                let order_total_amount =
                    Self::cal_total_price(pool, &mut existing_order, &clothes).await?;
                total_payment_amount += payment.payment_amount.unwrap_or(0.0);

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
                    }
                }

                // 如果所有衣物状态为 "已取件"，更新订单状态为 "已完成"
                if clothes
                    .iter()
                    .all(|cloth| cloth.clothing_status == Some(ClothStatus::PickedUp))
                {
                    existing_order.status = Some(OrderStatus::Completed);
                }

                // 收集订单信息，用于后续扫码支付
                order_ids.push(order_id);
                if let Some(order_number) = &existing_order.order_number {
                    order_numbers.push(order_number.clone());
                }
                if user_id.is_none() {
                    user_id = existing_order.user_id;
                }
            }
        }

        tracing::debug!("支付请求信息: {:?}", payment_req);

        #[derive(Debug, Clone, Serialize, Deserialize, Default)]
        struct OrderWithPayment {
            order: Order,
            payment: Payment,
        }

        impl Request for Vec<OrderWithPayment> {
            const URL: &'static str = "/orders/payment";
        }

        let mut orders_with_payments = Vec::with_capacity(orders.len());
        // 如果是扫码支付，调用相应的支付接口
        if is_qr_code_payment {
            let subject_text =
                subject.unwrap_or_else(|| format!("订单支付-{}", order_numbers.join(",")));

            // 根据支付类型调用不同的支付接口
            if payment_type == PaymentReqMethod::Alipay {
                if let Some(auth_code) = auth_code {
                    // 使用支付宝付款码支付
                    let req_body = weapay::alipay::prelude::ReqOrderBody {
                        out_trade_no: format!("{}{}", "PAY", chrono::Utc::now().timestamp_millis()),
                        subject: subject_text.clone(),
                        total_amount: total_payment_amount.to_string(),
                        auth_code: Some(auth_code.clone()),
                        scene: Some("bar_code".to_string()),
                        ..Default::default()
                    };

                    let alipay_result =
                        crate::pay::pay_with_alipay_auth_code(pool, store_id, req_body).await?;

                    // 检查支付结果
                    if let Some(trade_status) = &alipay_result.trade_status {
                        if trade_status == "TRADE_SUCCESS" || trade_status == "TRADE_FINISHED" {
                            // 支付成功，更新订单状态
                            for order_id in &order_ids {
                                let mut existing_order = Self::get_by_id(pool, store_id, *order_id)
                                    .await?
                                    .ok_or(Error::bad_request("订单不存在"))?;

                                // 更新订单支付状态为已支付
                                existing_order.payment_status = Some(PaymentStatus::Paid);
                                if !existing_order.update(&mut tr).await? {
                                    return Err(Error::internal("update order failed"));
                                }

                                // 创建支付记录
                                payment.pay_id = Some(uuid::Uuid::new_v4().to_string());
                                payment.order_status = Some(PAY_STATUS_PAID.to_string());
                                payment.payment_status = Some(PaymentStatus::Paid);
                                payment.pay_number = existing_order.order_number.clone();
                                payment.uc_order_id = Some(*order_id);
                                payment.transaction_id = alipay_result
                                    .trade_no
                                    .clone()
                                    .map(|s| s.parse::<i64>().unwrap_or_default());
                                let payment = payment.create_payment(&mut tr).await?;

                                // 创建qrcode payment record
                                let qrcode_payment = QrcodePayment {
                                    pay_id: payment.pay_id.clone(),
                                    store_id: Some(store_id),
                                    payment_type: Some("alipay".to_string()),
                                    auth_code: Some(auth_code.clone()),
                                    out_trade_no: alipay_result.out_trade_no.clone(),
                                    trade_no: alipay_result.trade_no.clone(),
                                    total_amount: Some(total_payment_amount),
                                    subject: Some(subject_text.clone()),
                                    trade_status: alipay_result.trade_status.clone(),
                                    buyer_id: None,
                                    buyer_logon_id: None,
                                    receipt_amount: alipay_result
                                        .total_amount
                                        .as_ref()
                                        .map(|s| s.parse::<f64>().unwrap_or_default()),
                                    raw_response: Some(
                                        serde_json::to_string(&alipay_result).unwrap_or_default(),
                                    ),
                                    create_time: Some(utils::get_now()),
                                    ..Default::default()
                                };
                                qrcode_payment.create(&mut tr).await?;
                                orders_with_payments.push(OrderWithPayment {
                                    order: existing_order,
                                    payment: payment.clone(),
                                });
                            }

                            // 更新用户积分
                            if let Some(user_id) = user_id {
                                if !User::increase_points(
                                    &mut tr,
                                    user_id,
                                    total_payment_amount as i64,
                                )
                                .await?
                                {
                                    return Err(Error::internal("更新用户积分失败"));
                                }
                            }

                            // 同步支付信息到服务端
                            orders_with_payments.create_request(state).await?;
                            tr.commit().await?;

                            return Ok(());
                        } else {
                            // 支付失败
                            return Err(Error::bad_request(&format!(
                                "支付宝付款码支付失败: {}",
                                trade_status
                            )));
                        }
                    } else {
                        // 支付状态未知
                        return Err(Error::bad_request("支付宝付款码支付状态未知"));
                    }
                }
            } else if payment_type == PaymentReqMethod::Wechat {
                // 微信支付逻辑可以在这里实现
                // 目前先返回错误，表示功能尚未实现
                return Err(Error::bad_request("微信扫码支付功能尚未实现"));
            }
        } else {
            // 非扫码支付，继续原有流程
            for order_id in &order_ids {
                let mut existing_order = Self::get_by_id(pool, store_id, *order_id)
                    .await?
                    .ok_or(Error::bad_request("order is not exist"))?;

                // 更新订单支付状态为已支付
                existing_order.payment_status = Some(PaymentStatus::Paid);
                if !existing_order.update(&mut tr).await? {
                    return Err(Error::internal("update order failed"));
                }

                // 创建支付记录
                payment.pay_id = Some(uuid::Uuid::new_v4().to_string());
                payment.order_status = Some(PAY_STATUS_PAID.to_string());
                payment.payment_status = Some(PaymentStatus::Paid);
                payment.pay_number = existing_order.order_number.clone();
                payment.uc_order_id = Some(*order_id);
                payment.create_payment(&mut tr).await?;

                // save order and payment
                orders_with_payments.push(OrderWithPayment {
                    order: existing_order,
                    payment: payment.clone(),
                });
            }

            // 更新用户积分
            if let Some(user_id) = user_id {
                if !User::increase_points(&mut tr, user_id, total_payment_amount as i64).await? {
                    return Err(Error::internal("更新用户积分失败"));
                }
            }
            // 同步支付信息到服务端
            orders_with_payments.create_request(state).await?;
        }

        tr.commit().await?;
        Ok(())
    }

    async fn validate_and_apply_coupons(
        tr: &mut Transaction<'_, Sqlite>,
        payment: &mut Payment,
        user_coupons: &mut Vec<UserCoupon>,
        total_amount: f64,
    ) -> Result<bool> {
        if user_coupons.is_empty() {
            return Ok(true);
        }

        // 将输入值转换为Decimal，确保精度
        let mut total_amount_decimal = Decimal::from_f64(total_amount).unwrap_or_default();

        // 校验卡券是否过期
        for coupon in user_coupons.iter() {
            let coupon = coupon
                .coupon
                .as_ref()
                .ok_or(Error::internal("get coupon failed"))?;
            if Some(utils::get_now()) > coupon.valid_to {
                return Err(Error::bad_request(format!(
                    "优惠券 <{}> 已过期",
                    coupon.coupon_title.clone().unwrap_or_default()
                )));
            }
        }

        // 校验卡券类型一致性
        let (storage_cards, discount_cards, other_coupons): (Vec<_>, Vec<_>, Vec<_>) = {
            let mut storage = Vec::new();
            let mut discount = Vec::new();
            let mut others = Vec::new();

            for coupon in user_coupons.iter_mut() {
                if let Some(c) = coupon.coupon.as_ref() {
                    match c.coupon_type {
                        Some(CouponType::StoredValueCard) => storage.push(coupon),
                        Some(CouponType::DiscountCard) => discount.push(coupon),
                        _ => others.push(coupon),
                    }
                }
            }
            (storage, discount, others)
        };

        // 检查卡券类型冲突
        let card_types_count = [
            !storage_cards.is_empty(),
            !discount_cards.is_empty(),
            !other_coupons.is_empty(),
        ]
        .iter()
        .filter(|&&x| x)
        .count();

        if card_types_count > 1 {
            return Err(Error::bad_request("不能同时使用不同类型的卡券"));
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
                let available_value_decimal =
                    Decimal::from_f64(available_value).unwrap_or_default();

                if available_value_decimal >= total_amount_decimal {
                    // 使用储值卡支付足够金额
                    let new_balance = available_value_decimal - total_amount_decimal;
                    storage_card.available_value = Some(new_balance.to_f64().unwrap_or_default());
                    payment.payment_method = Some(PaymentMethod::StoredValueCard);
                    // 储值卡类型不计算卡券优惠金额
                    // payment.payment_amount_vip = None;
                    // update user coupon
                    if !storage_card.update(tr).await? {
                        return Err(Error::internal("update user coupon failed"));
                    }
                    return Ok(true);
                } else {
                    // 使用储值卡支付部分金额
                    total_amount_decimal -= available_value_decimal;
                    storage_card.available_value = Some(0.0);
                    // storage_card.uc_count = Some(0);
                    // update user coupon
                    if !storage_card.update(tr).await? {
                        return Err(Error::internal("update user coupon failed"));
                    }
                }
            }
            return Ok(true);
        }

        // 折扣卡处理逻辑
        if !discount_cards.is_empty() {
            // 验证所有折扣卡的折扣系数是否相同
            let first_discount_rate = discount_cards[0]
                .coupon
                .as_ref()
                .and_then(|c| c.usage_value)
                .ok_or(Error::internal("获取折扣卡折扣系数失败"))?;

            for discount_card in &discount_cards {
                let discount_rate = discount_card
                    .coupon
                    .as_ref()
                    .and_then(|c| c.usage_value)
                    .ok_or(Error::internal("获取折扣卡折扣系数失败"))?;

                if (discount_rate - first_discount_rate).abs() > 0.01 {
                    return Err(Error::bad_request("只能同时使用相同折扣系数的折扣卡"));
                }
            }

            // 计算折扣后的订单总金额
            let discount_rate_decimal = Decimal::from_f64(first_discount_rate).unwrap_or_default();
            let hundred = Decimal::from(100);
            let discount_multiplier = discount_rate_decimal / hundred;
            let discounted_total = (total_amount_decimal * discount_multiplier)
                .round_dp_with_strategy(2, RoundingStrategy::MidpointTowardZero);

            tracing::debug!(
                "[支付] 折扣卡逻辑: 原订单金额: {}, 折扣系数: {}%, 折扣后金额: {}",
                total_amount_decimal,
                first_discount_rate,
                discounted_total
            );

            // 按余额升序排序折扣卡
            let mut sorted_discount_cards = discount_cards
                .into_iter()
                .filter(|card| card.available_value.unwrap_or(0.0) > 0.0)
                .collect::<Vec<_>>();
            sorted_discount_cards
                .sort_by(|a, b| a.available_value.partial_cmp(&b.available_value).unwrap());

            let mut remaining_amount = discounted_total;

            for discount_card in sorted_discount_cards {
                let available_value = discount_card.available_value.unwrap_or(0.0);
                let available_value_decimal =
                    Decimal::from_f64(available_value).unwrap_or_default();

                if available_value_decimal >= remaining_amount {
                    // 使用折扣卡支付足够金额
                    let new_balance = available_value_decimal - remaining_amount;
                    discount_card.available_value = Some(new_balance.to_f64().unwrap_or_default());

                    // 更新用户卡券
                    if !discount_card.update(tr).await? {
                        return Err(Error::internal("更新用户折扣卡失败"));
                    }
                    break;
                } else {
                    // 使用折扣卡支付部分金额
                    remaining_amount -= available_value_decimal;
                    discount_card.available_value = Some(0.0);

                    // 更新用户卡券
                    if !discount_card.update(tr).await? {
                        return Err(Error::internal("更新用户折扣卡失败"));
                    }
                }
            }

            // 设置支付方式为折扣卡
            payment.payment_method = Some(PaymentMethod::DiscountCard);
            return Ok(true);
        }

        // 校验非储值卡优惠券数量
        if other_coupons.len() > 1 {
            return Err(Error::bad_request("每次支付只能使用一张优惠券"));
        }

        // 处理单张优惠券
        if let Some(user_coupon) = other_coupons.into_iter().next() {
            let coupon = user_coupon
                .coupon
                .as_ref()
                .ok_or(Error::internal("Invalid coupon data"))?;

            if coupon.coupon_type == Some(CouponType::DiscountCoupon)
                || coupon.coupon_type == Some(CouponType::SpendAndSaveCard)
            {
                // sub count
                let uc_count = user_coupon
                    .uc_count
                    .as_mut()
                    .ok_or(Error::internal("获取用户优惠券数量失败"))?;
                if *uc_count > 0 {
                    *uc_count -= 1;
                } else {
                    return Err(Error::bad_request("优惠券数量不足"));
                }

                // 校验卡券最低消费
                if let Some(min_spend) = coupon.min_spend {
                    let min_spend_decimal = Decimal::from_f64(min_spend).unwrap_or_default();
                    if total_amount_decimal < min_spend_decimal {
                        return Err(Error::bad_request("最小消费金额未达到，请选择其他优惠券"));
                    }
                }

                // 计算优惠金额
                let result = if coupon.coupon_type == Some(CouponType::DiscountCoupon) {
                    if coupon.usage_value.is_none() || coupon.usage_limit.is_none() {
                        return Err(Error::internal("get coupon usage value failed"));
                    }
                    // 折扣券逻辑
                    let usage_value_decimal =
                        Decimal::from_f64(coupon.usage_value.unwrap_or_default())
                            .unwrap_or_default();
                    let hundred = Decimal::from(100);
                    let discount = Decimal::ONE - (usage_value_decimal / hundred);
                    let discounted = (total_amount_decimal * discount)
                        .round_dp_with_strategy(2, RoundingStrategy::MidpointTowardZero);
                    let usage_limit = Decimal::from_f64(coupon.usage_limit.unwrap_or_default())
                        .unwrap_or_default();
                    let result = if discounted > usage_limit {
                        usage_limit
                    } else {
                        discounted
                    };
                    tracing::debug!(
                        "[支付] 折扣券逻辑: 总金额: {}, 折扣：{}, 优惠金额: {}",
                        total_amount_decimal,
                        discount,
                        result
                    );
                    total_amount_decimal - result
                } else {
                    // 满减券逻辑
                    let usage_value = Decimal::from_f64(coupon.usage_value.unwrap_or_default())
                        .unwrap_or_default();
                    (total_amount_decimal - usage_value)
                        .round_dp_with_strategy(2, RoundingStrategy::MidpointTowardZero)
                };

                // 更新金额和校验逻辑
                let payment_amount_vip =
                    (total_amount_decimal - result).to_f64().unwrap_or_default();
                payment.payment_amount_vip = Some(payment_amount_vip);
                let payment_amount = payment.payment_amount.unwrap_or_default();
                let payment_amount_decimal = Decimal::from_f64(payment_amount).unwrap_or_default();
                // let result_decimal =
                //     Decimal::from_f64(result.to_f64().unwrap_or_default()).unwrap_or_default();

                // 校验最终金额 - 使用Decimal比较以避免浮点误差
                let diff = (result - payment_amount_decimal).abs();
                tracing::debug!(
                    "[支付] 最终金额: {}, 订单最终金额: {}, 差值: {}",
                    result,
                    payment_amount_decimal,
                    diff
                );
                if diff > Decimal::from_f64(0.01).unwrap_or_default() {
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
            tracing::debug!("[次卡支付] 找到次卡数量: {}", time_based.len());
            for coupon in user_coupons.iter_mut() {
                if let Some(uc_id) = coupon.uc_id {
                    tracing::debug!("[次卡支付] 处理次卡ID: {}", uc_id);
                    // Get corresponding `TimeBasedCoupon`
                    let time_based_coupon = time_based
                        .iter_mut()
                        .filter(|t| t.uc_id == uc_id)
                        .next()
                        .ok_or(Error::internal("所选次卡信息错误"))?;
                    let usable_count = coupon.available_value.unwrap_or_default() as usize;
                    // Remaining uses in the system
                    // Uses requested by the user
                    tracing::debug!(
                        "[次卡支付] 次卡ID: {}, 剩余次数: {}, 请求使用次数: {}",
                        uc_id,
                        usable_count,
                        cloth_count
                    );
                    if usable_count == 0 {
                        continue;
                    }
                    if usable_count >= cloth_count {
                        tracing::debug!(
                            "[次卡支付] 次卡ID: {}, 可完全覆盖剩余衣物({}件)",
                            uc_id,
                            cloth_count
                        );
                        let new_value = usable_count - cloth_count;
                        tracing::debug!(
                            "[次卡支付] 次卡ID: {}, 当前值: {}, 扣减: {}, 新值: {}",
                            uc_id,
                            usable_count,
                            cloth_count,
                            new_value
                        );
                        coupon.available_value = Some(new_value as f64);
                        time_based_coupon.count -= cloth_count as i32;
                        tracing::debug!(
                            "[次卡支付] 次卡ID: {}, 更新后剩余次数: {}, 请求中剩余次数: {}",
                            uc_id,
                            coupon.available_value.unwrap_or_default(),
                            time_based_coupon.count
                        );
                        // Update the coupon in the database
                        if !coupon.update(tr).await? {
                            tracing::error!("[次卡支付] 次卡ID: {}, 更新数据库失败", uc_id);
                            return Err(Error::internal("update user coupon failed"));
                        }
                        used_coupon_ids.push(uc_id);
                        total_used_coupons += cloth_count;
                        cloth_count = 0;
                        tracing::debug!(
                            "[次卡支付] 次卡ID: {}, 使用完毕，总使用次数: {}",
                            uc_id,
                            total_used_coupons
                        );
                        break;
                    } else {
                        tracing::debug!(
                            "[次卡支付] 次卡ID: {}, 不足以覆盖所有衣物，部分使用",
                            uc_id
                        );
                        // If the coupon cannot cover all clothes, use it up and continue to the next coupon
                        cloth_count -= usable_count;
                        coupon.available_value = Some(0f64); // This coupon is fully consumed                        
                        time_based_coupon.count -= usable_count as i32;
                        tracing::debug!(
                            "[次卡支付] 次卡ID: {}, 已用完，剩余衣物数: {}",
                            uc_id,
                            cloth_count
                        ); // Update the coupon in the database
                        if !coupon.update(tr).await? {
                            tracing::error!("[次卡支付] 次卡ID: {}, 更新数据库失败", uc_id);
                            return Err(Error::internal("update user coupon failed"));
                        }
                        used_coupon_ids.push(uc_id);
                        total_used_coupons += usable_count;
                        tracing::debug!("[次卡支付] 已累计使用次数: {}", total_used_coupons);
                    }
                }
            }
            if cloth_count > 0 {
                tracing::error!("[次卡支付] 次卡不足，剩余未覆盖衣物: {}", cloth_count);
                return Err(Error::bad_request("所选次卡数量不足"));
            } // Update the `PaymentReq` object
            if let Some(payment) = payment.payment.as_mut() {
                let uc_ids = used_coupon_ids
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",");
                tracing::debug!(
                    "[次卡支付] 更新支付请求，使用次卡IDs: {}, 总使用次数: {}",
                    uc_ids,
                    total_used_coupons
                );
                payment.uc_id = Some(uc_ids);
                payment.payment_amount_vip = Some(total_used_coupons as f64);
            }
        } else {
            tracing::debug!("[次卡支付] 未找到次卡信息");
        }
        tracing::debug!("[次卡支付] 验证完成，次卡支付有效");
        Ok(())
    }

    fn cal_payment_method(order: &mut Order, payment: &mut Payment) {
        // 使用模式匹配提升可读性
        match payment.payment_method {
            Some(PaymentMethod::AlipayAndStoredValueCard)
            | Some(PaymentMethod::WechatPayAndStoredValueCard)
            | Some(PaymentMethod::CashAndStoredValueCard) => {
                order.payment_bonus_count = payment.payment_amount_vip;
                order.diff_price = payment.payment_amount_mv;
            }
            Some(PaymentMethod::Meituan) | Some(PaymentMethod::Douyin) => {
                order.payment_bonus_count = payment.payment_amount_vip;
            }
            Some(PaymentMethod::StoredValueCard) => {
                order.payment_bonus_count = payment.payment_amount_vip;
                order.diff_price = Some(0.0);
            }
            Some(PaymentMethod::DiscountCard) => {
                let total = payment.total_amount.unwrap_or_default();
                let paid = payment.payment_amount.unwrap_or_default();
                order.payment_bonus_count =
                    Some(((total * 100.0) as i64 - (paid * 100.0) as i64) as f64 / 100.0);
                order.diff_price = Some(paid);
            }
            _ => {
                order.payment_bonus_count = payment.payment_amount_vip;
                order.diff_price = payment.payment_amount;
            }
        }

        // 其他字段赋值
        order.payment_bonus_type = payment.payment_method.clone();
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
                if let Some(base_price) = cloth.price_value {
                    let base_price = Decimal::from_f64(base_price).unwrap_or_default();
                    let mut price = if Some(ServiceRequirmentType::Emergency)
                        == cloth.service_requirement
                    {
                        base_price * dec!(2.0)
                    } else if Some(ServiceRequirmentType::SingleWash) == cloth.service_requirement {
                        base_price * dec!(1.5)
                    } else {
                        base_price
                    };
                    price += Decimal::from_f64(cloth.process_markup.unwrap_or_default())
                        .unwrap_or_default();
                    price
                } else {
                    Decimal::ZERO
                }
            })
            .sum::<Decimal>();

        if let Some(price_ids) = &order.price_ids {
            for price_id in price_ids {
                // query price obj
                let cloth_price =
                    ClothPrice::get_by_id(pool, *price_id)
                        .await?
                        .ok_or(Error::with_details(
                            ErrorKind::NotFound,
                            "cloth price not found",
                        ))?;

                if let Some(price_value) = cloth_price.price_value {
                    price = Decimal::from_f64(price_value).unwrap_or_default();
                } else if let Some(price_discount) = cloth_price.price_discount {
                    // Apply discount to price
                    let discount =
                        Decimal::from_f64(price_discount).unwrap_or_default() / dec!(100);
                    let discount_value = price * discount;
                    price -= discount_value.round_dp(2);
                } else {
                    return Err(Error::internal("Price tag configuration error"));
                }
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

        // 始终执行一次截断，确保精度一致
        let rounded_price = price.round_dp_with_strategy(2, RoundingStrategy::ToZero);

        // Ensure the price is non-negative
        Ok(rounded_price.to_f64().unwrap_or_default().max(0.0))
    }

    pub async fn query_list(
        &self,
        pool: &Pool<Sqlite>,
        page_params: PageParams,
    ) -> Result<PageResult<Order>> {
        let mut result = self.list(pool, page_params).await?;
        // combine payment and clothes information
        for order in result.rows.iter_mut() {
            if let Some(id) = order.order_id {
                // query clothes by order id
                let clothes = OrderCloth::get_by_order_id(pool, id).await?;

                // extract cloth codes
                let (cloth_codes, cloth_ids): (Vec<String>, Vec<String>) = clothes
                    .iter()
                    .filter_map(|c| {
                        // 假设你有两个字段需要收集
                        Some((c.hang_cloth_code.clone()?, c.cloth_id.clone()?))
                    })
                    .unzip();
                order.cloth_codes = Some(cloth_codes);
                order.cloth_ids = Some(cloth_ids);

                // 处理支付方式和金额计算
                if order.payment.is_some() && order.payment.as_ref().unwrap().pay_id.is_some() {
                    // 先克隆 payment 以避免多重借用
                    let mut payment_clone = order.payment.clone().unwrap();
                    Self::cal_payment_method(order, &mut payment_clone);
                    order.payment = Some(payment_clone);
                } else {
                    // calculate total
                    order.payment_amount =
                        Some(Self::cal_total_price(pool, order, &clothes).await?);
                }
            }
        }

        Ok(result)
    }

    pub async fn query_list4home(&self, pool: &Pool<Sqlite>) -> Result<Vec<Self>> {
        self.list_4_home(pool).await
    }

    pub async fn query_list4history(
        pool: &Pool<Sqlite>,
        store_id: i64,
        page_params: PageParams,
        query: OrderQuery,
    ) -> Result<PageResult<Self>> {
        let rows = Self::list_by_cloth_name(pool, store_id, &page_params, &query).await?;
        let total = Self::count_by_cloth_name(pool, store_id, &query).await?;

        Ok(PageResult { total, rows })
    }

    pub async fn refund(
        pool: &Pool<Sqlite>,
        store_id: i64,
        order_id: i64,
        refund_reason: String,
    ) -> Result<()> {
        let mut order = Order::get_by_id(pool, store_id, order_id)
            .await?
            .ok_or(Error::not_found("order not found"))?;

        // check order's payment status
        if order.payment_status == Some(PaymentStatus::Refunded) {
            return Err(Error::bad_request("订单已经退单，请勿重复退单"));
        }

        // update order status to refund
        order.payment_status = Some(PaymentStatus::Refunded);
        // update order complete time
        order.complete_time = Some(utils::get_now());

        let mut tx = pool.begin().await?;

        // update clothes status to refund
        if !OrderCloth::refound_by_order_id(&mut tx, order.order_id.unwrap()).await? {
            return Err(Error::internal("update clothes status failed"));
        }

        // select payment record
        let payment = Payment::get_by_order_id(pool, order.order_id.unwrap(), store_id).await?;
        if payment.is_none() {
            order.status = Some(OrderStatus::Refunded);
            if !order.update(&mut tx).await? {
                return Err(Error::internal("update order failed"));
            }
            tx.commit().await?;
            return Ok(());
        } else {
            order.status = Some(OrderStatus::Refunded);
            if !order.update(&mut tx).await? {
                return Err(Error::internal("update order failed"));
            }
        }

        let mut payment = payment.unwrap();
        // Add refund reason to payment
        payment.refund_reason = Some(refund_reason);

        // check if user used coupon
        let uc_id = &payment.uc_id;
        if uc_id.is_none() {
            // update payment status
            payment.payment_status = Some(PaymentStatus::Refunded);
            if !payment.update_payment_status(&mut tx).await? {
                return Err(Error::internal("更新支付状态失败"));
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
        let mut user_coupons = UserCoupon::find_by_uc_ids(pool, store_id, &uc_ids).await?;
        if user_coupons.len() != uc_ids.len() {
            return Err(Error::internal("卡券信息不正确，无法退还"));
        }

        // Query clothes to determine how many times were used
        let order_clothes = OrderCloth::get_by_order_id(pool, order_id).await?;
        let cloth_count = order_clothes.len();
        tracing::debug!("[退款] 订单包含的衣物数量: {}", cloth_count);
        // Check if there are storage card type coupons, discount cards or time-based cards involved
        let is_type_000 = user_coupons.iter().any(|coupon| {
            coupon.coupon.is_some()
                && coupon.coupon.as_ref().unwrap().coupon_type == Some(CouponType::StoredValueCard)
        });
        let is_discount_card = user_coupons.iter().any(|coupon| {
            coupon.coupon.is_some()
                && coupon.coupon.as_ref().unwrap().coupon_type == Some(CouponType::DiscountCard)
        });
        // Check if this is a time-based payment by looking at payment_amount_vip value
        // and checking if it's a whole number matching cloth count
        let is_time_based = payment.payment_method == Some(PaymentMethod::SessionCard);
        tracing::debug!(
            "[退款] 支付类型: 储值卡: {}, 折扣卡: {}, 次卡: {}",
            is_type_000,
            is_discount_card,
            is_time_based
        );
        if is_time_based {
            // Handle time-based card refunds
            tracing::debug!("[退款] 处理次卡退款");
            // The payment_amount_vip field should equal the total number of times used
            let total_times_used = payment.payment_amount_vip.unwrap_or(0.0) as i32;
            let mut remaining_times = total_times_used;
            tracing::debug!("[退款] 需要退还的总次数: {}", remaining_times);
            if total_times_used <= 0 {
                tracing::debug!("[退款] 没有次数需要退还");
                tx.commit().await?;
                return Ok(());
            }
            // Sort user coupons by available value (ascending)
            // This is the reverse order of how they were likely consumed
            let mut sorted_coupons = user_coupons.clone();
            sorted_coupons.sort_by(|a, b| {
                a.available_value
                    .partial_cmp(&b.available_value)
                    .unwrap_or(Ordering::Equal)
            });
            for user_coupon in sorted_coupons.iter_mut() {
                if remaining_times <= 0 {
                    break;
                }
                if let Some(coupon) = &user_coupon.coupon {
                    // Get maximum capacity of this coupon
                    let max_capacity = user_coupon.uc_count.unwrap_or(0)
                        * coupon.usage_value.unwrap_or(0.0) as i32;
                    // Current available uses
                    let current_available = user_coupon.available_value.unwrap_or(0.0) as i32;
                    // Calculate how many uses were consumed from this coupon
                    let used_from_this_coupon = if max_capacity > 0 {
                        // If max_capacity is defined, calculate based on that
                        max_capacity - current_available
                    } else {
                        // Otherwise assume all remaining times came from this coupon
                        remaining_times
                    };
                    tracing::debug!(
                        "[退款] 处理次卡 ID: {}，当前剩余次数: {}，最大容量: {}，已使用次数: {}",
                        user_coupon.uc_id.unwrap_or_default(),
                        current_available,
                        max_capacity,
                        used_from_this_coupon
                    );
                    // Determine how many times to refund to this coupon
                    let times_to_refund = std::cmp::min(remaining_times, used_from_this_coupon);
                    if times_to_refund > 0 {
                        // Refund times to this coupon
                        let new_available = current_available + times_to_refund;
                        user_coupon.available_value = Some(new_available as f64);
                        tracing::debug!(
                            "[退款] 退还给次卡 ID: {} 的次数: {}，新的剩余次数: {}",
                            user_coupon.uc_id.unwrap_or_default(),
                            times_to_refund,
                            new_available
                        );
                        // Update the coupon in the database
                        if !user_coupon.update(&mut tx).await? {
                            return Err(Error::internal("退还次卡失败"));
                        }
                        // Reduce remaining times
                        remaining_times -= times_to_refund;
                    }
                }
            }
            // Check if all times were refunded
            if remaining_times > 0 {
                tracing::warn!("[退款] 警告：还有 {} 次未能退还给任何次卡", remaining_times);
            }
        } else if is_type_000 {
            // Sort storage cards by available value (descending) - reverse of payment order
            let mut storage_cards: Vec<_> = user_coupons
                .iter_mut()
                .filter(|coupon| {
                    coupon.coupon.is_some()
                        && coupon.coupon.as_ref().unwrap().coupon_type
                            == Some(CouponType::StoredValueCard)
                })
                .collect();
            tracing::debug!("[退款] 找到储值卡数量: {}", storage_cards.len());
            storage_cards.sort_by(|a, b| {
                b.available_value
                    .partial_cmp(&a.available_value)
                    .unwrap_or(Ordering::Equal)
            });
            // Total amount used in this order
            let total_amount = payment.payment_amount_vip;
            if total_amount.is_none() {
                tracing::debug!("[退款] 支付金额为空，无需退款");
                tx.commit().await?;
                return Ok(());
            }
            let mut total_amount = Decimal::from_f64(total_amount.unwrap()).unwrap_or_default();
            tracing::debug!("[退款] 需要退款的总金额: {}", total_amount);
            for user_coupon in storage_cards {
                if let Some(coupon) = &user_coupon.coupon {
                    // 使用Decimal进行所有计算，避免浮点精度问题
                    // Original total value of the storage card
                    let usage_value = Decimal::from_f64(coupon.usage_value.unwrap_or_default())
                        .unwrap_or_default();
                    let uc_count =
                        Decimal::from_f64(user_coupon.uc_count.unwrap_or_default() as f64)
                            .unwrap_or_default();
                    let old_value = usage_value * uc_count;
                    // 当前可用余额
                    let available_value =
                        Decimal::from_f64(user_coupon.available_value.unwrap_or_default())
                            .unwrap_or_default();
                    // Amount used in this order for this storage card
                    let used_value = old_value - available_value;
                    tracing::debug!(
                        "[退款] 处理储值卡 ID: {}，原始面值: {}，原始数量: {}，总价值: {}，当前余额: {}，已使用金额: {}",
                        user_coupon.uc_id.unwrap_or_default(),
                        usage_value,
                        uc_count,
                        old_value,
                        available_value,
                        used_value
                    );
                    if total_amount > used_value {
                        // Refund the full value
                        user_coupon.available_value = Some(old_value.to_f64().unwrap_or_default());
                        tracing::debug!(
                            "[退款] 全额退款，新余额: {}，剩余需退款金额: {}",
                            old_value,
                            total_amount - used_value
                        );
                        if !user_coupon.update(&mut tx).await? {
                            return Err(Error::internal("退还储值卡失败"));
                        }
                        total_amount -= used_value;
                    } else {
                        // Refund only part of the value
                        let new_available = available_value + total_amount;
                        user_coupon.available_value =
                            Some(new_available.to_f64().unwrap_or_default());
                        tracing::debug!("[退款] 部分退款，新余额: {}，退款完成", new_available);
                        if !user_coupon.update(&mut tx).await? {
                            return Err(Error::internal("退还储值卡失败"));
                        }
                        break;
                    }
                }
            }
        } else if is_discount_card {
            // Handle discount card refunds
            tracing::debug!("[退款] 处理折扣卡退款");

            // Get the original order amount and discount rate
            let original_amount = payment.total_amount.unwrap_or_default();
            let original_amount_decimal = Decimal::from_f64(original_amount).unwrap_or_default();

            // Sort discount cards by available value (descending) - reverse of payment order
            let mut discount_cards: Vec<_> = user_coupons
                .iter_mut()
                .filter(|coupon| {
                    coupon.coupon.is_some()
                        && coupon.coupon.as_ref().unwrap().coupon_type
                            == Some(CouponType::DiscountCard)
                })
                .collect();

            tracing::debug!("[退款] 找到折扣卡数量: {}", discount_cards.len());

            discount_cards.sort_by(|a, b| {
                b.available_value
                    .partial_cmp(&a.available_value)
                    .unwrap_or(Ordering::Equal)
            });

            // Get discount rate from the first card (all should have the same rate)
            let discount_rate = discount_cards[0]
                .coupon
                .as_ref()
                .and_then(|c| c.usage_value)
                .unwrap_or(100.0);

            let discount_rate_decimal = Decimal::from_f64(discount_rate).unwrap_or_default();
            let hundred = Decimal::from(100);
            let discount_multiplier = discount_rate_decimal / hundred;

            // Calculate the discounted amount that was actually paid
            let discounted_amount = (original_amount_decimal * discount_multiplier)
                .round_dp_with_strategy(2, RoundingStrategy::MidpointTowardZero);

            tracing::debug!(
                "[退款] 折扣卡退款: 原订单金额: {}, 折扣系数: {}%, 实际支付金额: {}",
                original_amount_decimal,
                discount_rate,
                discounted_amount
            );

            let mut total_amount = discounted_amount;

            for user_coupon in discount_cards {
                if total_amount <= Decimal::ZERO {
                    break;
                }

                let usage_value = user_coupon
                    .coupon
                    .as_ref()
                    .and_then(|c| c.usage_value)
                    .unwrap_or_default();
                let uc_count = user_coupon.uc_count.unwrap_or_default();
                let old_value =
                    Decimal::from_f64(usage_value).unwrap_or_default() * Decimal::from(uc_count);
                let available_value =
                    Decimal::from_f64(user_coupon.available_value.unwrap_or_default())
                        .unwrap_or_default();
                // Amount used in this order for this discount card
                let used_value = old_value - available_value;

                tracing::debug!(
                    "[退款] 处理折扣卡 ID: {}, 原始面值: {}, 原始数量: {}, 总价值: {}, 当前余额: {}, 已使用金额: {}",
                    user_coupon.uc_id.unwrap_or_default(),
                    usage_value,
                    uc_count,
                    old_value,
                    available_value,
                    used_value
                );

                if total_amount > used_value {
                    // Refund the full value
                    user_coupon.available_value = Some(old_value.to_f64().unwrap_or_default());
                    tracing::debug!(
                        "[退款] 全额退款，新余额: {}, 剩余需退款金额: {}",
                        old_value,
                        total_amount - used_value
                    );
                    if !user_coupon.update(&mut tx).await? {
                        return Err(Error::internal("退还折扣卡失败"));
                    }
                    total_amount -= used_value;
                } else {
                    // Refund only part of the value
                    let new_available = available_value + total_amount;
                    user_coupon.available_value = Some(new_available.to_f64().unwrap_or_default());
                    tracing::debug!("[退款] 部分退款，新余额: {}, 退款完成", new_available);
                    if !user_coupon.update(&mut tx).await? {
                        return Err(Error::internal("退还折扣卡失败"));
                    }
                    break;
                }
            }
        } else {
            if let Some(user_coupon) = user_coupons.first_mut() {
                // Refund usage count for non-storage card coupons
                tracing::debug!(
                    "[退款] 处理优惠券退款 - 优惠券ID: {}, 当前使用次数: {}, 优惠券类型: {:?}",
                    user_coupon.uc_id.unwrap_or_default(),
                    user_coupon.uc_count.unwrap_or_default(),
                    user_coupon.coupon.as_ref().map_or(None, |c| c.coupon_type.as_ref())
                );

                if let Some(coupon) = &user_coupon.coupon {
                    match coupon.coupon_type {
                        Some(CouponType::DiscountCoupon) => {
                            tracing::debug!("[退款] 处理折扣券退款");
                            // 折扣券只需要恢复使用次数
                            user_coupon.uc_count = user_coupon.uc_count.map(|c| c + 1);
                        }
                        Some(CouponType::SpendAndSaveCard) => {
                            tracing::debug!("[退款] 处理满减券退款");
                            // 满减券只需要恢复使用次数
                            user_coupon.uc_count = user_coupon.uc_count.map(|c| c + 1);
                        }
                        _ => {
                            tracing::warn!("[退款] 未知的优惠券类型: {:?}", coupon.coupon_type);
                        }
                    }
                }

                tracing::debug!(
                    "[退款] 优惠券退款后 - 优惠券ID: {}, 新的使用次数: {}",
                    user_coupon.uc_id.unwrap_or_default(),
                    user_coupon.uc_count.unwrap_or_default()
                );

                if !user_coupon.update(&mut tx).await? {
                    return Err(Error::internal("退还优惠券失败"));
                }
            }
        }

        // update payment status
        payment.payment_status = Some(PaymentStatus::Refunded);
        if !payment.update_payment_status(&mut tx).await? {
            return Err(Error::internal("更新支付状态失败"));
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
        store_id: i64,
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
        let payment = Payment::get_by_order_id(pool, order_id, store_id).await?;
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
            user_coupons = UserCoupon::find_by_uc_ids(pool, store_id, &uc_ids).await?;
        }

        resp.payment = Some(payment);
        resp.user_coupons = user_coupons;

        Ok(resp)
    }

    pub async fn delete_orders(pool: &Pool<Sqlite>, ids: &[i64]) -> Result<()> {
        let mut tx = pool.begin().await?;
        for order_id in ids {
            // delete related order clothes
            let cloth_ids: Vec<String> = OrderCloth::get_by_order_id(pool, *order_id)
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
    let store_id = utils::get_user_id(&state).await?;
    order.store_id = Some(store_id);
    Ok(order.add_order(&state).await?)
}

#[tauri::command]
pub async fn get_orders_pagination(
    state: tauri::State<'_, AppState>,
    page_params: PageParams,
    mut order: Order,
) -> Result<PageResult<Order>> {
    let store_id = utils::get_user_id(&state).await?;
    order.store_id = Some(store_id);
    order.query_list(&state.pool, page_params).await
}

#[tauri::command]
pub async fn get_orders4home(
    state: tauri::State<'_, AppState>,
    mut order: Order,
) -> Result<Vec<Order>> {
    let store_id = utils::get_user_id(&state).await?;
    order.store_id = Some(store_id);
    order.query_list4home(&state.pool).await
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OrderQuery {
    pub user_id: i64,
    pub cloth_name: Option<String>,
    #[serde(deserialize_with = "deserialize_date")]
    pub end_time: Option<DateTime<FixedOffset>>,
    #[serde(deserialize_with = "deserialize_date")]
    pub start_time: Option<DateTime<FixedOffset>>,
}

#[tauri::command]
pub async fn get_orders4history(
    state: tauri::State<'_, AppState>,
    query: OrderQuery,
    page_params: PageParams,
) -> Result<PageResult<Order>> {
    let store_id = utils::get_user_id(&state).await?;
    Order::query_list4history(&state.pool, store_id, page_params, query).await
}

#[tauri::command]
pub async fn get_order_by_id(state: tauri::State<'_, AppState>, id: i64) -> Result<Option<Order>> {
    let store_id = utils::get_user_id(&state).await?;
    Order::get_by_id(&state.pool, store_id, id).await
}

#[tauri::command]
pub async fn update_order(state: tauri::State<'_, AppState>, mut order: Order) -> Result<bool> {
    let store_id = utils::get_user_id(&state).await?;
    order.store_id = Some(store_id);
    order.udpate_order(&state).await
}

#[tauri::command]
pub async fn delete_orders(state: tauri::State<'_, AppState>, ids: Vec<i64>) -> Result<()> {
    Order::delete_orders(&state.pool, &ids).await
}

/// update adjust data
#[tauri::command]
pub async fn update_adjust(state: tauri::State<'_, AppState>, order: Order) -> Result<bool> {
    let mut tx = state.pool.begin().await?;
    if let Some(adjust) = order.adjust {
        if !adjust.upsert(&mut tx).await? {
            return Err(Error::internal("update adjust data failed"));
        }
    }
    tx.commit().await?;
    Ok(true)
}

#[tauri::command]
pub async fn pay_order(state: tauri::State<'_, AppState>, req: PaymentReq) -> Result<()> {
    let store_id = utils::get_user_id(&state).await?;
    Order::pay(&state, store_id, req).await
}

#[tauri::command]
pub async fn get_refund_info(
    state: tauri::State<'_, AppState>,
    order_id: i64,
    user_id: i64,
) -> Result<RefundInfoResp> {
    let store_id = utils::get_user_id(&state).await?;
    Order::get_refund_info(&state.pool, store_id, order_id, user_id).await
}

#[tauri::command]
pub async fn refund_order(
    state: tauri::State<'_, AppState>,
    order_id: i64,
    refund_reason: String,
) -> Result<()> {
    let store_id = utils::get_user_id(&state).await?;
    Order::refund(&state.pool, store_id, order_id, refund_reason).await
}

#[tauri::command]
pub async fn get_count_by_user_id(state: tauri::State<'_, AppState>, user_id: i64) -> Result<u64> {
    let store_id = utils::get_user_id(&state).await?;
    let order = Order {
        user_id: Some(user_id),
        store_id: Some(store_id),
        ..Default::default()
    };
    order.count(&state.pool, None).await
}

/// 订单时效预警管理器
#[derive(Debug, Clone)]
pub struct TimeWarningManager {
    task_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl TimeWarningManager {
    pub fn new() -> Self {
        Self {
            task_handle: Arc::new(Mutex::new(None)),
        }
    }

    /// 启动时效检查任务
    pub async fn start<R: Runtime>(&self, app_handle: AppHandle<R>) -> Result<()> {
        let state = app_handle.state::<AppState>();
        let pool = state.pool.clone();
        let store_id = utils::get_user_id(&state).await?;
        let task_handle = self.task_handle.clone();

        // 先停止已存在的任务
        self.stop().await;

        // 创建新任务
        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600)); // 每小时检查一次
            loop {
                if let Err(e) = Order::check_time_warning(&pool, store_id).await {
                    tracing::error!("检查订单时效失败: {}", e);
                }
                interval.tick().await;
            }
        });

        // 保存任务句柄
        let mut handle_guard = task_handle.lock().await;
        *handle_guard = Some(handle);

        Ok(())
    }

    /// 停止时效检查任务
    pub async fn stop(&self) {
        let mut handle_guard = self.task_handle.lock().await;
        if let Some(handle) = handle_guard.take() {
            handle.abort();
        }
    }
}
