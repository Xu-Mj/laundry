use serde::Serialize;
use tauri::State;

use crate::constants::OrderStatus;
use crate::db::orders::{Order, SourceDistribution};
use crate::db::payments::{
    PaymentSummary, get_daily_payment_summary, get_monthly_payment_summary,
    get_weekly_payment_summary,
};
use crate::error::Result;
use crate::payments::PaymentSummaryWithRate;
use crate::state::AppState;
use crate::utils;

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
    let store_id = utils::get_user_id(&state).await?;
    let pool = &state.pool;

    let mut count_list = Vec::<CountItem>::with_capacity(5);
    // select 洗护中订单
    let count = Order::query_count_by_status(pool, store_id, OrderStatus::Processing).await?;
    count_list.push(CountItem {
        title: "洗护中订单".to_string(),
        count,
    });
    // select 待取
    let count = Order::query_count_by_status(pool, store_id, OrderStatus::ReadyForPickup).await?;
    count_list.push(CountItem {
        title: "待取订单".to_string(),
        count,
    });
    // select 线上预约
    // let count = Order::query_count_by_status(pool, "002").await?;
    // count_list.push(CountItem {
    //     title: "线上预约".to_string(),
    //     count,
    // });
    // // select 线上订单
    // let count = Order::query_count_by_status(pool, "002").await?;
    // count_list.push(CountItem {
    //     title: "线上预约".to_string(),
    //     count,
    // });
    Ok(count_list)
}

// 获取首页统计数据
#[tauri::command]
pub async fn query_total_count(state: State<'_, AppState>) -> Result<i64> {
    let store_id = utils::get_user_id(&state).await?;
    Order::total(&state.pool, store_id).await
}

// 获取首页统计数据
#[tauri::command]
pub async fn query_chart(
    state: State<'_, AppState>,
    year: Option<i32>,
    month: Option<u32>,
) -> Result<Chart> {
    let store_id = utils::get_user_id(&state).await?;
    let source = Order::count_by_source(&state.pool, store_id, year, month).await?;
    Ok(Chart { source })
}

#[tauri::command]
pub async fn fetch_monthly_payment_summary(
    state: State<'_, AppState>,
    year: Option<i32>,
    month: Option<u32>,
) -> Result<Vec<PaymentSummary>> {
    let store_id = utils::get_user_id(&state).await?;
    let summaries = get_monthly_payment_summary(&state.pool, store_id, year, month).await?;
    Ok(summaries)
}

// 修改接口实现
#[tauri::command]
pub async fn fetch_payment_summary(
    state: State<'_, AppState>,
) -> Result<Vec<PaymentSummaryWithRate>> {
    let store_id = utils::get_user_id(&state).await?;
    let pool = &state.pool;

    let daily = get_daily_payment_summary(pool, store_id).await?;
    let weekly = get_weekly_payment_summary(pool, store_id).await?;

    Ok(vec![daily, weekly])
}
