use sqlx::types::chrono::{DateTime, FixedOffset};
use sqlx::{Sqlite, Transaction};

use crate::error::Result;
use crate::utils;

#[derive(Debug, Clone, Default, sqlx::FromRow)]
pub struct CouponOrder {
    pub order_id: i32,
    pub uc_ids: String,
    pub create_time: DateTime<FixedOffset>,
    pub store_id: Option<i64>,
}

impl CouponOrder {
    pub fn new_with_now(store_id: i64, uc_ids: String) -> Self {
        Self {
            order_id: 0,
            uc_ids,
            create_time: utils::get_now(),
            store_id: Some(store_id),
        }
    }
}

impl CouponOrder {
    pub async fn create(&mut self, tr: &mut Transaction<'_, Sqlite>) -> Result<bool> {
        let result = sqlx::query(
            "INSERT INTO coupon_orders(uc_id, create_time, store_id) VALUES (?, ?, ?) RETURNING *",
        )
        .bind(&self.uc_ids)
        .bind(&self.create_time)
        .bind(self.store_id)
        .execute(&mut **tr)
        .await?;
        self.order_id = result.last_insert_rowid() as i32;
        Ok(result.rows_affected() > 0)
    }
}
