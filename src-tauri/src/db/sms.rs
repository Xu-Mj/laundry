use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::sqlite::SqliteRow;
use sqlx::types::Json;
use sqlx::{QueryBuilder, Sqlite, Transaction};
use sqlx::{Row, prelude::FromRow};

use super::Curd;
use crate::error::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmsInvocation {
    pub id: i64,
    pub store_id: i64,
    pub phone: String,
    pub temp_id: i64,
    pub invoke_time: i64,
    pub complete_time: i64,
    pub args: Json<Value>,
    pub result: Option<String>,
}

impl FromRow<'_, SqliteRow> for SmsInvocation {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id").unwrap_or_default(),
            store_id: row.try_get("store_id").unwrap_or_default(),
            phone: row.try_get("phone").unwrap_or_default(),
            temp_id: row.try_get("temp_id").unwrap_or_default(),
            invoke_time: row.try_get("invoke_time").unwrap_or_default(),
            complete_time: row.try_get("complete_time").unwrap_or_default(),
            result: row.try_get("result").unwrap_or_default(),
            args: row.try_get("args").unwrap_or_default(),
        })
    }
}

impl SmsInvocation {
    pub async fn save(&self, pool: &mut Transaction<'_, Sqlite>) -> Result<()> {
        sqlx::query(
            "INSERT INTO sms_invocations (store_id,phone, temp_id, invoke_time, complete_time, args, result) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(self.store_id)
        .bind(&self.phone)
        .bind(self.temp_id)
        .bind(self.invoke_time)
        .bind(self.complete_time)
        .bind(&self.args)
        .bind(&self.result)
        .execute(&mut **pool)
        .await?;
        Ok(())
    }
}

impl Curd for SmsInvocation {
    const COUNT_SQL: &'static str = "SELECT COUNT(*) FROM sms_invocations";
    const QUERY_SQL: &'static str = "SELECT * FROM sms_invocations";
    const BY_ID_SQL: &'static str = "SELECT * FROM sms_invocations WHERE id = $1";
    const DELETE_BATCH_SQL: &'static str = "DELETE FROM sms_invocations WHERE id IN (";

    fn apply_filters<'a>(&'a self, _builder: &mut QueryBuilder<'a, sqlx::Sqlite>) {
        // 根据需要添加过滤条件
        // 例如：
        // if let Some(store_id) = self.store_id {
        //     builder.push(" AND store_id = ").push_bind(store_id);
        // }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmsInvocationResp {
    pub sms: SmsInvocation,
    pub message: Option<SmsInvocationMsg>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SmsInvocationMsgType {
    Info,
    Warn,
    Error,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SmsInvocationMsg {
    pub msg_type: SmsInvocationMsgType,
    pub msg: String,
}
