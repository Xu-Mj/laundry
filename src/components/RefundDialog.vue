<template>
  <el-dialog v-model="localVisible" width="500px" :align-center="true" :show-close="true" destroy-on-close
    class="refund-dialog" top="10vh">
    <template #header>
      <div class="refund-dialog-header">
        <el-icon>
          <Wallet />
        </el-icon>
        <span>订单退款</span>
      </div>
    </template>
    <div class="refund-dialog-content">
      <div class="refund-info-card">
        <!-- 订单基本信息 -->
        <div class="refund-card-header">
          <div class="order-number">
            <div class="label">订单编号</div>
            <div class="value">{{ refundForm.orderNumber }}</div>
          </div>
          <div class="status-tag">
            <el-tag size="small" type="warning" v-if="refundForm.unPay">订单未支付</el-tag>
            <el-tag size="small" type="success" v-else>可退款</el-tag>
          </div>
        </div>

        <!-- 订单总金额 -->
        <div class="total-amount-section" v-if="!refundForm.unPay">
          <div class="label">订单总金额</div>
          <div class="amount">{{ paymentDetails.totalAmount }}元</div>
        </div>

        <!-- 支付明细 -->
        <div v-if="!refundForm.unPay">
          <div class="section-header">
            <div class="section-title">支付明细</div>
          </div>

          <div class="payment-details">
            <!-- 储值卡 -->
            <div class="payment-item" v-if="paymentDetails.cardAmount > 0">
              <div class="payment-method">
                <el-icon>
                  <CreditCard />
                </el-icon>
                <span>储值卡支付</span>
              </div>
              <div class="payment-amount">{{ paymentDetails.cardAmount }}元</div>
            </div>

            <!-- 次卡 -->
            <div class="payment-item" v-if="paymentDetails.hasTimeCard">
              <div class="payment-method">
                <el-icon>
                  <Tickets />
                </el-icon>
                <span>{{ paymentDetails.timeCardName || '次卡抵扣' }}</span>
              </div>
              <div class="payment-amount">{{ paymentDetails.timeCardCount }}次</div>
            </div>

            <!-- 优惠券 -->
            <div class="payment-item" v-if="paymentDetails.couponAmount > 0">
              <div class="payment-method">
                <el-icon>
                  <Discount />
                </el-icon>
                <span>{{ paymentDetails.couponName || (paymentDetails.couponType + '抵扣') || '优惠券抵扣' }}</span>
              </div>
              <div class="payment-amount">{{ paymentDetails.couponAmount }}元</div>
            </div>

            <!-- 实际支付 -->
            <div class="payment-item highlight" v-if="paymentDetails.actualPayAmount > 0">
              <div class="payment-method">
                <el-icon>
                  <component :is="getPaymentIcon()" />
                </el-icon>
                <span>{{ getPaymentMethodName() }}支付</span>
              </div>
              <div class="payment-amount">{{ paymentDetails.actualPayAmount }}元</div>
            </div>
          </div>
        </div>

        <!-- 退款明细 -->
        <div v-if="!refundForm.unPay">
          <div class="section-header">
            <div class="section-title">退款明细</div>
          </div>

          <div class="payment-details">
            <!-- 储值卡 -->
            <div class="payment-item" v-if="paymentDetails.cardAmount > 0">
              <div class="payment-method">
                <el-icon>
                  <CreditCard />
                </el-icon>
                <span>退回储值卡</span>
              </div>
              <div class="payment-amount">{{ paymentDetails.cardAmount }}元</div>
            </div>

            <!-- 次卡 -->
            <div class="payment-item" v-if="paymentDetails.hasTimeCard">
              <div class="payment-method">
                <el-icon>
                  <Tickets />
                </el-icon>
                <span>退回次卡{{ paymentDetails.timeCardName || '次卡' }}</span>
              </div>
              <div class="payment-amount">{{ paymentDetails.timeCardCount }}次</div>
            </div>

            <!-- 优惠券 -->
            <div class="payment-item" v-if="paymentDetails.couponAmount > 0">
              <div class="payment-method">
                <el-icon>
                  <Discount />
                </el-icon>
                <span>退回优惠券{{ paymentDetails.couponName || paymentDetails.couponType || '优惠券' }}</span>
              </div>
              <div class="payment-amount">{{ paymentDetails.couponAmount }}元</div>
            </div>

            <!-- 实际退款 -->
            <div class="payment-item highlight" v-if="paymentDetails.actualPayAmount > 0">
              <div class="payment-method">
                <el-icon>
                  <Money />
                </el-icon>
                <span>实际退款金额</span>
              </div>
              <div class="payment-amount accent">{{ refundForm.expAmount }}元</div>
            </div>
          </div>
        </div>
      </div>

      <el-form ref="refundFormRef" :model="refundForm" :rules="refundRules" label-position="top" class="refund-form">
        <el-form-item label="客户信息" prop="customerInfo">
          <el-input v-model="refundForm.customerInfo" disabled class="refund-input">
            <template #prefix>
              <el-icon>
                <User />
              </el-icon>
            </template>
          </el-input>
        </el-form-item>
        <el-form-item label="实退金额" prop="expAmount" v-if="!refundForm.unPay && paymentDetails.actualPayAmount > 0">
          <el-input-number v-model="refundForm.expAmount" :precision="2" :step="0.01" :min="0"
            :max="paymentDetails.actualPayAmount" controls-position="right" style="width: 100%"
            class="refund-input-number" disabled>
            <template #prefix>
              <el-icon>
                <Money />
              </el-icon>
            </template>
          </el-input-number>
        </el-form-item>
        <el-form-item label="退款原因" prop="refundReason">
          <el-input type="textarea" v-model="refundForm.refundReason" placeholder="请输入退款原因" :rows="3" resize="none"
            class="refund-textarea" />
        </el-form-item>
      </el-form>
    </div>

    <template #footer>
      <div class="refund-footer">
        <el-button @click="closeDialog" plain>取消</el-button>
        <el-button type="primary" @click="submitRefundForm" :disabled="refundForm.unPay">
          <el-icon>
            <Check />
          </el-icon>确认退款
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup>
import { getRefundInfo, refund } from "@/api/system/orders";

const props = defineProps({
  visible: {
    type: Boolean,
    default: false
  },
  orderId: {
    type: Number,
    default: null
  },
  userId: {
    type: Number,
    default: null
  },
  orderNumber: {
    type: String,
    default: ''
  }
});

const emit = defineEmits(['update:visible', 'refund-success', 'refund-cancel']);
const { proxy } = getCurrentInstance();

const localVisible = ref(props.visible);
const refundFormRef = ref(null);

// 支付详情
const paymentDetails = ref({
  totalAmount: 0,          // 订单总金额
  cardAmount: 0,           // 储值卡支付金额
  couponAmount: 0,         // 优惠券抵扣金额
  actualPayAmount: 0,      // 实际支付金额
  paymentMethod: '',       // 支付方式
  hasTimeCard: false,      // 是否使用了次卡
  timeCardCount: 0,        // 次卡使用次数
  timeCardName: '',        // 次卡名称
  couponType: '',          // 优惠券类型
  couponName: ''           // 优惠券名称
});

const refundForm = ref({
  orderId: null,
  orderNumber: '',
  customerInfo: '',
  expAmount: 0,
  refundReason: '',
  unPay: false
});

const refundRules = {
  refundReason: [
    { required: true, message: '请输入退款原因', trigger: 'blur' }
  ]
};

// 监听props变化
watch(() => props.visible, (newVal) => {
  localVisible.value = newVal;
  if (newVal && props.orderId) {
    loadRefundInfo();
  }
});

watch(() => localVisible.value, (newVal) => {
  emit('update:visible', newVal);
});

watch(() => props.orderId, (newVal) => {
  if (newVal && localVisible.value) {
    loadRefundInfo();
  }
});

// 加载退款信息
function loadRefundInfo() {
  // 重置表单
  resetForm();

  // 设置基本信息
  refundForm.value.orderId = props.orderId;
  refundForm.value.orderNumber = props.orderNumber;

  // 获取退款详情
  getRefundInfo(props.orderId, props.userId).then(res => {
    console.log(res)
    refundForm.value.customerInfo = res.user.nickName || res.user.userName;

    if (res.payment) {
      // 已经支付了
      refundForm.value.unPay = false;
      const payment = res.payment;

      // 设置支付详情
      paymentDetails.value.totalAmount = payment.totalAmount || 0;
      paymentDetails.value.paymentMethod = payment.paymentMethod || '';

      const method = payment.paymentMethod;

      // 解析支付方式
      // 组合支付：支付方式 + 储值卡
      if (['16', '26', '56'].includes(method)) {
        // 组合支付: 支付宝/微信/现金 + 储值卡
        paymentDetails.value.cardAmount = payment.paymentAmountVip || 0;
        paymentDetails.value.actualPayAmount = payment.paymentAmountMv || 0;
      }
      // 微信+优惠券组合
      else if (method === '28' || method === '29') {
        paymentDetails.value.actualPayAmount = payment.paymentAmount || 0;
        paymentDetails.value.couponAmount = payment.totalAmount - payment.paymentAmount || 0;

        // 设置优惠券类型
        if (method === '28') {
          paymentDetails.value.couponType = '折扣券';
        } else if (method === '29') {
          paymentDetails.value.couponType = '满减券';
        }
      }
      // 单独的储值卡支付
      else if (method === '06') {
        paymentDetails.value.cardAmount = payment.paymentAmount || 0;
        paymentDetails.value.actualPayAmount = 0;
      }
      // 次卡支付
      else if (method === '07') {
        paymentDetails.value.hasTimeCard = true;
        paymentDetails.value.timeCardCount = payment.paymentAmountVip || 1;
        paymentDetails.value.actualPayAmount = 0;
      }
      // 普通支付
      else {
        paymentDetails.value.actualPayAmount = payment.paymentAmount || 0;
      }

      // 处理优惠券
      if (payment.ucId && res.userCoupons && res.userCoupons.length > 0) {
        // 遍历所有使用的优惠券
        res.userCoupons.forEach(uc => {
          const couponType = uc.coupon?.couponType;

          // 处理次卡 (004或002都是次卡类型)
          if ( couponType === '002') {
            paymentDetails.value.hasTimeCard = true;
            paymentDetails.value.timeCardCount = payment.paymentAmountVip || 1;
            paymentDetails.value.timeCardName = uc.coupon?.couponTitle;
          }
          // 处理满减券
          else if (couponType === '004') {
            paymentDetails.value.couponAmount = uc.coupon?.couponValue || payment.totalAmount - payment.paymentAmount || 0;
            paymentDetails.value.couponType = '满减券';
            paymentDetails.value.couponName = uc.coupon?.couponTitle;
          }
          // 处理折扣券
          else if (couponType === '003') {
            paymentDetails.value.couponAmount = payment.totalAmount - payment.paymentAmount || 0;
            paymentDetails.value.couponType = '折扣券';
            paymentDetails.value.couponName = uc.coupon?.couponTitle;
          }
        });
      }

      // 设置实际退款金额（默认为实际支付金额）
      refundForm.value.expAmount = paymentDetails.value.actualPayAmount;
    } else {
      // 没有支付
      refundForm.value.unPay = true;
    }
  }).catch(error => {
    proxy.$notify.error('获取退款信息失败：' + error);
    closeDialog();
  });
}

// 获取支付方式图标
function getPaymentIcon() {
  const method = paymentDetails.value.paymentMethod;

  // 基础支付方式
  if (method === '01' || method === '16' || method === '18' || method === '19') {
    return 'Alipay';
  } else if (method === '02' || method === '26' || method === '28' || method === '29') {
    return 'ChatDotSquare'; // 微信图标
  } else if (method === '05' || method === '56' || method === '58' || method === '59') {
    return 'Money'; // 现金图标
  } else if (method === '03') {
    return 'Dessert'; // 美团图标
  } else if (method === '04') {
    return 'VideoPlay'; // 抖音图标
  } else if (method === '07') {
    return 'Tickets'; // 次卡图标
  } else if (method === '06') {
    return 'CreditCard'; // 储值卡图标
  }

  return 'Money';
}

// 获取支付方式名称
function getPaymentMethodName() {
  const method = paymentDetails.value.paymentMethod;

  // 基础支付方式
  if (method === '01') {
    return '支付宝';
  } else if (method === '02') {
    return '微信';
  } else if (method === '05') {
    return '现金';
  } else if (method === '03') {
    return '美团';
  } else if (method === '04') {
    return '抖音';
  } else if (method === '06') {
    return '储值卡';
  } else if (method === '07') {
    return '次卡';
  }

  // 组合支付方式
  else if (method === '16') {
    return '支付宝';
  } else if (method === '26') {
    return '微信';
  } else if (method === '56') {
    return '现金';
  } else if (method === '28') {
    return '微信';
  } else if (method === '18') {
    return '支付宝';
  } else if (method === '58') {
    return '现金';
  } else if (method === '19') {
    return '支付宝'
  } else if (method === '29') {
    return '微信';
  } else if (method === '59') {
    return '现金';
  }

  return '其他';
}

// 提交退款表单
function submitRefundForm() {
  refundFormRef.value.validate((valid) => {
    if (valid) {
      proxy.$modal.confirm('确认要退款吗？此操作不可逆！').then(() => {
        proxy.$modal.loading("退款中，请稍候");

        // 使用新的API接口
        refund(refundForm.value.orderId, refundForm.value.refundReason).then(() => {
          proxy.$modal.closeLoading();
          proxy.$notify.success("退款成功");
          emit('refund-success');
          closeDialog();
        }).catch(error => {
          proxy.$modal.closeLoading();
          proxy.$notify.error("退款失败");
        });
      }).catch(() => {
        // 用户取消操作
      });
    }
  });
}

// 关闭对话框
function closeDialog() {
  localVisible.value = false;
  emit('refund-cancel');
}

// 重置表单
function resetForm() {
  refundForm.value = {
    orderId: null,
    orderNumber: '',
    customerInfo: '',
    expAmount: 0,
    refundReason: '',
    unPay: false
  };

  // 重置支付详情
  paymentDetails.value = {
    totalAmount: 0,
    cardAmount: 0,
    couponAmount: 0,
    actualPayAmount: 0,
    paymentMethod: '',
    hasTimeCard: false,
    timeCardCount: 0,
    timeCardName: '',
    couponType: '',
    couponName: ''
  };

  // 如果表单ref已存在，则重置验证
  if (refundFormRef.value) {
    refundFormRef.value.resetFields();
  }
}

// 对外暴露方法
defineExpose({
  openDialog: (orderId, userId, orderNumber) => {
    refundForm.value.orderId = orderId;
    refundForm.value.orderNumber = orderNumber;
    localVisible.value = true;
    loadRefundInfo();
  }
});
</script>

<style scoped>
/* 退单弹窗样式 */
.refund-dialog :deep(.el-dialog__header) {
  margin: 0;
  padding: 20px 24px;
  border-bottom: 1px solid var(--el-border-color-lighter);
}

.refund-dialog :deep(.el-dialog__body) {
  padding: 24px;
}

.refund-dialog :deep(.el-dialog__footer) {
  padding: 16px 24px;
  border-top: 1px solid var(--el-border-color-lighter);
}

.refund-dialog-header {
  display: flex;
  align-items: center;
  gap: 10px;
  color: var(--el-color-primary);
  font-size: 18px;
  font-weight: 600;
}

.refund-dialog-header .el-icon {
  font-size: 20px;
}

.refund-dialog-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

/* 重新设计的订单信息卡片 */
.refund-info-card {
  background-color: var(--el-fill-color-lighter);
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.02);
}

/* 订单头部信息 */
.refund-card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 20px;
}

.order-number .label {
  font-size: 12px;
  color: var(--el-text-color-secondary);
  margin-bottom: 4px;
}

.order-number .value {
  font-size: 16px;
  font-weight: 500;
  color: var(--el-text-color-primary);
}

/* 订单总金额部分 */
.total-amount-section {
  background-color: #fafafa;
  border-radius: 8px;
  padding: 12px 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.total-amount-section .label {
  font-size: 14px;
  color: var(--el-text-color-secondary);
}

.total-amount-section .amount {
  font-size: 18px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

/* 支付和退款明细部分 */
.section-header {
  display: flex;
  align-items: center;
  margin-bottom: 12px;
  padding-top: 4px;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--el-text-color-primary);
  position: relative;
  padding-left: 12px;
}

.section-title::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 4px;
  height: 14px;
  background-color: var(--el-color-primary);
  border-radius: 2px;
}

.payment-details {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding-bottom: 16px;
}

.payment-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 12px;
  background-color: white;
  border-radius: 8px;
  transition: all 0.2s;
}

.payment-item:hover {
  background-color: #f8f8f8;
}

.payment-item.highlight {
  background-color: #f0f9ff;
  border-left: 2px solid var(--el-color-primary);
}

.payment-method {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--el-text-color-regular);
}

.payment-method .el-icon {
  font-size: 16px;
  color: var(--el-text-color-secondary);
}

.payment-amount {
  font-size: 15px;
  font-weight: 500;
  color: var(--el-text-color-primary);
}

.payment-amount.accent {
  color: var(--el-color-danger);
  font-weight: 600;
}

.refund-form {
  margin-top: 8px;
}

.refund-form :deep(.el-form-item__label) {
  padding-bottom: 8px;
  font-weight: 500;
  color: var(--el-text-color-primary);
}

.refund-input {
  border-radius: 8px;
}

.refund-input-number :deep(.el-input__wrapper) {
  border-radius: 8px;
}

.refund-textarea {
  border-radius: 8px;
}

.refund-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}
</style>