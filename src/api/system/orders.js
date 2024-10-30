import request from '@/utils/request'

// 查询洗护服务订单列表
export function listOrders(query) {
  return request({
    url: '/system/orders/list',
    method: 'get',
    params: query
  })
}

// 查询洗护服务订单列表
export function listOrdersWithOutLimit(query) {
  return request({
    url: '/system/orders/list-no-limit',
    method: 'get',
    params: query
  })
}

// 查询洗护服务订单列表
export function selectListExceptCompleted(query) {
  return request({
    url: '/system/orders/list/4index',
    method: 'get',
    params: query
  })
}

// 查询洗护服务订单详细
export function getOrders(orderId) {
  return request({
    url: '/system/orders/' + orderId,
    method: 'get'
  })
}

// 查询退单所需的信息，用户手机号、订单实际支付金额
export function getRefundInfo(orderId, userId) {
  return request({
    url: '/system/orders/refund/' + orderId + '/' + userId,
    method: 'get'
  })
}

// 新增洗护服务订单
export function addOrders(data) {
  return request({
    url: '/system/orders',
    method: 'post',
    data: data
  })
}

// 新增洗护服务订单
export function addRewashOrder(data) {
  return request({
    url: '/system/orders/rewash',
    method: 'post',
    data: data
  })
}

// 修改洗护服务订单
export function updateOrders(data) {
  return request({
    url: '/system/orders',
    method: 'put',
    data: data
  })
}

// 修改洗护服务订单
export function updateAdjust(data) {
  return request({
    url: '/system/orders/adjust',
    method: 'put',
    data: data
  })
}

// 退款
export function pay(data) {
  return request({
    url: '/system/orders/pay',
    method: 'post',
    data: data
  })
}

// 退款
export function refund(data) {
  return request({
    url: '/system/orders/refund',
    method: 'put',
    data: data
  })
}

// 删除洗护服务订单
export function delOrders(orderId) {
  return request({
    url: '/system/orders/' + orderId,
    method: 'delete'
  })
}
