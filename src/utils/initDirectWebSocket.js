import directWebSocketManager from '@/utils/directWebSocket';
import { getToken } from '@/utils/auth';
import Notification from '@/utils/notification';

let isInitialized = false;

/**
 * 初始化直连WebSocket连接
 * 在应用启动时调用此函数，直接在Web端建立与服务器的WebSocket连接
 */
export async function initDirectWebSocketConnection() {
  // 如果已经初始化过，直接返回
  if (isInitialized) {
    return true;
  }

  // 检查是否有token，没有token则不连接
  const token = getToken();
  if (!token) {
    console.log('未登录，不初始化WebSocket连接');
    return false;
  }

  try {
    // 获取WebSocket服务器地址
    const serverUrl = await directWebSocketManager.getServerUrl();
    
    // 初始化WebSocket连接
    const connected = await directWebSocketManager.init(serverUrl);
    
    if (connected) {
      console.log('WebSocket连接初始化成功');
      isInitialized = true;
      return true;
    } else {
      console.error('WebSocket连接初始化失败');
      return false;
    }
  } catch (error) {
    console.error('初始化WebSocket连接时发生错误:', error);
    Notification.error('消息系统连接失败，部分功能可能无法正常使用');
    return false;
  }
}

/**
 * 关闭WebSocket连接
 * 在用户登出时调用此函数，关闭WebSocket连接
 */
export async function closeDirectWebSocketConnection() {
  try {
    await directWebSocketManager.close();
    isInitialized = false;
    console.log('WebSocket连接已关闭');
    return true;
  } catch (error) {
    console.error('关闭WebSocket连接时发生错误:', error);
    return false;
  }
}