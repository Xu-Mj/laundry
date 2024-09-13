import request from '@/utils/request'

// 查询用户卡券列表
export function listUserCoupon(query) {
  return request({
    url: '/system/user-coupon/list',
    method: 'get',
    params: query
  })
}

// 查询用户卡券详细
export function getUserCoupon(ucId) {
  return request({
    url: '/system/user-coupon/' + ucId,
    method: 'get'
  })
}

// 新增用户卡券
export function addUserCoupon(data) {
  return request({
    url: '/system/user-coupon',
    method: 'post',
    data: data
  })
}

// 修改用户卡券
export function updateUserCoupon(data) {
  return request({
    url: '/system/user-coupon',
    method: 'put',
    data: data
  })
}

// 删除用户卡券
export function delUserCoupon(ucId) {
  return request({
    url: '/system/user-coupon/' + ucId,
    method: 'delete'
  })
}
