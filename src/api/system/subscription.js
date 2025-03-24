import request from '@/utils/request'
import invoke from '@/utils/invoke'

// 获取所有订阅套餐
export function getAllPlans() {
  return request({
    url: '/plans',
    method: 'get'
  })
}

// 获取推荐订阅套餐
export function getRecommendedPlans() {
  return request({
    url: '/plans/recommended',
    method: 'get'
  })
}

// 根据类型获取订阅套餐
export function getPlansByType(type) {
  return request({
    url: `/plans/type/${type}`,
    method: 'get'
  })
}

// 获取订阅套餐详情
export function getPlanById(id) {
  return request({
    url: `/plans/${id}`,
    method: 'get'
  })
}

// 获取商家当前有效的订阅
export function getActiveSubscription(storeId) {
  return request({
    url: `/store/${storeId}/active`,
    method: 'get'
  })
}

// 获取用户所有有效订阅
export function getAllActiveSubscriptions() {
  return invoke('get_user_subscriptions')
}

// 获取商家所有订阅记录
export function getStoreSubscriptions(storeId) {
  return request({
    url: `/subscription/store/${storeId}`,
    method: 'get'
  })
}

// 获取订阅详情
export function saveSubscription(subscription, plan) {
  return invoke('create_subscription', {subscription, plan})
}

// 获取订阅详情
export function getSubscription(storeId, planId,id) {
  return request({
    url: `/subscription/${storeId}/${planId}/${id}`,
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

// 取消订阅
export function getSmsPlans() {
  return request({
    url: `/sms/plans`,
    method: 'get',
  })
}
