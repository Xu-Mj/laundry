import request from '@/utils/request';
import invoke from '@/utils/invoke'

// 获取路由
export const getRouters = () => {
  // return request({
  //   url: '/getRouters',
  //   method: 'get'
  // })
  return invoke('get_routers')
}