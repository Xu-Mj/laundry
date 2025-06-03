use alipay_sdk_rust::biz::{self, BizContenter};
use alipay_sdk_rust::pay::{PayClient, Payer};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use crate::db::alipay_config::AlipayConfig as StoreAlipayConfig;
use crate::error::{Error, ErrorKind, Result};

// 微信支付模块
pub mod wechat;
pub use wechat::*;

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

/// 获取支付宝支付客户端
async fn get_alipay_client(store_id: i64, pool: &Pool<Sqlite>) -> Result<impl Payer> {
    // 从数据库获取支付宝配置
    let alipay_config = StoreAlipayConfig::get_by_store_id(pool, store_id)
        .await?
        .ok_or_else(|| Error::with_details(ErrorKind::NotFound, "支付宝配置未找到"))?;

    // 创建支付宝支付客户端
    let api_url = if alipay_config.is_sandbox {
        "https://openapi-sandbox.dl.alipaydev.com/gateway.do"
    } else {
        "https://openapi.alipay.com/gateway.do"
    };

    let client = PayClient::builder()
        .api_url(api_url)
        .app_id(&alipay_config.app_id)
        .alipay_public_key(&alipay_config.alipay_public_key)
        .private_key(&alipay_config.private_key)
        .public_key(&alipay_config.alipay_public_key)
        .charset_utf8()
        .format_json()
        .sign_type_rsa2()
        .version_1_0()
        .build()
        .map_err(|e| {
            Error::with_details(
                ErrorKind::InternalServer,
                &format!("创建支付宝客户端失败: {}", e),
            )
        })?;

    Ok(client)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlipayAuthCodeRequest {
    pub out_trade_no: String,
    pub subject: String,
    pub total_amount: String,
    pub auth_code: String,
    pub scene: String,
}

/// 使用付款码支付（商家扫用户付款码）
pub async fn pay_with_alipay_auth_code(
    pool: &Pool<Sqlite>,
    store_id: i64,
    req: AlipayAuthCodeRequest,
) -> Result<AlipayPayCodeResponse> {
    // 获取支付宝支付客户端
    let client = get_alipay_client(store_id, pool).await?;

    // 创建交易支付请求
    let mut biz_content = biz::TradePayBiz::new();
    biz_content.set_subject(req.subject.into());
    biz_content.set_outtrade_no(req.out_trade_no.clone().into());
    biz_content.set("total_amount", req.total_amount.clone().into());
    biz_content.set_auth_code(req.auth_code.into());
    biz_content.set_scene(req.scene.into());

    // 发起支付请求
    let resp = client.trade_pay(&biz_content).map_err(|e| {
        Error::with_details(
            ErrorKind::InternalServer,
            &format!("支付宝付款码支付失败: {}", e),
        )
    })?;

    // 返回支付结果
    Ok(AlipayPayCodeResponse {
        out_trade_no: Some(req.out_trade_no),
        total_amount: Some(req.total_amount),
        trade_no: resp.response.trade_no,
        trade_status: resp.response.msg,
    })
}
