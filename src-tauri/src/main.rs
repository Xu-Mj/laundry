// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlx::sqlite::SqlitePoolOptions;
use tokio_native_tls::native_tls;
use tokio_tungstenite::Connector;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

use app_lib::captcha::start_cleanup_thread;
use app_lib::update::migrate;
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
        tracing::debug!("debug mode"); // 输出日志到控制台上
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
        tracing::debug!("release mode"); // 输出日志到文件
    }

    // 执行数据迁移
    migrate().await.expect("database migrate failed");

    let db_url = "sqlite://database.db";
    let pool = SqlitePoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    start_cleanup_thread();

    // 配置 TLS 连接器
    let tls_connector = native_tls::TlsConnector::builder()
        .danger_accept_invalid_certs(true) // 允许自签名证书
        .build()
        .unwrap();

    let connector = Connector::from(Connector::NativeTls(tls_connector.into()));

    let ws_plugin = tauri_plugin_websocket::Builder::new()
        .tls_connector(connector)
        .build();
    
    create_app(
        tauri::Builder::default()
            .plugin(tauri_plugin_updater::Builder::new().build())
            .plugin(tauri_plugin_dialog::init())
            .plugin(tauri_plugin_process::init())
            .plugin(tauri_plugin_fs::init())
            .plugin(ws_plugin),
        AppState::new(pool, config.get_url()),
    )
    .run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}
