import request from '@/utils/request'

// 查询支付记录列表
export function listPayment(query) {
  return request({
    url: '/system/payment/list',
    method: 'get',
    params: query
  })
}

// 查询支付记录详细
export function getPayment(payId) {
  return request({
    url: '/system/payment/' + payId,
    method: 'get'
  })
}

// 新增支付记录
export function addPayment(data) {
  return request({
    url: '/system/payment',
    method: 'post',
    data: data
  })
}

// 修改支付记录
export function updatePayment(data) {
  return request({
    url: '/system/payment',
    method: 'put',
    data: data
  })
}

// 删除支付记录
export function delPayment(payId) {
  return request({
    url: '/system/payment/' + payId,
    method: 'delete'
  })
}
