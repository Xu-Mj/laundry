import invoke from '@/utils/invoke'

// 获取设备信息
export function getDeviceInfo() {
  return invoke('get_device_info')
}

// 验证设备是否允许登录
export function validateLoginDevice(storeId, deviceId) {
  return invoke('validate_login_device', { storeId, deviceId })
}

// 登录方法
export function login(account, pwd, code, uuid) {
  const password = btoa(pwd).replace(/=+$/, '');

  // 获取设备信息
  return getDeviceInfo().then(deviceInfo => {
    const data = {
      account,
      password,
      code,
      uuid,
      deviceId: deviceInfo.deviceId // 添加设备ID
    }
    return invoke('login', { req: data })
  })
}

// 注册方法
export function register(data) {
  return invoke('register', { req: data })
}

// 注册方法
export function getMsgCode(phone) {
  return invoke('get_sms_verification_code', { req: { phone } })
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