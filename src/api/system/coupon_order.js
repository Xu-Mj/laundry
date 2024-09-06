import request from '@/utils/request'

// 查询卡券订单列表
export function listOrder(query) {
  return request({
    url: '/system/coupon-order/list',
    method: 'get',
    params: query
  })
}

// 查询卡券订单详细
export function getOrder(orderId) {
  return request({
    url: '/system/coupon-order/' + orderId,
    method: 'get'
  })
}

// 新增卡券订单
export function addOrder(data) {
  return request({
    url: '/system/coupon-order',
    method: 'post',
    data: data
  })
}

// 修改卡券订单
export function updateOrder(data) {
  return request({
    url: '/system/coupon-order',
    method: 'put',
    data: data
  })
}

// 删除卡券订单
export function delOrder(orderId) {
  return request({
    url: '/system/order/' + orderId,
    method: 'delete'
  })
}
