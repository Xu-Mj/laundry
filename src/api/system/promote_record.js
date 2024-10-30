import request from '@/utils/request'

// 查询推广记录列表
export function listRecord(query) {
  return request({
    url: '/system/promote-record/list',
    method: 'get',
    params: query
  })
}

// 查询推广记录详细
export function getRecord(id) {
  return request({
    url: '/system/promote-record/' + id,
    method: 'get'
  })
}

// 新增推广记录
export function addRecord(data) {
  return request({
    url: '/system/promote-record',
    method: 'post',
    data: data
  })
}

// 修改推广记录
export function updateRecord(data) {
  return request({
    url: '/system/promote-record',
    method: 'put',
    data: data
  })
}

// 删除推广记录
export function delRecord(id) {
  return request({
    url: '/system/promote-record/' + id,
    method: 'delete'
  })
}
