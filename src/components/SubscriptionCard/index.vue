<template>
  <el-card class="subscription-card" shadow="hover">
    <template #header>
      <div class="card-header">
        <span>{{ title }}</span>
        <el-tag v-if="subscriptionData.plan" :type="getSubscriptionTagType(subscriptionData.plan.planType)"
          effect="dark" size="small">
          {{ getSubscriptionTypeName(subscriptionData.plan.planType) }}
        </el-tag>
      </div>
    </template>

    <div v-if="subscriptionData.plan" class="subscription-info">
      <h3 class="plan-name">{{ subscriptionData.plan.name }}</h3>
      <div class="plan-price">
        <span class="price">¥{{ subscriptionData.plan.price }}</span>
        <span class="period">/ {{ getPeriodText(subscriptionData.plan.period) }}</span>
      </div>

      <div class="plan-features" v-if="subscriptionData.plan.features">
        <div v-for="(feature, index) in getFeaturesList(subscriptionData.plan.features)" :key="index"
          class="feature-item">
          <el-icon>
            <Check />
          </el-icon>
          <span>{{ feature }}</span>
        </div>
      </div>

      <div class="subscription-footer">
        <div class="expiry-info">
          <el-icon>
            <Calendar />
          </el-icon>
          <span>到期时间: {{ formatDate(subscriptionData.expiryDate) }}</span>
        </div>
        <div>
          <el-button type="primary" plain size="small" @click="handleRenew">续费</el-button>
          <el-button type="success" plain size="small" @click="handleUpgrade">升级</el-button>
        </div>
      </div>
    </div>

    <div v-else class="no-subscription">
      <el-empty description="暂无订阅信息" :image-size="60">
        <el-button type="primary" @click="handleSubscribe">立即订阅</el-button>
      </el-empty>
    </div>
  </el-card>
</template>

<script setup>
import { formatDate as formatDateUtil } from '@/utils/index';

const props = defineProps({
  title: {
    type: String,
    default: '当前订阅'
  },
  subscriptionData: {
    type: Object,
    required: true,
    default: () => ({
      plan: null,
      expiryDate: null,
      autoRenew: false
    })
  }
});

const emit = defineEmits(['renew', 'upgrade', 'subscribe']);

// 获取订阅类型名称
const getSubscriptionTypeName = (type) => {
  const typeMap = {
    'Standard': '标准版',
    'Premium': '高级版',
    'Enterprise': '企业版',
    'Custom': '定制版'
  }
  return typeMap[type] || type
}

// 获取订阅标签类型
const getSubscriptionTagType = (type) => {
  const typeMap = {
    'Standard': 'info',
    'Premium': 'success',
    'Enterprise': 'warning',
    'Custom': 'danger'
  }
  return typeMap[type] || 'info'
}

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

// 格式化日期
const formatDate = (timestamp) => {
  if (!timestamp) return '未设置'
  return formatDateUtil(new Date(timestamp), 'yyyy-MM-dd')
}

// 获取特性列表
const getFeaturesList = (features) => {
  if (!features) return []
  try {
    return typeof features === 'string' ? JSON.parse(features) : features
  } catch (e) {
    return []
  }
}

// 处理续费
const handleRenew = () => {
  emit('renew');
}

// 处理升级
const handleUpgrade = () => {
  emit('upgrade');
}

// 处理订阅
const handleSubscribe = () => {
  emit('subscribe');
}
</script>

<style scoped>
/* 订阅信息卡片 */
.subscription-card {
  border-radius: 8px;
  transition: all 0.3s;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.subscription-info {
  padding: 1rem 0;
}

.plan-name {
  font-size: 18px;
  font-weight: 600;
  margin: 0 0 12px;
}

.plan-price {
  margin-bottom: 1rem;
}

.price {
  font-size: 24px;
  font-weight: 700;
  color: #409eff;
}

.period {
  font-size: 14px;
  color: #909399;
}

.plan-features {
  margin-bottom: 1rem;
}

.feature-item {
  display: flex;
  align-items: center;
  margin-bottom: 8px;
}

.feature-item .el-icon {
  color: #67c23a;
  margin-right: 8px;
}

.subscription-footer {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 1rem;
  border-top: 1px solid #f0f0f0;
  padding-top: 1rem;
}

.expiry-info {
  display: flex;
  align-items: center;
  color: var(--el-text-color-secondary);
}

.expiry-info .el-icon {
  margin-right: 4px;
  color: #e6a23c;
}

.no-subscription {
  padding: 24px 0;
}

/* 响应式调整 */
@media (max-width: 768px) {
  .subscription-footer {
    flex-direction: column;
    align-items: flex-start;
  }

  .subscription-footer .el-button {
    margin-top: 8px;
    margin-left: 0 !important;
  }
}
</style>