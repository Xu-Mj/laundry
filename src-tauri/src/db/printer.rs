use serde::{Deserialize, Serialize};
use tauri::State;

use crate::error::Result;
use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PrinterConfiguration {
    pub name: String,
    pub system_name: String,
    pub driver_name: String,
    pub printer_type: String, // 'business' 或 'receipt'
}

impl PrinterConfiguration {
    pub fn new(name: String, system_name: String, driver_name: String, printer_type: String) -> Self {
        Self {
            name,
            system_name,
            driver_name,
            printer_type,
        }
    }
}

#[tauri::command]
pub async fn get_printers() -> Vec<PrinterConfiguration> {
    let printers = printers::get_printers();
    printers
        .into_iter()
        .map(|item| PrinterConfiguration::new(item.name, item.system_name, item.driver_name, "business".to_string()))
        .collect()
}

#[tauri::command]
pub async fn get_settled_printer(
    state: State<'_, AppState>,
    printer_type: String,
) -> Result<Option<PrinterConfiguration>> {
    let device = sqlx::query_as::<_, PrinterConfiguration>("SELECT * FROM printers WHERE printer_type = ? LIMIT 1")
        .bind(&printer_type)
        .fetch_optional(&state.pool)
        .await?;
    Ok(device)
}

#[tauri::command]
pub async fn set_printer(state: State<'_, AppState>, printer: PrinterConfiguration) -> Result<()> {
    sqlx::query(
        "INSERT OR REPLACE INTO printers (id, name, system_name, driver_name, printer_type) VALUES ((SELECT id FROM printers WHERE printer_type = ?), ?, ?, ?, ?)"
    )
    .bind(&printer.printer_type)
    .bind(&printer.name)
    .bind(&printer.system_name)
    .bind(&printer.driver_name)
    .bind(&printer.printer_type)
    .execute(&state.pool)
    .await?;
    Ok(())
}
