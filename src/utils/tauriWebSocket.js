import { getToken } from '@/utils/auth';
import Notification from '@/utils/notification';
import useUserStore from '@/store/modules/user';
import { saveMessage } from '@/api/system/message';
import { addServerOrders } from '@/api/system/orders';
import { addUser } from '@/api/system/user';
import WebSocket from '@tauri-apps/plugin-websocket';

/**
 * Tauri WebSocket管理类
 * 使用Tauri的WebSocket插件与服务端建立连接，处理消息的接收和发送
 */
class TauriWebSocketManager {
    constructor() {
        this.socket = null;
        this.isConnected = false;
        this.reconnectAttempts = 0;
        this.maxReconnectAttempts = 10;
        this.reconnectInterval = 3000;
        this.reconnectTimer = null;
        this.heartbeatTimer = null;
        this.heartbeatInterval = 30000;
        this.messageListeners = new Map();
        this.messageQueue = [];
        this.lastMessageId = 0;
        this.manualClose = false;
        this.connectionStateListeners = [];

        // 支持的消息类型
        this.messageTypes = {
            SUBSCRIPTION: 'subscription',
            APPOINTMENT: 'appointment',
            ORDER: 'order',
            SMS_BALANCE: 'sms_balance',
            SYSTEM: 'system',
            ORDER_UPDATE: 'order_update',
            ORDER_PAYMENT: 'order_payment',
            PAYMENT_UPDATE: 'payment_update',
            DELIVERY_UPDATE: 'delivery_update'
        };

        // 需要持久化的消息类型
        this.persistentMessageTypes = [
            'order_update',
            'order_payment',
            'payment_update',
            'delivery_update'
        ];

        // 消息状态
        this.messageStatus = {
            UNREAD: 0,
            READ: 1
        };
    }

    /**
     * 初始化WebSocket连接
     * @param {string} serverUrl - WebSocket服务器地址
     */
    async init(serverUrl) {
        if (this.reconnectTimer) {
            clearTimeout(this.reconnectTimer);
            this.reconnectTimer = null;
        }

        if (this.socket) {
            console.log('WebSocket已存在，关闭之前的连接');
            await this.close();
        }

        try {
            const token = getToken();
            if (!token) {
                console.error('WebSocket初始化失败: 未找到token');
                return false;
            }

            const userId = useUserStore().id;
            const wsUrl = `${serverUrl}/${userId}/${encodeURIComponent(token)}`;
            console.log('正在连接WebSocket服务器:', serverUrl);

            this.isConnected = false;
            this.manualClose = false;

            // 使用Tauri的WebSocket插件建立连接
            this.socket = await WebSocket.connect(wsUrl);

            // 设置消息监听器
            this.socket.addListener(this.handleMessage.bind(this));

            this.isConnected = true;
            this.reconnectAttempts = 0;
            console.log('WebSocket连接成功，连接已建立');

            // 通知连接状态变化
            this.notifyConnectionStateChange(true);

            // 发送队列中的消息
            await this.sendQueuedMessages();

            // 启动心跳检测
            this.startHeartbeat();

            return true;
        } catch (error) {
            console.error('WebSocket初始化异常:', error);
            
            // 通知连接状态变化
            this.notifyConnectionStateChange(false, error.message);
            
            this.reconnect();
            return false;
        }
    }

    /**
     * 关闭WebSocket连接
     */
    async close() {
        try {
            this.manualClose = true;
            this.stopHeartbeat();

            if (this.socket) {
                await this.socket.disconnect();
                this.socket = null;
            }

            this.isConnected = false;
            
            // 通知连接状态变化
            this.notifyConnectionStateChange(false, '连接已手动关闭');

            if (this.reconnectTimer) {
                clearTimeout(this.reconnectTimer);
                this.reconnectTimer = null;
            }

            console.log('WebSocket连接已手动关闭');
        } catch (error) {
            console.error('关闭WebSocket连接失败:', error);
            this.manualClose = false;
        }
    }

    /**
     * 发送消息
     * @param {string} type - 消息类型
     * @param {object} data - 消息数据
     */
    async send(type, data) {
        const message = {
            type,
            data,
            timestamp: new Date().getTime()
        };

        if (!this.isConnected) {
            this.messageQueue.push(message);
            console.log('WebSocket未连接，消息已加入队列');
            return false;
        }

        try {
            await this.socket.send(JSON.stringify(message));
            return true;
        } catch (error) {
            console.error('发送WebSocket消息失败:', error);
            this.messageQueue.push(message);
            return false;
        }
    }

    /**
     * 发送队列中的消息
     */
    async sendQueuedMessages() {
        if (this.messageQueue.length === 0 || !this.isConnected) {
            return;
        }

        console.log(`发送队列中的消息，共${this.messageQueue.length}条`);

        const queuedMessages = [...this.messageQueue];
        this.messageQueue = [];

        for (const message of queuedMessages) {
            try {
                await this.socket.send(JSON.stringify(message));
            } catch (error) {
                console.error('发送队列消息失败:', error);
                this.messageQueue.push(message);
            }
        }
    }

    /**
     * 处理接收到的消息
     * @param {any} msg - WebSocket消息
     */
    async handleMessage(msg) {
        try {
            const message = typeof msg === 'string' ? JSON.parse(msg) : msg;
            console.log('接收到WebSocket消息:', message);

            if (message.type === 'Pong') {
                console.log('收到服务器心跳响应');
                return;
            }

            if(message.type === 'Ping') {
                this.socket.send('Pong');
                return;
            }

            if (message.id && message.id > this.lastMessageId) {
                this.lastMessageId = message.id;
            }

            let processedMessage = { ...message };
            switch (message.type) {
                case 'order_update':
                case 'order_payment':
                    const orderData = JSON.parse(message.content);
                    await addServerOrders(orderData);
                    processedMessage.content = `用户${orderData.order.nickName}下单成功！`;
                    break;
                case 'new_user_register':
                    const userData = JSON.parse(message.content);
                    await addUser(userData);
                    processedMessage.content = `新用户${userData.nickName}注册成功！`;
                    break;
            }

            if (this.shouldPersistMessage(processedMessage)) {
                await this.persistMessage(processedMessage);
            }

            if (processedMessage.type && this.messageListeners.has(processedMessage.type)) {
                const listeners = this.messageListeners.get(processedMessage.type);
                listeners.forEach(listener => listener(processedMessage.data));
            }

            if (this.messageListeners.has('*')) {
                const listeners = this.messageListeners.get('*');
                listeners.forEach(listener => listener(processedMessage));
            }
        } catch (error) {
            console.error('处理WebSocket消息失败:', error);
        }
    }

    /**
     * 重新连接WebSocket
     */
    reconnect() {
        if (this.manualClose || this.isConnected) {
            return;
        }

        if (this.reconnectAttempts >= this.maxReconnectAttempts) {
            console.error('WebSocket重连次数超过最大限制，停止重连');
            Notification.error('消息系统连接已断开，请刷新页面重试');
            
            // 通知连接状态变化
            this.notifyConnectionStateChange(false, '重连次数超过最大限制，请刷新页面');
            
            return;
        }

        if (this.reconnectTimer) {
            clearTimeout(this.reconnectTimer);
        }

        const delay = Math.min(30000, this.reconnectInterval * Math.pow(1.5, this.reconnectAttempts));
        console.log(`WebSocket将在${delay / 1000}秒后尝试重连...`);

        this.reconnectTimer = setTimeout(async () => {
            this.reconnectAttempts++;
            console.log(`WebSocket重连尝试 ${this.reconnectAttempts}/${this.maxReconnectAttempts}`);

            const serverUrl = await this.getServerUrl();
            if (serverUrl) {
                await this.init(serverUrl);
            }
        }, delay);
    }

    /**
     * 获取WebSocket服务器地址
     */
    async getServerUrl() {
        return import.meta.env.VITE_APP_WEBSOCKET_URL;
    }

    /**
     * 开始心跳检测
     */
    startHeartbeat() {
        this.stopHeartbeat();

        this.heartbeatTimer = setInterval(async () => {
            if (this.isConnected && this.socket) {
                try {
                    await this.socket.send('Ping');
                    console.log('发送心跳消息');
                } catch (error) {
                    console.error('发送心跳消息失败:', error);
                    if (!this.manualClose) {
                        this.reconnect();
                    }
                }
            }
        }, this.heartbeatInterval);
    }

    /**
     * 停止心跳检测
     */
    stopHeartbeat() {
        if (this.heartbeatTimer) {
            clearInterval(this.heartbeatTimer);
            this.heartbeatTimer = null;
        }
    }

    /**
     * 判断消息是否需要持久化
     */
    shouldPersistMessage(message) {
        return true;
    }

    /**
     * 持久化消息到数据库
     */
    async persistMessage(message) {
        try {
            await saveMessage(message);
            console.log('消息已持久化到数据库');
        } catch (error) {
            console.error('持久化消息失败:', error);
        }
    }

    /**
     * 添加消息监听器
     */
    addMessageListener(type, listener) {
        if (!this.messageListeners.has(type)) {
            this.messageListeners.set(type, []);
        }
        this.messageListeners.get(type).push(listener);
    }

    /**
     * 移除消息监听器
     */
    removeMessageListener(type, listener) {
        if (!this.messageListeners.has(type)) {
            return;
        }

        const listeners = this.messageListeners.get(type);
        const index = listeners.indexOf(listener);

        if (index !== -1) {
            listeners.splice(index, 1);
        }

        if (listeners.length === 0) {
            this.messageListeners.delete(type);
        }
    }

    /**
     * 添加连接状态监听器
     * @param {Function} listener - 连接状态变化监听器函数
     */
    addConnectionStateListener(listener) {
        if (typeof listener === 'function' && !this.connectionStateListeners.includes(listener)) {
            this.connectionStateListeners.push(listener);
            
            // 立即通知当前状态
            listener(this.isConnected, null);
        }
    }

    /**
     * 移除连接状态监听器
     * @param {Function} listener - 连接状态变化监听器函数
     */
    removeConnectionStateListener(listener) {
        const index = this.connectionStateListeners.indexOf(listener);
        if (index !== -1) {
            this.connectionStateListeners.splice(index, 1);
        }
    }

    /**
     * 通知所有监听器连接状态变化
     * @param {boolean} connected - 连接状态
     * @param {string} reason - 连接断开原因
     */
    notifyConnectionStateChange(connected, reason = null) {
        this.connectionStateListeners.forEach(listener => {
            try {
                listener(connected, reason);
            } catch (error) {
                console.error('执行连接状态监听器时出错:', error);
            }
        });
    }

    /**
     * 尝试重新连接
     * 手动触发重新连接
     */
    async attemptReconnect() {
        if (this.isConnected) {
            console.log('WebSocket已连接，无需重连');
            return true;
        }

        try {
            this.reconnectAttempts = 0;
            const serverUrl = await this.getServerUrl();
            if (!serverUrl) {
                throw new Error('获取WebSocket服务器地址失败');
            }
            
            return await this.init(serverUrl);
        } catch (error) {
            console.error('手动重连失败:', error);
            this.notifyConnectionStateChange(false, error.message);
            throw error;
        }
    }
}

// 创建单例实例
const tauriWebSocketManager = new TauriWebSocketManager();

export default tauriWebSocketManager; 