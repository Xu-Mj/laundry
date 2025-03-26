use serde::{Deserialize, Serialize};
use sqlx::{FromRow, QueryBuilder, Row, Sqlite, Transaction, sqlite::SqliteRow};

use crate::{error::Result, utils};

use super::Curd;

/// 短信套餐类型
#[derive(Debug, Default, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "sms_plan_type_enum")]
pub enum SmsPlanType {
    #[default]
    /// 标准套餐
    Standard,
    /// 高级套餐
    Premium,
    /// 企业套餐
    Enterprise,
    /// 定制套餐
    Custom,
}

/// 短信套餐周期
#[derive(Debug, Default, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "sms_plan_period_enum")]
pub enum SmsPlanPeriod {
    #[default]
    /// 月度
    Monthly,
    /// 季度
    Quarterly,
    /// 半年
    HalfYearly,
    /// 年度
    Yearly,
}

/// 短信套餐模型
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SmsPlan {
    /// 套餐ID
    pub id: Option<i64>,

    /// 套餐名称
    pub name: String,

    /// 套餐类型
    pub plan_type: SmsPlanType,

    /// 套餐周期
    pub period: SmsPlanPeriod,

    /// 套餐价格（元）
    pub price: String,

    /// 短信条数
    pub sms_count: i32,

    /// 套餐描述
    pub description: Option<String>,

    /// 套餐特性（JSON格式存储）
    pub features: Option<serde_json::Value>,

    /// 是否为推荐套餐
    pub is_recommended: bool,

    /// 是否启用
    pub is_active: bool,

    /// 排序权重
    pub sort_order: i32,

    /// 创建时间
    pub created_at: i64,

    /// 更新时间
    pub updated_at: i64,

    /// 备注
    pub remark: Option<String>,
}

impl FromRow<'_, SqliteRow> for SmsPlan {
    fn from_row(row: &SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id").unwrap_or_default(),
            name: row.try_get("name").unwrap_or_default(),
            plan_type: row.try_get("plan_type").unwrap_or_default(),
            period: row.try_get("period").unwrap_or_default(),
            price: row.try_get("price").unwrap_or_default(),
            sms_count: row.try_get("sms_count").unwrap_or_default(),
            description: row.try_get("description").unwrap_or_default(),
            features: row.try_get("features").unwrap_or_default(),
            is_recommended: row.try_get("is_recommended").unwrap_or_default(),
            is_active: row.try_get("is_active").unwrap_or_default(),
            sort_order: row.try_get("sort_order").unwrap_or_default(),
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
        })
    }
}

impl Curd for SmsPlan {
    const COUNT_SQL: &'static str = "SELECT COUNT(*) FROM sms_plans";
    const QUERY_SQL: &'static str = "SELECT * FROM sms_plans";
    const BY_ID_SQL: &'static str = "SELECT * FROM sms_plans WHERE id = $1";
    const DELETE_BATCH_SQL: &'static str = "DELETE FROM sms_plans WHERE id IN (";

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, sqlx::Sqlite>) {
        if let Some(id) = self.id {
            builder.push(" AND id = ").push_bind(id);
        }

        if !self.name.is_empty() {
            builder
                .push(" AND name LIKE ")
                .push_bind(format!("%{}%", self.name));
        }

        // 可以根据需要添加更多过滤条件
    }
}

impl SmsPlan {
    /// 更新短信套餐信息
    pub async fn upsert(&self, pool: &mut Transaction<'_, Sqlite>) -> Result<bool> {
        let now = utils::get_timestamp();

        let result = sqlx::query(
            r#"
            INSERT INTO sms_plans (
                id, name, plan_type, period, price, sms_count,
                description, features, is_recommended, is_active,
                sort_order, created_at, updated_at, remark
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                plan_type = excluded.plan_type,
                period = excluded.period,
                price = excluded.price,
                sms_count = excluded.sms_count,
                description = excluded.description,
                features = excluded.features,
                is_recommended = excluded.is_recommended,
                is_active = excluded.is_active,
                sort_order = excluded.sort_order,
                updated_at = excluded.updated_at,
                remark = excluded.remark
            "#,
        )
        .bind(&self.id) // $1
        .bind(&self.name) // $2
        .bind(&self.plan_type) // $3
        .bind(&self.period) // $4
        .bind(&self.price) // $5 (修复原版参数顺序错误)
        .bind(&self.sms_count) // $6
        .bind(&self.description) // $7
        .bind(&self.features) // $8
        .bind(&self.is_recommended) // $9
        .bind(&self.is_active) // $10
        .bind(&self.sort_order) // $11
        .bind(now) // $12 created_at 使用当前时间减1秒（假设首次创建）
        .bind(now) // $13 updated_at 总是当前时间
        .bind(&self.remark) // $14
        .execute(&mut **pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
