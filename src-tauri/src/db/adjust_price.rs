use crate::error::Result;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow, Row, Sqlite, Transaction};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OrderClothAdjust {
    pub adjust_id: Option<i64>,
    pub order_id: Option<i64>,
    pub adjust_value_add: Option<f64>,
    pub adjust_value_sub: Option<f64>,
    pub adjust_total: Option<f64>,
    pub remark: Option<String>,
}

impl FromRow<'_, SqliteRow> for OrderClothAdjust {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, Error> {
        Ok(Self {
            adjust_id: row.try_get("adjust_id").unwrap_or_default(),
            order_id: row.try_get("order_id").unwrap_or_default(),
            adjust_value_add: row.try_get("adjust_value_add").unwrap_or_default(),
            adjust_value_sub: row.try_get("adjust_value_sub").unwrap_or_default(),
            adjust_total: row.try_get("adjust_total").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
        })
    }
}

impl OrderClothAdjust {
    pub async fn create(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<Self> {
        let result = sqlx::query_as(
            "INSERT INTO order_clothes_adjust (order_id, adjust_value_add, adjust_value_sub, adjust_total, remark)
             VALUES (?, ?, ?, ?, ?) RETURNING *",
        )
            .bind(self.order_id)
            .bind(self.adjust_value_add)
            .bind(self.adjust_value_sub)
            .bind(self.adjust_total)
            .bind(&self.remark)
            .fetch_one(&mut **tr)
            .await?;

        Ok(result)
    }
    pub async fn upsert(&self, tx: &mut Transaction<'_, Sqlite>) -> Result<bool> {
        let result = sqlx::query(
            "INSERT INTO order_clothes_adjust (order_id, adjust_value_add, adjust_value_sub, adjust_total, remark)
             VALUES (?, ?, ?, ?, ?)
             ON CONFLICT(order_id)
             DO UPDATE SET
                adjust_value_add = excluded.adjust_value_add,
                adjust_value_sub = excluded.adjust_value_sub,
                adjust_total = excluded.adjust_total,
                remark = excluded.remark",
        )
            .bind(self.order_id)
            .bind(self.adjust_value_add)
            .bind(self.adjust_value_sub)
            .bind(self.adjust_total)
            .bind(&self.remark)
            .execute(&mut **tx)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete(tx: &mut Transaction<'_, Sqlite>, order_id: i64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM order_clothes_adjust WHERE order_id = ?")
            .bind(order_id)
            .execute(&mut **tx)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    #[allow(dead_code)]
    pub async fn get_by_order_id(
        pool: &sqlx::SqlitePool,
        order_id: i64,
    ) -> Result<Option<OrderClothAdjust>> {
        let result = sqlx::query_as::<_, OrderClothAdjust>(
            "SELECT adjust_id, order_id, adjust_value_add, adjust_value_sub, adjust_total, remark
             FROM order_clothes_adjust WHERE order_id = ?",
        )
        .bind(order_id)
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;
    use tokio::test;

    // Test setup for creating a SQLite test database
    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();

        // Create the `order_clothes_adjust` table in memory
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS order_clothes_adjust (
                adjust_id INTEGER PRIMARY KEY AUTOINCREMENT,
                order_id INTEGER UNIQUE,
                adjust_value_add FLOAT,
                adjust_value_sub FLOAT,
                adjust_total FLOAT,
                remark VARCHAR(256)
            );
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    #[test]
    async fn test_create_order_clothes_adjust() {
        let pool = setup_test_db().await;

        let adjust = OrderClothAdjust {
            adjust_id: None,
            order_id: Some(1),
            adjust_value_add: Some(10.0),
            adjust_value_sub: Some(5.0),
            adjust_total: Some(5.0),
            remark: Some("Test remark".to_string()),
        };

        let mut tr = pool.begin().await.unwrap();

        let result = adjust.create(&mut tr).await.unwrap();

        assert_eq!(result.order_id, Some(1));
        assert_eq!(result.adjust_value_add, Some(10.0));
        assert_eq!(result.adjust_value_sub, Some(5.0));
        assert_eq!(result.adjust_total, Some(5.0));
        assert_eq!(result.remark, Some("Test remark".to_string()));
    }

    #[test]
    async fn test_upsert_order_clothes_adjust() {
        let pool = setup_test_db().await;
        let mut tx = pool.begin().await.unwrap();

        // First insert
        let adjust = OrderClothAdjust {
            adjust_id: None,
            order_id: Some(1),
            adjust_value_add: Some(10.0),
            adjust_value_sub: Some(5.0),
            adjust_total: Some(5.0),
            remark: Some("First insert".to_string()),
        };

        adjust.upsert(&mut tx).await.unwrap();

        // Now, update with new values
        let updated_adjust = OrderClothAdjust {
            adjust_id: None,
            order_id: Some(1),
            adjust_value_add: Some(20.0),
            adjust_value_sub: Some(10.0),
            adjust_total: Some(10.0),
            remark: Some("Updated remark".to_string()),
        };

        updated_adjust.upsert(&mut tx).await.unwrap();

        tx.commit().await.unwrap();
        // Verify the update
        let result = OrderClothAdjust::get_by_order_id(&pool, 1).await.unwrap();

        assert_eq!(result.as_ref().unwrap().adjust_value_add, Some(20.0));
        assert_eq!(result.as_ref().unwrap().adjust_value_sub, Some(10.0));
        assert_eq!(result.as_ref().unwrap().adjust_total, Some(10.0));
        assert_eq!(result.unwrap().remark, Some("Updated remark".to_string()));
    }

    #[test]
    async fn test_delete_order_clothes_adjust() {
        let pool = setup_test_db().await;

        let adjust = OrderClothAdjust {
            adjust_id: None,
            order_id: Some(1),
            adjust_value_add: Some(10.0),
            adjust_value_sub: Some(5.0),
            adjust_total: Some(5.0),
            remark: Some("To be deleted".to_string()),
        };

        let mut tr = pool.begin().await.unwrap();
        adjust.create(&mut tr).await.unwrap();

        let delete_result = OrderClothAdjust::delete(&mut tr, 1).await.unwrap();
        assert!(delete_result); // One row should be deleted

        let result = OrderClothAdjust::get_by_order_id(&pool, 1).await.unwrap();
        assert!(result.is_none()); // After delete, the record should not exist
        tr.commit().await.unwrap();
    }

    #[test]
    async fn test_get_by_order_id() {
        let pool = setup_test_db().await;

        let adjust = OrderClothAdjust {
            adjust_id: None,
            order_id: Some(1),
            adjust_value_add: Some(10.0),
            adjust_value_sub: Some(5.0),
            adjust_total: Some(5.0),
            remark: Some("Test remark".to_string()),
        };

        let mut tr = pool.begin().await.unwrap();
        adjust.create(&mut tr).await.unwrap();

        let result = OrderClothAdjust::get_by_order_id(&pool, 1).await.unwrap();

        assert_eq!(result.as_ref().unwrap().order_id, Some(1));
        assert_eq!(result.as_ref().unwrap().adjust_value_add, Some(10.0));
        assert_eq!(result.as_ref().unwrap().adjust_value_sub, Some(5.0));
        assert_eq!(result.as_ref().unwrap().adjust_total, Some(5.0));
        assert_eq!(result.unwrap().remark, Some("Test remark".to_string()));
    }
}
