import { getProfile } from '@/api/system/profile'
import { useStorage } from '@vueuse/core'
import { getInfo } from '@/api/login'

// 试用期配置，可以根据需要修改
const TRIAL_DAYS = 7
const GUEST_TRIAL_DAYS = 2 // 游客模式试用期为2天

// 使用本地存储记录首次使用时间
const firstUseTime = useStorage('app_first_use_time', null)

const useSubscriptionStore = defineStore(
  'subscription',
  {
    state: () => ({
      // 订阅信息
      plan: null,
      expiryDate: null,
      status: 'inactive', // active, inactive, trial
      // 试用期信息
      trialDays: TRIAL_DAYS,
      firstUseTime: firstUseTime.value,
      trialRemaining: 0,
      isInTrial: true,
      // 游客模式
      isGuest: true,
      // 水印显示控制
      showWatermark: false
    }),
    getters: {
      // 获取试用期剩余天数
      trialRemainingDays: (state) => {
        if (!state.firstUseTime) return state.trialDays
        
        const now = new Date()
        const firstUse = new Date(state.firstUseTime)
        const diffTime = now - firstUse
        const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24))
        
        return Math.max(0, state.trialDays - diffDays)
      },
      // 是否应该显示水印
      shouldShowWatermark: (state) => {
        // 如果有活跃订阅，不显示水印
        if (state.status === 'active') return false
        // 如果在试用期内，显示水印
        return state.isInTrial
      }
    },
    actions: {
      // 获取订阅信息
      async getSubscriptionInfo() {
        try {
          // 使用invoke调用Tauri后端获取用户信息和订阅信息
          const userInfo = await getInfo();
          console.log('获取用户信息和订阅信息成功:', userInfo)

          // 需要检查是否已经登录
          // 检查是否是游客账号
          if (userInfo.user && userInfo.user.id === 0) {
            this.isGuest = true
            this.isInTrial = true
            // 游客模式使用特殊的试用期天数
            this.trialDays = GUEST_TRIAL_DAYS
            this.status = 'trial'
          } else {
            this.isGuest = false
            this.trialDays = TRIAL_DAYS
            
            // 从本地数据库获取订阅信息
            if (userInfo.subscription) {
              this.plan = userInfo.subscription.planType
              this.expiryDate = userInfo.subscription.expiryDate
              
              // 检查订阅是否有效
              const now = Date.now()
              const expiry = userInfo.subscription.expiryDate
              
              if (expiry > now) {
                this.status = 'active'
                this.isInTrial = false
              } else {
                this.isInTrial = true
                this.status = 'inactive'
              }
            } else {
              this.isInTrial = true
              this.status = 'inactive'
            }
          }
          
          // 检查试用期状态
          this.checkTrialStatus()
          
          // 更新水印显示状态
          this.updateWatermarkStatus()
          
          return this.status
        } catch (error) {
          console.error('获取订阅信息失败:', error)
          // 如果获取失败，检查试用期状态
          this.checkTrialStatus()
          this.updateWatermarkStatus()
          return this.status
        }
      },
      
      // 检查试用期状态
      checkTrialStatus() {
        // 如果订阅有效，不需要检查试用期
        if (this.status === 'active') {
          this.isInTrial = false
          return
        }
        
        // 如果是首次使用，记录时间
        if (!this.firstUseTime) {
          this.firstUseTime = new Date().toISOString()
          firstUseTime.value = this.firstUseTime
        }
        
        // 计算试用期剩余天数
        this.trialRemaining = this.trialRemainingDays
        
        // 如果试用期剩余天数大于0，则处于试用期
        this.isInTrial = this.trialRemaining > 0
      },
      
      // 更新水印显示状态
      updateWatermarkStatus() {
        this.showWatermark = this.shouldShowWatermark
      },
      
      // 设置试用期天数（用于开发测试）
      setTrialDays(days) {
        this.trialDays = days
        this.checkTrialStatus()
        this.updateWatermarkStatus()
      },
      
      // 重置试用期（用于开发测试）
      resetTrial() {
        this.firstUseTime = null
        firstUseTime.value = null
        this.checkTrialStatus()
        this.updateWatermarkStatus()
      }
    }
  })

export default useSubscriptionStore