<template>
  <el-card class="subscription-card" shadow="hover">
    <template #header>
      <div class="card-header">
        <span>{{ title }}</span>
        <el-tag v-if="activeSubscription && activeSubscription.plan" 
          :type="getSubscriptionTagType(activeSubscription.plan.planType)"
          effect="dark" size="small">
          {{ getSubscriptionTypeName(activeSubscription.plan.planType) }}
        </el-tag>
      </div>
    </template>

    <!-- 有订阅时的展示 -->
    <template v-if="hasSubscriptions">
      <!-- 当前激活的订阅信息 -->
      <div class="subscription-info">
        <h3 class="plan-name">{{ activeSubscription.plan.name }}</h3>
        <div class="plan-price">
          <span class="price">¥{{ activeSubscription.plan.price }}</span>
          <span class="period">/ {{ getPeriodText(activeSubscription.plan.period) }}</span>
        </div>

        <div class="plan-features" v-if="activeSubscription.plan.features">
          <div v-for="(feature, index) in getFeaturesList(activeSubscription.plan.features)" :key="index"
            class="feature-item">
            <el-icon>
              <Check />
            </el-icon>
            <span>{{ feature }}</span>
          </div>
        </div>

        <div class="subscription-footer">
          <div class="expiry-info-container">
            <div class="expiry-info">
              <el-icon>
                <Calendar />
              </el-icon>
              <span>当前订阅到期: {{ formatDate(activeSubscription.expiryDate) }}</span>
            </div>
            <div class="expiry-info total-expiry" v-if="latestExpiryDate && props.subscriptions.length > 1">
              <el-icon>
                <Calendar />
              </el-icon>
              <span>总体到期时间: {{ formatDate(latestExpiryDate) }}</span>
            </div>
          </div>
          <div>
            <el-button type="primary" plain size="small" @click="handleRenew">续费</el-button>
            <el-button type="success" plain size="small" @click="handleUpgrade">升级</el-button>
          </div>
        </div>
      </div>

      <!-- 多订阅切换区域 -->
      <div v-if="subscriptions.length > 1" class="subscription-switcher">
        <div class="switcher-header">
          <el-divider content-position="left">其他可用订阅</el-divider>
        </div>
        <div class="subscription-list">
          <div v-for="(sub, index) in otherSubscriptions" :key="index" class="subscription-item">
            <div class="subscription-item-info">
              <div class="item-name">{{ sub.plan.name }}</div>
              <el-tag size="small" effect="plain" :type="getSubscriptionTypeTag(sub.plan.planType)">
                {{ getSubscriptionTypeName(sub.plan.planType) }}
              </el-tag>
            </div>
            <div class="subscription-item-expiry">到期: {{ formatDate(sub.expiryDate) }}</div>
            <el-button type="primary" size="small" @click="activateSubscription(sub)">设为当前</el-button>
          </div>
        </div>
      </div>
    </template>

    <!-- 无订阅时的展示 -->
    <div v-else class="no-subscription">
      <el-empty description="暂无订阅信息" :image-size="60">
        <el-button type="primary" @click="handleSubscribe">立即订阅</el-button>
      </el-empty>
    </div>
  </el-card>
</template>

<script setup>
import { formatDate as formatDateUtil } from '@/utils/index';
import { ElMessage } from 'element-plus';
import { Check, Calendar } from '@element-plus/icons-vue';

const props = defineProps({
  title: {
    type: String,
    default: '当前订阅'
  },
  // 当前激活的订阅数据
  subscriptionData: {
    type: Object,
    required: true,
    default: () => ({
      id: null,
      plan: null,
      expiryDate: null,
      autoRenew: false
    })
  },
  // 所有有效订阅列表
  subscriptions: {
    type: Array,
    default: () => []
  }
});
console.log(props);

const emit = defineEmits(['renew', 'upgrade', 'subscribe', 'subscription-activated']);

// 计算属性：是否有订阅
const hasSubscriptions = computed(() => {
  return props.subscriptions && props.subscriptions.length > 0;
});

// 计算属性：当前激活的订阅
const activeSubscription = computed(() => {
  // 如果有传入subscriptionData且有plan，则使用subscriptionData
  if (props.subscriptionData && props.subscriptionData.plan) {
    return props.subscriptionData;
  }
  
  // 否则从subscriptions中找到第一个有效订阅
  if (hasSubscriptions.value) {
    return props.subscriptions[0];
  }
  
  // 如果没有任何订阅，返回空对象
  return { plan: null, expiryDate: null };
});

console.log(activeSubscription);
// 计算属性：其他可用订阅（排除当前激活的）
const otherSubscriptions = computed(() => {
  if (!hasSubscriptions.value) return [];
  
  return props.subscriptions.filter(sub => {
    return sub.id !== activeSubscription.value.id;
  });
});

// 计算属性：所有订阅的最晚到期时间
const latestExpiryDate = computed(() => {
  if (!hasSubscriptions.value) return null;
  
  // 找出所有订阅中最晚的到期时间
  return props.subscriptions.reduce((latest, sub) => {
    if (!sub.expiryDate) return latest;
    
    const currentExpiry = new Date(sub.expiryDate).getTime();
    return !latest || currentExpiry > latest ? currentExpiry : latest;
  }, null);
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

// 获取订阅类型对应的标签类型（用于列表项）
const getSubscriptionTypeTag = (type) => {
  return getSubscriptionTagType(type);
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

// 激活指定订阅
const activateSubscription = (subscription) => {
  // 更新订阅状态为激活
  const updatedSubscription = {
    ...subscription,
    status: 'Active'
  };
  
  // 通知父组件激活指定订阅
  emit('subscription-activated', updatedSubscription);
  ElMessage.success(`已将「${subscription.plan.name}」设为当前激活订阅`);
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

.expiry-info-container {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
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

.total-expiry {
  font-weight: 500;
}

.total-expiry .el-icon {
  color: #409eff;
}

.no-subscription {
  padding: 24px 0;
}

/* 订阅切换区域样式 */
.subscription-switcher {
  margin-top: 1rem;
}

.switcher-header {
  margin-bottom: 0.5rem;
}

.subscription-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.subscription-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem;
  border-radius: 6px;
  background-color: var(--el-fill-color-light);
  transition: all 0.2s;
}

.subscription-item:hover {
  background-color: var(--el-fill-color);
}

.subscription-item-info {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.item-name {
  font-weight: 500;
  font-size: 14px;
}

.subscription-item-expiry {
  font-size: 12px;
  color: var(--el-text-color-secondary);
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
  
  .subscription-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }
  
  .subscription-item .el-button {
    width: 100%;
  }
}
</style>