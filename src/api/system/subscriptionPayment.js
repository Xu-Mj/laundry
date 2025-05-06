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

// 发起订阅支付请求
export function createSubscriptionPaymentWithWechat(data) {
  return request({
    url: '/payment/subscription/wechat',
    method: 'post',
    data: data
  })
}

// 查询订阅支付状态
export function querySubscriptionPaymentWithWechat(data) {
  return request({
    url: '/payment/subscription/wechat/query',
    method: 'post',
    data: data
  })
}