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
        <div class="refund-order-info">
          <div class="refund-info-item">
            <span class="refund-label">订单编号</span>
            <span class="refund-value">{{ refundForm.orderNumber }}</span>
          </div>
          <div class="refund-info-item">
            <span class="refund-label">退款状态</span>
            <el-tag size="small" type="warning" v-if="refundForm.unPay">订单未支付</el-tag>
            <el-tag size="small" type="success" v-else>可退款</el-tag>
          </div>
        </div>
        <div class="refund-divider"></div>
        <div class="refund-amount-info" v-if="!refundForm.unPay">
          <div class="refund-info-item">
            <span class="refund-label">订单金额</span>
            <span class="refund-original-amount">{{ refundForm.expAmount }}元</span>
          </div>
        </div>
      </div>

      <el-form ref="refundFormRef" :model="refundForm" :rules="refundRules" label-position="top" class="refund-form">
        <el-form-item label="退款账目" prop="expTitle">
          <el-input v-model="refundForm.expTitle" placeholder="请输入支出账目名称" class="refund-input">
            <template #prefix>
              <el-icon>
                <Document />
              </el-icon>
            </template>
          </el-input>
        </el-form-item>
        <el-form-item label="对方账户" prop="recvAccountTitle">
          <el-input v-model="refundForm.recvAccountTitle" disabled class="refund-input">
            <template #prefix>
              <el-icon>
                <User />
              </el-icon>
            </template>
          </el-input>
        </el-form-item>
        <el-form-item label="实退金额" prop="expAmount" v-if="!refundForm.unPay">
          <el-input-number v-model="refundForm.expAmount" :precision="2" :step="0.01" :min="0"
            :max="refundForm.expAmount" controls-position="right" style="width: 100%" class="refund-input-number">
            <template #prefix>
              <el-icon>
                <Money />
              </el-icon>
            </template>
          </el-input-number>
        </el-form-item>
        <el-form-item label="备注信息" prop="remark">
          <el-input type="textarea" v-model="refundForm.remark" placeholder="请输入退款原因或其他备注信息" :rows="3" resize="none"
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

const refundForm = ref({
  orderId: null,
  orderNumber: '',
  expTitle: '订单退款',
  expType: '00',
  recvAccount: null,
  recvAccountTitle: '',
  expAmount: 0,
  remark: '',
  unPay: false
});

const refundRules = {
  expTitle: [
    { required: true, message: '请输入支出账目名称', trigger: 'blur' }
  ],
  remark: [
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
  refundForm.value.recvAccount = props.userId;

  // 获取退款详情
  getRefundInfo(props.orderId, props.userId).then(res => {
    refundForm.value.recvAccountTitle = res.user.userName;
    if (res.payment) {
      // 已经支付了
      refundForm.value.unPay = false;
      if (res.payment.paymentAmountMv) {
        refundForm.value.expAmount = res.payment.paymentAmountMv;
      } else {
        refundForm.value.expAmount = res.payment.paymentAmount;
      }
    } else {
      // 没有支付
      refundForm.value.unPay = true;
    }
  }).catch(error => {
    proxy.$notify.error('获取退款信息失败：' + error);
    closeDialog();
  });
}

// 提交退款表单
function submitRefundForm() {
  refundFormRef.value.validate((valid) => {
    if (valid) {

      proxy.$modal.confirm('确认要退款吗？此操作不可逆！').then(() => {
        proxy.$modal.loading("退款中，请稍候");

        refund(refundForm.value).then(() => {
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
    expTitle: '订单退款',
    expType: '00',
    recvAccount: null,
    recvAccountTitle: '',
    expAmount: 0,
    remark: '',
    unPay: false
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
    refundForm.value.recvAccount = userId;
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

.refund-info-card {
  background-color: var(--el-fill-color-lighter);
  border-radius: 8px;
  padding: 16px;
}

.refund-order-info {
  display: flex;
  justify-content: space-between;
  margin-bottom: 16px;
}

.refund-info-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.refund-label {
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

.refund-value {
  font-size: 15px;
  font-weight: 500;
  color: var(--el-text-color-primary);
}

.refund-divider {
  height: 1px;
  background-color: var(--el-border-color-lighter);
  margin: 12px 0;
}

.refund-amount-info {
  display: flex;
  justify-content: flex-end;
}

.refund-original-amount {
  font-size: 18px;
  font-weight: 600;
  color: var(--el-color-danger);
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
  border-radius: 6px;
}

.refund-input-number :deep(.el-input__wrapper) {
  border-radius: 6px;
}

.refund-textarea {
  border-radius: 6px;
}

.refund-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}
</style>