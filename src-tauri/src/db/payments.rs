use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{
    FromRow, Pool, Row, Sqlite, Transaction,
    types::chrono::{DateTime, FixedOffset},
};
use std::collections::HashMap;
use tauri::State;

use crate::db::Validator;
use crate::error::{Error, Result};
use crate::state::AppState;
use crate::utils;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Payment {
    pub pay_id: Option<String>,
    pub pay_number: Option<String>,
    pub order_type: Option<String>,
    pub total_amount: Option<f64>,
    pub payment_amount: Option<f64>,
    pub payment_amount_vip: Option<f64>,
    pub payment_amount_mv: Option<f64>,
    pub payment_status: Option<String>,
    pub payment_method: Option<String>,
    pub transaction_id: Option<i64>,
    pub uc_order_id: Option<i64>,
    pub uc_id: Option<String>,
    pub order_status: Option<String>,
    pub create_time: Option<i64>,
    pub update_time: Option<i64>,
    pub store_id: Option<i64>, // 商家ID，用于数据隔离
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

        if self.order_type.is_none() {
            return Err(Error::bad_request("order_type is required"));
        }

        if self.total_amount.is_none() {
            return Err(Error::bad_request("total_amount is required"));
        }

        if self.payment_amount.is_none() {
            return Err(Error::bad_request("payment_amount is required"));
        }

        if self.payment_status.is_none() {
            return Err(Error::bad_request("payment_status is required"));
        }

        if self.payment_method.is_none() {
            return Err(Error::bad_request("payment_method is required"));
        }

        if self.order_status.is_none() {
            return Err(Error::bad_request("order_status is required"));
        }

        Ok(())
    }
}

impl FromRow<'_, SqliteRow> for Payment {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            pay_id: row.try_get("pay_id").unwrap_or_default(),
            pay_number: row.try_get("pay_number").unwrap_or_default(),
            order_type: row.try_get("order_type").unwrap_or_default(),
            total_amount: row.try_get("total_amount").unwrap_or_default(),
            payment_amount: row.try_get("payment_amount").unwrap_or_default(),
            payment_amount_vip: row.try_get("payment_amount_vip").unwrap_or_default(),
            payment_amount_mv: row.try_get("payment_amount_mv").unwrap_or_default(),
            payment_status: row.try_get("payment_status").unwrap_or_default(),
            payment_method: row.try_get("payment_method").unwrap_or_default(),
            transaction_id: row.try_get("transaction_id").unwrap_or_default(),
            uc_order_id: row.try_get("uc_order_id").unwrap_or_default(),
            uc_id: row.try_get("uc_id").unwrap_or_default(),
            order_status: row.try_get("order_status").unwrap_or_default(),
            create_time: row.try_get("create_time").unwrap_or_default(),
            update_time: row.try_get("update_time").unwrap_or_default(),
            store_id: row.try_get("store_id").unwrap_or_default(),
        })
    }
}

impl Payment {
    pub async fn create_payment(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<Payment> {
        let query = r#"
        INSERT INTO payments (pay_id, pay_number, order_type, total_amount, payment_amount, payment_amount_vip,
                             payment_amount_mv, payment_status, payment_method, transaction_id,
                             uc_order_id, uc_id, create_time, order_status, store_id)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *
    "#;

        let result = sqlx::query_as(query)
            .bind(&self.pay_id)
            .bind(&self.pay_number)
            .bind(&self.order_type)
            .bind(&self.total_amount)
            .bind(&self.payment_amount)
            .bind(&self.payment_amount_vip)
            .bind(&self.payment_amount_mv)
            .bind(&self.payment_status)
            .bind(&self.payment_method)
            .bind(&self.transaction_id)
            .bind(&self.uc_order_id)
            .bind(&self.uc_id)
            .bind(utils::get_timestamp())
            .bind(&self.order_status)
            .bind(&self.store_id)
            .fetch_one(&mut **tr)
            .await?;

        Ok(result)
    }

    pub async fn get_by_order_id(
        pool: &Pool<Sqlite>,
        order_id: i64,
        store_id: i64,
    ) -> Result<Option<Self>> {
        let payment =
            sqlx::query_as("SELECT * FROM payments WHERE uc_order_id = ? AND store_id = ?")
                .bind(order_id)
                .bind(store_id)
                .fetch_optional(pool)
                .await?;
        Ok(payment)
    }

    pub async fn cal_total_amount(pool: &Pool<Sqlite>, user_id: i64, store_id: i64) -> Result<f64> {
        let result = sqlx::query_scalar(
            "
            SELECT COALESCE(SUM(payment_amount), 0.0)
            FROM payments p
            INNER JOIN orders o ON p.uc_order_id = o.order_id
            WHERE o.user_id =? AND p.store_id = ?
            ",
        )
        .bind(user_id)
        .bind(store_id)
        .fetch_optional(pool)
        .await?
        .unwrap_or(0.0);

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

    // 查询收入数据
    let income_data: HashMap<String, f64> = sqlx::query(
        r#"
        SELECT
            strftime('%Y-%m-%d', datetime(create_time/1000, 'unixepoch')) as date,
            SUM(payment_amount) as income
        FROM payments
        WHERE create_time BETWEEN ? AND ?
          AND payment_status = '01'
          AND payment_amount > 0
          AND store_id =?
        GROUP BY date
        ORDER BY date
        "#,
    )
    .bind(start_timestamp)
    .bind(end_timestamp)
    .bind(store_id)
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
        AND store_id =?
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

    // 合并数据
    let mut summaries = Vec::new();
    for date in all_dates {
        let income = *income_data.get(&date).unwrap_or(&0.0);
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
    // 收入查询（带可选支付状态条件）
    let income = sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(payment_amount), 0.0)
        FROM payments
        WHERE create_time BETWEEN ? AND ?
          AND payment_status = '01'
          AND payment_amount > 0
          AND store_id =?
        "#,
    )
    .bind(start)
    .bind(end)
    .bind(store_id)
    .fetch_one(pool)
    .await?;

    // 支出查询（公共部分）
    let expense: i64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(exp_amount), 0)
        FROM expenditure
        WHERE create_time BETWEEN ? AND ?
        AND store_id =?
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
