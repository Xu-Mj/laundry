import tauriWebSocketManager from '@/utils/tauriWebSocket';
import { getToken } from '@/utils/auth';
import Notification from '@/utils/notification';
import useUserStore from '@/store/modules/user';

let isInitialized = false;

/**
 * 初始化Tauri WebSocket连接
 * 在应用启动时调用此函数，使用Tauri的WebSocket插件建立与服务器的连接
 */
export async function initTauriWebSocketConnection() {
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
  
  // 检查是否为游客登录，游客不初始化WebSocket连接
  const userStore = useUserStore();
  if (userStore.trial && userStore.trial.isGuest) {
    console.log('游客登录，不初始化WebSocket连接');
    return false;
  }

  try {
    // 获取WebSocket服务器地址
    const serverUrl = await tauriWebSocketManager.getServerUrl();
    
    // 初始化WebSocket连接
    const connected = await tauriWebSocketManager.init(serverUrl);
    
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
export async function closeTauriWebSocketConnection() {
  try {
    await tauriWebSocketManager.close();
    isInitialized = false;
    console.log('WebSocket连接已关闭');
    return true;
  } catch (error) {
    console.error('关闭WebSocket连接时发生错误:', error);
    return false;
  }
} 