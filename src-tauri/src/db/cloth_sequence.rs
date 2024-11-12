use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, types::chrono::NaiveDate};

use crate::error::Result;
use crate::utils::chrono_serde::naive_date_serde;

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ClothSequence {
    id: i64,
    #[serde(with = "naive_date_serde")]
    date: NaiveDate,
    sequence_number: i64,
}


impl ClothSequence {
    /// 插入新记录
    pub async fn add(&mut self, pool: &Pool<Sqlite>) -> Result<()> {
        self.id = sqlx::query("INSERT INTO cloth_sequence (date, sequence_number) VALUES (?, ?)")
            .bind(self.date)
            .bind(self.sequence_number)
            .execute(pool)
            .await?
            .last_insert_rowid();

        Ok(())
    }

    /// 根据 ID 获取记录
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: i64) -> Result<Option<Self>> {
        let result = sqlx::query_as::<_, ClothSequence>("SELECT * FROM cloth_sequence WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    /// 更新记录
    pub async fn update(&self, pool: &Pool<Sqlite>) -> Result<u64> {
        let rows_affected = sqlx::query("UPDATE cloth_sequence SET date = ?, sequence_number = ? WHERE id = ?")
            .bind(self.date)
            .bind(self.sequence_number)
            .bind(self.id)
            .execute(pool)
            .await?
            .rows_affected();

        Ok(rows_affected)
    }

    /// 删除记录
    pub async fn delete(pool: &Pool<Sqlite>, id: i64) -> Result<u64> {
        let rows_affected = sqlx::query("DELETE FROM cloth_sequence WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected();

        Ok(rows_affected)
    }

    /// 获取最新的 sequence_number（用于当天或特定日期的最大序列号）
    pub async fn get_latest_sequence(pool: &Pool<Sqlite>) -> Result<Option<Self>> {
        let result = sqlx::query_as::<_, ClothSequence>(
            "SELECT id, date, sequence_number
             FROM cloth_sequence
             WHERE date = DATE('now')
             ORDER BY sequence_number DESC
             LIMIT 1"
        )
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{Pool, Sqlite, SqlitePool};
    use sqlx::types::chrono::NaiveDate;
    
    async fn setup_pool() -> Pool<Sqlite> {
        // Setup an in-memory SQLite database for testing
    let pool = SqlitePool::connect(":memory:").await.unwrap();

        // Create the cloth_sequence table for testing
        sqlx::query(
            r#"
            CREATE TABLE cloth_sequence (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date TEXT NOT NULL,
                sequence_number INTEGER NOT NULL
            );
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_add_cloth_sequence() {
        let pool = setup_pool().await;

        // Create a new ClothSequence instance
        let mut cloth_sequence = ClothSequence {
            id: 0,  // id will be set after insertion
            date: NaiveDate::from_ymd_opt(2024, 11, 12).unwrap(), // Test date
            sequence_number: 1, // Test sequence number
        };

        // Add the new record
        cloth_sequence.add(&pool).await.unwrap();

        // Check if the record has been inserted by getting it by ID
        let result = ClothSequence::get_by_id(&pool, cloth_sequence.id).await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().sequence_number, 1);
    }

    #[tokio::test]
    async fn test_get_by_id() {
        let pool = setup_pool().await;

        // Insert a record to be retrieved
        let mut cloth_sequence = ClothSequence {
            id: 0,
            date: NaiveDate::from_ymd_opt(2024, 11, 12).unwrap(),
            sequence_number: 2,
        };
        cloth_sequence.add(&pool).await.unwrap();

        // Retrieve the record by ID
        let result = ClothSequence::get_by_id(&pool, cloth_sequence.id).await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().sequence_number, 2);
    }

    #[tokio::test]
    async fn test_update_cloth_sequence() {
        let pool = setup_pool().await;

        // Insert a record to be updated
        let mut cloth_sequence = ClothSequence {
            id: 0,
            date: NaiveDate::from_ymd_opt(2024, 11, 12).unwrap(),
            sequence_number: 3,
        };
        cloth_sequence.add(&pool).await.unwrap();

        // Update the record
        cloth_sequence.sequence_number = 5;
        let rows_affected = cloth_sequence.update(&pool).await.unwrap();
        assert_eq!(rows_affected, 1);

        // Retrieve the updated record
        let updated_result = ClothSequence::get_by_id(&pool, cloth_sequence.id).await.unwrap();
        assert!(updated_result.is_some());
        assert_eq!(updated_result.unwrap().sequence_number, 5);
    }

    #[tokio::test]
    async fn test_delete_cloth_sequence() {
        let pool = setup_pool().await;

        // Insert a record to be deleted
        let mut cloth_sequence = ClothSequence {
            id: 0,
            date: NaiveDate::from_ymd_opt(2024, 11, 12).unwrap(),
            sequence_number: 4,
        };
        cloth_sequence.add(&pool).await.unwrap();

        // Delete the record
        let rows_affected = ClothSequence::delete(&pool, cloth_sequence.id).await.unwrap();
        assert_eq!(rows_affected, 1);

        // Try retrieving the deleted record
        let result = ClothSequence::get_by_id(&pool, cloth_sequence.id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_get_latest_sequence() {
        let pool = setup_pool().await;

        // Insert a few records
        let mut cloth_sequence_1 = ClothSequence {
            id: 0,
            date: NaiveDate::from_ymd_opt(2024, 11, 12).unwrap(),
            sequence_number: 10,
        };
        cloth_sequence_1.add(&pool).await.unwrap();

        let mut cloth_sequence_2 = ClothSequence {
            id: 0,
            date: NaiveDate::from_ymd_opt(2024, 11, 12).unwrap(),
            sequence_number: 20,
        };
        cloth_sequence_2.add(&pool).await.unwrap();

        // Get the latest sequence for the current date
        let result = ClothSequence::get_latest_sequence(&pool).await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().sequence_number, 20);
    }
}
