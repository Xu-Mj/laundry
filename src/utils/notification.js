import { ElNotification } from 'element-plus';

const Notification = {
  success(message) {
    ElNotification({
      type: 'success',
      message
    });
  },
  warning(message) {
    ElNotification({
      type: 'warning',
      message
    });
  },
  info(message) {
    ElNotification({
      type: 'info',
      message
    });
  },
  error(message) {
    console.error(message);
    ElNotification({
      type: 'error',
      message
    });
  }
};

export default Notification;
