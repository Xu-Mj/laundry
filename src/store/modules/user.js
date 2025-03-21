import { login, logout, getInfo, guestLogin } from '@/api/login'
import { getToken, setToken, removeToken } from '@/utils/auth'
import defAva from '@/assets/images/avatar1.png'
import { convertFileSrc } from '@tauri-apps/api/core'
import { useStorage } from '@vueuse/core'
import { updatePwd } from '@/api/system/user'
// 试用期配置，可以根据需要修改
const TRIAL_DAYS = 7
const GUEST_TRIAL_DAYS = 2 // 游客模式试用期为2天

// 使用本地存储记录首次使用时间
const firstUseTime = useStorage('app_first_use_time', null)

const useUserStore = defineStore(
  'user',
  {
    state: () => ({
      token: getToken(),
      id: '',
      name: '',
      account: '',
      avatar: '',
      isGuest: true,
      roles: [],
      permissions: [],
      sub: {
        // 订阅信息
        plan: {},
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
        showWatermark: false,
        subscription: {},
      }
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
      // 登录
      login(userInfo) {
        const account = userInfo.account.trim()
        const password = userInfo.password
        const code = userInfo.code
        const uuid = userInfo.uuid
        return new Promise((resolve, reject) => {
          login(account, password, code, uuid).then(res => {
            setToken(res.token)
            this.token = res.token
            resolve(res)
          }).catch(error => {
            reject(error)
          })
        })
      },
      guestLogin() {
        return new Promise((resolve, reject) => {
          guestLogin().then(res => {
            setToken(res.token)
            this.token = res.token
            this.isGuest = true
            resolve(res)
          }).catch(error => {
            reject(error)
          })
        })
      },
      // 获取用户信息
      getInfo() {
        return new Promise((resolve, reject) => {
          getInfo().then(res => {
            const user = res.user;
            const avatar = (user.avatar == "" || user.avatar == null) ? defAva : convertFileSrc(user.avatar);

            // if (res.roles && res.roles.length > 0) {
            //   this.roles = res.roles
            //   this.permissions = res.permissions
            // } else {
            //   this.roles = ['ROLE_DEFAULT']
            // }
            this.id = user.id
            this.name = user.nickname
            this.account = user.ownerPhone
            this.isGuest = user.isGuest ? user.isGuest : false
            this.avatar = avatar
            res.user.avatar = avatar
            // 需要检查是否已经登录
            // 检查是否是游客账号
            if (res.user && res.user.id === 0) {
              this.isGuest = true
              this.sub.isInTrial = true
              // 游客模式使用特殊的试用期天数
              this.sub.trialDays = GUEST_TRIAL_DAYS
              this.sub.status = 'trial'
            } else {
              this.isGuest = false
              this.sub.trialDays = TRIAL_DAYS

              // 从本地数据库获取订阅信息
              if (res.subscription) {
                this.sub.subscription = res.subscription
                // this.plan = userInfo.subscription.planType
                this.sub.expiryDate = res.subscription.expiryDate

                // 检查订阅是否有效
                const now = Date.now()
                const expiry = res.subscription.expiryDate

                if (expiry > now) {
                  this.sub.status = 'active'
                  this.sub.isInTrial = false
                } else {
                  this.sub.isInTrial = true
                  this.sub.status = 'inactive'
                }
              } else {
                this.sub.isInTrial = true
                this.sub.status = 'inactive'
              }
            }

            // 检查试用期状态
            this.checkTrialStatus()

            // 更新水印显示状态
            this.updateWatermarkStatus()
            console.log('获取订阅信息成功:', this.sub)
            resolve(res)
          }).catch(error => {
            reject(error)
          })
        })
      },// 修改密码
      updatePassword(params) {
        return new Promise((resolve, reject) => {
          updatePwd(params).then(() => {
            // 密码修改成功后处理逻辑
            this.token = '';            // 清空token
            this.roles = [];            // 清空角色
            this.permissions = [];      // 清空权限
            removeToken();              // 移除本地token
            resolve();
          }).catch(error => {
            reject(error);
          });
        });
      },
      // 退出系统
      logOut() {
        return new Promise((resolve, reject) => {
          logout().then(() => {
            this.token = ''
            this.roles = []
            this.permissions = []
            removeToken()
            resolve()
          }).catch(error => {
            reject(error)
          })
        })
      },

      // 检查试用期状态
      checkTrialStatus() {
        // 如果订阅有效，不需要检查试用期
        if (this.status === 'active') {
          this.sub.isInTrial = false
          return
        }

        // 如果是首次使用，记录时间
        if (!this.sub.firstUseTime) {
          this.sub.firstUseTime = new Date().toISOString()
          firstUseTime.value = this.sub.firstUseTime
        }

        // 计算试用期剩余天数
        this.sub.trialRemaining = this.sub.trialRemainingDays

        // 如果试用期剩余天数大于0，则处于试用期
        this.sub.isInTrial = this.sub.trialRemaining > 0
      },

      // 更新水印显示状态
      updateWatermarkStatus() {
        this.sub.showWatermark = this.sub.shouldShowWatermark
      },

      // 设置试用期天数（用于开发测试）
      setTrialDays(days) {
        this.sub.trialDays = days
        this.checkTrialStatus()
        this.updateWatermarkStatus()
      },

      // 重置试用期（用于开发测试）
      resetTrial() {
        this.sub.firstUseTime = null
        firstUseTime.value = null
        this.checkTrialStatus()
        this.updateWatermarkStatus()
      }
    }
  })

export default useUserStore
