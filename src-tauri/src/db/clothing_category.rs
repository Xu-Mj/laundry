use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Pool, QueryBuilder, Row, Sqlite};
use tauri::State;

use crate::error::{Error, ErrorKind, Result};
use crate::state::AppState;
use crate::utils;
use crate::utils::request::Request;

use super::{Curd, PageParams, PageResult, Validator};

/// 衣物品类表
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct ClothingCategory {
    /// 品类ID
    pub category_id: Option<i64>,
    /// 商家ID，用于数据隔离
    pub store_id: i64,
    /// 品类编码
    pub category_code: String,
    /// 品类名称
    pub category_name: String,
    /// 显示顺序
    pub order_num: Option<i64>,
    /// 备注
    pub remark: Option<String>,
    /// 删除标志（0代表存在 2代表删除）
    pub del_flag: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl FromRow<'_, SqliteRow> for ClothingCategory {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            category_id: row.try_get("category_id").unwrap_or_default(),
            store_id: row.try_get("store_id").unwrap_or_default(),
            category_code: row.try_get("category_code").unwrap_or_default(),
            category_name: row.try_get("category_name").unwrap_or_default(),
            order_num: row.try_get("order_num").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
            del_flag: row.try_get("del_flag").unwrap_or_default(),
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
        })
    }
}

impl Validator for ClothingCategory {
    fn validate(&self) -> Result<()> {
        if self.store_id <= 0 {
            return Err(Error::bad_request("商家ID不能为空"));
        }
        if self.category_name.trim().is_empty() {
            return Err(Error::bad_request("品类名称不能为空"));
        }
        Ok(())
    }
}

impl Curd for ClothingCategory {
    const COUNT_SQL: &'static str =
        "SELECT COUNT(*) FROM clothing_categories WHERE del_flag = '0' ";
    const QUERY_SQL: &'static str = "SELECT * FROM clothing_categories WHERE del_flag = '0' ";
    const BY_ID_SQL: &'static str =
        "SELECT * FROM clothing_categories WHERE category_id = ? AND del_flag = '0' ";
    const DELETE_BATCH_SQL: &'static str =
        "UPDATE clothing_categories SET del_flag = '2' WHERE category_id IN (";
    const ORDER_SQL: Option<&'static str> = Some("ORDER BY order_num ASC");

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, Sqlite>) {
        if self.store_id > 0 {
            builder.push(" AND store_id = ").push_bind(self.store_id);
        }

        if let Some(category_id) = &self.category_id {
            builder.push(" AND category_id = ").push_bind(category_id);
        }

        if !self.category_name.is_empty() {
            builder
                .push(" AND category_name LIKE ")
                .push_bind(format!("%{}%", self.category_name));
        }

        if !self.category_code.is_empty() {
            builder
                .push(" AND category_code = ")
                .push_bind(&self.category_code);
        }

        if let Some(del_flag) = &self.del_flag {
            builder.push(" AND del_flag = ").push_bind(del_flag);
        }
    }
}

impl ClothingCategory {
    /// 创建新的衣物品类
    pub async fn insert(self, pool: &Pool<Sqlite>) -> Result<Self> {
        let now = utils::get_timestamp();
        let result = sqlx::query_as::<_, Self>(
            "INSERT INTO clothing_categories (category_id, store_id, category_code, category_name, order_num, remark, del_flag, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, '0', ?, ?)
             RETURNING *"
        )
            .bind(self.category_id)
            .bind(self.store_id)
            .bind(&self.category_code)
            .bind(&self.category_name)
            .bind(&self.order_num.unwrap_or(0))
            .bind(&self.remark)
            .bind(now)
            .bind(now)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    /// 更新衣物品类
    pub async fn update(self, pool: &Pool<Sqlite>) -> Result<bool> {
        let mut query_builder = QueryBuilder::<Sqlite>::new("UPDATE clothing_categories SET ");
        let mut has_update = false;

        // 始终更新category_name，即使是空字符串也会更新
        if has_update {
            query_builder.push(", ");
        }
        query_builder
            .push("category_name = ")
            .push_bind(&self.category_name);
        has_update = true;

        // 始终更新category_code，即使是空字符串也会更新
        if has_update {
            query_builder.push(", ");
        }
        query_builder
            .push("category_code = ")
            .push_bind(&self.category_code);
        has_update = true;

        // 处理可空字段，无论是Some还是None都会更新
        if has_update {
            query_builder.push(", ");
        }
        query_builder
            .push("order_num = ")
            .push_bind(&self.order_num);
        has_update = true;

        if has_update {
            query_builder.push(", ");
        }
        query_builder.push("remark = ").push_bind(&self.remark);
        has_update = true;

        if has_update {
            query_builder.push(", ");
        }
        query_builder.push("del_flag = ").push_bind(&self.del_flag);
        has_update = true;

        // 添加更新时间
        if has_update {
            query_builder
                .push(", updated_at = ")
                .push_bind(utils::get_timestamp());
        }

        if has_update {
            query_builder
                .push(" WHERE store_id = ")
                .push_bind(self.store_id);

            if let Some(category_id) = self.category_id {
                query_builder
                    .push(" AND category_id = ")
                    .push_bind(category_id);
            } else {
                return Err(Error::with_details(ErrorKind::BadRequest, "缺少品类ID"));
            }

            let result = query_builder.build().execute(pool).await?;
            Ok(result.rows_affected() > 0)
        } else {
            Ok(false)
        }
    }

    /// 检查品类名称是否已经存在
    pub async fn exists_by_name(
        pool: &Pool<Sqlite>,
        store_id: i64,
        category_name: &str,
    ) -> Result<bool> {
        let result = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM clothing_categories WHERE store_id = ? AND category_name = ? AND del_flag = '0')",
        )
        .bind(store_id)
        .bind(category_name)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    /// 软删除衣物品类
    pub async fn soft_delete(pool: &Pool<Sqlite>, category_id: i64) -> Result<u64> {
        let result =
            sqlx::query("UPDATE clothing_categories SET del_flag = '2' WHERE category_id = ?")
                .bind(category_id)
                .execute(pool)
                .await?;

        Ok(result.rows_affected())
    }
}

impl Request for ClothingCategory {
    const URL: &str = "/clothing/category";
}

// Tauri命令接口
#[tauri::command]
pub async fn list_clothing_category_pagination(
    state: State<'_, AppState>,
    mut category: ClothingCategory,
    page_params: PageParams,
) -> Result<PageResult<ClothingCategory>> {
    let store_id = utils::get_user_id(&state).await?; // check user is login
    category.store_id = store_id;
    category.get_list(&state.pool, page_params).await
}

#[tauri::command]
pub async fn list_clothing_category_all(
    state: State<'_, AppState>,
    mut category: ClothingCategory,
) -> Result<Vec<ClothingCategory>> {
    let store_id = utils::get_user_id(&state).await?; // check user is login
    category.store_id = store_id;
    category.get_all(&state.pool).await
}

#[tauri::command]
pub async fn get_clothing_category_by_id(
    state: State<'_, AppState>,
    id: i64,
) -> Result<Option<ClothingCategory>> {
    let pool = &state.pool;
    ClothingCategory::get_by_id(pool, id).await
}

#[tauri::command]
pub async fn add_clothing_category(
    state: State<'_, AppState>,
    mut category: ClothingCategory,
) -> Result<ClothingCategory> {
    let store_id = utils::get_user_id(&state).await?; // check user is login
    category.store_id = store_id;
    let pool = &state.pool;
    category.validate()?;
    // check category_name is exists
    if ClothingCategory::exists_by_name(pool, category.store_id, &category.category_name).await? {
        return Err(Error::bad_request("品类名称已经存在"));
    }
    // create to server
    let category = category.create_request(&state).await?;
    category.insert(pool).await
}

#[tauri::command]
pub async fn update_clothing_category(
    state: State<'_, AppState>,
    category: ClothingCategory,
) -> Result<bool> {
    let pool = &state.pool;
    category.validate()?;
    // check category_name is exists
    if ClothingCategory::exists_by_name(pool, category.store_id, &category.category_name).await? {
        return Err(Error::bad_request("品类名称已经存在"));
    }
    // update to server
    if !category.update_request(&state).await? {
        return Err(Error::bad_request("更新失败"));
    }
    category.update(pool).await
}

#[tauri::command]
pub async fn delete_clothing_category_batch(
    state: State<'_, AppState>,
    ids: Vec<i64>,
) -> Result<u64> {
    let pool = &state.pool;
    let mut affected = 0;
    for id in ids {
        affected += ClothingCategory::soft_delete(pool, id).await?;
    }
    Ok(affected)
}
