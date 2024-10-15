import request from '@/utils/request'

// 查询用于配置系统中用到的所有标准化的数据，包括衣物类型、衣物颜色、洗前瑕疵、以后预估等列表
export function listTags(query) {
  return request({
    url: '/system/tags/list',
    method: 'get',
    params: query
  })
}

// 查询用于配置系统中用到的所有标准化的数据，包括衣物类型、衣物颜色、洗前瑕疵、以后预估等列表
export function listTagsNoLimit(query) {
  return request({
    url: '/system/tags/list-no-limit',
    method: 'get',
    params: query
  })
}

// 查询用于配置系统中用到的所有标准化的数据，包括衣物类型、衣物颜色、洗前瑕疵、以后预估等详细
export function getTags(tagId) {
  return request({
    url: '/system/tags/' + tagId,
    method: 'get'
  })
}

// 新增用于配置系统中用到的所有标准化的数据，包括衣物类型、衣物颜色、洗前瑕疵、以后预估等
export function addTags(data) {
  return request({
    url: '/system/tags',
    method: 'post',
    data: data
  })
}

// 修改用于配置系统中用到的所有标准化的数据，包括衣物类型、衣物颜色、洗前瑕疵、以后预估等
export function updateTags(data) {
  return request({
    url: '/system/tags',
    method: 'put',
    data: data
  })
}

// 修改用于配置系统中用到的所有标准化的数据，包括衣物类型、衣物颜色、洗前瑕疵、以后预估等
export function updateTagsRefNum(data) {
  return request({
    url: '/system/tags/update-ref-num',
    method: 'put',
    data: data
  })
}

// 删除用于配置系统中用到的所有标准化的数据，包括衣物类型、衣物颜色、洗前瑕疵、以后预估等
export function delTags(tagId) {
  return request({
    url: '/system/tags/' + tagId,
    method: 'delete'
  })
}


// 标签状态修改
export function changeTagStatus(tagId, status) {
  const data = {
    tagId,
    status
  }
  return request({
    url: '/system/tags/changeStatus',
    method: 'put',
    data: data
  })
}