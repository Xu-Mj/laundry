use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::types::chrono::{DateTime, FixedOffset};
use sqlx::{FromRow, Pool, QueryBuilder, Row, Sqlite, Transaction};
use tauri::State;
use tracing::debug;

use super::{AppState, Curd, PageParams, PageResult, Validator};
use crate::error::{Error, Result};
use crate::utils;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClothPrice {
    /// 主键id
    pub price_id: Option<i64>,
    /// 价格编号
    pub price_number: Option<String>,
    /// 类型
    pub order_type: Option<String>,
    /// 价格名称
    pub price_name: Option<String>,
    /// 价格
    pub price_value: Option<f64>,
    /// 折扣
    pub price_discount: Option<f64>,
    /// 显示顺序
    pub order_num: Option<i64>,
    /// 引用计数
    pub ref_num: Option<i64>,
    /// 状态
    pub status: Option<String>,
    /// 删除标记
    pub del_flag: Option<String>,
    /// 备注
    pub remark: Option<String>,

    /// 创建时间
    pub create_time: Option<DateTime<FixedOffset>>,
    /// 更新时间
    pub update_time: Option<DateTime<FixedOffset>>,
}

impl FromRow<'_, SqliteRow> for ClothPrice {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            price_id: row.try_get("price_id").unwrap_or_default(),
            price_number: row.try_get("price_number").unwrap_or_default(),
            order_type: row.try_get("order_type").unwrap_or_default(),
            price_name: row.try_get("price_name").unwrap_or_default(),
            price_value: row.try_get("price_value").unwrap_or_default(),
            price_discount: row.try_get("price_discount").unwrap_or_default(),
            order_num: row.try_get("order_num").unwrap_or_default(),
            ref_num: row.try_get("ref_num").unwrap_or_default(),
            status: row.try_get("status").unwrap_or_default(),
            del_flag: row.try_get("del_flag").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
            create_time: row.try_get("create_time").unwrap_or_default(),
            update_time: row.try_get("update_time").unwrap_or_default(),
        })
    }
}

impl Validator for ClothPrice {
    fn validate(&self) -> Result<()> {
        if self.order_type.is_none() || self.order_type.as_ref().unwrap().trim().is_empty() {
            return Err(Error::with_details(
                crate::error::ErrorKind::BadRequest,
                "order_type cannot be empty",
            ));
        }

        if self.price_name.is_none() || self.price_name.as_ref().unwrap().trim().is_empty() {
            return Err(Error::with_details(
                crate::error::ErrorKind::BadRequest,
                "price_name cannot be empty",
            ));
        }

        if self.price_value.is_none() && self.price_discount.is_none() {
            return Err(Error::with_details(
                crate::error::ErrorKind::BadRequest,
                "price_value or price_discount cannot be both None",
            ));
        }
        Ok(())
    }
}

impl Curd for ClothPrice {
    const COUNT_SQL: &'static str = "SELECT COUNT(*) FROM cloth_price WHERE del_flag = '0' ";
    const QUERY_SQL: &'static str = "SELECT * FROM cloth_price WHERE del_flag = '0' ";
    const BY_ID_SQL: &'static str = "SELECT * FROM cloth_price WHERE price_id = ?";
    const DELETE_BATCH_SQL: &'static str =
        "UPDATE cloth_price SET del_flag = '2' WHERE price_id IN (";
    const ORDER_SQL: Option<&'static str> = Some(" ORDER BY ref_num DESC, order_num ASC ");

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, Sqlite>) {
        if let Some(name) = &self.price_name {
            builder
                .push(" AND price_name LIKE ")
                .push_bind(format!("%{}%", name));
        }

        if let Some(number) = &self.price_number {
            builder
                .push(" AND price_number LIKE ")
                .push_bind(format!("%{}%", number));
        }

        if let Some(order_type) = &self.order_type {
            builder
                .push(" AND order_type = ")
                .push_bind(format!("{}", order_type));
        }
    }
}

impl ClothPrice {
    /// 插入记录
    pub async fn add(self, pool: &Pool<Sqlite>) -> Result<Self> {
        let result = sqlx::query_as::<_, ClothPrice>(
            "INSERT INTO cloth_price (price_number, order_type, price_name, price_value, price_discount, order_num, ref_num, status, remark, create_time, del_flag)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, '0')
            RETURNING *"
        )
            .bind(&self.price_number)
            .bind(&self.order_type)
            .bind(&self.price_name)
            .bind(self.price_value)
            .bind(self.price_discount)
            .bind(self.order_num)
            .bind(self.ref_num)
            .bind(&self.status)
            .bind(&self.remark)
            .bind(self.create_time) // create_time
            .fetch_one(pool)
            .await?;
        Ok(result) // 返回插入的 `price_id`
    }

    /// 更新记录
    pub async fn update(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<u64> {
        let result = sqlx::query(
            "UPDATE cloth_price SET
                price_number = ?,
                order_type = ?,
                price_name = ?,
                price_value = ?,
                price_discount = ?,
                order_num = ?,
                ref_num = ?,
                status = ?,
                remark = ?,
                update_time = ?
            WHERE price_id = ?",
        )
        .bind(&self.price_number)
        .bind(&self.order_type)
        .bind(&self.price_name)
        .bind(self.price_value)
        .bind(self.price_discount)
        .bind(self.order_num)
        .bind(self.ref_num)
        .bind(&self.status)
        .bind(&self.remark)
        .bind(self.update_time) // update_time
        .bind(self.price_id)
        .execute(&mut **tr)
        .await?;

        Ok(result.rows_affected())
    }

    // 增加ref_num
    pub async fn update_ref_num(
        pool: &Pool<Sqlite>,
        ref_num: i64,
        clothing_ids: Vec<i64>,
    ) -> Result<()> {
        let mut query_builder = QueryBuilder::<Sqlite>::new("UPDATE cloth_price SET ref_num = ");
        query_builder
            .push_bind(ref_num)
            .push(" WHERE price_id IN (");

        let mut separated = query_builder.separated(", ");
        for clothing_id in &clothing_ids {
            separated.push_bind(clothing_id);
        }
        query_builder.push(")");

        query_builder.build().execute(pool).await?;
        Ok(())
    }

    /// 获取特定状态的记录
    #[allow(dead_code)]
    pub async fn get_by_status(pool: &Pool<Sqlite>, status: &str) -> Result<Vec<ClothPrice>> {
        let result = sqlx::query_as::<_, ClothPrice>("SELECT * FROM cloth_price WHERE status = ?")
            .bind(status)
            .fetch_all(pool)
            .await?;

        Ok(result)
    }
    /// 获取所有记录
    pub async fn list_by_order_type(
        pool: &Pool<Sqlite>,
        order_type: String,
    ) -> Result<Vec<ClothPrice>> {
        let result = sqlx::query_as::<_, ClothPrice>(
            "SELECT * FROM cloth_price WHERE status = '0' AND del_flag = '0' AND order_type = ?",
        )
        .bind(&order_type)
        .fetch_all(pool)
        .await?;

        Ok(result)
    }
}

#[tauri::command]
pub async fn add_cloth_price(
    state: State<'_, AppState>,
    mut cloth_price: ClothPrice,
) -> Result<ClothPrice> {
    cloth_price.validate()?;

    // gen number
    cloth_price.price_number = Some(format!("P-{}", utils::gen_random_number()));

    // set create time
    cloth_price.create_time = Some(utils::get_now());

    debug!("add cloth_price: {:?}", cloth_price);
    Ok(cloth_price.add(&state.pool).await?)
}

#[tauri::command]
pub async fn get_cloth_price(
    state: State<'_, AppState>,
    price_id: i64,
) -> Result<Option<ClothPrice>> {
    ClothPrice::get_by_id(&state.pool, price_id).await
}

#[tauri::command]
pub async fn update_cloth_price(
    state: State<'_, AppState>,
    mut cloth_price: ClothPrice,
) -> Result<u64> {
    if cloth_price.price_id.is_none() || cloth_price.price_id == Some(0) {
        return Err(Error::with_details(
            crate::error::ErrorKind::BadRequest,
            "price_id cannot be 0",
        ));
    }

    cloth_price.validate()?;

    cloth_price.update_time = Some(utils::get_now());

    let mut tr = state.pool.begin().await?;
    let i = cloth_price.update(&mut tr).await?;
    tr.commit().await?;
    Ok(i)
}

#[tauri::command]
pub async fn update_cloth_price_status(
    state: State<'_, AppState>,
    price_id: i64,
    status: String,
) -> Result<u64> {
    if price_id == 0 {
        return Err(Error::with_details(
            crate::error::ErrorKind::BadRequest,
            "price_id cannot be 0",
        ));
    }

    let mut cloth_price = ClothPrice::get_by_id(&state.pool, price_id).await?.unwrap();

    cloth_price.update_time = Some(utils::get_now());
    cloth_price.status = Some(status);

    debug!("update cloth_price: {:?}", cloth_price);
    let mut tr = state.pool.begin().await?;
    let i = cloth_price.update(&mut tr).await?;
    tr.commit().await?;
    Ok(i)
}

#[tauri::command]
pub async fn update_cloth_price_ref_num(
    state: State<'_, AppState>,
    ref_num: i64,
    cloth_price_ids: Vec<i64>,
) -> Result<()> {
    ClothPrice::update_ref_num(&state.pool, ref_num, cloth_price_ids).await
}

#[tauri::command]
pub async fn delete_cloth_prices(state: State<'_, AppState>, price_ids: Vec<i64>) -> Result<bool> {
    let mut tr = state.pool.begin().await?;
    let result = ClothPrice::delete_batch(&mut tr, &price_ids).await?;
    tr.commit().await?;
    Ok(result)
}

#[tauri::command]
pub async fn list_cloth_prices_by_order_type(
    state: State<'_, AppState>,
    order_type: String,
) -> Result<Vec<ClothPrice>> {
    ClothPrice::list_by_order_type(&state.pool, order_type).await
}

#[tauri::command]
pub async fn list_cloth_prices_pagination(
    state: State<'_, AppState>,
    page_params: PageParams,
    cloth_price: ClothPrice,
) -> Result<PageResult<ClothPrice>> {
    cloth_price.get_list(&state.pool, page_params).await
}
