import request from '@/utils/request'

// 查询晾衣架列表
export function listRack(query) {
  return request({
    url: '/system/rack/list',
    method: 'get',
    params: query
  })
}

// 查询晾衣架详细
export function getRack(id) {
  return request({
    url: '/system/rack/' + id,
    method: 'get'
  })
}

// 查询晾衣架详细
export function getAvailableRack() {
  return request({
    url: '/system/rack/position' ,
    method: 'get'
  })
}

// 新增晾衣架
export function addRack(data) {
  return request({
    url: '/system/rack',
    method: 'post',
    data: data
  })
}

// 修改晾衣架
export function updateRack(data) {
  return request({
    url: '/system/rack',
    method: 'put',
    data: data
  })
}

// 删除晾衣架
export function delRack(id) {
  return request({
    url: '/system/rack/' + id,
    method: 'delete'
  })
}
