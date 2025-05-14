// src/stores/user.js
import { defineStore } from 'pinia'
import { login, logout, getInfo, guestLogin } from '@/api/login'
import { getToken, setToken, removeToken } from '@/utils/auth'
import defAva from '@/assets/images/avatar1.png'
import { convertFileSrc } from '@tauri-apps/api/core'
import { useStorage } from '@vueuse/core'
import { updatePwd } from '@/api/system/user'
import { closeTauriWebSocketConnection } from '@/utils/initTauriWebSocket'

// 试用配置
const TRIAL_DAYS = 7
const GUEST_TRIAL_DAYS = 2

const useUserStore = defineStore('user', {
  state: () => {

    return {
      token: getToken(),
      id: '',          // 用户唯一标识
      name: '',
      account: '',
      avatar: '',
      user: null,

      // 订阅系统状态 - 累积模式
      subscription: {
        isActive: false,       // 是否有有效订阅
        latestExpiryDate: null, // 最新的过期日期
        planType: 'free'       // 当前计划类型
      },

      // 试用系统状态（运行时状态）
      trial: {
        isInTrial: false,
        days: TRIAL_DAYS,
        startTime: null,     // 动态加载
        remainingDays: 0,
        isGuest: false,
        isExpired: false,
      },

      // 其他业务状态
      showWatermark: false,
      allSubscriptions: [],  // 所有订阅

      // 短信订阅系统状态 - 累积模式
      smsSub: {
        isActive: false,
        latestExpiryDate: null,
        planType: 'free',
        totalSmsCount: 0,
        usedSmsCount: 0,
        remainingSmsCount: 0
      },
      allSmsSubscriptions: [] // 所有短信订阅
    }
  },

  getters: {
    // 计算剩余试用天数
    trialRemainingDays: (state) => {
      if (!state.trial.startTime) return state.trial.days
      const start = new Date(state.trial.startTime)
      const now = new Date()
      const diffDays = Math.floor((now - start) / (1000 * 60 * 60 * 24))
      return Math.max(0, state.trial.days - diffDays)
    },

    // 是否显示水印
    shouldShowWatermark: (state) => {
      return !state.subscription.isActive && state.trial.isInTrial
    },

    // 获取用户状态标签
    userStatus: (state) => {
      if (state.subscription.isActive) return 'active'
      if (state.trial.isInTrial) return 'trial'
      return 'inactive'
    }
  },

  actions: {
    // 核心方法：加载用户试用数据
    async loadUserTrial() {
      // 游客试用处理
      if (this.trial.isGuest) {
        const guestKey = 'guest_trial_start'
        const storedTime = useStorage(guestKey, null).value

        if (!storedTime) {
          const newTime = new Date().toISOString()
          useStorage(guestKey, newTime).value = newTime
          this.trial.startTime = newTime
        } else {
          this.trial.startTime = storedTime
        }
        this.trial.days = GUEST_TRIAL_DAYS
      }
      // 正式用户试用处理
      else if (this.id) {
        const userKey = `user_trial_${this.id}`
        const storedTime = useStorage(userKey, null).value

        if (!storedTime) {
          const newTime = new Date().toISOString()
          useStorage(userKey, newTime).value = newTime
          this.trial.startTime = newTime
        } else {
          this.trial.startTime = storedTime
        }
        this.trial.days = TRIAL_DAYS
      }

      // 更新计算状态
      this.trial.remainingDays = this.trialRemainingDays
      this.trial.isInTrial = this.trial.remainingDays > 0

      // 订阅状态优先
      if (this.subscription.isActive) {
        this.trial.isInTrial = false
      }

      this.showWatermark = this.shouldShowWatermark
      // 更新到期状态
      this.trial.isExpired =
        this.trial.remainingDays <= 0 &&
        !this.subscription.isActive
    },

    // 用户登录
    async login(userInfo) {
      try {
        const res = await login(
          userInfo.account.trim(),
          userInfo.password,
          userInfo.code,
          userInfo.uuid
        )
        setToken(res.token)
        this.$patch({ token: res.token })
        await this.getInfo() // 触发用户数据加载
        return res
      } catch (error) {
        throw error
      }
    },

    // 获取用户信息（核心入口）
    async getInfo() {
      try {
        const res = await getInfo()
        const user = res.user

        if (user) {
          user.avatar = user.avatar ? convertFileSrc(user.avatar) : defAva
          // 更新基础信息
          this.$patch({
            id: user.id,
            name: user.nickname,
            account: user.ownerPhone,
            avatar: user.avatar,
            trial: {
              ...this.trial,
              isGuest: !!user.isGuest
            },
            user: user
          })

          // 加载试用数据
          await this.loadUserTrial()

          // 处理订阅信息 - 累积模式
          if (res.subscriptions && res.subscriptions.length > 0) {
            this.updateSubscriptions(res.subscriptions)
          }

          // 处理短信订阅信息 - 累积模式
          if (res.sms_subscriptions && res.sms_subscriptions.length > 0) {
            this.updateSmsSubscriptions(res.sms_subscriptions)
          } else {
            // Initialize with default values instead of null
            this.$patch({
              smsSub: {
                isActive: false,
                latestExpiryDate: null,
                planType: 'free',
                totalSmsCount: 0,
                usedSmsCount: 0,
                remainingSmsCount: 0
              },
              allSmsSubscriptions: []
            })
          }
        }

        return res
      } catch (error) {
        console.error('Error in getInfo:', error) // Debug log
        throw error
      }
    },

    // 游客登录
    async guestLogin() {
      try {
        const res = await guestLogin()
        setToken(res.token)
        this.$patch({
          token: res.token,
          trial: {
            ...this.trial,
            isGuest: true
          }
        })
        await this.loadUserTrial()
        return res
      } catch (error) {
        throw error
      }
    },

    // 更新订阅信息 - 累积模式
    updateSubscriptions(subscriptions) {
      // 存储所有订阅
      this.allSubscriptions = subscriptions

      // 找出最晚的过期日期
      let latestExpiryDate = null
      let isActive = false
      let highestPlanType = 'free'

      // 检查是否有任何有效订阅
      const now = Date.now()
      subscriptions.forEach(sub => {
        // 如果订阅有效
        if (sub.expiryDate > now) {
          isActive = true

          // 更新最晚过期日期
          if (!latestExpiryDate || sub.expiryDate > latestExpiryDate) {
            latestExpiryDate = sub.expiryDate
          }

          // 更新最高级别计划类型
          if (sub.planType && (sub.planType === 'premium' || (sub.planType === 'standard' && highestPlanType === 'free'))) {
            highestPlanType = sub.planType
          }
        }
      })

      // 更新订阅状态
      this.$patch({
        subscription: {
          isActive,
          latestExpiryDate,
          planType: isActive ? highestPlanType : 'free'
        }
      })

      // 订阅激活时强制结束试用
      if (isActive) {
        this.$patch({
          trial: {
            ...this.trial,
            isInTrial: false
          }
        })
      }

      this.showWatermark = this.shouldShowWatermark
    },

    // 更新短信订阅信息 - 累积模式
    updateSmsSubscriptions(smsSubscriptions) {

      // 存储所有短信订阅
      this.allSmsSubscriptions = smsSubscriptions

      // 找出最晚的过期日期和计算累计短信数
      let latestExpiryDate = null
      let isActive = false
      let highestPlanType = 'free'
      let totalSmsCount = 0
      let usedSmsCount = 0
      let remainingSmsCount = 0

      // 检查是否有任何有效订阅
      const now = Date.now()
      smsSubscriptions.forEach(sub => {
        // 如果订阅有效
        if (sub.expiryDate > now) {
          isActive = true

          // 更新最晚过期日期
          if (!latestExpiryDate || sub.expiryDate > latestExpiryDate) {
            latestExpiryDate = sub.expiryDate
          }

          // 更新最高级别计划类型
          if (sub.plan && sub.plan.planType &&
            (sub.plan.planType === 'Premium' ||
              (sub.plan.planType === 'Standard' && highestPlanType === 'free'))) {
            highestPlanType = sub.plan.planType
          }

          // 累加短信数量
          totalSmsCount += sub.totalSmsCount || 0
          usedSmsCount += sub.usedSmsCount || 0
          remainingSmsCount += sub.remainingSmsCount || 0
        }
      })

      // 更新短信订阅状态
      const smsSubUpdate = {
        isActive,
        latestExpiryDate,
        planType: isActive ? highestPlanType : 'free',
        totalSmsCount,
        usedSmsCount,
        remainingSmsCount
      }

      this.$patch({
        smsSub: smsSubUpdate
      })
    },

    // 修改密码
    async updatePassword(params) {
      try {
        await updatePwd(params)
        await this.logOut()
      } catch (error) {
        throw error
      }
    },

    // 退出登录（关键清理）
    async logOut() {
      try {
        await logout()

        // 关闭WebSocket连接
        await closeTauriWebSocketConnection()

        // 清除运行时状态
        removeToken()

        // 保存当前状态，以便检查什么被重置了

        this.$reset()

        // 保留持久化存储数据
        this.$patch({
          trial: {
            startTime: null,
            isInTrial: false,
            remainingDays: 0
          },

          // 确保smsSub结构保持一致
          smsSub: {
            isActive: false,
            latestExpiryDate: null,
            planType: 'free',
            totalSmsCount: 0,
            usedSmsCount: 0,
            remainingSmsCount: 0
          }
        })

      } catch (error) {
        throw error
      }
    },

    // 重置试用（开发工具）
    resetTrial() {
      if (this.trial.isGuest) {
        useStorage('guest_trial_start', null).value = null
      } else if (this.id) {
        const userKey = `user_trial_${this.id}`
        useStorage(userKey, null).value = null
      }

      this.$patch({
        trial: {
          ...this.trial,
          startTime: null,
          isInTrial: false,
          remainingDays: 0
        }
      })
    }
  },

  // 持久化配置（仅游客标记）
  persist: {
    paths: ['trial.isGuest']
  }
})

export default useUserStore