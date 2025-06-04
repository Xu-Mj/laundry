use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    db::{PageResult, sms_plan::SmsPlan},
    error::{Error, Result},
    state::AppState,
    utils,
};

use super::subscription_plan::SubscriptionPlan;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub id: i64,
    pub store_id: i64,
    pub plan_id: i64,
    pub start_date: Option<i64>,
    pub expiry_date: Option<i64>,
    pub status: Option<String>,
    pub auto_renew: bool,
    pub price_paid: String,
    pub promo_code: Option<String>,
    pub is_first_year_free: bool,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
    pub remark: Option<String>,
    pub plan: Option<SubscriptionPlan>,
}

// Forward requests to the backend API
#[tauri::command]
pub async fn get_all_plans(state: State<'_, AppState>) -> Result<Vec<SubscriptionPlan>> {
    state.http_client.get("/plans", None).await
}

#[tauri::command]
pub async fn get_recommended_plans(state: State<'_, AppState>) -> Result<Vec<SubscriptionPlan>> {
    state.http_client.get("/plans/recommended", None).await
}

#[tauri::command]
pub async fn get_plans_by_type(
    state: State<'_, AppState>,
    type_: String,
) -> Result<Vec<SubscriptionPlan>> {
    let token = state.try_get_token().await?;
    let plans: Vec<SubscriptionPlan> = state
        .http_client
        .get(&format!("/plans/type/{}", type_), Some(&token))
        .await?;
    Ok(plans)
}

#[tauri::command]
pub async fn get_plan_by_id(
    state: State<'_, AppState>,
    id: i64,
) -> Result<Option<SubscriptionPlan>> {
    let token = state.try_get_token().await?;
    let plan: Option<SubscriptionPlan> = state
        .http_client
        .get(&format!("/plans/{}", id), Some(&token))
        .await?;
    Ok(plan)
}

#[tauri::command]
pub async fn get_subscription_by_id(state: State<'_, AppState>, id: i64) -> Result<Subscription> {
    let token = state.try_get_token().await?;
    let subscription: Subscription = state
        .http_client
        .get(&format!("/subscription/{}", id), Some(&token))
        .await?;
    Ok(subscription)
}

#[tauri::command]
pub async fn cancel_subscription(
    state: State<'_, AppState>,
    id: i64,
    reason: Option<String>,
) -> Result<Subscription> {
    // Create a cancellation reason payload
    let payload = serde_json::json!({
        "cancellationReason": reason
    });

    let token = state.try_get_token().await?;
    let result: Subscription = state
        .http_client
        .post(
            &format!("/subscription/{}/cancel", id),
            payload,
            Some(&token),
        )
        .await?;
    Ok(result)
}

#[tauri::command]
pub async fn check_store_subscription(state: State<'_, AppState>, store_id: i64) -> Result<bool> {
    let token = state.try_get_token().await?;
    let result: bool = state
        .http_client
        .get(&format!("/subscription/check/{}", store_id), Some(&token))
        .await?;
    Ok(result)
}

#[tauri::command]
pub async fn get_sms_plans(state: State<'_, AppState>) -> Result<Vec<SmsPlan>> {
    state.http_client.get("/sms/plans/all", None).await
}

/// 支付宝Native支付（二维码支付）请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlipayQRCodePayRequest {
    /// 商家ID
    pub store_id: i64,
    /// 订阅计划ID
    pub plan_id: i64,
    /// 是否自动续费
    #[serde(default = "default_auto_renew")]
    pub auto_renew: bool,
    /// 商品详情信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_detail: Option<serde_json::Value>,
    /// 附加数据，在查询API和支付通知中原样返回，可作为自定义参数使用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
}

/// 默认自动续费为false
fn default_auto_renew() -> bool {
    false
}

/// 支付宝Native支付（二维码支付）响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlipayQRCodePayResponse {
    /// 支付是否成功
    pub success: bool,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 商户订单号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
    /// 二维码链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_code: Option<String>,
    /// 支付交易ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<i64>,
}

#[tauri::command]
pub async fn get_alipay_qr_code(
    state: State<'_, AppState>,
    req: AlipayQRCodePayRequest,
) -> Result<AlipayQRCodePayResponse> {
    let store_id = utils::get_user_id(&state).await?;
    if store_id == 0 {
        return Err(Error::bad_request("游客无法订阅"));
    }
    let token = state.try_get_token().await?;
    state
        .http_client
        .post("/payment/subscription/alipay", &req, Some(&token))
        .await
}

#[tauri::command]
pub async fn get_alipay_sms_sub_qr_code(
    state: State<'_, AppState>,
    req: AlipayQRCodePayRequest,
) -> Result<AlipayQRCodePayResponse> {
    let store_id = utils::get_user_id(&state).await?;
    if store_id == 0 {
        return Err(Error::bad_request("游客无法订阅"));
    }
    let token = state.try_get_token().await?;
    state
        .http_client
        .post("/payment/sms-sub/alipay", &req, Some(&token))
        .await
}

/// 支付宝支付查询请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlipayPayQueryRequest {
    /// 商家ID
    pub store_id: i64,
    /// 商户订单号
    pub out_trade_no: String,
}

/// 支付宝支付查询响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlipayPayQueryResponse {
    /// 查询是否成功
    pub success: bool,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 交易状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_state: Option<String>,
    /// 交易状态描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_state_desc: Option<String>,
    /// 支付完成时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_time: Option<String>,
    /// 支付宝交易号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[tauri::command]
pub async fn check_alipay_qr_code_payment_status(
    state: State<'_, AppState>,
    req: AlipayPayQueryRequest,
) -> Result<AlipayPayQueryResponse> {
    let token = state.try_get_token().await?;
    state
        .http_client
        .post("/payment/subscription/alipay/query", &req, Some(&token))
        .await
}

#[tauri::command]
pub async fn check_alipay_sms_sub_payment(
    state: State<'_, AppState>,
    req: AlipayPayQueryRequest,
) -> Result<AlipayPayQueryResponse> {
    let token = state.try_get_token().await?;
    state
        .http_client
        .post("/payment/sms-sub/alipay/query", &req, Some(&token))
        .await
}

/// 短信模板结构体（对应数据库表）
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct SmsTemplate {
    // --- 阿里云原始字段 ---
    pub id: i64,                  // 本地数据库主键
    pub template_code: String,    // 模板CODE（如 SMS_123456）
    pub template_name: String,    // 模板名称
    pub template_content: String, // 模板内容（如 "验证码：${code}"）
    pub template_status: String,  // 审核状态
    pub template_type: String,    // 模板类型
    pub reason: Option<String>,   // 审核失败原因（阿里云返回）
    #[serde(with = "aliyun_date_format")]
    pub create_time: NaiveDateTime, // 创建时间（阿里云格式：2023-01-01 12:00:00）
    pub related_sign_name: Option<String>, // 关联签名名称

    // --- 本地管理扩展字段 ---
    pub is_active: bool,                  // 是否启用（本地控制）
    pub visible_to_store: bool,           // 是否启用（本地控制）
    pub update_time: NaiveDateTime,       // 最后更新时间（本地维护）
    pub sync_time: Option<NaiveDateTime>, // 最后一次与阿里云同步的时间
}

mod aliyun_date_format {
    use chrono::NaiveDateTime;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(dt: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&dt.format("%Y-%m-%d %H:%M:%S").to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map_err(serde::de::Error::custom)
    }
}

#[tauri::command]
pub async fn sms_template_list(state: State<'_, AppState>) -> Result<PageResult<SmsTemplate>> {
    let token = state.try_get_token().await?;
    state.http_client.get("/sms/templates", Some(&token)).await
}
