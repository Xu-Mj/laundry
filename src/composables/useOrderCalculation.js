import { ref, computed } from 'vue'

/**
 * 订单价格计算相关逻辑
 */
export function useOrderCalculation() {
  const totalPrice = ref(0)

  /**
   * 判断价格项是否为折扣类型
   * @param {Object} item - 价格项
   * @returns {Boolean} - 是否为折扣类型
   */
  const isPriceDiscount = (item) => {
    return item.priceDiscount !== null && item.priceDiscount !== undefined
  }

  /**
   * 获取价格项的提示文本
   * @param {Object} item - 价格项
   * @returns {String} - 提示文本
   */
  const getPriceTooltip = (item) => {
    if (isPriceDiscount(item)) {
      return `${item.priceName}（折扣：${item.priceDiscount}%）`
    } else {
      return `${item.priceName}（固定价格：${item.priceValue}元）`
    }
  }

  /**
   * 计算订单总价
   * @param {Object} form - 订单表单数据
   * @param {Array} priceList - 价格列表
   * @returns {Number} - 计算后的总价
   */
  const calculateTotalPrice = (form, priceList) => {
    // 强制转换调价字符串为数字
    const adjustValueAdd = form.adjust.adjustValueAdd ? 
      Number(form.adjust.adjustValueAdd) : 0
    
    const adjustValueSub = form.adjust.adjustValueSub ? 
      Number(form.adjust.adjustValueSub) : 0
    
    // 处理 adjustTotal
    if (form.adjust.adjustTotal) {
      const adjustTotal = Number(form.adjust.adjustTotal)
      // 截断为两位小数（不四舍五入）
      totalPrice.value = Math.floor(adjustTotal * 100) / 100
      return totalPrice.value
    }
    
    // 计算原始价格
    let originalPrice = 0

    // 如果选择了价格方案
    if (form.priceIds && form.priceIds.length > 0) {
      // 检查是否选择了折扣类型的价格方案
      const discountPriceItem = priceList.find(item =>
        form.priceIds.includes(item.priceId) &&
        isPriceDiscount(item)
      )

      if (discountPriceItem) {
        // 如果是折扣类型，先计算衣物的原始价格总和
        originalPrice = form.cloths.reduce((acc, cur) => {
          // 计算总价
          // 如果服务要求为加急
          let priceValue = cur.priceValue
          if (cur.serviceRequirement == 'Emergency') {
            priceValue *= 2
          } else if (cur.serviceRequirement == 'SingleWash') {
            priceValue *= 1.5
          }
          return acc + priceValue + cur.processMarkup
        }, 0)

        // 然后应用折扣
        const discountFactor = discountPriceItem.priceDiscount / 100 // 将百分比转换为小数
        originalPrice = originalPrice * discountFactor

      } else {
        // 如果是固定价格类型，使用所有选中价格方案的总和
        originalPrice = form.priceIds.reduce((acc, priceId) => {
          const item = priceList.find(item => item.priceId === priceId)
          return acc + (item && item.priceValue ? item.priceValue : 0)
        }, 0)
      }
    } else {
      // 如果没有选择价格方案，计算衣物的原始价格总和
      originalPrice = form.cloths.reduce((acc, cur) => {
        // 计算总价
        // 如果服务要求为加急
        let priceValue = cur.priceValue
        if (cur.serviceRequirement == 'Emergency') {
          priceValue *= 2
        } else if (cur.serviceRequirement == 'SingleWash') {
          priceValue *= 1.5
        }
        return acc + priceValue + cur.processMarkup
      }, 0)
    }

    // 截断为两位小数（不四舍五入）
    originalPrice = Math.floor(originalPrice * 100) / 100

    // 计算最终价格（包含调整）
    let price = originalPrice + adjustValueAdd - adjustValueSub

    // 截断为两位小数（不四舍五入）
    price = Math.floor(price * 100) / 100
    
    totalPrice.value = price > 0 ? price : 0
    return totalPrice.value
  }

  /**
   * 处理价格选择变更
   * @param {Boolean} event - 是否选中
   * @param {Number} priceId - 价格ID
   * @param {Object} form - 订单表单数据
   * @param {Array} priceList - 价格列表
   */
  const handlePriceChange = (event, priceId, form, priceList) => {
    // 获取当前选择的价格项
    const currentPriceItem = priceList.find(item => item.priceId === priceId)

    // 如果找不到价格项，直接返回
    if (!currentPriceItem) return

    // 判断当前价格项是固定价格还是折扣系数
    const isDiscount = isPriceDiscount(currentPriceItem)

    if (event) {
      // 如果选中
      // 检查当前已选择的价格项中是否有折扣类型
      const hasDiscountSelected = form.priceIds.some(id => {
        const item = priceList.find(p => p.priceId === id)
        return item && isPriceDiscount(item)
      })

      // 检查当前已选择的价格项中是否有固定价格类型
      const hasFixedPriceSelected = form.priceIds.some(id => {
        const item = priceList.find(p => p.priceId === id)
        return item && !isPriceDiscount(item)
      })

      // 如果当前选择的是折扣类型
      if (isDiscount) {
        // 如果已经选择了其他折扣，则先移除所有折扣
        if (hasDiscountSelected) {
          // 移除所有折扣类型的价格项
          form.priceIds = form.priceIds.filter(id => {
            const item = priceList.find(p => p.priceId === id)
            return !(item && isPriceDiscount(item))
          })
          form.isDiscount = true
        } else {
          form.isDiscount = false
        }

        // 如果已经选择了固定价格，则移除所有固定价格
        if (hasFixedPriceSelected) {
          // 移除所有固定价格类型的价格项
          form.priceIds = form.priceIds.filter(id => {
            const item = priceList.find(p => p.priceId === id)
            return !(item && !isPriceDiscount(item))
          })
        }
      } else {
        // 如果当前选择的是固定价格，但已经选择了折扣，则移除所有折扣
        if (hasDiscountSelected) {
          // 移除所有折扣类型的价格项
          form.priceIds = form.priceIds.filter(id => {
            const item = priceList.find(p => p.priceId === id)
            return !(item && isPriceDiscount(item))
          })
        }
      }

      // 添加到选中数组
      if (!form.priceIds.includes(priceId)) {
        form.priceIds.push(priceId)
      }
    } else {
      // 如果取消选中，从数组中移除
      const index = form.priceIds.indexOf(priceId)
      if (index > -1) {
        form.priceIds.splice(index, 1)
      }
    }

    // 清空调整金额
    form.adjust.adjustValueSub = null
    form.adjust.adjustValueAdd = null
    form.adjust.adjustTotal = null
  }

  return {
    totalPrice,
    isPriceDiscount,
    getPriceTooltip,
    calculateTotalPrice,
    handlePriceChange
  }
} 