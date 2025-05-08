import request from '@/utils/request'
import invoke from '@/utils/invoke'

// 查询用户列表
export function listUser(query) {
  const pageParams = { pageSize: query.pageSize, page: query.pageNum, params: query.params };
  const user = { userName: query.userName, phonenumber: query.phonenumber, levelId: query.levelId };
  console.log(query)
  return invoke('get_users_pagination', { pageParams, user })
}

// 查询用户列表
export function listUserWithNoLimit() {
  return invoke('get_all_users', { user: {} })
}

// 查询用户详细
export function getUser(userId) {
  return invoke('get_user_by_id', { id: userId })
}

export function updatePwd(data) {
  return invoke('update_pwd', { req: data })
}

// 查询用户详细
export function getUserByClothCode(clothCode) {
  return invoke('get_user_by_cloth_code', { code: clothCode })

}

// 查询用户详细
export function getUserListByIds(userIds) {
  return invoke('get_user_by_ids', { ids: [].concat(userIds) })

}

// 新增用户
export function addUser(data) {
  return invoke('create_user', { user: data })
}

// 修改用户
export function updateUser(data) {
  return invoke('update_user', { user: data })
}

// 删除用户
export function delUser(userId) {
  return invoke('delete_users', { ids: [].concat(userId) })
}

// 用户密码重置
export function resetUserPwd(userId, password) {
  const data = {
    userId,
    password
  }
  return request({
    url: '/system/user/resetPwd',
    method: 'put',
    data: data
  })
}

// 用户状态修改
export function changeUserStatus(userId, status) {
  return invoke('change_user_status', { userId, status })
}

// 查询用户个人信息
export function getUserProfile() {
  return request({
    url: '/system/user/profile',
    method: 'get'
  })
}

// 修改用户个人信息
export function updateUserProfile(data) {
  return request({
    url: '/system/user/profile',
    method: 'put',
    data: data
  })
}

// 用户密码重置
export function updateUserPwd(oldPassword, newPassword) {
  const data = {
    oldPassword,
    newPassword
  }
  return request({
    url: '/system/user/profile/updatePwd',
    method: 'put',
    params: data
  })
}

// 用户头像上传
export function uploadAvatar(data) {
  return request({
    url: '/system/user/profile/avatar',
    method: 'post',
    headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
    data: data
  })
}

// 查询授权角色
export function getAuthRole(userId) {
  return request({
    url: '/system/user/authRole/' + userId,
    method: 'get'
  })
}

// 保存授权角色
export function updateAuthRole(data) {
  return request({
    url: '/system/user/authRole',
    method: 'put',
    params: data
  })
}

// 查询部门下拉树结构
export function deptTreeSelect() {
  return request({
    url: '/system/user/deptTree',
    method: 'get'
  })
}
