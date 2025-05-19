import invoke from '@/utils/invoke';

// 取走
export function delivery(data) {
    return invoke('create_delivery', { delivery: data });
  }
  
  // 完成派送
  export function completeDelivery(deliveryId) {
    return invoke('complete_delivery', { deliveryId });
  }
  
  // 取消派送
  export function cancelDelivery(deliveryId) {
    return invoke('cancel_delivery', { deliveryId });
  }
  
  // 获取派送详情
  export function getDeliveryById(deliveryId) {
    return invoke('get_delivery_by_id', { deliveryId });
  }
  
  // 获取派送列表
  export function listDeliveries(query) {
    const pageParams = { pageSize: query.pageSize, page: query.pageNum, params: query.params };
    return invoke('list_deliveries', { pageParams, delivery: query });
  }
  
  // 获取可派送的衣物列表
  export function listDeliveryEligibleClothes(userId) {
    return invoke('list_delivery_eligible_clothes', { userId });
  }
