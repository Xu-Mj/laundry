use sqlx::{Pool, Sqlite};

pub mod printer;
pub mod user;

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
