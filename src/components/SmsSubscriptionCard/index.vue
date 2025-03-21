<template>
  <el-card class="subscription-card" shadow="hover">
    <template #header>
      <div class="card-header">
        <span>{{ title }}</span>
        <el-tag v-if="smsSubscriptionData.plan" :type="getSubscriptionTagType(smsSubscriptionData.plan.planType)"
          effect="dark" size="small">
          {{ getSubscriptionTypeName(smsSubscriptionData.plan.planType) }}
        </el-tag>
      </div>
    </template>

    <div v-if="smsSubscriptionData.plan" class="subscription-info">
      <h3 class="plan-name">{{ smsSubscriptionData.plan.name }}</h3>
      <div class="plan-price">
        <span class="price">¥{{ smsSubscriptionData.plan.price }}</span>
        <span class="period">/ {{ getPeriodText(smsSubscriptionData.plan.period) }}</span>
      </div>

      <!-- 短信使用情况 -->
      <div class="sms-usage-info">
        <el-progress :percentage="getSmsUsagePercentage()" :format="formatSmsUsage" :stroke-width="18" />
        <div class="sms-count-info">
          <span>总短信条数: {{ smsSubscriptionData.totalSmsCount }}</span>
          <span>已使用: {{ smsSubscriptionData.usedSmsCount }}</span>
          <span>剩余: {{ smsSubscriptionData.remainingSmsCount }}</span>
        </div>
      </div>

      <div class="plan-features" v-if="smsSubscriptionData.plan.features">
        <div v-for="(feature, index) in getFeaturesList(smsSubscriptionData.plan.features)" :key="index"
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
          <span>到期时间: {{ formatDate(smsSubscriptionData.expiryDate) }}</span>
        </div>
        <el-button type="primary" plain size="small" @click="handleRenew">续费</el-button>
        <el-button type="success" plain size="small" @click="handleUpgrade">升级</el-button>
      </div>
    </div>

    <div v-else class="no-subscription">
      <el-empty description="暂无短信订阅" :image-size="60">
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
    default: '短信订阅'
  },
  smsSubscriptionData: {
    type: Object,
    required: true,
    default: () => ({
      plan: null,
      expiryDate: null,
      autoRenew: false,
      totalSmsCount: 0,
      usedSmsCount: 0,
      remainingSmsCount: 0,
      status: 'Active'
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

// 获取短信使用百分比
const getSmsUsagePercentage = () => {
  if (!props.smsSubscriptionData.totalSmsCount) return 0;
  return Math.round((props.smsSubscriptionData.usedSmsCount / props.smsSubscriptionData.totalSmsCount) * 100);
}

// 格式化短信使用情况
const formatSmsUsage = (percentage) => {
  return `${percentage}% 已使用`;
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
  padding: 16px 0;
}

.plan-name {
  font-size: 18px;
  font-weight: 600;
  margin: 0 0 12px;
  color: #303133;
}

.plan-price {
  margin-bottom: 16px;
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

/* 短信使用情况 */
.sms-usage-info {
  margin-bottom: 16px;
}

.sms-count-info {
  display: flex;
  justify-content: space-between;
  margin-top: 8px;
  font-size: 12px;
  color: #606266;
}

.plan-features {
  margin-bottom: 16px;
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
  border-top: 1px solid #f0f0f0;
  padding-top: 16px;
}

.expiry-info {
  display: flex;
  align-items: center;
  margin-right: auto;
  margin-bottom: 12px;
  color: #606266;
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

  .sms-count-info {
    flex-direction: column;
    gap: 4px;
  }
}
</style>