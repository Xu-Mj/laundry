use serde::Serialize;
use tauri::State;

use crate::db::orders::{Order, SourceDistribution};
use crate::db::AppState;
use crate::error::Result;

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
    let pool = &state.0;

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
    Order::total(&state.0).await
}

// 获取首页统计数据
#[tauri::command]
pub async fn query_chart(state: State<'_, AppState>) -> Result<Chart> {
    let pool = &state.0;
    let source = Order::count_by_source(pool).await?;
    Ok(Chart { source })
}
