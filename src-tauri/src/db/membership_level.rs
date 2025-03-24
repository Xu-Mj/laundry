use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::types::chrono::{DateTime, FixedOffset};
use sqlx::{FromRow, Pool, QueryBuilder, Row, Sqlite};
use tauri::State;

use crate::db::{Curd, PageParams, PageResult, Validator};
use crate::error::{Error, Result};
use crate::state::AppState;
use crate::utils;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct MembershipLevel {
    pub level_id: Option<i64>,                      // 会员等级ID (自动增长)
    pub level_code: Option<String>,                 // 等级编码
    pub level_name: Option<String>,                 // 等级名称
    pub level_sort: i32,                            // 显示顺序
    pub status: Option<String>,                     // 状态 (0: 正常, 1: 停用)
    pub create_time: Option<DateTime<FixedOffset>>, // 创建时间
    pub update_time: Option<DateTime<FixedOffset>>, // 更新时间
    pub remark: Option<String>,                     // 备注信息
}

impl FromRow<'_, SqliteRow> for MembershipLevel {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            level_id: row.try_get("level_id").unwrap_or_default(),
            level_code: row.try_get("level_code").unwrap_or_default(),
            level_name: row.try_get("level_name").unwrap_or_default(),
            level_sort: row.try_get("level_sort").unwrap_or_default(),
            status: row.try_get("status").unwrap_or_default(),
            create_time: row.try_get("create_time").unwrap_or_default(),
            update_time: row.try_get("update_time").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
        })
    }
}

impl Validator for MembershipLevel {
    fn validate(&self) -> Result<()> {
        if self.level_name.is_none() {
            return Err(Error::bad_request("会员等级名称不能为空"));
        }

        if self.level_code.is_none() {
            return Err(Error::bad_request("会员等级编码不能为空"));
        }
        Ok(())
    }
}

impl Curd for MembershipLevel {
    const COUNT_SQL: &'static str = "SELECT COUNT(1) FROM membership_level WHERE 1=1 ";
    const QUERY_SQL: &'static str = "SELECT * FROM membership_level WHERE 1=1 ";
    const BY_ID_SQL: &'static str = "SELECT * FROM membership_level WHERE level_id = ? ";
    const DELETE_BATCH_SQL: &'static str = "DELETE FROM membership_level WHERE level_id IN ( ";
    const ORDER_SQL: Option<&'static str> = Some("ORDER BY level_sort ASC");

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, Sqlite>) {
        if let Some(level_name) = &self.level_name {
            builder
                .push("AND level_name LIKE ")
                .push_bind(format!("%{}%", level_name));
        }

        if let Some(level_code) = &self.level_code {
            builder
                .push("AND level_code LIKE ")
                .push_bind(format!("%{}%", level_code));
        }

        if let Some(status) = &self.status {
            builder.push("AND status = ").push_bind(status);
        }
    }
}

impl MembershipLevel {
    // insert
    pub async fn insert(&mut self, tx: &mut sqlx::Transaction<'_, Sqlite>) -> Result<()> {
        let id = sqlx::query("INSERT INTO membership_level (level_code, level_name, level_sort, status, create_time, remark) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&self.level_code)
            .bind(&self.level_name)
            .bind(self.level_sort)
            .bind(&self.status)
            .bind(utils::get_now())
            .bind(&self.remark)
            .execute(&mut **tx)
            .await?
            .last_insert_rowid();
        self.level_id = Some(id);
        Ok(())
    }

    // update
    pub async fn update(&self, tx: &mut sqlx::Transaction<'_, Sqlite>) -> Result<bool> {
        let result = sqlx::query("UPDATE membership_level SET level_code = ?, level_name = ?, level_sort = ?, status = ?, update_time = ?, remark = ? WHERE level_id = ?")
            .bind(&self.level_code)
            .bind(&self.level_name)
            .bind(self.level_sort)
            .bind(&self.status)
            .bind(utils::get_now())
            .bind(&self.remark)
            .bind(self.level_id.unwrap())
            .execute(&mut **tx)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // check level name unique
    pub async fn check_level_name_unique(pool: &Pool<Sqlite>, level_name: &str) -> Result<bool> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(1) FROM membership_level WHERE level_name = ?",
        )
        .bind(level_name)
        .fetch_one(pool)
        .await?;
        Ok(result == 0)
    }

    // check level code unique
    pub async fn check_level_code_unique(pool: &Pool<Sqlite>, level_code: &str) -> Result<bool> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(1) FROM membership_level WHERE level_code = ?",
        )
        .bind(level_code)
        .fetch_one(pool)
        .await?;
        Ok(result == 0)
    }
}

#[tauri::command]
pub async fn get_membership_level_pagination(
    state: State<'_, AppState>,
    page_params: PageParams,
    ml: MembershipLevel,
) -> Result<PageResult<MembershipLevel>> {
    ml.get_list(&state.pool, page_params).await
}

#[tauri::command]
pub async fn get_membership_level_by_id(
    state: State<'_, AppState>,
    id: i64,
) -> Result<Option<MembershipLevel>> {
    MembershipLevel::get_by_id(&state.pool, id).await
}

#[tauri::command]
pub async fn get_membership_level_list(
    state: State<'_, AppState>,
    ml: MembershipLevel,
) -> Result<Vec<MembershipLevel>> {
    ml.get_all(&state.pool).await
}

#[tauri::command]
pub async fn create_membership_level(
    state: State<'_, AppState>,
    mut ml: MembershipLevel,
) -> Result<MembershipLevel> {
    ml.validate()?;
    let mut tx = state.pool.begin().await?;
    if !MembershipLevel::check_level_name_unique(&state.pool, ml.level_name.as_ref().unwrap())
        .await?
    {
        return Err(Error::bad_request("会员等级名称已存在"));
    }

    if !MembershipLevel::check_level_code_unique(&state.pool, ml.level_code.as_ref().unwrap())
        .await?
    {
        return Err(Error::bad_request("会员等级编码已存在"));
    }

    ml.insert(&mut tx).await?;
    tx.commit().await?;
    Ok(ml)
}

#[tauri::command]
pub async fn update_membership_level(
    state: State<'_, AppState>,
    ml: MembershipLevel,
) -> Result<bool> {
    ml.validate()?;

    let mut tx = state.pool.begin().await?;
    let result = ml.update(&mut tx).await?;
    tx.commit().await?;
    Ok(result)
}

#[tauri::command]
pub async fn delete_membership_level(state: State<'_, AppState>, ids: Vec<i64>) -> Result<bool> {
    let mut tx = state.pool.begin().await?;
    let result = MembershipLevel::delete_batch(&mut tx, &ids).await?;
    tx.commit().await?;
    Ok(result)
}
