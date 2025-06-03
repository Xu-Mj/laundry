/**
 * 简单的事件总线实现
 * 用于组件间通信
 */
class EventBus {
  constructor() {
    this.events = {};
  }

  /**
   * 订阅事件
   * @param {string} event 事件名称
   * @param {Function} callback 回调函数
   */
  on(event, callback) {
    if (!this.events[event]) {
      this.events[event] = [];
    }
    this.events[event].push(callback);
  }

  /**
   * 取消订阅事件
   * @param {string} event 事件名称
   * @param {Function} callback 回调函数
   */
  off(event, callback) {
    if (!this.events[event]) return;
    
    if (callback) {
      this.events[event] = this.events[event].filter(cb => cb !== callback);
    } else {
      delete this.events[event];
    }
  }

  /**
   * 触发事件
   * @param {string} event 事件名称
   * @param {any} args 传递给回调函数的参数
   */
  emit(event, ...args) {
    if (!this.events[event]) return;
    
    this.events[event].forEach(callback => {
      callback(...args);
    });
  }

  /**
   * 只订阅一次事件，触发后自动取消订阅
   * @param {string} event 事件名称
   * @param {Function} callback 回调函数
   */
  once(event, callback) {
    const onceCallback = (...args) => {
      callback(...args);
      this.off(event, onceCallback);
    };
    
    this.on(event, onceCallback);
  }
}

// 创建一个全局事件总线实例
const eventBus = new EventBus();

export default eventBus; 