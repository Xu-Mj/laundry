use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Pool, QueryBuilder, Row, Sqlite, Transaction};
use tauri::State;

use crate::db::PageResult;
use crate::error::{Error, ErrorKind, Result};
use crate::state::AppState;
use crate::utils;
use crate::utils::request::Request;

use super::{Curd, PageParams, Validator};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Clothing {
    /// 商家ID，用于数据隔离
    store_id: Option<i64>,
    /// 衣物商品ID (spuId)
    id: Option<i64>,
    pub clothing_number: Option<String>,
    /// 品类ID，关联clothing_categories表
    category_id: Option<i64>,
    /// 分类ID，关联clothing_styles表
    style_id: Option<i64>,
    /// 衣物名称 (title)
    title: Option<String>,
    /// 简介 (etitle)
    etitle: Option<String>,
    /// 主图 (primaryImage)
    primary_image: Option<String>,
    /// 展示图片列表 (images)
    images: Option<String>,
    images_vec: Vec<String>,
    /// 详情图片列表 (desc)
    description_images: Option<String>,
    description_images_vec: Vec<String>,
    /// 是否上架 (isPutOnSale)
    is_put_on_sale: Option<bool>,
    /// 是否可用 (isAvailable)
    is_available: Option<bool>,
    /// 是否售罄 (isSoldOut)
    is_sold_out: Option<bool>,
    /// 是否是默认参数
    is_default: Option<bool>,
    /// 原始价格 (minLinePrice/maxLinePrice)
    clothing_base_price: Option<f64>,
    /// 销售价格 (minSalePrice/maxSalePrice)
    sale_price: Option<f64>,
    /// 最小利润价格 (minProfitPrice)
    clothing_min_price: Option<f64>,
    /// 库存数量 (spuStockQuantity)
    stock_quantity: Option<i64>,
    /// 已售数量 (soldNum)
    sold_num: Option<i64>,
    /// 关联的SKU列表 (skuList)
    sku_list: Option<String>,
    /// 规格列表 (specList)
    spec_list: Option<String>,
    /// 标签列表 (spuTagList)
    tag_list: Option<String>,
    tag_list_vec: Vec<String>,
    hang_type: Option<String>,
    /// 显示顺序，默认显示顺序
    order_num: Option<i64>,
    /// 使用次数，实际被使用的次数，当该值不为0时，将优先将按照该值排序，然后才是显示顺序排序
    clothing_degree: Option<i64>,
    /// 软删除标志
    del_flag: Option<String>,
    create_time: i64,
    update_time: i64,

    clothing_category: Option<String>,
    clothing_style: Option<String>,
}

impl FromRow<'_, SqliteRow> for Clothing {
    fn from_row(row: &'_ SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        let images: Option<String> = row.try_get("images").unwrap_or_default();
        let description_images: Option<String> =
            row.try_get("description_images").unwrap_or_default();
        let tag_list: Option<String> = row.try_get("tag_list").unwrap_or_default();

        let tag_list_vec = tag_list
            .as_deref()
            .map(|s| {
                if s.is_empty() {
                    vec![]
                } else {
                    s.split(',').map(|s| s.to_string()).collect()
                }
            })
            .unwrap_or_default();

        let images_vec = images
            .as_deref()
            .map(|s| {
                if s.is_empty() {
                    vec![]
                } else {
                    s.split(',').map(|s| s.to_string()).collect()
                }
            })
            .unwrap_or_default();

        let description_images_vec = description_images
            .as_deref()
            .map(|s| {
                if s.is_empty() {
                    vec![]
                } else {
                    s.split(',').map(|s| s.to_string()).collect()
                }
            })
            .unwrap_or_default();
        Ok(Self {
            store_id: row.try_get("store_id").unwrap_or_default(),
            id: row.try_get("id").unwrap_or_default(),
            clothing_number: row.try_get("clothing_number").unwrap_or_default(),
            category_id: row.try_get("category_id").unwrap_or_default(),
            style_id: row.try_get("style_id").unwrap_or_default(),
            title: row.try_get("title").unwrap_or_default(),
            etitle: row.try_get("etitle").unwrap_or_default(),
            primary_image: row.try_get("primary_image").unwrap_or_default(),
            images,
            description_images,
            is_put_on_sale: row.try_get("is_put_on_sale").unwrap_or_default(),
            is_available: row.try_get("is_available").unwrap_or_default(),
            is_sold_out: row.try_get("is_sold_out").unwrap_or_default(),
            is_default: row.try_get("is_default").unwrap_or_default(),
            clothing_base_price: row.try_get("clothing_base_price").unwrap_or_default(),
            sale_price: row.try_get("sale_price").unwrap_or_default(),
            clothing_min_price: row.try_get("clothing_min_price").ok(),
            stock_quantity: row.try_get("stock_quantity").unwrap_or_default(),
            sold_num: row.try_get("sold_num").unwrap_or_default(),
            sku_list: row.try_get("sku_list").unwrap_or_default(),
            spec_list: row.try_get("spec_list").unwrap_or_default(),
            tag_list,
            hang_type: row.try_get("hang_type").unwrap_or_default(),
            order_num: row.try_get("order_num").unwrap_or_default(),
            clothing_degree: row.try_get("clothing_degree").unwrap_or_default(),
            del_flag: row.try_get("del_flag").unwrap_or_default(),
            create_time: row.try_get("create_time").unwrap_or_default(),
            update_time: row.try_get("update_time").unwrap_or_default(),
            clothing_category: row.try_get("clothing_category").unwrap_or_default(),
            clothing_style: row.try_get("clothing_style").unwrap_or_default(),
            images_vec,
            description_images_vec,
            tag_list_vec,
        })
    }
}

impl Validator for Clothing {
    fn validate(&self) -> Result<()> {
        if self.title.is_none() || self.title.as_ref().unwrap().trim().is_empty() {
            return Err(Error::bad_request("衣物商品名称不能为空"));
        }

        if self.category_id.is_none() {
            return Err(Error::bad_request("品类ID不能为空"));
        }

        if self.style_id.is_none() {
            return Err(Error::bad_request("分类ID不能为空"));
        }

        if self.primary_image.is_none() || self.primary_image.as_ref().unwrap().trim().is_empty() {
            return Err(Error::bad_request("衣物主图不能为空"));
        }

        if self.clothing_base_price.is_none() {
            return Err(Error::bad_request("原始价格不能为空"));
        }

        if self.sale_price.is_none() {
            return Err(Error::bad_request("销售价格不能为空"));
        }

        if self.stock_quantity.is_none() {
            return Err(Error::bad_request("库存数量不能为空"));
        }

        Ok(())
    }
}

impl Curd for Clothing {
    const COUNT_SQL: &'static str = "SELECT COUNT(*) FROM clothing c 
        LEFT JOIN clothing_categories cc ON c.category_id = cc.category_id 
        LEFT JOIN clothing_styles cs ON c.style_id = cs.style_id  WHERE c.del_flag = '0' ";
    const QUERY_SQL: &'static str = "SELECT c.*, cc.category_name as clothing_category, cs.style_name as clothing_style  FROM clothing c 
        LEFT JOIN clothing_categories cc ON c.category_id = cc.category_id 
        LEFT JOIN clothing_styles cs ON c.style_id = cs.style_id 
        WHERE c.del_flag = '0' ";
    const BY_ID_SQL: &'static str =
        "SELECT c.*, cc.category_name as clothing_category, cs.style_name as clothing_style FROM clothing c 
        LEFT JOIN clothing_categories cc ON c.category_id = cc.category_id 
        LEFT JOIN clothing_styles cs ON c.style_id = cs.style_id 
        WHERE c.id = ? AND c.del_flag = '0' ";
    const DELETE_BATCH_SQL: &'static str = "UPDATE clothing SET del_flag = '2' WHERE id IN (";
    const ORDER_SQL: Option<&'static str> = Some(" ORDER BY order_num DESC, clothing_degree ASC ");

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, Sqlite>) {
        if self.store_id.is_none() {
            builder.push(" AND store_id = ").push_bind(self.store_id);
        }

        if let Some(id) = &self.id {
            builder.push(" AND id = ").push_bind(id);
        }

        if let Some(title) = &self.title {
            builder
                .push(" AND title ILIKE ")
                .push_bind(format!("%{}%", title));
        }

        if let Some(is_put_on_sale) = &self.is_put_on_sale {
            builder
                .push(" AND is_put_on_sale = ")
                .push_bind(is_put_on_sale);
        }

        if let Some(is_available) = &self.is_available {
            builder.push(" AND is_available = ").push_bind(is_available);
        }

        if let Some(is_sold_out) = &self.is_sold_out {
            builder.push(" AND is_sold_out = ").push_bind(is_sold_out);
        }

        if let Some(del_flag) = &self.del_flag {
            builder.push(" AND del_flag = ").push_bind(del_flag);
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

    // 根据商家ID获取所有商品
    pub async fn get_by_store_id(pool: &Pool<Sqlite>, store_id: i64) -> Result<Vec<Self>> {
        let result =
            sqlx::query_as("SELECT * FROM clothing WHERE store_id = $1 AND del_flag = '0'")
                .bind(store_id)
                .fetch_all(pool)
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

    // 检查clothing_name是否已经存在
    pub async fn exists_by_clothing_name(
        pool: &Pool<Sqlite>,
        store_id: i64,
        clothing_name: &str,
    ) -> Result<bool> {
        let result = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM clothing WHERE store_id = ? AND title = $1 AND del_flag = '0')",
        )
        .bind(store_id)
        .bind(clothing_name)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    // 添加新商品
    pub async fn insert(self, tx: &mut Transaction<'_, Sqlite>) -> Result<Self> {
        let now = utils::get_timestamp();

        let result = sqlx::query_as(
            r#"
            INSERT INTO clothing (
                store_id, clothing_number, category_id, style_id, title, etitle, primary_image, 
                images, description_images, is_put_on_sale, is_available, 
                is_sold_out, is_default, clothing_base_price, sale_price, clothing_min_price, 
                stock_quantity, sold_num, sku_list, spec_list, order_num, clothing_degree,
                tag_list, del_flag, create_time, update_time
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, '0', ?, ?
            ) RETURNING *
            "#,
        )
        .bind(self.store_id)
        .bind(self.clothing_number)
        .bind(self.category_id)
        .bind(self.style_id)
        .bind(self.title)
        .bind(self.etitle)
        .bind(self.primary_image)
        .bind(self.images)
        .bind(self.description_images)
        .bind(self.is_put_on_sale)
        .bind(self.is_available)
        .bind(self.is_sold_out)
        .bind(self.is_default)
        .bind(self.clothing_base_price)
        .bind(self.sale_price)
        .bind(self.clothing_min_price)
        .bind(self.stock_quantity)
        .bind(self.sold_num)
        .bind(self.sku_list)
        .bind(self.spec_list)
        .bind(self.order_num)
        .bind(self.clothing_degree)
        .bind(self.tag_list)
        .bind(now)
        .bind(now)
        .fetch_one(&mut **tx)
        .await?;

        Ok(result)
    }

    // 更新商品信息
    pub async fn update(&self, tx: &mut Transaction<'_, Sqlite>) -> Result<bool> {
        let now = utils::get_timestamp();

        let result = sqlx::query(
            r#"
            UPDATE clothing SET
                category_id = ?,
                style_id = ?,
                title = ?,
                etitle = ?,
                primary_image = ?,
                images = ?,
                description_images = ?,
                is_put_on_sale = ?,
                is_available = ?,
                is_sold_out = ?,
                clothing_base_price = ?,
                sale_price = ?,
                clothing_min_price = ?,
                stock_quantity = ?,
                sold_num = ?,
                sku_list = ?,
                spec_list = ?,
                tag_list = ?,
                order_num =?,
                clothing_degree =?,
                update_time = ?
            WHERE id = ? AND store_id = ? AND del_flag = '0'
            "#,
        )
        .bind(self.category_id)
        .bind(&self.style_id)
        .bind(&self.title)
        .bind(&self.etitle)
        .bind(&self.primary_image)
        .bind(&self.images)
        .bind(&self.description_images)
        .bind(self.is_put_on_sale)
        .bind(self.is_available)
        .bind(self.is_sold_out)
        .bind(self.clothing_base_price)
        .bind(self.sale_price)
        .bind(self.clothing_min_price)
        .bind(self.stock_quantity)
        .bind(self.sold_num)
        .bind(&self.sku_list)
        .bind(&self.spec_list)
        .bind(&self.tag_list)
        .bind(self.order_num)
        .bind(self.clothing_degree)
        .bind(now)
        .bind(self.id)
        .bind(self.store_id)
        .execute(&mut **tx)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // 增加ref_num
    pub async fn increment_ref_num(tx: &mut Transaction<'_, Sqlite>, id: i64) -> Result<bool> {
        let result =
            sqlx::query("UPDATE clothing SET clothing_degree = clothing_degree + 1 WHERE id = ? ")
                .bind(id)
                .execute(&mut **tx)
                .await?;

        Ok(result.rows_affected() > 0)
    }

    // 增加ref_num
    pub async fn update_ref_num(pool: &Pool<Sqlite>, ref_num: i64, ids: Vec<i64>) -> Result<()> {
        let mut query_builder =
            QueryBuilder::<Sqlite>::new("UPDATE clothing SET clothing_degree = ");
        query_builder.push_bind(ref_num).push(" WHERE id IN (");

        let mut separated = query_builder.separated(", ");
        for id in &ids {
            separated.push_bind(id);
        }
        query_builder.push(")");

        query_builder.build().execute(pool).await?;
        Ok(())
    }

    // 更新库存数量
    pub async fn update_stock(
        tx: &mut Transaction<'_, Sqlite>,
        id: i64,
        quantity: i64,
    ) -> Result<()> {
        sqlx::query("UPDATE clothing SET stock_quantity = stock_quantity + ? WHERE id = ?")
            .bind(quantity)
            .bind(id)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }

    // 更新已售数量
    pub async fn update_sold_num(
        tx: &mut Transaction<'_, Sqlite>,
        id: i64,
        quantity: i64,
    ) -> Result<()> {
        sqlx::query("UPDATE clothing SET sold_num = sold_num + ? WHERE id = ?")
            .bind(quantity)
            .bind(id)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }
}

impl Request for Clothing {
    const URL: &'static str = "/clothing";
}

/// tauri commands
///
/// 分页查询衣物列表
#[tauri::command]
pub async fn list_clothing_pagination(
    state: State<'_, AppState>,
    page_params: PageParams,
    mut clothing: Clothing,
) -> Result<PageResult<Clothing>> {
    let store_id = utils::get_user_id(&state).await?;
    clothing.store_id = Some(store_id);
    clothing.get_list(&state.pool, page_params).await
}

#[tauri::command]
pub async fn list_clothing_all(
    state: State<'_, AppState>,
    mut clothing: Clothing,
) -> Result<Vec<Clothing>> {
    clothing.store_id = Some(utils::get_user_id(&state).await?);
    clothing.query_all_clothing(&state.pool).await
}

// 上传衣物图片到服务端，返回服务端图片路径
async fn upload_clothing_images(
    state: &State<'_, AppState>,
    local_paths: &[String],
) -> Result<Vec<String>> {
    let token = &state.try_get_token().await?;
    state
        .http_client
        .upload_files("/upload/images", local_paths, "file", Some(token))
        .await
}

#[tauri::command]
pub async fn add_clothing(state: State<'_, AppState>, mut clothing: Clothing) -> Result<Clothing> {
    if clothing.title.is_none() || clothing.title.as_ref().unwrap().trim().is_empty() {
        return Err(Error::with_details(ErrorKind::BadRequest, "衣物名不能为空"));
    }

    clothing.store_id = Some(utils::get_user_id(&state).await?);

    // gen clothing_number
    let code = utils::gen_code(clothing.title.as_ref().unwrap());
    clothing.clothing_number =
        Some(Clothing::select_next_num(&state.pool, &format!("{code}-")).await?);

    // 确保tag_list字段正确处理
    if let Some(tag_list) = &clothing.tag_list {
        clothing.tag_list_vec = if tag_list.is_empty() {
            vec![]
        } else {
            tag_list.split(',').map(|s| s.to_string()).collect()
        };
    }

    tracing::debug!("clothing: {:?}", clothing);
    // 保存本地图片路径的副本
    let local_primary_image = clothing.primary_image.clone();
    let local_images = clothing.images_vec.clone();

    // 上传主图到服务端
    if let Some(primary_path) = &local_primary_image {
        if !primary_path.is_empty() {
            tracing::debug!("primary_path: {:?}", primary_path);
            let server_paths = upload_clothing_images(&state, &[primary_path.clone()]).await?;
            tracing::debug!("server_paths: {:?}", server_paths);
            if !server_paths.is_empty() {
                clothing.primary_image = Some(server_paths[0].clone());
            }
        }
    }

    // 上传图片集合到服务端
    if !local_images.is_empty() {
        let server_paths = upload_clothing_images(&state, &local_images).await?;
        if !server_paths.is_empty() {
            clothing.images = Some(server_paths.join(","));
            clothing.images_vec = server_paths;
        }
    }

    // 上传详情图片到服务端
    if !clothing.description_images_vec.is_empty() {
        let server_paths = upload_clothing_images(&state, &clothing.description_images_vec).await?;
        if !server_paths.is_empty() {
            clothing.description_images = Some(server_paths.join(","));
            clothing.description_images_vec = server_paths;
        }
    }

    let mut tx = state.pool.begin().await?;
    let mut clothing = clothing.create_request(&state).await?;

    // 恢复本地图片路径，用于本地存储
    clothing.primary_image = local_primary_image;
    if !local_images.is_empty() {
        clothing.images = Some(local_images.join(","));
        clothing.images_vec = local_images;
    }

    let clothing = clothing.insert(&mut tx).await?;

    tx.commit().await?;

    Ok(clothing)
}

#[tauri::command]
pub async fn get_clothing_by_id(state: State<'_, AppState>, id: i64) -> Result<Option<Clothing>> {
    Clothing::get_by_id(&state.pool, id).await
}

#[tauri::command]
pub async fn update_clothing(state: State<'_, AppState>, mut clothing: Clothing) -> Result<bool> {
    if clothing.id.is_none() {
        return Err(Error::bad_request("衣物id不能为空"));
    }
    if clothing.title.is_none() || clothing.title.as_ref().unwrap().trim().is_empty() {
        return Err(Error::with_details(ErrorKind::BadRequest, "衣物名不能为空"));
    }

    clothing.is_default = Some(false);

    clothing.store_id = Some(utils::get_user_id(&state).await?);

    // 确保tag_list字段正确处理
    if let Some(tag_list) = &clothing.tag_list {
        clothing.tag_list_vec = if tag_list.is_empty() {
            vec![]
        } else {
            tag_list.split(',').map(|s| s.to_string()).collect()
        };
    }

    // 保存本地图片路径的副本
    let local_primary_image = clothing.primary_image.clone();
    let local_images = clothing.images_vec.clone();

    // 上传主图到服务端
    if let Some(primary_path) = &local_primary_image {
        if !primary_path.is_empty() {
            let server_paths = upload_clothing_images(&state, &[primary_path.clone()]).await?;
            if !server_paths.is_empty() {
                clothing.primary_image = Some(server_paths[0].clone());
            }
        }
    }

    // 上传图片集合到服务端
    if !local_images.is_empty() {
        let server_paths = upload_clothing_images(&state, &local_images).await?;
        if !server_paths.is_empty() {
            clothing.images = Some(server_paths.join(","));
            clothing.images_vec = server_paths;
        }
    }

    // 上传详情图片到服务端
    if !clothing.description_images_vec.is_empty() {
        let server_paths = upload_clothing_images(&state, &clothing.description_images_vec).await?;
        if !server_paths.is_empty() {
            clothing.description_images = Some(server_paths.join(","));
            clothing.description_images_vec = server_paths;
        }
    }

    let mut tx = state.pool.begin().await?;
    if !clothing.update_request(&state).await? {
        return Err(Error::internal("更新失败"));
    }

    // 恢复本地图片路径，用于本地存储
    clothing.primary_image = local_primary_image;
    if !local_images.is_empty() {
        clothing.images = Some(local_images.join(","));
        clothing.images_vec = local_images;
    }

    let res = clothing.update(&mut tx).await?;

    tx.commit().await?;
    Ok(res)
}

#[tauri::command]
pub async fn update_clothing_ref_num(
    state: State<'_, AppState>,
    ref_num: i64,
    ids: Vec<i64>,
) -> Result<()> {
    Clothing::update_ref_num(&state.pool, ref_num, ids).await
}

#[tauri::command]
pub async fn clothing_name_exists(state: State<'_, AppState>, clothing_name: &str) -> Result<bool> {
    let store_id = utils::get_user_id(&state).await?;
    Clothing::exists_by_clothing_name(&state.pool, store_id, clothing_name).await
}

#[tauri::command]
pub async fn delete_clothing_batch(state: State<'_, AppState>, ids: Vec<i64>) -> Result<bool> {
    let mut tr = state.pool.begin().await?;
    let result = Clothing::delete_batch(&mut tr, &ids).await?;
    tr.commit().await?;
    Ok(result)
}

#[tauri::command]
pub async fn create_clothing_4_create_order(
    state: State<'_, AppState>,
    mut clothing: Clothing,
) -> Result<Clothing> {
    if clothing.title.is_none() || clothing.title.as_ref().unwrap().trim().is_empty() {
        return Err(Error::with_details(ErrorKind::BadRequest, "衣物名不能为空"));
    }

    clothing.store_id = Some(utils::get_user_id(&state).await?);

    clothing.is_default = Some(true);
    clothing.is_put_on_sale = Some(false);
    clothing.is_available = Some(false);
    clothing.stock_quantity = Some(0);
    
    
    // gen clothing_number
    let code = utils::gen_code(clothing.title.as_ref().unwrap());
    clothing.clothing_number =
        Some(Clothing::select_next_num(&state.pool, &format!("{code}-")).await?);

    // 如果没有设置主图，使用默认衣物图片
    if clothing.primary_image.is_none() || clothing.primary_image.as_ref().unwrap().trim().is_empty() {
        // 使用相对路径，这样在不同平台上都能正确找到图片
        clothing.primary_image = Some("images/default_cloth.svg".to_string());
    }

    // 确保tag_list字段正确处理
    if let Some(tag_list) = &clothing.tag_list {
        clothing.tag_list_vec = if tag_list.is_empty() {
            vec![]
        } else {
            tag_list.split(',').map(|s| s.to_string()).collect()
        };
    }

    tracing::debug!("clothing: {:?}", clothing);

    let mut tx = state.pool.begin().await?;
    let clothing = clothing.create_request(&state).await?;

    let clothing = clothing.insert(&mut tx).await?;

    tx.commit().await?;

    Ok(clothing)
}
