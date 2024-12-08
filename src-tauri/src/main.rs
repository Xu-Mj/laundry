// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

use app_lib::captcha::start_cleanup_thread;
use app_lib::{
    config::Config,
    create_app,
    db::{initialize_database, AppState},
};

const DEFAULT_CONFIG_PATH: &str = "./config.yml";
struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        let local_time = chrono::Local::now();
        write!(w, "{}", local_time.format("%Y-%m-%dT%H:%M:%S%.6f%:z"))
    }
}

#[tokio::main]
async fn main() {
    // 初始化日志配置
    let config = Config::load(DEFAULT_CONFIG_PATH).unwrap();

    // init tracing
    if config.log.output != "console" {
        // redirect log to file
        let file_appender = tracing_appender::rolling::daily(&config.log.output, "laundry");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        // builder = builder.with_writer(non_blocking);
        tracing_subscriber::FmtSubscriber::builder()
            .with_line_number(true)
            .with_max_level(config.log.level())
            .with_writer(non_blocking)
            .with_timer(LocalTimer)
            .init();
    } else {
        // log to console
        tracing_subscriber::FmtSubscriber::builder()
            .with_line_number(true)
            .with_max_level(config.log.level())
            .with_timer(LocalTimer)
            .init();
    }
    // 获取应用数据目录
    let db_url = "sqlite://database.db";
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
    start_cleanup_thread();

    create_app(
        tauri::Builder::default().plugin(tauri_plugin_fs::init()),
        AppState::new(pool),
    )
    .run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}
