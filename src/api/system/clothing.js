import request from '@/utils/request'

// 查询衣物管理列表
export function listClothing(query) {
  return request({
    url: '/system/clothing/list',
    method: 'get',
    params: query
  })
}

// 查询衣物管理详细
export function getClothing(clothingId) {
  return request({
    url: '/system/clothing/' + clothingId,
    method: 'get'
  })
}

// 新增衣物管理
export function addClothing(data) {
  return request({
    url: '/system/clothing',
    method: 'post',
    data: data
  })
}

// 修改衣物管理
export function updateClothing(data) {
  return request({
    url: '/system/clothing',
    method: 'put',
    data: data
  })
}

// 删除衣物管理
export function delClothing(clothingId) {
  return request({
    url: '/system/clothing/' + clothingId,
    method: 'delete'
  })
}
