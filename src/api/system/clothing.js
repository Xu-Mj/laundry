import request from '@/utils/request'
import invoke from '@/utils/invoke'

// 查询衣物管理列表
export function listClothing(query) {
  const page_params = { pageSize: query.pageSize, page: query.pageNum };
  const clothing = {
    // 映射旧字段到新字段
    category_id: query.clothingCategory,
    title: query.clothingName,
    // 其他查询条件可以根据需要添加
  };
  return invoke('list_clothing_pagination', {
    pageParams: page_params, clothing: clothing
  })
}
// export function listClothing(query) {
//   return request({
//     url: '/system/clothing/list',
//     method: 'get',
//     params: query
//   })
// }

// 查询衣物管理列表
export function listClothingWithNoLimit() {
  return invoke('list_clothing_all', { clothing: {} })
}
// export function listClothingWithNoLimit() {
//   return request({
//     url: '/system/clothing/list-no-limit',
//     method: 'get',
//   })
// }

// 查询衣物管理详细
export function getClothing(clothingId) {
  return invoke('get_clothing_by_id', { id: clothingId })
}

// 新增衣物管理
export function addClothing(clothing) {
  return invoke('add_clothing', { clothing });
}

// 新增衣物管理
export function createClothingCreateOrder(clothing) {
  return invoke('create_clothing_4_create_order', { clothing });
}

// 修改衣物管理
export function updateClothing(data) {
  // 将前端字段映射到后端字段
  const clothing = {
    id: data.id || data.clothingId,
    category_id: data.clothingCategory,
    style_id: data.clothingStyle,
    title: data.title || data.clothingName,
    clothing_base_price: data.clothingBasePrice,
    clothing_min_price: data.clothingMinPrice,
    order_num: data.orderNum,
    clothing_degree: data.clothingDegree,
    hang_type: data.hangType || '1',
    tag_list: data.tag_list || (data.tagList && data.tagList.join(',')),
    primary_image: data.primaryImage,
    images: Array.isArray(data.images) ? data.images.join(',') : data.images,
    is_put_on_sale: data.isPutOnSale !== undefined ? data.isPutOnSale : false,
    stock_quantity: data.stockQuantity || 0,
    // 保留其他可能存在的字段
    store_id: data.storeId
  };
  return invoke('update_clothing', { clothing: clothing });
}

export function updateClothingRefNum(data) {
  return invoke('update_clothing_ref_num', data)
}

// 删除衣物管理
export function delClothing(clothingId) {
  return invoke('delete_clothing_batch', {ids: [].concat(clothingId)})
}
