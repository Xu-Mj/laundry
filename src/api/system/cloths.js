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
export function rewash(query) {
  return request({
    url: '/system/cloths/rewash/' + query,
    method: 'post',
  })
}

// 查询订单包含的衣物清单列表
export function listHistoryCloths(query) {
  return request({
    url: '/system/cloths/list/history',
    method: 'get',
    params: query
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
export function getClothByCode(code) {
  return request({
    url: '/system/cloths/code/' + code,
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

// 取走
export function delivery(data) {
  return request({
    url: '/system/cloths/delivery',
    method: 'post',
    data: data
  })
}

// 取走
export function pickUp(ids) {
  return request({
    url: '/system/cloths/pickup/' + ids,
    method: 'post',
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
export function delClothPicture(clothId, picId) {
  return request({
    url: `/system/cloths/picture/${clothId}/${picId}`,
    method: 'delete'
  })
}

// 删除订单包含的衣物清单
export function delCloths(orderClothId) {
  return request({
    url: '/system/cloths/' + orderClothId,
    method: 'delete'
  })
}
