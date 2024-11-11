import request from '@/utils/request'
import { invoke } from '@tauri-apps/api/core';

// 查询衣物管理列表
export function listClothing(query) {
  const page_params = { pageSize: query.pageSize, page: query.pageNum };
  const clothing = {
    clothingCategory: query.clothingCategory,
    clothingNumber: query.clothingNumber,
    clothingName: query.clothingName,
  };
  return invoke('list_clothing_pagination', {
    pageParams: page_params, clothing: clothing
  })
}
// export function listClothing(query) {
//   return request({
//     url: '/system/clothing/list',
//     method: 'get',
//     params: query
//   })
// }

// 查询衣物管理列表
export function listClothingWithNoLimit() {
  return invoke('list_clothing_all', { clothing: {} })
}
// export function listClothingWithNoLimit() {
//   return request({
//     url: '/system/clothing/list-no-limit',
//     method: 'get',
//   })
// }

// 查询衣物管理详细
export function getClothing(clothingId) {
  return invoke('get_clothing_by_id', { id: clothingId })
}

// 新增衣物管理
export function addClothing(data) {
  return invoke('add_clothing', { clothing: data });
}
// export function addClothing(data) {
//   return request({
//     url: '/system/clothing',
//     method: 'post',
//     data: data
//   })
// }

// 修改衣物管理
export function updateClothing(data) {
  console.log(data)
  return invoke('update_clothing', { clothing: data });
}

export function updateClothingRefNum(data) {
  return invoke('update_clothing_ref_num', data)
}

// 删除衣物管理
export function delClothing(clothingId) {
  return invoke('delete_clothing_batch', {ids: [].concat(clothingId)})
}
