import request from '@/utils/request'
import invoke from '@/utils/invoke'

// 登录方法
export function login(username, pwd, code, uuid) {
  const password = btoa(pwd).replace(/=+$/, '');
  const data = {
    username,
    password,
    code,
    uuid
  }
  // return request({
  //   url: '/login',
  //   headers: {
  //     isToken: false,
  //     repeatSubmit: false
  //   },
  //   method: 'post',
  //   data: data
  // })
  return invoke('login', {req: data})
}

// 注册方法
export function register(data) {
  return request({
    url: '/register',
    headers: {
      isToken: false
    },
    method: 'post',
    data: data
  })
}

// 获取用户详细信息
export function getInfo() {
return invoke('get_info')
}

// 退出方法
export function logout() {
  return invoke('logout')
}

// 获取验证码
export function getCodeImg() {
return invoke('get_captcha')
}