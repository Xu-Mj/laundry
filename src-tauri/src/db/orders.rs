use std::cmp::Reverse;
use std::collections::BinaryHeap;

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
    AlarmType, ClothStatus, CouponType, OrderStatus, PaymentMethod, PaymentStatus,
    ServiceRequirmentType,
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
use crate::payments::PaymentMethodDetail;
use crate::qrcode_payments::QrcodePayment;
use crate::state::AppState;
use crate::utils;
use crate::utils::chrono_serde::deserialize_date;
use crate::utils::request::Request;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use super::payments::CouponUsage;

// const PAY_STATUS_NOT_PAID: &str = "01";
// const PAY_STATUS_REFUND: &str = "05";
// const ORDER_STATUS_REFUND: &str = "06";
// const CLOTH_STATUS_REFUND: &str = "03";
// const PAY_STATUS_PAID: &str = "00";
const NORMAL_ORDER: &str = "00";
// const CLOTHING_STATUS_PICKED_UP: &str = "00";
// const REWASH_ORDER: &str = "02";
// const STATUS_LAUNDRY: &str = "01";
// const STATUS_COMPLETED: &str = "04";
// const NORMAL_ALARM: &str = "00";
// const WARNING_ALARM: &str = "01";
// const OVERDUE_ALARM: &str = "02";
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
    pub cost_time_alarm: Option<AlarmType>,
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
    pub uc_ids: Option<Vec<i64>>,
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
    pub user_coupons: Vec<UserCoupon>,
}

impl FromRow<'_, SqliteRow> for Order {
    fn from_row(row: &SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        // 创建Payment对象，使用别名字段
        let payment = if let Ok(pay_id) = row.try_get::<Option<String>, _>("pay_id") {
            if let Some(pay_id) = pay_id {
                // Extract payment JSON fields
                let payment_method_details_json: Option<String> =
                    row.try_get("payment_method_details")?;
                let coupon_usages_json: Option<String> = row.try_get("coupon_usages")?;

                // Parse JSON arrays if they exist
                let payment_method_details = if let Some(json) = payment_method_details_json {
                    serde_json::from_str(&json).map_err(|e| sqlx::Error::ColumnDecode {
                        index: "payment_method_details".to_string(),
                        source: Box::new(e),
                    })?
                } else {
                    Vec::new()
                };

                let coupon_usages = if let Some(json) = coupon_usages_json {
                    serde_json::from_str(&json).map_err(|e| sqlx::Error::ColumnDecode {
                        index: "coupon_usages".to_string(),
                        source: Box::new(e),
                    })?
                } else {
                    Vec::new()
                };

                Some(Payment {
                    pay_id: Some(pay_id),
                    pay_number: row.try_get("pay_number")?,
                    uc_order_id: row.try_get("uc_order_id")?,
                    order_type: row.try_get("p_order_type")?,
                    total_amount: row.try_get("total_amount")?,
                    payment_status: row.try_get("p_payment_status")?,
                    payment_method: row.try_get("payment_method")?,
                    create_time: row.try_get("p_create_time")?,
                    update_time: row.try_get("p_update_time")?,
                    store_id: row.try_get("p_store_id")?,
                    refund_reason: row.try_get("refund_reason")?,
                    payment_method_details,
                    coupon_usages,
                })
            } else {
                None
            }
        } else {
            None
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
            payment,
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
 p.payment_status as p_payment_status,
 p.payment_method,
 p.uc_order_id,
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
 a.remark as adjust_remark,
COALESCE(
    (SELECT json_group_array(json_object(
        'id', pmd.id,
        'paymentId', pmd.payment_id,
        'method', pmd.method,
        'amount', pmd.amount,
        'transactionId', pmd.transaction_id,
        'createTime', pmd.create_time
    )) 
    FROM payment_method_details pmd
    WHERE pmd.payment_id = p.pay_id),
    '[]'
) AS payment_method_details,

COALESCE(
(SELECT json_group_array(json_object(
    'id', cu.id,
    'paymentId', cu.payment_id,
    'couponId', cu.coupon_id,
    'couponType', cu.coupon_type,
    'appliedAmount', cu.applied_amount,
    'isRefunded', CASE WHEN cu.is_refunded = 1 THEN json('true') ELSE json('false') END  -- 转换为JSON布尔值
))
FROM coupon_usages cu
WHERE cu.payment_id = p.pay_id),
'[]'
) AS coupon_usages
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
    p.payment_status as p_payment_status,
    p.payment_method,
    p.uc_order_id,
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
    COALESCE(
    (SELECT json_group_array(json_object(
        'id', pmd.id,
        'paymentId', pmd.payment_id,
        'method', pmd.method,
        'amount', pmd.amount,
        'transactionId', pmd.transaction_id,
        'createTime', pmd.create_time
    )) 
    FROM payment_method_details pmd
    WHERE pmd.payment_id = p.pay_id),
    '[]'
    ) AS payment_method_details,

    COALESCE(
    (SELECT json_group_array(json_object(
        'id', cu.id,
        'paymentId', cu.payment_id,
        'couponId', cu.coupon_id,
        'couponType', cu.coupon_type,
        'appliedAmount', cu.applied_amount,
        'isRefunded', CASE WHEN cu.is_refunded = 1 THEN json('true') ELSE json('false') END  -- 转换为JSON布尔值
    ))
    FROM coupon_usages cu
    WHERE cu.payment_id = p.pay_id),
    '[]'
    ) AS coupon_usages
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

        self.cost_time_alarm.as_ref().map(|cost_time_alarm| {
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
        self.build_query(pool, " WHERE o.status != 'Completed' AND o.status != 'Cancelled' AND o.status != 'Refunded' ", None)
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
        status: OrderStatus,
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
            AND status NOT IN ('Completed', 'Cancelled', 'Refunded') 
            AND desire_complete_time IS NOT NULL",
        )
        .bind(store_id)
        .fetch_all(pool)
        .await?;

        let mut tx = pool.begin().await?;

        for mut order in orders {
            let Some(desire_time) = order.desire_complete_time else {
                continue;
            };

            // 时间转换优化
            let desire_datetime = desire_time
                .and_hms_opt(0, 0, 0)
                .map(|ndt| DateTime::<Local>::from_naive_utc_and_offset(ndt, now.offset().clone()))
                .unwrap_or(now);

            let current_alarm = order.cost_time_alarm.as_ref().unwrap_or(&AlarmType::Normal);
            let status_ref = &order.status;
            let new_alarm = match (desire_datetime < now, status_ref) {
                (true, Some(OrderStatus::ReadyForPickup)) => AlarmType::OverdueNotPickedUp,
                (true, _) => AlarmType::Overdue,
                (false, _) if desire_datetime <= warning_threshold => match status_ref {
                    Some(OrderStatus::ReadyForPickup) => AlarmType::Normal,
                    _ => AlarmType::Warning,
                },
                _ => AlarmType::Normal,
            };

            if *current_alarm == new_alarm {
                continue;
            }

            // 状态更新
            order.cost_time_alarm = Some(new_alarm.clone());
            if !order.update(&mut tx).await? {
                return Err(Error::internal("更新订单预警状态失败"));
            }

            // 日志记录优化
            let order_num = order.order_number.unwrap_or_default();
            match new_alarm {
                AlarmType::Overdue => {
                    tracing::warn!("订单 {} 已超时，期望完成时间: {}", order_num, desire_time)
                }
                AlarmType::Warning => {
                    tracing::info!("订单 {} 即将到期，期望完成时间: {}", order_num, desire_time)
                }
                AlarmType::Normal => tracing::info!(
                    "订单 {} 恢复正常状态，期望完成时间: {}",
                    order_num,
                    desire_time
                ),
                AlarmType::OverdueNotPickedUp => tracing::info!(
                    "订单 {} 未在规定时间内取件，期望完成时间: {}",
                    order_num,
                    desire_time
                ),
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
        self.cost_time_alarm = Some(AlarmType::Normal);
        self.business_type = Some(BUSINESS_MAIN.to_string());
        self.create_time = Some(utils::get_now());
    }

    fn cal_payment_method(order: &mut Order, payment: &mut Payment) {
        let total_amount = payment.total_amount.unwrap_or_default();

        // 使用模式匹配提升可读性
        match payment.payment_method {
            Some(PaymentMethod::AlipayAndStoredValueCard)
            | Some(PaymentMethod::WechatPayAndStoredValueCard)
            | Some(PaymentMethod::CashAndStoredValueCard) => {
                let paid = payment
                    .coupon_usages
                    .iter()
                    .map(|usage| (usage.applied_amount * 100.0).round() as i64)
                    .sum::<i64>() as f64
                    / 100.0;

                // 精确计算，避免浮点数精度问题
                order.payment_bonus_count = Some(paid);
                order.diff_price =
                    Some(((total_amount * 100.0) as i64 - (paid * 100.0) as i64) as f64 / 100.0);
            }
            Some(PaymentMethod::Meituan) | Some(PaymentMethod::Douyin) => {
                order.payment_bonus_count = Some(total_amount);
            }
            Some(PaymentMethod::StoredValueCard) | Some(PaymentMethod::DiscountCard) => {
                order.diff_price = Some(
                    payment
                        .coupon_usages
                        .iter()
                        .map(|usage| (usage.applied_amount * 100.0).round() as i64)
                        .sum::<i64>() as f64
                        / 100.0,
                );
            }
            Some(PaymentMethod::SessionCard) => {
                order.payment_bonus_count = Some(total_amount);
            }
            Some(PaymentMethod::CashAndSessionCard)
            | Some(PaymentMethod::AlipayAndSessionCard)
            | Some(PaymentMethod::WechatPayAndSessionCard) => {
                let paid = payment
                    .payment_method_details
                    .iter()
                    .filter(|detail| detail.method != Some(PaymentMethod::SessionCard))
                    .map(|detail| (detail.amount * 100.).round() as i64)
                    .sum::<i64>() as f64
                    / 100.0;

                order.payment_bonus_count =
                    Some(((total_amount * 100.0) as i64 - (paid * 100.0) as i64) as f64 / 100.0);
                order.diff_price = Some(paid);
            }
            _ => {
                order.payment_bonus_count = Some(0.0);
                order.diff_price = Some(total_amount);
            }
        }

        // 其他字段赋值
        order.payment_bonus_type = payment.payment_method.clone();
        order.payment_amount = payment.total_amount;
    }

    pub async fn get_refund_info(
        pool: &Pool<Sqlite>,
        store_id: i64,
        order_id: i64,
        user_id: i64,
    ) -> Result<RefundInfoResp> {
        let mut resp = RefundInfoResp::default();

        // 查询用户信息
        let user = User::get_by_id(pool, user_id)
            .await?
            .ok_or(Error::not_found("用户不存在"))?;

        resp.user = Some(user);

        // 查询支付信息通过订单ID
        let payment = Payment::get_by_order_id(pool, order_id, store_id).await?;
        if payment.is_none() {
            return Ok(resp);
        }

        let payment = payment.unwrap();

        let ids = payment
            .coupon_usages
            .iter()
            .map(|usage| usage.coupon_id)
            .collect::<Vec<_>>();
        let user_coupons = UserCoupon::find_by_uc_ids(pool, store_id, &ids).await?;

        resp.payment = Some(payment);
        resp.user_coupons = user_coupons;

        Ok(resp)
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
        let mut user_coupons: Vec<UserCoupon> = if let Some(ids) = &payment_req.uc_ids {
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
                total_payment_amount += order_total_amount;

                // 应用卡券或储值卡
                if !user_coupons.is_empty() {
                    if is_time_based {
                        Self::validate_time_based_on_payment_new(
                            &mut tr,
                            &mut payment,
                            &mut user_coupons,
                            &clothes,
                        )
                        .await?;
                    } else {
                        Self::validate_and_apply_coupons_new(
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

        // 设置支付的总金额
        payment.total_amount = Some(total_payment_amount);

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
                                payment.payment_status = Some(PaymentStatus::Paid);
                                payment.pay_number = existing_order.order_number.clone();
                                payment.uc_order_id = Some(*order_id);

                                // 添加支付宝支付方式明细
                                let alipay_payment = PaymentMethodDetail {
                                    id: None,
                                    transaction_id: alipay_result
                                        .trade_no
                                        .clone()
                                        .map(|s| s.parse::<i64>().unwrap_or_default()),
                                    store_id: Some(store_id),
                                    payment_id: payment.pay_id.clone().unwrap_or_default(),
                                    method: Some(PaymentMethod::Alipay),
                                    amount: total_payment_amount,
                                    payment_status: Some(PaymentStatus::Paid),

                                    creat_time: payment.create_time,
                                };
                                payment.payment_method_details = vec![alipay_payment];

                                let created_payment = payment.create_payment(&mut tr).await?;

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
                                    payment: created_payment,
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
                payment.payment_status = Some(PaymentStatus::Paid);
                payment.pay_number = existing_order.order_number.clone();
                payment.uc_order_id = Some(*order_id);

                let created_payment = payment.create_payment(&mut tr).await?;

                // save order and payment
                orders_with_payments.push(OrderWithPayment {
                    order: existing_order,
                    payment: created_payment,
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

    async fn validate_and_apply_coupons_new(
        tr: &mut Transaction<'_, Sqlite>,
        payment: &mut Payment,
        user_coupons: &mut Vec<UserCoupon>,
        total_amount: f64,
    ) -> Result<bool> {
        if user_coupons.is_empty() {
            return Ok(true);
        }

        let payment_method = payment
            .payment_method
            .as_ref()
            .ok_or(Error::bad_request("支付方式不能为空"))?;
        // 将输入值转换为Decimal，确保精度
        let total_amount_decimal = Decimal::from_f64(total_amount).unwrap_or_default();

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

        // 清空现有的支付方式明细和卡券使用记录
        payment.payment_method_details = Vec::new();
        payment.coupon_usages = Vec::new();

        // 储值卡处理逻辑
        if !storage_cards.is_empty() {
            // 按余额升序排序储值卡
            let mut sorted_storage_cards = storage_cards
                .into_iter()
                .filter(|card| card.available_value.unwrap_or(0.0) > 0.0)
                .collect::<Vec<_>>();
            sorted_storage_cards
                .sort_by(|a, b| a.available_value.partial_cmp(&b.available_value).unwrap());

            let mut remaining_amount = total_amount_decimal;

            for storage_card in sorted_storage_cards {
                let available_value = storage_card.available_value.unwrap_or(0.0);
                let available_value_decimal =
                    Decimal::from_f64(available_value).unwrap_or_default();

                // 获取卡券ID
                let coupon_id = storage_card.uc_id.unwrap_or_default();

                if available_value_decimal >= remaining_amount {
                    // 使用储值卡支付足够金额
                    let applied_amount = remaining_amount.to_f64().unwrap_or_default();
                    let new_balance = available_value_decimal - remaining_amount;
                    storage_card.available_value = Some(new_balance.to_f64().unwrap_or_default());

                    // 添加支付方式明细
                    let payment_method_detail = PaymentMethodDetail {
                        id: None,
                        transaction_id: None,
                        store_id: payment.store_id,
                        payment_id: payment.pay_id.clone().unwrap_or_default(),
                        method: Some(PaymentMethod::StoredValueCard),
                        amount: applied_amount,
                        payment_status: Some(PaymentStatus::Paid),

                        creat_time: payment.create_time,
                    };
                    payment.payment_method_details.push(payment_method_detail);

                    // 添加卡券使用记录
                    let coupon_usage = CouponUsage {
                        id: None,
                        payment_id: payment.pay_id.clone().unwrap_or_default(),
                        coupon_id: coupon_id,
                        coupon_type: CouponType::StoredValueCard,
                        applied_amount,
                        is_refunded: false,
                    };
                    payment.coupon_usages.push(coupon_usage);

                    // 更新用户卡券
                    if !storage_card.update(tr).await? {
                        return Err(Error::internal("update user coupon failed"));
                    }

                    remaining_amount = Decimal::ZERO;
                    break;
                } else {
                    // 使用储值卡支付部分金额
                    let applied_amount = available_value_decimal.to_f64().unwrap_or_default();
                    remaining_amount -= available_value_decimal;
                    storage_card.available_value = Some(0.0);

                    // 添加支付方式明细
                    let payment_method_detail = PaymentMethodDetail {
                        id: None,
                        transaction_id: None,
                        store_id: payment.store_id,
                        payment_id: payment.pay_id.clone().unwrap_or_default(),
                        method: Some(PaymentMethod::StoredValueCard),
                        amount: applied_amount,
                        payment_status: Some(PaymentStatus::Paid),

                        creat_time: payment.create_time,
                    };
                    payment.payment_method_details.push(payment_method_detail);

                    // 添加卡券使用记录
                    let coupon_usage = CouponUsage {
                        id: None,
                        payment_id: payment.pay_id.clone().unwrap_or_default(),
                        coupon_id: coupon_id,
                        coupon_type: CouponType::StoredValueCard,
                        applied_amount,
                        is_refunded: false,
                    };
                    payment.coupon_usages.push(coupon_usage);

                    // 更新用户卡券
                    if !storage_card.update(tr).await? {
                        return Err(Error::internal("update user coupon failed"));
                    }
                }
            }

            // 如果还有剩余金额，添加现金支付方式
            if remaining_amount > Decimal::ZERO {
                let cash_amount = remaining_amount.to_f64().unwrap_or_default();
                let cash_payment = PaymentMethodDetail {
                    id: None,
                    transaction_id: None,
                    store_id: payment.store_id,
                    payment_id: payment.pay_id.clone().unwrap_or_default(),
                    method: payment_method
                        .get_other_payment_method(&PaymentMethod::StoredValueCard),

                    amount: cash_amount,
                    payment_status: Some(PaymentStatus::Paid),

                    creat_time: payment.create_time,
                };
                payment.payment_method_details.push(cash_payment);
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

                // 获取卡券ID
                let coupon_id = discount_card.uc_id.unwrap_or_default();

                if available_value_decimal >= remaining_amount {
                    // 使用折扣卡支付足够金额
                    let applied_amount = remaining_amount.to_f64().unwrap_or_default();
                    let new_balance = available_value_decimal - remaining_amount;
                    discount_card.available_value = Some(new_balance.to_f64().unwrap_or_default());

                    // 添加支付方式明细
                    let payment_method_detail = PaymentMethodDetail {
                        id: None,
                        transaction_id: None,
                        store_id: payment.store_id,
                        payment_id: payment.pay_id.clone().unwrap_or_default(),
                        method: Some(PaymentMethod::DiscountCard),
                        amount: applied_amount,
                        payment_status: Some(PaymentStatus::Paid),

                        creat_time: payment.create_time,
                    };
                    payment.payment_method_details.push(payment_method_detail);

                    // 添加卡券使用记录
                    let coupon_usage = CouponUsage {
                        id: None,
                        payment_id: payment.pay_id.clone().unwrap_or_default(),
                        coupon_id: coupon_id,
                        coupon_type: CouponType::DiscountCard,
                        applied_amount,
                        is_refunded: false,
                    };
                    payment.coupon_usages.push(coupon_usage);

                    // 更新用户卡券
                    if !discount_card.update(tr).await? {
                        return Err(Error::internal("更新用户折扣卡失败"));
                    }

                    break;
                } else {
                    // 使用折扣卡支付部分金额
                    let applied_amount = available_value_decimal.to_f64().unwrap_or_default();
                    remaining_amount -= available_value_decimal;
                    discount_card.available_value = Some(0.0);

                    // 添加支付方式明细
                    let payment_method_detail = PaymentMethodDetail {
                        id: None,
                        transaction_id: None,
                        store_id: payment.store_id,
                        payment_id: payment.pay_id.clone().unwrap_or_default(),
                        method: Some(PaymentMethod::DiscountCard),
                        amount: applied_amount,
                        payment_status: Some(PaymentStatus::Paid),

                        creat_time: payment.create_time,
                    };
                    payment.payment_method_details.push(payment_method_detail);

                    // 添加卡券使用记录
                    let coupon_usage = CouponUsage {
                        id: None,
                        payment_id: payment.pay_id.clone().unwrap_or_default(),
                        coupon_id: coupon_id,
                        coupon_type: CouponType::DiscountCard,
                        applied_amount,
                        is_refunded: false,
                    };
                    payment.coupon_usages.push(coupon_usage);

                    // 更新用户卡券
                    if !discount_card.update(tr).await? {
                        return Err(Error::internal("更新用户折扣卡失败"));
                    }
                }
            }

            // 如果还有剩余金额，添加现金支付方式
            if remaining_amount > Decimal::ZERO {
                let cash_amount = remaining_amount.to_f64().unwrap_or_default();
                let cash_payment = PaymentMethodDetail {
                    id: None,
                    transaction_id: None,
                    store_id: payment.store_id,
                    payment_id: payment.pay_id.clone().unwrap_or_default(),
                    method: payment_method.get_other_payment_method(&PaymentMethod::DiscountCard),
                    amount: cash_amount,
                    payment_status: Some(PaymentStatus::Paid),

                    creat_time: payment.create_time,
                };
                payment.payment_method_details.push(cash_payment);
            }

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

            // 获取卡券ID
            let coupon_id = user_coupon.uc_id.unwrap_or_default();
            let coupon_type = coupon.coupon_type.clone().unwrap_or_default();

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
                let (final_amount, discount_amount) =
                    if coupon.coupon_type == Some(CouponType::DiscountCoupon) {
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
                        let discount_amount = if discounted > usage_limit {
                            usage_limit
                        } else {
                            discounted
                        };
                        tracing::debug!(
                            "[支付] 折扣券逻辑: 总金额: {}, 折扣：{}, 优惠金额: {}",
                            total_amount_decimal,
                            discount,
                            discount_amount
                        );
                        (total_amount_decimal - discount_amount, discount_amount)
                    } else {
                        // 满减券逻辑
                        let usage_value = Decimal::from_f64(coupon.usage_value.unwrap_or_default())
                            .unwrap_or_default();
                        let final_amount = (total_amount_decimal - usage_value)
                            .round_dp_with_strategy(2, RoundingStrategy::MidpointTowardZero);
                        (final_amount, usage_value)
                    };

                // 添加卡券使用记录
                let coupon_usage = CouponUsage {
                    id: None,
                    payment_id: payment.pay_id.clone().unwrap_or_default(),
                    coupon_id: coupon_id,
                    coupon_type: coupon_type,
                    applied_amount: discount_amount.to_f64().unwrap_or_default(),
                    is_refunded: false,
                };
                payment.coupon_usages.push(coupon_usage);

                // 添加现金支付方式明细 - 剩余金额
                if final_amount > Decimal::ZERO {
                    let cash_payment = PaymentMethodDetail {
                        id: None,
                        transaction_id: None,
                        store_id: payment.store_id,
                        payment_id: payment.pay_id.clone().unwrap_or_default(),
                        method: payment_method.get_coupon_another_method(),
                        amount: final_amount.to_f64().unwrap_or_default(),
                        payment_status: Some(PaymentStatus::Paid),

                        creat_time: payment.create_time,
                    };
                    payment.payment_method_details.push(cash_payment);
                }

                // update user coupon
                if !user_coupon.update(tr).await? {
                    return Err(Error::internal("update user coupon failed"));
                }
            }
        } else {
            // 如果没有使用优惠券，添加现金支付方式
            let cash_payment = PaymentMethodDetail {
                id: None,
                transaction_id: None,
                store_id: payment.store_id,
                payment_id: payment.pay_id.clone().unwrap_or_default(),
                method: payment.payment_method.clone(),
                amount: total_amount_decimal.to_f64().unwrap_or_default(),
                payment_status: Some(PaymentStatus::Paid),

                creat_time: payment.create_time,
            };
            payment.payment_method_details.push(cash_payment);
        }

        Ok(true)
    }

    async fn validate_time_based_on_payment_new(
        tr: &mut Transaction<'_, Sqlite>,
        payment: &mut Payment,
        user_coupons: &mut Vec<UserCoupon>,
        clothes: &[OrderCloth],
    ) -> Result<()> {
        // Get cloth count
        let mut cloth_count = clothes.len();
        let mut used_coupon_ids = Vec::with_capacity(cloth_count);
        let mut total_used_coupons = 0;

        // 清空现有的支付方式明细和卡券使用记录
        payment.payment_method_details = Vec::new();
        payment.coupon_usages = Vec::new();

        tracing::debug!("[次卡支付] 找到次卡数量: {}", user_coupons.len());

        for coupon in user_coupons.iter_mut() {
            if let Some(uc_id) = coupon.uc_id {
                tracing::debug!("[次卡支付] 处理次卡ID: {}", uc_id);

                let usable_count = coupon.available_value.unwrap_or_default() as usize;

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

                    // 添加卡券使用记录
                    let coupon_usage = CouponUsage {
                        id: None,
                        payment_id: payment.pay_id.clone().unwrap_or_default(),
                        coupon_id: uc_id,
                        coupon_type: CouponType::SessionCard,
                        applied_amount: cloth_count as f64,
                        is_refunded: false,
                    };
                    payment.coupon_usages.push(coupon_usage);

                    // 更新数据库中的卡券
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
                    tracing::debug!("[次卡支付] 次卡ID: {}, 不足以覆盖所有衣物，部分使用", uc_id);

                    cloth_count -= usable_count;
                    coupon.available_value = Some(0f64);

                    // 添加卡券使用记录
                    let coupon_usage = CouponUsage {
                        id: None,
                        payment_id: payment.pay_id.clone().unwrap_or_default(),
                        coupon_id: uc_id,
                        coupon_type: CouponType::SessionCard,
                        applied_amount: usable_count as f64,
                        is_refunded: false,
                    };
                    payment.coupon_usages.push(coupon_usage);

                    tracing::debug!(
                        "[次卡支付] 次卡ID: {}, 已用完，剩余衣物数: {}",
                        uc_id,
                        cloth_count
                    );

                    // 更新数据库中的卡券
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
            // tracing::error!("[次卡支付] 次卡不足，剩余未覆盖衣物: {}", cloth_count);
            // return Err(Error::bad_request("所选次卡数量不足"));
            let price_diff = Self::calculate_top_x_total_price(clothes, cloth_count);
            tracing::debug!("[次卡支付] 剩余衣物价格: {}", price_diff);
            payment.payment_method_details.push(PaymentMethodDetail {
                id: None,
                transaction_id: None,
                store_id: payment.store_id,
                payment_id: payment.pay_id.clone().unwrap_or_default(),
                method: payment
                    .payment_method
                    .as_ref()
                    .map(|method| method.get_coupon_another_method().unwrap()),
                amount: price_diff,
                payment_status: Some(PaymentStatus::Paid),

                creat_time: payment.create_time,
            })
        }

        tracing::debug!("[次卡支付] 验证完成，次卡支付有效");
        Ok(())
    }

    fn calculate_top_x_total_price(order_cloths: &[OrderCloth], x: usize) -> f64 {
        // 如果 x 为 0 或数组为空，直接返回 0
        if x == 0 || order_cloths.is_empty() {
            return 0.0;
        }

        // 创建一个最小堆，用于保存前 x 个最大的价格
        let mut min_heap = BinaryHeap::new();

        for cloth in order_cloths {
            // 提取价格，如果不存在则跳过
            if let Some(price) = cloth
                .cloth_info
                .as_ref()
                .and_then(|info| info.clothing_base_price)
            {
                let price = (price * 100.0) as i64;
                // 如果堆的大小小于 x，直接添加
                if min_heap.len() < x {
                    min_heap.push(Reverse(price));
                }
                // 如果当前价格大于堆中最小的价格，替换它
                else if let Some(Reverse(min_price)) = min_heap.peek() {
                    if price > *min_price {
                        min_heap.pop();
                        min_heap.push(Reverse(price));
                    }
                }
            }
        }

        // 计算堆中所有价格的总和
        min_heap
            .into_iter()
            .map(|Reverse(price)| price)
            .sum::<i64>() as f64
            / 100.0
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

        // 如果没有卡券使用记录，简单更新支付状态并退还积分
        if payment.coupon_usages.is_empty() {
            // update payment status
            payment.payment_status = Some(PaymentStatus::Refunded);
            if !payment.refund(&mut tx).await? {
                return Err(Error::internal("更新支付状态失败"));
            }
            // refund user points
            if let Some(total_amount) = payment.total_amount {
                if total_amount > 0.0 {
                    if !User::decrease_points(
                        &mut tx,
                        order.user_id.unwrap_or_default(),
                        total_amount as i64,
                    )
                    .await?
                    {
                        return Err(Error::internal("退还积分失败"));
                    }
                }
            }
            tx.commit().await?;
            return Ok(());
        }

        // 处理卡券退款
        let order_clothes = OrderCloth::get_by_order_id(pool, order_id).await?;
        tracing::debug!("[退款] 订单包含的衣物数量: {}", order_clothes.len());

        // 获取所有涉及的卡券ID
        let coupon_ids: Vec<i64> = payment
            .coupon_usages
            .iter()
            .map(|usage| usage.coupon_id)
            .collect();

        // 查询所有涉及的用户卡券
        let mut user_coupons = UserCoupon::find_by_uc_ids(pool, store_id, &coupon_ids).await?;

        // 按照卡券使用记录退还
        for usage in &payment.coupon_usages {
            // 找到对应的用户卡券
            if let Some(user_coupon) = user_coupons
                .iter_mut()
                .find(|uc| uc.uc_id == Some(usage.coupon_id))
            {
                match usage.coupon_type {
                    CouponType::SessionCard => {
                        // 次卡退款：退还使用次数
                        let times_to_refund = usage.applied_amount as i32;
                        let current_available = user_coupon.available_value.unwrap_or(0.0) as i32;
                        let new_available = current_available + times_to_refund;
                        user_coupon.available_value = Some(new_available as f64);

                        tracing::debug!(
                            "[退款] 退还给次卡 ID: {} 的次数: {}, 新的剩余次数: {}",
                            user_coupon.uc_id.unwrap_or_default(),
                            times_to_refund,
                            new_available
                        );
                    }
                    CouponType::StoredValueCard | CouponType::DiscountCard => {
                        // 储值卡和折扣卡退款：退还金额
                        let amount_to_refund = usage.applied_amount;
                        let current_available = user_coupon.available_value.unwrap_or(0.0);
                        let new_available = current_available + amount_to_refund;
                        user_coupon.available_value = Some(new_available);

                        tracing::debug!(
                            "[退款] 退还给{}卡 ID: {} 的金额: {}, 新余额: {}",
                            if usage.coupon_type == CouponType::StoredValueCard {
                                "储值"
                            } else {
                                "折扣"
                            },
                            user_coupon.uc_id.unwrap_or_default(),
                            amount_to_refund,
                            new_available
                        );
                    }
                    CouponType::DiscountCoupon | CouponType::SpendAndSaveCard => {
                        // 优惠券退款：增加使用次数
                        user_coupon.uc_count = user_coupon.uc_count.map(|c| c + 1);

                        tracing::debug!(
                            "[退款] 优惠券退款 - 优惠券ID: {}, 新的使用次数: {}",
                            user_coupon.uc_id.unwrap_or_default(),
                            user_coupon.uc_count.unwrap_or_default()
                        );
                    }
                }

                // 更新数据库中的用户卡券
                if !user_coupon.update(&mut tx).await? {
                    return Err(Error::internal("退还卡券失败"));
                }
            }
        }

        // 更新支付状态
        payment.payment_status = Some(PaymentStatus::Refunded);
        if !payment.refund(&mut tx).await? {
            return Err(Error::internal("更新支付状态失败"));
        }

        // 退还用户积分
        if let Some(total_amount) = payment.total_amount {
            if total_amount > 0.0 {
                if !User::decrease_points(
                    &mut tx,
                    order.user_id.unwrap_or_default(),
                    total_amount as i64,
                )
                .await?
                {
                    return Err(Error::internal("退还积分失败"));
                }
            }
        }

        tx.commit().await?;
        Ok(())
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
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60)); // 每小时检查一次
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
