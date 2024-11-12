use crate::db::DbPool;
use crate::error::{Error, ErrorKind, Result};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, QueryBuilder, Sqlite};
use tauri::State;

/// 晾衣架
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct DryingRack {
    /// 容量
    capacity: i32,
    /// ID
    id: i64,
    /// 架子编号
    name: String,
    /// 当前挂钩位置
    position: i32,
    /// 架子类型，1: 输送线 2: 其他
    rack_type: Option<String>,
    /// 剩余容量
    remaining_capacity: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DryingRackParam {
    capacity: Option<i32>,
    id: Option<i64>,
    name: Option<String>,
    position: Option<i32>,
    rack_type: Option<String>,
    remaining_capacity: Option<i32>,
}


impl From<DryingRackParam> for DryingRack {
    fn from(value: DryingRackParam) -> Self {
        DryingRack {
            capacity: value.capacity.unwrap_or(0),
            id: value.id.unwrap_or(0),
            name: value.name.unwrap_or_default(),
            position: value.position.unwrap_or(0),
            rack_type: value.rack_type,
            remaining_capacity: value.remaining_capacity.unwrap_or(0),
        }
    }
}

impl From<DryingRack> for DryingRackParam {
    fn from(value: DryingRack) -> Self {
        DryingRackParam {
            capacity: Some(value.capacity),
            id: Some(value.id),
            name: Some(value.name),
            position: Some(value.position),
            rack_type: value.rack_type,
            remaining_capacity: Some(value.remaining_capacity),
        }
    }
}

impl DryingRackParam {
    pub fn validate(&self) -> Result<()> {
        if self.name.is_none() || self.name.as_ref().unwrap().trim().is_empty() {
            return Err(Error::with_details(ErrorKind::BadRequest, "衣架名不能为空"));
        }
        if self.capacity.is_none() || *self.capacity.as_ref().unwrap() <= 0 {
            return Err(Error::with_details(ErrorKind::BadRequest, "衣架容量必须大于0"));
        }
        Ok(())
    }
}

impl DryingRackParam {
    /// Insert a new drying rack.
    pub async fn add(self, pool: &Pool<Sqlite>) -> Result<DryingRack> {
        let result = sqlx::query_as::<_, DryingRack>(
            "INSERT INTO drying_rack (name, capacity, position, rack_type, remaining_capacity)
                values ( ?, ?, ?, ?, ?)
             RETURNING *"
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
    pub async fn update(self, pool: &Pool<Sqlite>) -> Result<DryingRack> {
        let mut query_builder = QueryBuilder::<Sqlite>::new("UPDATE drying_rack SET ");
        let mut has_update = false;

        if let Some(name) = &self.name {
            query_builder.push("name = ").push_bind(name);
            has_update = true;
        }

        if let Some(capacity) = &self.capacity {
            if has_update { query_builder.push(", "); }
            query_builder.push("capacity = ").push_bind(capacity);
            has_update = true;
        }

        if let Some(position) = &self.position {
            if has_update { query_builder.push(", "); }
            query_builder.push(" position = ").push_bind(position);
        }

        if let Some(rack_type) = &self.rack_type {
            if has_update { query_builder.push(", "); }
            query_builder.push(" rack_type = ").push_bind(rack_type);
        }

        if let Some(remaining_capacity) = &self.remaining_capacity {
            if has_update { query_builder.push(", "); }
            query_builder.push(" remaining_capacity = ").push_bind(remaining_capacity);
        }


        if has_update {
            query_builder
                .push(" WHERE id = ")
                .push_bind(self.id)
                .push(" RETURNING *");
            let updated_rack = query_builder
                .build_query_as::<DryingRack>()
                .fetch_one(pool)
                .await?;
            Ok(updated_rack)
        } else {
            Ok(DryingRack::from(self))
        }
    }

    /// Retrieve a drying rack by ID.
    pub async fn find_by_id(pool: &Pool<Sqlite>, id: i64) -> Result<Option<DryingRack>> {
        let result = sqlx::query_as::<_, DryingRack>("SELECT * FROM drying_rack WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(result)
    }

    /// Delete a drying rack by ID.
    #[allow(dead_code)]
    pub async fn delete_by_id(pool: &Pool<Sqlite>, id: i64) -> Result<u64> {
        let result = sqlx::query("DELETE FROM drying_rack WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
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
        let result = sqlx::query_as::<_, DryingRack>("
        SELECT *
        FROM sys_drying_rack
        WHERE rack_type = ?
        ORDER BY remaining_capacity DESC
        LIMIT 1;")
            .bind(rack_typ)
            .fetch_optional(pool)
            .await?;
        Ok(result)
    }

    pub async fn list(pool: &Pool<Sqlite>) -> Result<Vec<DryingRack>> {
        let result = sqlx::query_as::<_, DryingRack>("SELECT * FROM drying_rack").fetch_all(pool).await?;
        Ok(result)
    }
}

#[tauri::command]
pub async fn list_rack_all(state: State<'_, DbPool>) -> Result<Vec<DryingRack>> {
    DryingRackParam::list(&state.0).await
}

#[tauri::command]
pub async fn add_rack(state: State<'_, DbPool>, mut rack: DryingRackParam) -> Result<DryingRack> {
    rack.validate()?;

    rack.remaining_capacity = Some(rack.capacity.unwrap());
    rack.position = Some(1);

    let rack = rack.add(&state.0).await?;

    Ok(rack)
}

#[tauri::command]
pub async fn get_rack_by_id(state: State<'_, DbPool>, id: i64) -> Result<Option<DryingRack>> {
    DryingRackParam::find_by_id(&state.0, id).await
}

#[tauri::command]
pub async fn update_rack(state: State<'_, DbPool>, mut rack: DryingRackParam) -> Result<DryingRack> {
    rack.validate()?;

    let result = DryingRackParam::find_by_id(&state.0, rack.id.unwrap()).await?.ok_or(Error::with_details(ErrorKind::NotFound, "衣架不存在"))?;

    rack.remaining_capacity = Some(rack.capacity.unwrap() - (result.capacity - rack.remaining_capacity.unwrap_or(0)));
    rack.update(&state.0).await
}

#[tauri::command]
pub async fn get_position(state: State<'_, DbPool>, mut rack_type: String) -> Result<DryingRack> {
    if rack_type.is_empty() {
        rack_type = "1".to_string();
    }

    // 根据类型获取当前空余最多的衣架
    let mut rack = DryingRackParam::select_one(&state.0, rack_type).await?.ok_or(Error::with_details(ErrorKind::NotFound, "没有可用的衣架"))?;

    // todo 查询已经占用的挂钩
    let used_hangers = vec![];
    let result = find_first_available_hanger(&used_hangers, rack.capacity);
    let position =
        if result.is_none() {
            if rack.position > rack.capacity { 1 } else { rack.position }
        } else {
            rack.remaining_capacity -= 1;
            result.unwrap()
        };
    rack.position = position + 1;
    let rack_param = DryingRackParam::from(rack.clone());
    rack_param.update(&state.0).await?;
    rack.position = position;
    Ok(rack)
}

#[tauri::command]
pub async fn delete_racks(state: State<'_, DbPool>, ids: Vec<i64>) -> Result<u64> {
    DryingRackParam::delete_batch(&state.0, ids).await
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
