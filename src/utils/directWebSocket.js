import { getToken } from '@/utils/auth';
import Notification from '@/utils/notification';
import useUserStore from '@/store/modules/user';
import { saveMessage, } from '@/api/system/message';
import { addServerOrders } from '@/api/system/orders';
import { addUser } from '@/api/system/user';

/**
 * 直连WebSocket管理类
 * 负责直接与服务端建立WebSocket连接，处理消息的接收和发送
 * 在需要持久化数据时调用Tauri命令
 */
class DirectWebSocketManager {
    constructor() {
        this.socket = null;
        this.isConnected = false;
        this.reconnectAttempts = 0;
        this.maxReconnectAttempts = 10;
        this.reconnectInterval = 3000; // 初始重连间隔3秒
        this.reconnectTimer = null;
        this.heartbeatTimer = null; // 心跳定时器
        this.heartbeatInterval = 30000; // 心跳间隔30秒
        this.messageListeners = new Map(); // 消息监听器集合
        this.messageQueue = []; // 消息队列，用于存储离线时的消息
        this.lastMessageId = 0; // 最后接收到的消息ID
        this.manualClose = false; // 标记是否为手动关闭连接

        // 支持的消息类型
        this.messageTypes = {
            SUBSCRIPTION: 'subscription',   // 订阅相关通知
            APPOINTMENT: 'appointment',     // 预约消息
            ORDER: 'order',                // 线上订单
            SMS_BALANCE: 'sms_balance',    // 短信余量通知
            SYSTEM: 'system',              // 系统消息
            ORDER_UPDATE: 'order_update',  // 订单状态更新
            ORDER_PAYMENT: 'order_payment', // 订单支付消息
            PAYMENT_UPDATE: 'payment_update', // 支付状态更新
            DELIVERY_UPDATE: 'delivery_update' // 配送状态更新
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
        // 清除之前的重连定时器
        if (this.reconnectTimer) {
            clearTimeout(this.reconnectTimer);
            this.reconnectTimer = null;
        }

        // 如果已经连接，先关闭之前的连接
        if (this.socket) {
            console.log('WebSocket已存在，关闭之前的连接');
            this.close();
        }

        try {
            // 获取token
            const token = getToken();
            if (!token) {
                console.error('WebSocket初始化失败: 未找到token');
                return false;
            }

            const userId = useUserStore().id;
            // 构建WebSocket URL，添加token作为查询参数
            const wsUrl = `${serverUrl}/${userId}/${encodeURIComponent(token)}`;
            console.log('正在连接WebSocket服务器:', serverUrl);

            // 重置连接状态
            this.isConnected = false;
            this.manualClose = false;

            // 直接在Web端创建WebSocket连接
            this.socket = new WebSocket(wsUrl);

            // 设置事件处理器
            this.socket.onopen = this.handleOpen.bind(this);
            this.socket.onmessage = this.handleMessage.bind(this);
            this.socket.onclose = this.handleClose.bind(this);
            this.socket.onerror = this.handleError.bind(this);

            return true;
        } catch (error) {
            console.error('WebSocket初始化异常:', error);
            this.reconnect();
            return false;
        }
    }

    /**
     * 处理连接打开事件
     */
    handleOpen() {
        this.isConnected = true;
        this.reconnectAttempts = 0;
        console.log('WebSocket连接成功，连接已建立');

        // 添加连接状态日志
        console.log('WebSocket连接状态:', this.socket.readyState, '(0-连接中, 1-已连接, 2-正在关闭, 3-已关闭)');

        // 连接成功后发送队列中的消息
        this.sendQueuedMessages();
    }

    /**
     * 处理连接关闭事件
     * @param {CloseEvent} event - 关闭事件
     */
    handleClose(event) {
        this.isConnected = false;
        console.log('WebSocket连接关闭，代码:', event.code, '原因:', event.reason);

        // 只有在非手动关闭的情况下才进行重连
        if (!this.manualClose) {
            this.reconnect();
        } else {
            console.log('WebSocket连接被手动关闭，不进行重连');
            this.manualClose = false; // 重置手动关闭标志
        }
    }

    /**
     * 处理连接错误事件
     * @param {Event} event - 错误事件
     */
    handleError(event) {
        this.isConnected = false;
        console.error('WebSocket连接错误:', event);
        this.reconnect();
    }

    /**
     * 重新连接WebSocket
     */
    reconnect() {
        // 如果是手动关闭，不进行重连
        if (this.manualClose) {
            console.log('WebSocket是手动关闭的，不进行重连');
            return;
        }

        // 如果已经连接，不进行重连
        if (this.isConnected) {
            console.log('WebSocket已经连接，不进行重连');
            return;
        }

        if (this.reconnectAttempts >= this.maxReconnectAttempts) {
            console.error('WebSocket重连次数超过最大限制，停止重连');
            // 通知用户连接已断开，可能需要刷新页面
            Notification.error('消息系统连接已断开，请刷新页面重试');
            return;
        }

        if (this.reconnectTimer) {
            clearTimeout(this.reconnectTimer);
        }

        // 使用指数退避算法增加重连间隔
        const delay = Math.min(30000, this.reconnectInterval * Math.pow(1.5, this.reconnectAttempts));

        console.log(`WebSocket将在${delay / 1000}秒后尝试重连...`);

        this.reconnectTimer = setTimeout(async () => {
            this.reconnectAttempts++;
            console.log(`WebSocket重连尝试 ${this.reconnectAttempts}/${this.maxReconnectAttempts}`);

            // 获取服务器地址并重新初始化连接
            const serverUrl = await this.getServerUrl();
            if (serverUrl) {
                await this.init(serverUrl);
            }
        }, delay);
    }

    /**
     * 获取WebSocket服务器地址
     * 可以从配置中读取或通过API获取
     */
    async getServerUrl() {
        return 'ws://127.0.0.1:50000/ws';
    }

    /**
     * 关闭WebSocket连接
     */
    async close() {
        try {
            // 设置手动关闭标志，防止触发自动重连
            this.manualClose = true;

            // 停止心跳检测
            this.stopHeartbeat();

            if (this.socket) {
                this.socket.close();
                this.socket = null;
            }

            this.isConnected = false;

            if (this.reconnectTimer) {
                clearTimeout(this.reconnectTimer);
                this.reconnectTimer = null;
            }

            console.log('WebSocket连接已手动关闭');
        } catch (error) {
            console.error('关闭WebSocket连接失败:', error);
            this.manualClose = false; // 出错时重置标志
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
            // 如果未连接，将消息加入队列
            this.messageQueue.push(message);
            console.log('WebSocket未连接，消息已加入队列');
            return false;
        }

        try {
            // 直接通过WebSocket发送消息
            this.socket.send(JSON.stringify(message));
            return true;
        } catch (error) {
            console.error('发送WebSocket消息失败:', error);
            // 发送失败，将消息加入队列
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

        // 复制队列并清空原队列
        const queuedMessages = [...this.messageQueue];
        this.messageQueue = [];

        // 逐个发送消息
        for (const message of queuedMessages) {
            try {
                this.socket.send(JSON.stringify(message));
            } catch (error) {
                console.error('发送队列消息失败:', error);
                // 发送失败，重新加入队列
                this.messageQueue.push(message);
            }
        }
    }

    /**
     * 处理接收到的消息
     * @param {MessageEvent} event - WebSocket消息事件
     */
    async handleMessage(event) {
        try {
            // 解析消息数据
            const data = event.data;
            const message = typeof data === 'string' ? JSON.parse(data) : data;

            console.log('接收到WebSocket消息:', message);

            // 如果是心跳响应消息，直接返回
            if (message.type === 'pong') {
                console.log('收到服务器心跳响应');
                return;
            }

            // 更新最后接收到的消息ID
            if (message.id && message.id > this.lastMessageId) {
                this.lastMessageId = message.id;
            }

            // 根据消息类型处理消息内容
            let processedMessage = { ...message };
            switch (message.type) {
                case 'order_update':
                case 'order_payment':
                    // 解析订单数据
                    const orderData = JSON.parse(message.content);
                    // 调用保存订单命令
                    await addServerOrders(orderData);
                    // 更新消息内容
                    processedMessage.content = `用户${orderData.order.nickName}下单成功！`;
                    break;
                case 'new_user_register':
                    // 解析用户数据
                    const userData = JSON.parse(message.content);
                    // 调用添加用户命令
                    await addUser(userData);
                    // 更新消息内容
                    processedMessage.content = `新用户${userData.nickName}注册成功！`;
                    break;
            }

            // 检查是否需要持久化消息
            if (this.shouldPersistMessage(processedMessage)) {
                await this.persistMessage(processedMessage);
            }

            // 根据消息类型分发给对应的监听器
            if (processedMessage.type && this.messageListeners.has(processedMessage.type)) {
                const listeners = this.messageListeners.get(processedMessage.type);
                listeners.forEach(listener => listener(processedMessage.data));
            }

            // 触发通用消息监听器
            if (this.messageListeners.has('*')) {
                const listeners = this.messageListeners.get('*');
                listeners.forEach(listener => listener(processedMessage));
            }
        } catch (error) {
            console.error('处理WebSocket消息失败:', error);
        }
    }

    /**
     * 判断消息是否需要持久化
     * @param {object} message - 消息对象
     * @returns {boolean} - 是否需要持久化
     */
    shouldPersistMessage(message) {
        // 检查消息类型是否在需要持久化的列表中
        // return this.persistentMessageTypes.includes(message.type_name) &&
        //     message.sender_id &&
        //     message.receiver_id;
        return true;
    }

    /**
     * 持久化消息到数据库
     * @param {object} message - 消息对象
     */
    async persistMessage(message) {
        try {
            // 调用Tauri命令保存消息
            await saveMessage(message);

            console.log('消息已持久化到数据库');
        } catch (error) {
            console.error('持久化消息失败:', error);
        }
    }

    /**
     * 添加消息监听器
     * @param {string} type - 消息类型
     * @param {function} listener - 监听函数
     */
    addMessageListener(type, listener) {
        if (!this.messageListeners.has(type)) {
            this.messageListeners.set(type, []);
        }
        this.messageListeners.get(type).push(listener);
    }

    /**
     * 移除消息监听器
     * @param {string} type - 消息类型
     * @param {function} listener - 监听函数
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

        // 如果没有监听器了，删除该类型
        if (listeners.length === 0) {
            this.messageListeners.delete(type);
        }
    }

    /**
     * 开始心跳检测
     */
    startHeartbeat() {
        // 清除之前的心跳定时器
        this.stopHeartbeat();

        // 设置新的心跳定时器
        this.heartbeatTimer = setInterval(() => {
            if (this.isConnected && this.socket && this.socket.readyState === WebSocket.OPEN) {
                // 发送心跳消息
                try {
                    const heartbeatMessage = {
                        type: 'ping',
                        timestamp: new Date().getTime()
                    };
                    this.socket.send(JSON.stringify(heartbeatMessage));
                    console.log('发送心跳消息');
                } catch (error) {
                    console.error('发送心跳消息失败:', error);
                }
            } else if (this.socket) {
                // 如果连接已关闭但未触发关闭事件，手动触发重连
                console.log('心跳检测到连接异常，当前状态:', this.socket.readyState);
                if (this.socket.readyState !== WebSocket.OPEN && !this.manualClose) {
                    this.reconnect();
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
}

// 创建单例实例
const directWebSocketManager = new DirectWebSocketManager();

export default directWebSocketManager;