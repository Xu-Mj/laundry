import request from '@/utils/request'

// 获取所有订阅套餐
export function getAllPlans() {
  return request({
    url: '/subscription/plans',
    method: 'get'
  })
}

// 获取推荐订阅套餐
export function getRecommendedPlans() {
  return request({
    url: '/subscription/plans/recommended',
    method: 'get'
  })
}

// 根据类型获取订阅套餐
export function getPlansByType(type) {
  return request({
    url: `/subscription/plans/type/${type}`,
    method: 'get'
  })
}

// 获取订阅套餐详情
export function getPlanById(id) {
  return request({
    url: `/subscription/plans/${id}`,
    method: 'get'
  })
}

// 获取商家当前有效的订阅
export function getActiveSubscription(storeId) {
  return request({
    url: `/subscription/store/${storeId}/active`,
    method: 'get'
  })
}

// 获取商家所有订阅记录
export function getStoreSubscriptions(storeId) {
  return request({
    url: `/subscription/store/${storeId}`,
    method: 'get'
  })
}

// 获取所有订阅记录
export function getAllSubscriptions(params) {
  return request({
    url: '/subscription/list',
    method: 'get',
    params: params
  })
}

// 获取订阅详情
export function getSubscriptionById(id) {
  return request({
    url: `/subscription/${id}`,
    method: 'get'
  })
}

// 更新订阅
export function updateSubscription(data) {
  return request({
    url: '/subscription',
    method: 'put',
    data: data
  })
}

// 取消订阅
export function cancelSubscription(id, reason) {
  return request({
    url: `/subscription/${id}/cancel`,
    method: 'post',
    data: { cancellationReason: reason }
  })
}
