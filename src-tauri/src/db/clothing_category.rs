use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Pool, QueryBuilder, Row, Sqlite};
use tauri::State;

use crate::error::{Error, ErrorKind, Result};
use crate::state::AppState;
use crate::utils;
use crate::utils::request::{Request, StoreIdWithIds};

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
        if self.store_id >= 0 {
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
        // 验证category_id是否存在
        let category_id = match self.category_id {
            Some(id) => id,
            None => return Err(Error::with_details(ErrorKind::BadRequest, "缺少品类ID")),
        };

        // 使用单一SQL语句更新所有字段
        let now = utils::get_timestamp();
        let result = sqlx::query(
            "UPDATE clothing_categories SET 
             category_name = ?, 
             category_code = ?, 
             order_num = ?, 
             remark = ?, 
             del_flag = ?, 
             updated_at = ? 
             WHERE store_id = ? AND category_id = ?",
        )
        .bind(&self.category_name)
        .bind(&self.category_code)
        .bind(&self.order_num)
        .bind(&self.remark)
        .bind(&self.del_flag)
        .bind(now)
        .bind(self.store_id)
        .bind(category_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 检查品类名称是否已经存在
    ///
    /// 如果提供了category_id，则会排除该ID的记录，用于更新时检查
    pub async fn exists_by_name(
        pool: &Pool<Sqlite>,
        store_id: i64,
        category_name: &str,
        exclude_category_id: Option<i64>,
    ) -> Result<bool> {
        let sql = match exclude_category_id {
            Some(_) => {
                "SELECT EXISTS(SELECT 1 FROM clothing_categories WHERE store_id = ? AND category_name = ? AND category_id != ? AND del_flag = '0')"
            }
            None => {
                "SELECT EXISTS(SELECT 1 FROM clothing_categories WHERE store_id = ? AND category_name = ? AND del_flag = '0')"
            }
        };

        let mut query = sqlx::query_scalar(sql).bind(store_id).bind(category_name);

        if let Some(id) = exclude_category_id {
            query = query.bind(id);
        }

        let result = query.fetch_one(pool).await?;

        Ok(result)
    }
}

impl Request for ClothingCategory {
    const URL: &str = "/clothing/category";
    async fn delete_request(state: &State<'_, AppState>, body: StoreIdWithIds) -> Result<bool> {
        let result = state
            .http_client
            .delete(Self::URL, body, Some(&state.try_get_token().await?))
            .await?;
        Ok(result)
    }
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
    if ClothingCategory::exists_by_name(pool, category.store_id, &category.category_name, None)
        .await?
    {
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
    // check category_name is exists, 排除当前正在更新的记录
    if ClothingCategory::exists_by_name(
        pool,
        category.store_id,
        &category.category_name,
        category.category_id,
    )
    .await?
    {
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
) -> Result<bool> {
    let mut tx = state.pool.begin().await?;

    // 删除本地数据库中的记录
    ClothingCategory::delete_batch(&mut tx, &ids).await?;

    let store_id = utils::get_user_id(&state).await?; // check user is login
    let body = StoreIdWithIds { store_id, ids };
    if !ClothingCategory::delete_request(&state, body).await? {
        return Err(Error::bad_request("删除失败"));
    }

    tx.commit().await?;
    Ok(true)
}
