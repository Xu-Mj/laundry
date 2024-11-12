pub(crate) mod printer;

pub(crate) mod user;
pub(crate) mod tags;
pub(crate) mod clothing;
pub(crate) mod drying_rack;
mod cloth_sequence;

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use crate::sql::DDL;
// SQLite 连接池
pub struct DbPool(Pool<Sqlite>);

impl DbPool {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self(pool)
    }
}

// 初始化数据库并创建表
pub async fn initialize_database(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    for sql in DDL {
        sqlx::query(sql)
            .execute(pool)
            .await?;
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
pub struct PageParams {
    #[serde(default = "default_page_size")]
    pub page_size: i64,

    #[serde(default = "default_page")]
    pub page: i64,
}

/// Common return Object
#[derive(Debug, Deserialize, Serialize)]
pub struct PageResult<T> {
    pub total: u64,
    pub rows: Vec<T>,
}
