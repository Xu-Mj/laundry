import { login, logout, getInfo } from '@/api/login'
import { getToken, setToken, removeToken } from '@/utils/auth'
import defAva from '@/assets/images/profile.jpg'
import { convertFileSrc } from '@tauri-apps/api/core'
import { updatePwd } from '@/api/system/user'

const useUserStore = defineStore(
  'user',
  {
    state: () => ({
      token: getToken(),
      id: '',
      name: '',
      account: '',
      avatar: '',
      isFirstLogin: false,
      roles: [],
      permissions: []
    }),
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
            this.isFirstLogin = res.isFirstLogin
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
            const user = res.user
            const avatar = (user.avatar == "" || user.avatar == null) ? defAva : convertFileSrc(user.avatar);

            if (res.roles && res.roles.length > 0) { // 验证返回的roles是否是一个非空数组
              this.roles = res.roles
              this.permissions = res.permissions
            } else {
              this.roles = ['ROLE_DEFAULT']
            }
            this.id = user.id
            this.name = user.account
            this.account = user.account
            this.isFirstLogin = user.isFirstLogin
            this.avatar = avatar
            resolve(res)
          }).catch(error => {
            reject(error)
          })
        })
      },// 修改密码
      updatePassword(params) {
        return new Promise((resolve, reject) => {
          updatePwd(params).then(() => { // 需要确保已引入updateUserPwd
            // 密码修改成功后处理逻辑
            this.isFirstLogin = false;  // 更新首次登录状态
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
      }
    }
  })

export default useUserStore
