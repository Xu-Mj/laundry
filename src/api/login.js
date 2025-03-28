import request from '@/utils/request'
import invoke from '@/utils/invoke'

// 登录方法
export function login(account, pwd, code, uuid) {
  const password = btoa(pwd).replace(/=+$/, '');
  const data = {
    account,
    password,
    code,
    uuid
  }
  return invoke('login', { req: data })
}

// 注册方法
export function register(data) {
  return request({
    url: '/register/store',
    headers: {
      isToken: false
    },
    method: 'post',
    data: data
  })
}

// 注册方法
export function getMsgCode(phone) {
  return request({
    url: '/register/code',
    headers: {
      isToken: false
    },
    method: 'post',
    data: { phone }
  })
}

// 获取用户详细信息
export function getInfo() {
  return invoke('get_info')
}

export function guestLogin() {
  return invoke('guest_login')
}

// 退出方法
export function logout() {
  return invoke('logout')
}

// 获取验证码
export function getCodeImg() {
  return invoke('get_captcha')
}

export function validatePwd(account, pwd) {
  return invoke('validate_pwd', { account, pwd })
}