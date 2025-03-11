use serde::Serialize;
use serde_json::json;
use tauri::State;

use crate::db::orders::{Order, SourceDistribution};
use crate::db::payments::{
    PaymentSummary, get_daily_payment_summary, get_monthly_payment_summary,
    get_weekly_payment_summary,
};
use crate::error::Result;
use crate::state::AppState;

#[derive(Debug, Default, Serialize)]
pub struct Chart {
    pub source: Vec<SourceDistribution>,
}

#[derive(Debug, Default, Serialize)]
pub struct CountItem {
    pub title: String,
    pub count: i64,
}

#[tauri::command]
pub async fn query_count(state: State<'_, AppState>) -> Result<Vec<CountItem>> {
    let pool = &state.pool;

    let mut count_list = Vec::<CountItem>::with_capacity(5);
    // select 洗护中订单
    let count = Order::query_count_by_status(pool, "01").await?;
    count_list.push(CountItem {
        title: "洗护中订单".to_string(),
        count,
    });
    // select 待取
    let count = Order::query_count_by_status(pool, "02").await?;
    count_list.push(CountItem {
        title: "待取订单".to_string(),
        count,
    });
    // select 线上预约
    let count = Order::query_count_by_status(pool, "002").await?;
    count_list.push(CountItem {
        title: "线上预约".to_string(),
        count,
    });
    // select 线上订单
    let count = Order::query_count_by_status(pool, "002").await?;
    count_list.push(CountItem {
        title: "线上预约".to_string(),
        count,
    });
    Ok(count_list)
}

// 获取首页统计数据
#[tauri::command]
pub async fn query_total_count(state: State<'_, AppState>) -> Result<i64> {
    Order::total(&state.pool).await
}

// 获取首页统计数据
#[tauri::command]
pub async fn query_chart(state: State<'_, AppState>) -> Result<Chart> {
    let pool = &state.pool;
    let source = Order::count_by_source(pool).await?;
    Ok(Chart { source })
}

#[tauri::command]
pub async fn fetch_monthly_payment_summary(
    state: State<'_, AppState>,
    year: Option<i32>,
    month: Option<u32>,
) -> Result<Vec<PaymentSummary>> {
    let pool = &state.pool;
    let summaries = get_monthly_payment_summary(pool, year, month).await?;
    Ok(summaries)
}

#[tauri::command]
pub async fn fetch_payment_summary(state: State<'_, AppState>) -> Result<serde_json::Value> {
    let pool = &state.pool;
    let daily_summary = get_daily_payment_summary(pool).await?;
    let weekly_summary = get_weekly_payment_summary(pool).await?;
    Ok(json!([daily_summary, weekly_summary]))
}
