<template>
  <div class="subscription-congrats-container" v-if="dialogVisible">
    <div class="kaleidoscope-modal">
      <div class="kaleidoscope-effect">
        <div class="kaleidoscope-pattern"></div>
      </div>
      <div class="congrats-content">
        <!-- 简化的标题 -->
        <h2 class="congrats-title">订阅成功</h2>
        
        <!-- 简化的套餐信息 -->
        <div class="plan-info">
          <div class="plan-name">{{ planData.name }}</div>
          <div class="plan-price">¥{{ planData.price }} / {{ getPeriodText(planData.period) }}</div>
        </div>

        <!-- 按钮区域 -->
        <div class="action-buttons">
          <button class="confirm-button" @click="closeDialog">开始使用</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue';

const props = defineProps({
  visible: {
    type: Boolean,
    required: true,
    default: false
  },
  planData: {
    type: Object,
    required: true,
    default: () => ({})
  },
  expiryDate: {
    type: [Date, Number, String],
    required: false,
    default: () => Date.now() + 30 * 24 * 60 * 60 * 1000 // 默认30天
  }
});

const emit = defineEmits(['update:visible', 'confirmed']);

const dialogVisible = ref(false);

// 监听visible属性变化
watch(() => props.visible, (newVal) => {
  dialogVisible.value = newVal;
});

// 监听dialogVisible变化，同步更新父组件的visible属性
watch(dialogVisible, (newVal) => {
  emit('update:visible', newVal);
});

// 关闭弹窗
const closeDialog = () => {
  dialogVisible.value = false;
  emit('confirmed');
};

// 获取周期文本
const getPeriodText = (period) => {
  const periodMap = {
    'month': '月',
    'quarter': '季度',
    'half_year': '半年',
    'year': '年'
  };
  return periodMap[period] || '月';
};
</script>

<style scoped>
.subscription-congrats-container {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 9999;
  background-color: rgba(0, 0, 0, 0.5);
  animation: modalFadeIn 0.2s ease-out forwards;
}

.kaleidoscope-modal {
  position: relative;
  width: 400px;
  height: 300px; /* 4:3 宽高比 */
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
  animation: modalScaleIn 0.2s ease-out forwards;
}

.kaleidoscope-effect {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, #6e8efb, #a777e3);
  opacity: 0.8;
  z-index: 1;
}

.kaleidoscope-pattern {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: conic-gradient(
    from 0deg,
    rgba(255, 255, 255, 0.2) 0%,
    rgba(255, 255, 255, 0.1) 25%,
    rgba(255, 255, 255, 0.2) 50%,
    rgba(255, 255, 255, 0.1) 75%,
    rgba(255, 255, 255, 0.2) 100%
  );
  animation: rotate 20s linear infinite, scale 15s ease-in-out infinite alternate;
  z-index: 2;
}

@keyframes rotate {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

@keyframes scale {
  0% {
    transform: scale(1) rotate(0deg);
  }
  50% {
    transform: scale(1.5) rotate(180deg);
  }
  100% {
    transform: scale(1) rotate(360deg);
  }
}

.congrats-content {
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
  padding: 2rem;
  z-index: 3;
  text-align: center;
  backdrop-filter: blur(5px);
}

/* 标题 */
.congrats-title {
  font-size: 28px;
  font-weight: 600;
  color: white;
  margin: 0 0 1.5rem;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  animation: fadeIn 0.8s ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 套餐信息 */
.plan-info {
  margin-bottom: 2rem;
  animation: fadeIn 1s ease 0.2s both;
}

.plan-name {
  font-size: 20px;
  font-weight: 600;
  color: white;
  margin-bottom: 0.5rem;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
}

.plan-price {
  font-size: 18px;
  color: rgba(255, 255, 255, 0.9);
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
}

/* 按钮区域 */
.action-buttons {
  margin-top: 1rem;
}

.confirm-button {
  min-width: 160px;
  padding: 10px 20px;
  border-radius: 24px;
  font-size: 16px;
  font-weight: 500;
  background: rgba(255, 255, 255, 0.2);
  color: white;
  border: 2px solid rgba(255, 255, 255, 0.4);
  cursor: pointer;
  transition: all 0.3s ease;
  animation: fadeIn 1s ease 0.4s both;
  backdrop-filter: blur(5px);
}

.confirm-button:hover {
  background: rgba(255, 255, 255, 0.3);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}
@keyframes modalFadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes modalScaleIn {
  from {
    transform: scale(0.8);
    opacity: 0;
  }
  to {
    transform: scale(1);
    opacity: 1;
  }
}
</style>