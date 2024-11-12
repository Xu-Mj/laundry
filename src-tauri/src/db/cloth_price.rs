use crate::error::{Error, Result};
use crate::utils::chrono_serde::naive_date_time_serde;
use chrono::FixedOffset;
use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{FromRow, Pool, QueryBuilder, Sqlite};
use tauri::State;
use tracing::debug;

use super::{DbPool, PageParams, PageResult};

const EAST_ZONE: i32 = 8 * 60 * 60;
#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ClothPrice {
    #[serde(default)]
    pub price_id: Option<i64>,
    #[serde(default)]
    pub price_number: String,
    #[serde(default)]
    pub order_type: String,
    #[serde(default)]
    pub price_name: String,
    #[serde(default)]
    pub price_value: Option<f64>,
    #[serde(default)]
    pub price_discount: Option<f64>,
    #[serde(default)]
    pub order_num: i64,
    #[serde(default)]
    pub ref_num: i64,
    #[serde(default = "status_default")]
    pub status: String,
    #[serde(default = "status_default")]
    pub del_flag: String,
    #[serde(default)]
    pub remark: Option<String>,

    #[serde(with = "naive_date_time_serde")]
    #[serde(skip_deserializing)]
    pub created_at: Option<DateTime<FixedOffset>>, // Use sqlx's NaiveDateTime type
    #[serde(with = "naive_date_time_serde")]
    #[serde(skip_deserializing)]
    pub updated_at: Option<DateTime<FixedOffset>>, // Use sqlx's NaiveDateTime type
}

fn status_default() -> String {
    "0".to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClothPriceParam {
    #[serde(default)]
    price_id: Option<i64>,
    #[serde(default)]
    price_number: Option<String>,
    #[serde(default)]
    order_type: Option<String>,
    #[serde(default)]
    price_name: Option<String>,
}

impl ClothPrice {
    fn validate(&self) -> Result<()> {
        if self.order_type.is_empty() {
            return Err(Error::with_details(
                crate::error::ErrorKind::BadRequest,
                "order_type cannot be empty",
            ));
        }

        if self.price_name.is_empty() {
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

impl ClothPrice {
    /// 插入记录
    pub async fn add(self, pool: &Pool<Sqlite>) -> Result<Self> {
        let result = sqlx::query_as::<_, ClothPrice>(
            "INSERT INTO cloth_price (price_number, order_type, price_name, price_value, price_discount, order_num, ref_num, status, del_flag, remark, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
            .bind(&self.del_flag)
            .bind(&self.remark)
            .bind(self.created_at) // created_at
            .fetch_one(pool)
            .await?;
        Ok(result) // 返回插入的 `price_id`
    }

    /// 获取记录（根据 `price_id` 查询）
    pub async fn get(pool: &Pool<Sqlite>, price_id: i64) -> Result<Option<ClothPrice>> {
        let result =
            sqlx::query_as::<_, ClothPrice>("SELECT * FROM cloth_price WHERE price_id = ?")
                .bind(price_id)
                .fetch_optional(pool)
                .await?;

        Ok(result)
    }

    /// 更新记录
    pub async fn update(&self, pool: &Pool<Sqlite>) -> Result<u64> {
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
                updated_at = ?
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
            .bind(self.updated_at) // updated_at
            .bind(self.price_id.unwrap())
            .execute(pool)
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

    /// 删除记录
    pub async fn delete_prices(pool: &Pool<Sqlite>, price_ids: &[i64]) -> Result<u64> {
        if price_ids.is_empty() {
            return Ok(0);
        }

        let mut query_builder = QueryBuilder::<Sqlite>::new(
            "UPDATE cloth_price SET del_flag = '2' WHERE price_id IN (",
        );

        for (i, clothing_id) in price_ids.iter().enumerate() {
            if i > 0 {
                query_builder.push(", ");
            }
            query_builder.push_bind(clothing_id);
        }

        query_builder.push(")");

        let query = query_builder.build();
        let result = query.execute(pool).await?;

        Ok(result.rows_affected())
    }

    /// 获取特定状态的记录
    pub async fn get_by_status(pool: &Pool<Sqlite>, status: &str) -> Result<Vec<ClothPrice>> {
        let result = sqlx::query_as::<_, ClothPrice>("SELECT * FROM cloth_price WHERE status = ?")
            .bind(status)
            .fetch_all(pool)
            .await?;

        Ok(result)
    }
}

impl ClothPriceParam {
    /// 获取所有记录
    pub async fn list_by_order_type(
        pool: &Pool<Sqlite>,
        order_type: String,
    ) -> Result<Vec<ClothPrice>> {
        let result =
            sqlx::query_as::<_, ClothPrice>("SELECT * FROM cloth_price where order_type = ?")
                .bind(&order_type)
                .fetch_all(pool)
                .await?;

        Ok(result)
    }

    fn apply_cloth_price_filters<'a>(&'a self, query_builder: &mut QueryBuilder<'a, Sqlite>) {
        if let Some(name) = &self.price_name {
            query_builder
                .push(" AND price_name LIKE ")
                .push_bind(format!("%{}%", name));
        }

        if let Some(number) = &self.price_number {
            query_builder
                .push(" AND price_number LIKE ")
                .push_bind(format!("%{}%", number));
        }

        if let Some(order_type) = &self.order_type {
            query_builder
                .push(" AND order_type = ")
                .push_bind(format!("{}", order_type));
        }
    }

    async fn count_clothing(&self, pool: &Pool<Sqlite>) -> Result<u64> {
        let mut query_builder =
            QueryBuilder::<Sqlite>::new("SELECT COUNT(*) FROM cloth_price WHERE del_flag = '0'");
        self.apply_cloth_price_filters(&mut query_builder);
        let query = query_builder.build_query_scalar::<u64>();
        Ok(query.fetch_one(pool).await?)
    }

    pub async fn list_pagination(
        &self,
        pool: &Pool<Sqlite>,
        page_params: PageParams,
    ) -> Result<PageResult<ClothPrice>> {
        let mut query_builder =
            QueryBuilder::<Sqlite>::new("SELECT * FROM cloth_price WHERE del_flag = '0'");
        self.apply_cloth_price_filters(&mut query_builder);
        query_builder.push(" ORDER BY ref_num DESC, order_num ASC ");

        query_builder
            .push(" LIMIT ")
            .push_bind(page_params.page_size);
        query_builder
            .push(" OFFSET ")
            .push_bind((page_params.page - 1) * page_params.page_size);

        let query = query_builder.build_query_as::<ClothPrice>();
        let rows = query.fetch_all(pool).await?;
        let total = self.count_clothing(pool).await?;
        Ok(PageResult { total, rows })
    }
}

fn generate_random_number() -> i32 {
    // 使用随机种子初始化 StdRng
    let seed = rand::random::<[u8; 32]>();
    let mut rng = StdRng::from_seed(seed);

    // 生成一个六位随机数
    let range = Uniform::from(100000..=999999);
    let random_number: i32 = rng.sample(&range);

    random_number
}

#[tauri::command]
pub async fn add_cloth_price(
    state: State<'_, DbPool>,
    mut cloth_price: ClothPrice,
) -> Result<ClothPrice> {
    cloth_price.validate()?;

    // gen number
    cloth_price.price_number = format!("P-{}", generate_random_number());

    // set create time
    cloth_price.created_at = Some(Utc::now().with_timezone(&FixedOffset::east_opt(EAST_ZONE).unwrap()));

    debug!("add cloth_price: {:?}", cloth_price);
    Ok(cloth_price.add(&state.0).await?)
}

#[tauri::command]
pub async fn get_cloth_price(
    state: State<'_, DbPool>,
    price_id: i64,
) -> Result<Option<ClothPrice>> {
    ClothPrice::get(&state.0, price_id).await
}

#[tauri::command]
pub async fn update_cloth_price(
    state: State<'_, DbPool>,
    mut cloth_price: ClothPrice,
) -> Result<u64> {
    if cloth_price.price_id.is_none() || cloth_price.price_id == Some(0) {
        return Err(Error::with_details(
            crate::error::ErrorKind::BadRequest,
            "price_id cannot be 0",
        ));
    }

    cloth_price.validate()?;

    cloth_price.updated_at = Some(Utc::now().with_timezone(&FixedOffset::east_opt(EAST_ZONE).unwrap()));

    cloth_price.update(&state.0).await
}

#[tauri::command]
pub async fn update_cloth_price_status(
    state: State<'_, DbPool>,
    price_id: i64,
    status: String,
) -> Result<u64> {
    if price_id == 0 {
        return Err(Error::with_details(
            crate::error::ErrorKind::BadRequest,
            "price_id cannot be 0",
        ));
    }

    let mut cloth_price = ClothPrice::get(&state.0, price_id).await?.unwrap();

    cloth_price.updated_at = Some(Utc::now().with_timezone(&FixedOffset::east_opt(EAST_ZONE).unwrap()));
    cloth_price.status = status;

    debug!("update cloth_price: {:?}", cloth_price);
    cloth_price.update(&state.0).await
}

#[tauri::command]
pub async fn update_cloth_price_ref_num(
    state: State<'_, DbPool>,
    ref_num: i64,
    cloth_price_ids: Vec<i64>,
) -> Result<()> {
    ClothPrice::update_ref_num(&state.0, ref_num, cloth_price_ids).await
}

#[tauri::command]
pub async fn delete_cloth_prices(state: State<'_, DbPool>, price_ids: Vec<i64>) -> Result<u64> {
    ClothPrice::delete_prices(&state.0, &price_ids).await
}

#[tauri::command]
pub async fn list_cloth_prices_by_order_type(
    state: State<'_, DbPool>,
    order_type: String,
) -> Result<Vec<ClothPrice>> {
    ClothPriceParam::list_by_order_type(&state.0, order_type).await
}

#[tauri::command]
pub async fn list_cloth_prices_pagination(
    state: State<'_, DbPool>,
    page_params: PageParams,
    cloth_price: ClothPriceParam,
) -> Result<PageResult<ClothPrice>> {
    cloth_price.list_pagination(&state.0, page_params).await
}
