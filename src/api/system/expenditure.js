import request from '@/utils/request'

// 查询支出列表
export function listExpenditure(query) {
  return request({
    url: '/system/expenditure/list',
    method: 'get',
    params: query
  })
}

// 查询支出详细
export function getExpenditure(expId) {
  return request({
    url: '/system/expenditure/' + expId,
    method: 'get'
  })
}

// 新增支出
export function addExpenditure(data) {
  return request({
    url: '/system/expenditure',
    method: 'post',
    data: data
  })
}

// 修改支出
export function updateExpenditure(data) {
  return request({
    url: '/system/expenditure',
    method: 'put',
    data: data
  })
}

// 删除支出
export function delExpenditure(expId) {
  return request({
    url: '/system/expenditure/' + expId,
    method: 'delete'
  })
}
