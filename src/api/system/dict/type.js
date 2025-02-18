import invoke from '@/utils/invoke'

// 查询字典类型列表
export function listType(query) {
  const pageParams = { pageSize: query.pageSize, page: query.pageNum, params: query.params };
  const dictType = {
    dictName: query.dictName,
    dictType: query.dictType,
    status: query.status
  };
  return invoke('get_dict_type_list', { pageParams, dictType })
}

// 查询字典类型详细
export function getType(id) {
  return invoke('get_dict_type_by_id', { id })
}

// 查询字典类型详细
export function getTypeByType(dictType) {
  return invoke('get_dict_type_by_type', { dictType })
}

// 新增字典类型
export function addType(data) {
  return invoke('add_dict_type', { dictType: data })
}

// 修改字典类型
export function updateType(data) {
  return invoke('update_dict_type', { dictType: data })
}

// 删除字典类型
export function delType(dictId) {
  return invoke('delete_dict_types', { ids: [].concat(dictId) })
}

// 获取字典选择框列表
export function optionselect() {
  return invoke('get_dict_type_all', { dictType: {} })
}
