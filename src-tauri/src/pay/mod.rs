use serde::{Deserialize, Serialize};
use sqlx::Pool;
use sqlx::Sqlite;
use weapay::alipay::common::BaseTrait;
use weapay::{AlipayConfig, Payment, alipay::prelude::ReqOrderBody};

use crate::db::alipay_config::AlipayConfig as StoreAlipayConfig;
use crate::error::{Error, ErrorKind, Result};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlipayQrCodeResponse {
    pub qr_code: String,
    pub out_trade_no: String,
    pub total_amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlipayPayCodeResponse {
    pub out_trade_no: Option<String>,
    pub total_amount: Option<String>,
    pub trade_no: Option<String>,
    pub trade_status: Option<String>,
}

/// 获取支付宝支付对象
async fn get_alipay_payment(store_id: i64, pool: &Pool<Sqlite>) -> Result<Payment<AlipayConfig>> {
    // 从数据库获取支付宝配置
    let alipay_config = StoreAlipayConfig::get_by_store_id(pool, store_id)
        .await?
        .ok_or_else(|| Error::with_details(ErrorKind::NotFound, "支付宝配置未找到"))?;

    // 创建支付宝支付对象
    let app_private_key = alipay_config.private_key;
    let alipay_public_cert = alipay_config.alipay_public_key;

    // 创建支付宝支付对象
    let payment = Payment::new(AlipayConfig {
        app_id: alipay_config.app_id,
        app_private_key,
        alipay_public_cert,
        is_sandbox: Some(alipay_config.is_sandbox),
        ..Default::default()
    });

    Ok(payment)
}

/// 使用付款码支付（商家扫用户付款码）
pub async fn pay_with_alipay_auth_code(
    pool: &Pool<Sqlite>,
    store_id: i64,
    req_body: ReqOrderBody,
) -> Result<AlipayPayCodeResponse> {
    // 获取支付宝支付对象
    let payment = get_alipay_payment(store_id, pool).await?;

    // 发起支付请求
    let resp = payment
        .create_order("alipay.trade.pay", req_body)
        .await
        .map_err(|e| {
            Error::with_details(
                ErrorKind::InternalServer,
                &format!("支付宝付款码支付失败: {}", e),
            )
        })?;

    // 返回支付结果
    Ok(AlipayPayCodeResponse {
        out_trade_no: resp.out_trade_no,
        total_amount: resp.total_amount,
        trade_no: resp.trade_no,
        trade_status: resp.trade_status,
    })
}
