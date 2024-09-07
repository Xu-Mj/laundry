import request from '@/utils/request'

// 查询价格管理列表
export function listPrice(query) {
  return request({
    url: '/system/price/list',
    method: 'get',
    params: query
  })
}

// 查询价格管理详细
export function getPrice(priceId) {
  return request({
    url: '/system/price/' + priceId,
    method: 'get'
  })
}

// 新增价格管理
export function addPrice(data) {
  return request({
    url: '/system/price',
    method: 'post',
    data: data
  })
}

// 修改价格管理
export function updatePrice(data) {
  return request({
    url: '/system/price',
    method: 'put',
    data: data
  })
}

// 删除价格管理
export function delPrice(priceId) {
  return request({
    url: '/system/price/' + priceId,
    method: 'delete'
  })
}
