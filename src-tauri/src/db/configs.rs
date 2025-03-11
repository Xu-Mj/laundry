use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::types::chrono::{DateTime, FixedOffset};
use sqlx::{Error, FromRow, Pool, QueryBuilder, Row, Sqlite};
use tauri::State;

use crate::db::{Curd, PageParams, PageResult};
use crate::error::Result;
use crate::state::AppState;
use crate::utils;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Config {
    pub config_id: Option<i32>,                     // 参数主键
    pub config_name: Option<String>,                // 参数名称
    pub config_key: Option<String>,                 // 参数键名
    pub config_value: Option<String>,               // 参数键值
    pub config_type: Option<String>,                // 系统内置（Y是 N否）
    pub create_by: Option<String>,                  // 创建者
    pub create_time: Option<DateTime<FixedOffset>>, // 创建时间
    pub update_by: Option<String>,                  // 更新者
    pub update_time: Option<DateTime<FixedOffset>>, // 更新时间
    pub remark: Option<String>,                     // 备注
}

impl FromRow<'_, SqliteRow> for Config {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, Error> {
        Ok(Self {
            config_id: row.try_get("config_id").unwrap_or_default(),
            config_name: row.try_get("config_name").unwrap_or_default(),
            config_key: row.try_get("config_key").unwrap_or_default(),
            config_value: row.try_get("config_value").unwrap_or_default(),
            config_type: row.try_get("config_type").unwrap_or_default(),
            create_by: row.try_get("create_by").unwrap_or_default(),
            create_time: row.try_get("create_time").unwrap_or_default(),
            update_by: row.try_get("update_by").unwrap_or_default(),
            update_time: row.try_get("update_time").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
        })
    }
}

impl Curd for Config {
    const COUNT_SQL: &'static str = "SELECT COUNT(*) FROM configs WHERE 1=1 ";
    const QUERY_SQL: &'static str = "SELECT * FROM configs WHERE 1=1";
    const BY_ID_SQL: &'static str = "SELECT * FROM configs WHERE config_id = ?";
    const DELETE_BATCH_SQL: &'static str = "DELETE FROM configs WHERE config_id IN (";

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, Sqlite>) {
        if let Some(config_name) = &self.config_name {
            builder
                .push(" AND config_name LIKE ")
                .push_bind(format!("%{}%", config_name));
        }

        if let Some(config_type) = &self.config_type {
            builder.push(" AND config_type = ").push_bind(config_type);
        }

        if let Some(config_key) = &self.config_key {
            builder
                .push(" AND config_key LIKE ")
                .push_bind(format!("%{}%", config_key));
        }
    }
}

impl Config {
    pub async fn create(&self, pool: &Pool<Sqlite>) -> Result<Self> {
        let result = sqlx::query_as(
            "INSERT INTO configs
                (config_name, config_key, config_value, config_type,
                 create_by, create_time, update_by, update_time, remark)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                RETURNING *",
        )
        .bind(&self.config_name)
        .bind(&self.config_key)
        .bind(&self.config_value)
        .bind(&self.config_type)
        .bind(&self.create_by)
        .bind(utils::get_now())
        .bind(&self.update_by)
        .bind(&self.update_time)
        .bind(&self.remark)
        .fetch_one(pool)
        .await?;
        Ok(result)
    }

    pub async fn update(&self, pool: &Pool<Sqlite>) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE configs
                SET config_name = ?, config_key = ?, config_value = ?, config_type = ?,
                 update_by = ?, update_time = ?, remark = ?
                WHERE config_id = ?",
        )
        .bind(&self.config_name)
        .bind(&self.config_key)
        .bind(&self.config_value)
        .bind(&self.config_type)
        .bind(&self.update_by)
        .bind(utils::get_now())
        .bind(&self.remark)
        .bind(&self.config_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_config_by_key(pool: &Pool<Sqlite>, key: &str) -> Result<Option<Self>> {
        let result = sqlx::query_as(
            "SELECT config_id, config_name, config_key, config_value, config_type,
                 create_by, create_time, update_by, update_time, remark
                FROM configs
                WHERE config_key = ?",
        )
        .bind(key)
        .fetch_optional(pool)
        .await?;
        Ok(result)
    }
}

impl Config {
    pub async fn is_captcha_enabled(pool: &Pool<Sqlite>) -> Result<bool> {
        let config = Config::get_config_by_key(pool, "sys.account.captchaEnabled").await?;
        if config.is_none() {
            return Ok(true);
        }
        Ok(utils::to_bool(
            config.unwrap().config_value.unwrap_or_default(),
        ))
    }
}

#[tauri::command]
pub async fn get_config_list(
    state: State<'_, AppState>,
    page_params: PageParams,
    config: Config,
) -> Result<PageResult<Config>> {
    config.get_list(&state.pool, page_params).await
}

#[tauri::command]
pub async fn get_config_by_id(state: State<'_, AppState>, id: i64) -> Result<Option<Config>> {
    Config::get_by_id(&state.pool, id).await
}

#[tauri::command]
pub async fn delete_configs(state: State<'_, AppState>, ids: Vec<i64>) -> Result<bool> {
    let mut tr = state.pool.begin().await?;
    let result = Config::delete_batch(&mut tr, &ids).await?;
    tr.commit().await?;
    Ok(result)
}

// todo add create user or delete create_by attr
#[tauri::command]
pub async fn add_config(state: State<'_, AppState>, config: Config) -> Result<Config> {
    tracing::debug!("add config: {:?}", config);
    config.create(&state.pool).await
}

#[tauri::command]
pub async fn update_config(state: State<'_, AppState>, config: Config) -> Result<bool> {
    config.update(&state.pool).await
}

#[tauri::command]
pub async fn get_config_by_key(state: State<'_, AppState>, key: String) -> Result<Option<Config>> {
    Config::get_config_by_key(&state.pool, &key).await
}
