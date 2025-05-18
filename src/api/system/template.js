import request from '@/utils/request'
import invoke from '@/utils/invoke'

// 查询通知模板管理列表
export function listTemplate() {
  return invoke('sms_template_list')
}

// 查询通知模板管理详细
export function getTemplate(id) {
  return invoke('get_temp_by_id', { id })
}

// 新增通知模板管理
export function addTemplate(data) {
  return invoke('create_temp', { temp: data })
}

// 修改通知模板管理
export function updateTemplate(data) {
  return invoke('update_temp', { temp: data })
}

// 删除通知模板管理
export function delTemplate(tempId) {
  return invoke('delete_temp', { ids: [].concat(tempId) })
}

// 发送通知
export function sendNotice(data) {
  return request({
    url: '/system/template/send',
    method: 'post',
    data: data
  })
}

// 发送通知给所有会员
export function sendNotice2All(tempId) {
  return request({
    url: '/system/template/send-2all/' + tempId,
    method: 'post',
  })
}
