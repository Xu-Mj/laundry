use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{
    FromRow, Pool, QueryBuilder, Row, Sqlite, Transaction,
    types::chrono::{DateTime, FixedOffset},
};
use tauri::State;

use crate::db::Validator;
use crate::db::coupons::Coupon;
use crate::error::{Error, ErrorKind, Result};
use crate::state::AppState;
use crate::utils;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct UserCoupon {
    pub uc_id: Option<i64>,
    pub user_id: Option<i64>,
    pub coupon_id: Option<i64>,
    pub create_time: Option<DateTime<FixedOffset>>,
    pub obtain_at: Option<DateTime<FixedOffset>>,
    pub available_value: Option<f64>,
    pub uc_count: Option<i32>,
    pub pay_id: Option<i64>,
    pub uc_type: Option<String>,
    pub status: Option<String>,
    pub remark: Option<String>,
    pub coupon: Option<Coupon>,
    pub store_id: Option<i64>,
}

impl FromRow<'_, SqliteRow> for UserCoupon {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        let coupon = Coupon::from_row(row)?;
        Ok(Self {
            uc_id: row.try_get("uc_id").unwrap_or_default(),
            user_id: row.try_get("user_id").unwrap_or_default(),
            coupon_id: row.try_get("coupon_id").unwrap_or_default(),
            obtain_at: row.try_get("obtain_at").unwrap_or_default(),
            available_value: row.try_get("available_value").unwrap_or_default(),
            uc_count: row.try_get("uc_count").unwrap_or_default(),
            pay_id: row.try_get("pay_id").unwrap_or_default(),
            uc_type: row.try_get("uc_type").unwrap_or_default(),
            create_time: row.try_get("create_time").unwrap_or_default(),
            status: row.try_get("status").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
            store_id: row.try_get("store_id").unwrap_or_default(),
            coupon: Some(coupon),
        })
    }
}

impl Validator for UserCoupon {
    fn validate(&self) -> Result<()> {
        if self.store_id.is_none() {
            return Err(Error::unauthorized());
        }

        if self.user_id.is_none() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "user_id is required",
            ));
        }

        if self.coupon_id.is_none() {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "coupon_id is required",
            ));
        }

        Ok(())
    }
}

const SQL: &str =
    "SELECT uc.*, c.* FROM user_coupons uc left join coupons c on uc.coupon_id = c.coupon_id ";

impl UserCoupon {
    // Create operation
    pub async fn create(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<UserCoupon> {
        let result = sqlx::query_as(
            r#"
            INSERT INTO user_coupons (
                store_id, user_id, coupon_id, create_time, obtain_at, available_value,
                uc_count, pay_id, uc_type, status, remark
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(self.store_id)
        .bind(self.user_id)
        .bind(self.coupon_id)
        .bind(self.create_time)
        .bind(self.obtain_at)
        .bind(self.available_value)
        .bind(self.uc_count)
        .bind(self.pay_id)
        .bind(&self.uc_type)
        .bind(&self.status)
        .bind(&self.remark)
        .fetch_one(&mut **tr)
        .await?;

        Ok(result)
    }

    pub async fn find_by_user_id(
        pool: &Pool<Sqlite>,
        store_id: i64,
        user_id: i64,
    ) -> Result<Vec<UserCoupon>> {
        let result = sqlx::query_as(&format!("{SQL} WHERE uc.store_id = ? AND uc.user_id = ?"))
            .bind(store_id)
            .bind(user_id)
            .fetch_all(pool)
            .await?;

        Ok(result)
    }

    pub async fn find_valid_time_by_user_id(
        pool: &Pool<Sqlite>,
        store_id: i64,
        user_id: i64,
    ) -> Result<Vec<Self>> {
        let result = sqlx::query_as(&format!(
            "{SQL} WHERE uc.store_id = ?  AND uc.user_id = ? 
                  AND ? BETWEEN c.valid_from AND c.valid_to
                  AND uc.available_value > 0
                  AND uc.uc_count > 0;"
        ))
        .bind(store_id)
        .bind(user_id)
        .bind(utils::get_now())
        .fetch_all(pool)
        .await?;
        Ok(result)
    }

    pub async fn exists_by_coupon_id(
        pool: &Pool<Sqlite>,
        store_id: i64,
        coupon_id: i64,
    ) -> Result<bool> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT 1 FROM user_coupons WHERE store_id = ? AND coupon_id = ? LIMIT 1",
        )
        .bind(store_id)
        .bind(coupon_id)
        .fetch_optional(pool)
        .await?;

        Ok(result.is_some())
    }

    // Update operation
    pub async fn update(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<bool> {
        let result = sqlx::query(
            r#"
            UPDATE user_coupons
            SET
                user_id = ?,
                coupon_id = ?,
                create_time = ?,
                obtain_at = ?,
                available_value = ?,
                uc_count = ?,
                pay_id = ?,
                uc_type = ?,
                status = ?,
                remark = ?
            WHERE store_id = ? AND uc_id = ?
            "#,
        )
        .bind(self.user_id)
        .bind(self.coupon_id)
        .bind(self.create_time)
        .bind(self.obtain_at)
        .bind(self.available_value)
        .bind(self.uc_count)
        .bind(self.pay_id)
        .bind(&self.uc_type)
        .bind(&self.status)
        .bind(&self.remark)
        .bind(self.store_id)
        .bind(self.uc_id)
        .execute(&mut **tr)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn update_pay_id(
        tr: &mut Transaction<'_, Sqlite>,
        store_id: i64,
        uc_ids: &[i64],
        pay_id: &str,
    ) -> Result<()> {
        let mut builder = QueryBuilder::new("UPDATE user_coupons SET pay_id = ");
        builder.push_bind(pay_id);
        builder.push(" WHERE store_id = ");
        builder.push_bind(store_id);
        builder.push(" AND  pay_id IN (");

        uc_ids.iter().enumerate().for_each(|(i, id)| {
            if i > 0 {
                builder.push(",");
            }
            builder.push_bind(id);
        });
        builder.push(")");

        builder.build().execute(&mut **tr).await?;

        Ok(())
    }

    // Delete operation
    #[allow(dead_code)]
    pub async fn delete_by_user_ids(
        tx: &mut Transaction<'_, Sqlite>,
        user_ids: &[i64],
    ) -> Result<bool> {
        let mut builder = QueryBuilder::new("DELETE FROM user_coupons WHERE user_id IN (");
        user_ids.iter().enumerate().for_each(|(i, id)| {
            if i > 0 {
                builder.push(",");
            }
            builder.push_bind(id);
        });
        builder.push(")");
        let result = builder.build().execute(&mut **tx).await?;
        Ok(result.rows_affected() == user_ids.len() as u64)
    }

    // Get all user coupons
    #[allow(dead_code)]
    pub async fn find_all(pool: &Pool<Sqlite>) -> Result<Vec<UserCoupon>> {
        let result = sqlx::query_as(SQL).fetch_all(pool).await?;

        Ok(result)
    }

    pub async fn find_by_uc_ids(
        pool: &Pool<Sqlite>,
        store_id: i64,
        uc_ids: &[i64],
    ) -> Result<Vec<UserCoupon>> {
        let mut builder = QueryBuilder::new(&format!(
            "{SQL} WHERE uc.store_id = {store_id} AND uc.uc_id IN ("
        ));

        uc_ids.iter().enumerate().for_each(|(i, id)| {
            if i > 0 {
                builder.push(",");
            }
            builder.push_bind(id);
        });
        builder.push(")");

        let result = builder.build_query_as().fetch_all(pool).await?;

        Ok(result)
    }
}

#[tauri::command]
pub async fn get_user_coupons(state: State<'_, AppState>, user_id: i64) -> Result<Vec<UserCoupon>> {
    let store_id = utils::get_user_id(&state).await?;
    UserCoupon::find_by_user_id(&state.pool, store_id, user_id).await
}

#[tauri::command]
pub async fn get_user_coupons4sale(
    state: State<'_, AppState>,
    user_id: i64,
) -> Result<Vec<UserCoupon>> {
    let store_id = utils::get_user_id(&state).await?;
    UserCoupon::find_valid_time_by_user_id(&state.pool, store_id, user_id).await
}

#[tauri::command]
pub async fn get_user_coupon_by_user_id(
    state: State<'_, AppState>,
    id: i64,
) -> Result<Vec<UserCoupon>> {
    let store_id = utils::get_user_id(&state).await?;
    UserCoupon::find_by_user_id(&state.pool, store_id, id).await
}
