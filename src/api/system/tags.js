import { invoke } from '@tauri-apps/api/core'

// 查询用于配置系统中用到的所有标准化的数据，包括衣物类型、衣物颜色、洗前瑕疵、以后预估等列表
export function listTags(query) {
  const page_params = { pageSize: query.pageSize, page: query.pageNum };
  const tag = {
    tagNumber: query.tagNumber,
    tagOrder: query.tagOrder,
    tagName: query.tagName,
    status: query.status,
  }
  return invoke('list_pagination', {
    pageParams: page_params, tag: tag
  })
}

// export function listTags(query) {
//   return request({
//     url: '/system/tags/list',
//     method: 'get',
//     params: query
//   })
// }

// 查询用于配置系统中用到的所有标准化的数据，包括衣物类型、衣物颜色、洗前瑕疵、以后预估等列表
export function listTagsNoLimit(query) {
  return invoke('list_all',{tag: query})
}
// export function listTagsNoLimit(query) {
//   return request({
//     url: '/system/tags/list-no-limit',
//     method: 'get',
//     params: query
//   })
// }

// 查询用于配置系统中用到的所有标准化的数据，包括衣物类型、衣物颜色、洗前瑕疵、以后预估等详细
export function getTags(tagId) {
  return invoke('get_tag_by_id',{id: tagId})
}
// export function getTags(tagId) {
//   return request({
//     url: '/system/tags/' + tagId,
//     method: 'get'
//   })
// }

// 新增用于配置系统中用到的所有标准化的数据，包括衣物类型、衣物颜色、洗前瑕疵、以后预估等
export function addTags(data) {
  return invoke('add_tag', { tag: data })
}

// export function addTags(data) {
//   return request({
//     url: '/system/tags',
//     method: 'post',
//     data: data
//   })
// }

// 修改用于配置系统中用到的所有标准化的数据，包括衣物类型、衣物颜色、洗前瑕疵、以后预估等
export function updateTags(data) {
  return invoke('update_tag', {tag: data})
}
// export function updateTags(data) {
//   return request({
//     url: '/system/tags',
//     method: 'put',
//     data: data
//   })
// }

// 修改用于配置系统中用到的所有标准化的数据，包括衣物类型、衣物颜色、洗前瑕疵、以后预估等
export function updateTagsRefNum(data) {
  return invoke('update_ref_num', data)
}

// export function updateTagsRefNum(data) {
//   return request({
//     url: '/system/tags/update-ref-num',
//     method: 'put',
//     data: data
//   })
// }

// 删除用于配置系统中用到的所有标准化的数据，包括衣物类型、衣物颜色、洗前瑕疵、以后预估等
export function delTags(tagId) {
  // 如果只有一条，那么需要将该id转为数组
  return invoke('delete_tags_batch', {ids: [].concat(tagId)})
}


// 标签状态修改
export function changeTagStatus(tagId, status) {
  const data = {
    tagId,
    status
  }
  return invoke('change_tag_status',{tagParam: data})
}