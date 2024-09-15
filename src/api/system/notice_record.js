import request from '@/utils/request'

// 查询通知记录管理列表
export function listRecord(query) {
  return request({
    url: '/system/notice-record/list',
    method: 'get',
    params: query
  })
}

// 查询通知记录管理详细
export function getRecord(noticeId) {
  return request({
    url: '/system/notice-record/' + noticeId,
    method: 'get'
  })
}

// 新增通知记录管理
export function addRecord(data) {
  return request({
    url: '/system/notice-record',
    method: 'post',
    data: data
  })
}

// 修改通知记录管理
export function updateRecord(data) {
  return request({
    url: '/system/notice-record',
    method: 'put',
    data: data
  })
}

// 删除通知记录管理
export function delRecord(noticeId) {
  return request({
    url: '/system/notice-record/' + noticeId,
    method: 'delete'
  })
}

// 删除所有通知记录管理
export function delAllRecord() {
  return request({
    url: '/system/notice-record/',
    method: 'delete'
  })
}

// 删除30天通知记录管理
export function delRecordsByDay(day) {
  return request({
    url: '/system/notice-record/day/' + day,
    method: 'delete'
  })
}

