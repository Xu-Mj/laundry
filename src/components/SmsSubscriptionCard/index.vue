<template>
  <el-card class="subscription-card" shadow="hover">
    <template #header>
      <div class="s-card-header">
        <span>{{ title }}</span>
        <div v-if="hasActiveSubs">
          <el-tag :type="getHighestPlanTagType()" effect="dark" size="small">
            {{ getHighestPlanName() }}
          </el-tag>
        </div>
      </div>
    </template>

    <div v-if="hasActiveSubs" class="subscription-info">
      <h3 class="plan-name">{{ smsSubscriptionData.length }} 个有效短信套餐</h3>
      
      <!-- 短信使用情况 -->
      <div class="sms-usage-info">
        <el-progress :percentage="getSmsUsagePercentage()" :format="formatSmsUsage" :stroke-width="18" />
        <div class="sms-count-info">
          <span>总短信条数: {{ getTotalSmsCount() }}</span>
          <span>已使用: {{ getUsedSmsCount() }}</span>
          <span>剩余: {{ getRemainingSmsCount() }}</span>
        </div>
      </div>

      <!-- 订阅列表 -->
      <div class="subscription-list">
        <div v-for="(sub, index) in smsSubscriptionData" :key="index" class="subscription-item">
          <div class="plan-info">
            <div class="plan-header">
              <span class="plan-title">{{ sub.plan.name }}</span>
              <el-tag :type="getSubscriptionTagType(sub.plan.planType)" size="small">
                {{ getSubscriptionTypeName(sub.plan.planType) }}
              </el-tag>
            </div>
            <div class="sms-detail">
              <span>{{ sub.remainingSmsCount }}/{{ sub.totalSmsCount }} 条短信</span>
            </div>
          </div>
          <div class="plan-expiry">到期: {{ formatDate(sub.expiryDate) }}</div>
        </div>
      </div>

      <div class="subscription-footer">
        <div class="expiry-info">
          <el-icon>
            <Calendar />
          </el-icon>
          <span>最晚到期时间: {{ formatDate(getLatestExpiryDate()) }}</span>
        </div>
        <el-button type="primary" @click="handleAddSubscription">添加订阅</el-button>
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
import { computed } from 'vue';
import { formatDate as formatDateUtil } from '@/utils/index';

const props = defineProps({
  title: {
    type: String,
    default: '短信订阅'
  },
  smsSubscriptionData: {
    type: Array,
    required: true,
    default: () => []
  }
});

const emit = defineEmits(['add-subscription', 'subscribe']);

// 是否有有效订阅
const hasActiveSubs = computed(() => {
  return Array.isArray(props.smsSubscriptionData) && props.smsSubscriptionData.length > 0;
});

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

// 获取最高级别套餐名称
const getHighestPlanName = () => {
  if (!hasActiveSubs.value) return '';
  
  // 优先级: Premium > Enterprise > Standard > 其他
  const planTypes = props.smsSubscriptionData.map(sub => sub.plan?.planType || '');
  if (planTypes.includes('Premium')) return '高级版';
  if (planTypes.includes('Enterprise')) return '企业版';
  if (planTypes.includes('Standard')) return '标准版';
  return '基础版';
}

// 获取最高级别套餐标签类型
const getHighestPlanTagType = () => {
  if (!hasActiveSubs.value) return 'info';
  
  const planTypes = props.smsSubscriptionData.map(sub => sub.plan?.planType || '');
  if (planTypes.includes('Premium')) return 'success';
  if (planTypes.includes('Enterprise')) return 'warning';
  if (planTypes.includes('Standard')) return 'info';
  return 'info';
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

// 获取最晚到期日期
const getLatestExpiryDate = () => {
  if (!hasActiveSubs.value) return null;
  
  return props.smsSubscriptionData.reduce((latest, sub) => {
    if (!sub.expiryDate) return latest;
    return !latest || sub.expiryDate > latest ? sub.expiryDate : latest;
  }, null);
}

// 获取总短信条数
const getTotalSmsCount = () => {
  if (!hasActiveSubs.value) return 0;
  return props.smsSubscriptionData.reduce((total, sub) => total + (sub.totalSmsCount || 0), 0);
}

// 获取已使用短信条数
const getUsedSmsCount = () => {
  if (!hasActiveSubs.value) return 0;
  return props.smsSubscriptionData.reduce((total, sub) => total + (sub.usedSmsCount || 0), 0);
}

// 获取剩余短信条数
const getRemainingSmsCount = () => {
  if (!hasActiveSubs.value) return 0;
  return props.smsSubscriptionData.reduce((total, sub) => total + (sub.remainingSmsCount || 0), 0);
}

// 获取短信使用百分比
const getSmsUsagePercentage = () => {
  const total = getTotalSmsCount();
  if (!total) return 0;
  return Math.min(100, Math.round((getUsedSmsCount() / total) * 100));
}

// 格式化短信使用情况
const formatSmsUsage = (percentage) => {
  return `${percentage}% 已使用`;
}

// 处理添加订阅
const handleAddSubscription = () => {
  emit('add-subscription');
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

.s-card-header {
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

/* 订阅列表 */
.subscription-list {
  margin: 16px 0;
  border-top: 1px solid #f0f0f0;
  padding-top: 16px;
}

.subscription-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
  border-bottom: 1px dashed #f0f0f0;
}

.plan-info {
  flex: 1;
}

.plan-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}

.plan-title {
  font-weight: 500;
}

.sms-detail {
  font-size: 12px;
  color: #606266;
}

.plan-expiry {
  font-size: 12px;
  color: #606266;
}

.subscription-footer {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  border-top: 1px solid #f0f0f0;
  padding-top: 16px;
  margin-top: 16px;
}

.expiry-info {
  display: flex;
  align-items: center;
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
    gap: 12px;
  }

  .sms-count-info {
    flex-direction: column;
    gap: 4px;
  }
}
</style>