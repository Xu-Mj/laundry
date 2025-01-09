use crate::db::order_clothes::OrderCloth;
use crate::db::AppState;
use crate::error::{Error, ErrorKind, Result};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, QueryBuilder, Sqlite, Transaction};
use tauri::State;

/// 晾衣架
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct DryingRack {
    pub(crate) capacity: Option<i32>,
    pub(crate) id: Option<i64>,
    pub(crate) name: Option<String>,
    pub(crate) position: Option<i32>,
    pub(crate) rack_type: Option<String>,
    pub(crate) remaining_capacity: Option<i32>,
}

impl DryingRack {
    pub fn validate(&self) -> Result<()> {
        if self.name.is_none() || self.name.as_ref().unwrap().trim().is_empty() {
            return Err(Error::with_details(ErrorKind::BadRequest, "衣架名不能为空"));
        }
        if self.capacity.is_none() || *self.capacity.as_ref().unwrap() <= 0 {
            return Err(Error::with_details(
                ErrorKind::BadRequest,
                "衣架容量必须大于0",
            ));
        }
        Ok(())
    }
}

impl DryingRack {
    /// Insert a new drying rack.
    pub async fn add(self, pool: &Pool<Sqlite>) -> Result<DryingRack> {
        let result = sqlx::query_as::<_, DryingRack>(
            "INSERT INTO drying_rack (name, capacity, position, rack_type, remaining_capacity)
                values ( ?, ?, ?, ?, ?)
             RETURNING *",
        )
        .bind(&self.name)
        .bind(&self.capacity)
        .bind(&self.position)
        .bind(&self.rack_type)
        .bind(&self.remaining_capacity)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    /// Update an existing drying rack.
    pub async fn update(&self, tr: &mut Transaction<'_, Sqlite>) -> Result<bool> {
        let mut query_builder = QueryBuilder::<Sqlite>::new("UPDATE drying_rack SET ");
        let mut has_update = false;

        if let Some(name) = &self.name {
            query_builder.push("name = ").push_bind(name);
            has_update = true;
        }

        if let Some(capacity) = &self.capacity {
            if has_update {
                query_builder.push(", ");
            }
            query_builder.push("capacity = ").push_bind(capacity);
            has_update = true;
        }

        if let Some(position) = &self.position {
            if has_update {
                query_builder.push(", ");
            }
            query_builder.push(" position = ").push_bind(position);
        }

        if let Some(rack_type) = &self.rack_type {
            if has_update {
                query_builder.push(", ");
            }
            query_builder.push(" rack_type = ").push_bind(rack_type);
        }

        if let Some(remaining_capacity) = &self.remaining_capacity {
            if has_update {
                query_builder.push(", ");
            }
            query_builder
                .push(" remaining_capacity = ")
                .push_bind(remaining_capacity);
        }

        if has_update {
            query_builder
                .push(" WHERE id = ")
                .push_bind(self.id)
                .push(" RETURNING *");
            let result = query_builder.build().execute(&mut **tr).await?;
            return Ok(result.rows_affected() > 0);
        }
        Ok(false)
    }

    /// Retrieve a drying rack by ID.
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: i64) -> Result<Option<DryingRack>> {
        let result = sqlx::query_as::<_, DryingRack>("SELECT * FROM drying_rack WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(result)
    }

    async fn delete_batch(pool: &Pool<Sqlite>, ids: Vec<i64>) -> Result<u64> {
        if ids.is_empty() {
            return Ok(0);
        }

        let mut query_builder =
            QueryBuilder::<Sqlite>::new("DELETE FROM drying_rack WHERE id IN (");

        for (i, id) in ids.iter().enumerate() {
            if i > 0 {
                query_builder.push(", ");
            }
            query_builder.push_bind(id);
        }

        query_builder.push(")");

        let query = query_builder.build();
        let result = query.execute(pool).await?;

        Ok(result.rows_affected())
    }

    #[allow(dead_code)]
    pub async fn select_one(pool: &Pool<Sqlite>, rack_typ: String) -> Result<Option<DryingRack>> {
        let result = sqlx::query_as::<_, DryingRack>(
            "
        SELECT *
        FROM drying_rack
        WHERE rack_type = ?
        ORDER BY remaining_capacity DESC
        LIMIT 1;",
        )
        .bind(rack_typ)
        .fetch_optional(pool)
        .await?;
        Ok(result)
    }

    pub async fn list(pool: &Pool<Sqlite>) -> Result<Vec<DryingRack>> {
        let result = sqlx::query_as::<_, DryingRack>("SELECT * FROM drying_rack")
            .fetch_all(pool)
            .await?;
        Ok(result)
    }
}

impl DryingRack {
    pub async fn get_position(pool: &Pool<Sqlite>, mut rack_type: String) -> Result<Self> {
        if rack_type.is_empty() {
            rack_type = "1".to_string();
        }

        // 根据类型获取当前空余最多的衣架
        let mut rack = DryingRack::select_one(pool, rack_type)
            .await?
            .ok_or(Error::not_found("没有可用的衣架"))?;

        if rack.capacity.is_none() || rack.capacity.unwrap() == 0 {
            return Err(Error::internal("衣架没有设置容量"));
        }

        let capacity = rack.capacity.unwrap();

        // 查询已经占用的挂钩
        let used_hangers = OrderCloth::query_hanger_number(pool, rack.id.unwrap()).await?;
        let result = find_first_available_hanger(&used_hangers, capacity);

        let position = if result.is_none() {
            if rack.position.unwrap_or_default() > capacity {
                1
            } else {
                rack.position.unwrap_or_default()
            }
        } else {
            // 找到空余挂钩
            rack.remaining_capacity = Some(rack.remaining_capacity.unwrap_or_default() - 1);
            result.unwrap() // 返回找到的挂钩位置
        };

        // 设置新位置
        rack.position = Some(position + 1);

        Ok(rack)
    }
}

#[tauri::command]
pub async fn list_rack_all(state: State<'_, AppState>) -> Result<Vec<DryingRack>> {
    DryingRack::list(&state.0).await
}

#[tauri::command]
pub async fn add_rack(state: State<'_, AppState>, mut rack: DryingRack) -> Result<DryingRack> {
    rack.validate()?;

    rack.remaining_capacity = Some(rack.capacity.unwrap());
    rack.position = Some(1);

    let rack = rack.add(&state.0).await?;

    Ok(rack)
}

#[tauri::command]
pub async fn get_rack_by_id(state: State<'_, AppState>, id: i64) -> Result<Option<DryingRack>> {
    DryingRack::get_by_id(&state.0, id).await
}

#[tauri::command]
pub async fn update_rack(state: State<'_, AppState>, mut rack: DryingRack) -> Result<bool> {
    rack.validate()?;

    let pool = &state.0;
    let result = DryingRack::get_by_id(pool, rack.id.unwrap())
        .await?
        .ok_or(Error::with_details(ErrorKind::NotFound, "衣架不存在"))?;

    let mut tr = pool.begin().await?;

    rack.remaining_capacity = Some(
        rack.capacity.unwrap()
            - (result.capacity.unwrap_or_default() - rack.remaining_capacity.unwrap_or_default()),
    );
    let result = rack.update(&mut tr).await?;
    tr.commit().await?;
    Ok(result)
}

// #[tauri::command]
// pub async fn get_position(state: State<'_, AppState>, rack_type: String) -> Result<DryingRack> {
//     let pool = &state.0;
//     let rack = DryingRack::get_position(pool, rack_type).await?;
//     let mut tr = pool.begin().await?;
//     if !rack.update(&mut tr).await? {
//         return Err(Error::internal("更新衣架位置失败"));
//     }
//     tr.commit().await?;
//     Ok(rack)
// }

#[tauri::command]
pub async fn delete_racks(state: State<'_, AppState>, ids: Vec<i64>) -> Result<u64> {
    DryingRack::delete_batch(&state.0, ids).await
}

fn find_first_available_hanger(used_hangers: &[i32], capacity: i32) -> Option<i32> {
    let mut hanger_used = vec![false; (capacity + 1) as usize];

    // 标记已使用的挂钩
    for &used in used_hangers {
        if used > 0 && used <= capacity {
            hanger_used[used as usize] = true;
        }
    }

    // 查找第一个未被占用的挂钩号，从1开始
    (1..=capacity).find(|&i| !hanger_used[i as usize])
}
