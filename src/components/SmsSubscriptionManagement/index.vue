<template>
  <div class="sms-subscription-management">
    <div class="current-plan-info" v-if="smsSubscriptionData.plan">
      <h3>当前短信订阅计划</h3>
      <el-descriptions :column="2" border>
        <el-descriptions-item label="套餐名称">{{ smsSubscriptionData.plan.name }}</el-descriptions-item>
        <el-descriptions-item label="套餐类型">{{ getSmsSubscriptionTypeName(smsSubscriptionData.plan.planType)
        }}</el-descriptions-item>
        <el-descriptions-item label="订阅周期">{{ getPeriodText(smsSubscriptionData.plan.period)
        }}</el-descriptions-item>
        <el-descriptions-item label="套餐价格">¥{{ smsSubscriptionData.plan.price }}</el-descriptions-item>
        <el-descriptions-item label="到期时间">{{ formatDate(smsSubscriptionData.expiryDate)
        }}</el-descriptions-item>
        <el-descriptions-item label="自动续费">
          <el-switch v-model="smsSubscriptionData.autoRenew" @change="handleAutoRenewChange" />
        </el-descriptions-item>
      </el-descriptions>

      <!-- 短信使用情况 -->
      <div class="sms-usage-info">
        <h3>短信使用情况</h3>
        <el-progress :percentage="getSmsUsagePercentage()" :format="formatSmsUsage" :stroke-width="18" />
        <div class="sms-count-info">
          <span>总短信条数: {{ smsSubscriptionData.totalSmsCount }}</span>
          <span>已使用: {{ smsSubscriptionData.usedSmsCount }}</span>
          <span>剩余: {{ smsSubscriptionData.remainingSmsCount }}</span>
        </div>
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

            <el-button type="primary" class="subscribe-btn" :disabled="isCurrentPlan(plan.id)"
              @click="showSmsSubscriptionDialog(plan)">
              {{ isCurrentPlan(plan.id) ? '当前套餐' : '选择套餐' }}
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
import { ElMessage } from 'element-plus';
import { formatDate as formatDateUtil } from '@/utils/index';
import { getSmsPlans, getSubscription, saveSubscription } from '@/api/system/subscription';
import useUserStore from '@/store/modules/user';
import SmsSubscriptionPayment from '@/components/SmsSubscriptionPayment/index.vue';
import SubscriptionCongrats from '@/components/SubscriptionCongrats/index.vue';

const props = defineProps({
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

const emit = defineEmits(['subscription-updated']);

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
  // 这里应该调用获取短信套餐的API，暂时使用getAllPlans代替
  getSmsPlans().then(res => {
    // 在实际应用中，应该过滤出短信套餐
    availableSmsPlans.value = res;
  });
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

// 判断是否为当前套餐
const isCurrentPlan = (planId) => {
  return props.smsSubscriptionData.plan && props.smsSubscriptionData.plan.id === planId
}

// 处理自动续费变更
const handleAutoRenewChange = async () => {
  try {
    // 实际项目中应调用API更新自动续费状态
    ElMessage.success(`自动续费已${props.smsSubscriptionData.autoRenew ? '开启' : '关闭'}`)
    // 通知父组件订阅已更新
    emit('subscription-updated');
  } catch (error) {
    props.smsSubscriptionData.autoRenew = !props.smsSubscriptionData.autoRenew
    ElMessage.error('设置失败：' + (error.message || '未知错误'))
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
</script>

<style scoped>
.subscription-management {
  padding: 16px 0;
  height: 100%;
}

.current-plan-info {
  margin-bottom: 32px;
}

.current-plan-info h3,
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
}
</style>