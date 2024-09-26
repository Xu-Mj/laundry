import request from '@/utils/request'

// 查询卡券列表
export function listCoupon(query) {
  return request({
    url: '/system/coupon/list',
    method: 'get',
    params: query
  })
}

// 查询卡券列表
export function listCoupon4sale(query) {
  return request({
    url: '/system/coupon/list4sale',
    method: 'get',
    params: query
  })
}

// 查询卡券详细
export function getCoupon(couponId) {
  return request({
    url: '/system/coupon/' + couponId,
    method: 'get'
  })
}

// 新增卡券
export function addCoupon(data) {
  return request({
    url: '/system/coupon',
    method: 'post',
    data: data
  })
}

// 购买卡券
export function buyCoupon(data) {
  return request({
    url: '/system/coupon/buy',
    method: 'post',
    data: data
  })
}

// 修改卡券
export function updateCoupon(data) {
  return request({
    url: '/system/coupon',
    method: 'put',
    data: data
  })
}

// 删除卡券
export function delCoupon(couponId) {
  return request({
    url: '/system/coupon/' + couponId,
    method: 'delete'
  })
}
