import request from '@/utils/request'
import invoke from '@/utils/invoke'

// 查询订单包含的衣物清单列表
export function listCloths(query) {
  return invoke('list_order_clothes', { orderId: query.orderId });
}

// 查询订单包含的衣物清单列表
export function rewash(query) {
  return request({
    url: '/system/cloths/rewash/' + query,
    method: 'post',
  })
}

// 查询订单包含的衣物清单列表
export function listHistoryCloths(query) {
  const pageParams = { pageSize: query.pageSize, page: query.pageNum, params: query.params };
  const userId = query.userId;
  return invoke('list_order_clothes_history', { pageParams: pageParams, userId: userId });
}

// 查询订单包含的衣物清单详细
export function getCloths(clothId) {
  return invoke('get_order_cloth_by_id', { clothId });
}

// 查询订单包含的衣物清单详细
export function getClothByCode(code) {
  return invoke('get_order_cloth_by_code', { code });
}

// 查询订单包含的衣物清单详细
export function getPic(picId) {
  return request({
    url: '/system/cloths/download/' + picId,
    method: 'get'
  })
}

// 新增订单包含的衣物清单
export function addCloths(data) {
  return invoke('add_order_cloth', { orderCloth: data });
}

// 取走
export function pickUp(ids) {
  return invoke('pickup_order_cloth', { clothesId: ids });
}

// 修改订单包含的衣物清单
export function updateCloths(cloth) {
  return invoke('update_order_cloth', { cloth });
}

// 上挂
export function hangup(hangReq) {
  return invoke('hang_order_cloth', { hangReq });
}

// 删除订单包含的衣物清单
export function delClothPicture(clothId, picId) {
  return invoke('remove_pic_from_order_cloth', { clothId, picId });
}

// 删除订单包含的衣物清单
export function delCloths(orderClothId) {
  return invoke('delete_order_cloth_by_ids', { ids: [].concat(orderClothId) });
}
