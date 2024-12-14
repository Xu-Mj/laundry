use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::types::chrono::{DateTime, FixedOffset};
use sqlx::{FromRow, Pool, QueryBuilder, Row, Sqlite};
use tauri::State;

use crate::db::{AppState, Curd, PageParams, PageResult};
use crate::error::Result;
use crate::utils;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct DictData {
    pub dict_code: Option<i64>,     // 自增主键，查询时可为 None
    pub dict_sort: Option<i32>,     // 排序
    pub dict_label: Option<String>, // 字典标签
    pub dict_value: Option<String>, // 字典值
    pub dict_type: Option<String>,  // 字典类型，外键
    pub css_class: Option<String>,  // 样式类名
    pub list_class: Option<String>, // 列表类名
    pub is_default: Option<String>, // 是否默认
    pub status: Option<String>,     // 状态
    pub create_time: Option<DateTime<FixedOffset>>, // 创建时间
    pub update_time: Option<DateTime<FixedOffset>>, // 更新时间
    pub remark: Option<String>,     // 备注
}

impl FromRow<'_, SqliteRow> for DictData {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            dict_code: row.try_get("dict_code").unwrap_or_default(),
            dict_sort: row.try_get("dict_sort").unwrap_or_default(),
            dict_label: row.try_get("dict_label").unwrap_or_default(),
            dict_value: row.try_get("dict_value").unwrap_or_default(),
            dict_type: row.try_get("dict_type").unwrap_or_default(),
            css_class: row.try_get("css_class").unwrap_or_default(),
            list_class: row.try_get("list_class").unwrap_or_default(),
            is_default: row.try_get("is_default").unwrap_or_default(),
            status: row.try_get("status").unwrap_or_default(),
            create_time: row.try_get("create_time").unwrap_or_default(),
            update_time: row.try_get("update_time").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
        })
    }
}

impl Curd for DictData {
    const COUNT_SQL: &'static str = "SELECT COUNT(1) FROM dict_data WHERE 1=1 ";
    const QUERY_SQL: &'static str = "SELECT * FROM dict_data WHERE 1=1 ";
    const BY_ID_SQL: &'static str = "SELECT * FROM dict_data WHERE dict_code = ?";
    const DELETE_BATCH_SQL: &'static str = "DELETE FROM dict_data WHERE dict_code IN ( ";
    const ORDER_SQL: Option<&'static str> = Some("ORDER BY dict_sort ASC");

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, Sqlite>) {
        if let Some(dict_label) = &self.dict_label {
            builder
                .push(" AND dict_label LIKE ")
                .push_bind(format!("%{}%", dict_label));
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

impl DictData {
    // insert
    pub async fn create_dict_data(&self, pool: &Pool<Sqlite>) -> Result<Self> {
        let query = r#"
        INSERT INTO dict_data (dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, create_time, update_time, remark)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *
    "#;

        let result = sqlx::query_as(query)
            .bind(&self.dict_sort)
            .bind(&self.dict_label)
            .bind(&self.dict_value)
            .bind(&self.dict_type)
            .bind(&self.css_class)
            .bind(&self.list_class)
            .bind(&self.is_default)
            .bind(&self.status)
            .bind(utils::get_now())
            .bind(&self.update_time)
            .bind(&self.remark)
            .fetch_one(pool)
            .await?;
        Ok(result)
    }

    pub async fn update_dict_data(&self, pool: &Pool<Sqlite>) -> Result<bool> {
        let query = r#"
        UPDATE dict_data SET dict_sort = ?, dict_label = ?, dict_value = ?, dict_type = ?, css_class = ?, list_class = ?, is_default = ?, status = ?, update_time = ?, remark = ?
        WHERE dict_code = ?
    "#;

        let result = sqlx::query(query)
            .bind(&self.dict_sort)
            .bind(&self.dict_label)
            .bind(&self.dict_value)
            .bind(&self.dict_type)
            .bind(&self.css_class)
            .bind(&self.list_class)
            .bind(&self.is_default)
            .bind(&self.status)
            .bind(utils::get_now())
            .bind(&self.remark)
            .bind(&self.dict_code)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    /// select by dict_type
    pub async fn select_by_dict_type(pool: &Pool<Sqlite>, dict_type: &str) -> Result<Vec<Self>> {
        let result = sqlx::query_as(
            "SELECT * FROM dict_data WHERE status = '0' AND dict_type = ? ORDER BY dict_sort ASC",
        )
        .bind(dict_type)
        .fetch_all(pool)
        .await?;
        Ok(result)
    }

    /// select dict_label from sys_dict_data
    /// 		where dict_type = #{dictType} and dict_value = #{dictValue}
    #[allow(dead_code)]
    pub async fn select_dict_label(
        pool: &Pool<Sqlite>,
        dict_type: &str,
        dict_value: &str,
    ) -> Result<Option<String>> {
        let result = sqlx::query_scalar(
            "SELECT dict_label FROM dict_data WHERE dict_type = ? AND dict_value = ?",
        )
        .bind(dict_type)
        .bind(dict_value)
        .fetch_optional(pool)
        .await?;
        Ok(result)
    }

    // select count(1) from sys_dict_data where dict_type=#{dictType}
    #[allow(dead_code)]
    pub async fn select_count_by_dict_type(pool: &Pool<Sqlite>, dict_type: &str) -> Result<i64> {
        let result = sqlx::query_scalar("SELECT COUNT(1) FROM dict_data WHERE dict_type = ?")
            .bind(dict_type)
            .fetch_one(pool)
            .await?;
        Ok(result)
    }

    // update sys_dict_data set dict_type = #{newDictType} where dict_type = #{oldDictType}
    #[allow(dead_code)]
    pub async fn update_dict_type(
        pool: &Pool<Sqlite>,
        old_dict_type: &str,
        new_dict_type: &str,
    ) -> Result<bool> {
        let result = sqlx::query("UPDATE dict_data SET dict_type = ? WHERE dict_type = ?")
            .bind(new_dict_type)
            .bind(old_dict_type)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

#[tauri::command]
pub async fn get_dict_data_list(
    state: State<'_, AppState>,
    page_params: PageParams,
    dict_data: DictData,
) -> Result<PageResult<DictData>> {
    dict_data.get_list(&state.0, page_params).await
}

#[tauri::command]
pub async fn get_by_dict_type(
    state: State<'_, AppState>,
    dict_type: String,
) -> Result<Vec<DictData>> {
    DictData::select_by_dict_type(&state.0, &dict_type).await
}

#[tauri::command]
pub async fn get_dict_data_by_code(
    state: State<'_, AppState>,
    code: i64,
) -> Result<Option<DictData>> {
    DictData::get_by_id(&state.0, code).await
}

#[tauri::command]
pub async fn add_dict_data(state: State<'_, AppState>, dict_data: DictData) -> Result<DictData> {
    dict_data.create_dict_data(&state.0).await
}

#[tauri::command]
pub async fn update_dict_data(state: State<'_, AppState>, dict_data: DictData) -> Result<bool> {
    dict_data.update_dict_data(&state.0).await
}

#[tauri::command]
pub async fn delete_dict_data(state: State<'_, AppState>, ids: Vec<i64>) -> Result<bool> {
    let mut tr = state.0.begin().await?;
    let result = DictData::delete_batch(&mut tr, &ids).await?;
    tr.commit().await?;
    Ok(result)
}
