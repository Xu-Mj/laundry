use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{
    FromRow, Pool, Row, Sqlite, Transaction,
    types::chrono::{DateTime, FixedOffset},
};
use std::collections::HashMap;
use tauri::State;

use crate::constants::{CouponType, PaymentMethod, PaymentOrderType, PaymentStatus};
use crate::db::Validator;
use crate::error::{Error, Result};
use crate::state::AppState;
use crate::utils;

// 支付主表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub pay_id: Option<String>,
    pub pay_number: Option<String>,
    pub uc_order_id: Option<i64>,
    pub order_type: Option<PaymentOrderType>,
    pub total_amount: Option<f64>,
    pub payment_status: Option<PaymentStatus>,
    pub payment_method: Option<PaymentMethod>,
    pub create_time: Option<i64>,
    pub update_time: Option<i64>,
    pub store_id: Option<i64>,
    pub refund_reason: Option<String>,
    pub payment_method_details: Vec<PaymentMethodDetail>,
    pub coupon_usages: Vec<CouponUsage>,
}

// 支付方式明细表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethodDetail {
    pub id: Option<i64>,
    pub store_id: Option<i64>,
    pub transaction_id: Option<i64>,
    pub payment_id: String,
    pub method: Option<PaymentMethod>,
    pub amount: f64,
    pub payment_status: Option<PaymentStatus>,
    pub creat_time: Option<i64>,
}

// 卡券使用记录表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CouponUsage {
    pub id: Option<i64>,
    pub payment_id: String,
    pub coupon_id: i64,
    pub coupon_type: CouponType,
    pub applied_amount: f64,
    pub is_refunded: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PaymentSummary {
    pub date: String,
    pub income: f64,
    pub expense: f64,
}

impl Validator for Payment {
    fn validate(&self) -> Result<()> {
        if self.pay_number.is_none() {
            return Err(Error::bad_request("pay_number is required"));
        }
        if self.uc_order_id.is_none() {
            // 新增校验
            return Err(Error::bad_request("uc_order_id is required"));
        }
        if self.order_type.is_none() {
            return Err(Error::bad_request("order_type is required"));
        }

        if self.total_amount.is_none() {
            return Err(Error::bad_request("total_amount is required"));
        }

        if self.payment_status.is_none() {
            return Err(Error::bad_request("payment_status is required"));
        }

        Ok(())
    }
}

const SQL: &str = "SELECT 
    p.*,
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
FROM 
    payments p ";

impl FromRow<'_, SqliteRow> for Payment {
    fn from_row(row: &SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        // Extract the JSON arrays from the query result
        let payment_method_details_json: String = row.try_get("payment_method_details")?;
        let coupon_usages_json: String = row.try_get("coupon_usages")?;

        tracing::debug!(
            "payment_method_details_json: {}",
            payment_method_details_json
        );
        tracing::debug!("coupon_usages_json: {}", coupon_usages_json);
        // Parse the JSON arrays into vectors of the respective types
        let payment_method_details: Vec<PaymentMethodDetail> =
            serde_json::from_str(&payment_method_details_json).map_err(|e| {
                sqlx::Error::ColumnDecode {
                    index: "payment_method_details".to_string(),
                    source: Box::new(e),
                }
            })?;

        let coupon_usages: Vec<CouponUsage> =
            serde_json::from_str(&coupon_usages_json).map_err(|e| sqlx::Error::ColumnDecode {
                index: "coupon_usages".to_string(),
                source: Box::new(e),
            })?;

        // Create and return the Payment object
        Ok(Self {
            pay_id: row.try_get("pay_id")?,
            pay_number: row.try_get("pay_number")?,
            uc_order_id: row.try_get("uc_order_id")?,
            order_type: row.try_get("order_type")?,
            total_amount: row.try_get("total_amount")?,
            payment_status: row.try_get("payment_status")?,
            payment_method: row.try_get("payment_method")?,
            create_time: row.try_get("create_time")?,
            update_time: row.try_get("update_time")?,
            store_id: row.try_get("store_id")?,
            refund_reason: row.try_get("refund_reason")?,
            payment_method_details,
            coupon_usages,
        })
    }
}

impl FromRow<'_, SqliteRow> for PaymentMethodDetail {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id").ok(),
            transaction_id: row.try_get("transaction_id")?,
            store_id: row.try_get("store_id")?,
            payment_id: row.try_get("payment_id").unwrap_or_default(),
            method: row.try_get("method").unwrap_or_default(),
            amount: row.try_get("amount").unwrap_or_default(),
            payment_status: row.try_get("payment_status")?,
            creat_time: row.try_get("creat_time")?,
        })
    }
}

impl FromRow<'_, SqliteRow> for CouponUsage {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id").ok(),
            payment_id: row.try_get("payment_id").unwrap_or_default(),
            coupon_id: row.try_get("coupon_id").unwrap_or_default(),
            coupon_type: row.try_get("coupon_type").unwrap_or_default(),
            applied_amount: row.try_get("applied_amount").unwrap_or_default(),
            is_refunded: row.try_get("is_refunded").unwrap_or_default(),
        })
    }
}

impl Payment {
    pub async fn create_payment(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<Payment> {
        let now = utils::get_timestamp();

        // 首先插入主支付记录
        let query = r#"
        INSERT INTO payments (
            pay_id, pay_number, uc_order_id, order_type, total_amount,
            payment_status, payment_method, 
            create_time, update_time, store_id, refund_reason
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;

        sqlx::query(query)
            .bind(&self.pay_id)
            .bind(&self.pay_number)
            .bind(&self.uc_order_id)
            .bind(&self.order_type)
            .bind(&self.total_amount)
            .bind(&self.payment_status)
            .bind(&self.payment_method)
            .bind(now)
            .bind(now)
            .bind(&self.store_id)
            .bind(&self.refund_reason)
            .execute(&mut **tr)
            .await?;

        // 批量插入支付方式明细
        if !self.payment_method_details.is_empty() {
            let mut query_builder = sqlx::QueryBuilder::new(
                "INSERT INTO payment_method_details (
                        payment_id, 
                        method, 
                        amount, 
                        transaction_id,
                        store_id,
                        payment_status,
                        create_time
                    ) ",
            );

            query_builder.push_values(&self.payment_method_details, |mut b, detail| {
                b.push_bind(&self.pay_id)
                    .push_bind(&detail.method)
                    .push_bind(detail.amount)
                    .push_bind(detail.transaction_id)
                    .push_bind(detail.store_id)
                    .push_bind(&detail.payment_status)
                    .push_bind(now);
            });

            query_builder.build().execute(&mut **tr).await?;
        }

        // 批量插入卡券使用记录
        if !self.coupon_usages.is_empty() {
            let mut query_builder = sqlx::QueryBuilder::new(
                "INSERT INTO coupon_usages (payment_id, coupon_id, coupon_type, applied_amount, is_refunded) ",
            );

            query_builder.push_values(&self.coupon_usages, |mut b, usage| {
                b.push_bind(&self.pay_id)
                    .push_bind(usage.coupon_id)
                    .push_bind(&usage.coupon_type)
                    .push_bind(usage.applied_amount)
                    .push_bind(usage.is_refunded);
            });

            query_builder.build().execute(&mut **tr).await?;
        }

        // 返回完整的支付记录（包含关联数据）
        Self::get_by_pay_id_with_details(tr, self.pay_id.as_deref().unwrap()).await
    }

    pub async fn get_by_order_id(
        pool: &Pool<Sqlite>,
        order_id: i64,
        store_id: i64,
    ) -> Result<Option<Self>> {
        // 首先获取主支付记录
        let payment = sqlx::query_as(&format!("{SQL} WHERE p.uc_order_id =? AND p.store_id =?"))
            .bind(order_id)
            .bind(store_id)
            .fetch_optional(pool)
            .await?;

        Ok(payment)
    }

    pub async fn get_by_pay_id_with_details(
        executor: &mut Transaction<'_, Sqlite>,
        pay_id: &str,
    ) -> Result<Payment> {
        // 获取主支付记录
        let payment: Payment = sqlx::query_as(&format!("{SQL} WHERE p.pay_id =?"))
            .bind(pay_id)
            .fetch_one(&mut **executor)
            .await?;

        Ok(payment)
    }

    pub async fn cal_total_amount(pool: &Pool<Sqlite>, user_id: i64, store_id: i64) -> Result<f64> {
        let result = sqlx::query_scalar(
            "
            SELECT COALESCE(SUM(p.total_amount), 0.0)
            FROM payments p
            INNER JOIN orders o ON p.uc_order_id = o.order_id
            WHERE o.user_id = ? 
                AND p.store_id = ? 
                AND p.payment_status = 'Paid'
                AND p.total_amount > 0
                AND p.order_type = ?
            ",
        )
        .bind(user_id)
        .bind(store_id)
        .bind(PaymentOrderType::Laundry)
        .fetch_optional(pool)
        .await?
        .unwrap_or(0.0);

        Ok(result)
    }

    pub async fn refund(&self, tx: &mut Transaction<'_, Sqlite>) -> Result<bool> {
        if self.pay_id.is_none() {
            return Err(Error::bad_request("pay_id is required"));
        }

        let query = r#"
        UPDATE payments SET 
            payment_status = ?, 
            refund_reason = ?,
            update_time = ?
        WHERE pay_id = ?
        "#;

        let result = sqlx::query(query)
            .bind(&self.payment_status)
            .bind(&self.refund_reason)
            .bind(utils::get_timestamp())
            .bind(&self.pay_id)
            .execute(&mut **tx)
            .await?;

        if result.rows_affected() == 0 {
            return Err(Error::not_found("退款失败"));
        }
        // update details
        let result =
            PaymentMethodDetail::mark_as_refunded(tx, &self.pay_id.clone().unwrap()).await?;
        if !result {
            return Err(Error::bad_request("退款失败"));
        }

        CouponUsage::mark_as_refunded(tx, &self.pay_id.clone().unwrap()).await?;
        Ok(result)
    }

    // 新增：获取用户的支付历史（包含明细）
    pub async fn get_user_payment_history(
        pool: &Pool<Sqlite>,
        user_id: i64,
        store_id: i64,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Self>> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        let query = r#"
        SELECT p.* FROM payments p
        INNER JOIN orders o ON p.uc_order_id = o.order_id
        WHERE o.user_id = ? AND p.store_id = ?
        ORDER BY p.create_time DESC
        LIMIT ? OFFSET ?
        "#;

        let payments: Vec<Payment> = sqlx::query_as(query)
            .bind(user_id)
            .bind(store_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        let mut result = Vec::new();
        for mut payment in payments {
            let pay_id = payment.pay_id.clone().unwrap_or_default();

            // 获取支付方式明细
            let details_query = "SELECT * FROM payment_method_details WHERE payment_id = ?";
            let details: Vec<PaymentMethodDetail> = sqlx::query_as(details_query)
                .bind(&pay_id)
                .fetch_all(pool)
                .await?;
            payment.payment_method_details = details;

            // 获取卡券使用记录
            let usages_query = "SELECT * FROM coupon_usages WHERE payment_id = ?";
            let usages: Vec<CouponUsage> = sqlx::query_as(usages_query)
                .bind(&pay_id)
                .fetch_all(pool)
                .await?;
            payment.coupon_usages = usages;

            result.push(payment);
        }

        Ok(result)
    }

    // 新增：统计支付方式使用情况
    pub async fn get_payment_method_stats(
        pool: &Pool<Sqlite>,
        store_id: i64,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<Vec<(PaymentMethod, f64, i64)>> {
        // (支付方式, 总金额, 使用次数)
        let mut query = String::from(
            r#"
        SELECT 
            pmd.method,
            COALESCE(SUM(pmd.amount), 0.0) as total_amount,
            COUNT(*) as usage_count
        FROM payment_method_details pmd
        INNER JOIN payments p ON pmd.payment_id = p.pay_id
        WHERE p.store_id = ? AND p.payment_status = 'Paid'
        "#,
        );

        let mut bindings = vec![store_id.to_string()];

        if let Some(start) = start_time {
            query.push_str(" AND p.create_time >= ?");
            bindings.push(start.to_string());
        }

        if let Some(end) = end_time {
            query.push_str(" AND p.create_time <= ?");
            bindings.push(end.to_string());
        }

        query.push_str(" GROUP BY pmd.method ORDER BY total_amount DESC");

        let rows = sqlx::query(&query).bind(store_id).fetch_all(pool).await?;

        let mut result = Vec::new();
        for row in rows {
            let method: PaymentMethod = row.try_get("method")?;
            let total_amount: f64 = row.try_get("total_amount")?;
            let usage_count: i64 = row.try_get("usage_count")?;
            result.push((method, total_amount, usage_count));
        }

        Ok(result)
    }
}

impl PaymentMethodDetail {
    pub async fn mark_as_refunded(tx: &mut Transaction<'_, Sqlite>, pay_id: &str) -> Result<bool> {
        let result =
            sqlx::query("UPDATE payment_method_details SET payment_status = 'Refunded' WHERE payment_id =?")
                .bind(pay_id)
                .execute(&mut **tx)
                .await?;

        Ok(result.rows_affected() > 0)
    }
}

// CouponUsage 的相关实现
impl CouponUsage {
    pub async fn mark_as_refunded(tx: &mut Transaction<'_, Sqlite>, pay_id: &str) -> Result<bool> {
        let result =
            sqlx::query("UPDATE coupon_usages SET is_refunded = true WHERE payment_id = ?")
                .bind(pay_id)
                .execute(&mut **tx)
                .await?;

        Ok(result.rows_affected() > 0)
    }

    // 新增：获取卡券的使用历史
    pub async fn get_by_coupon_id(
        pool: &Pool<Sqlite>,
        coupon_id: i64,
        store_id: i64,
    ) -> Result<Vec<Self>> {
        let query = r#"
        SELECT cu.* FROM coupon_usages cu
        INNER JOIN payments p ON cu.payment_id = p.pay_id
        WHERE cu.coupon_id = ? AND p.store_id = ?
        ORDER BY cu.id DESC
        "#;

        let usages = sqlx::query_as(query)
            .bind(coupon_id)
            .bind(store_id)
            .fetch_all(pool)
            .await?;

        Ok(usages)
    }

    // 新增：获取指定时间范围内的卡券使用统计
    pub async fn get_usage_stats(
        pool: &Pool<Sqlite>,
        store_id: i64,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<Vec<(CouponType, f64, i64)>> {
        // (卡券类型, 总使用金额, 使用次数)
        let mut query = String::from(
            r#"
        SELECT 
            cu.coupon_type,
            COALESCE(SUM(cu.applied_amount), 0.0) as total_amount,
            COUNT(*) as usage_count
        FROM coupon_usages cu
        INNER JOIN payments p ON cu.payment_id = p.pay_id
        WHERE p.store_id = ? AND p.payment_status = 'Paid'
        "#,
        );

        if start_time.is_some() {
            query.push_str(" AND p.create_time >= ?");
        }

        if end_time.is_some() {
            query.push_str(" AND p.create_time <= ?");
        }

        query.push_str(" GROUP BY cu.coupon_type ORDER BY total_amount DESC");

        let mut query_builder = sqlx::query(&query).bind(store_id);

        if let Some(start) = start_time {
            query_builder = query_builder.bind(start);
        }

        if let Some(end) = end_time {
            query_builder = query_builder.bind(end);
        }

        let rows = query_builder.fetch_all(pool).await?;

        let mut result = Vec::new();
        for row in rows {
            let coupon_type: CouponType = row.try_get("coupon_type")?;
            let total_amount: f64 = row.try_get("total_amount")?;
            let usage_count: i64 = row.try_get("usage_count")?;
            result.push((coupon_type, total_amount, usage_count));
        }

        Ok(result)
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

pub async fn get_monthly_payment_summary(
    pool: &Pool<Sqlite>,
    store_id: i64,
    year: Option<i32>,
    month: Option<u32>,
) -> Result<Vec<PaymentSummary>> {
    let now = utils::get_now().naive_local();
    let year = year.unwrap_or(now.year());
    let month = month.unwrap_or(now.month());

    let start_of_month = NaiveDate::from_ymd_opt(year, month, 1)
        .ok_or(Error::bad_request("Invalid date"))?
        .and_hms_opt(0, 0, 0)
        .ok_or(Error::bad_request("Invalid time"))?;
    let end_of_month = NaiveDate::from_ymd_opt(year, month, days_in_month(year, month))
        .ok_or(Error::bad_request("Invalid date"))?
        .and_hms_opt(23, 59, 59)
        .ok_or(Error::bad_request("Invalid time"))?;

    // 转换为时间戳，用于查询
    let start_timestamp = start_of_month.and_utc().timestamp_millis();
    let end_timestamp = end_of_month.and_utc().timestamp_millis();

    // 获取该月份所有日期
    let mut all_dates = Vec::new();
    let mut current_date = start_of_month;
    while current_date <= end_of_month {
        all_dates.push(current_date.format("%Y-%m-%d").to_string());
        current_date = current_date + Duration::days(1);
    }

    // 查询正常收入数据（排除储值卡支付，避免重复统计）
    let normal_income_data: HashMap<String, f64> = sqlx::query(
        r#"
        SELECT
            strftime('%Y-%m-%d', datetime(create_time/1000, 'unixepoch', 'localtime')) as date,
            SUM(amount) as income
        FROM payment_method_details
        WHERE create_time BETWEEN ? AND ?
          AND payment_status = 'Paid'
          AND amount > 0
          AND method != ? -- 排除储值卡支付
          AND method != ? -- 排除折扣卡支付
          AND store_id = ?
        GROUP BY date
        ORDER BY date
        "#,
    )
    .bind(start_timestamp)
    .bind(end_timestamp)
    .bind(store_id)
    .bind(PaymentMethod::StoredValueCard.to_string())
    .bind(PaymentMethod::DiscountCard.to_string())
    .map(|row: SqliteRow| {
        let date: String = row.get("date");
        let income: f64 = row.get("income");
        (date, income)
    })
    .fetch_all(pool)
    .await?
    .into_iter()
    .collect();

    // 查询支出数据
    let expense_data: HashMap<String, f64> = sqlx::query(
        r#"
        SELECT
            strftime('%Y-%m-%d', datetime(create_time/1000, 'unixepoch')) as date,
            SUM(exp_amount) as expense
        FROM expenditure
        WHERE create_time BETWEEN ? AND ?
        AND store_id = ?
        GROUP BY date
        ORDER BY date
        "#,
    )
    .bind(start_timestamp)
    .bind(end_timestamp)
    .bind(store_id)
    .map(|row: SqliteRow| {
        let date: String = row.get("date");
        let expense: i64 = row.get("expense");
        (date, expense as f64)
    })
    .fetch_all(pool)
    .await?
    .into_iter()
    .collect();

    // 合并数据，总收入 = 正常销售收入 + 储值卡销售收入
    let mut summaries = Vec::new();
    for date in all_dates {
        let income = *normal_income_data.get(&date).unwrap_or(&0.0);
        // let storage_card_income = *storage_card_income_data.get(&date).unwrap_or(&0.0);
        // let total_income = normal_income + storage_card_income;
        let expense = *expense_data.get(&date).unwrap_or(&0.0);

        summaries.push(PaymentSummary {
            date,
            income,
            expense,
        });
    }

    Ok(summaries)
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BarChartData {
    pub label: String,
    pub income: f64,
    pub expense: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSummaryWithRate {
    income: f64,       // 收入总额
    expense: f64,      // 支出总额
    income_rate: f64,  // 增长率百分比
    expense_rate: f64, // 增长率百分比
    label: String,     // 时间标签: today/week
}

/// 获取指定日期的收支数据
pub async fn get_day_amounts(
    pool: &Pool<Sqlite>,
    store_id: i64,
    date: DateTime<FixedOffset>,
) -> Result<(f64, f64)> {
    // 返回 (income, expense)
    let start_of_day = NaiveDate::from_ymd_opt(date.year(), date.month(), date.day())
        .ok_or(Error::bad_request("Invalid date"))?
        .and_hms_opt(0, 0, 0)
        .ok_or(Error::bad_request("Invalid time"))?;
    let end_of_day = NaiveDate::from_ymd_opt(date.year(), date.month(), date.day())
        .ok_or(Error::bad_request("Invalid date"))?
        .and_hms_opt(23, 59, 59)
        .ok_or(Error::bad_request("Invalid time"))?;

    query_amounts(pool, store_id, start_of_day, end_of_day).await
}

// 修改现有的日统计方法
pub async fn get_daily_payment_summary(
    pool: &Pool<Sqlite>,
    store_id: i64,
) -> Result<PaymentSummaryWithRate> {
    let now = utils::get_now();

    // 获取当天数据
    let (current_income, current_expense) = get_day_amounts(pool, store_id, now).await?;

    // 获取前一天数据
    let prev_day = now - Duration::days(1);
    let (prev_income, prev_expense) = get_day_amounts(pool, store_id, prev_day).await?;

    Ok(PaymentSummaryWithRate {
        income: current_income,
        expense: current_expense,
        income_rate: calculate_rate(current_income, prev_income),
        expense_rate: calculate_rate(current_expense, prev_expense),
        label: "today".into(),
    })
}

// 补充周统计方法（需要同时修改）
pub async fn get_weekly_payment_summary(
    pool: &Pool<Sqlite>,
    store_id: i64,
) -> Result<PaymentSummaryWithRate> {
    let now = utils::get_now();

    // 获取本周数据
    let (current_income, current_expense) = get_week_amounts(pool, store_id, now).await?;

    // 获取上周数据
    let last_week = now - Duration::weeks(1);
    let (prev_income, prev_expense) = get_week_amounts(pool, store_id, last_week).await?;

    Ok(PaymentSummaryWithRate {
        income: current_income,
        expense: current_expense,
        income_rate: calculate_rate(current_income, prev_income),
        expense_rate: calculate_rate(current_expense, prev_expense),
        label: "week".into(),
    })
}

/// 辅助方法：获取指定周的数据
async fn get_week_amounts(
    pool: &Pool<Sqlite>,
    store_id: i64,
    date: DateTime<FixedOffset>,
) -> Result<(f64, f64)> {
    let naive_date = date.naive_local();

    // 计算周开始（周一）和结束（周日）
    let days_from_monday = naive_date.weekday().num_days_from_monday() as i64;
    let start_of_week = naive_date - Duration::days(days_from_monday);
    let end_of_week = start_of_week + Duration::days(6);
    query_amounts(pool, store_id, start_of_week, end_of_week).await
}

async fn query_amounts(
    pool: &Pool<Sqlite>,
    store_id: i64,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Result<(f64, f64)> {
    let start = start.and_utc().timestamp_millis();
    let end = end.and_utc().timestamp_millis();

    // 正常销售收入（排除储值卡支付的交易，避免重复统计）
    let income: f64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(amount), 0.0)
        FROM payment_method_details
        WHERE payment_status = 'Paid'
          AND amount > 0
          AND store_id =?
          AND method NOT IN (?, ?)
          AND create_time BETWEEN ? AND ?
        "#,
    )
    .bind(store_id)
    .bind(PaymentMethod::StoredValueCard)
    .bind(PaymentMethod::DiscountCard)
    .bind(start)
    .bind(end)
    .fetch_one(pool)
    .await?;

    // 支出查询
    let expense: i64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(exp_amount), 0)
        FROM expenditure
        WHERE create_time BETWEEN ? AND ?
        AND store_id = ?
        "#,
    )
    .bind(start)
    .bind(end)
    .bind(store_id)
    .fetch_one(pool)
    .await?;

    Ok((income, expense as f64))
}

/// 增长率计算函数
fn calculate_rate(current: f64, previous: f64) -> f64 {
    if previous == 0.0 {
        return if current > 0.0 { 100.0 } else { 0.0 };
    }
    ((current - previous) / previous * 100.0).round()
}

#[tauri::command]
pub async fn get_total_amount(state: State<'_, AppState>, user_id: i64) -> Result<f64> {
    let store_id = utils::get_user_id(&state).await?;
    Payment::cal_total_amount(&state.pool, user_id, store_id).await
}
