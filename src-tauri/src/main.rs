// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_lib::{
    db::{self, initialize_database, user, DbPool},
    printer,
};

use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite};
use tauri::generate_handler;

#[tokio::main]
async fn main() {
    // 获取应用数据目录
    let db_url = "sqlite://my_database.db";
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        // 数据库不存在，创建数据库文件
        println!("Creating database file...");
        Sqlite::create_database(&db_url)
            .await
            .expect("Failed to create database file");
    }
    let pool = SqlitePoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    // 初始化数据库
    initialize_database(&pool)
        .await
        .expect("Failed to initialize database");

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .manage(DbPool::new(pool))
        .invoke_handler(generate_handler![
            user::insert_user,
            user::get_users,
            printer::print,
            db::printer::get_printers,
            db::printer::set_printer,
            db::printer::get_settled_printer,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
