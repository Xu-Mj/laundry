import request from '@/utils/request'
import invoke from '@/utils/invoke'

// 查询通知记录管理列表
export function listRecord(query) {
  const pageParams = { pageSize: query.pageSize, page: query.pageNum, params: query.params }
  const record = {
    phonenumber: query.phonenumber,
    orderNumber: query.orderNumber
  };
  return invoke('get_notice_record_pagination', { pageParams, record })
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
  return invoke('delete_all_record')
}

// 删除30天通知记录管理
export function delRecordsByDay(days) {
  return invoke('delete_old_record', {days})

}

