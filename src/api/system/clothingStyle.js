import invoke from '@/utils/invoke'

// 查询衣物分类列表
export function listStyle(query) {
  const page_params = { pageSize: query.pageSize, page: query.pageNum };
  const style = {
    styleName: query.styleName,
    styleCode: query.styleCode,
    categoryId: query.categoryId || 0,
    storeId: query.storeId || 0
  };
  return invoke('list_clothing_style_pagination', {
    pageParams: page_params, style: style
  })
}

// 查询衣物分类列表（不分页）
export function listStyleAll(query = {}) {
  const style = {
    storeId: query.storeId || 0,
    categoryId: query.categoryId || 0,
    styleName: query.styleName || '',
    styleCode: query.styleCode || ''
  };
  return invoke('list_clothing_style_all', { style: style })
}

// 根据品类ID查询分类列表
export function listStyleByCategoryId(categoryId) {
  return invoke('get_clothing_style_by_category_id', { categoryId })
}

// 查询衣物分类详细
export function getStyle(styleId) {
  return invoke('get_clothing_style_by_id', { id: styleId })
}

// 新增衣物分类
export function addStyle(data) {
  return invoke('add_clothing_style', { style: data });
}

// 修改衣物分类
export function updateStyle(data) {
  return invoke('update_clothing_style', { style: data });
}

// 删除衣物分类
export function delStyle(styleId) {
  return invoke('delete_clothing_style_batch', {ids: [].concat(styleId)})
}