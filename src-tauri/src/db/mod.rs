pub(crate) mod adjust_price;
pub(crate) mod cloth_price;
pub(crate) mod cloth_sequence;
pub(crate) mod clothing;
pub(crate) mod configs;
pub(crate) mod coupon_orders;
pub(crate) mod coupons;
pub(crate) mod dict_data;
pub(crate) mod dict_type;
pub(crate) mod drying_rack;
pub(crate) mod expenditure;
pub(crate) mod local_users;
pub(crate) mod membership_level;
pub(crate) mod menu;
pub(crate) mod notice_temp;
pub(crate) mod order_clothes;
pub(crate) mod order_pictures;
pub(crate) mod orders;
pub(crate) mod payments;
pub(crate) mod printer;
pub(crate) mod tags;
pub(crate) mod user;
pub(crate) mod user_coupons;
pub(crate) mod user_membership_level;
pub(crate) mod user_tags;

use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Pool, QueryBuilder, Sqlite, Transaction};
use std::collections::HashMap;

use crate::error::Result;
use crate::sql::DDL;

// SQLite 连接池
#[derive(Clone, Debug)]
pub struct AppState(pub Pool<Sqlite>);

impl AppState {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self(pool)
    }
}

// 初始化数据库并创建表
pub async fn initialize_database(pool: &Pool<Sqlite>) -> Result<()> {
    for sql in DDL {
        sqlx::query(sql).execute(pool).await?;
    }
    Ok(())
}

fn default_page_size() -> i64 {
    10
}

fn default_page() -> i64 {
    1
}

/// Page parameters
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PageParams {
    pub page_size: i64,

    pub page: i64,

    pub params: Option<HashMap<String, String>>,
}

impl Default for PageParams {
    fn default() -> Self {
        PageParams {
            page_size: default_page_size(),
            page: default_page(),
            params: None,
        }
    }
}

/// Common return Object
#[derive(Debug, Deserialize, Serialize)]
pub struct PageResult<T> {
    pub total: u64,
    pub rows: Vec<T>,
}

pub trait Validator {
    fn validate(&self) -> Result<()>;
}

#[async_trait::async_trait]
pub trait Curd: Sized + for<'f> FromRow<'f, SqliteRow> + Send + Unpin {
    const COUNT_SQL: &'static str;
    const QUERY_SQL: &'static str;
    const BY_ID_SQL: &'static str;
    const DELETE_BATCH_SQL: &'static str;
    const ORDER_SQL: Option<&'static str> = None;

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, Sqlite>);

    async fn get_list(
        &self,
        pool: &Pool<Sqlite>,
        page_params: PageParams,
    ) -> Result<PageResult<Self>> {
        let rows = self.build_query_builder(pool, Some(page_params)).await?;
        let total = self.count(pool).await?;
        Ok(PageResult { total, rows })
    }

    async fn get_all(&self, pool: &Pool<Sqlite>) -> Result<Vec<Self>> {
        self.build_query_builder(pool, None).await
    }

    async fn count(&self, pool: &Pool<Sqlite>) -> Result<u64> {
        let mut query_builder = QueryBuilder::new(Self::COUNT_SQL);
        self.apply_filters(&mut query_builder);
        let count = query_builder
            .build_query_scalar::<u64>()
            .fetch_one(pool)
            .await?;
        Ok(count)
    }

    async fn build_query_builder(
        &self,
        pool: &Pool<Sqlite>,
        page_params: Option<PageParams>,
    ) -> Result<Vec<Self>> {
        let mut builder = QueryBuilder::new(Self::QUERY_SQL);
        self.apply_filters(&mut builder);

        // build request params
        if let Some(page_params) = &page_params {
            if let Some(param) = &page_params.params {
                if let Some(begin_time) = param.get("beginTime") {
                    builder.push(" AND strftime('%Y%m%d', create_time) >= strftime('%Y%m%d', ");
                    builder.push_bind(begin_time);
                    builder.push(") ");
                }
            }
        }

        if let Some(page_params) = &page_params {
            if let Some(param) = &page_params.params {
                if let Some(end_time) = param.get("endTime") {
                    builder.push(" AND strftime('%Y%m%d', create_time) <= strftime('%Y%m%d', ");
                    builder.push_bind(end_time);
                    builder.push(") ");
                }
            }
        }
        // add order
        if let Some(sql) = Self::ORDER_SQL {
            builder.push(sql);
        }

        // pagination
        if let Some(page_params) = &page_params {
            builder.push(" LIMIT ").push_bind(page_params.page_size);

            builder
                .push(" OFFSET ")
                .push_bind((page_params.page - 1) * page_params.page_size);
        }

        let result = builder.build_query_as().fetch_all(pool).await?;
        Ok(result)
    }

    async fn get_by_id(pool: &Pool<Sqlite>, id: i64) -> Result<Option<Self>> {
        let result = sqlx::query_as(Self::BY_ID_SQL)
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(result)
    }

    async fn delete_batch(tx: &mut Transaction<'_, Sqlite>, ids: &[i64]) -> Result<bool> {
        if ids.is_empty() {
            return Ok(true);
        }

        let mut builder = QueryBuilder::new(Self::DELETE_BATCH_SQL);

        ids.iter().enumerate().for_each(|(i, id)| {
            if i > 0 {
                builder.push(", ");
            }
            builder.push_bind(id);
        });

        builder.push(")");

        let result = builder.build().execute(&mut **tx).await?;

        Ok(result.rows_affected() > 0)
    }
}
