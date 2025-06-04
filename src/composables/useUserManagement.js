import { ref } from 'vue'
import { addUser, getUser } from "@/api/system/user"
import { listUserCouponWithValidTime } from '@/api/system/user_coupon'
import eventBus from "@/utils/eventBus"

/**
 * 用户管理相关逻辑
 */
export function useUserManagement() {
  const currentUser = ref({})
  const showCreateUser = ref(false)
  const userCouponList = ref([])
  const mergedCoupons = ref([])
  const couponTypeList = ref()
  const phoneRegex = /^1[3-9]\d{9}$/

  /**
   * 处理用户选择事件
   * @param {Object} val - 选中的用户
   * @param {Object} form - 订单表单
   */
  const selectUser = async (val, form) => {
    if (!val) {
      form.userInfo = null
      form.userId = null
      form.nickName = null
      currentUser.value = {}
      userCouponList.value = []
      showCreateUser.value = false
      return
    }

    // 确保val是对象且有userId属性
    if (typeof val === 'object' && val.userId) {
      // 设置引用并更新表单
      form.userInfo = val
      form.userId = val.userId
      form.nickName = val.nickName
      showCreateUser.value = false

      // 获取完整用户信息
      currentUser.value = await getUser(val.userId)

      // 获取用户卡券信息
      await getUserCoupons(val.userId)
      
      // 确保userId被设置，这是关键
      return val.userId
    }
    
    return null
  }

  /**
   * 获取用户卡券信息
   * @param {Number} userId - 用户ID
   */
  const getUserCoupons = async (userId) => {
    const response = await listUserCouponWithValidTime(userId)
    userCouponList.value = response
    
    // 合并相同类型的卡券
    mergedCoupons.value = response.reduce((acc, cur) => {
      const existing = acc.find(item => 
        item.coupon.couponId === cur.coupon.couponId && 
        item.coupon.couponType !== 'StoredValueCard'
      )
      if (existing) {
        existing.ucCount += cur.ucCount
      } else {
        acc.push(cur)
      }
      return acc
    }, [])
    
    // 设置次卡默认值
    userCouponList.value
      .filter(item => item.coupon.couponType === 'SessionCard')
      .forEach(item => {
        item.selected = false
        item.count = 1
      })
    
    // 获取卡券类型列表
    couponTypeList.value = new Set(userCouponList.value.map(coupon => 
      coupon.coupon.couponType
    ))
  }

  /**
   * 处理需要创建用户的情况
   * @param {String} phoneNumber - 手机号
   * @param {Object} form - 订单表单
   */
  const handleNeedCreateUser = (phoneNumber, form) => {
    showCreateUser.value = true
    form.nickName = null
    currentUser.value = {
      phonenumber: phoneNumber,
      status: "0",
    }

    // 设置用户ID为临时值，确保右侧添加衣物组件能够正确显示
    // 使用一个特殊的标记值，表示这是一个待创建的用户
    form.userId = -999 // 临时ID，表示待创建用户

    // 触发事件，通知其他组件刷新
    eventBus.emit('user-selected', { isNewUser: true, phonenumber: phoneNumber })
  }

  /**
   * 处理更新手机号事件，但保留姓名
   * @param {String} phoneNumber - 手机号
   */
  const handleUpdatePhone = (phoneNumber) => {
    // 只更新手机号，保留其他信息
    if (showCreateUser.value && currentUser.value) {
      currentUser.value.phonenumber = phoneNumber

      // 触发事件，通知其他组件更新手机号
      eventBus.emit('user-phone-updated', {
        isNewUser: true,
        phonenumber: phoneNumber,
        nickName: currentUser.value.nickName
      })
    }
  }

  /**
   * 创建新用户
   * @param {Object} form - 订单表单
   * @returns {Promise} - 创建结果
   */
  const createUser = async (form) => {
    if (!showCreateUser.value) return Promise.resolve()

    try {
      const res = await addUser({
        phonenumber: currentUser.value.phonenumber,
        nickName: form.nickName
      })

      form.userId = res.userId
      form.userInfo = res // 设置userInfo

      showCreateUser.value = false
      // 通知UserSelect组件刷新用户列表
      eventBus.emit('user-created', res)
      
      return res
    } catch (err) {
      return Promise.reject(err)
    }
  }

  /**
   * 处理卡券购买成功事件
   * @param {Object} data - 购买数据
   * @param {Object} form - 订单表单
   */
  const handleCouponPurchase = async (data, form) => {
    // 检查是否是当前用户购买的卡券
    if (data.userId && data.userId == form.userId) {
      // 重新获取用户卡券列表
      await getUserCoupons(form.userId)

      // 更新用户信息（余额、积分等可能变化）
      const res = await getUser(form.userId)
      currentUser.value = res
    }
  }

  return {
    currentUser,
    showCreateUser,
    userCouponList,
    mergedCoupons,
    couponTypeList,
    phoneRegex,
    selectUser,
    handleNeedCreateUser,
    handleUpdatePhone,
    createUser,
    handleCouponPurchase,
    getUserCoupons
  }
} 