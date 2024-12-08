import invoke from '@/utils/invoke'

// 查询岗位列表
export function listPost(query) {
  const pageParams = { pageSize: query.pageSize, page: query.pageNum, params: query.params };
  const ml = {
    levelCode: query.levelCode,
    levelName: query.levelName,
    status: query.status,
  };
  return invoke('get_membership_level_pagination', { pageParams, ml })
}

// 查询岗位列表
export function listPostAll() {
  return invoke('get_membership_level_list', { ml: {} })
}

// 查询岗位详细
export function getPost(id) {
  return invoke('get_membership_level_by_id', { id })
}

// 新增岗位
export function addPost(ml) {
  return invoke('create_membership_level', { ml })
}

// 修改岗位
export function updatePost(ml) {
  return invoke('update_membership_level', { ml })
}

// 删除岗位
export function delPost(postId) {
  return invoke('delete_membership_level', { ids: [].concat(postId) })
}
