import router from './router'
import { ElMessage } from 'element-plus'
import NProgress from 'nprogress'
import 'nprogress/nprogress.css'
import { getToken } from '@/utils/auth'
import { isRelogin } from '@/utils/request'
import useUserStore from '@/store/modules/user'
import { initTauriWebSocketConnection } from '@/utils/initTauriWebSocket'

NProgress.configure({ showSpinner: false });

const whiteList = ['/login', '/register'];
// 不需要验证订阅状态的页面
const subscriptionWhiteList = [...whiteList, '/profile'];

// 记录是否已经初始化过WebSocket
let isWebSocketInitialized = false;

router.beforeEach((to, from, next) => {
  NProgress.start()
  if (getToken()) {
    /* has token*/
    if (to.path === '/login') {
      next({ path: '/' })
      NProgress.done()
    } else if (whiteList.indexOf(to.path) !== -1) {
      next()
    } else {
      if (useUserStore().id === '') {
        isRelogin.show = true
        // 判断当前用户是否已拉取完user_info信息
        useUserStore().getInfo().then(() => {
          isRelogin.show = false
          next({ ...to, replace: true })
        }).catch(err => {
          useUserStore().logOut().then(() => {
            ElMessage.error(err)
            next({ path: '/' })
          })
        })
      } else {
        // 检查订阅状态
        const userStore = useUserStore();
        // 每次路由变化时重新加载试用状态
        userStore.loadUserTrial();
        
        // 如果不在订阅白名单中，且订阅已过期，则重定向到个人资料页面的订阅管理标签页
        if (subscriptionWhiteList.indexOf(to.path) === -1 && userStore.trial.isExpired) {
          ElMessage.warning('您的试用期已过期，请订阅以继续使用')
          next({ path: '/profile?tab=subscription' })
        } else {
          next()
          
          // 只在第一次进入非白名单页面时初始化WebSocket
          if (!isWebSocketInitialized) {
            initTauriWebSocketConnection().then(success => {
              if (success) {
                isWebSocketInitialized = true;
              }
            });
          }
        }
      }
    }
  } else {
    // 没有token
    if (whiteList.indexOf(to.path) !== -1) {
      // 在免登录白名单，直接进入
      next()
    } else {
      next(`/login?redirect=${to.fullPath}`) // 否则全部重定向到登录页
      NProgress.done()
    }
  }
})

router.afterEach(() => {
  NProgress.done()
})
