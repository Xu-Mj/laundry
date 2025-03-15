import { ElNotification } from 'element-plus';

const Notification = {
  success(message) {
    ElNotification({
      type: 'success',
      offset: 10,
      message
    });
  },
  warning(message) {
    ElNotification({
      type: 'warning',
      offset: 10,
      message
    });
  },
  info(message) {
    ElNotification({
      type: 'info',
      message,
      offset: 10,
    });
  },
  error(message) {
    console.error(message);
    ElNotification({
      type: 'error',
      message,
      offset: 10,
    });
  }
};

export default Notification;
