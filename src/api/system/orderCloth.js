import invoke from '@/utils/invoke'

// 根据ID获取衣物详情
export function getOrderClothById(clothId) {
  return invoke('get_order_cloth_by_id', { cloth_id: clothId });
}

// 根据ID数组获取多个衣物详情
export function getOrderClothByIds(clothIds) {
  return invoke('get_order_cloths_by_ids', { clothIds });
}

// 获取可派送的衣物列表
export function getDeliveryEligibleClothes(userId) {
  return invoke('list_delivery_eligible_clothes', { user_id: userId });
} 