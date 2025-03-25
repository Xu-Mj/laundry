use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::types::chrono::{DateTime, FixedOffset};
use sqlx::{FromRow, Pool, Row, Sqlite, Transaction};
use tauri::State;

use crate::db::Validator;
use crate::error::{Error, ErrorKind, Result};
use crate::state::AppState;
use crate::utils;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct QrcodePayment {
    pub id: Option<i64>,
    pub pay_id: Option<i64>,
    pub store_id: Option<i64>,
    pub payment_type: Option<String>,
    pub auth_code: Option<String>,
    pub qr_code: Option<String>,
    pub out_trade_no: Option<String>,
    pub trade_no: Option<String>,
    pub total_amount: Option<f64>,
    pub subject: Option<String>,
    pub trade_status: Option<String>,
    pub buyer_id: Option<String>,
    pub buyer_logon_id: Option<String>,
    pub receipt_amount: Option<f64>,
    pub point_amount: Option<f64>,
    pub invoice_amount: Option<f64>,
    pub fund_bill_list: Option<String>,
    pub voucher_detail_list: Option<String>,
    pub gmt_payment: Option<DateTime<FixedOffset>>,
    pub raw_response: Option<String>,
    pub create_time: Option<DateTime<FixedOffset>>,
    pub update_time: Option<DateTime<FixedOffset>>,
}

impl Validator for QrcodePayment {
    fn validate(&self) -> Result<()> {
        if self.pay_id.is_none() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "pay_id is required",
            ));
        }

        if self.store_id.is_none() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "store_id is required",
            ));
        }

        if self.payment_type.is_none() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "payment_type is required",
            ));
        }

        if self.total_amount.is_none() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "total_amount is required",
            ));
        }

        Ok(())
    }
}

impl FromRow<'_, SqliteRow> for QrcodePayment {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id").unwrap_or_default(),
            pay_id: row.try_get("pay_id").unwrap_or_default(),
            store_id: row.try_get("store_id").unwrap_or_default(),
            payment_type: row.try_get("payment_type").unwrap_or_default(),
            auth_code: row.try_get("auth_code").unwrap_or_default(),
            qr_code: row.try_get("qr_code").unwrap_or_default(),
            out_trade_no: row.try_get("out_trade_no").unwrap_or_default(),
            trade_no: row.try_get("trade_no").unwrap_or_default(),
            total_amount: row.try_get("total_amount").unwrap_or_default(),
            subject: row.try_get("subject").unwrap_or_default(),
            trade_status: row.try_get("trade_status").unwrap_or_default(),
            buyer_id: row.try_get("buyer_id").unwrap_or_default(),
            buyer_logon_id: row.try_get("buyer_logon_id").unwrap_or_default(),
            receipt_amount: row.try_get("receipt_amount").unwrap_or_default(),
            point_amount: row.try_get("point_amount").unwrap_or_default(),
            invoice_amount: row.try_get("invoice_amount").unwrap_or_default(),
            fund_bill_list: row.try_get("fund_bill_list").unwrap_or_default(),
            voucher_detail_list: row.try_get("voucher_detail_list").unwrap_or_default(),
            gmt_payment: row.try_get("gmt_payment").unwrap_or_default(),
            raw_response: row.try_get("raw_response").unwrap_or_default(),
            create_time: row.try_get("create_time").unwrap_or_default(),
            update_time: row.try_get("update_time").unwrap_or_default(),
        })
    }
}

impl QrcodePayment {
    pub async fn create(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<Self> {
        let query = r#"
        INSERT INTO qrcode_payments (
            pay_id, store_id, payment_type, auth_code, qr_code, out_trade_no, trade_no,
            total_amount, subject, trade_status, buyer_id, buyer_logon_id, receipt_amount,
            point_amount, invoice_amount, fund_bill_list, voucher_detail_list, gmt_payment,
            raw_response, create_time
        ) VALUES (
            ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
        ) RETURNING *
        "#;

        let result = sqlx::query_as(query)
            .bind(&self.pay_id)
            .bind(&self.store_id)
            .bind(&self.payment_type)
            .bind(&self.auth_code)
            .bind(&self.qr_code)
            .bind(&self.out_trade_no)
            .bind(&self.trade_no)
            .bind(&self.total_amount)
            .bind(&self.subject)
            .bind(&self.trade_status)
            .bind(&self.buyer_id)
            .bind(&self.buyer_logon_id)
            .bind(&self.receipt_amount)
            .bind(&self.point_amount)
            .bind(&self.invoice_amount)
            .bind(&self.fund_bill_list)
            .bind(&self.voucher_detail_list)
            .bind(&self.gmt_payment)
            .bind(&self.raw_response)
            .bind(utils::get_now())
            .fetch_one(&mut **tr)
            .await?;

        Ok(result)
    }

    pub async fn get_by_pay_id(pool: &Pool<Sqlite>, pay_id: i64) -> Result<Option<Self>> {
        let result = sqlx::query_as("SELECT * FROM qrcode_payments WHERE pay_id = ?")
            .bind(pay_id)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    pub async fn get_by_trade_no(pool: &Pool<Sqlite>, trade_no: &str) -> Result<Option<Self>> {
        let result = sqlx::query_as("SELECT * FROM qrcode_payments WHERE trade_no = ?")
            .bind(trade_no)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    pub async fn get_by_out_trade_no(
        pool: &Pool<Sqlite>,
        out_trade_no: &str,
    ) -> Result<Option<Self>> {
        let result = sqlx::query_as("SELECT * FROM qrcode_payments WHERE out_trade_no = ?")
            .bind(out_trade_no)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }
}

#[tauri::command]
pub async fn get_qrcode_payment_by_pay_id(
    state: State<'_, AppState>,
    pay_id: i64,
) -> Result<Option<QrcodePayment>> {
    QrcodePayment::get_by_pay_id(&state.pool, pay_id).await
}

#[tauri::command]
pub async fn get_qrcode_payment_by_trade_no(
    state: State<'_, AppState>,
    trade_no: String,
) -> Result<Option<QrcodePayment>> {
    QrcodePayment::get_by_trade_no(&state.pool, &trade_no).await
}

#[tauri::command]
pub async fn get_qrcode_payment_by_out_trade_no(
    state: State<'_, AppState>,
    out_trade_no: String,
) -> Result<Option<QrcodePayment>> {
    QrcodePayment::get_by_out_trade_no(&state.pool, &out_trade_no).await
}
