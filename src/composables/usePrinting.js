import { print, printReceipt2 } from "@/api/system/printer"

/**
 * 打印相关功能
 */
export function usePrinting() {
  /**
   * 打印衣物标签
   * @param {Array} cloths - 衣物列表
   * @param {Object} userData - 用户数据
   * @param {Object} form - 订单表单
   * @param {Object} proxy - 组件代理
   * @returns {Promise}
   */
  const printCloth = async (cloths, userData, form, proxy) => {
    // 如果没有衣物，直接返回
    if (!cloths || cloths.length === 0) return Promise.resolve()
    
    const length = cloths.length

    // 处理用户数据
    let userInfo = userData
    
    // Handle the case when a new user is being created (temporary ID)
    if (form.userId === -999) {
      userInfo = {
        nickName: form.nickName,
        phonenumber: userData.phonenumber
      }
    } else if (!userInfo || Object.keys(userInfo).length === 0) {
      // 如果用户不存在但有足够信息，创建临时用户对象
      if (form.nickName) {
        userInfo = {
          nickName: form.nickName,
          phonenumber: userData?.phonenumber || ""
        }
      } else {
        // 确保至少有一个名字显示
        userInfo = {
          nickName: form.nickName || "顾客",
          phonenumber: ""
        }
      }
    }

    // 准备打印数据
    const printData = cloths.map((item, index) => ({
      cloth_name: item.clothInfo.title,
      cloth_color: item.clothingColor ? item.clothingColor : 0,
      cloth_flaw: item.clothingFlawArr,
      sum: length,
      num: index + 1,
      code: item.hangClothCode,
      time: item.createTime,
      client: {
        name: userInfo.nickName,
        phone: userInfo.phonenumber,
      },
      shelf: {
        name: String(item.hangLocationCode),
        position: item.hangerNumber,
      }
    }))

    try {
      if (proxy) proxy.$modal.loading('正在打印衣物信息...')
      await print(printData)
      return Promise.resolve()
    } catch (error) {
      console.error("打印失败:", error)
      return Promise.reject(error)
    } finally {
      if (proxy) proxy.$modal.closeLoading()
    }
  }

  /**
   * 打印小票
   * @param {Object} orderData - 订单数据
   * @param {String} paymentMethod - 支付方式
   * @param {Number} amount - 支付金额
   * @param {Object} proxy - 组件代理
   * @returns {Promise}
   */
  const printReceipt = async (orderData, paymentMethod = '未付款', amount, proxy) => {
    try {
      await printReceipt2({
        ...orderData,
        paymentMethod,
        mount: amount || orderData.totalPrice
      })
      return Promise.resolve()
    } catch (error) {
      console.error("小票打印失败:", error)
      if (proxy) proxy.notify.error("小票打印失败")
      return Promise.reject(error)
    }
  }

  return {
    printCloth,
    printReceipt
  }
} 