use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::types::chrono::{DateTime, FixedOffset};
use sqlx::{FromRow, Pool, QueryBuilder, Row, Sqlite};
use tauri::State;

use crate::db::{AppState, Curd, PageParams, PageResult};
use crate::error::Result;
use crate::utils;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct DictType {
    pub dict_id: Option<i64>,                       // 自增主键，查询时可为 None
    pub dict_name: Option<String>,                  // 字典名称
    pub dict_type: Option<String>,                  // 字典类型
    pub status: Option<String>,                     // 状态
    pub create_time: Option<DateTime<FixedOffset>>, // 创建时间
    pub update_time: Option<DateTime<FixedOffset>>, // 更新时间
    pub remark: Option<String>,                     // 备注
}

impl FromRow<'_, SqliteRow> for DictType {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            dict_id: row.try_get("dict_id").unwrap_or_default(),
            dict_name: row.try_get("dict_name").unwrap_or_default(),
            dict_type: row.try_get("dict_type").unwrap_or_default(),
            status: row.try_get("status").unwrap_or_default(),
            create_time: row.try_get("create_time").unwrap_or_default(),
            update_time: row.try_get("update_time").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
        })
    }
}

impl Curd for DictType {
    const COUNT_SQL: &'static str = "SELECT COUNT(1) FROM dict_type ";
    const QUERY_SQL: &'static str = "SELECT * FROM dict_type WHERE 1=1 ";
    const BY_ID_SQL: &'static str = "SELECT * FROM dict_type WHERE dict_id = ?";
    const DELETE_BATCH_SQL: &'static str = "DELETE FROM dict_type WHERE dict_id IN ( ";

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, Sqlite>) {
        if let Some(dict_name) = &self.dict_name {
            builder
                .push(" AND dict_name LIKE ")
                .push_bind(format!("%{}%", dict_name));
        }

        if let Some(dict_type) = &self.dict_type {
            builder
                .push(" AND dict_type LIKE ")
                .push_bind(format!("%{}%", dict_type));
        }

        if let Some(status) = &self.status {
            builder.push(" AND status = ").push_bind(status);
        }
    }
}

impl DictType {
    // insert
    pub async fn create_dict_type(&self, pool: &Pool<Sqlite>) -> Result<Self> {
        let query = r#"
        INSERT INTO dict_type (dict_name, dict_type, status, create_time, update_time, remark)
        VALUES (?, ?, ?, ?, ?, ?) RETURNING dict_id
    "#;

        let result = sqlx::query_as(query)
            .bind(&self.dict_name)
            .bind(&self.dict_type)
            .bind(&self.status)
            .bind(utils::get_now())
            .bind(&self.update_time)
            .bind(&self.remark)
            .fetch_one(pool)
            .await?;
        Ok(result)
    }

    // update
    pub async fn update_dict_type(&self, pool: &Pool<Sqlite>) -> Result<bool> {
        let query = r#"
        UPDATE dict_type SET dict_name = ?, dict_type = ?, status = ?, update_time = ?, remark = ?
        WHERE dict_id = ?
    "#;

        let result = sqlx::query(query)
            .bind(&self.dict_name)
            .bind(&self.dict_type)
            .bind(&self.status)
            .bind(utils::get_now())
            .bind(&self.remark)
            .bind(&self.dict_id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

#[tauri::command]
pub async fn get_dict_type_list(
    state: State<'_, AppState>,
    page_params: PageParams,
    dict_type: DictType,
) -> Result<PageResult<DictType>> {
    dict_type.get_list(&state.0, page_params).await
}

#[tauri::command]
pub async fn get_dict_type_all(
    state: State<'_, AppState>,
    dict_type: DictType,
) -> Result<Vec<DictType>> {
    dict_type.get_all(&state.0).await
}

#[tauri::command]
pub async fn get_dict_type_by_id(state: State<'_, AppState>, id: i64) -> Result<Option<DictType>> {
    DictType::get_by_id(&state.0, id).await
}

// add
#[tauri::command]
pub async fn add_dict_type(state: State<'_, AppState>, dict_type: DictType) -> Result<DictType> {
    let result = dict_type.create_dict_type(&state.0).await?;
    Ok(result)
}

#[tauri::command]
pub async fn update_dict_type(state: State<'_, AppState>, dict_type: DictType) -> Result<bool> {
    dict_type.update_dict_type(&state.0).await
}

#[tauri::command]
pub async fn delete_dict_types(state: State<'_, AppState>, ids: Vec<i64>) -> Result<bool> {
    let mut tr = state.0.begin().await?;
    let result = DictType::delete_batch(&mut tr, &ids).await?;
    tr.commit().await?;
    Ok(result)
}
