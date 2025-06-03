import request from '@/utils/request'
import invoke from '@/utils/invoke'

// 发起订阅支付请求
export function createSubscriptionPaymentWithAlipay(data) {
  return invoke('get_alipay_qr_code', { req: data })
}

// 查询订阅支付状态
export function querySubscriptionPaymentWithAlipay(data) {
  return invoke('check_alipay_qr_code_payment_status', { req: data })
}

// wechat
export function createSubscriptionPaymentWithWechat(data) {
  return invoke('get_wechat_qr_code', { req: data })
}

// 查询订阅支付状态
export function querySubscriptionPaymentWithWechat(data) {
  return invoke('check_wechat_qr_code_payment_status', { req: data })
}

// 发起订阅支付请求
export function createSmsSubPaymentWithAlipay(data) {
  return invoke('get_alipay_sms_sub_qr_code', { req: data })
}

// 查询订阅支付状态
export function querySmsSubPaymentWithAlipay(data) {
  return invoke('check_alipay_sms_sub_payment', { req: data })
}

// wechat
export function createSmsSubPaymentWithWechat(data) {
  return invoke('get_wechat_sms_sub_qr_code', { req: data })
}

// 查询订阅支付状态
export function querySmsSubPaymentWithWechat(data) {
  return invoke('check_wechat_sms_sub_qr_code_payment_status', { req: data })
}
