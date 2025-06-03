# 支付依赖迁移说明

## 概述

本次迁移将项目中的 `weapay` 依赖替换为独立的微信支付和支付宝支付SDK：

- **支付宝SDK**: `alipay_sdk_rust` v1.0.8
- **微信支付SDK**: `wechat-pay-rust-sdk` v0.1.0

## 已完成的工作

### 1. 依赖更新
- ✅ 移除 `weapay = "0.1.0"` 依赖
- ✅ 添加 `alipay_sdk_rust = "1.0.8"` 依赖
- ✅ 添加 `wechat-pay-rust-sdk = "0.1.0"` 依赖

### 2. 支付宝支付模块重构
- ✅ 更新 `src/pay/mod.rs` 中的支付宝支付逻辑
- ✅ 替换 `weapay::alipay` 为 `alipay_sdk_rust`
- ✅ 创建新的 `AlipayAuthCodeRequest` 结构体
- ✅ 重构 `pay_with_alipay_auth_code` 函数
- ✅ 更新 `src/db/orders.rs` 中的支付宝调用代码

### 3. 微信支付模块创建
- ✅ 创建 `src/pay/wechat.rs` 微信支付模块
- ✅ 实现基础的微信Native支付功能
- ✅ 在 `src/pay/mod.rs` 中导入微信支付模块

## 需要后续完成的工作

### 1. 微信支付配置完善

当前微信支付配置缺少以下字段，需要扩展数据库结构：

```sql
ALTER TABLE wechat_configs ADD COLUMN serial_no TEXT;
ALTER TABLE wechat_configs ADD COLUMN v3_key TEXT;
ALTER TABLE wechat_configs ADD COLUMN notify_url TEXT;
```

### 2. 支付宝配置验证

确保支付宝配置表包含以下必要字段：
- `app_id`: 应用ID
- `private_key`: 应用私钥
- `alipay_public_key`: 支付宝公钥
- `is_sandbox`: 是否沙箱环境

### 3. 证书文件处理

- **支付宝**: 确保私钥格式为PKCS1（非Java适用）
- **微信**: 需要处理证书文件路径或将证书内容存储在数据库中

### 4. 错误处理优化

根据新SDK的错误类型，优化错误处理和用户提示信息。

### 5. 功能扩展

根据业务需求，可能需要实现以下功能：

#### 支付宝功能
- 二维码支付（Native支付）
- H5支付
- APP支付
- 支付结果查询
- 退款功能

#### 微信支付功能
- JSAPI支付
- H5支付
- APP支付
- 小程序支付
- 支付结果查询
- 退款功能

## 测试建议

1. **沙箱环境测试**
   - 配置支付宝沙箱环境
   - 配置微信支付测试环境
   - 测试付款码支付流程

2. **集成测试**
   - 测试订单创建到支付完成的完整流程
   - 测试支付失败的异常处理
   - 测试支付状态更新逻辑

3. **性能测试**
   - 测试高并发支付请求
   - 测试支付接口响应时间

## 注意事项

1. **API兼容性**: 新SDK的API可能与原有的`weapay`不完全兼容，需要仔细测试
2. **证书安全**: 确保支付证书和密钥的安全存储
3. **回调处理**: 需要实现支付结果的异步回调处理
4. **日志记录**: 添加详细的支付日志，便于问题排查

## 相关文档

- [支付宝SDK文档](https://github.com/wandercn/alipay_sdk_rust)
- [微信支付SDK文档](https://github.com/dounine/wechat-pay-rust-sdk)
- [支付宝开放平台](https://opendocs.alipay.com/)
- [微信支付开发文档](https://pay.weixin.qq.com/wiki/doc/api/index.html)