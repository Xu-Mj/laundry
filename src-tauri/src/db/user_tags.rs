use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row, Sqlite, Transaction};

use crate::error::Result;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct UserTags {
    pub user_id: Option<i64>,
    pub tags: Option<String>,
    pub remark: Option<String>,
}

impl FromRow<'_, SqliteRow> for UserTags {
    fn from_row(row: &SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(UserTags {
            user_id: row.try_get("user_id").unwrap_or_default(),
            tags: row.try_get("tags").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
        })
    }
}

impl UserTags {
    pub fn new(user_id: i64, tags: String, remark: Option<String>) -> Self {
        Self {
            user_id: Some(user_id),
            tags: Some(tags),
            remark,
        }
    }
}

impl UserTags {
    #[allow(dead_code)]
    pub async fn get_by_user_id(
        tr: &mut Transaction<'_, Sqlite>,
        id: i64,
    ) -> Result<Option<Self>> {
        let mut query = sqlx::QueryBuilder::new("SELECT * FROM user_tags WHERE user_id = ");
        query.push_bind(id);

        let tag = query.build_query_as().fetch_optional(&mut **tr).await?;
        Ok(tag)
    }

    pub async fn insert(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<()> {
        sqlx::query("INSERT INTO user_tags (user_id, tags, remark) VALUES (?, ?, ?)")
            .bind(self.user_id)
            .bind(&self.tags)
            .bind(&self.remark)
            .execute(&mut **tr)
            .await?;
        Ok(())
    }

    pub async fn update(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<()> {
        sqlx::query(
            "INSERT INTO user_tags (user_id, tags, remark)
                        VALUES (?, ?, ?)
                        ON CONFLICT(user_id) DO UPDATE SET
                        tags = excluded.tags,
                        remark = excluded.remark",
        )
        .bind(self.user_id)
        .bind(&self.tags)
        .bind(&self.remark)
        .execute(&mut **tr)
        .await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn delete_by_user_id(
        tr: &mut Transaction<'_, Sqlite>,
        user_id: i64,
    ) -> Result<()> {
        let mut query = sqlx::QueryBuilder::new("DELETE FROM user_tags WHERE user_id = ");
        query.push_bind(user_id);

        query.build().execute(&mut **tr).await?;
        Ok(())
    }
}
