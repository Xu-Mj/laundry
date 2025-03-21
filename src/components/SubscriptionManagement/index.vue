<template>
  <div class="subscription-management">
    <div class="current-plan-info" v-if="subscriptionData.plan">
      <h3>当前订阅计划</h3>
      <el-descriptions :column="2" border>
        <el-descriptions-item label="套餐名称">{{ subscriptionData.plan.name }}</el-descriptions-item>
        <el-descriptions-item label="套餐类型">{{ getSubscriptionTypeName(subscriptionData.plan.planType)
        }}</el-descriptions-item>
        <el-descriptions-item label="订阅周期">{{ getPeriodText(subscriptionData.plan.period)
        }}</el-descriptions-item>
        <el-descriptions-item label="套餐价格">¥{{ subscriptionData.plan.price }}</el-descriptions-item>
        <el-descriptions-item label="到期时间">{{ formatDate(subscriptionData.expiryDate)
        }}</el-descriptions-item>
        <el-descriptions-item label="自动续费">
          <el-switch v-model="subscriptionData.autoRenew" @change="handleAutoRenewChange" />
        </el-descriptions-item>
      </el-descriptions>
    </div>

    <div class="available-plans">
      <h3>可用套餐</h3>
      <el-row :gutter="20" style="row-gap: 20px">
        <el-col :span="8" v-for="plan in availablePlans" :key="plan.id">
          <el-card class="plan-card" shadow="hover" :class="{ 'recommended-plan': plan.isRecommended }">
            <!-- 推荐标签 -->
            <div class="plan-ribbon" v-if="plan.isRecommended">推荐</div>
            
            <div class="plan-card-header">
              <h4>{{ plan.name }}</h4>
              <el-tag :type="getSubscriptionTypeTag(plan.planType)" effect="dark" size="small">
                {{ getSubscriptionTypeName(plan.planType) }}
              </el-tag>
            </div>
            
            <div class="plan-card-price">
              <span class="price-value">¥{{ plan.price }}</span>
              <span class="price-period">/ {{ getPeriodText(plan.period) }}</span>
            </div>
            
            <!-- 套餐描述 -->
            <!-- <div class="plan-description" v-if="plan.description">
              {{ plan.description }}
            </div> -->
            
            <div class="plan-card-features">
              <!-- 当features为null时，显示描述作为特性 -->
              <template v-if="getFeaturesList(plan.features).length > 0">
                <div v-for="(feature, index) in getFeaturesList(plan.features)" :key="index" class="feature-item">
                  <el-icon><Check /></el-icon>
                  <span>{{ feature }}</span>
                </div>
              </template>
              <template v-else-if="plan.description">
                <div class="feature-item">
                  <el-icon><Check /></el-icon>
                  <span>{{ plan.description }}</span>
                </div>
              </template>
            </div>
            
            <el-button type="primary" class="subscribe-btn" :disabled="isCurrentPlan(plan.id)"
              @click="showSubscriptionDialog(plan)">
              {{ isCurrentPlan(plan.id) ? '当前套餐' : '选择套餐' }}
            </el-button>
          </el-card>
        </el-col>
      </el-row>
    </div>

    <!-- 订阅套餐付款弹窗 -->
    <subscription-payment v-model:visible="subscriptionDialogVisible" :plan-data="selectedPlan"
      @payment-success="handlePaymentSuccess" @payment-cancel="handlePaymentCancel" />

    <!-- 订阅成功贺卡 -->
    <subscription-congrats v-model:visible="congratsVisible" :plan-data="selectedPlan"
      :expiry-date="subscriptionExpiryDate" @confirmed="handleCongratsConfirmed" />
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { formatDate as formatDateUtil } from '@/utils/index';
import { getAllPlans, getSubscription, saveSubscription } from '@/api/system/subscription';
import useUserStore from '@/store/modules/user';
import SubscriptionPayment from '@/components/SubscriptionPayment/index.vue';
import SubscriptionCongrats from '@/components/SubscriptionCongrats/index.vue';

const props = defineProps({
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

const emit = defineEmits(['subscription-updated']);

// 订阅套餐相关
const subscriptionDialogVisible = ref(false);
const congratsVisible = ref(false);
const selectedPlan = ref({});
const subscriptionExpiryDate = ref(null);

// 可用套餐列表
const availablePlans = ref([]);

// 显示订阅套餐弹窗
const showSubscriptionDialog = async (plan) => {
  selectedPlan.value = plan;
  subscriptionDialogVisible.value = true;
  console.log('showSubscriptionDialog', plan);
};

// 处理支付成功
const handlePaymentSuccess = (paymentInfo) => {
  console.log('支付成功', selectedPlan.value);

  // 获取订阅信息
  getSubscription(useUserStore().user.id, paymentInfo.planId, paymentInfo.subscriptionId).then(res => {
    saveSubscription(res.subscription, res.plan).catch(err => { })
    // 通知父组件订阅已更新
    emit('subscription-updated');
  }).catch(err => { })

  // 关闭支付弹窗
  subscriptionDialogVisible.value = false;

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
  console.log('支付已取消');
  subscriptionDialogVisible.value = false;
};

// 获取可用套餐列表
onMounted(async () => {
  getAllPlans().then(res => {
    availablePlans.value = res;
  });
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

// 获取订阅类型对应的标签类型
const getSubscriptionTypeTag = (type) => {
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

// 判断是否为当前套餐
const isCurrentPlan = (planId) => {
  return props.subscriptionData.plan && props.subscriptionData.plan.id === planId
}

// 处理自动续费变更
const handleAutoRenewChange = async () => {
  try {
    // 实际项目中应调用API更新自动续费状态
    // await updateSubscription({ autoRenew: subscriptionData.value.autoRenew })
    ElMessage.success(`自动续费已${props.subscriptionData.autoRenew ? '开启' : '关闭'}`)
    // 通知父组件订阅已更新
    emit('subscription-updated');
  } catch (error) {
    props.subscriptionData.autoRenew = !props.subscriptionData.autoRenew
    ElMessage.error('设置失败：' + (error.message || '未知错误'))
  }
}
</script>

<style scoped>
.subscription-management {
  padding: 1rem 0;
  height: 100%;
}

.current-plan-info {
  margin-bottom: 32px;
}

.current-plan-info h3,
.available-plans h3 {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 1rem;
  color: var(--el-text-color-primary);
}

.plan-card {
  height: 100%;
  border-radius: 8px;
  padding: 1rem;
  transition: all 0.3s;
  position: relative;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
}

.plan-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
}

.recommended-plan {
  border: 2px solid #e6a23c;
}

/* 新的推荐标签样式 */
.plan-ribbon {
  position: absolute;
  top: 12px;
  right: -35px;
  background-color: #e6a23c;
  color: white;
  padding: 5px 40px;
  transform: rotate(45deg);
  font-size: 12px;
  font-weight: bold;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  z-index: 1;
}

.plan-card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.plan-card-header h4 {
  font-size: 18px;
  font-weight: 600;
  margin: 0;
}

.plan-card-price {
  margin-bottom: 1rem;
  font-size: 28px;
  font-weight: 700;
  color: #409EFF;
}

.price-period {
  font-size: 14px;
  font-weight: normal;
  color: #909399;
}

/* 套餐描述样式 */
.plan-description {
  margin-bottom: 1rem;
  padding: 8px 0;
  color: #606266;
  font-size: 14px;
  border-top: 1px dashed #EBEEF5;
  border-bottom: 1px dashed #EBEEF5;
}

.plan-card-features {
  margin-bottom: 20px;
  flex-grow: 1;
  min-height: 120px;
}

.feature-item {
  display: flex;
  align-items: flex-start;
  margin-bottom: 10px;
  font-size: 14px;
  line-height: 1.5;
}

.feature-item .el-icon {
  margin-right: 8px;
  margin-top: 3px;
  color: #67c23a;
  flex-shrink: 0;
}

.subscribe-btn {
  width: 100%;
  margin-top: auto;
  font-weight: 500;
  border-radius: 4px;
  transition: all 0.3s;
}

.subscribe-btn:hover:not(:disabled) {
  transform: scale(1.02);
}

/* 响应式调整 */
@media (max-width: 768px) {
  .profile-header {
    margin-bottom: 1rem;
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