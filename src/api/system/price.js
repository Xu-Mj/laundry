import invoke from '@/utils/invoke'

// 查询价格管理列表
export function listPricePagination(query) {
  const pageParams = { pageSize: query.pageSize, page: query.pageNum, params: query.params };
  const clothPrice = {
    orderType: query.orderType,
    priceName: query.priceName,
  }
  return invoke('list_cloth_prices_pagination', { pageParams: pageParams, clothPrice: clothPrice })
}
// 查询价格管理列表
export function listPrice(query) {
  return invoke('list_cloth_prices_by_order_type', query)
}

// 查询价格管理详细
export function getPrice(priceId) {
  return invoke('get_cloth_price', { priceId: priceId })
}

// 新增价格管理
export function addPrice(data) {
  return invoke('add_cloth_price', { clothPrice: data })

}

// 修改价格管理
export function updatePrice(data) {
  return invoke('update_cloth_price', { clothPrice: data })
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
  return invoke('delete_cloth_prices', { priceIds: [].concat(priceId) })

}
