import request from '@/utils/request'

// 查询通知模板管理列表
export function listTemplate(query) {
  return request({
    url: '/system/template/list',
    method: 'get',
    params: query
  })
}

// 查询通知模板管理详细
export function getTemplate(tempId) {
  return request({
    url: '/system/template/' + tempId,
    method: 'get'
  })
}

// 新增通知模板管理
export function addTemplate(data) {
  return request({
    url: '/system/template',
    method: 'post',
    data: data
  })
}

// 修改通知模板管理
export function updateTemplate(data) {
  return request({
    url: '/system/template',
    method: 'put',
    data: data
  })
}

// 删除通知模板管理
export function delTemplate(tempId) {
  return request({
    url: '/system/template/' + tempId,
    method: 'delete'
  })
}
