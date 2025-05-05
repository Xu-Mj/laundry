<template>
  <div class="message-center">
    <!-- 消息图标和未读消息数量徽章 -->
    <div class="message-icon-container" @click="openMessageDialog">
      <el-badge :value="unreadCount" :hidden="unreadCount === 0" :max="99">
        <el-icon class="message-icon">
          <Message />
        </el-icon>
      </el-badge>
    </div>

    <!-- 消息中心弹窗 -->
    <el-dialog v-model="dialogVisible" title="消息中心" width="920px" :show-close="false" destroy-on-close append-to-body
      align-center class="message-dialog">
      <template #header>
        <div class="dialog-header">
          <h3 class="dialog-title">消息中心</h3>
          <div class="dialog-actions">
            <el-button type="primary" plain size="small" @click="markAllAsRead">全部已读</el-button>
            <el-button circle @click="dialogVisible = false">
              <el-icon>
                <Close />
              </el-icon>
            </el-button>
          </div>
        </div>
      </template>

      <div class="message-container">
        <!-- 左侧消息类型导航 -->
        <div class="message-nav">
          <el-menu :default-active="activeTab" class="message-menu" mode="horizontal" @select="handleTypeSelect">
            <el-menu-item index="all">
              <el-icon>
                <MessageBox />
              </el-icon>
              <template #title>
                <span>全部消息</span>
                <el-badge :value="unreadCount" :max="99" :hidden="unreadCount === 0" class="menu-badge" />
              </template>
            </el-menu-item>

            <el-menu-item v-for="(typeId, index) in Object.keys(typeConfig)" :key="index" :index="typeId">
              <el-icon :style="{ color: getTypeConfig(typeId).color }">
                <component :is="getTypeConfig(typeId).icon"></component>
              </el-icon>
              <template #title>
                <span>{{ getTypeConfig(typeId).title }}</span>
                <el-badge :value="getUnreadCountByType(typeId)" :max="99" :hidden="getUnreadCountByType(typeId) === 0"
                  class="menu-badge" />
              </template>
            </el-menu-item>
          </el-menu>
        </div>

        <!-- 右侧消息列表 -->
        <div class="message-content-wrapper">
          <div class="message-header">
            <div class="message-title">{{ getActiveTypeTitle() }}</div>
            <div class="message-filter">
              <el-radio-group v-model="readStatus" size="small" @change="handleFilterChange">
                <el-radio-button label="all">全部</el-radio-button>
                <el-radio-button label="unread">未读</el-radio-button>
                <el-radio-button label="read">已读</el-radio-button>
              </el-radio-group>
            </div>
          </div>

          <div class="message-list-container">
            <div class="empty-wrapper" v-if="filteredMessages.length === 0">
              <el-empty description="暂无消息" />
            </div>

            <div v-else class="message-list">
              <div v-for="message in filteredMessages" :key="message.id" class="message-item"
                :class="{ 'unread': !message.read }" @click="handleMessageClick(message)">
                <div class="message-item-header">
                  <div class="message-icon-wrapper">
                    <el-icon :size="20" :color="getTypeConfig(message.type).color">
                      <component :is="getTypeConfig(message.type).icon"></component>
                    </el-icon>
                  </div>
                  <div class="message-meta">
                    <div class="message-title">{{ message.title }}</div>
                    <div class="message-time">{{ formatTime(message.time) }}</div>
                  </div>
                </div>

                <div class="message-text">{{ message.content }}</div>

                <div class="message-actions">
                  <el-button type="primary" plain size="small" @click.stop="markAsRead(message)" v-if="!message.read">
                    <el-icon>
                      <Check />
                    </el-icon>
                    <span>标为已读</span>
                  </el-button>
                  <el-button type="danger" plain size="small" @click.stop="delMsg(message)">
                    <el-icon>
                      <Delete />
                    </el-icon>
                    <span>删除</span>
                  </el-button>
                </div>
              </div>
            </div>

            <el-pagination v-show="pagination.total > 0" v-model:current-page="pagination.page"
              v-model:page-size="pagination.pageSize" :total="pagination.total" :page-sizes="[10, 20, 30, 50]"
              layout="total, sizes, prev, pager, next" @size-change="handlePaginationChange"
              @current-change="handlePaginationChange" />

            <div v-if="filteredMessages.length > 0" class="message-actions-footer">
              <el-button type="danger" plain @click="clearTypeMessages(activeTab)">清空消息</el-button>
            </div>
          </div>
        </div>
      </div>
    </el-dialog>
  </div>
</template>

<script setup>
import { Check, Delete, Close, MessageBox } from '@element-plus/icons-vue';
import { ref, computed, onMounted, onUnmounted } from 'vue';
import directWebSocketManager from '@/utils/directWebSocket';
import Notification from '@/utils/notification';
import { parseTime } from '@/utils/ruoyi';
import { getUnreadCount, getMessages, deleteMessage, clearMessages } from '@/api/system/message';

// 消息类型常量
const MESSAGE_TYPES = {
  SUBSCRIPTION: 'subscription', // 订阅相关通知
  APPOINTMENT: 'appointment',   // 预约消息
  ORDER: 'order',               // 线上订单
  SMS_BALANCE: 'sms_balance',   // 短信余量通知
  SYSTEM: 'system'              // 系统消息
};

const activeTab = ref('all'); // 默认显示全部消息
const dialogVisible = ref(false);
const readStatus = ref('all'); // 消息筛选状态：all, unread, read
const messages = ref([]); // 消息列表
const unreadCount = ref(0); // 未读消息数量
const pagination = ref({
  page: 1,
  pageSize: 10,
  total: 0
});

// 消息类型配置
const typeConfig = {
  [MESSAGE_TYPES.SUBSCRIPTION]: {
    icon: 'Ticket',
    color: '#409EFF',
    title: '订阅通知',
    description: '与订阅相关的通知消息',
    sound: 'notification.mp3'
  },
  [MESSAGE_TYPES.APPOINTMENT]: {
    icon: 'Calendar',
    color: '#67C23A',
    title: '预约消息',
    description: '客户线上预约的消息',
    sound: 'appointment.mp3'
  },
  [MESSAGE_TYPES.ORDER]: {
    icon: 'ShoppingCart',
    color: '#E6A23C',
    title: '线上订单',
    description: '新的线上订单通知',
    sound: 'order.mp3'
  },
  [MESSAGE_TYPES.SMS_BALANCE]: {
    icon: 'ChatDotRound',
    color: '#F56C6C',
    title: '短信余量',
    description: '短信余量提醒',
    sound: 'notification.mp3'
  },
  [MESSAGE_TYPES.SYSTEM]: {
    icon: 'Bell',
    color: '#909399',
    title: '系统消息',
    description: '系统通知和消息',
    sound: 'notification.mp3'
  }
};

// 计算属性：根据当前选择的类型和筛选条件过滤消息
const filteredMessages = computed(() => {
  return messages.value;
});

// 获取指定类型的未读消息数量
const getUnreadCountByType = (type) => {
  return messages.value.filter(msg => msg.messageType === type && !msg.read).length;
};

// 获取消息类型配置
const getTypeConfig = (type) => {
  return typeConfig[type] || typeConfig[MESSAGE_TYPES.SYSTEM];
};

// 获取当前选中类型的标题
const getActiveTypeTitle = () => {
  if (activeTab.value === 'all') {
    return '全部消息';
  }
  return getTypeConfig(activeTab.value).title;
};

// 格式化时间
const formatTime = (timestamp) => {
  if (!timestamp) return '';
  return parseTime(timestamp);
};

// 打开消息弹窗
const openMessageDialog = () => {
  dialogVisible.value = true;

  // 切换到第一个有未读消息的标签页
  if (unreadCount.value > 0) {
    // 先检查是否有未读消息
    for (const type of Object.values(MESSAGE_TYPES)) {
      if (getUnreadCountByType(type) > 0) {
        activeTab.value = type;
        return;
      }
    }
  }

  // 如果没有未读消息，默认显示全部
  activeTab.value = 'all';
};

// 获取消息列表
const fetchMessages = async () => {
  try {
    const params = {
      page: pagination.value.page,
      pageSize: pagination.value.pageSize,
      messageType: activeTab.value === 'all' ? null : activeTab.value,
      read: readStatus.value === 'all' ? null : readStatus.value === 'read'
    };

    const res = await getMessages(params);
    messages.value = res.rows;
    pagination.value.total = res.total;
  } catch (error) {
    console.error('获取消息列表失败:', error);
    Notification.error('获取消息列表失败');
  }
};

// 处理消息类型选择
const handleTypeSelect = (index) => {
  activeTab.value = index;
  pagination.value.page = 1;
  fetchMessages();
};

// 处理筛选条件变更
const handleFilterChange = () => {
  pagination.value.page = 1;
  fetchMessages();
};

// 处理分页变更
const handlePaginationChange = (page) => {
  pagination.value.page = page;
  fetchMessages();
};

// 处理消息点击事件
const handleMessageClick = (message) => {
  // 标记消息为已读
  markAsRead(message);

  // 处理消息相关跳转或操作
  const messageType = message.type;

  // 根据消息类型执行不同操作
  switch (messageType) {
    case MESSAGE_TYPES.ORDER:
      if (message.data && message.data.orderId) {
        // 跳转到订单详情
        Notification.info(`查看订单 #${message.data.orderId}`);
        // router.push(`/order/detail/${message.data.orderId}`);
      }
      break;
    case MESSAGE_TYPES.APPOINTMENT:
      // 跳转到预约管理
      Notification.info('查看预约消息');
      // router.push('/appointment');
      break;
    case MESSAGE_TYPES.SUBSCRIPTION:
      // 跳转到订阅管理
      Notification.info('查看订阅消息');
      // router.push('/subscription');
      break;
    case MESSAGE_TYPES.SMS_BALANCE:
      // 跳转到短信设置
      Notification.info('查看短信余量');
      // router.push('/system/sms');
      break;
    default:
      // 默认行为
      Notification.info(`处理消息: ${message.title}`);
      break;
  }
};

// 标记消息为已读
const markAsRead = async (message) => {
  if (!message.read) {
    try {
      await directWebSocketManager.markMessagesAsRead([message.id]);
      message.read = true;
      unreadCount.value--;
    } catch (error) {
      console.error('标记消息为已读失败:', error);
      Notification.error('标记消息为已读失败');
    }
  }
};

// 标记所有消息为已读
const markAllAsRead = async () => {
  try {
    const unreadIds = messages.value
      .filter(msg => !msg.read)
      .map(msg => msg.id);

    if (unreadIds.length > 0) {
      await directWebSocketManager.markMessagesAsRead(unreadIds);
      messages.value.forEach(msg => {
        if (!msg.read) {
          msg.read = true;
        }
      });
      unreadCount.value = 0;
      Notification.success('所有消息已标记为已读');
    }
  } catch (error) {
    console.error('标记所有消息为已读失败:', error);
    Notification.error('标记所有消息为已读失败');
  }
};

// 删除消息
const delMsg = async (message) => {
  try {
    if (await deleteMessage(message.id)) {
      Notification.success('消息已删除');
      fetchMessages();
      unreadCount.value = await getUnreadCount();
    } else {
      Notification.error('删除消息失败');
    }
  } catch (error) {
    console.error('删除消息失败:', error);
    Notification.error('删除消息失败');
  }
};

// 清空指定类型的消息
const clearTypeMessages = async (type) => {
  try {
    if (type === 'all') {
      // 清空所有消息
      await clearMessages();
      messages.value = [];
      unreadCount.value = 0;
      pagination.value.page = 1;
      pagination.value.total = 0;
      Notification.success('已清空所有消息');
    } else {
      // 清空指定类型的消息
      if (await clearMessages(activeTab.value)) {
        await fetchMessages();
        unreadCount.value = await getUnreadCount();
        Notification.success(`已清空${getTypeConfig(type).title}`);
      } else {
        Notification.error('清空消息失败');
      }
    }
  } catch (error) {
    console.error('清空消息失败:', error);
    Notification.error('清空消息失败');
  }
};

// 处理新消息
const handleNewMessage = (message) => {
  // 添加到消息列表
  messages.value.unshift(message);

  // 更新未读消息数量
  if (!message.read) {
    unreadCount.value++;
  }

  // 显示通知
  Notification.ws_message(message.content);
};

// 初始化消息系统
const initMessageSystem = async () => {
  try {
    // 获取初始消息列表
    unreadCount.value = await getUnreadCount();
    await fetchMessages();

    // 添加消息监听器
    directWebSocketManager.addMessageListener('*', handleNewMessage);
  } catch (error) {
    console.error('初始化消息系统失败:', error);
    Notification.error('初始化消息系统失败');
  }
};

// 组件挂载时初始化消息系统
onMounted(() => {
  initMessageSystem();
});

// 组件卸载时移除消息监听器
onUnmounted(() => {
  directWebSocketManager.removeMessageListener('*', handleNewMessage);
});
</script>

<style lang="scss" scoped>
.message-center {
  display: inline-block;
  cursor: pointer;

  .message-icon-container {
    padding: 0 10px;
    height: 50px;
    display: flex;
    align-items: center;
    transition: all 0.3s;

    &:hover {
      .message-icon {
        transform: scale(1.1);
      }
    }

    .message-icon {
      font-size: 20px;
      color: var(--el-color-primary);
      transition: all 0.3s;
    }
  }



  .message-pagination {
    display: flex;
    justify-content: center;
    padding: 16px 0;
    border-top: 1px solid var(--el-border-color-lighter);
  }
}

// 消息对话框样式
.message-dialog {
  .el-dialog__header {
    margin: 0;
    padding: 0;
  }

  .el-dialog__body {
    padding: 0;
  }

  .dialog-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 20px;
    border-bottom: 1px solid var(--el-border-color-lighter);

    .dialog-title {
      margin: 0;
      font-size: 18px;
      font-weight: 600;
      color: var(--el-text-color-primary);
    }

    .dialog-actions {
      display: flex;
      align-items: center;
      gap: 12px;
    }
  }

  // 消息容器布局
  .message-container {
    display: flex;
    flex-direction: column;
    height: 600px;

    // 左侧导航
    .message-nav {
      width: 100%;
      border-bottom: 1px solid var(--el-border-color-lighter);
      background-color: var(--el-fill-color-light);
      border-radius: .2rem;

      .message-menu {
        border-bottom: none;
        display: flex;
        justify-content: flex-start;
        align-items: center;
        padding: 0 20px;

        .el-menu-item {
          display: flex;
          align-items: center;
          height: 50px;
          padding: 0 16px;

          .el-icon {
            margin-right: 8px;
            font-size: 18px;
          }

          &.is-active {
            color: var(--el-color-primary);

            &::before {
              content: '';
              position: absolute;
              bottom: 0;
              left: 0;
              right: 0;
              height: 3px;
              background-color: var(--el-color-primary);
            }
          }
        }

        .menu-badge {
          margin-left: 8px;
        }
      }
    }

    // 右侧内容区
    .message-content-wrapper {
      flex: 1;
      display: flex;
      flex-direction: column;
      overflow: hidden;

      // 消息头部
      .message-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 12px 20px;
        border-bottom: 1px solid var(--el-border-color-lighter);

        .message-title {
          font-size: 16px;
          font-weight: 600;
          color: var(--el-text-color-primary);
        }
      }

      // 消息列表容器
      .message-list-container {
        flex: 1;
        padding: 0 20px 20px;
        overflow-y: auto;

        .empty-wrapper {
          display: flex;
          justify-content: center;
          align-items: center;
          height: 100%;
        }

        // 消息列表
        .message-list {
          padding: 16px 0;
          display: flex;
          flex-direction: column;
          gap: 1rem;

          .message-item {
            display: flex;
            flex-direction: column;
            gap: 1rem;
            padding: 16px;
            border-radius: 8px;
            background-color: var(--el-fill-color-blank);
            box-shadow: var(--el-box-shadow-lighter);
            transition: all 0.3s;

            &:hover {
              box-shadow: var(--el-box-shadow-light);
              transform: translateY(-2px);
            }

            &.unread {
              border-left: 4px solid var(--el-color-primary);
              background-color: var(--el-color-primary-light-9);

              .message-title {
                font-weight: bold;
              }
            }

            // 消息项头部
            .message-item-header {
              display: flex;
              align-items: center;

              .message-icon-wrapper {
                width: 32px;
                height: 32px;
                border-radius: 6px;
                display: flex;
                justify-content: center;
                align-items: center;
                margin-right: 12px;
                flex-shrink: 0;
              }

              .message-meta {
                flex: 1;
                min-width: 0;

                .message-title {
                  font-size: 15px;
                  margin-bottom: 4px;
                  color: var(--el-text-color-primary);
                }

                .message-time {
                  font-size: 12px;
                  color: var(--el-text-color-secondary);
                }
              }
            }

            // 消息内容
            .message-text {
              font-size: 14px;
              color: var(--el-text-color-regular);
              line-height: 1.6;
              word-break: break-word;
            }

            // 消息操作按钮
            .message-actions {
              display: flex;
              justify-content: flex-end;
              gap: 12px;

              .el-button {
                padding: 6px 12px;

                .el-icon {
                  margin-right: 4px;
                }
              }
            }
          }
        }

        // 底部操作区
        .message-actions-footer {
          display: flex;
          justify-content: flex-end;
          padding: 16px 0;
          border-top: 1px solid var(--el-border-color-lighter);
        }
      }
    }
  }
}
</style>