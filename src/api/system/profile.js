import invoke from '@/utils/invoke'

export function updateProfile(user) {
  return invoke('update_local_user', { user })
}

export function updateAlipayConfig(config) {
  return invoke('save_alipay_config', { config })
}

export function getAlipayConfig(storeId) {
  return invoke('get_alipay_config', { storeId })
}

export function uploadFile(name, data) {
  return invoke('save_file', { name, data })
}

export function deleteFile(filePath) {
  return invoke('delete_file', { filePath })
}

export function updateWechatConfig(config) {
  return invoke('save_wechat_config', { config })
}

export function getWechatConfig(storeId) {
  return invoke('get_wechat_config', { storeId })
}
