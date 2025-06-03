use serde::{Deserialize, Serialize};
use sqlx::Pool;
use sqlx::Sqlite;
use wechat_pay_rust_sdk::model::NativeParams;
use wechat_pay_rust_sdk::pay::WechatPay;

use crate::db::wechat_config::WechatConfig as StoreWechatConfig;
use crate::error::{Error, ErrorKind, Result};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WechatQrCodeResponse {
    pub qr_code: String,
    pub out_trade_no: String,
    pub total_amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WechatPayCodeResponse {
    pub out_trade_no: Option<String>,
    pub total_amount: Option<String>,
    pub transaction_id: Option<String>,
    pub trade_state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WechatAuthCodeRequest {
    pub out_trade_no: String,
    pub subject: String,
    pub total_amount: String,
    pub auth_code: String,
}

/// 获取微信支付客户端
async fn get_wechat_client(store_id: i64, pool: &Pool<Sqlite>) -> Result<WechatPay> {
    // 从数据库获取微信支付配置
    let wechat_config = StoreWechatConfig::get_by_store_id(pool, store_id)
        .await?
        .ok_or_else(|| Error::with_details(ErrorKind::NotFound, "微信支付配置未找到"))?;

    // 创建微信支付客户端
    // 注意：这里需要根据实际的wechat-pay-rust-sdk API进行调整
    // 当前数据库结构中没有serial_no、v3_key、notify_url等字段
    // 可能需要扩展数据库结构或使用现有字段
    let wechat_pay = WechatPay::new(
        &wechat_config.app_id,
        &wechat_config.mchid,
        &wechat_config.apiclient_key,         // 使用私钥文件路径
        &"serial_no_placeholder".to_string(), // 需要在数据库中添加此字段
        &wechat_config.mch_key,               // 使用商户密钥作为v3_key
        &"https://your-domain.com/wechat/notify".to_string(), // 需要配置回调地址
    );

    Ok(wechat_pay)
}

/// 微信Native支付（生成二维码）
pub async fn create_wechat_native_pay(
    pool: &Pool<Sqlite>,
    store_id: i64,
    out_trade_no: String,
    description: String,
    total_amount: i32, // 单位：分
) -> Result<WechatQrCodeResponse> {
    // 获取微信支付客户端
    let wechat_pay = get_wechat_client(store_id, pool).await?;

    // 创建Native支付参数
    let params = NativeParams::new(&description, &out_trade_no, total_amount.into());

    // 发起Native支付请求
    let resp = wechat_pay.native_pay(params).map_err(|e| {
        Error::with_details(
            ErrorKind::InternalServer,
            &format!("微信Native支付失败: {}", e),
        )
    })?;

    // 返回支付结果
    Ok(WechatQrCodeResponse {
        qr_code: resp.code_url.unwrap_or_default(),
        out_trade_no,
        total_amount: total_amount as f64 / 100.0, // 转换为元
    })
}
