use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, QueryBuilder, Row, Sqlite};
use tauri::{State, command};

use super::{Curd, PageParams, PageResult};
use crate::error::Result;
use crate::state::AppState;
use crate::utils;

/// WebSocket消息类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    /// 普通消息
    Normal,
    /// 错误消息
    Error,
    /// 系统通知
    System,
    /// 客户端已收到消息
    Received,
    /// 客户端已读消息
    Read,
    /// 订单状态更新
    OrderUpdate,
    /// 订单支付消息
    OrderPayment,
    /// 支付状态更新
    PaymentUpdate,
    /// 配送状态更新
    DeliveryUpdate,
    NewUserRegister,
}

impl MessageType {
    /// 将消息类型枚举转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Error => "error",
            Self::System => "system",
            Self::Received => "received",
            Self::Read => "read",
            Self::OrderUpdate => "order_update",
            Self::OrderPayment => "order_payment",
            Self::PaymentUpdate => "payment_update",
            Self::DeliveryUpdate => "delivery_update",
            Self::NewUserRegister => "new_user_register",
        }
    }
}

impl Default for MessageType {
    fn default() -> Self {
        Self::Normal
    }
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct WsMessage {
    pub id: Option<i64>,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub message_type: Option<String>,
    pub content: String,
    pub read: Option<bool>,
    pub sent: Option<bool>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl FromRow<'_, sqlx::sqlite::SqliteRow> for WsMessage {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(WsMessage {
            id: row.try_get("id")?,
            sender_id: row.try_get("sender_id")?,
            receiver_id: row.try_get("receiver_id")?,
            message_type: row.try_get("message_type")?,
            content: row.try_get("content")?,
            read: row.try_get("read")?,
            sent: row.try_get("sent")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

impl Curd for WsMessage {
    const COUNT_SQL: &'static str = "SELECT COUNT(*) FROM messages WHERE 1=1";
    const QUERY_SQL: &'static str = "SELECT * FROM messages WHERE 1=1";
    const BY_ID_SQL: &'static str = "SELECT * FROM messages WHERE id = ?";
    const DELETE_BATCH_SQL: &'static str = "DELETE FROM messages WHERE id IN (";
    const ORDER_SQL: Option<&'static str> = Some(" ORDER BY created_at DESC ");

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, Sqlite>) {
        if let Some(t) = &self.message_type {
            builder.push(" AND message_type = ");
            builder.push_bind(t);
        }

        if let Some(read) = self.read {
            builder.push(" AND read = ");
            builder.push_bind(read);
        }

        if let Some(send) = self.sent {
            builder.push(" AND sent = ");
            builder.push_bind(send);
        }
    }
}

impl WsMessage {
    pub async fn create(&self, pool: &Pool<Sqlite>) -> Result<Self> {
        let result = sqlx::query_as(
            "INSERT INTO messages (sender_id, receiver_id, message_type, content, read, sent, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?) RETURNING *"
        )
        .bind(self.sender_id)
        .bind(self.receiver_id)
        .bind(&self.message_type)
        .bind(&self.content)
        .bind(self.read)
        .bind(self.sent)
        .bind(self.created_at)
        .bind(self.updated_at)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn mark_as_read(pool: &Pool<Sqlite>, ids: Vec<i64>) -> Result<u64> {
        if ids.is_empty() {
            return Ok(0);
        }

        let now = chrono::Utc::now().timestamp();
        let mut builder = QueryBuilder::new("UPDATE messages SET read = true, updated_at = ");
        builder.push_bind(now);
        builder.push(" WHERE id IN (");

        ids.iter().enumerate().for_each(|(i, id)| {
            if i > 0 {
                builder.push(", ");
            }
            builder.push_bind(id);
        });

        builder.push(")");

        let result = builder.build().execute(pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn count_unread(pool: &Pool<Sqlite>, receiver_id: i64) -> Result<i64> {
        let count = sqlx::query_scalar(
            "SELECT COUNT(*) FROM messages WHERE receiver_id = ? AND read = false",
        )
        .bind(receiver_id)
        .fetch_one(pool)
        .await?;

        Ok(count)
    }

    pub async fn clear_msgs_by_type(
        pool: &Pool<Sqlite>,
        receiver_id: i64,
        message_type: Option<MessageType>,
    ) -> Result<bool> {
        let mut builder = QueryBuilder::new("DELETE FROM messages WHERE receiver_id = ");
        builder.push_bind(receiver_id);

        if let Some(t) = message_type {
            builder.push(" AND message_type = ");
            builder.push_bind(t.as_str());
        }

        let result = builder.build().execute(pool).await?;
        Ok(result.rows_affected() > 0)
    }
}

#[command]
pub async fn save_message(state: State<'_, AppState>, msg: WsMessage) -> Result<WsMessage> {
    msg.create(&state.pool).await
}

#[command]
pub async fn get_messages(
    state: State<'_, AppState>,
    params: PageParams,
    msg: WsMessage,
) -> Result<PageResult<WsMessage>> {
    msg.get_list(&state.pool, params).await
}

#[command]
pub async fn delete_message(state: State<'_, AppState>, ids: Vec<i64>) -> Result<bool> {
    let mut tx = state.pool.begin().await?;
    let res = WsMessage::delete_batch(&mut tx, &ids).await?;
    tx.commit().await?;
    Ok(res)
}

#[command]
pub async fn clear_messages(state: State<'_, AppState>, t: Option<MessageType>) -> Result<bool> {
    let store_id = utils::get_user_id(&state).await?;
    let res = WsMessage::clear_msgs_by_type(&state.pool, store_id, t).await?;
    Ok(res)
}

#[command]
pub async fn get_unread_message_count(state: State<'_, AppState>) -> Result<i64> {
    let store_id = utils::get_user_id(&state).await?;
    WsMessage::count_unread(&state.pool, store_id).await
}

#[command]
pub async fn mark_messages_as_read(state: State<'_, AppState>, ids: Vec<i64>) -> Result<u64> {
    WsMessage::mark_as_read(&state.pool, ids).await
}
