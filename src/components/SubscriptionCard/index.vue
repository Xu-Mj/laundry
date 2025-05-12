<template>
  <el-card class="subscription-card" shadow="hover"> <template #header>
      <div class="s-card-header"> <span>{{ title }}</span> </div>
    </template>

    <!-- 有订阅时的展示 -->
    <template v-if="hasSubscriptions">
      <!-- 订阅总览信息 -->
      <div class="subscription-info">
        <h3 class="subscription-title">有效订阅</h3>

        <div class="subscription-list">
          <div v-for="(sub, index) in subscriptions" :key="index" class="subscription-item">
            <div class="subscription-item-info">
              <div class="item-name">{{ sub.plan.name }}</div>
              <el-tag size="small" effect="plain" :type="getSubscriptionTypeTag(sub.plan.planType)">
                {{ getSubscriptionTypeName(sub.plan.planType) }}
              </el-tag>
            </div>
            <div class="subscription-details">
              <div class="plan-price">
                <span class="price">¥{{ sub.plan.price }}</span>
                <span class="period">/ {{ getPeriodText(sub.plan.period) }}</span>
              </div>
              <div class="subscription-item-expiry">到期: {{ formatDate(sub.expiryDate) }}</div>
            </div>

            <div class="plan-features" v-if="sub.plan.features">
              <div v-for="(feature, index) in getFeaturesList(sub.plan.features)" :key="index" class="feature-item">
                <el-icon>
                  <Check />
                </el-icon>
                <span>{{ feature }}</span>
              </div>
            </div>
          </div>
        </div>

        <div class="subscription-footer">
          <div class="expiry-info-container">
            <div class="expiry-info total-expiry">
              <el-icon>
                <Calendar />
              </el-icon>
              <span>订阅到期时间: {{ formatDate(latestExpiryDate) }}</span>
            </div>
          </div>
          <div> <el-button type="primary" plain size="small" @click="handleSubscribe">添加订阅</el-button> </div>
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
import { formatDate as formatDateUtil } from '@/utils/index'; import { Check, Calendar } from '@element-plus/icons-vue';

const props = defineProps({
  title: { type: String, default: '我的订阅' },
  // 所有有效订阅列表  
  subscriptions: { type: Array, default: () => [] }
});


const emit = defineEmits(['subscribe']);

// 计算属性：是否有订阅
const hasSubscriptions = computed(() => Array.isArray(props.subscriptions) && props.subscriptions.length > 0);

// 计算属性：所有订阅的最晚到期时间
const latestExpiryDate = computed(() => {
  if (!hasSubscriptions.value) return null;

  // 找出所有订阅中最晚的到期日期
  return props.subscriptions.reduce((latest, current) => {
    const currentDate = new Date(current.expiryDate);
    return !latest || currentDate > latest ? currentDate : latest;
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



// 获取订阅类型对应的标签类型const getSubscriptionTypeTag = (type) => {  const typeMap = {    'Standard': 'info',    'Premium': 'success',    'Enterprise': 'warning',    'Custom': 'danger'  }  return typeMap[type] || 'info'}

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
  padding: 1rem 0;
}

.subscription-title {
  font-size: 18px;
  font-weight: 600;
  margin: 0 0 12px;
}

.subscription-list {
  margin-bottom: 1rem;
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

.subscription-details {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
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

.subscription-item-expiry {
  font-size: 12px;
  color: var(--el-text-color-secondary);
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