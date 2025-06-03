// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;
use tauri::Manager;
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

fn ensure_log_dir(log_dir: &str) -> std::io::Result<()> {
    let path = std::path::Path::new(log_dir);
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

// 初始化 rustls 加密提供者
fn init_rustls_crypto() {
    use rustls::crypto::{CryptoProvider, aws_lc_rs};

    // 尝试安装 aws-lc-rs 作为默认加密提供者
    if CryptoProvider::get_default().is_none() {
        let _ = aws_lc_rs::default_provider().install_default();
    }
}

// 创建一个允许无效证书的 rustls 配置（仅用于开发环境）
fn create_insecure_rustls_config() -> Arc<rustls::ClientConfig> {
    use rustls::ClientConfig;

    // 创建一个自定义的证书验证器，接受所有证书
    #[derive(Debug)]
    struct InsecureVerifier;

    impl rustls::client::danger::ServerCertVerifier for InsecureVerifier {
        fn verify_server_cert(
            &self,
            _end_entity: &rustls::pki_types::CertificateDer<'_>,
            _intermediates: &[rustls::pki_types::CertificateDer<'_>],
            _server_name: &rustls::pki_types::ServerName<'_>,
            _ocsp_response: &[u8],
            _now: rustls::pki_types::UnixTime,
        ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
            Ok(rustls::client::danger::ServerCertVerified::assertion())
        }

        fn verify_tls12_signature(
            &self,
            _message: &[u8],
            _cert: &rustls::pki_types::CertificateDer<'_>,
            _dss: &rustls::DigitallySignedStruct,
        ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
            Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
        }

        fn verify_tls13_signature(
            &self,
            _message: &[u8],
            _cert: &rustls::pki_types::CertificateDer<'_>,
            _dss: &rustls::DigitallySignedStruct,
        ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
            Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
        }

        fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
            vec![
                rustls::SignatureScheme::RSA_PKCS1_SHA1,
                rustls::SignatureScheme::ECDSA_SHA1_Legacy,
                rustls::SignatureScheme::RSA_PKCS1_SHA256,
                rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
                rustls::SignatureScheme::RSA_PKCS1_SHA384,
                rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
                rustls::SignatureScheme::RSA_PKCS1_SHA512,
                rustls::SignatureScheme::ECDSA_NISTP521_SHA512,
                rustls::SignatureScheme::RSA_PSS_SHA256,
                rustls::SignatureScheme::RSA_PSS_SHA384,
                rustls::SignatureScheme::RSA_PSS_SHA512,
                rustls::SignatureScheme::ED25519,
                rustls::SignatureScheme::ED448,
            ]
        }
    }

    let config = ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(InsecureVerifier))
        .with_no_client_auth();

    Arc::new(config)
}

// 创建标准的 rustls 配置（用于生产环境）
#[allow(dead_code)]
fn create_secure_rustls_config() -> Result<Arc<rustls::ClientConfig>, Box<dyn std::error::Error>> {
    let mut root_store = rustls::RootCertStore::empty();

    // 加载系统根证书
    for cert in rustls_native_certs::load_native_certs().certs {
        root_store.add(cert)?;
    }

    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    Ok(Arc::new(config))
}

#[tokio::main]
async fn main() {
    // 首先初始化 rustls 加密提供者 - 必须在任何 rustls 操作之前
    init_rustls_crypto();

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
        let log_dir = &config.log.output;
        ensure_log_dir(log_dir).unwrap();

        let file_appender = tracing_appender::rolling::daily(log_dir, "laundry.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        let subscriber = tracing_subscriber::fmt()
            .with_line_number(true)
            .with_file(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_target(true)
            .with_max_level(config.log.level())
            .with_timer(LocalTimer)
            .with_ansi(cfg!(debug_assertions))
            .with_writer(non_blocking)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set global default subscriber");
    }

    // 执行数据迁移
    migrate().await.expect("database migrate failed");

    let db_url = "sqlite://database.db";
    let pool = SqlitePoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    start_cleanup_thread();

    // 配置 rustls TLS 连接器
    // let rustls_config = if cfg!(debug_assertions) {
    //     // 开发环境：允许无效证书
    //     tracing::warn!("Using insecure TLS configuration (debug mode)");
    //     create_insecure_rustls_config()
    // } else {
    //     // 生产环境：使用安全配置
    //     create_secure_rustls_config().expect("Failed to create secure TLS configuration")
    // };

    let connector = Connector::Rustls(create_insecure_rustls_config());

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
    .run(|app_handle, event| match event {
        tauri::RunEvent::WindowEvent {
            label: _,
            event: window_event,
            ..
        } => {
            match window_event {
                tauri::WindowEvent::Focused(focused) => {
                    if focused {
                        // 窗口获得焦点时，检查是否需要刷新 token
                        let state = app_handle.state::<AppState>().inner().clone();
                        let state_clone = state.clone();
                        tokio::spawn(async move {
                            state_clone.check_and_refresh_after_sleep().await;
                        });
                    }
                }
                _ => {}
            }
        }
        _ => {}
    });
}
