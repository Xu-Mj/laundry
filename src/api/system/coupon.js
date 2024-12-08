import request from '@/utils/request';
import invoke from '@/utils/invoke'

// 查询卡券列表
export function listCoupon(query) {
  const pageParams = { pageSize: query.pageSize, page: query.pageNum, params: query.params };
  const coupon = {
    couponTitle: query.couponTitle,
    couponType: query.couponType,
    status: query.status,
    delFlag: query.delFlag,
    couponNumber: query.couponNumber,
  };
  return invoke('get_coupon_list', { pageParams: pageParams, coupon: coupon })
}

// 查询卡券列表
export function listCoupon4sale() {
  return invoke('get_coupons4sale')
}

// 查询卡券详细
export function getCoupon(couponId) {
  return invoke('get_coupon_by_id', { id: couponId })

}

// 新增卡券
export function addCoupon(data) {
  return invoke('add_coupon', { coupon: data })

}

// 购买卡券
export function buyCoupon(data) {
  return invoke('buy_coupons', { couponBuyReq: data })
}

// 赠送卡券
export function gift(data) {
  return invoke('gift_coupons', { couponBuyReq: data })
}

// 修改卡券
export function updateCoupon(data) {
  return invoke('update_coupon', { coupon: data })
}

// 删除卡券
export function delCoupon(couponId) {
  return invoke('delete_coupons', { ids: [].concat(couponId) })
}
