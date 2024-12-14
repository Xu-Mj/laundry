use crate::db::order_clothes::OrderCloth;
use crate::error::{Error, ErrorKind, Result};
use crate::utils;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, FixedOffset};
use sqlx::{FromRow, Pool, Sqlite, Transaction};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct OrderPicture {
    pub picture_id: Option<i64>,
    pub picture_path: Option<String>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime<FixedOffset>>,
}

impl OrderPicture {
    pub fn new_with_path(picture_path: String) -> Self {
        Self {
            picture_path: Some(picture_path),
            create_time: Some(utils::get_now()),
            ..Default::default()
        }
    }
}

impl OrderPicture {
    pub async fn insert(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<Self> {
        let query = "
            INSERT INTO order_pictures (picture_path, create_by, create_time)
            VALUES (?, ?, ?)
            RETURNING picture_id, picture_path, create_by, create_time
        ";

        // 执行 SQL 插入操作并返回插入后的完整记录
        let picture: OrderPicture = sqlx::query_as(query)
            .bind(&self.picture_path)
            .bind(&self.create_by)
            .bind(&self.create_time)
            .fetch_one(&mut **tr)
            .await?;

        Ok(picture)
    }

    pub async fn get_by_id(pool: &Pool<Sqlite>, picture_id: i64) -> Result<Option<OrderPicture>> {
        let query = "SELECT picture_id, picture_path, create_by, create_time
                     FROM order_pictures WHERE picture_id = ?";

        let picture = sqlx::query_as::<_, OrderPicture>(query)
            .bind(picture_id)
            .fetch_optional(pool)
            .await?;

        Ok(picture)
    }

    pub async fn get_by_ids(
        tr: &mut Transaction<'_, Sqlite>,
        ids: &[i64],
    ) -> Result<Vec<OrderPicture>> {
        let mut builder = sqlx::QueryBuilder::new(
            "SELECT picture_id, picture_path, create_by, create_time
                     FROM order_pictures WHERE picture_id IN (",
        );

        // bind ids
        for (i, id) in ids.iter().enumerate() {
            if i > 0 {
                builder.push(", ");
            }
            builder.push_bind(id);
        }
        builder.push(")");

        // query
        let picture = builder.build_query_as().fetch_all(&mut **tr).await?;

        Ok(picture)
    }

    #[allow(dead_code)]
    pub async fn get_all(pool: &Pool<Sqlite>) -> Result<Vec<OrderPicture>> {
        let query = "SELECT picture_id, picture_path, create_by, create_time FROM order_pictures";

        let pictures = sqlx::query_as::<_, OrderPicture>(query)
            .fetch_all(pool)
            .await?;

        Ok(pictures)
    }

    #[allow(dead_code)]
    pub async fn update(&self, pool: &Pool<Sqlite>) -> Result<()> {
        let query = "
            UPDATE order_pictures
            SET picture_path = ?, create_by = ?, create_time = ?
            WHERE picture_id = ?
        ";

        sqlx::query(query)
            .bind(&self.picture_path)
            .bind(&self.create_by)
            .bind(&self.create_time)
            .bind(self.picture_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn delete(tr: &mut Transaction<'_, Sqlite>, picture_id: i64) -> Result<u64> {
        let query = "DELETE FROM order_pictures WHERE picture_id = ?";

        let result = sqlx::query(query)
            .bind(picture_id)
            .execute(&mut **tr)
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn delete_batch(
        tr: &mut Transaction<'_, Sqlite>,
        pictures_id: &[i64],
    ) -> Result<u64> {
        let query = "DELETE FROM order_pictures WHERE picture_id IN (";

        let mut builder = sqlx::QueryBuilder::new(query);
        for (i, id) in pictures_id.iter().enumerate() {
            if i > 0 {
                builder.push(", ");
            }
            builder.push_bind(id);
        }
        builder.push(")");

        let result = builder.build().execute(&mut **tr).await?;

        Ok(result.rows_affected())
    }
}

impl OrderPicture {
    pub async fn delete_by_id(pool: &Pool<Sqlite>, picture_id: i64, cloth_id: i64) -> Result<u64> {
        let mut tr = pool.begin().await?;

        // query picture
        let picture =
            OrderPicture::get_by_id(&pool, picture_id)
                .await?
                .ok_or(Error::with_details(
                    ErrorKind::NotFound,
                    "picture not found",
                ))?;

        // update clothes information
        let cloth = OrderCloth::get_by_id(pool, cloth_id)
            .await?
            .ok_or(Error::with_kind(ErrorKind::NotFound))?;
        let update_err = OrderCloth::update_picture(&mut tr, cloth, picture_id).await;
        // ignore not found error
        if let Err(err) = update_err {
            if err.kind() != ErrorKind::NotFound {
                return Err(err);
            }
        }

        // delete picture from fs
        if let Some(path) = picture.picture_path {
            std::fs::remove_file(path)
                .map_err(|e| Error::with_details(ErrorKind::IOError, e.to_string()))?;
        }

        let result = OrderPicture::delete(&mut tr, picture_id).await?;

        tr.commit().await?;

        Ok(result)
    }
}
