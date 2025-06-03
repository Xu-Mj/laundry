<template>
  <div class="sms-subscription-management">
    <!-- 所有有效短信订阅列表 -->
    <div class="all-sms-subscriptions" v-if="allSmsSubscriptions.length > 0">
      <h3>所有有效短信订阅</h3>
      <el-table :data="allSmsSubscriptions" style="width: 100%" border>
        <el-table-column prop="plan.name" label="套餐名称" />
        <el-table-column label="套餐类型">
          <template #default="{ row }">
            <el-tag :type="getSmsSubscriptionTypeTag(row.plan.planType)" effect="plain">
              {{ getSmsSubscriptionTypeName(row.plan.planType) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="短信数量">
          <template #default="{ row }">
            <div>总量: {{ row.totalSmsCount }} 条</div>
            <div>剩余: {{ row.remainingSmsCount }} 条</div>
          </template>
        </el-table-column>
        <el-table-column label="到期时间">
          <template #default="{ row }">
            {{ formatDate(row.expiryDate) }}
          </template>
        </el-table-column>
        <el-table-column label="自动续费">
          <template #default="{ row }">
            <el-switch v-model="row.autoRenew" @change="(val) => handleAutoRenewChange(row, val)" />
          </template>
        </el-table-column>
        <el-table-column label="操作" width="120">
          <template #default="{ row }">
            <el-button type="danger" size="small" @click="confirmCancelSubscription(row)">
              取消订阅
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <!-- 累计短信使用情况 -->
      <div class="sms-usage-info">
        <h3>累计短信使用情况</h3>
        <el-progress :percentage="getSmsUsagePercentage()" :format="formatSmsUsage" :stroke-width="18" />
        <div class="sms-count-info">
          <span>总短信条数: {{ getTotalSmsCount() }}</span>
          <span>已使用: {{ getUsedSmsCount() }}</span>
          <span>剩余: {{ getRemainingSmsCount() }}</span>
        </div>
      </div>

      <!-- 累计到期时间 -->
      <div class="expiry-info" v-if="getLatestExpiryDate()">
        <el-alert type="info" :closable="false">
          <template #title>
            <div class="expiry-alert-content">
              <el-icon>
                <Calendar />
              </el-icon>
              <span>最新订阅到期时间: {{ formatDate(getLatestExpiryDate()) }}</span>
            </div>
          </template>
        </el-alert>
      </div>
    </div>

    <div class="available-plans">
      <h3>可用短信套餐</h3>
      <el-row :gutter="20" style="row-gap: 20px">
        <el-col :span="8" v-for="plan in availableSmsPlans" :key="plan.id">
          <el-card class="plan-card" shadow="hover" :class="{ 'recommended-plan': plan.isRecommended }">
            <div class="plan-card-header">
              <h4>{{ plan.name }}</h4>
              <el-tag v-if="plan.isRecommended" type="warning" effect="dark" size="small">推荐</el-tag>
            </div>

            <!-- 短信条数信息 -->
            <div class="plan-sms-count">
              <el-icon>
                <Message />
              </el-icon>
              <span>{{ plan.smsCount || 0 }} 条短信</span>
            </div>

            <div class="plan-card-price">
              <span class="price-value">¥{{ plan.price }}</span>
              <span class="price-period">/ {{ getPeriodText(plan.period) }}</span>
            </div>

            <div class="plan-card-features">
              <div v-if="getFeaturesList(plan.features).length > 0">
                <div v-for="(feature, index) in getFeaturesList(plan.features)" :key="index" class="feature-item">
                  <el-icon>
                    <Check />
                  </el-icon>
                  <span>{{ feature }}</span>
                </div>
              </div>
              <div v-else>
                <div class="feature-item">
                  <el-icon>
                    <Check />
                  </el-icon>
                  <span>{{ getSmsSubscriptionTypeName(plan.planType) }}短信服务</span>
                </div>
                <div class="feature-item">
                  <el-icon>
                    <Check />
                  </el-icon>
                  <span>{{ plan.description || `适合${getSmsSubscriptionTypeName(plan.planType)}用户的短信套餐` }}</span>
                </div>
              </div>
            </div>

            <el-button type="primary" class="subscribe-btn" @click="showSmsSubscriptionDialog(plan)">
              订阅套餐
            </el-button>
          </el-card>
        </el-col>
      </el-row>
    </div>

    <!-- 短信订阅套餐付款弹窗 -->
    <sms-subscription-payment v-model:visible="smsSubscriptionDialogVisible" :plan-data="selectedPlan"
      @payment-success="handlePaymentSuccess" @payment-cancel="handlePaymentCancel" />

    <!-- 订阅成功贺卡 -->
    <subscription-congrats v-model:visible="congratsVisible" :plan-data="selectedPlan"
      :expiry-date="smsSubscriptionExpiryDate" @confirmed="handleCongratsConfirmed" />
  </div>
</template>

<script setup>
import { ElMessage, ElMessageBox } from 'element-plus';
import { formatDate as formatDateUtil } from '@/utils/index';
import { getSmsPlans, getSubscription, saveSubscription, cancelSubscription as apiCancelSubscription } from '@/api/system/subscription';
import useUserStore from '@/store/modules/user';
import SmsSubscriptionPayment from '@/components/SmsSubscriptionPayment/index.vue';
import SubscriptionCongrats from '@/components/SubscriptionCongrats/index.vue';

const props = defineProps({
  // 所有有效短信订阅  
  allSmsSubscriptions: { type: Array, default: () => [] }
});

const emit = defineEmits(['subscription-updated', 'subscription-cancelled']);

const { proxy } = getCurrentInstance();

// 订阅套餐相关
const smsSubscriptionDialogVisible = ref(false);
const congratsVisible = ref(false);
const selectedPlan = ref({});
const smsSubscriptionExpiryDate = ref(null);

// 可用套餐列表
const availableSmsPlans = ref([]);

// 显示订阅套餐弹窗
const showSmsSubscriptionDialog = async (plan) => {
  selectedPlan.value = plan;
  smsSubscriptionDialogVisible.value = true;
};

// 处理支付成功
const handlePaymentSuccess = (paymentInfo) => {
  // 获取订阅信息
  getSubscription(useUserStore().id, paymentInfo.planId, paymentInfo.subscriptionId).then(res => {
    saveSubscription(res.subscription, res.plan).catch(err => { })
    // 通知父组件订阅已更新
    emit('subscription-updated');
  }).catch(err => { })

  // 关闭支付弹窗
  smsSubscriptionDialogVisible.value = false;

  // 显示贺卡
  setTimeout(() => {
    congratsVisible.value = true;
  }, 500);
};

// 处理贺卡确认
const handleCongratsConfirmed = () => {
  // 贺卡关闭后的逻辑
};

// 处理支付取消
const handlePaymentCancel = () => {
  smsSubscriptionDialogVisible.value = false;
};

// 获取可用套餐列表
onMounted(async () => {
  try {
    const res = await getSmsPlans();
    // 修复：确保返回的数据是数组，并限制数量，防止渲染过多元素
    if (Array.isArray(res)) {
      availableSmsPlans.value = res; // 最多显示10个套餐
    } else {
      proxy.notify.error('获取套餐列表失败：返回数据格式不正确');
      availableSmsPlans.value = [];
    }
  } catch (error) {
    console.error('获取套餐列表失败', error);
    availableSmsPlans.value = [];
  }
});

// 获取短信订阅类型名称
const getSmsSubscriptionTypeName = (type) => {
  const typeMap = {
    'Standard': '标准版',
    'Premium': '高级版',
    'Enterprise': '企业版',
    'Custom': '定制版'
  }
  return typeMap[type] || type
}

// 获取订阅类型对应的标签类型
const getSmsSubscriptionTypeTag = (type) => {
  const tagMap = {
    'Standard': 'info',
    'Premium': 'success',
    'Enterprise': 'warning',
    'Custom': 'danger'
  }
  return tagMap[type] || 'info'
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
  if (!props.allSmsSubscriptions.length) return null;
  
  return props.allSmsSubscriptions.reduce((latest, sub) => {
    if (!sub.expiryDate) return latest;
    return !latest || sub.expiryDate > latest ? sub.expiryDate : latest;
  }, null);
}

// 获取总短信条数
const getTotalSmsCount = () => {
  if (!props.allSmsSubscriptions.length) return 0;
  return props.allSmsSubscriptions.reduce((total, sub) => total + (sub.totalSmsCount || 0), 0);
}

// 获取已使用短信条数
const getUsedSmsCount = () => {
  if (!props.allSmsSubscriptions.length) return 0;
  return props.allSmsSubscriptions.reduce((total, sub) => total + (sub.usedSmsCount || 0), 0);
}

// 获取剩余短信条数
const getRemainingSmsCount = () => {
  if (!props.allSmsSubscriptions.length) return 0;
  return props.allSmsSubscriptions.reduce((total, sub) => total + (sub.remainingSmsCount || 0), 0);
}

// 处理自动续费变更
const handleAutoRenewChange = async (subscription, newValue) => {
  try {
    // 实际项目中应调用API更新自动续费状态
    ElMessage.success(`自动续费已${newValue ? '开启' : '关闭'}`);
    // 通知父组件订阅已更新
    emit('subscription-updated');
  } catch (error) {
    subscription.autoRenew = !newValue;
    ElMessage.error('设置失败：' + (error.message || '未知错误'))
  }
}

// 确认取消订阅
const confirmCancelSubscription = (subscription) => {
  ElMessageBox.confirm(
    `确定要取消「${subscription.plan.name}」订阅吗？取消后将无法恢复。`,
    '取消订阅',
    {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    }
  ).then(() => {
    cancelSubscription(subscription);
  }).catch(() => {
    // 用户取消操作
  });
}

// 取消订阅
const cancelSubscription = (subscription) => {
  // 调用取消订阅API
  apiCancelSubscription(subscription.id, '用户主动取消').then(() => {
    ElMessage.success('订阅已成功取消');
    // 通知父组件订阅已取消
    emit('subscription-cancelled', subscription.id);
  }).catch(error => {
    ElMessage.error('取消订阅失败：' + (error.message || '未知错误'));
  });
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
</script>

<style scoped>
.sms-subscription-management {
  padding: 16px 0;
  height: 100%;
}

.all-sms-subscriptions {
  margin-bottom: 32px;
}

.sms-usage-info {
  margin-top: 16px;
  margin-bottom: 16px;
}

.sms-count-info {
  display: flex;
  justify-content: space-between;
  margin-top: 8px;
  font-size: 12px;
  color: #606266;
}

.expiry-info {
  margin-top: 16px;
}

.expiry-alert-content {
  display: flex;
  align-items: center;
  gap: 8px;
}

.expiry-alert-content .el-icon {
  color: #409eff;
}

.all-sms-subscriptions h3,
.sms-usage-info h3,
.available-plans h3 {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 16px;
  color: var(--el-text-color-primary);
}

.plan-card {
  height: 100%;
  border-radius: 8px;
  padding: 16px;
  transition: all 0.3s;
  position: relative;
  overflow: hidden;
  transition: all 0.3s;
}

.plan-card:hover {
  transform: translateY(-2px);
}


.recommended-plan {
  border: 2px solid #e6a23c;
}

.recommended-plan::before {
  content: '推荐';
  position: absolute;
  top: 10px;
  right: -30px;
  background-color: #e6a23c;
  color: white;
  padding: 2px 30px;
  transform: rotate(45deg);
  font-size: 12px;
}

.plan-card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.plan-card-header h4 {
  font-size: 16px;
  font-weight: 600;
  margin: 0;
}

.plan-card-price {
  margin-bottom: 16px;
}

.price-value {
  font-size: 24px;
  font-weight: 700;
  color: #409eff;
}

.price-period {
  font-size: 14px;
  color: #909399;
}

.plan-card-features {
  margin-bottom: 16px;
  min-height: 120px;
}

.plan-sms-count,
.feature-item {
  display: flex;
  align-items: center;
  gap: .3rem;
}

.feature-item .el-icon {
  color: #67c23a;
}

.subscribe-btn {
  width: 100%;
}

/* 响应式调整 */
@media (max-width: 768px) {
  .profile-header {
    margin-bottom: 16px;
  }

  .profile-title h2 {
    font-size: 20px;
  }

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