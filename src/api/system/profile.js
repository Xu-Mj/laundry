import request from '@/utils/request'
import invoke from '@/utils/invoke'

export function getProfile() {
  return request({
    url: '/api/profile',
    method: 'get'
  })
}

export function updateProfile(data) {
  return request({
    url: '/api/profile',
    method: 'put',
    data
  })
}

export function updateAlipayConfig(config) {
  return invoke('save_alipay_config', { config })
}

export function getAlipayConfig(storeId) {
  return invoke('get_alipay_config', { store_id: storeId })
}

export function uploadFile(name, data) {
  return invoke('save_file', { name, data })
}

export function deleteFile(filePath) {
  return invoke('delete_file', { file_path: filePath })
}

export function updateWechatConfig(config) {
  return invoke('save_wechat_config', { config })
}

export function getWechatConfig(storeId) {
  return invoke('get_wechat_config', { store_id: storeId })
}
