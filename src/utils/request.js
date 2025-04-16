import axios from 'axios'
import { ElNotification , ElMessageBox, ElMessage, ElLoading } from 'element-plus'
import { getToken } from '@/utils/auth'
import errorCode from '@/utils/errorCode'
import { tansParams, blobValidate } from '@/utils/ruoyi'
import cache from '@/plugins/cache'
import { saveAs } from 'file-saver'
import useUserStore from '@/store/modules/user'

let downloadLoadingInstance;
// 是否显示重新登录
export let isRelogin = { show: false };

axios.defaults.headers['Content-Type'] = 'application/json;charset=utf-8'
// 创建axios实例
const service = axios.create({
  // axios中请求配置有baseURL选项，表示请求URL公共部分
  baseURL: import.meta.env.VITE_APP_BASE_API,
  // 超时
  timeout: 10000
})

// request拦截器
service.interceptors.request.use(config => {
  // 是否需要设置 token
  const isToken = (config.headers || {}).isToken === false
  // 是否需要防止数据重复提交
  const isRepeatSubmit = (config.headers || {}).repeatSubmit === false
  if (getToken() && !isToken) {
    config.headers['Authorization'] = 'Bearer ' + getToken() // 让每个请求携带自定义token 请根据实际情况自行修改
  }
  // get请求映射params参数
  if (config.method === 'get' && config.params) {
    let url = config.url + '?' + tansParams(config.params);
    url = url.slice(0, -1);
    config.params = {};
    config.url = url;
  }
  if (!isRepeatSubmit && (config.method === 'post' || config.method === 'put')) {
    const requestObj = {
      url: config.url,
      data: typeof config.data === 'object' ? JSON.stringify(config.data) : config.data,
      time: new Date().getTime()
    }
    const requestSize = Object.keys(JSON.stringify(requestObj)).length; // 请求数据大小
    const limitSize = 5 * 1024 * 1024; // 限制存放数据5M
    if (requestSize >= limitSize) {
      console.warn(`[${config.url}]: ` + '请求数据大小超出允许的5M限制，无法进行防重复提交验证。')
      return config;
    }
    const sessionObj = cache.session.getJSON('sessionObj')
    if (sessionObj === undefined || sessionObj === null || sessionObj === '') {
      cache.session.setJSON('sessionObj', requestObj)
    } else {
      const s_url = sessionObj.url;                // 请求地址
      const s_data = sessionObj.data;              // 请求数据
      const s_time = sessionObj.time;              // 请求时间
      const interval = 1000;                       // 间隔时间(ms)，小于此时间视为重复提交
      if (s_data === requestObj.data && requestObj.time - s_time < interval && s_url === requestObj.url) {
        const message = '数据正在处理，请勿重复提交';
        console.warn(`[${s_url}]: ` + message)
        return Promise.reject(new Error(message))
      } else {
        cache.session.setJSON('sessionObj', requestObj)
      }
    }
  }
  return config
}, error => {
    console.log(error)
    Promise.reject(error)
})

// 响应拦截器
service.interceptors.response.use(res => {
  // 二进制数据则直接返回
  if (res.request.responseType === 'blob' || res.request.responseType === 'arraybuffer') {
    return res.data;
  }

  // 获取状态码
  const code = res.status;
  // 获取错误信息
  const msg = errorCode[code] || res.statusText || errorCode['default'];

  if (code === 401) {
    if (res.data && res.data.kind === 'AccountNotRegister') {
      ElMessage.error(`账户未注册: ${res.data.details || ''}`);
    } else if (response.data && response.data.kind === 'AccountOrPassword') {
      ElMessage.error(`账户或密码错误: ${response.data.details || ''}`);
    } else if (!isRelogin.show) {
      isRelogin.show = true;
      ElMessageBox.confirm('登录状态已过期，您可以继续留在该页面，或者重新登录', '系统提示', {
        confirmButtonText: '重新登录',
        cancelButtonText: '取消',
        type: 'warning'
      }).then(() => {
        isRelogin.show = false;
        useUserStore().logOut().then(() => {
          location.href = '/index';
        });
      }).catch(() => {
        isRelogin.show = false;
      });
    }
    return Promise.reject('无效的会话，或者会话已过期，请重新登录。');
  } else if (code !== 200) {
    // 检查响应数据是否包含错误信息
    if (res.data && res.data.kind) {
      handleError(res.data);
    } else {
      ElNotification.error({ title: msg });
    }
    return Promise.reject('error');
  } else {
    return Promise.resolve(res.data);
  }
},
  error => {
    console.error('err' + error);
    let message = error.message;

    if (error.response) {
      // The request was made and the server responded with a status code
      // that falls out of the range of 2xx
      const response = error.response;
      const code = response.status;
      const msg = errorCode[code] || response.statusText || errorCode['default'];

      if (code === 401) {
        if (response.data && response.data.kind === 'AccountNotRegister') {
          ElMessage.error(`账户未注册: ${response.data.details || ''}`);
        } else if (response.data && response.data.kind === 'AccountOrPassword') {
          ElMessage.error(`账户或密码错误: ${response.data.details || ''}`);
        } else if (!isRelogin.show) {
          isRelogin.show = true;
          ElMessageBox.confirm('登录状态已过期，您可以继续留在该页面，或者重新登录', '系统提示', {
            confirmButtonText: '重新登录',
            cancelButtonText: '取消',
            type: 'warning'
          }).then(() => {
            isRelogin.show = false;
            useUserStore().logOut().then(() => {
              location.href = '/index';
            });
          }).catch(() => {
            isRelogin.show = false;
          });
        }
        return Promise.reject('无效的会话，或者会话已过期，请重新登录。');
      } else {
        // 检查响应数据是否包含错误信息
        if (response.data && response.data.kind) {
          handleError(response.data);
        } else {
          ElNotification.error({ title: msg });
        }
        return Promise.reject('error');
      }
    } else if (message === "Network Error") {
      message = "后端接口连接异常";
    } else if (message.includes("timeout")) {
      message = "系统接口请求超时";
    } else if (message.includes("Request failed with status code")) {
      message = "系统接口" + message.substr(message.length - 3) + "异常";
    }

    ElMessage({ message: message, type: 'error', duration: 5 * 1000 });
    return Promise.reject(error);
  });
// 通用下载方法
export function download(url, params, filename, config) {
  downloadLoadingInstance = ElLoading.service({ text: "正在下载数据，请稍候", background: "rgba(0, 0, 0, 0.7)", });
  return service.post(url, params, {
    transformRequest: [(params) => { return tansParams(params) }],
    headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
    responseType: 'blob',
    ...config
  }).then(async (data) => {
    const isBlob = blobValidate(data);
    if (isBlob) {
      const blob = new Blob([data]);
      saveAs(blob, filename);
    } else {
      const resText = await data.text();
      const rspObj = JSON.parse(resText);
      if (rspObj && rspObj.kind) {
        handleError(rspObj);
      } else {
        const errMsg = errorCode[rspObj.code] || rspObj.msg || errorCode['default'];
        ElMessage.error(errMsg);
      }
    }
    downloadLoadingInstance.close();
  }).catch((r) => {
    console.error(r);
    ElMessage.error('下载文件出现错误，请联系管理员！');
    downloadLoadingInstance.close();
  });
}

function handleError(error) {
  const errorKind = error.kind;
  const details = error.details || '';

  switch (errorKind) {
    case 'UnknownError':
      ElMessage.error(`未知错误: ${details}`);
      break;
    case 'AccountDisabled':
      ElMessage.error(`账户已禁用: ${details}`);
      break;
    case 'DbError':
      ElMessage.error(`数据库错误: ${details}`);
      break;
    case 'ConfigParseError':
      ElMessage.error(`配置解析错误: ${details}`);
      break;
    case 'NotFound':
      ElMessage.error(`未找到资源: ${details}`);
      break;
    case 'InternalServer':
      ElMessage.error(`服务器内部错误: ${details}`);
      break;
    case 'BodyParsing':
      ElMessage.error(`请求体解析错误: ${details}`);
      break;
    case 'PathParsing':
      ElMessage.error(`路径解析错误: ${details}`);
      break;
    case 'ParseError':
      ElMessage.error(`解析错误: ${details}`);
      break;
    case 'IOError':
      ElMessage.error(`IO 错误: ${details}`);
      break;
    case 'BadRequest':
      ElMessage.error(`请求错误: ${details}`);
      break;
    case 'AccountOrPassword':
      ElMessage.error(`账户或密码错误: ${details}`);
      break;
    case 'CodeIsExpired':
      ElMessage.error(`验证码已过期: ${details}`);
      break;
    case 'CodeIsInvalid':
      ElMessage.error(`验证码无效: ${details}`);
      break;
    case 'SmsNotSubscribed':
      ElMessage.error(`未订阅短信服务，请先订阅短信服务`);
      break;
    case 'SmsRemainShort':
      ElMessage.error(`短信余量不足，请充值`);
      break;
    default:
      ElMessage.error(`未知错误类型: ${details}`);
      break;
  }
}

export default service
