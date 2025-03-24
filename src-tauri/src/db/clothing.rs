use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{Executor, FromRow, Pool, QueryBuilder, Row, Sqlite, SqlitePool, Transaction};
use tauri::State;

use crate::error::{Error, ErrorKind, Result};
use crate::state::AppState;
use crate::utils;

use super::{Curd, PageParams, PageResult};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Clothing {
    /// 衣物唯一标识ID
    #[serde(default)]
    clothing_id: Option<i64>,
    /// 衣物名称
    #[serde(default)]
    clothing_name: Option<String>,
    /// 衣物编码
    #[serde(default)]
    clothing_number: Option<String>,
    /// 衣物品类
    #[serde(default)]
    clothing_category: Option<String>,
    /// 所属分类，000上衣，001鞋，002裤子等
    #[serde(default)]
    clothing_style: Option<String>,
    /// 显示顺序，默认显示顺序
    #[serde(default)]
    order_num: Option<i64>,
    /// 使用次数，实际被使用的次数，当该值不为0时，将优先将按照该值排序，然后才是显示顺序排序
    #[serde(default)]
    clothing_degree: Option<i64>,
    /// 基础价格，用于计算价格
    #[serde(default)]
    clothing_base_price: Option<f64>,
    /// 最小价格，用于计算价格
    #[serde(default)]
    clothing_min_price: Option<f64>,
    #[serde(default)]
    hang_type: Option<String>,
    /// 备注
    #[serde(default)]
    remark: Option<String>,
    #[serde(default)]
    del_flag: Option<String>,
}

impl FromRow<'_, SqliteRow> for Clothing {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            clothing_id: row.try_get("clothing_id").unwrap_or_default(),
            clothing_name: row.try_get("clothing_name").unwrap_or_default(),
            clothing_number: row.try_get("clothing_number").unwrap_or_default(),
            clothing_category: row.try_get("clothing_category").unwrap_or_default(),
            clothing_style: row.try_get("clothing_style").unwrap_or_default(),
            order_num: row.try_get("order_num").unwrap_or_default(),
            clothing_degree: row.try_get("clothing_degree").unwrap_or_default(),
            clothing_base_price: row.try_get("clothing_base_price").unwrap_or_default(),
            clothing_min_price: row.try_get("clothing_min_price").unwrap_or_default(),
            hang_type: row.try_get("hang_type").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
            del_flag: row.try_get("del_flag").unwrap_or_default(),
        })
    }
}

impl Curd for Clothing {
    const COUNT_SQL: &'static str = "SELECT COUNT(*) FROM clothing WHERE del_flag = '0' ";
    const QUERY_SQL: &'static str = "SELECT * FROM clothing WHERE del_flag = '0' ";
    const BY_ID_SQL: &'static str =
        "SELECT * FROM clothing WHERE clothing_id = ? AND del_flag = '0'";
    const DELETE_BATCH_SQL: &'static str =
        "UPDATE clothing SET del_flag = '2' WHERE clothing_id IN (";

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, Sqlite>) {
        if let Some(clothing_id) = &self.clothing_id {
            builder.push(" AND clothing_id = ").push_bind(clothing_id);
        }

        if let Some(clothing_name) = &self.clothing_name {
            builder
                .push(" AND clothing_name LIKE ")
                .push_bind(format!("%{}%", clothing_name));
        }

        if let Some(clothing_category) = &self.clothing_category {
            builder
                .push(" AND clothing_category = ")
                .push_bind(clothing_category);
        }

        if let Some(clothing_style) = &self.clothing_style {
            builder
                .push(" AND clothing_style = ")
                .push_bind(clothing_style);
        }

        if let Some(clothing_number) = &self.clothing_number {
            builder
                .push(" AND clothing_number LIKE ")
                .push_bind(format!("%{}%", clothing_number));
        }

        if let Some(clothing_degree) = &self.clothing_degree {
            builder
                .push(" AND clothing_degree = ")
                .push_bind(clothing_degree);
        }

        if let Some(remark) = &self.remark {
            builder.push(" AND remark = ").push_bind(remark);
        }
    }
}

impl Clothing {
    /// 查询是否衣物编码是否已经存在
    /// 这里存在一个潜在的bug：如果超过四位，那么会变成1000，进而产生唯一索引错误
    pub async fn select_next_num(pool: &Pool<Sqlite>, prefix: &str) -> Result<String> {
        let next_clothing_number: (String,) = sqlx::query_as(
            r#"
        SELECT
            CASE
                WHEN MAX_number IS NULL THEN printf('%s%04d', ?1, 1)
                ELSE printf('%s%04d', ?1, MAX_number + 1)
            END AS next_clothing_number
        FROM (
            SELECT MAX(CAST(SUBSTR(clothing_number, LENGTH(?1) + 1) AS INTEGER)) AS MAX_number
            FROM clothing
            WHERE clothing_number LIKE (?1 || '%')
        ) AS subquery;
        "#,
        )
        .bind(prefix)
        .fetch_one(pool)
        .await?;

        Ok(next_clothing_number.0)
    }

    // 软删除衣物
    pub async fn soft_delete(pool: &Pool<Sqlite>, clothing_id: i64) -> Result<u64> {
        let result = sqlx::query("UPDATE clothing SET del_flag = '2' WHERE clothing_id = ?")
            .bind(clothing_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }

    // 增加ref_num
    pub async fn increment_ref_num(
        tx: &mut Transaction<'_, Sqlite>,
        clothing_id: i64,
    ) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE clothing SET clothing_degree = clothing_degree + 1 WHERE clothing_id = ? RETURNING *",
        )
        .bind(clothing_id)
        .execute(&mut **tx)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // 增加ref_num
    pub async fn update_ref_num(
        pool: &Pool<Sqlite>,
        ref_num: i64,
        clothing_ids: Vec<i64>,
    ) -> Result<()> {
        let mut query_builder =
            QueryBuilder::<Sqlite>::new("UPDATE clothing SET clothing_degree = ");
        query_builder
            .push_bind(ref_num)
            .push(" WHERE clothing_id IN (");

        let mut separated = query_builder.separated(", ");
        for clothing_id in &clothing_ids {
            separated.push_bind(clothing_id);
        }
        query_builder.push(")");

        query_builder.build().execute(pool).await?;
        Ok(())
    }

    // 检查clothing_name是否已经存在
    pub async fn exists_by_clothing_name(pool: &Pool<Sqlite>, clothing_name: &str) -> Result<bool> {
        let result = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM clothing WHERE clothing_name = $1 AND del_flag = '0')",
        )
        .bind(clothing_name)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    /// 异步向数据库中添加衣物信息，并返回新添加的衣物对象。
    ///
    /// # Parameters
    ///
    /// * `pool`: 数据库连接池的引用，用于执行数据库操作。
    ///
    /// # Returns
    ///
    /// 返回一个结果类型，包含新添加的衣物信息。
    pub async fn add(self, pool: &Pool<Sqlite>) -> Result<Clothing> {
        let result = sqlx::query_as::<_, Clothing>(
            "INSERT INTO clothing (clothing_name, clothing_number, clothing_category, clothing_style,
             clothing_base_price, clothing_min_price, order_num, clothing_degree, remark, del_flag)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, '0')
             RETURNING *"
        )
            .bind(&self.clothing_name)
            .bind(&self.clothing_number)
            .bind(&self.clothing_category)
            .bind(&self.clothing_style)
            .bind(&self.clothing_base_price)
            .bind(&self.clothing_min_price)
            .bind(&self.order_num)
            .bind(&self.clothing_degree)
            .bind(&self.remark)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    /// 查询所有衣物
    ///
    /// 此函数用于根据提供的衣物参数查询数据库中的所有相关衣物它利用异步SQLITE数据库连接池来执行查询
    /// 主要用途是在需要根据特定条件获取一组衣物时使用
    ///
    /// # 参数
    ///
    /// * `pool` - 一个引用，指向SQLITE数据库的连接池，用于执行数据库操作
    /// * `clothing` - 一个clothingParam类型的实例，包含查询所需的参数信息
    ///
    /// # 返回值
    ///
    /// 返回一个Result类型，包含一个clothing类型的向量，代表查询到的衣物列表如果查询失败，将返回一个sqlx::Error
    pub async fn query_all_clothing(&self, pool: &Pool<Sqlite>) -> Result<Vec<Self>> {
        // 调用build_clothing_query方法来构建和执行查询，这里不需要传递额外的参数，因为查询条件已经包含在clothing参数中
        self.build_query_builder(pool, None).await
    }

    /// 异步更新衣物信息
    ///
    /// 此函数负责将当前衣物对象的更改更新到数据库中的clothing表它动态构建一个UPDATE SQL语句，
    /// 包含需要更新的字段，并执行更新操作如果没有任何字段需要更新，则直接返回当前对象作为结果
    ///
    /// # 参数
    ///
    /// - `pool`: 数据库连接池引用，用于执行SQL查询
    ///
    /// # 返回
    ///
    /// - `Result<Clothing>`: 更新后的衣物对象如果无字段需要更新，则返回当前对象
    pub async fn update(self, pool: &Pool<Sqlite>) -> Result<Clothing> {
        let mut query_builder = QueryBuilder::<Sqlite>::new("UPDATE clothing SET ");
        let mut has_update = false;

        if let Some(clothing_name) = &self.clothing_name {
            query_builder
                .push("clothing_name = ")
                .push_bind(clothing_name);
            has_update = true;
        }

        if let Some(clothing_number) = &self.clothing_number {
            if has_update {
                query_builder.push(", ");
            }
            query_builder
                .push("clothing_number = ")
                .push_bind(clothing_number);
            has_update = true;
        }

        if let Some(clothing_category) = &self.clothing_category {
            if has_update {
                query_builder.push(", ");
            }
            query_builder
                .push(" clothing_category = ")
                .push_bind(clothing_category);
        }

        if let Some(clothing_style) = &self.clothing_style {
            if has_update {
                query_builder.push(", ");
            }
            query_builder
                .push(" clothing_style = ")
                .push_bind(clothing_style);
        }

        if let Some(clothing_degree) = &self.clothing_degree {
            if has_update {
                query_builder.push(", ");
            }
            query_builder
                .push(" clothing_degree = ")
                .push_bind(clothing_degree);
        }

        if let Some(order_num) = &self.order_num {
            if has_update {
                query_builder.push(", ");
            }
            query_builder.push("order_num = ").push_bind(order_num);
            has_update = true;
        }

        if let Some(clothing_base_price) = &self.clothing_base_price {
            if has_update {
                query_builder.push(", ");
            }
            query_builder
                .push("clothing_base_price = ")
                .push_bind(clothing_base_price);
            has_update = true;
        }

        if let Some(clothing_min_price) = &self.clothing_min_price {
            if has_update {
                query_builder.push(", ");
            }
            query_builder
                .push("clothing_min_price = ")
                .push_bind(clothing_min_price);
            has_update = true;
        }

        if let Some(hang_type) = &self.hang_type {
            if has_update {
                query_builder.push(", ");
            }
            query_builder.push("hang_type = ").push_bind(hang_type);
            has_update = true;
        }

        if let Some(remark) = &self.remark {
            if has_update {
                query_builder.push(", ");
            }
            query_builder.push("remark = ").push_bind(remark);
            has_update = true;
        }

        if let Some(del_flag) = &self.del_flag {
            if has_update {
                query_builder.push(", ");
            }
            query_builder.push("del_flag = ").push_bind(del_flag);
            has_update = true;
        }

        if has_update {
            query_builder
                .push(" WHERE clothing_id = ")
                .push_bind(self.clothing_id)
                .push(" RETURNING *");
            let updated_clothing = query_builder
                .build_query_as::<Clothing>()
                .fetch_one(pool)
                .await?;
            Ok(updated_clothing)
        } else {
            Ok(Clothing::from(self))
        }
    }
}

/// tauri commands
///
/// 分页查询衣物列表
#[tauri::command]
pub async fn list_clothing_pagination(
    state: State<'_, AppState>,
    page_params: PageParams,
    clothing: Clothing,
) -> Result<PageResult<Clothing>> {
    clothing.get_list(&state.pool, page_params).await
}

#[tauri::command]
pub async fn list_clothing_all(
    state: State<'_, AppState>,
    clothing: Clothing,
) -> Result<Vec<Clothing>> {
    clothing.query_all_clothing(&state.pool).await
}

#[tauri::command]
pub async fn add_clothing(state: State<'_, AppState>, mut clothing: Clothing) -> Result<Clothing> {
    if clothing.clothing_name.is_none()
        || clothing.clothing_name.as_ref().unwrap().trim().is_empty()
    {
        return Err(Error::with_details(ErrorKind::BadRequest, "衣物名不能为空"));
    }

    // 生成衣物编码
    let code = utils::gen_code(clothing.clothing_name.as_ref().unwrap());

    clothing.clothing_number =
        Some(Clothing::select_next_num(&state.pool, &format!("{code}-")).await?);

    let clothing = clothing.add(&state.pool).await?;

    Ok(clothing)
}

#[tauri::command]
pub async fn get_clothing_by_id(state: State<'_, AppState>, id: i64) -> Result<Option<Clothing>> {
    Clothing::get_by_id(&state.pool, id).await
}

#[tauri::command]
pub async fn update_clothing(state: State<'_, AppState>, clothing: Clothing) -> Result<Clothing> {
    clothing.update(&state.pool).await
}

#[tauri::command]
pub async fn soft_delete_clothing(state: State<'_, AppState>, id: i64) -> Result<u64> {
    Clothing::soft_delete(&state.pool, id).await
}

#[tauri::command]
pub async fn update_clothing_ref_num(
    state: State<'_, AppState>,
    ref_num: i64,
    clothing_ids: Vec<i64>,
) -> Result<()> {
    Clothing::update_ref_num(&state.pool, ref_num, clothing_ids).await
}

#[tauri::command]
pub async fn clothing_name_exists(state: State<'_, AppState>, clothing_name: &str) -> Result<bool> {
    Clothing::exists_by_clothing_name(&state.pool, clothing_name).await
}

#[tauri::command]
pub async fn delete_clothing_batch(state: State<'_, AppState>, ids: Vec<i64>) -> Result<bool> {
    let mut tr = state.pool.begin().await?;
    let result = Clothing::delete_batch(&mut tr, &ids).await?;
    tr.commit().await?;
    Ok(result)
}

/// 在内存中设置和初始化一个SQLite数据库，用于测试目的。
///
/// 此函数会创建一个SQLite数据库连接池，并在该池中执行SQL语句以创建一个名为`clothing`的表。
/// 表结构包括多个字段，用于存储衣物信息，如衣物名称、衣物编号、显示顺序等。
/// 此外，函数确保通过在内存中创建数据库来隔离测试环境，避免影响实际数据库。
///
/// 返回值:
/// - `Pool<Sqlite>`: 一个SQLite数据库连接池，用于管理和执行与数据库的异步操作。
#[allow(dead_code)]
async fn setup_test_db() -> Pool<Sqlite> {
    // 创建一个SQLite数据库连接池，连接到内存数据库
    let pool = SqlitePool::connect(":memory:").await.unwrap();

    // 在数据库中执行SQL语句以创建`clothing`表，包含多个字段用于存储衣物信息
    pool.execute(
        r#"
            CREATE TABLE IF NOT EXISTS clothing
            (
                clothing_id         INTEGER PRIMARY KEY AUTOINCREMENT,
                clothing_category   VARCHAR(3)  NOT NULL,
                clothing_number     VARCHAR(30) NOT NULL,
                clothing_style      VARCHAR(3)  NOT NULL,
                clothing_name       VARCHAR(50) NOT NULL,
                clothing_base_price DOUBLE      NOT NULL,
                clothing_min_price  DOUBLE      NOT NULL,
                order_num           INTEGER              DEFAULT 0,
                clothing_degree     INTEGER              DEFAULT 0,
                hang_type           CHAR(1)     NOT NULL DEFAULT '1',
                del_flag            CHAR(1)              DEFAULT '0',
                remark              VARCHAR(500)
            );
            "#,
    )
    .await
    .unwrap();

    // 返回SQLite数据库连接池
    pool
}

// #[cfg(test)]
// mod command_tests {
//     use crate::create_app;
//     use crate::db::clothing::{setup_test_db, Clothing};
//     use crate::db::DbPool;
//     use tauri::test::mock_builder;

//     /// there is an issue in windows platform: STATUS_ENTRYPOINT_NOT_FOUND,
//     /// so we skip it
//     #[tokio::test]
//     async fn test_add_clothing() {
//         // 创建一个新的 clothing 实例
//         let clothing = Clothing {
//             clothing_name: "Test clothing".to_string(),
//             ..Default::default()
//         };

//         // 验证结果
//         let app = create_app(mock_builder(), DbPool::new(setup_test_db().await));
//         let webview = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
//             .build()
//             .unwrap();

//         // run the `ping` command and assert it returns `pong`
//         let res = tauri::test::get_ipc_response(
//             &webview,
//             tauri::webview::InvokeRequest {
//                 cmd: "add_clothing".into(),
//                 callback: tauri::ipc::CallbackFn(0),
//                 error: tauri::ipc::CallbackFn(1),
//                 url: "http://tauri.localhost".parse().unwrap(),
//                 body: tauri::ipc::InvokeBody::Json(serde_json::to_value(&clothing).unwrap()),
//                 headers: Default::default(),
//                 invoke_key: tauri::test::INVOKE_KEY.to_string(),
//             },
//         )
//             .map(|b| b.deserialize::<Clothing>().unwrap())
//             .unwrap();
//         assert_eq!(res.clothing_name, clothing.clothing_name);
//     }
// }
