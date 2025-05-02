import invoke from '@/utils/invoke'

// 查询衣物品类列表
export function listCategory(query) {
  const page_params = { pageSize: query.pageSize, page: query.pageNum };
  const category = {
    categoryName: query.categoryName,
    categoryCode: query.categoryCode,
    storeId: query.storeId || 0
  };
  return invoke('list_clothing_category_pagination', {
    pageParams: page_params, category: category
  })
}

// 查询衣物品类列表（不分页）
export function listCategoryAll(query = {}) {
  const category = {
    categoryName: query.categoryName || '',
    categoryCode: query.categoryCode || ''
  };
  return invoke('list_clothing_category_all', { category: category })
}

// 查询衣物品类详细
export function getCategory(categoryId) {
  return invoke('get_clothing_category_by_id', { id: categoryId })
}

// 新增衣物品类
export function addCategory(data) {
  return invoke('add_clothing_category', { category: data });
}

// 修改衣物品类
export function updateCategory(data) {
  return invoke('update_clothing_category', { category: data });
}

// 删除衣物品类
export function delCategory(categoryId) {
  return invoke('delete_clothing_category_batch', {ids: [].concat(categoryId)})
}