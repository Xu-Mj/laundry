use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use tauri::State;

use crate::error::Result;
use crate::state::AppState;


/// 更新用户的引导记录
#[tauri::command]
pub async fn update_tour_guide(state: State<'_, AppState>, page_key: String) -> Result<bool> {
    let mut user = match state.get_user_info().await {
        Some(user) => user,
        None => return Ok(false),
    };

    // 解析当前的completed_tours字段
    let mut completed_tours = match &user.completed_tours {
        Some(tours) => {
            if tours.is_empty() {
                serde_json::Map::new()
            } else {
                match serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(tours) {
                    Ok(map) => map,
                    Err(_) => serde_json::Map::new(),
                }
            }
        }
        None => serde_json::Map::new(),
    };

    // 添加或更新页面引导状态
    completed_tours.insert(
        page_key,
        serde_json::Value::Bool(true),
    );

    // 更新用户的completed_tours字段
    user.completed_tours = Some(serde_json::to_string(&completed_tours).unwrap_or_default());
    
    // 保存更新后的用户信息
    match user.upsert(&state.pool).await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// 检查用户是否已完成特定页面的引导
#[tauri::command]
pub async fn check_tour_completed(state: State<'_, AppState>, page_key: String) -> Result<bool> {
    let user = match state.get_user_info().await {
        Some(user) => user,
        None => return Ok(false),
    };
    
    // 解析completed_tours字段
    match &user.completed_tours {
        Some(tours) if !tours.is_empty() => {
            match serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(tours) {
                Ok(map) => {
                    // 检查是否包含指定页面的引导记录
                    if let Some(value) = map.get(&page_key) {
                        if let Some(completed) = value.as_bool() {
                            return Ok(completed);
                        }
                    }
                    Ok(false)
                },
                Err(_) => Ok(false),
            }
        },
        _ => Ok(false),
    }
}