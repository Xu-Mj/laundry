import invoke from '@/utils/invoke'

// 查询参数列表
export function listConfig(query) {
  const pageParams = { pageSize: query.pageSize, page: query.pageNum, params: query.params };
  const config = {
    configType: query.configType,
    configName: query.configName,
  }
  return invoke('get_config_list', { pageParams: pageParams, config: config })
}

// 查询参数详细
export function getConfig(configId) {
 return invoke('get_config_by_id', { id: configId })
}

// 根据参数键名查询参数值
export function getConfigKey(configKey) {
  return invoke('get_config_by_key', { key: configKey })
}

// 新增参数配置
export function addConfig(data) {
  return invoke('add_config', { config: data })
}

// 修改参数配置
export function updateConfig(data) {
  return invoke('update_config', { config: data })
}

// 删除参数配置
export function delConfig(configId) {
  return invoke('delete_configs', { ids: [].concat(configId) })
}
