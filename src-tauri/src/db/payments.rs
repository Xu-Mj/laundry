use crate::db::Validator;
use crate::error::{Error, ErrorKind, Result};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{types::chrono::{DateTime, FixedOffset}, FromRow, Pool, Row, Sqlite, Transaction};
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

    pub async fn get_by_order_id(
        pool: &Pool<Sqlite>,
        order_id: i64,
    ) -> Result<Option<Self>> {
        let payment = sqlx::query_as("SELECT * FROM payments WHERE uc_order_id = ?")
            .bind(order_id)
            .fetch_optional(pool)
            .await?;
        Ok(payment)
    }
}
