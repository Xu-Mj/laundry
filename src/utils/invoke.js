import { invoke as invokeCommand } from '@tauri-apps/api/core'
import Notification from '@/utils/notification';

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
};

async function invoke(command, params = {}) {
    try {
        return await invokeCommand(command, params);
    } catch (error) {
        const errorMsg = errorMessages[error.kind] || errorMessages.default;
        let details = '';

        if (error.kind !== 'ReqwestError') {
            details = error.details ? `: ${error.details}` : '';
        }

        // ElMessage({ message: `${errorMsg}${details}`, type: 'error', duration: 5000 });
        Notification.error(`${errorMsg}${details}`);
        throw error; // 继续向上抛出错误，供调用者处理
    }
}

export default invoke;
