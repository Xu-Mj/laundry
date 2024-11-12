use sqlx::types::chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use crate::error::Result;

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct ClothSequence {
    id: i64,
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
    pub async fn get_latest_sequence(pool: &Pool<Sqlite>, date: &str) -> Result<Option<Self>> {
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
