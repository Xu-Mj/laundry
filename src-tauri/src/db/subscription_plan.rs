use serde::{Deserialize, Serialize};
use sqlx::{FromRow, QueryBuilder, Row, Sqlite, Transaction, sqlite::SqliteRow};

use crate::{error::Result, utils};

use super::Curd;

/// 订阅套餐类型
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub enum PlanType {
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

/// 订阅套餐周期
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub enum PlanPeriod {
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

/// 订阅套餐模型
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionPlan {
    /// 套餐ID
    pub id: Option<i64>,

    /// 套餐名称
    pub name: String,

    /// 套餐类型
    pub plan_type: String,

    /// 套餐周期
    pub period: String,

    /// 套餐价格（元）
    pub price: String,

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

impl FromRow<'_, SqliteRow> for SubscriptionPlan {
    fn from_row(row: &SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id").unwrap_or_default(),
            name: row.try_get("name").unwrap_or_default(),
            plan_type: row.try_get("plan_type").unwrap_or_default(),
            period: row.try_get("period").unwrap_or_default(),
            price: row.try_get("price").unwrap_or_default(),
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

impl Curd for SubscriptionPlan {
    const COUNT_SQL: &'static str =
        "SELECT COUNT(*) FROM subscription_plans WHERE is_active = true";

    const QUERY_SQL: &'static str = "SELECT * FROM subscription_plans WHERE is_active = true";

    const BY_ID_SQL: &'static str =
        "SELECT * FROM subscription_plans WHERE id = $1 AND is_active = true";

    const DELETE_BATCH_SQL: &'static str =
        "UPDATE subscription_plans SET is_active = false WHERE id = ANY($1)";

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, Sqlite>) {
        if let Some(id) = self.id {
            builder.push(" AND id = ");
            builder.push_bind(id);
        }

        if !self.name.is_empty() {
            builder.push(" AND name LIKE ");
            builder.push_bind(format!("%{}%", &self.name));
        }

        builder.push(" ORDER BY sort_order ASC, created_at DESC");
    }
}

impl SubscriptionPlan {
    pub async fn upsert(&self, pool: &mut Transaction<'_, Sqlite>) -> Result<Self> {
        let now = utils::get_timestamp();
        let result = sqlx::query_as(
            r#"
            INSERT INTO subscription_plans (
                id, name, plan_type, period, price, description, features,
                is_recommended, is_active, sort_order, created_at, updated_at, remark
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                plan_type = excluded.plan_type,
                period = excluded.period,
                price = excluded.price,
                description = excluded.description,
                features = excluded.features,
                is_recommended = excluded.is_recommended,
                is_active = excluded.is_active,
                sort_order = excluded.sort_order,
                updated_at = excluded.updated_at,
                remark = excluded.remark
            RETURNING *
            "#,
        )
        .bind(&self.id)
        .bind(&self.name)
        .bind(&self.plan_type)
        .bind(&self.period)
        .bind(&self.price)
        .bind(&self.description)
        .bind(&self.features)
        .bind(self.is_recommended)
        .bind(self.is_active)
        .bind(self.sort_order)
        .bind(now)
        .bind(now)
        .bind(&self.remark)
        .fetch_one(&mut **pool)
        .await?;

        Ok(result)
    }
}
