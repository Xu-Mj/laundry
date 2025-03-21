import request from '@/utils/request'
import invoke from '@/utils/invoke'

// 发起订阅支付请求
export function createSubscriptionPaymentWithAlipay(data) {
  return request({
    url: '/payment/alipay/precreate',
    method: 'post',
    data: data
  })
}

// 查询订阅支付状态
export function querySubscriptionPaymentWithAlipay(data) {
  return request({
    url: '/payment/alipay/trade/query',
    method: 'post',
    data: data
  })
}

// 发起订阅支付请求
export function createSubscriptionPaymentWithWechat(data) {
  return request({
    url: '/payment/wechat/precreate',
    method: 'post',
    data: data
  })
}

// 查询订阅支付状态
export function querySubscriptionPaymentWithWechat(data) {
  return request({
    url: '/payment/wechat/trade/query',
    method: 'post',
    data: data
  })
}