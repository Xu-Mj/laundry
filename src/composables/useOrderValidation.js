import { reactive } from 'vue'

/**
 * 订单表单验证逻辑
 * @param {Object} options - 配置选项
 * @param {Ref} options.userSelectRef - 用户选择组件引用
 * @param {Ref} options.showCreateUser - 是否显示创建用户
 * @param {Ref} options.currentUser - 当前用户
 * @returns {Object} - 验证规则和方法
 */
export function useOrderValidation({ userSelectRef, showCreateUser, currentUser }) {
  // 手机号正则表达式
  const phoneRegex = /^1[3-9]\d{9}$/

  /**
   * 订单表单验证规则
   */
  const rules = reactive({
    businessType: [
      { required: true, message: "业务类型不能为空", trigger: "change" }
    ],
    userId: [
      { required: true, message: "手机号不能为空", trigger: "blur" },
      {
        validator: (rule, value, callback) => {
          // 获取当前输入值
          const currentInput = userSelectRef.value?.getInputValue() || ''

          // 如果是需要创建用户的情况且手机号有效，不报错
          if (showCreateUser.value && 
              currentUser.value?.phonenumber && 
              phoneRegex.test(currentUser.value.phonenumber)) {
            callback()
          }
          // 如果有输入但不是有效手机号
          else if (currentInput && currentInput.length > 0) {
            // 如果输入的不是11位，或者不是有效手机号
            if (currentInput.length !== 11 || !phoneRegex.test(currentInput)) {
              callback(new Error("请输入有效的手机号"))
            } else {
              callback()
            }
          }
          // 如果没有输入任何内容且触发了表单提交
          else if (!value && !currentInput && rule.trigger === 'submit') {
            callback(new Error("会员手机号不能为空"))
          }
          // 其他情况通过验证
          else {
            callback()
          }
        },
        trigger: ['blur', 'submit']
      }
    ],
    nickName: [
      { required: true, message: "会员姓名不能为空", trigger: "blur" }
    ],
    source: [
      { required: true, message: "订单来源不能为空", trigger: "blur" }
    ],
    cloths: [
      { required: true, message: "衣物信息不能为空", trigger: "change" }
    ]
  })

  /**
   * 处理用户选择组件的验证结果
   * @param {Boolean} valid - 是否有效
   * @param {String} message - 错误消息
   * @param {Object} ordersRef - 表单引用
   */
  const handleUserValidate = (valid, message, ordersRef) => {
    if (!valid) {
      // 如果是需要创建用户的情况，不显示错误
      if (showCreateUser.value && 
          currentUser.value?.phonenumber && 
          phoneRegex.test(currentUser.value.phonenumber)) {
        return
      }

      // 触发表单验证
      if (ordersRef) {
        // 使用nextTick确保在DOM更新后再触发验证
        nextTick(() => {
          ordersRef.validateField('userId')
        })
      }
    }
  }

  /**
   * 处理失去焦点的情况
   * @param {Object} ordersRef - 表单引用
   */
  const handleBlur = (ordersRef) => {
    // 如果是需要创建用户的情况且手机号有效，不进行验证
    if (showCreateUser.value && 
        currentUser.value?.phonenumber && 
        phoneRegex.test(currentUser.value.phonenumber)) {
      return
    }
    
    // 验证userId字段
    if (ordersRef) {
      ordersRef.validateField('userId')
    }
  }

  /**
   * 清除用户选择验证
   * @param {Object} ordersRef - 表单引用
   */
  const clearUserValidation = (ordersRef) => {
    if (ordersRef) {
      ordersRef.clearValidate('userId')
    }
  }

  /**
   * 验证订单表单
   * @param {Object} form - 订单表单
   * @param {Object} ordersRef - 表单引用
   * @param {Function} proxy - 组件代理
   * @param {Object} options - 验证选项
   * @returns {Promise<Boolean>} - 验证结果
   */
  const validateOrderForm = (form, ordersRef, proxy, options = {}) => {
    return new Promise((resolve, reject) => {
      // 验证表单
      ordersRef.validate(valid => {
        if (valid) {
          // 验证衣物是否为空
          if (!form.cloths || form.cloths.length === 0) {
            proxy.notify.error("衣物信息不能为空")
            reject(false)
            return
          }
          
          // 根据来源验证价格标签
          if ((form.source === 'Douyin' || form.source === 'Meituan' || form.source === 'MiniProgram') && 
              (!form.priceIds || form.priceIds.length === 0)) {
            proxy.notify.error("请选择价格标签")
            reject(false)
            return
          }
          
          resolve(true)
        } else {
          reject(false)
        }
      }, options)
    })
  }

  return {
    rules,
    handleUserValidate,
    handleBlur,
    clearUserValidation,
    validateOrderForm
  }
} 