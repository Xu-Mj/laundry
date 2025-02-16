use serde::{Deserialize, Serialize};
use tauri::State;

use super::AppState;

use crate::error::Result;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PrinterConfiguration {
    pub name: String,
    pub system_name: String,
    pub driver_name: String,
}

impl PrinterConfiguration {
    pub fn new(name: String, system_name: String, driver_name: String) -> Self {
        Self {
            name,
            system_name,
            driver_name,
        }
    }
}
#[tauri::command]
pub async fn get_printers() -> Vec<PrinterConfiguration> {
    let printers = printers::get_printers();
    printers
        .into_iter()
        .map(|item| PrinterConfiguration::new(item.name, item.system_name, item.driver_name))
        .collect()
}

#[tauri::command]
pub async fn get_settled_printer(
    state: State<'_, AppState>,
) -> Result<Option<PrinterConfiguration>> {
    let device = sqlx::query_as::<_, PrinterConfiguration>("SELECT * FROM printers LIMIT 1")
        .fetch_optional(&state.pool)
        .await?;
    Ok(device)
}

#[tauri::command]
pub async fn set_printer(state: State<'_, AppState>, printer: PrinterConfiguration) -> Result<()> {
    sqlx::query(
        "INSERT OR REPLACE INTO printers (id, name, system_name, driver_name) VALUES (1, ?, ?, ?)",
    )
    .bind(printer.name)
    .bind(printer.system_name)
    .bind(printer.driver_name)
    .execute(&state.pool)
    .await?;
    Ok(())
}
