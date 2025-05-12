use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    error::{Error, Result},
    state::AppState,
    utils,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionPlan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub name: String,
    pub plan_type: String,
    pub period: String,
    pub price: String,
    pub description: Option<String>,
    pub features: Option<serde_json::Value>,
    pub is_recommended: bool,
    pub is_active: bool,
    pub sort_order: i32,
    pub created_at: i64,
    pub updated_at: i64,
    pub remark: Option<String>,
}

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
pub async fn get_sms_plans(state: State<'_, AppState>) -> Result<Vec<SubscriptionPlan>> {
    state.http_client.get("/sms/plans", None).await
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
