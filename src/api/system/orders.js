import request from '@/utils/request'
import invoke from '@/utils/invoke'

// 查询洗护服务订单列表
export function listOrders(query) {
  let pageParams = {
    pageSize: query.pageSize,
    page: query.pageNum,
  };
  if (query.startTime && query.endTime) {
    pageParams.params = {
      startTime: query.startTime,
      endTime: query.endTime
    }
  }
  const order = {
    orderNumber: query.orderNumber,
    phonenumber: query.phonenumber,
    paymentStatus: query.paymentStatus,
    costTimeAlarm: query.costTimeAlarm,
    pickupCode: query.pickupCode,
    status: query.status
  };
  return invoke('get_orders_pagination', { pageParams: pageParams, order: order })
}

// 查询洗护服务订单列表
export function selectListExceptCompleted(query) {
  return invoke('get_orders4home', { order: query })
}

export function selectListHistory(param) {
  const pageParams = { pageSize: param.pageSize, page: param.pageNum, params: param.params };
  const query = {
    clothName: param.title,
    startTime: param.startTime,
    endTime: param.endTime,
    userId: param.userId
  };
  return invoke('get_orders4history', { query, pageParams })
}

// 查询洗护服务订单详细
export function getOrders(orderId) {
  return invoke('get_order_by_id', { id: orderId })
}

export function getCountByUserId(userId) {
  return invoke('get_count_by_user_id', { userId })
}

// 查询退单所需的信息，用户手机号、订单实际支付金额
export function getRefundInfo(orderId, userId) {
  return invoke('get_refund_info', { orderId, userId })
}

// 新增洗护服务订单
export function addOrders(data) {
  return invoke('create_order', { order: data })
}

// 新增洗护服务订单
export function addServerOrders(data) {
  return invoke('create_server_order', { data })
}

// 新增洗护服务订单
export function addRewashOrder(data) {
  return request({
    url: '/system/orders/rewash',
    method: 'post',
    data: data
  })
}

// 修改洗护服务订单
export function updateOrders(data) {
  return invoke('update_order', { order: data })
}

// 修改洗护服务订单
export function updateAdjust(data) {
  return invoke('update_adjust', { order: data })
}

// 退款
export function pay(req) {
  return invoke('pay_order', { req })
}

// 更新退款接口
export function refund(orderId, refundReason) {
  return invoke('refund_order', { orderId, refundReason });
}

// 删除洗护服务订单
export function delOrders(orderId) {
  return invoke('delete_orders', { ids: [].concat(orderId) })
}
