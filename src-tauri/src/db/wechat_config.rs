use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Row, Sqlite, sqlite::SqliteRow};
use tauri::State;

use crate::{
    error::{Error, Result},
    state::AppState,
    utils,
};

use super::{Curd, Validator};

/// 商家微信支付配置
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct WechatConfig {
    /// 配置ID
    pub id: Option<i64>,

    /// 商家ID
    pub store_id: i64,

    // 服务商公众号或小程序appid
    pub sp_appid: Option<String>,
    // 服务商商户号
    pub sp_mchid: Option<String>,
    // 公众号或小程序或绑定到三方平台应用的appid,
    // 如果是服务商模式，此处填写服务商的appid
    pub app_id: String,
    // 商户号，如果是服务商模式，此处填写服务商的商户号
    pub mchid: String,
    // 商户支付密钥
    pub mch_key: String,
    // 商户证书内容文件路径
    pub apiclient_key: String,
    // 商户证书内容文件路径
    pub apiclient_cert: String,

    /// 是否激活
    pub is_active: bool,

    /// 创建时间
    pub created_at: i64,

    /// 更新时间
    pub updated_at: i64,
}

impl Validator for WechatConfig {
    fn validate(&self) -> Result<()> {
        if self.store_id == 0 {
            return Err(Error::bad_request("商家ID不能为空"));
        }
        if self.app_id.is_empty() {
            return Err(Error::bad_request(
                "公众号或小程序或绑定到三方平台应用的appid不能为空",
            ));
        }
        if self.mchid.is_empty() {
            return Err(Error::bad_request("商户号不能为空"));
        }

        Ok(())
    }
}

impl FromRow<'_, SqliteRow> for WechatConfig {
    fn from_row(row: &SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id").unwrap_or_default(),
            store_id: row.try_get("store_id").unwrap_or_default(),
            sp_appid: row.try_get("sp_appid").unwrap_or_default(),
            sp_mchid: row.try_get("sp_mchid").unwrap_or_default(),
            app_id: row.try_get("app_id").unwrap_or_default(),
            mchid: row.try_get("mchid").unwrap_or_default(),
            mch_key: row.try_get("mch_key").unwrap_or_default(),
            apiclient_key: row.try_get("apiclient_key").unwrap_or_default(),
            apiclient_cert: row.try_get("apiclient_cert").unwrap_or_default(),
            is_active: row.try_get("is_active").unwrap_or_default(),
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
        })
    }
}

impl Curd for WechatConfig {
    const COUNT_SQL: &'static str = "SELECT COUNT(*) FROM wechat_configs";

    const QUERY_SQL: &'static str = "SELECT * FROM wechat_configs";

    const BY_ID_SQL: &'static str = "SELECT * FROM wechat_configs WHERE id = $1";

    const DELETE_BATCH_SQL: &'static str = "DELETE FROM wechat_configs WHERE id = ANY($1)";
}

impl WechatConfig {
    /// 创建或更新商家微信支付配置
    pub async fn upsert(&self, pool: &Pool<Sqlite>) -> Result<Self> {
        let result = sqlx::query_as(
            "
        INSERT INTO wechat_configs (
            id, store_id, sp_appid, sp_mchid, app_id, mchid,
            mch_key, apiclient_key, apiclient_cert, is_active, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        ON CONFLICT(id) DO UPDATE SET
            sp_appid = excluded.sp_appid,
            sp_mchid = excluded.sp_mchid,
            app_id = excluded.app_id,
            mchid = excluded.mchid,
            mch_key = excluded.mch_key,
            apiclient_key = excluded.apiclient_key,
            apiclient_cert = excluded.apiclient_cert,
            is_active = excluded.is_active,
            updated_at = excluded.updated_at
        RETURNING *
        ",
        )
        .bind(self.id)
        .bind(self.store_id)
        .bind(&self.sp_appid)
        .bind(&self.sp_mchid)
        .bind(&self.app_id)
        .bind(&self.mchid)
        .bind(&self.mch_key)
        .bind(&self.apiclient_key)
        .bind(&self.apiclient_cert)
        .bind(self.is_active)
        .bind(utils::get_timestamp()) // created_at (只在插入时使用)
        .bind(utils::get_timestamp()) // updated_at (始终更新)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }
    /// 获取商家的微信支付配置
    pub async fn get_by_store_id(pool: &Pool<Sqlite>, store_id: i64) -> Result<Option<Self>> {
        let config = sqlx::query_as(
            "
            SELECT * FROM wechat_configs
            WHERE store_id = $1 AND is_active = true
            LIMIT 1
            ",
        )
        .bind(store_id)
        .fetch_optional(pool)
        .await?;

        Ok(config)
    }

    /// 停用商家的微信支付配置
    pub async fn deactivate(pool: &Pool<Sqlite>, store_id: i64) -> Result<Self> {
        let result = sqlx::query_as(
            "
            UPDATE wechat_configs
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
pub async fn save_wechat_config(
    state: State<'_, AppState>,
    config: WechatConfig,
) -> Result<WechatConfig> {
    config.validate()?;
    config.upsert(&state.pool).await
}

#[tauri::command]
pub async fn get_wechat_config(
    state: State<'_, AppState>,
    store_id: i64,
) -> Result<Option<WechatConfig>> {
    if store_id == 0 {
        return Err(Error::bad_request("商家ID不能为空"));
    }
    WechatConfig::get_by_store_id(&state.pool, store_id).await
}

#[tauri::command]
pub async fn deactivate_wechat(state: State<'_, AppState>, store_id: i64) -> Result<WechatConfig> {
    if store_id == 0 {
        return Err(Error::bad_request("商家ID不能为空"));
    }
    WechatConfig::deactivate(&state.pool, store_id).await
}
