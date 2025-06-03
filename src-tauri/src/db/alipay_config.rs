use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, QueryBuilder, Row, Sqlite, sqlite::SqliteRow};
use tauri::State;

use crate::{
    error::{Error, Result},
    state::AppState,
    utils,
};

use super::{Curd, Validator};

/// 商家支付宝配置
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct AlipayConfig {
    /// 配置ID
    pub id: Option<i64>,

    /// 商家ID
    pub store_id: i64,

    /// 支付宝应用ID
    pub app_id: String,

    /// 应用私钥
    pub private_key: String,

    /// 支付宝公钥
    pub alipay_public_key: String,

    /// 卖家支付宝用户ID
    pub seller_id: Option<String>,

    /// 是否激活
    pub is_active: bool,

    /// 是否沙箱环境
    pub is_sandbox: bool,

    /// 创建时间
    pub created_at: i64,

    /// 更新时间
    pub updated_at: i64,
}

impl Validator for AlipayConfig {
    fn validate(&self) -> Result<()> {
        if self.store_id == 0 {
            return Err(Error::bad_request("商家id不能为空"));
        }

        if self.app_id.is_empty() {
            return Err(Error::bad_request("支付宝appid不能为空"));
        }

        if self.private_key.is_empty() {
            return Err(Error::bad_request("支付宝私钥不能为空"));
        }

        if self.alipay_public_key.is_empty() {
            return Err(Error::bad_request("支付宝公钥不能为空"));
        }

        Ok(())
    }
}

impl FromRow<'_, SqliteRow> for AlipayConfig {
    fn from_row(row: &SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id").unwrap_or_default(),
            store_id: row.try_get("store_id").unwrap_or_default(),
            app_id: row.try_get("app_id").unwrap_or_default(),
            private_key: row.try_get("private_key").unwrap_or_default(),
            alipay_public_key: row.try_get("alipay_public_key").unwrap_or_default(),
            seller_id: row.try_get("seller_id").unwrap_or_default(),
            is_active: row.try_get("is_active").unwrap_or_default(),
            is_sandbox: row.try_get("is_sandbox").unwrap_or_default(),
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
        })
    }
}

impl Curd for AlipayConfig {
    const COUNT_SQL: &'static str = "SELECT COUNT(*) FROM alipay_configs";

    const QUERY_SQL: &'static str = "SELECT * FROM alipay_configs";

    const BY_ID_SQL: &'static str = "SELECT * FROM alipay_configs WHERE id = $1";

    const DELETE_BATCH_SQL: &'static str = "DELETE FROM alipay_configs WHERE id = ANY($1)";

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, Sqlite>) {
        if let Some(id) = self.id {
            builder.push(" AND id = ");
            builder.push_bind(id);
        }

        builder.push(" AND store_id = ");
        builder.push_bind(self.store_id);

        if !self.app_id.is_empty() {
            builder.push(" AND app_id = ");
            builder.push_bind(&self.app_id);
        }

        builder.push(" ORDER BY created_at DESC");
    }
}

impl AlipayConfig {
    pub async fn upsert(&self, pool: &Pool<Sqlite>) -> Result<Self> {
        let now = utils::get_timestamp();

        let result = sqlx::query_as(
            r#"
            INSERT INTO alipay_configs (
                id, app_id, private_key, alipay_public_key,
                seller_id, is_active, is_sandbox, created_at, updated_at, store_id
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT(id) DO UPDATE SET
                app_id = excluded.app_id,
                private_key = excluded.private_key,
                alipay_public_key = excluded.alipay_public_key,
                seller_id = excluded.seller_id,
                is_active = excluded.is_active,
                is_sandbox = excluded.is_sandbox,
                updated_at = excluded.updated_at,
                store_id = excluded.store_id
            RETURNING *
            "#,
        )
        .bind(&self.id) // $1
        .bind(&self.app_id) // $2
        .bind(&self.private_key) // $3
        .bind(&self.alipay_public_key) // $5
        .bind(&self.seller_id) // $7
        .bind(self.is_active) // $8
        .bind(self.is_sandbox) // $9
        .bind(now) // $10 created_at (仅插入时设置)
        .bind(now) // $11 updated_at (总是更新)
        .bind(self.store_id) // $11 updated_at (总是更新)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    /// 获取商家的支付宝配置
    pub async fn get_by_store_id(pool: &Pool<Sqlite>, store_id: i64) -> Result<Option<Self>> {
        let config = sqlx::query_as(
            "
            SELECT * FROM alipay_configs
            WHERE store_id = $1 AND is_active = true
            LIMIT 1
            ",
        )
        .bind(store_id)
        .fetch_optional(pool)
        .await?;

        Ok(config)
    }

    /// 停用商家的支付宝配置
    pub async fn deactivate(pool: &Pool<Sqlite>, store_id: i64) -> Result<Self> {
        let result = sqlx::query_as(
            "
            UPDATE alipay_configs
            SET is_active = false, updated_at = $1
            WHERE id = $2
            RETURNING *
            ",
        )
        .bind(utils::get_timestamp())
        .bind(store_id)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }
}

#[tauri::command]
pub async fn save_alipay_config(
    state: State<'_, AppState>,
    config: AlipayConfig,
) -> Result<AlipayConfig> {
    config.validate()?;
    config.upsert(&state.pool).await
}

#[tauri::command]
pub async fn get_alipay_config(
    state: State<'_, AppState>,
    store_id: i64,
) -> Result<Option<AlipayConfig>> {
    if store_id == 0 {
        return Err(Error::bad_request("商家ID不能为空"));
    }
    AlipayConfig::get_by_store_id(&state.pool, store_id).await
}

#[tauri::command]
pub async fn deactivate_alipay(state: State<'_, AppState>, store_id: i64) -> Result<AlipayConfig> {
    if store_id == 0 {
        return Err(Error::bad_request("商家ID不能为空"));
    }
    AlipayConfig::deactivate(&state.pool, store_id).await
}
