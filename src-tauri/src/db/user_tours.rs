use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Pool, Row, Sqlite};
use tauri::State;

use crate::error::Result;
use crate::state::AppState;
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

/// 更新用户的引导记录
#[tauri::command]
pub async fn update_tour_guide(state: State<'_, AppState>, page_key: String) -> Result<bool> {
    let mut user = match state.get_user_info().await {
        Some(user) => user,
        None => return Ok(false),
    };

    let user_id = match user.id {
        Some(id) => id,
        None => return Ok(false),
    };

    // 获取用户的引导记录
    let mut user_tours = UserTours::get_by_user_id(&state.pool, user_id).await?;

    // 检查页面是否已存在，如果不存在则添加
    if !user_tours
        .iter()
        .any(|v| v.page_key.as_deref() == Some(&page_key))
    {
        // 创建新记录
        let new_tour = UserTours::new(user_id, page_key);
        new_tour.insert(&state.pool).await?;
        user_tours.push(new_tour);
        user.completed_tours = Some(user_tours);
    }

    state.update_user_info(user).await;

    Ok(true)
}

/// 检查用户是否已完成特定页面的引导
#[tauri::command]
pub async fn check_tour_completed(state: State<'_, AppState>, page_key: String) -> Result<bool> {
    let user = match state.get_user_info().await {
        Some(user) => user,
        None => return Ok(false),
    };

    let user_id = match user.id {
        Some(id) => id,
        None => return Ok(false),
    };

    UserTours::check(&state.pool, user_id, &page_key).await
}
