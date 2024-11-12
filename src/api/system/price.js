import request from '@/utils/request';
import { invoke } from '@tauri-apps/api/core';

// 查询价格管理列表
export function listPrice(query) {
  const pageParams = { pageSize: query.pageSize, page: query.pageNum };
  const clothPrice = {
    orderType: query.orderType,
    priceNumber: query.priceNumber,
    priceName: query.priceName,
  };
  return invoke('list_cloth_prices_pagination', { pageParams: pageParams, clothPrice: clothPrice })
}

// 查询价格管理详细
export function getPrice(priceId) {
  return invoke('get_cloth_price',{priceId: priceId})
}

// 新增价格管理
export function addPrice(data) {
  return invoke('add_cloth_price',{clothPrice: data})

}

// 修改价格管理
export function updatePrice(data) {
  return invoke('update_cloth_price',{clothPrice: data})
}

// 修改价格状态
export function updatePriceStatus(data) {
  return invoke('update_cloth_price_status', data)
}

export function updatePriceRefNum(data) {
  return invoke('update_cloth_price_ref_num', data)

}
// 删除价格管理
export function delPrice(priceId) {
  return invoke('delete_cloth_prices', {priceIds: [].concat(priceId)})

}
