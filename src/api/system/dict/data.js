import invoke from '@/utils/invoke'

// 查询字典数据列表
export function listData(query) {
  const pageParams = { pageSize: query.pageSize, page: query.pageNum, params: query.params };
  const dictData = {
    dictType: query.dictType,
    dictLabel: query.dictLabel,
    status: query.status
  };
  return invoke('get_dict_data_list', { pageParams, dictData })
}

// 查询字典数据详细
export function getData(code) {
  return invoke('get_dict_data_by_code', { code })
}

// 根据字典类型查询字典数据信息
export function getDicts(dictType) {
  return invoke('get_by_dict_type', { dictType })
}

// 新增字典数据
export function addData(data) {
  return invoke('add_dict_data', { dictData: data })
}

// 修改字典数据
export function updateData(data) {
  return invoke('update_dict_data', { dictData: data })
}

// 删除字典数据
export function delData(dictCode) {
  return invoke('delete_dict_data', { ids: [].concat(dictCode) })
}
