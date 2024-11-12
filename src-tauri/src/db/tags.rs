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

impl From<TagParam> for Tags {
    fn from(param: TagParam) -> Self {
        Tags {
            tag_id: param.tag_id.unwrap_or_default(),
            tag_name: param.tag_name.unwrap_or_default(),
            tag_number: param.tag_number.unwrap_or_default(),
            tag_order: param.tag_order,
            order_num: param.order_num.unwrap_or_default(),
            status: param.status.unwrap_or_else(status_default),
            ref_num: param.ref_num.unwrap_or_default(),
            remark: param.remark,
            del_flag: param.del_flag.unwrap_or_else(|| "0".to_string()),
        }
    }
}

impl Tags {
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
        "#,
        )
            .bind(prefix)
            .fetch_one(pool)
            .await?;

        Ok(next_tag_number.0)
    }

    // 根据ID查询标签
    pub async fn get_by_id(pool: &Pool<Sqlite>, tag_id: i64) -> Result<Option<Self>> {
        let result =
            sqlx::query_as::<_, Tags>("SELECT * FROM tags WHERE tag_id = ? AND del_flag = '0'")
                .bind(tag_id)
                .fetch_optional(pool)
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
            "UPDATE tags SET ref_num = ref_num + 1 WHERE tag_id = ? RETURNING *",
        )
            .bind(tag_id)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    // 增加ref_num
    pub async fn update_ref_num(pool: &Pool<Sqlite>, ref_num: i64, tag_ids: Vec<i64>) -> Result<()> {
        let mut query_builder = QueryBuilder::<Sqlite>::new("UPDATE tags SET ref_num = ");
        query_builder.push_bind(ref_num).push(" WHERE tag_id IN (");

        let mut separated = query_builder.separated(", ");
        for tag_id in &tag_ids {
            separated.push_bind(tag_id);
        }
        query_builder.push(")");

        query_builder.build().execute(pool).await?;
        Ok(())
    }

    // 检查tag_name是否已经存在
    pub async fn exists_by_tag_name(pool: &Pool<Sqlite>, tag_name: &str) -> Result<bool> {
        let result = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM tags WHERE tag_name = $1 AND del_flag = '0')",
        )
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

        let mut query_builder =
            QueryBuilder::<Sqlite>::new("UPDATE tags SET del_flag = '2' WHERE tag_id IN (");

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

impl TagParam {
    /// 异步向数据库中添加标签信息，并返回新添加的标签对象。
    ///
    /// # Parameters
    ///
    /// * `pool`: 数据库连接池的引用，用于执行数据库操作。
    ///
    /// # Returns
    ///
    /// 返回一个结果类型，包含新添加的标签信息。
    pub async fn add(self, pool: &Pool<Sqlite>) -> Result<Tags> {
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

    /// 根据标签参数应用过滤条件到查询构建器中
    ///
    /// # Parameters
    ///
    /// * `query_builder`: &mut QueryBuilder<Sqlite> - 查询构建器的可变引用，用于构建SQL查询
    /// * `tag`: &TagParam - 标签参数的引用，包含多个可选字段，用于指定查询条件
    ///
    /// # Description
    ///
    /// 该函数根据`TagParam`结构体中的字段，动态地向`QueryBuilder`中添加查询条件
    /// 每个字段对应数据库中的一个列，如果字段有值，则相应的查询条件会被添加到查询构建器中
    fn apply_tag_filters<'a>(&'a self, query_builder: &mut QueryBuilder<'a, Sqlite>) {
        if let Some(tag_id) = &self.tag_id {
            query_builder.push(" AND tag_id = ").push_bind(tag_id);
        }

        if let Some(tag_name) = &self.tag_name {
            query_builder
                .push(" AND tag_name LIKE ")
                .push_bind(format!("%{}%", tag_name));
        }

        if let Some(tag_order) = &self.tag_order {
            query_builder.push(" AND tag_order = ").push_bind(tag_order);
        }

        if let Some(tag_number) = &self.tag_number {
            query_builder
                .push(" AND tag_number LIKE ")
                .push_bind(format!("%{}%", tag_number));
        }

        if let Some(ref_num) = &self.ref_num {
            query_builder.push(" AND ref_num = ").push_bind(ref_num);
        }

        if let Some(status) = &self.status {
            query_builder.push(" AND status = ").push_bind(status);
        }

        if let Some(remark) = &self.remark {
            query_builder.push(" AND remark = ").push_bind(remark);
        }
    }

    /// 异步计算与指定标签参数匹配的标签数量
    ///
    /// 该函数接收一个数据库连接池和一个标签参数对象，用于构建查询以统计符合条件的标签数量
    /// 它首先构建一个查询字符串，然后根据传入的标签参数应用必要的过滤条件，最后执行查询并返回结果
    ///
    /// # Parameters
    /// - `pool`: &Pool<Sqlite> - 数据库连接池
    /// - `tag`: &TagParam - 标签参数引用，用于构建查询过滤条件
    ///
    /// # Returns
    /// - `Result<u64>` - 返回一个结果类型，包含符合条件的标签数量
    async fn count_tags(&self, pool: &Pool<Sqlite>) -> Result<u64> {
        let mut query_builder =
            QueryBuilder::<Sqlite>::new("SELECT COUNT(*) FROM tags WHERE del_flag = '0'");
        self.apply_tag_filters(&mut query_builder);
        let query = query_builder.build_query_scalar::<u64>();
        Ok(query.fetch_one(pool).await?)
    }

    /// 异步构建标签查询
    ///
    /// 该函数根据提供的标签参数和分页选项，查询未删除的标签
    /// 它首先构建一个SQL查询，然后根据需要应用过滤和分页
    ///
    /// # 参数
    /// - `pool`: 数据库连接池引用
    /// - `tag`: 标签参数，用于过滤查询结果
    /// - `with_pagination`: 布尔值，指示是否应用分页
    /// - `page_params`: 可选的分页参数，当`with_pagination`为`true`时必须提供
    ///
    /// # 返回
    /// 返回一个结果，包含一个标签列表
    /// 如果查询成功，返回Ok(Vec<Tags>)；如果查询失败，返回错误
    async fn build_tag_query(&self, pool: &Pool<Sqlite>, with_pagination: bool, page_params: Option<PageParams>) -> Result<Vec<Tags>> {
        let mut query_builder =
            QueryBuilder::<Sqlite>::new("SELECT * FROM tags WHERE del_flag = '0'");
        self.apply_tag_filters(&mut query_builder);

        query_builder.push(" ORDER BY ref_num DESC, order_num ASC ");

        if with_pagination {
            if let Some(params) = page_params {
                query_builder.push(" LIMIT ").push_bind(params.page_size);
                query_builder
                    .push(" OFFSET ")
                    .push_bind((params.page - 1) * params.page_size);
            }
        }

        let query = query_builder.build_query_as::<Tags>();
        Ok(query.fetch_all(pool).await?)
    }

    /// 异步查询标签
    ///
    /// 本函数通过构建针对Sqlite数据库的查询语句，根据提供的分页参数和标签参数，来获取标签数据
    /// 它首先调用`build_tag_query`方法来构建查询，然后执行查询并返回查询结果
    ///
    /// # 参数
    /// - `pool`: 数据库连接池的引用，用于执行数据库操作
    /// - `query_params`: 分页参数，用于指定查询的页码和每页的记录数
    /// - `tag`: 标签参数，用于指定查询的标签条件
    ///
    /// # 返回
    /// 返回一个结果类型，其中包含一个分页结果类型`PageResult<Tags>`，里面有标签的总数和标签列表
    ///
    /// # 错误处理
    /// 如果在构建查询或执行查询过程中发生错误，将返回一个`Result`类型的错误
    pub async fn query_tags(
        &self,
        pool: &Pool<Sqlite>,
        query_params: PageParams,
    ) -> Result<PageResult<Tags>> {
        // 构建并执行标签查询
        let rows = self.build_tag_query(pool, true, Some(query_params)).await?;
        let total = self.count_tags(pool).await?;
        // 返回分页结果，其中包含标签的总数和标签列表
        Ok(PageResult { total, rows })
    }

    /// 查询所有标签
    ///
    /// 此函数用于根据提供的标签参数查询数据库中的所有相关标签它利用异步SQLITE数据库连接池来执行查询
    /// 主要用途是在需要根据特定条件获取一组标签时使用
    ///
    /// # 参数
    ///
    /// * `pool` - 一个引用，指向SQLITE数据库的连接池，用于执行数据库操作
    /// * `tag` - 一个TagParam类型的实例，包含查询所需的参数信息
    ///
    /// # 返回值
    ///
    /// 返回一个Result类型，包含一个Tags类型的向量，代表查询到的标签列表如果查询失败，将返回一个sqlx::Error
    pub async fn query_all_tags(&self, pool: &Pool<Sqlite>) -> Result<Vec<Tags>> {
        // 调用build_tag_query方法来构建和执行查询，这里不需要传递额外的参数，因为查询条件已经包含在tag参数中
        self.build_tag_query(pool, false, None).await
    }

    /// 异步更新标签信息
    ///
    /// 此函数负责将当前标签对象的更改更新到数据库中的tags表它动态构建一个UPDATE SQL语句，
    /// 包含需要更新的字段，并执行更新操作如果没有任何字段需要更新，则直接返回当前对象作为结果
    ///
    /// # 参数
    ///
    /// - `pool`: 数据库连接池引用，用于执行SQL查询
    ///
    /// # 返回
    ///
    /// - `Result<Tags>`: 更新后的标签对象如果无字段需要更新，则返回当前对象
    pub async fn update(self, pool: &Pool<Sqlite>) -> Result<Tags> {
        let mut query_builder = QueryBuilder::<Sqlite>::new("UPDATE tags SET ");
        let mut has_update = false;

        if let Some(tag_name) = &self.tag_name {
            query_builder.push("tag_name = ").push_bind(tag_name);
            has_update = true;
        }
        if let Some(tag_number) = &self.tag_number {
            if has_update { query_builder.push(", "); }
            query_builder.push("tag_number = ").push_bind(tag_number);
            has_update = true;
        }
        if let Some(tag_order) = &self.tag_order {
            if has_update { query_builder.push(", "); }
            query_builder.push("tag_order = ").push_bind(tag_order);
            has_update = true;
        }
        if let Some(order_num) = &self.order_num {
            if has_update { query_builder.push(", "); }
            query_builder.push("order_num = ").push_bind(order_num);
            has_update = true;
        }
        if let Some(ref_num) = &self.ref_num {
            if has_update { query_builder.push(", "); }
            query_builder.push("ref_num = ").push_bind(ref_num);
            has_update = true;
        }
        if let Some(status) = &self.status {
            if has_update { query_builder.push(", "); }
            query_builder.push("status = ").push_bind(status);
            has_update = true;
        }
        if let Some(remark) = &self.remark {
            if has_update { query_builder.push(", "); }
            query_builder.push("remark = ").push_bind(remark);
            has_update = true;
        }
        if let Some(del_flag) = &self.del_flag {
            if has_update { query_builder.push(", "); }
            query_builder.push("del_flag = ").push_bind(del_flag);
            has_update = true;
        }

        if has_update {
            query_builder
                .push(" WHERE tag_id = ")
                .push_bind(self.tag_id)
                .push(" RETURNING *");
            let updated_tag = query_builder
                .build_query_as::<Tags>()
                .fetch_one(pool)
                .await?;
            Ok(updated_tag)
        } else {
            Ok(Tags::from(self))
        }
    }
}

/// tauri commands
///
/// 分页查询标签列表
#[tauri::command]
pub async fn list_pagination(
    state: State<'_, DbPool>,
    page_params: PageParams,
    tag: TagParam,
) -> Result<PageResult<Tags>> {
    tag.query_tags(&state.0, page_params).await
}

/// 异步获取所有标签信息
///
/// 本函数通过接收一个数据库连接池状态和一个标签参数，来查询并返回所有相关的标签信息
/// 它主要用于处理和响应来自前端的获取标签列表的请求，在后端进行数据处理和查询
///
/// # 参数
/// - `state`: 包含数据库连接池的状态，用于执行数据库查询
/// - `tag`: 标签参数，用于指定查询的条件
///
/// # 返回
/// 返回一个结果，其中包含一个标签信息的向量，如果查询成功，否则包含一个错误
#[tauri::command]
pub async fn list_all(state: State<'_, DbPool>, tag: TagParam) -> Result<Vec<Tags>> {
    tag.query_all_tags(&state.0).await
}

///
/// 异步添加标签
///
/// 此函数通过接收一个标签对象和一个数据库连接池的状态来实现标签的添加操作
/// 它首先检查标签名称是否为空，如果为空则返回一个错误
/// 然后，它生成标签的唯一编码，并将其与数据库中同类型标签的数量结合，形成标签编号
/// 最后，将带有新编号的标签对象添加到数据库中
///
/// # 参数
/// - `state`: 包含数据库连接池的程序状态
/// - `tag`: 待添加的标签对象
///
/// # 返回
/// - `Ok(Tags)`: 添加成功后，返回带有数据库分配编号的标签对象
/// - `Err(Error)`: 如果标签名为空或数据库操作失败，返回相应的错误
#[tauri::command]
pub async fn add_tag(state: State<'_, DbPool>, mut tag: TagParam) -> Result<Tags> {
    if tag.tag_name.is_none() || tag.tag_name.as_ref().unwrap().trim().is_empty() {
        return Err(Error::with_details(ErrorKind::BadRequest, "标签名不能为空"));
    }

    // 生成标签编码
    let code = utils::gen_code(tag.tag_name.as_ref().unwrap());

    tag.tag_number = Some(Tags::select_next_num(&state.0, &format!("{code}-")).await?);

    let tag = tag.add(&state.0).await?;

    Ok(tag)
}

/// 根据ID获取标签信息
///
/// 该函数通过ID异步查询数据库中的标签信息，并返回查询结果
/// 主要用于在前端请求时，根据特定的标签ID获取详细的标签信息
///
/// # 参数
///
/// * `state` - 包含数据库连接池的程序状态，用于执行数据库操作
/// * `id` - 需要查询的标签ID
///
/// # 返回值
///
/// 返回一个结果类型，包含可能的标签信息（`Tags`）或错误信息
#[tauri::command]
pub async fn get_tag_by_id(state: State<'_, DbPool>, id: i64) -> Result<Option<Tags>> {
    Tags::get_by_id(&state.0, id).await
}

/// 异步更新标签信息
///
/// 该函数通过接收一个标签对象和一个数据库连接池状态对象，异步地更新标签的信息
/// 它主要用于在用户界面对标签进行修改后，将这些修改保存到数据库中
///
/// # 参数
///
/// - `state`: 包含数据库连接池的态对象，用于执行数据库操作
/// - `tag`: 需要更新的标签对象，包含新的标签信息
///
/// # 返回
///
/// - `Ok(Tags)`: 更新后的标签对象
/// - `Err(_)`: 如果更新过程中发生错误，返回一个错误对象
#[tauri::command]
pub async fn update_tag(state: State<'_, DbPool>, tag: TagParam) -> Result<Tags> {
    tag.update(&state.0).await
}

/// 使用软删除方式删除指定的标签。
///
/// 本函数通过异步操作从数据库中软删除一个标签，即更新标签的状态而不是物理删除。
/// 它接受一个数据库连接池状态和一个标签ID作为参数，并返回一个结果，表明受影响的行数。
///
/// # 参数
/// - `state`: 数据库连接池状态，用于异步访问数据库。
/// - `id`: 需要软删除的标签的唯一标识符。
///
/// # 返回
/// - `Result<u64>`: 一个异步结果，包含受影响的行数，表示成功删除的标签数量。
///
/// # 错误处理
/// 如果数据库操作失败，将返回一个错误。
#[tauri::command]
pub async fn soft_delete_tag(state: State<'_, DbPool>, id: i64) -> Result<u64> {
    Tags::soft_delete(&state.0, id).await
}

/// 异步增加指定标签的引用数目
///
/// 该函数通过接收一个数据库连接池状态和一个标签ID，来增加该标签的引用数目
/// 它使用了数据库操作，因此需要异步执行以避免阻塞主线程
///
/// # 参数
///
/// * `state` - 一个包含数据库连接池的状态，用于执行数据库操作
/// * `id` - 需要增加引用数目的标签的ID
///
/// # 返回
///
/// 返回一个结果，如果数据库操作成功，则包含更新后的`Tags`对象；如果操作失败，则包含一个错误
#[tauri::command]
pub async fn update_ref_num(state: State<'_, DbPool>, ref_num: i64, tag_ids: Vec<i64>) -> Result<()> {
    Tags::update_ref_num(&state.0, ref_num, tag_ids).await
}

#[tauri::command]
pub async fn change_tag_status(state: State<'_, DbPool>, tag_param: TagParam) -> Result<()> {
    tag_param.update(&state.0).await?;
    Ok(())
}

/// 检查给定标签名称是否已存在于数据库中
///
/// 此函数通过异步查询数据库来判断特定标签名称是否已经存在它接受一个数据库连接池状态和一个标签名称字符串作为输入参数，并返回一个布尔值，指示标签名称是否存在
///
/// # 参数
///
/// - `state`: 数据库连接池状态，用于执行数据库操作
/// - `tag_name`: 需要检查的标签名称
///
/// # 返回值
///
/// - `Result<bool>`: 返回一个布尔值，指示标签名称是否存在如果查询过程中发生错误，将返回一个错误类型
#[tauri::command]
pub async fn tag_name_exists(state: State<'_, DbPool>, tag_name: &str) -> Result<bool> {
    Tags::exists_by_tag_name(&state.0, tag_name).await
}

/// 批量软删除标签
///
/// 本函数接收一个数据库连接池状态和一个长整型向量作为参数，
/// 异步地对给定的标签ID进行批量软删除操作。
/// 软删除意味着这些标签在数据库中会被标记为已删除，但实际数据不会被移除。
///
/// # 参数
/// - `state`: 数据库连接池状态，用于执行数据库操作
/// - `ids`: 待删除的标签ID列表
///
/// # 返回值
/// - 成功时，返回实际被软删除的标签数量
/// - 失败时，返回一个错误
#[tauri::command]
pub async fn delete_tags_batch(state: State<'_, DbPool>, ids: Vec<i64>) -> Result<u64> {
    Tags::batch_soft_delete(&state.0, &ids).await
}

/// 在内存中设置和初始化一个SQLite数据库，用于测试目的。
///
/// 此函数会创建一个SQLite数据库连接池，并在该池中执行SQL语句以创建一个名为`tags`的表。
/// 表结构包括多个字段，用于存储标签信息，如标签名称、标签编号、显示顺序等。
/// 此外，函数确保通过在内存中创建数据库来隔离测试环境，避免影响实际数据库。
///
/// 返回值:
/// - `Pool<Sqlite>`: 一个SQLite数据库连接池，用于管理和执行与数据库的异步操作。
#[allow(dead_code)]
async fn setup_test_db() -> Pool<Sqlite> {
    // 创建一个SQLite数据库连接池，连接到内存数据库
    let pool = SqlitePool::connect(":memory:").await.unwrap();

    // 在数据库中执行SQL语句以创建`tags`表，包含多个字段用于存储标签信息
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

    // 返回SQLite数据库连接池
    pool
}

// #[cfg(test)]
// mod command_tests {
//     use crate::create_app;
//     use crate::db::tags::{setup_test_db, Tags};
//     use crate::db::DbPool;
//     use tauri::test::mock_builder;

//     /// there is an issue in windows platform: STATUS_ENTRYPOINT_NOT_FOUND,
//     /// so we skip it
//     #[tokio::test]
//     async fn test_add_tag() {
//         // 创建一个新的 Tags 实例
//         let tag = Tags {
//             tag_name: "Test Tag".to_string(),
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
//                 cmd: "add_tag".into(),
//                 callback: tauri::ipc::CallbackFn(0),
//                 error: tauri::ipc::CallbackFn(1),
//                 url: "http://tauri.localhost".parse().unwrap(),
//                 body: tauri::ipc::InvokeBody::Json(serde_json::to_value(&tag).unwrap()),
//                 headers: Default::default(),
//                 invoke_key: tauri::test::INVOKE_KEY.to_string(),
//             },
//         )
//             .map(|b| b.deserialize::<Tags>().unwrap())
//             .unwrap();
//         assert_eq!(res.tag_name, tag.tag_name);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_tag() {
        let pool = setup_test_db().await;

        let tag = TagParam {
            tag_name: Some("Test Tag".to_string()),
            tag_number: Some("T001".to_string()),
            tag_order: Some("001".to_string()),
            order_num: Some(1),
            status: Some("0".to_string()),
            remark: Some("A test tag".to_string()),
            del_flag: Some("0".to_string()),
            ..Default::default()
        };

        let inserted_tag = tag.add(&pool).await.unwrap();

        assert_eq!(inserted_tag.tag_name, "Test Tag");
        assert_eq!(inserted_tag.tag_number, "T001");
    }

    #[tokio::test]
    async fn test_query_tags() {
        let pool = setup_test_db().await;

        // Insert sample tags
        let tag1 = TagParam {
            tag_name: Some("Tag1".to_string()),
            tag_number: Some("T-001".to_string()),
            tag_order: Some("001".to_string()),
            status: Some("0".to_string()),
            remark: Some("Remark1".to_string()),
            ..Default::default()
        };
        tag1.add(&pool).await.unwrap();
        let tag2 = TagParam {
            tag_name: Some("Tag2".to_string()),
            tag_number: Some("T-002".to_string()),
            tag_order: Some("002".to_string()),
            status: Some("0".to_string()),
            remark: Some("Remark2".to_string()),
            ..Default::default()
        };
        tag2.add(&pool).await.unwrap();
        let tag3 = TagParam {
            tag_name: Some("Tag3".to_string()),
            tag_number: Some("T-003".to_string()),
            tag_order: Some("003".to_string()),
            status: Some("1".to_string()),
            ref_num: Some(2),
            remark: Some("Remark3".to_string()),
            ..Default::default()
        };
        tag3.add(&pool).await.unwrap();

        // Test 1: Basic filtering by tag_name
        let query_params = PageParams {
            page: 0,
            page_size: 10,
        };
        let tag_param = TagParam {
            tag_name: Some("Tag1".to_string()),
            ..Default::default()
        };
        let result = tag_param.query_tags(&pool, query_params.clone())
            .await
            .unwrap();
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0].tag_name, "Tag1");

        // Test 2: Filtering by tag_order and status
        let tag_param = TagParam {
            tag_order: Some("002".to_string()),
            status: Some("0".to_string()),
            ..Default::default()
        };
        let result = tag_param.query_tags(&pool, query_params.clone())
            .await
            .unwrap();
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0].tag_number, "T-002");

        // Test 3: Filtering by del_flag and ref_num
        let tag_param = TagParam {
            del_flag: Some("0".to_string()),
            ref_num: Some(2),
            ..Default::default()
        };
        let result = tag_param.query_tags(&pool, query_params.clone())
            .await
            .unwrap();
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0].tag_number, "T-003");

        // Test 4: Pagination - request only first page with 2 items per page
        let query_params = PageParams {
            page: 0,
            page_size: 2,
        };
        let tag_param = TagParam::default();
        let result = tag_param.query_tags(&pool, query_params.clone())
            .await
            .unwrap();
        assert_eq!(result.rows.len(), 2); // Should return first 2 items

        // Test 5: Pagination - request second page with 2 items per page
        let query_params = PageParams {
            page: 1,
            page_size: 2,
        };
        let result = tag_param.query_tags(&pool, query_params.clone())
            .await
            .unwrap();
        assert_eq!(result.rows.len(), 1); // Should return only the last item (Tag3)

        // Test 6: Fuzzy search for tag_name containing "Tag"
        let query_params = PageParams {
            page: 0,
            page_size: 10,
        };
        let tag_param = TagParam {
            tag_name: Some("Tag".to_string()),
            ..Default::default()
        };
        let result = tag_param.query_tags(&pool, query_params.clone())
            .await
            .unwrap();
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
        let new_tag = TagParam {
            tag_name: Some("Test Tag".to_string()),
            tag_number: Some("001".to_string()),
            ..Default::default()
        };
        let added_tag = new_tag.add(&pool).await.unwrap();
        let retrieved_tag = Tags::get_by_id(&pool, added_tag.tag_id).await.unwrap();
        assert!(retrieved_tag.is_some());
        assert_eq!(retrieved_tag.unwrap().tag_id, added_tag.tag_id);
    }

    #[tokio::test]
    async fn test_update_tag() {
        let pool = setup_test_db().await;
        let new_tag = TagParam {
            tag_name: Some("Test Tag".to_string()),
            tag_number: Some("001".to_string()),
            ..Default::default()
        };
        let added_tag = new_tag.add(&pool).await.unwrap();
        let updated_tag = TagParam {
            tag_id: Some(added_tag.tag_id),
            tag_name: Some("Updated Tag".to_string()),
            ..Default::default()
        };
        let retrieved_tag = updated_tag.update(&pool).await.unwrap();
        assert_eq!(retrieved_tag.tag_name, "Updated Tag");
    }

    #[tokio::test]
    async fn test_soft_delete() {
        let pool = setup_test_db().await;
        let new_tag = TagParam {
            tag_name: Some("Test Tag".to_string()),
            tag_number: Some("001".to_string()),
            ..Default::default()
        };
        let added_tag = new_tag.add(&pool).await.unwrap();
        let affected_rows = Tags::soft_delete(&pool, added_tag.tag_id).await.unwrap();
        assert_eq!(affected_rows, 1);
    }

    #[tokio::test]
    async fn test_increment_ref_num() {
        let pool = setup_test_db().await;
        let new_tag = TagParam {
            tag_name: Some("Test Tag".to_string()),
            tag_number: Some("001".to_string()),
            ref_num: Some(1),
            ..Default::default()
        };
        let added_tag = new_tag.add(&pool).await.unwrap();
        let incremented_tag = Tags::increment_ref_num(&pool, added_tag.tag_id)
            .await
            .unwrap();
        assert_eq!(incremented_tag.ref_num, 2);
    }

    #[tokio::test]
    async fn test_exists_by_tag_name() {
        let pool = setup_test_db().await;
        let new_tag = TagParam {
            tag_name: Some("Test Tag".to_string()),
            tag_number: Some("001".to_string()),
            ..Default::default()
        };
        let _added_tag = new_tag.add(&pool).await.unwrap();
        let exists = Tags::exists_by_tag_name(&pool, "Test Tag").await.unwrap();
        assert!(exists);
    }

    #[tokio::test]
    async fn test_batch_soft_delete() {
        let pool = setup_test_db().await;
        let tag1 = TagParam {
            tag_name: Some("Test Tag 1".to_string()),
            tag_number: Some("001".to_string()),
            ..Default::default()
        };
        let tag2 = TagParam {
            tag_name: Some("Test Tag 2".to_string()),
            tag_number: Some("002".to_string()),
            ..Default::default()
        };
        let added_tag1 = tag1.add(&pool).await.unwrap();
        let added_tag2 = tag2.add(&pool).await.unwrap();

        let tag_ids = vec![added_tag1.tag_id, added_tag2.tag_id];
        let affected_rows = Tags::batch_soft_delete(&pool, &tag_ids).await.unwrap();
        assert_eq!(affected_rows, 2);
    }
}
