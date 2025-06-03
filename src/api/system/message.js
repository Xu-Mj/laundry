import invoke from '@/utils/invoke';

/**
 * 获取消息列表
 * @param {Object} msg - 查询参数
 * @returns {Promise} - 返回消息列表
 */
export function saveMessage(msg) {
  return invoke('save_message', { msg });
}

/**
 * 获取消息列表
 * @param {Object} params - 查询参数
 * @param {number} params.pageNum - 页码
 * @param {number} params.pageSize - 每页数量
 * @param {string} [params.messageType] - 消息类型
 * @param {boolean} [params.read] - 是否已读
 * @returns {Promise} - 返回消息列表
 */
export function getMessages(params) {
  const msg = {
    messageType: params.messageType,
    read: params.read
  }
  return invoke('get_messages', {
    params: {
      page: params.page,
      pageSize: params.pageSize
    },
    msg
  });
}

/**
 * 标记消息为已读
 * @param {String} ids - 消息ID，多个ID用逗号分隔
 * @returns {Promise} - 返回操作结果
 */
export function readMessages(ids) {
  return invoke('mark_messages_as_read', { ids: [].concat(ids) });
}

/**
 * 删除消息
 * @param {String} messageIds - 消息ID，多个ID用逗号分隔
 * @returns {Promise} - 返回操作结果
 */
export function deleteMessage(ids) {
  return invoke('delete_message', { ids: [].concat(ids) });
}

export function clearMessages(t) {
  return invoke('clear_messages', { t });
}

/**
 * 获取未读消息数量
 * @param {Object} id - 查询参数
 * @returns {Promise} - 返回未读消息数量
 */
export function getUnreadCount(id) {
  return invoke('get_unread_message_count', id);
}

/**
 * 拉取离线消息
 * @param {Object} query - 查询参数
 * @returns {Promise} - 返回离线消息列表
 */
export function fetchOfflineMessages(query) {
  return invoke('fetch_offline_messages', query);
}