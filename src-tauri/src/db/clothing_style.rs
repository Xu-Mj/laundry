use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Pool, QueryBuilder, Row, Sqlite, Transaction};
use tauri::State;

use crate::error::{Error, ErrorKind, Result};
use crate::state::AppState;
use crate::utils;
use crate::utils::request::{Request, StoreIdWithIds};

use super::{Curd, PageParams, PageResult, Validator};

/// 衣物分类表
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct ClothingStyle {
    /// 分类ID
    pub style_id: Option<i64>,
    /// 商家ID，用于数据隔离
    pub store_id: i64,
    /// 品类ID，表示该分类属于哪个品类
    pub category_id: i64,
    /// 分类编码，如000上衣，001鞋，002裤子等
    pub style_code: String,
    /// 分类名称
    pub style_name: String,
    /// 显示顺序
    pub order_num: Option<i64>,
    /// 备注
    pub remark: Option<String>,
    /// 删除标志（0代表存在 2代表删除）
    pub del_flag: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl FromRow<'_, SqliteRow> for ClothingStyle {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            style_id: row.try_get("style_id").unwrap_or_default(),
            store_id: row.try_get("store_id").unwrap_or_default(),
            category_id: row.try_get("category_id").unwrap_or_default(),
            style_code: row.try_get("style_code").unwrap_or_default(),
            style_name: row.try_get("style_name").unwrap_or_default(),
            order_num: row.try_get("order_num").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
            del_flag: row.try_get("del_flag").unwrap_or_default(),
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
        })
    }
}

impl Validator for ClothingStyle {
    fn validate(&self) -> Result<()> {
        if self.store_id <= 0 {
            return Err(Error::bad_request("必须指定所属商家"));
        }

        if self.style_name.trim().is_empty() {
            return Err(Error::bad_request("分类名称不能为空"));
        }

        if self.category_id <= 0 {
            return Err(Error::bad_request("必须指定所属品类"));
        }

        Ok(())
    }
}

impl Curd for ClothingStyle {
    const COUNT_SQL: &'static str = "SELECT COUNT(*) FROM clothing_styles WHERE del_flag = '0' ";
    const QUERY_SQL: &'static str = "SELECT * FROM clothing_styles WHERE del_flag = '0' ";
    const BY_ID_SQL: &'static str =
        "SELECT * FROM clothing_styles WHERE style_id = ? AND del_flag = '0' ";
    const DELETE_BATCH_SQL: &'static str =
        "UPDATE clothing_styles SET del_flag = '2' WHERE style_id IN (";
    const ORDER_SQL: Option<&'static str> = Some("ORDER BY order_num ASC");

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, Sqlite>) {
        if self.store_id >= 0 {
            builder.push(" AND store_id = ").push_bind(self.store_id);
        }

        if self.category_id > 0 {
            builder
                .push(" AND category_id = ")
                .push_bind(self.category_id);
        }

        if let Some(style_id) = &self.style_id {
            builder.push(" AND style_id = ").push_bind(style_id);
        }

        if !self.style_name.is_empty() {
            builder
                .push(" AND style_name LIKE ")
                .push_bind(format!("%{}%", self.style_name));
        }

        if !self.style_code.is_empty() {
            builder
                .push(" AND style_code = ")
                .push_bind(&self.style_code);
        }

        if let Some(del_flag) = &self.del_flag {
            builder.push(" AND del_flag = ").push_bind(del_flag);
        }
    }
}

impl ClothingStyle {
    /// 创建新的衣物分类
    pub async fn insert(self, pool: &Pool<Sqlite>) -> Result<Self> {
        let now = utils::get_timestamp();
        let result = sqlx::query_as::<_, Self>(
            "INSERT INTO clothing_styles (style_id,store_id, category_id, style_code, style_name, order_num, remark, del_flag, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, '0', ?, ?)
             RETURNING *"
        )
            .bind(self.style_id)
            .bind(self.store_id)
            .bind(self.category_id)
            .bind(&self.style_code)
            .bind(&self.style_name)
            .bind(&self.order_num.unwrap_or(0))
            .bind(&self.remark)
            .bind(now)
            .bind(now)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    /// 更新衣物分类
    pub async fn update(self, pool: &Pool<Sqlite>) -> Result<bool> {
        // 验证style_id是否存在
        let style_id = match self.style_id {
            Some(id) => id,
            None => return Err(Error::with_details(ErrorKind::BadRequest, "缺少分类ID")),
        };

        // 使用单一SQL语句更新所有字段
        let now = utils::get_timestamp();
        let result = sqlx::query(
            "UPDATE clothing_styles SET 
             category_id =?,
             style_name = ?, 
             style_code = ?, 
             order_num = ?, 
             remark = ?, 
             updated_at = ? 
             WHERE store_id = ? AND style_id = ?",
        )
        .bind(&self.category_id)
        .bind(&self.style_name)
        .bind(&self.style_code)
        .bind(&self.order_num)
        .bind(&self.remark)
        .bind(now)
        .bind(self.store_id)
        .bind(style_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 检查分类名称是否已经存在
    ///
    /// 如果提供了style_id，则会排除该ID的记录，用于更新时检查
    pub async fn exists_by_name(
        pool: &Pool<Sqlite>,
        store_id: i64,
        category_id: i64,
        style_name: &str,
        exclude_style_id: Option<i64>,
    ) -> Result<bool> {
        let sql = match exclude_style_id {
            Some(_) => {
                "SELECT EXISTS(SELECT 1 FROM clothing_styles WHERE store_id = ? AND category_id = ? AND style_name = ? AND style_id != ? AND del_flag = '0')"
            }
            None => {
                "SELECT EXISTS(SELECT 1 FROM clothing_styles WHERE store_id = ? AND category_id = ? AND style_name = ? AND del_flag = '0')"
            }
        };

        let mut query = sqlx::query_scalar(sql)
            .bind(store_id)
            .bind(category_id)
            .bind(style_name);

        if let Some(id) = exclude_style_id {
            query = query.bind(id);
        }

        let result = query.fetch_one(pool).await?;

        Ok(result)
    }

    /// 获取指定品类下的所有衣物分类
    pub async fn get_by_category_id(pool: &Pool<Sqlite>, category_id: i64) -> Result<Vec<Self>> {
        let result = sqlx::query_as::<_, Self>(
            "SELECT * FROM clothing_styles WHERE category_id = ? AND del_flag = '0' ORDER BY order_num ASC"
        )
        .bind(category_id)
        .fetch_all(pool)
        .await?;

        Ok(result)
    }

    pub async fn insert_batch(
        tx: &mut Transaction<'_, Sqlite>,
        styles: &[ClothingStyle],
    ) -> Result<()> {
        if styles.is_empty() {
            return Ok(());
        }
        let now = utils::get_timestamp();
        let mut builder = QueryBuilder::new(
            "INSERT INTO clothing_styles (style_id, store_id, category_id, style_code, style_name, order_num, remark, del_flag, created_at, updated_at) ",
        );
        builder.push_values(styles.iter(), |mut b, style| {
            b.push_bind(style.style_id)
                .push_bind(style.store_id)
                .push_bind(style.category_id)
                .push_bind(&style.style_code)
                .push_bind(&style.style_name)
                .push_bind(style.order_num.unwrap_or(0))
                .push_bind(&style.remark)
                .push_bind("0")
                .push_bind(now)
                .push_bind(now);
        });
        let query = builder.build();
        query.execute(&mut **tx).await?;
        Ok(())
    }
}

impl Request for ClothingStyle {
    const URL: &'static str = "/clothing/style";
}

impl ClothingStyle {
    pub async fn create_batch_request(
        state: &AppState,
        categories: &[ClothingStyle],
    ) -> Result<Vec<ClothingStyle>> {
        let token = state.try_token().await?;
        if token.user.id == Some(0) {
            return Ok(Vec::new());
        }

        let result = state
            .http_client
            .post("/clothing/style/batch", categories, Some(&token.token))
            .await?;
        Ok(result)
    }
}

// Tauri命令接口
#[tauri::command]
pub async fn list_clothing_style_pagination(
    state: State<'_, AppState>,
    mut style: ClothingStyle,
    page_params: PageParams,
) -> Result<PageResult<ClothingStyle>> {
    let store_id = utils::get_user_id(&state).await?;
    style.store_id = store_id;
    style.get_list(&state.pool, page_params).await
}

#[tauri::command]
pub async fn list_clothing_style_all(
    state: State<'_, AppState>,
    mut style: ClothingStyle,
) -> Result<Vec<ClothingStyle>> {
    let store_id = utils::get_user_id(&state).await?;
    style.store_id = store_id;
    style.get_all(&state.pool).await
}

#[tauri::command]
pub async fn get_clothing_style_by_id(
    state: State<'_, AppState>,
    id: i64,
) -> Result<Option<ClothingStyle>> {
    let pool = &state.pool;
    ClothingStyle::get_by_id(pool, id).await
}

#[tauri::command]
pub async fn get_clothing_style_by_category_id(
    state: State<'_, AppState>,
    category_id: i64,
) -> Result<Vec<ClothingStyle>> {
    let pool = &state.pool;
    ClothingStyle::get_by_category_id(pool, category_id).await
}

#[tauri::command]
pub async fn add_clothing_style(
    state: State<'_, AppState>,
    mut style: ClothingStyle,
) -> Result<ClothingStyle> {
    let store_id = utils::get_user_id(&state).await?;
    style.store_id = store_id;
    let pool = &state.pool;
    style.validate()?;
    // check if style name exists
    if ClothingStyle::exists_by_name(
        pool,
        style.store_id,
        style.category_id,
        &style.style_name,
        None,
    )
    .await?
    {
        return Err(Error::bad_request("分类名称已存在"));
    }

    // create to server
    let style = style.create_request(&state).await?;

    style.insert(pool).await
}

#[tauri::command]
pub async fn update_clothing_style(
    state: State<'_, AppState>,
    style: ClothingStyle,
) -> Result<bool> {
    let pool = &state.pool;
    style.validate()?;
    // check if style name exists, 排除当前正在更新的记录
    if ClothingStyle::exists_by_name(
        pool,
        style.store_id,
        style.category_id,
        &style.style_name,
        style.style_id,
    )
    .await?
    {
        return Err(Error::bad_request("分类名称已存在"));
    }

    // update to server
    if !style.update_request(&state).await? {
        return Err(Error::bad_request("更新失败"));
    }

    style.update(pool).await
}

#[tauri::command]
pub async fn delete_clothing_style_batch(
    state: State<'_, AppState>,
    ids: Vec<i64>,
) -> Result<bool> {
    let mut tx = state.pool.begin().await?;

    // 删除本地数据库中的记录
    ClothingStyle::delete_batch(&mut tx, &ids).await?;

    let store_id = utils::get_user_id(&state).await?; // check user is login
    let body = StoreIdWithIds { store_id, ids };
    if !ClothingStyle::delete_request(&state, body).await? {
        return Err(Error::bad_request("删除失败"));
    }

    tx.commit().await?;
    Ok(true)
}
