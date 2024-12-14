use crate::db::{AppState, Curd, PageParams, PageResult, Validator};
use crate::error::Result;
use crate::utils;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::types::chrono::{DateTime, FixedOffset};
use sqlx::{FromRow, QueryBuilder, Row, Sqlite};
use tauri::State;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Expenditure {
    pub exp_id: Option<i64>,                        // Primary key
    pub order_id: Option<i64>,                      // Nullable TEXT
    pub cloth_ids: Option<String>,                  // Nullable TEXT
    pub exp_title: Option<String>,                  // NOT NULL TEXT
    pub recv_account: Option<i64>,                  // Nullable INTEGER
    pub recv_account_title: Option<String>,         // Nullable TEXT
    pub exp_type: Option<String>,                   // NOT NULL TEXT
    pub exp_amount: i64,                            // NOT NULL INTEGER
    pub create_time: Option<DateTime<FixedOffset>>, // Nullable TIMESTAMP
    pub remark: Option<String>,                     // Nullable TEXT
}

impl FromRow<'_, SqliteRow> for Expenditure {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            exp_id: row.try_get("exp_id").unwrap_or_default(),
            order_id: row.try_get("order_id").unwrap_or_default(),
            cloth_ids: row.try_get("cloth_ids").unwrap_or_default(),
            exp_title: row.try_get("exp_title").unwrap_or_default(),
            recv_account: row.try_get("recv_account").unwrap_or_default(),
            recv_account_title: row.try_get("recv_account_title").unwrap_or_default(),
            exp_type: row.try_get("exp_type").unwrap_or_default(),
            exp_amount: row.try_get("exp_amount").unwrap_or_default(),
            create_time: row.try_get("create_time").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
        })
    }
}

impl Curd for Expenditure {
    const COUNT_SQL: &'static str = "SELECT COUNT(1) FROM expenditure WHERE 1=1";
    const QUERY_SQL: &'static str = "SELECT * FROM expenditure WHERE 1=1";
    const BY_ID_SQL: &'static str = "SELECT * FROM expenditure WHERE exp_id = ?";
    const DELETE_BATCH_SQL: &'static str = "DELETE FROM expenditure WHERE exp_id IN (";

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, Sqlite>) {
        if let Some(order_id) = self.order_id {
            builder.push(" AND order_id = ").push_bind(order_id);
        }

        if let Some(title) = &self.exp_title {
            builder
                .push(" AND exp_title LIKE ")
                .push(format!("'%{}%'", title));
        }

        if let Some(exp_type) = &self.exp_type {
            builder
                .push(" AND exp_type LIKE ")
                .push(format!("'%{}%'", exp_type));
        }
    }
}

impl Validator for Expenditure {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

impl Expenditure {
    // insert
    pub async fn create(&self, tx: &mut sqlx::Transaction<'_, Sqlite>) -> Result<Self> {
        let result = sqlx::query_as("INSERT INTO expenditure (order_id, cloth_ids, exp_title, recv_account, recv_account_title, exp_type, exp_amount, create_time, remark) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *")
            .bind(&self.order_id)
            .bind(&self.cloth_ids)
            .bind(&self.exp_title)
            .bind(&self.recv_account)
            .bind(&self.recv_account_title)
            .bind(&self.exp_type)
            .bind(&self.exp_amount)
            .bind(utils::get_now())
            .bind(&self.remark)
            .fetch_one(&mut **tx)
            .await?;
        Ok(result)
    }

    // update
    pub async fn update(&self, tx: &mut sqlx::Transaction<'_, Sqlite>) -> Result<bool> {
        let result = sqlx::query("UPDATE expenditure SET order_id = ?, cloth_ids = ?, exp_title = ?, recv_account = ?, recv_account_title = ?, exp_type = ?, exp_amount = ?, create_time = ?, remark = ? WHERE exp_id = ?")
            .bind(&self.order_id)
            .bind(&self.cloth_ids)
            .bind(&self.exp_title)
            .bind(&self.recv_account)
            .bind(&self.recv_account_title)
            .bind(&self.exp_type)
            .bind(&self.exp_amount)
            .bind(&self.create_time)
            .bind(&self.remark)
            .bind(&self.exp_id)
            .execute(&mut **tx)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

#[tauri::command]
pub async fn get_exp_pagination(
    state: State<'_, AppState>,
    page_params: PageParams,
    exp: Expenditure,
) -> Result<PageResult<Expenditure>> {
    exp.get_list(&state.0, page_params).await
}

#[tauri::command]
pub async fn get_exp_by_id(state: State<'_, AppState>, exp_id: i64) -> Result<Option<Expenditure>> {
    Expenditure::get_by_id(&state.0, exp_id).await
}

#[tauri::command]
pub async fn create_exp(state: State<'_, AppState>, exp: Expenditure) -> Result<Expenditure> {
    let mut tx = state.0.begin().await?;
    let result = exp.create(&mut tx).await?;
    tx.commit().await?;
    Ok(result)
}

#[tauri::command]
pub async fn update_exp(state: State<'_, AppState>, exp: Expenditure) -> Result<bool> {
    let mut tx = state.0.begin().await?;
    let result = exp.update(&mut tx).await?;
    tx.commit().await?;
    Ok(result)
}

#[tauri::command]
pub async fn delete_exp(state: State<'_, AppState>, ids: Vec<i64>) -> Result<bool> {
    let mut tx = state.0.begin().await?;
    let result = Expenditure::delete_batch(&mut tx, &ids).await?;
    tx.commit().await?;
    Ok(result)
}
