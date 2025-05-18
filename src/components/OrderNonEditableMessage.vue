<template>
  <div class="non-editable-container">
    <div class="non-editable-message">
      <div class="non-editable-icon">
        <el-icon v-if="order.paymentStatus === '00'" style="color:var(--el-color-success)"><CircleCheckFilled /></el-icon>
        <el-icon v-else-if="order.status === '06'" style="color:var(--el-color-danger)"><CircleCloseFilled /></el-icon>
      </div>
      <h2 class="non-editable-title">
        {{ order.paymentStatus === '00' ? '订单已支付' : order.status === '06' ? '订单已退单' : '订单不可编辑' }}
      </h2>
      <p class="non-editable-description">
        {{ order.paymentStatus === '00' ? '该订单已完成支付，不能修改订单信息' : order.status === '05' ? '该订单已办理退款，不能修改订单信息' : '当前状态不允许修改订单信息' }}
      </p>
      <div class="non-editable-order-info">
        <div class="order-info-item">
          <span class="info-label">订单编号</span>
          <span class="info-value">{{ order.orderNumber }}</span>
        </div>
        <div class="order-info-item">
          <span class="info-label">订单状态</span>
          <el-tag size="small" type="success" v-if="order.paymentStatus === '00'">已支付</el-tag>
          <el-tag size="small" type="danger" v-else-if="order.status === '06'">已退单</el-tag>
        </div>
        <div class="order-info-item">
          <span class="info-label">衣物数量</span>
          <span class="info-value">{{ totalClothes }} 件</span>
        </div>
        <div class="order-info-item">
          <span class="info-label">订单金额</span>
          <span class="info-value price">{{ totalPrice }} 元</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { CircleCheckFilled, CircleCloseFilled } from '@element-plus/icons-vue';

defineProps({
  order: {
    type: Object,
    required: true
  },
  totalClothes: {
    type: Number,
    default: 0
  },
  totalPrice: {
    type: Number,
    default: 0
  }
});
</script>

<style scoped>
.non-editable-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  background-color: var(--el-bg-color-page);
  overflow: auto;
  padding: 1rem;
}

.non-editable-message {
  background-color: var(--el-bg-color);
  border-radius: 12px;
  box-shadow: var(--el-box-shadow-light);
  padding: 2rem;
  text-align: center;
  width: 100%;
  max-width: 600px;
  animation: fadeIn 0.5s ease-in-out;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(20px); }
  to { opacity: 1; transform: translateY(0); }
}

.non-editable-icon {
  font-size: 64px;
  display: flex;
  justify-content: center;
  align-items: center;
  margin-bottom: 1.5rem;
}

.non-editable-icon .el-icon {
  border-radius: 50%;
  padding: 8px;
}

.non-editable-icon :deep(svg) {
  filter: drop-shadow(0 4px 8px rgba(0, 0, 0, 0.1));
}

.non-editable-title {
  font-size: 24px;
  font-weight: bold;
  margin-bottom: 1rem;
  color: var(--el-text-color-primary);
}

.non-editable-description {
  font-size: 16px;
  margin-bottom: 2rem;
  color: var(--el-text-color-secondary);
  line-height: 1.5;
}

.non-editable-order-info {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1.5rem;
  padding-top: 1.5rem;
  border-top: 1px solid var(--el-border-color-lighter);
}

.order-info-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  padding: 1rem;
  background-color: var(--el-fill-color-lighter);
  border-radius: 8px;
  transition: all 0.3s ease;
}

.order-info-item:hover {
  transform: translateY(-5px);
  box-shadow: var(--el-box-shadow-light);
}

.info-label {
  font-size: 14px;
  color: var(--el-text-color-secondary);
}

.info-value {
  font-size: 18px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.price {
  font-size: 24px;
  font-weight: bold;
  color: var(--el-color-danger);
}
</style> 