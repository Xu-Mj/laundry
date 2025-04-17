<template>
  <el-dialog v-model="dialogVisible" width="600px" append-to-body lock-scroll modal :close-on-click-modal="false"
    :show-close="false" :align-center="true" class="subscription-payment-dialog">
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

    <!-- 订阅选项 -->
    <!-- <div class="section-title">订阅选项</div>
    <div class="subscription-options">
      <el-checkbox v-model="autoRenew" label="到期后自动续费" size="large" />
      <el-tooltip content="开启自动续费，系统将在订阅到期前自动为您续费，避免服务中断" placement="top">
        <el-icon class="info-icon">
          <InfoFilled />
        </el-icon>
      </el-tooltip>
    </div> -->

    <!-- 优惠码 -->
    <!-- <div class="section-title">优惠码</div>
    <div class="promo-code-section">
      <el-input v-model="promoCode" placeholder="请输入优惠码（选填）" clearable>
        <template #append>
          <el-button @click="applyPromoCode" :disabled="isLoading">应用</el-button>
        </template>
      </el-input>
      <div v-if="promoCodeApplied" class="promo-code-success">
        <el-icon><SuccessFilled /></el-icon>
        <span>优惠码已应用</span>
      </div>
    </div> -->
    <!-- 支付二维码区域 -->
    <div class="qrcode-section">
      <div class="qrcode-container">
        <div v-if="isLoading" class="loading-container">
          <el-icon class="loading-icon">
            <Loading />
          </el-icon>
          <span>正在生成支付二维码...</span>
        </div>
        <div v-else-if="paymentError" class="error-container">
          <el-icon class="error-icon">
            <WarningFilled />
          </el-icon>
          <span>{{ paymentErrorMsg }}</span>
          <el-button type="primary" size="small" @click="generateQrCode">重试</el-button>
        </div>
        <div v-else class="qrcode-wrapper">
          <div class="qrcode-image">
            <img :src="qrCodeUrl || 'https://placeholder.pics/svg/200x200/DEDEDE/555555/支付二维码'" alt="支付二维码" />
          </div>
          <div class="qrcode-status" v-if="currentPaymentStatus === PAYMENT_STATUS.SCANNED">
            <el-alert type="info" :closable="false">
              <template #title>
                <div class="status-alert">
                  <el-icon>
                    <Loading />
                  </el-icon>
                  <span>已扫码，请尽快完成支付</span>
                </div>
              </template>
            </el-alert>
          </div>
          <div class="qrcode-status" v-else-if="currentPaymentStatus === PAYMENT_STATUS.PROCESSING">
            <el-alert type="info" :closable="false">
              <template #title>
                <div class="status-alert">
                  <el-icon>
                    <Loading />
                  </el-icon>
                  <span>支付处理中，请稍候...</span>
                </div>
              </template>
            </el-alert>
          </div>
          <div class="qrcode-tip" v-else>
            请使用{{ paymentMethod === '01' ? '支付宝' : '微信' }}扫码支付
          </div>
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
        <el-button size="large" type="danger" @click="closeDialog">取消</el-button>
        <el-button size="large" type="primary" @click="checkPaymentStatus">已完成支付</el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref, watch, onMounted, onBeforeUnmount } from 'vue';
import { ElMessage } from 'element-plus';
import useUserStore from '@/store/modules/user';
import { createSubscriptionPaymentWithAlipay, querySubscriptionPaymentWithAlipay, querySubscriptionPaymentWithWechat, createSubscriptionPaymentWithWechat } from '@/api/system/subscriptionPayment';
import QRCode from 'qrcode';

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

const userStore = useUserStore();
const dialogVisible = ref(false);
const paymentMethod = ref('01'); // 默认微信支付
const isLoading = ref(false);
const qrCodeUrl = ref('');
const orderNumber = ref('');
const merchantName = ref('洗衣店管理系统');
const autoRenew = ref(true); // 默认自动续费
const promoCode = ref('');
const promoCodeApplied = ref(false);
const remark = ref('');
const paymentError = ref(false);
const paymentErrorMsg = ref('');
const subscriptionId = ref(null);
const paymentId = ref(null);
const paymentStatusTimer = ref(null);

// 监听visible属性变化
watch(() => props.visible, (newVal) => {
  dialogVisible.value = newVal;
  if (newVal) {
    // 当弹窗打开时，生成订单号并获取支付二维码
    // orderNumber.value = 'SUB' + Date.now();
    generateQrCode();
  } else {
    // 弹窗关闭时清除定时器
    clearPaymentStatusTimer();
  }
});

// 监听dialogVisible变化，同步更新父组件的visible属性
watch(dialogVisible, (newVal) => {
  emit('update:visible', newVal);
  if (!newVal) {
    clearPaymentStatusTimer();
  }
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

// 应用优惠码
const applyPromoCode = () => {
  if (!promoCode.value) {
    ElMessage.warning('请输入优惠码');
    return;
  }

  // 这里可以添加验证优惠码的逻辑
  // 模拟验证成功
  promoCodeApplied.value = true;
  ElMessage.success('优惠码应用成功');
};

// 生成支付二维码
const generateQrCode = async () => {
  isLoading.value = true;
  paymentError.value = false;
  // 重置支付状态
  currentPaymentStatus.value = PAYMENT_STATUS.WAITING;

  try {
    // 构建支付请求参数
    const paymentRequest = {
      storeId: userStore.id, // 使用当前用户ID作为商家ID
      planId: props.planData.id,
      autoRenew: autoRenew.value,
      promoCode: promoCodeApplied.value ? promoCode.value : undefined,
    };

    // 根据不同的支付方式生成不同的支付请求参数
    let res;
    if (paymentMethod.value === '01') {
      // 支付宝支付
      res = await createSubscriptionPaymentWithAlipay(paymentRequest);
    } else if (paymentMethod.value === '02') {
      // 微信支付
      res = await createSubscriptionPaymentWithWechat(paymentRequest);
    }

    if (res && res.success) {
      // 支付请求成功
      orderNumber.value = res.outTradeNo;
      subscriptionId.value = res.subscriptionId;
      paymentId.value = res.paymentId;

      // 使用qrcode.js将二维码字符串转换为图片URL
      if (res.qrCode) {
        try {
          // 生成二维码图片的DataURL
          qrCodeUrl.value = await QRCode.toDataURL(res.qrCode, {
            width: 200,
            margin: 2,
            color: {
              dark: '#000000',
              light: '#ffffff'
            }
          });

          // 立即查询一次支付状态，然后启动定时器
          await queryPaymentStatus();
          startPaymentStatusTimer();

        } catch (qrError) {
          console.error('二维码生成失败:', qrError);
          paymentError.value = true;
          paymentErrorMsg.value = '二维码生成失败，请重试';
          ElMessage.error(paymentErrorMsg.value);
          return;
        }
      } else {
        paymentError.value = true;
        paymentErrorMsg.value = '支付二维码数据为空，请重试';
        ElMessage.error(paymentErrorMsg.value);
        return;
      }
    } else {
      // 支付请求失败
      paymentError.value = true;
      paymentErrorMsg.value = res?.errorMessage || '生成支付二维码失败，请重试';
      ElMessage.error(paymentErrorMsg.value);
    }
  } catch (error) {
    console.error('支付请求异常:', error);
    paymentError.value = true;
    paymentErrorMsg.value = '网络异常，请重试';
    ElMessage.error(paymentErrorMsg.value);
  } finally {
    isLoading.value = false;
  }
};

// 启动支付状态查询定时器
const startPaymentStatusTimer = () => {
  // 先清除可能存在的定时器
  clearPaymentStatusTimer();

  // 每3秒查询一次支付状态
  paymentStatusTimer.value = setInterval(async () => {
    await queryPaymentStatus();
  }, 3000);
};

// 清除支付状态查询定时器
const clearPaymentStatusTimer = () => {
  if (paymentStatusTimer.value) {
    clearInterval(paymentStatusTimer.value);
    paymentStatusTimer.value = null;
  }
};

// 支付状态常量
const PAYMENT_STATUS = {
  WAITING: 'WAITING',           // 等待支付
  SCANNED: 'SCANNED',           // 已扫码
  PROCESSING: 'PROCESSING',     // 处理中
  SUCCESS: 'TRADE_SUCCESS',     // 支付成功
  FINISHED: 'TRADE_FINISHED',   // 交易完成
  CLOSED: 'TRADE_CLOSED',       // 交易关闭
  FAILED: 'TRADE_FAILED'        // 支付失败
};

// 当前支付状态
const currentPaymentStatus = ref(PAYMENT_STATUS.WAITING);

// 查询支付状态
const queryPaymentStatus = async () => {
  if (!orderNumber.value) return;

  try {
    const queryRequest = {
      storeId: userStore.id,
      outTradeNo: orderNumber.value
    };

    // 根据支付方式选择对应的查询API
    let res;
    if (paymentMethod.value === '01') {
      res = await querySubscriptionPaymentWithAlipay(queryRequest);
    } else if (paymentMethod.value === '02') {
      res = await querySubscriptionPaymentWithWechat(queryRequest);
    }

    if (res && res.success) {
      // 更新当前支付状态
      if (res.tradeStatus) {
        currentPaymentStatus.value = res.tradeStatus;
      }

      // 根据不同状态显示不同提示
      if (res.tradeStatus === PAYMENT_STATUS.SCANNED) {
        // 用户已扫码但未支付
        ElMessage.info('已扫码，请尽快完成支付');
      } else if (res.tradeStatus === PAYMENT_STATUS.PROCESSING) {
        // 支付处理中
        ElMessage.info('支付处理中，请稍候...');
      } else if (res.tradeStatus === PAYMENT_STATUS.SUCCESS || res.tradeStatus === PAYMENT_STATUS.FINISHED) {
        // 支付成功或交易完成
        clearPaymentStatusTimer();
        ElMessage.success('支付成功！');
        dialogVisible.value = false;
        emit('payment-success', {
          planId: props.planData.id,
          paymentMethod: paymentMethod.value,
          orderNumber: orderNumber.value,
          subscriptionId: subscriptionId.value,
          paymentId: paymentId.value
        });
      } else if (res.tradeStatus === PAYMENT_STATUS.CLOSED || res.tradeStatus === PAYMENT_STATUS.FAILED) {
        // 交易关闭或支付失败
        clearPaymentStatusTimer();
        ElMessage.error('支付失败或已取消');
        paymentError.value = true;
        paymentErrorMsg.value = '支付失败或已取消，请重试';
      }
    }
  } catch (error) {
    console.error('查询支付状态异常:', error);
  }
};

// 切换支付方式时重新生成二维码
watch(paymentMethod, () => {
  generateQrCode();
});

// 关闭弹窗
const closeDialog = () => {
  // 清除定时器
  clearPaymentStatusTimer();
  dialogVisible.value = false;
  emit('payment-cancel');
};

// 手动检查支付状态
const checkPaymentStatus = async () => {
  await queryPaymentStatus();
};

onMounted(() => {
  dialogVisible.value = props.visible;
});

onBeforeUnmount(() => {
  // 组件销毁前清除定时器
  clearPaymentStatusTimer();
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
  justify-content: center;
  align-items: center;
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
  border-radius: 4px;
  overflow: hidden;
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

.qrcode-status {
  margin-top: 8px;
  width: 100%;
}

.status-alert {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-alert .el-icon {
  animation: rotate 2s linear infinite;
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

.subscription-options {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 16px;
}

.info-icon {
  color: var(--el-color-info);
  cursor: pointer;
}

.promo-code-section {
  margin-bottom: 16px;
}

.promo-code-success {
  display: flex;
  align-items: center;
  gap: 4px;
  margin-top: 8px;
  color: var(--el-color-success);
  font-size: 14px;
}

.remark-section {
  margin-bottom: 16px;
}

.error-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 200px;
  width: 200px;
  gap: 12px;
}

.error-icon {
  font-size: 40px;
  color: var(--el-color-danger);
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