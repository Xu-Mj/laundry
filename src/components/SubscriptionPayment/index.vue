<template>
  <el-dialog v-model="dialogVisible" width="600px" append-to-body lock-scroll modal :close-on-click-modal="false"
    :show-close="false" class="subscription-payment-dialog">
    <template #header>
      <div class="dialog-header">
        <div class="order-info">
          <el-icon>
            <Ticket />
          </el-icon>
          <span>订阅套餐 - {{ planData.name }}</span>
        </div>
        <el-button circle size="small" @click="closeDialog">
          <el-icon>
            <Close />
          </el-icon>
        </el-button>
      </div>
    </template>

    <!-- 套餐信息卡片 -->
    <div class="plan-card">
      <div class="plan-header">
        <div class="plan-name">{{ planData.name }}</div>
        <el-tag :type="getTagType(planData.planType)" effect="light" size="small">
          {{ getTypeName(planData.planType) }}
        </el-tag>
      </div>
      <div class="plan-price">
        <span class="price">¥{{ planData.price }}</span>
        <span class="period">/ {{ getPeriodText(planData.period) }}</span>
      </div>
      <div class="plan-features" v-if="planData.features">
        <div v-for="(feature, index) in getFeaturesList(planData.features)" :key="index" class="feature-item">
          <el-icon>
            <Check />
          </el-icon>
          <span>{{ feature }}</span>
        </div>
      </div>
    </div>

    <!-- 支付方式选择 -->
    <div class="section-title">支付方式</div>
    <div class="payment-method-section">
      <el-radio-group v-model="paymentMethod" class="payment-method-group">
        <el-radio value="02" class="payment-method-radio">
          <div class="payment-method-card" :class="{ 'selected': paymentMethod === '02' }">
            <el-icon>
              <ChatDotRound />
            </el-icon>
            <span>微信支付</span>
          </div>
        </el-radio>
        <el-radio value="01" class="payment-method-radio">
          <div class="payment-method-card" :class="{ 'selected': paymentMethod === '01' }">
            <el-icon>
              <Money />
            </el-icon>
            <span>支付宝</span>
          </div>
        </el-radio>
      </el-radio-group>
    </div>

    <!-- 支付二维码区域 -->
    <div class="qrcode-section">
      <div class="qrcode-container">
        <div v-if="isLoading" class="loading-container">
          <el-icon class="loading-icon">
            <Loading />
          </el-icon>
          <span>正在生成支付二维码...</span>
        </div>
        <div v-else class="qrcode-wrapper">
          <div class="qrcode-image">
            <!-- 这里使用一个占位图，实际项目中应该替换为真实的二维码 -->
            <img :src="qrCodeUrl || 'https://placeholder.pics/svg/200x200/DEDEDE/555555/支付二维码'" alt="支付二维码" />
          </div>
          <div class="qrcode-tip">请使用{{ paymentMethod === '01' ? '支付宝' : '微信' }}扫码支付</div>
        </div>
      </div>
      <div class="payment-info">
        <div class="payment-row">
          <span class="payment-label">订单编号:</span>
          <span class="payment-value">{{ orderNumber }}</span>
        </div>
        <div class="payment-row">
          <span class="payment-label">收款方:</span>
          <span class="payment-value">{{ merchantName }}</span>
        </div>
        <div class="payment-row">
          <span class="payment-label">支付金额:</span>
          <span class="payment-value price">¥{{ planData.price }}</span>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="payment-footer">
        <el-button size="large" @click="closeDialog" plain>取消</el-button>
        <el-button size="large" type="primary" @click="checkPaymentStatus">已完成支付</el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref, watch, onMounted } from 'vue';
import { ElMessage } from 'element-plus';
import { Check, Ticket, Money, ChatDotRound, Close, Loading } from '@element-plus/icons-vue';

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
  }
});

const emit = defineEmits(['update:visible', 'payment-success', 'payment-cancel']);

const dialogVisible = ref(false);
const paymentMethod = ref('02'); // 默认微信支付
const isLoading = ref(true);
const qrCodeUrl = ref('');
const orderNumber = ref('SUB' + Date.now());
const merchantName = ref('洗衣店管理系统');

// 监听visible属性变化
watch(() => props.visible, (newVal) => {
  dialogVisible.value = newVal;
  if (newVal) {
    // 当弹窗打开时，模拟获取支付二维码
    generateQrCode();
  }
});

// 监听dialogVisible变化，同步更新父组件的visible属性
watch(dialogVisible, (newVal) => {
  emit('update:visible', newVal);
});

// 获取套餐类型名称
const getTypeName = (type) => {
  const typeMap = {
    'basic': '基础版',
    'standard': '标准版',
    'premium': '高级版',
    'enterprise': '企业版'
  };
  return typeMap[type] || '未知';
};

// 获取标签类型
const getTagType = (type) => {
  const typeMap = {
    'basic': 'info',
    'standard': 'success',
    'premium': 'warning',
    'enterprise': 'danger'
  };
  return typeMap[type] || 'info';
};

// 获取周期文本
const getPeriodText = (period) => {
  const periodMap = {
    'Monthly': '月',
    'Quarterly': '季度',
    'HalfYearly': '半年',
    'Yearly': '年'
  }
  return periodMap[period] || period
}

// 解析功能列表
const getFeaturesList = (features) => {
  if (typeof features === 'string') {
    try {
      return JSON.parse(features);
    } catch (e) {
      return features.split(',');
    }
  }
  return Array.isArray(features) ? features : [];
};

// 模拟生成支付二维码
const generateQrCode = () => {
  isLoading.value = true;
  // 模拟API请求延迟
  setTimeout(() => {
    // 实际项目中，这里应该调用后端API获取支付二维码
    // qrCodeUrl.value = 后端返回的二维码URL
    isLoading.value = false;
  }, 1500);
};

// 切换支付方式时重新生成二维码
watch(paymentMethod, () => {
  generateQrCode();
});

// 关闭弹窗
const closeDialog = () => {
  dialogVisible.value = false;
  emit('payment-cancel');
};

// 检查支付状态
const checkPaymentStatus = () => {
  // 实际项目中，这里应该调用后端API检查支付状态
  // 模拟支付成功
  ElMessage.success('支付成功！');
  dialogVisible.value = false;
  emit('payment-success', {
    planId: props.planData.id,
    paymentMethod: paymentMethod.value,
    orderNumber: orderNumber.value
  });
};

onMounted(() => {
  dialogVisible.value = props.visible;
});
</script>

<style scoped>
.subscription-payment-dialog {
  border-radius: 12px;
  overflow: hidden;
  background-color: var(--el-bg-color-page);
}

.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 16px;
}

.order-info {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 18px;
  font-weight: 600;
  color: var(--el-color-primary);
}

.plan-card {
  background: linear-gradient(135deg, var(--el-color-primary-light-9) 0%, var(--el-color-primary-light-8) 100%);
  border-radius: 12px;
  padding: 1rem;
  margin-bottom: 24px;
  box-shadow: var(--el-box-shadow-light);
}

:root.dark .plan-card {
  --el-color-primary-light-9: #1d2c40;
  --el-color-primary-light-8: #2b6095;
}

.plan-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.plan-name {
  font-size: 20px;
  font-weight: 600;
  color: var(--el-color-primary-dark-2);
}

.plan-price {
  margin-bottom: 16px;
}

.price {
  font-size: 24px;
  font-weight: 700;
  color: var(--el-color-danger);
}

.period {
  font-size: 14px;
  color: var(--el-text-color-secondary);
}

.plan-features {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.feature-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.feature-item .el-icon {
  color: var(--el-color-success);
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  margin: 16px 0 12px 0;
  color: var(--el-text-color-primary);
}

.payment-method-section {
  margin-bottom: 24px;
}

.payment-method-group {
  display: flex;
  gap: 1rem;
}

.payment-method-radio {
  margin-right: 0 !important;
  height: auto;
}

.payment-method-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100px;
  height: 80px;
  border-radius: 8px;
  border: 1px solid var(--el-border-color);
  transition: all 0.3s;
  cursor: pointer;
  background-color: var(--el-bg-color-overlay);
}

.payment-method-card:hover {
  border-color: var(--el-color-primary);
  transform: translateY(-2px);
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
}

.payment-method-card.selected {
  border-color: var(--el-color-primary);
  background-color: var(--el-fill-color-light);
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
}

.payment-method-card .el-icon {
  font-size: 24px;
  margin-bottom: 8px;
  color: var(--el-color-primary);
}

.qrcode-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  background-color: var(--el-fill-color-blank);
  border-radius: 8px;
  padding: 1rem;
  margin-bottom: 1rem;
  border: 1px dashed var(--el-border-color);
}

.qrcode-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-bottom: 1rem;
}

.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 200px;
  width: 200px;
}

.loading-icon {
  font-size: 40px;
  color: var(--el-color-primary);
  animation: rotate 2s linear infinite;
}

@keyframes rotate {
  from {
    transform: rotate(0deg);
  }

  to {
    transform: rotate(360deg);
  }
}

.qrcode-wrapper {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.qrcode-image {
  width: 200px;
  height: 200px;
  margin-bottom: 8px;
}

.qrcode-image img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.qrcode-tip {
  font-size: 14px;
  color: var(--el-text-color-secondary);
  margin-top: 8px;
}

.payment-info {
  width: 100%;
  border-top: 1px solid var(--el-border-color-lighter);
  padding-top: 1rem;
}

.payment-row {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;
}

.payment-label {
  color: var(--el-text-color-secondary);
  font-size: 14px;
}

.payment-value {
  font-weight: 500;
  font-size: 14px;
}

.payment-value.price {
  color: var(--el-color-danger);
  font-weight: 700;
}

.payment-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;

  button {
    transition: all .3s;
  }

  button:hover {
    transform: translateY(-2px);
  }
}

/* 响应式调整 */
@media screen and (max-width: 768px) {
  .payment-method-group {
    flex-direction: column;
  }

  .qrcode-section {
    padding: 15px;
  }

  .payment-footer {
    flex-direction: column;
    gap: 8px;
  }

  .payment-footer .el-button {
    width: 100%;
  }

  .plan-card {
    padding: 15px;
  }
}

/* 动画效果 */
.el-dialog-fade-enter-active {
  animation: dialog-fade-in 0.3s;
}

.el-dialog-fade-leave-active {
  animation: dialog-fade-out 0.3s;
}

@keyframes dialog-fade-in {
  0% {
    transform: translate3d(0, -20px, 0);
    opacity: 0;
  }

  100% {
    transform: translate3d(0, 0, 0);
    opacity: 1;
  }
}

@keyframes dialog-fade-out {
  0% {
    transform: translate3d(0, 0, 0);
    opacity: 1;
  }

  100% {
    transform: translate3d(0, -20px, 0);
    opacity: 0;
  }
}
</style>