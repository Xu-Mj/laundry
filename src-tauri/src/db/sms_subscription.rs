use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, QueryBuilder, Row, Sqlite, Transaction, sqlite::SqliteRow};

use crate::{error::Result, utils};

use super::{Curd, sms_plan::SmsPlan};

/// 商家短信订阅记录
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SmsSubscription {
    /// 订阅ID
    pub id: Option<i64>,

    /// 商家ID
    pub store_id: i64,

    /// 套餐ID
    pub plan_id: i64,

    /// 订阅状态
    pub status: Option<String>,

    /// 开始时间
    pub start_date: i64,

    /// 到期时间
    pub expiry_date: i64,

    /// 是否自动续费
    pub auto_renew: bool,

    /// 最近一次支付ID
    pub last_payment_id: Option<String>,

    /// 下次续费时间
    pub next_billing_date: Option<i64>,

    /// 订阅价格（实际支付金额）
    pub price_paid: String,

    /// 总短信条数
    pub total_sms_count: i32,

    /// 已使用短信条数
    pub used_sms_count: i32,

    /// 剩余短信条数
    pub remaining_sms_count: i32,

    /// 优惠码
    pub promo_code: Option<String>,

    /// 是否为首次免费
    pub is_first_free: bool,

    /// 创建时间
    pub created_at: i64,

    /// 更新时间
    pub updated_at: i64,

    /// 取消原因
    pub cancellation_reason: Option<String>,

    /// 备注
    pub remark: Option<String>,

    pub plan: Option<SmsPlan>,
}

impl FromRow<'_, SqliteRow> for SmsSubscription {
    fn from_row(row: &SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        let plan = SmsPlan::from_row(row)?;
        Ok(Self {
            id: row.try_get("id").unwrap_or_default(),
            store_id: row.try_get("store_id").unwrap_or_default(),
            plan_id: row.try_get("plan_id").unwrap_or_default(),
            status: row.try_get("status").unwrap_or_default(),
            start_date: row.try_get("start_date").unwrap_or_default(),
            expiry_date: row.try_get("expiry_date").unwrap_or_default(),
            auto_renew: row.try_get("auto_renew").unwrap_or_default(),
            last_payment_id: row.try_get("last_payment_id").unwrap_or_default(),
            next_billing_date: row.try_get("next_billing_date").unwrap_or_default(),
            price_paid: row.try_get("price_paid").unwrap_or_default(),
            total_sms_count: row.try_get("total_sms_count").unwrap_or_default(),
            used_sms_count: row.try_get("used_sms_count").unwrap_or_default(),
            remaining_sms_count: row.try_get("remaining_sms_count").unwrap_or_default(),
            promo_code: row.try_get("promo_code").unwrap_or_default(),
            is_first_free: row.try_get("is_first_free").unwrap_or_default(),
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
            cancellation_reason: row.try_get("cancellation_reason").unwrap_or_default(),
            remark: row.try_get("remark").unwrap_or_default(),
            plan: Some(plan),
        })
    }
}

impl Curd for SmsSubscription {
    const COUNT_SQL: &'static str = "SELECT COUNT(*) FROM sms_subscriptions";
    const QUERY_SQL: &'static str = "SELECT * FROM sms_subscriptions";
    const BY_ID_SQL: &'static str = "SELECT * FROM sms_subscriptions WHERE id = $1";
    const DELETE_BATCH_SQL: &'static str = "DELETE FROM sms_subscriptions WHERE id IN (";

    fn apply_filters<'a>(&'a self, builder: &mut QueryBuilder<'a, sqlx::Sqlite>) {
        if self.store_id != 0 {
            builder.push(" AND store_id = ").push_bind(self.store_id);
        }

        if self.plan_id != 0 {
            builder.push(" AND plan_id = ").push_bind(self.plan_id);
        }

        if let Some(id) = self.id {
            builder.push(" AND id = ").push_bind(id);
        }
    }
}

impl SmsSubscription {
    pub async fn upsert(&self, pool: &mut Transaction<'_, Sqlite>) -> Result<SmsSubscription> {
        let now = utils::get_timestamp();

        let result = sqlx::query_as(
            r#"
            INSERT INTO sms_subscriptions (
                id, store_id, plan_id, status, start_date, expiry_date, 
                auto_renew, last_payment_id, next_billing_date, price_paid, 
                total_sms_count, used_sms_count, remaining_sms_count, 
                promo_code, is_first_free, created_at, updated_at, 
                cancellation_reason, remark
            ) VALUES (
                $1, $2, $3, $4, $5, $6, 
                $7, $8, $9, $10, 
                $11, $12, $13, 
                $14, $15, $16, $17, 
                $18, $19
            )
            ON CONFLICT (id) DO UPDATE SET
                store_id = excluded.store_id,
                plan_id = excluded.plan_id,
                status = excluded.status,
                start_date = excluded.start_date,
                expiry_date = excluded.expiry_date,
                auto_renew = excluded.auto_renew,
                last_payment_id = excluded.last_payment_id,
                next_billing_date = excluded.next_billing_date,
                price_paid = excluded.price_paid,
                total_sms_count = excluded.total_sms_count,
                used_sms_count = excluded.used_sms_count,
                remaining_sms_count = excluded.remaining_sms_count,
                promo_code = excluded.promo_code,
                is_first_free = excluded.is_first_free,
                updated_at = excluded.updated_at,
                cancellation_reason = excluded.cancellation_reason,
                remark = excluded.remark
            RETURNING *
            "#,
        )
        .bind(self.id) // $1
        .bind(self.store_id) // $2
        .bind(self.plan_id) // $3
        .bind(&self.status) // $4
        .bind(self.start_date) // $5
        .bind(self.expiry_date) // $6
        .bind(self.auto_renew) // $7
        .bind(&self.last_payment_id) // $8
        .bind(self.next_billing_date) // $9
        .bind(&self.price_paid) // $10
        .bind(self.total_sms_count) // $11
        .bind(self.used_sms_count) // $12
        .bind(self.remaining_sms_count) // $13
        .bind(&self.promo_code) // $14
        .bind(self.is_first_free) // $15
        .bind(now) // $16 created_at (仅插入时设置)
        .bind(now) // $17 updated_at (总是更新)
        .bind(&self.cancellation_reason) // $18
        .bind(&self.remark) // $19
        .fetch_one(&mut **pool)
        .await?;

        Ok(result)
    }

    /// 获取商家当前有效的短信订阅
    pub async fn get_active_subscription(
        pool: &Pool<Sqlite>,
        store_id: i64,
    ) -> Result<Option<SmsSubscription>> {
        let subscription = sqlx::query_as(
            "SELECT * FROM sms_subscriptions 
            WHERE store_id = $1 AND status = 'Active' 
            ORDER BY expiry_date DESC LIMIT 1",
        )
        .bind(store_id)
        .fetch_optional(pool)
        .await?;

        Ok(subscription)
    }
}
