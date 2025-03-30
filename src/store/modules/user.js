// src/stores/user.js
import { defineStore } from 'pinia'
import { login, logout, getInfo, guestLogin } from '@/api/login'
import { getToken, setToken, removeToken } from '@/utils/auth'
import defAva from '@/assets/images/avatar1.png'
import { convertFileSrc } from '@tauri-apps/api/core'
import { useStorage } from '@vueuse/core'
import { updatePwd } from '@/api/system/user'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'

// 试用配置
const TRIAL_DAYS = 7
const GUEST_TRIAL_DAYS = 2

const useUserStore = defineStore({
  id: 'user',

  state: () => ({
    token: getToken(),
    id: '',          // 用户唯一标识
    name: '',
    account: '',
    avatar: '',
    user: null,

    // 订阅系统状态
    subscription: {
      status: 'inactive', // active | inactive
      expiryDate: null,
      planType: 'free'
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
    allSubscriptions: [],
    smsSub: null
  }),

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
      return state.subscription.status !== 'active' && state.trial.isInTrial
    },

    // 获取用户状态标签
    userStatus: (state) => {
      if (state.subscription.status === 'active') return 'active'
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
      if (this.subscription.status === 'active') {
        this.trial.isInTrial = false
      }

      this.showWatermark = this.shouldShowWatermark
      // 更新到期状态
      this.trial.isExpired =
        this.trial.remainingDays <= 0 &&
        this.subscription.status !== 'active'
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

          // 处理订阅信息
          if (res.subscription) {
            this.updateSubscription(res.subscription)
          }
        }

        return res
      } catch (error) {
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

    // 更新订阅信息
    updateSubscription(newSub) {
      const isActive = newSub.expiryDate > Date.now()

      this.$patch({
        subscription: {
          status: isActive ? 'active' : 'inactive',
          expiryDate: newSub.expiryDate,
          planType: newSub.planType || 'free'
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

        // 清除运行时状态
        removeToken()
        this.$reset()

        // 保留持久化存储数据
        this.$patch({
          trial: {
            startTime: null,
            isInTrial: false,
            remainingDays: 0
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

// 安装持久化插件
const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)

export default useUserStore