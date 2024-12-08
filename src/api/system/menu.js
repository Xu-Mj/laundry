import request from '@/utils/request';
import invoke from '@/utils/invoke'

// 查询菜单列表
export function listMenu(query) {
  return invoke('get_menu_list', {menu: query?query:{}})
}

// 查询菜单详细
export function getMenu(id) {
  return invoke('get_menu_by_id', {id})
}

// 查询菜单下拉树结构
export function treeselect() {
  return request({
    url: '/system/menu/treeselect',
    method: 'get'
  })
}

// 根据角色ID查询菜单下拉树结构
export function roleMenuTreeselect(roleId) {
  return request({
    url: '/system/menu/roleMenuTreeselect/' + roleId,
    method: 'get'
  })
}

// 新增菜单
export function addMenu(menu) {
  return invoke('add_menu', {menu})
}

// 修改菜单
export function updateMenu(menu) {
  return invoke('update_menu', {menu})
}

// 删除菜单
export function delMenu(id) {
  return invoke('delete_menu', {id})
}