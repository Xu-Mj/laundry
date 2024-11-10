use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

pub mod printer;
pub mod user;
pub mod tags;

// SQLite 连接池
pub struct DbPool(Pool<Sqlite>);

impl DbPool {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self(pool)
    }
}

// 初始化数据库并创建表
pub async fn initialize_database(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        )",
    )
        .execute(pool)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS printers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            system_name TEXT NOT NULL,
            driver_name TEXT NOT NULL
        )",
    )
        .execute(pool)
        .await?;
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
