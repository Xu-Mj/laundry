import request from '@/utils/request'

// 查询推广模板列表
export function listTemplate(query) {
  return request({
    url: '/system/promote-template/list',
    method: 'get',
    params: query
  })
}

// 查询推广模板详细
export function getTemplate(id) {
  return request({
    url: '/system/promote-template/' + id,
    method: 'get'
  })
}

// 新增推广模板
export function addTemplate(data) {
  return request({
    url: '/system/promote-template',
    method: 'post',
    data: data
  })
}

export function promote(data) {
  return request({
    url: '/system/promote-template/promote',
    method: 'post',
    data: data
  })
}

// 修改推广模板
export function updateTemplate(data) {
  return request({
    url: '/system/promote-template',
    method: 'put',
    data: data
  })
}

// 删除推广模板
export function delTemplate(id) {
  return request({
    url: '/system/promote-template/' + id,
    method: 'delete'
  })
}
