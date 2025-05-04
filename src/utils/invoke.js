import { invoke as invokeCommand } from '@tauri-apps/api/core'
import Notification from '@/utils/notification';
import { removeToken } from '@/utils/auth';
import useUserStore from '@/store/modules/user'
import { ElMessageBox } from 'element-plus'

const errorMessages = {
    UnknownError: "未知错误",
    DbError: "数据库操作失败",
    ConfigReadError: "配置读取失败",
    ConfigParseError: "配置解析失败",
    NotFound: "资源未找到",
    ParseError: "解析错误",
    InternalServer: "内部错误",
    IOError: "输入/输出错误",
    ReqwestError: "网络请求错误--请检查网络连接",
    BadRequest: "请求参数错误",
    PrintError: "打印失败",
    PrinterNotSet: "打印机未设置",
    PrinterNotFound: "打印机未找到",
    InvalidPassword: "用户名或密码错误",
    default: "未知错误，请联系管理员",
    AccountOrPassword: "用户名或密码错误",
    AccountNotRegister: "该账号未注册",
    RegisterDenied: "该账号注册还未通过审核，请联系管理员",
    SmsNotSubscribed: "未订阅短信服务，请先订阅",
    SmsRemainShort: "短信余量不足，请充值",
};

async function invoke(command, params = {}) {
    try {
        return await invokeCommand(command, params);
    } catch (error) {
        console.error(`Error invoking command ${command}:`, error);
        const errorMsg = errorMessages[error.kind] || errorMessages.default;
        let details = '';

        if (error.kind !== 'ReqwestError') {
            details = error.details ? `: ${error.details}` : '';
        } 

        // 处理UnAuthorized错误
        if (error.kind === 'UnAuthorized') {
            ElMessageBox.confirm('登录状态已过期，您可以继续留在该页面，或者重新登录', '系统提示', {
                confirmButtonText: '重新登录',
                cancelButtonText: '取消',
                type: 'warning'
              }).then(() => {
                useUserStore().logOut().then(() => {
                  location.href = '/index';
                });
              });
            // Notification.error('登录过期，请重新登录');
            // // 清除token
            // removeToken();
            // // 跳转到登录页面
            // window.location.href = '/login';
            return Promise.reject('无效的会话，或者会话已过期，请重新登录。');
        } else 

        // 处理UnAuthorized错误
        if (error.kind === 'UnAuthorizedDevice') {
            details = error.details ? `${error.details}` : '未授权设备';
            ElMessageBox.alert(details, '警告', {
                confirmButtonText: '确认',
              })
            // 清除token
            removeToken();
            return Promise.reject('未授权设备，请联系管理员。');
        }

        // ElMessage({ message: `${errorMsg}${details}`, type: 'error', duration: 5000 });
        Notification.error(`${errorMsg}${details}`);
        throw error; // 继续向上抛出错误，供调用者处理
    }
}

export default invoke;
