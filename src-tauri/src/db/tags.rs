// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::tags;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: tags = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};
use sqlx::{Executor, Pool, QueryBuilder, Sqlite, SqlitePool};
use tauri::State;

use super::{DbPool, PageParams, PageResult};
use crate::error::{Error, ErrorKind, Result};
use crate::utils;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Tags {
    /// 标签唯一标识ID
    #[serde(default)]
    tag_id: i64,
    /// 标签名称
    #[serde(default)]
    tag_name: String,
    /// 标签编码
    #[serde(default)]
    tag_number: String,
    /// 标签类别，001 洗前瑕疵，002 洗后预估，003 衣物颜色等
    #[serde(default)]
    tag_order: Option<String>,
    /// 显示顺序，默认显示顺序
    #[serde(default)]
    order_num: i64,
    /// 标签状态，0正常，1停用
    #[serde(default = "status_default")]
    status: String,
    /// 使用次数，实际被使用的次数，当该值不为0时，将优先将按照该值排序，然后才是显示顺序排序
    #[serde(default)]
    ref_num: i64,
    /// 备注
    #[serde(default)]
    remark: Option<String>,
    /// 删除标志（0代表存在 2代表删除）
    del_flag: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TagParam {
    /// 标签唯一标识ID
    #[serde(default)]
    tag_id: Option<i64>,
    /// 标签名称
    #[serde(default)]
    tag_name: Option<String>,
    /// 标签编码
    #[serde(default)]
    tag_number: Option<String>,
    /// 标签类别，001 洗前瑕疵，002 洗后预估，003 衣物颜色等
    #[serde(default)]
    tag_order: Option<String>,
    /// 显示顺序，默认显示顺序
    #[serde(default)]
    order_num: Option<i64>,
    /// 标签状态，0正常，1停用
    #[serde(default)]
    status: Option<String>,
    /// 使用次数，实际被使用的次数，当该值不为0时，将优先将按照该值排序，然后才是显示顺序排序
    #[serde(default)]
    ref_num: Option<i64>,
    /// 备注
    #[serde(default)]
    remark: Option<String>,
    #[serde(default)]
    del_flag: Option<String>,
}

fn status_default() -> String {
    "0".to_string()
}

impl Tags {
    pub async fn add(&self, pool: &Pool<Sqlite>) -> Result<Self> {
        let result = sqlx::query_as::<_, Tags>(
            "INSERT INTO tags (tag_name, tag_number, tag_order, order_num, ref_num, status, remark, del_flag)
             VALUES (?, ?, ?, ?, ?, ?, ?, '0')
             RETURNING *"
        )
            .bind(&self.tag_name)
            .bind(&self.tag_number)
            .bind(&self.tag_order)
            .bind(&self.order_num)
            .bind(&self.ref_num)
            .bind(&self.status)
            .bind(&self.remark)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    /// 查询标签列表
    pub async fn query_tags(
        pool: &Pool<Sqlite>,
        query_params: PageParams,
        tag: TagParam,
    ) -> Result<PageResult<Tags>> {
        let mut query_builder = QueryBuilder::<Sqlite>::new("SELECT * FROM tags WHERE del_flag = '0' ");

        if let Some(tag_id) = &tag.tag_id {
            query_builder.push(" AND tag_id = ").push_bind(tag_id);
        }

        if let Some(tag_name) = &tag.tag_name {
            query_builder.push(" AND tag_name LIKE ").push_bind(format!("%{}%", tag_name));
        }

        if let Some(tag_order) = &tag.tag_order {
            query_builder.push(" AND tag_order = ").push_bind(tag_order);
        }

        if let Some(tag_number) = &tag.tag_number {
            query_builder.push(" AND tag_number = ").push_bind(tag_number);
        }

        if let Some(ref_num) = &tag.ref_num {
            query_builder.push(" AND ref_num = ").push_bind(ref_num);
        }

        if let Some(status) = &tag.status {
            query_builder.push(" AND status = ").push_bind(status);
        }

        if let Some(remark) = &tag.remark {
            query_builder.push(" AND remark = ").push_bind(remark);
        }

        // Add pagination
        query_builder.push(" LIMIT ").push_bind(query_params.page_size);
        query_builder.push(" OFFSET ").push_bind(query_params.page * query_params.page_size);

        let query = query_builder.build_query_as::<Tags>();
        let results = query.fetch_all(pool).await?;

        Ok(PageResult {
            total: results.len(),
            rows: results,
        })
    }

    /// 查询是否标签编码是否已经存在
    /// 这里存在一个潜在的bug：如果超过四位，那么会变成1000，进而产生唯一索引错误
    pub async fn select_next_num(pool: &Pool<Sqlite>, prefix: &str) -> Result<String> {
        let next_tag_number: (String,) = sqlx::query_as(
            r#"
        SELECT
            CASE
                WHEN MAX_number IS NULL THEN printf('%s%04d', ?1, 1)
                ELSE printf('%s%04d', ?1, MAX_number + 1)
            END AS next_tag_number
        FROM (
            SELECT MAX(CAST(SUBSTR(tag_number, LENGTH(?1) + 1) AS INTEGER)) AS MAX_number
            FROM tags
            WHERE tag_number LIKE (?1 || '%')
        ) AS subquery;
        "#
        )
            .bind(prefix)
            .fetch_one(pool)
            .await?;

        Ok(next_tag_number.0)
    }

    // 根据ID查询标签
    pub async fn get_by_id(pool: &Pool<Sqlite>, tag_id: i64) -> Result<Self> {
        let result = sqlx::query_as::<_, Tags>("SELECT * FROM tags WHERE tag_id = ? AND del_flag = '0'")
            .bind(tag_id)
            .fetch_one(pool)
            .await?;
        Ok(result)
    }

    // 更新标签
    pub async fn update(&self, pool: &Pool<Sqlite>) -> Result<Self> {
        let result = sqlx::query_as::<_, Tags>(
            "UPDATE tags
             SET tag_name = ?, tag_number = ?, tag_order = ?, order_num = ?, ref_num = ?, status = ?, remark = ?
             WHERE tag_id = ?
             RETURNING *"
        )
            .bind(&self.tag_name)
            .bind(&self.tag_number)
            .bind(&self.tag_order)
            .bind(&self.order_num)
            .bind(&self.ref_num)
            .bind(&self.status)
            .bind(&self.remark)
            .bind(&self.tag_id)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    // 软删除标签
    pub async fn soft_delete(pool: &Pool<Sqlite>, tag_id: i64) -> Result<u64> {
        let result = sqlx::query("UPDATE tags SET del_flag = '2' WHERE tag_id = ?")
            .bind(tag_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }

    // 增加ref_num
    pub async fn increment_ref_num(pool: &Pool<Sqlite>, tag_id: i64) -> Result<Self> {
        let result = sqlx::query_as::<_, Tags>(
            "UPDATE tags SET ref_num = ref_num + 1 WHERE tag_id = ? RETURNING *"
        )
            .bind(tag_id)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    // 检查tag_name是否已经存在
    pub async fn exists_by_tag_name(pool: &Pool<Sqlite>, tag_name: &str) -> Result<bool> {
        let result = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM tags WHERE tag_name = $1 AND del_flag = '0')")
            .bind(tag_name)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    // 批量删除接口

    pub async fn batch_soft_delete(pool: &Pool<Sqlite>, tag_ids: &[i64]) -> Result<u64> {
        if tag_ids.is_empty() {
            return Ok(0);
        }

        let mut query_builder = QueryBuilder::<Sqlite>::new("UPDATE tags SET del_flag = '2' WHERE tag_id IN (");

        for (i, tag_id) in tag_ids.iter().enumerate() {
            if i > 0 {
                query_builder.push(", ");
            }
            query_builder.push_bind(tag_id);
        }

        query_builder.push(")");

        let query = query_builder.build();
        let result = query.execute(pool).await?;

        Ok(result.rows_affected())
    }
}

#[tauri::command]
pub async fn add_tag(state: State<'_, DbPool>, mut tag: Tags) -> Result<Tags> {
    if tag.tag_name.trim().is_empty() {
        return Err(Error::with_details(ErrorKind::BadRequest, "标签名不能为空"));
    }

    // 生成标签编码
    let code = utils::gen_code(&tag.tag_name);

    tag.tag_number = format!("{code}-{}", Tags::select_next_num(&state.0, &code).await?);

    tag.add(&state.0).await?;

    Ok(tag)
}

#[tauri::command]
pub async fn get_tag_by_id(state: State<'_, DbPool>, id: i64) -> Result<Tags> {
    Tags::get_by_id(&state.0, id).await
}

#[tauri::command]
pub async fn update_tag(state: State<'_, DbPool>, tag: Tags) -> Result<Tags> {
    tag.update(&state.0).await
}

#[tauri::command]
pub async fn soft_delete_tag(state: State<'_, DbPool>, id: i64) -> Result<u64> {
    Tags::soft_delete(&state.0, id).await
}

#[tauri::command]
pub async fn increase_ref_num(state: State<'_, DbPool>, id: i64) -> Result<Tags> {
    Tags::increment_ref_num(&state.0, id).await
}

#[tauri::command]
pub async fn tag_name_exists(state: State<'_, DbPool>, tag_name: &str) -> Result<bool> {
    Tags::exists_by_tag_name(&state.0, tag_name).await
}

#[tauri::command]
pub async fn delete_tags_batch(state: State<'_, DbPool>, ids: Vec<i64>) -> Result<u64> {
    Tags::batch_soft_delete(&state.0, &ids).await
}


async fn setup_test_db() -> Pool<Sqlite> {
    let pool = SqlitePool::connect(":memory:").await.unwrap();

    pool.execute(
        r#"
            CREATE TABLE tags (
                tag_id INTEGER PRIMARY KEY,
                tag_name TEXT,
                tag_number TEXT UNIQUE,
                tag_order TEXT,
                order_num INTEGER,
                status TEXT DEFAULT '0',
                ref_num INTEGER,
                remark TEXT,
                del_flag TEXT DEFAULT '0'
            );
            "#,
    )
        .await
        .unwrap();

    pool
}
#[cfg(test)]
mod command_tests {
    use crate::create_app;
    use crate::db::tags::{setup_test_db, Tags};
    use crate::db::DbPool;
    use tauri::test::mock_builder;


    /// there is an issue in windows platform: STATUS_ENTRYPOINT_NOT_FOUND,
    /// so we skip it
    #[tokio::test]
    async fn test_add_tag() {
        // 创建一个新的 Tags 实例
        let tag = Tags {
            tag_name: "Test Tag".to_string(),
            ..Default::default()
        };

        // 验证结果
        let app = create_app(mock_builder(), DbPool::new(setup_test_db().await));
        let webview = tauri::WebviewWindowBuilder::new(&app, "main", Default::default()).build().unwrap();

        // run the `ping` command and assert it returns `pong`
        let res = tauri::test::get_ipc_response(
            &webview,
            tauri::webview::InvokeRequest {
                cmd: "add_tag".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                url: "http://tauri.localhost".parse().unwrap(),
                body: tauri::ipc::InvokeBody::Json(serde_json::to_value(&tag).unwrap()),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        ).map(|b| b.deserialize::<Tags>().unwrap()).unwrap();
        assert_eq!(res.tag_name, tag.tag_name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_tag() {
        let pool = setup_test_db().await;

        let tag = Tags {
            tag_id: 0,
            tag_name: "Test Tag".to_string(),
            tag_number: "T001".to_string(),
            tag_order: Some("001".to_string()),
            order_num: 1,
            status: "0".to_string(),
            ref_num: 0,
            remark: Some("A test tag".to_string()),
            del_flag: "0".to_string(),
        };

        let inserted_tag = tag.add(&pool).await.unwrap();

        assert_eq!(inserted_tag.tag_name, "Test Tag");
        assert_eq!(inserted_tag.tag_number, "T001");
    }

    #[tokio::test]
    async fn test_query_tags() {
        let pool = setup_test_db().await;

        // Insert sample tags
        let tag1 = Tags {
            tag_name: "Tag1".to_string(),
            tag_number: "T-001".to_string(),
            tag_order: Some("001".to_string()),
            status: "0".to_string(),
            remark: Some("Remark1".to_string()),
            ..Default::default()
        };
        tag1.add(&pool).await.unwrap();
        let tag2 = Tags {
            tag_name: "Tag2".to_string(),
            tag_number: "T-002".to_string(),
            tag_order: Some("002".to_string()),
            status: "0".to_string(),
            remark: Some("Remark2".to_string()),
            ..Default::default()
        };
        tag2.add(&pool).await.unwrap();
        let tag3 = Tags {
            tag_name: "Tag3".to_string(),
            tag_number: "T-003".to_string(),
            tag_order: Some("003".to_string()),
            status: "1".to_string(),
            ref_num: 2,
            remark: Some("Remark3".to_string()),
            ..Default::default()
        };
        tag3.add(&pool).await.unwrap();


        // sqlx::query("INSERT INTO tags (tag_name, tag_number, tag_order, ref_num, status, remark, del_flag) VALUES (?, ?, ?, ?, ?, ?, ?)")
        //     .bind("Tag3")
        //     .bind("T003")
        //     .bind(Some("003"))
        //     .bind(2)
        //     .bind("1")
        //     .bind(Some("Remark3"))
        //     .bind("1")
        //     .execute(&pool)
        //     .await
        //     .unwrap();

        // Test 1: Basic filtering by tag_name
        let query_params = PageParams { page: 0, page_size: 10 };
        let tag_param = TagParam {
            tag_name: Some("Tag1".to_string()),
            ..Default::default()
        };
        let result = Tags::query_tags(&pool, query_params.clone(), tag_param).await.unwrap();
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0].tag_name, "Tag1");

        // Test 2: Filtering by tag_order and status
        let tag_param = TagParam {
            tag_order: Some("002".to_string()),
            status: Some("0".to_string()),
            ..Default::default()
        };
        let result = Tags::query_tags(&pool, query_params.clone(), tag_param).await.unwrap();
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0].tag_number, "T-002");

        // Test 3: Filtering by del_flag and ref_num
        let tag_param = TagParam {
            del_flag: Some("0".to_string()),
            ref_num: Some(2),
            ..Default::default()
        };
        let result = Tags::query_tags(&pool, query_params.clone(), tag_param).await.unwrap();
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0].tag_number, "T-003");

        // Test 4: Pagination - request only first page with 2 items per page
        let query_params = PageParams { page: 0, page_size: 2 };
        let tag_param = TagParam::default();
        let result = Tags::query_tags(&pool, query_params.clone(), tag_param).await.unwrap();
        assert_eq!(result.rows.len(), 2); // Should return first 2 items

        // Test 5: Pagination - request second page with 2 items per page
        let query_params = PageParams { page: 1, page_size: 2 };
        let result = Tags::query_tags(&pool, query_params, TagParam::default()).await.unwrap();
        assert_eq!(result.rows.len(), 1); // Should return only the last item (Tag3)

        // Test 6: Fuzzy search for tag_name containing "Tag"
        let query_params = PageParams { page: 0, page_size: 10 };
        let tag_param = TagParam {
            tag_name: Some("Tag".to_string()),
            ..Default::default()
        };
        let result = Tags::query_tags(&pool, query_params, tag_param).await.unwrap();
        assert_eq!(result.rows.len(), 3); // Should match all three tags
    }

    #[tokio::test]
    async fn test_select_next_num() {
        let pool = setup_test_db().await;

        // Insert a tag with a specific number pattern
        sqlx::query("INSERT INTO tags (tag_name, tag_number, ref_num, status) VALUES (?, ?, ?, ?)")
            .bind("Tag1")
            .bind("P001")
            .bind(0)
            .bind("0")
            .execute(&pool)
            .await
            .unwrap();

        let prefix = "P";
        let next_number = Tags::select_next_num(&pool, prefix).await.unwrap();

        assert_eq!(next_number, "P0002");
    }

    #[tokio::test]
    async fn test_get_by_id() {
        let pool = setup_test_db().await;
        let new_tag = Tags {
            tag_name: "Test Tag".to_string(),
            tag_number: "001".to_string(),
            ..Default::default()
        };
        let added_tag = new_tag.add(&pool).await.unwrap();
        let retrieved_tag = Tags::get_by_id(&pool, added_tag.tag_id).await.unwrap();
        assert_eq!(retrieved_tag.tag_id, added_tag.tag_id);
    }

    #[tokio::test]
    async fn test_update_tag() {
        let pool = setup_test_db().await;
        let new_tag = Tags {
            tag_name: "Test Tag".to_string(),
            tag_number: "001".to_string(),
            ..Default::default()
        };
        let added_tag = new_tag.add(&pool).await.unwrap();
        let updated_tag = Tags {
            tag_id: added_tag.tag_id,
            tag_name: "Updated Tag".to_string(),
            ..added_tag
        };
        let retrieved_tag = updated_tag.update(&pool).await.unwrap();
        assert_eq!(retrieved_tag.tag_name, "Updated Tag");
    }

    #[tokio::test]
    async fn test_soft_delete() {
        let pool = setup_test_db().await;
        let new_tag = Tags {
            tag_name: "Test Tag".to_string(),
            tag_number: "001".to_string(),
            ..Default::default()
        };
        let added_tag = new_tag.add(&pool).await.unwrap();
        let affected_rows = Tags::soft_delete(&pool, added_tag.tag_id).await.unwrap();
        assert_eq!(affected_rows, 1);
    }

    #[tokio::test]
    async fn test_increment_ref_num() {
        let pool = setup_test_db().await;
        let new_tag = Tags {
            tag_name: "Test Tag".to_string(),
            tag_number: "001".to_string(),
            ref_num: 1,
            ..Default::default()
        };
        let added_tag = new_tag.add(&pool).await.unwrap();
        let incremented_tag = Tags::increment_ref_num(&pool, added_tag.tag_id).await.unwrap();
        assert_eq!(incremented_tag.ref_num, 2);
    }

    #[tokio::test]
    async fn test_exists_by_tag_name() {
        let pool = setup_test_db().await;
        let new_tag = Tags {
            tag_name: "Test Tag".to_string(),
            tag_number: "001".to_string(),
            ..Default::default()
        };
        new_tag.add(&pool).await.unwrap();
        let exists = Tags::exists_by_tag_name(&pool, "Test Tag").await.unwrap();
        assert!(exists);
    }

    #[tokio::test]
    async fn test_batch_soft_delete() {
        let pool = setup_test_db().await;
        let tag1 = Tags {
            tag_name: "Test Tag 1".to_string(),
            tag_number: "001".to_string(),
            ..Default::default()
        };
        let tag2 = Tags {
            tag_name: "Test Tag 2".to_string(),
            tag_number: "002".to_string(),
            ..Default::default()
        };
        let added_tag1 = tag1.add(&pool).await.unwrap();
        let added_tag2 = tag2.add(&pool).await.unwrap();

        let tag_ids = vec![added_tag1.tag_id, added_tag2.tag_id];
        let affected_rows = Tags::batch_soft_delete(&pool, &tag_ids).await.unwrap();
        assert_eq!(affected_rows, 2);
    }
}
