import request from '@/utils/request'

// 查询派送列表
export function listDispatch(query) {
  return request({
    url: '/system/dispatch/list',
    method: 'get',
    params: query
  })
}

// 查询派送详细
export function getDispatch(dispatchId) {
  return request({
    url: '/system/dispatch/' + dispatchId,
    method: 'get'
  })
}

// 新增派送
export function addDispatch(data) {
  return request({
    url: '/system/dispatch',
    method: 'post',
    data: data
  })
}

// 修改派送
export function updateDispatch(data) {
  return request({
    url: '/system/dispatch',
    method: 'put',
    data: data
  })
}

// 删除派送
export function delDispatch(dispatchId) {
  return request({
    url: '/system/dispatch/' + dispatchId,
    method: 'delete'
  })
}
