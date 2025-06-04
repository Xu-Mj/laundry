use std::fs;
use std::path::Path;

use sqlx::Sqlite;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;
use tauri::Emitter;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_updater::UpdaterExt;
use tracing::{debug, info};
use serde::Serialize;

#[derive(Clone, Serialize)]
struct UpdateProgress {
    downloaded: usize,
    total: Option<u64>,
    percent: f32,
}

pub async fn update<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> tauri_plugin_updater::Result<()> {
    // 打印当前应用版本
    let current_version = app.package_info().version.to_string();
    tracing::info!("Current app version: {}", current_version);

    match app.updater()?.check().await {
        Ok(Some(update)) => {
            // 获取更新信息
            let update_available = update.version.to_string();
            
            // 创建更新器的克隆，以便在回调中使用
            let update_clone = update.clone();
            let app_clone = app.clone();
            
            // 弹出对话框请求用户确认
            app.dialog()
                .message(format!("发现新版本 {}，是否现在更新？", update_available))
                .title("发现新版本")
                .buttons(MessageDialogButtons::OkCancelCustom(
                    "更新".into(),
                    "取消".into(),
                ))
                .show(move |result| {
                    if result {
                        // 用户同意更新，启动一个新的异步任务来处理更新
                        let app_handle = app_clone.clone();
                        let update_handle = update_clone.clone();
                        
                        // 通知前端开始更新，显示进度条
                        app_handle.emit("update-started", ()).unwrap_or_else(|e| {
                            tracing::error!("Failed to emit update-started event: {:?}", e);
                        });
                        
                        // 使用tauri命令在后台处理更新下载和安装
                        tauri::async_runtime::spawn(async move {
                            let mut downloaded = 0;
                            match update_handle
                                .download_and_install(
                                    |chunk_length, content_length| {
                                        downloaded += chunk_length;
                                        
                                        // 计算下载百分比
                                        let percent = if let Some(total) = content_length {
                                            (downloaded as f32 / total as f32) * 100.0
                                        } else {
                                            0.0
                                        };
                                        
                                        // 向前端发送进度更新
                                        let progress = UpdateProgress {
                                            downloaded,
                                            total: content_length,
                                            percent,
                                        };
                                        
                                        app_handle.emit("update-progress", progress).unwrap_or_else(|e| {
                                            tracing::error!("Failed to emit update progress: {:?}", e);
                                        });
                                        
                                        tracing::info!(
                                            "Downloaded {} from {:?} ({:.2}%)",
                                            downloaded,
                                            content_length,
                                            percent
                                        );
                                    },
                                    || {
                                        // 下载完成，通知前端
                                        app_handle.emit("update-downloaded", ()).unwrap_or_else(|e| {
                                            tracing::error!("Failed to emit update-downloaded event: {:?}", e);
                                        });
                                        tracing::info!("Download finished");
                                    },
                                )
                                .await 
                            {
                                Ok(_) => {
                                    // 安装完成，通知前端
                                    app_handle.emit("update-installed", ()).unwrap_or_else(|e| {
                                        tracing::error!("Failed to emit update-installed event: {:?}", e);
                                    });
                                    
                                    tracing::info!("Update installed");
                                    
                                    // 通知用户更新成功，准备重启
                                    app_handle.dialog()
                                        .message("更新已安装完成，点击确定后将重启应用。")
                                        .title("更新完成")
                                        .show(move |_| {
                                            app_handle.restart();
                                        });
                                },
                                Err(err) => {
                                    // 更新失败，通知前端
                                    app_handle.emit("update-failed", ()).unwrap_or_else(|e| {
                                        tracing::error!("Failed to emit update-failed event: {:?}", e);
                                    });
                                    
                                    tracing::error!("Failed to install update: {:?}", err);
                                    
                                    // 通知用户更新失败
                                    app_handle.dialog()
                                        .message("更新安装失败，请稍后再试。")
                                        .title("更新失败")
                                        .show(|_| {});
                                }
                            }
                        });
                    } else {
                        tracing::info!("Update cancelled by user");
                    }
                });
        }
        Ok(None) => {
            tracing::info!("No updates found");
        }
        Err(err) => {
            // 打印详细的错误信息
            tracing::error!("Failed to check for updates: {:?}", err);
        }
    }

    Ok(())
}

pub async fn migrate() -> Result<(), sqlx::Error> {
    let db_path = "database.db";
    if !Sqlite::database_exists(&db_path).await.unwrap_or(false) {
        // 数据库不存在，创建数据库文件
        println!("Creating database file...");
        Sqlite::create_database(&db_path).await?;
    }
    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite://{}", db_path))
        .await?;

    info!("Connecting to database at {}", db_path);

    // 检查 migrations 表是否存在
    let migrations_table_exists: Option<i32> =
        sqlx::query_scalar("SELECT 1 FROM sqlite_master WHERE type='table' AND name='migrations'")
            .fetch_optional(&pool)
            .await?;
    let migrations_table_exists = migrations_table_exists.is_some();
    if !migrations_table_exists {
        info!("migrations table does not exist, creating...");

        // 执行 init.sql
        let init_sql_path = "scripts/V1__init.sql";
        let init_sql = fs::read_to_string(init_sql_path)?;
        sqlx::query(&init_sql).execute(&pool).await?;

        // 记录初始版本
        sqlx::query("INSERT INTO migrations (version) VALUES (?)")
            .bind(1)
            .execute(&pool)
            .await?;

        info!("migrations table created and initial version recorded");
    } else {
        info!("migrations table already exists");
    }

    // 获取已应用的最新版本号
    let last_version: Option<i32> = sqlx::query_scalar("SELECT MAX(version) FROM migrations")
        .fetch_optional(&pool)
        .await?;

    info!("Last applied migration version: {:?}", last_version);

    // 获取所有迁移脚本文件
    let migrations_dir = Path::new("scripts");
    let mut migration_files: Vec<_> = fs::read_dir(migrations_dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("sql")
        })
        .filter(|path| {
            // 检查文件名是否符合 V<version>__<description>.sql 的命名规则
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let parts: Vec<&str> = file_name.split('_').collect();
            parts.len() >= 2 && parts[0].starts_with('V') && parts[0][1..].parse::<i32>().is_ok()
        })
        .collect();

    // 按版本号排序
    migration_files.sort_by_key(|path| {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let version: i32 = file_name.split('_').nth(0).unwrap()[1..].parse().unwrap();
        version
    });

    debug!("Migration files: {:?}", migration_files);

    // 执行未应用的迁移脚本
    for migration_file in migration_files {
        let file_name = migration_file.file_name().unwrap().to_str().unwrap();
        let version: i32 = file_name.split('_').nth(0).unwrap()[1..].parse().unwrap();

        if last_version.map_or(true, |last| version > last) {
            info!("Applying migration script: {}", file_name);

            let sql = fs::read_to_string(&migration_file)?;
            sqlx::query(&sql).execute(&pool).await?;

            // 记录应用的版本
            sqlx::query("INSERT INTO migrations (version) VALUES (?)")
                .bind(version)
                .execute(&pool)
                .await?;

            info!("Migration script {} applied successfully", file_name);
        } else {
            info!("Migration script {} already applied", file_name);
        }
    }

    info!("Database migration completed successfully");

    Ok(())
}
