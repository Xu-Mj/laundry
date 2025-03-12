import request from '@/utils/request'

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

export function updatePaymentConfig(data) {
  return request({
    url: '/api/profile/payment',
    method: 'put',
    data
  })
}
