import request from '@/utils/request'

// 查询订单包含的衣物清单列表
export function listCloths(query) {
  return request({
    url: '/system/cloths/list',
    method: 'get',
    params: query
  })
}
// 查询订单包含的衣物清单列表
export function listHistoryCloths(userId) {
  return request({
    url: '/system/cloths/list/' + userId,
    method: 'get',
  })
}

// 查询订单包含的衣物清单详细
export function getCloths(orderClothId) {
  return request({
    url: '/system/cloths/' + orderClothId,
    method: 'get'
  })
}

// 查询订单包含的衣物清单详细
export function getPic(picId) {
  return request({
    url: '/system/cloths/download/' + picId,
    method: 'get'
  })
}

// 新增订单包含的衣物清单
export function addCloths(data) {
  return request({
    url: '/system/cloths',
    method: 'post',
    data: data
  })
}

// 修改订单包含的衣物清单
export function updateCloths(data) {
  return request({
    url: '/system/cloths',
    method: 'put',
    data: data
  })
}

// 上挂
export function hangup(data) {
  return request({
    url: '/system/cloths/hang',
    method: 'put',
    data: data
  })
}

// 删除订单包含的衣物清单
export function delCloths(orderClothId) {
  return request({
    url: '/system/cloths/' + orderClothId,
    method: 'delete'
  })
}
