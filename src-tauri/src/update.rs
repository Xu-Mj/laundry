use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Sqlite;
use std::fs;
use std::path::Path;
use tracing::{info, debug};

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