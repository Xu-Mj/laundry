import { invoke } from '@tauri-apps/api/core';

// 查询晾衣架列表
export function listRack() {
  return invoke('list_rack_all')
}

// 查询晾衣架详细
export function getRack(id) {
  return invoke('get_rack_by_id', { id: id })
}

// 新增晾衣架
export function addRack(data) {
  return invoke('add_rack', { rack: data })
}

// 修改晾衣架
export function updateRack(data) {
  return invoke('update_rack', { rack: data })
}

// 删除晾衣架
export function delRack(id) {
  return invoke('delete_racks', { ids: [].concat(id) })
}

export function checkRackInitialData() {
  return invoke('check_rack_initial_data')
}
