// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_lib::update::migrate;
use sqlx::sqlite::SqlitePoolOptions;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

use app_lib::captcha::start_cleanup_thread;
use app_lib::{config::Config, create_app, state::AppState};

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
    if cfg!(debug_assertions) {
        // 在 debug 模式下，日志输出到控制台
        tracing_subscriber::FmtSubscriber::builder()
            .with_line_number(true)
            .with_max_level(config.log.level())
            .with_timer(LocalTimer)
            .init();
    } else {
        // 在 release 模式下，日志输出到文件
        let file_appender = tracing_appender::rolling::daily(&config.log.output, "laundry");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        tracing_subscriber::FmtSubscriber::builder()
            .with_line_number(true)
            .with_max_level(config.log.level())
            .with_writer(non_blocking)
            .with_timer(LocalTimer)
            .init();
    }

    // 执行数据迁移
    migrate().await.expect("database migrate failed");
    // 获取应用数据目录
    let db_url = "sqlite://database.db";

    let pool = SqlitePoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    start_cleanup_thread();

    create_app(
        tauri::Builder::default()
            .plugin(tauri_plugin_updater::Builder::new().build())
            .plugin(tauri_plugin_dialog::init())
            .plugin(tauri_plugin_process::init())
            .plugin(tauri_plugin_fs::init()),
        AppState::new(pool, config.get_url()),
    )
    .run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}
