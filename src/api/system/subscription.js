import request from '@/utils/request'
import invoke from '@/utils/invoke'

// 获取所有订阅套餐
export function getAllPlans() {
  return invoke('get_all_plans')
}

// 获取推荐订阅套餐
export function getRecommendedPlans() {
  return invoke('get_recommended_plans')
}

// 根据类型获取订阅套餐
export function getPlansByType(type) {
  return invoke('get_plans_by_type', { type_ : type })
}

// 获取订阅套餐详情
export function getPlanById(id) {
  return invoke('get_plan_by_id', { id })
}

// 获取商家当前有效的订阅
export function getActiveSubscription(storeId) {
  return invoke('get_active_subscription', { store_id: storeId })
}

// 获取用户所有有效订阅
export function getAllActiveSubscriptions() {
  return invoke('get_all_active_subscriptions')
}

// 获取商家所有订阅记录
export function getStoreSubscriptions(storeId) {
  return invoke('get_store_subscriptions', { store_id: storeId })
}

// 创建订阅
export function createSubscription(subscription) {
  return invoke('create_subscription', { subscription })
}

// 获取订阅详情
export function getSubscription(id) {
  return invoke('get_subscription_by_id', { id })
}

// 更新订阅
export function updateSubscription(data) {
  return invoke('update_subscription', { data })
}

// 取消订阅
export function cancelSubscription(id, reason) {
  return invoke('cancel_subscription', { id, reason })
}

// 获取短信套餐
export function getSmsPlans() {
  return invoke('get_sms_plans')
}

// 检查商家是否有有效订阅
export function checkStoreSubscription(storeId) {
  return invoke('check_store_subscription', { store_id: storeId })
}

// 获取所有订阅记录（分页）
export function getAllSubscriptions(params) {
  return invoke('get_all_subscriptions', { params })
}

// 创建订阅套餐
export function createPlan(plan) {
  return invoke('create_plan', { plan })
}

// 更新订阅套餐
export function updatePlan(plan) {
  return invoke('update_plan', { plan })
}

// 删除订阅套餐
export function deletePlan(id) {
  return invoke('delete_plan', { id })
}

// 获取订阅统计数据
export function getSubscriptionStats() {
  return invoke('get_subscription_stats')
}

// 获取订阅趋势数据
export function getSubscriptionTrends(timeRange) {
  return invoke('get_subscription_trends', { time_range: timeRange })
}

// 获取套餐分布数据
export function getPlanDistribution() {
  return invoke('get_plan_distribution')
}

// 获取营收统计数据
export function getRevenueStats(timeRange) {
  return invoke('get_revenue_stats', { time_range: timeRange })
}

// 获取订阅状态分布数据
export function getStatusDistribution() {
  return invoke('get_status_distribution')
}

// 获取即将到期的订阅
export function getExpiringSubscriptions() {
  return invoke('get_expiring_subscriptions')
}
