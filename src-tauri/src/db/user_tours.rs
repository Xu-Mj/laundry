use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Pool, Row, Sqlite};

use crate::error::Result;
use crate::utils;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct UserTours {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub page_key: Option<String>,
    pub created_at: Option<i64>,
}

impl FromRow<'_, SqliteRow> for UserTours {
    fn from_row(row: &SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(UserTours {
            id: row.try_get("id").unwrap_or_default(),
            user_id: row.try_get("user_id").unwrap_or_default(),
            page_key: row.try_get("page_key").unwrap_or_default(),
            created_at: row.try_get("created_at").unwrap_or_default(),
        })
    }
}

impl UserTours {
    pub fn new(user_id: i64, page_key: String) -> Self {
        let now = utils::get_timestamp();
        Self {
            id: None,
            user_id: Some(user_id),
            page_key: Some(page_key),
            created_at: Some(now),
        }
    }

    // 根据用户ID获取引导记录
    pub async fn check(pool: &Pool<Sqlite>, user_id: i64, page_key: &str) -> Result<bool> {
        let tours: Option<Self> =
            sqlx::query_as("SELECT * FROM user_tours WHERE user_id = ? AND page_key = ?")
                .bind(user_id)
                .bind(page_key)
                .fetch_optional(pool)
                .await?;
        Ok(tours.is_some())
    }

    // 根据用户ID获取引导记录
    pub async fn get_by_user_id(pool: &Pool<Sqlite>, user_id: i64) -> Result<Vec<Self>> {
        let tours = sqlx::query_as("SELECT * FROM user_tours WHERE user_id = ?")
            .bind(user_id)
            .fetch_all(pool)
            .await?;
        Ok(tours)
    }

    // 插入新的引导记录
    pub async fn insert(&self, pool: &Pool<Sqlite>) -> Result<i64> {
        let now = utils::get_timestamp();
        let result = sqlx::query(
            "INSERT INTO user_tours (user_id, page_key, created_at) 
             VALUES (?, ?, ?) RETURNING *",
        )
        .bind(self.user_id)
        .bind(&self.page_key)
        .bind(self.created_at.unwrap_or(now))
        .fetch_one(pool)
        .await?;

        let id: i64 = result.get(0);
        Ok(id)
    }
}
