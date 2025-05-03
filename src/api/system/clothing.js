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

// 查询衣物管理列表
export function listClothingWithNoLimit() {
  return invoke('list_clothing_all', { clothing: {} })
}

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
export function updateClothing(clothing) {
  return invoke('update_clothing', { clothing });
}

export function updateClothingRefNum(data) {
  return invoke('update_clothing_ref_num', data)
}

// 删除衣物管理
export function delClothing(clothingId) {
  return invoke('delete_clothing_batch', { ids: [].concat(clothingId) })
}
