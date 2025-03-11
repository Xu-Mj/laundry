use chrono::{Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{
    FromRow, Pool, Row, Sqlite, Transaction,
    types::chrono::{DateTime, FixedOffset},
};
use tauri::State;

use crate::db::Validator;
use crate::error::{Error, ErrorKind, Result};
use crate::state::AppState;
use crate::utils;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Payment {
    pub pay_id: Option<i64>,
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
    pub create_time: Option<DateTime<FixedOffset>>,
    pub update_time: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PaymentSummary {
    pub date: String,
    pub total_amount: f64,
}

impl Validator for Payment {
    fn validate(&self) -> Result<()> {
        if self.pay_number.is_none() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "pay_number is required",
            ));
        }

        if self.order_type.is_none() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "order_type is required",
            ));
        }

        if self.total_amount.is_none() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "total_amount is required",
            ));
        }

        if self.payment_amount.is_none() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "payment_amount is required",
            ));
        }

        if self.payment_status.is_none() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "payment_status is required",
            ));
        }

        if self.payment_method.is_none() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "payment_method is required",
            ));
        }

        if self.order_status.is_none() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "order_status is required",
            ));
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
        })
    }
}

impl Payment {
    pub async fn create_payment(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<Payment> {
        let query = r#"
        INSERT INTO payments (pay_number, order_type, total_amount, payment_amount, payment_amount_vip,
                             payment_amount_mv, payment_status, payment_method, transaction_id,
                             uc_order_id, uc_id, create_time, order_status)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *
    "#;

        let result = sqlx::query_as(query)
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
            .bind(utils::get_now())
            .bind(&self.order_status)
            .fetch_one(&mut **tr)
            .await?;

        Ok(result)
    }

    pub async fn get_by_order_id(pool: &Pool<Sqlite>, order_id: i64) -> Result<Option<Self>> {
        let payment = sqlx::query_as("SELECT * FROM payments WHERE uc_order_id = ?")
            .bind(order_id)
            .fetch_optional(pool)
            .await?;
        Ok(payment)
    }

    pub async fn cal_total_amount(pool: &Pool<Sqlite>, user_id: i64) -> Result<f64> {
        let result = sqlx::query_scalar(
            "
            SELECT SUM(payment_amount)
            FROM payments p
            INNER JOIN orders o ON p.uc_order_id = o.order_id
            WHERE o.user_id =?
            ",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

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
    year: Option<i32>,
    month: Option<u32>,
) -> Result<Vec<PaymentSummary>> {
    let now = Utc::now().naive_utc();
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

    let summaries = sqlx::query_as(
        r#"
        SELECT
            strftime('%Y-%m-%d', create_time) as date,
            SUM(payment_amount) as total_amount
        FROM payments
        WHERE create_time BETWEEN ? AND ?
        GROUP BY date
        ORDER BY date
        "#,
    )
    .bind(start_of_month)
    .bind(end_of_month)
    .fetch_all(pool)
    .await?;

    Ok(summaries)
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BarChartData {
    pub label: String,
    pub income: f64,
    pub expense: f64,
}

pub async fn get_daily_payment_summary(pool: &Pool<Sqlite>) -> Result<BarChartData> {
    let now = Utc::now().naive_utc();
    let start_of_day = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
        .ok_or(Error::bad_request("Invalid date"))?
        .and_hms_opt(0, 0, 0)
        .ok_or(Error::bad_request("Invalid time"))?;
    let end_of_day = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
        .ok_or(Error::bad_request("Invalid date"))?
        .and_hms_opt(23, 59, 59)
        .ok_or(Error::bad_request("Invalid time"))?;

    let income = sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(payment_amount), 0.0)
        FROM payments
        WHERE create_time BETWEEN ? AND ? AND payment_amount > 0
        "#,
    )
    .bind(start_of_day)
    .bind(end_of_day)
    .fetch_one(pool)
    .await?;

    let expense: i64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(exp_amount), 0)
        FROM expenditure
        WHERE create_time BETWEEN ? AND ?
        "#,
    )
    .bind(start_of_day)
    .bind(end_of_day)
    .fetch_one(pool)
    .await?;

    Ok(BarChartData {
        label: "今日".to_string(),
        income,
        expense: -expense as f64,
    })
}

pub async fn get_weekly_payment_summary(pool: &Pool<Sqlite>) -> Result<BarChartData> {
    let now = Utc::now().naive_utc();
    let start_of_week = now
        .date()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .checked_sub_signed(chrono::Duration::days(
            now.weekday().num_days_from_monday() as i64
        ))
        .ok_or(Error::bad_request("Invalid date"))?;
    let end_of_week = start_of_week
        .checked_add_signed(chrono::Duration::days(6))
        .ok_or(Error::bad_request("Invalid date"))?
        .date()
        .and_hms_opt(23, 59, 59)
        .unwrap();
    let income = sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(payment_amount), 0.0)
        FROM payments
        WHERE create_time BETWEEN ? AND ? AND payment_amount > 0
        "#,
    )
    .bind(start_of_week)
    .bind(end_of_week)
    .fetch_one(pool)
    .await?;

    let expense: i64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(exp_amount), 0)
        FROM expenditure
        WHERE create_time BETWEEN ? AND ?
        "#,
    )
    .bind(start_of_week)
    .bind(end_of_week)
    .fetch_one(pool)
    .await?;

    Ok(BarChartData {
        label: "本周".to_string(),
        income,
        expense: -expense as f64,
    })
}

#[tauri::command]
pub async fn get_total_amount(state: State<'_, AppState>, user_id: i64) -> Result<f64> {
    Payment::cal_total_amount(&state.pool, user_id).await
}
