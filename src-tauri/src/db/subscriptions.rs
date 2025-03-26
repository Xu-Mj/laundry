use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Pool, Row, Sqlite, Transaction};
use tauri::State;

use crate::db::Validator;
use crate::db::subscription_plan::SubscriptionPlan;
use crate::error::{Error, ErrorKind, Result};
use crate::state::AppState;
use crate::utils;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Subscription {
    pub id: i64,
    pub store_id: i64,
    pub plan_id: i64,
    pub start_date: Option<i64>,
    pub expiry_date: Option<i64>,
    pub status: Option<String>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
    pub remark: Option<String>,
    pub plan: Option<SubscriptionPlan>,
}

impl FromRow<'_, SqliteRow> for Subscription {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        let plan = SubscriptionPlan::from_row(row)?;
        Ok(Self {
            id: row.try_get("id").unwrap_or_default(),
            store_id: row.try_get("store_id").unwrap_or_default(),
            plan_id: row.try_get("plan_id").unwrap_or_default(),
            start_date: row.try_get("start_date").unwrap_or_default(),
            expiry_date: row.try_get("expiry_date").unwrap_or_default(),
            status: row.try_get("status").unwrap_or_default(),
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
            plan: Some(plan),
        })
    }
}

impl Validator for Subscription {
    fn validate(&self) -> Result<()> {
        if self.expiry_date.is_none() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "expiry_date cannot be empty",
            ));
        }

        Ok(())
    }
}

impl Subscription {
    // 创建新订阅
    pub async fn create(&self, pool: &mut Transaction<'_, Sqlite>) -> Result<Self> {
        let now = utils::get_timestamp();
        let subscription = sqlx::query_as(
            r#"
            INSERT INTO subscriptions (
                store_id, plan_id, start_date, expiry_date, status, created_at, updated_at, remark
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(self.store_id)
        .bind(&self.plan_id)
        .bind(self.start_date.unwrap_or(now))
        .bind(self.expiry_date)
        .bind(&self.status)
        .bind(now)
        .bind(now)
        .bind(&self.remark)
        .fetch_one(&mut **pool)
        .await?;

        Ok(subscription)
    }
    pub async fn upsert(&self, pool: &mut Transaction<'_, Sqlite>) -> Result<bool> {
        let now = utils::get_timestamp();
        let result = sqlx::query(
            r#"
            INSERT INTO subscriptions (
                id,
                store_id,
                plan_id,
                start_date,
                expiry_date,
                status,
                created_at,
                updated_at,
                remark
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                store_id = excluded.store_id,
                plan_id = excluded.plan_id,
                start_date = excluded.start_date,
                expiry_date = excluded.expiry_date,
                status = excluded.status,
                updated_at = excluded.updated_at,
                remark = excluded.remark
            "#,
        )
        .bind(&self.id)
        .bind(&self.store_id)
        .bind(&self.plan_id)
        .bind(self.start_date)
        .bind(self.expiry_date)
        .bind(&self.status)
        .bind(now) // 创建时间（首次插入时设置）
        .bind(now) // 更新时间（总是设置为当前时间）
        .bind(&self.remark)
        .execute(&mut **pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
    // 更新订阅信息
    pub async fn update(&self, pool: &Pool<Sqlite>) -> Result<bool> {
        let now = utils::get_timestamp();
        let result = sqlx::query(
            r#"
            UPDATE subscriptions
            SET plan_id = ?,
                start_date = ?,
                expiry_date = ?,
                status = ?,
                updated_at = ?,
                remark = ?
            WHERE id = ?
            "#,
        )
        .bind(&self.plan_id)
        .bind(self.start_date)
        .bind(self.expiry_date)
        .bind(&self.status)
        .bind(now)
        .bind(&self.remark)
        .bind(self.id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // 根据用户ID获取有效的订阅
    pub async fn get_active_by_user_id(pool: &Pool<Sqlite>, user_id: i64) -> Result<Option<Self>> {
        let now = utils::get_timestamp();
        let subscription = sqlx::query_as(
            r#"
            SELECT sup.*, sp.* FROM subscriptions sup
            JOIN subscription_plans sp ON sup.plan_id = sp.id
            WHERE sup.store_id = ? AND sup.expiry_date > ? AND sup.status = 'Active'
            ORDER BY sup.expiry_date DESC LIMIT 1
            "#,
        )
        .bind(user_id)
        .bind(now)
        .fetch_optional(pool)
        .await?;

        Ok(subscription)
    }

    // 获取用户的所有订阅
    pub async fn get_all_by_user_id(pool: &Pool<Sqlite>, user_id: i64) -> Result<Vec<Self>> {
        let subscriptions = sqlx::query_as(
            r#"
            SELECT * FROM subscriptions 
            WHERE store_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(subscriptions)
    }

    // 获取用户的所有有效订阅
    pub async fn get_all_active_by_user_id(pool: &Pool<Sqlite>, user_id: i64) -> Result<Vec<Self>> {
        let now = utils::get_timestamp();
        let subscriptions = sqlx::query_as(
            r#"
            SELECT sup.*, sp.* FROM subscriptions sup
            JOIN subscription_plans sp ON sup.plan_id = sp.id
            WHERE sup.store_id = ? AND sup.expiry_date > ? AND sup.status = 'Active'
            ORDER BY sup.expiry_date DESC
            "#,
        )
        .bind(user_id)
        .bind(now)
        .fetch_all(pool)
        .await?;

        Ok(subscriptions)
    }

    // 根据ID获取订阅
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: i64) -> Result<Option<Self>> {
        let subscription = sqlx::query_as("SELECT * FROM subscriptions WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(subscription)
    }

    // 检查用户是否有有效订阅
    pub async fn has_active_subscription(pool: &Pool<Sqlite>, user_id: i64) -> Result<bool> {
        let now = utils::get_timestamp();
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM subscriptions 
            WHERE store_id = ? AND expiry_date > ? AND status = 'active'
            "#,
        )
        .bind(user_id)
        .bind(now)
        .fetch_one(pool)
        .await?;

        Ok(count > 0)
    }
}

// service
impl Subscription {
    // create
    pub async fn create_subscription(
        pool: &Pool<Sqlite>,
        subscription: Subscription,
        plan: SubscriptionPlan,
    ) -> Result<()> {
        let mut tx = pool.begin().await?;
        subscription.create(&mut tx).await?;
        plan.create(&mut tx).await?;
        tx.commit().await?;
        Ok(())
    }
}

#[tauri::command]
pub async fn get_user_subscriptions(state: State<'_, AppState>) -> Result<Vec<Subscription>> {
    let user = state.get_user_info().await;
    if let Some(user) = user {
        if let Some(id) = user.id {
            return Subscription::get_all_active_by_user_id(&state.pool, id).await;
        }
    }
    Ok(vec![])
}

#[tauri::command]
pub async fn create_subscription(
    state: State<'_, AppState>,
    subscription: Subscription,
    plan: SubscriptionPlan,
) -> Result<()> {
    subscription.validate()?;
    Subscription::create_subscription(&state.pool, subscription, plan).await
}

#[tauri::command]
pub async fn update_subscription(
    state: State<'_, AppState>,
    subscription: Subscription,
) -> Result<bool> {
    subscription.validate()?;
    subscription.update(&state.pool).await
}

#[tauri::command]
pub async fn get_active_by_user_id(state: State<'_, AppState>) -> Result<Option<Subscription>> {
    let user = state.get_user_info().await;
    if let Some(user) = user {
        if let Some(id) = user.id {
            return Subscription::get_active_by_user_id(&state.pool, id).await;
        }
    }
    Ok(None)
}
