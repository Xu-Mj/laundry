<template>
  <div class="app-container">
    <div class="profile-header">
      <div class="profile-title">
        <h2>个人中心</h2>
        <p>管理您的个人信息、支付配置和订阅信息</p>
      </div>
    </div>

    <el-row :gutter="20">
      <!-- 左侧个人信息卡片 -->
      <el-col :span="6" :xs="24">
        <el-card class="profile-card" shadow="hover">
          <div class="profile-avatar-container">
            <el-upload class="avatar-uploader" :show-file-list="false" :on-success="handleAvatarSuccess">
              <div class="avatar-wrapper">
                <img v-if="profileData.avatar" :src="profileData.avatar" class="avatar">
                <el-icon v-else class="avatar-icon">
                  <UserFilled />
                </el-icon>
                <div class="avatar-mask">
                  <el-icon>
                    <Camera />
                  </el-icon>
                  <span>更换头像</span>
                </div>
              </div>
            </el-upload>
            <h3 class="profile-name">{{ profileData.nickname || '未设置昵称' }}</h3>
            <p class="profile-store">{{ profileData.storeName || '未设置店铺' }}</p>
          </div>

          <div class="profile-info-list">
            <div class="info-item">
              <el-icon>
                <Phone />
              </el-icon>
              <span class="info-label">联系电话:</span>
              <span class="info-value">{{ profileData.ownerPhone || '未设置' }}</span>
            </div>
            <div class="info-item">
              <el-icon>
                <Message />
              </el-icon>
              <span class="info-label">电子邮箱:</span>
              <span class="info-value">{{ profileData.email || '未设置' }}</span>
            </div>
            <div class="info-item">
              <el-icon>
                <Location />
              </el-icon>
              <span class="info-label">店铺地址:</span>
              <span class="info-value">{{ profileData.storeLocation || '未设置' }}</span>
            </div>
          </div>
        </el-card>

        <!-- 订阅信息卡片 -->
        <el-card class="subscription-card" shadow="hover">
          <template #header>
            <div class="card-header">
              <span>当前订阅</span>
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
              <el-button type="primary" plain size="small" @click="handleRenewSubscription">续费</el-button>
              <el-button type="success" plain size="small" @click="handleUpgradeSubscription">升级</el-button>
            </div>
          </div>

          <div v-else class="no-subscription">
            <el-empty description="暂无订阅信息" :image-size="60">
              <el-button type="primary" @click="handleSubscribe">立即订阅</el-button>
            </el-empty>
          </div>
        </el-card>
      </el-col>

      <!-- 右侧内容区域 -->
      <el-col :span="18" :xs="24">
        <el-card class="content-card" shadow="hover">
          <el-tabs v-model="activeTab" class="profile-tabs">
            <!-- 基本信息标签页 -->
            <el-tab-pane label="基本信息" name="basic">
              <el-form ref="profileForm" :model="profileData" label-width="120px" class="profile-form">
                <el-row :gutter="20">
                  <el-col :span="12">
                    <el-form-item label="昵称">
                      <el-input v-model="profileData.nickname" placeholder="请输入昵称" prefix-icon="User" />
                    </el-form-item>
                  </el-col>
                  <el-col :span="12">
                    <el-form-item label="性别">
                      <el-radio-group v-model="profileData.ownerGender" class="gender-radio">
                        <el-radio label="male">男</el-radio>
                        <el-radio label="female">女</el-radio>
                      </el-radio-group>
                    </el-form-item>
                  </el-col>
                </el-row>

                <el-row :gutter="20">
                  <el-col :span="12">
                    <el-form-item label="店铺名称">
                      <el-input v-model="profileData.storeName" placeholder="请输入店铺名称" prefix-icon="Shop" />
                    </el-form-item>
                  </el-col>
                  <el-col :span="12">
                    <el-form-item label="店主姓名">
                      <el-input v-model="profileData.ownerName" placeholder="请输入店主姓名" prefix-icon="User" />
                    </el-form-item>
                  </el-col>
                </el-row>

                <el-row :gutter="20">
                  <el-col :span="12">
                    <el-form-item label="联系电话">
                      <el-input v-model="profileData.ownerPhone" placeholder="请输入联系电话" prefix-icon="Phone" />
                    </el-form-item>
                  </el-col>
                  <el-col :span="12">
                    <el-form-item label="电子邮箱">
                      <el-input v-model="profileData.email" placeholder="请输入电子邮箱" prefix-icon="Message" />
                    </el-form-item>
                  </el-col>
                </el-row>

                <el-form-item label="店铺地址">
                  <el-input v-model="profileData.storeLocation" placeholder="请输入店铺地址" prefix-icon="Location" />
                </el-form-item>

                <el-form-item>
                  <el-button type="primary" @click="handleUpdateProfile" :loading="updating">
                    <el-icon>
                      <Check />
                    </el-icon> 保存修改
                  </el-button>
                  <el-button @click="resetForm">
                    <el-icon>
                      <RefreshRight />
                    </el-icon> 重置
                  </el-button>
                </el-form-item>
              </el-form>
            </el-tab-pane>

            <!-- 支付配置标签页 -->
            <el-tab-pane label="支付配置" name="payment">
              <el-row :gutter="20" class="payment-config-container">
                <!-- 支付宝配置 -->
                <el-col :span="12">
                  <el-card class="payment-card alipay-card" shadow="hover">
                    <template #header>
                      <div class="payment-header">
                        <span style="display: flex; align-items: center; gap: .3rem;">
                          <AliPayIcon />
                          <span>支付宝</span>
                        </span>
                        <el-switch v-model="paymentConfig.alipay.isActive" active-text="已启用" inactive-text="未启用"
                          @change="handlePaymentStatusChange('alipay')" />
                      </div>
                    </template>

                    <el-form label-position="top" class="payment-form">
                      <el-form-item label="应用ID">
                        <el-input v-model="paymentConfig.alipay.appId" placeholder="请输入应用ID" />
                      </el-form-item>
                      <el-form-item label="私钥">
                        <el-input v-model="paymentConfig.alipay.privateKey" type="password" placeholder="请输入私钥"
                          show-password />
                      </el-form-item>
                      <el-form-item label="支付宝公钥">
                        <el-input v-model="paymentConfig.alipay.alipayPublicKey" type="password" placeholder="请输入支付宝公钥"
                          show-password />
                      </el-form-item>
                      <el-form-item label="商户ID">
                        <el-input v-model="paymentConfig.alipay.sellerId" placeholder="请输入商户ID" />
                      </el-form-item>
                      <el-button type="primary" @click="handleUpdatePaymentConfig('alipay')" class="save-btn">
                        <el-icon>
                          <Check />
                        </el-icon> 保存配置
                      </el-button>
                    </el-form>
                  </el-card>
                </el-col>

                <!-- 微信支付配置 -->
                <el-col :span="12">
                  <el-card class="payment-card wechat-card" shadow="hover">
                    <template #header>
                      <div class="payment-header">
                        <span style="display: flex; align-items: center; gap: .3rem;">
                          <WechatPayIcon />
                          <span>微信支付</span>
                        </span>
                        <el-switch v-model="paymentConfig.wechat.isActive" active-text="已启用" inactive-text="未启用"
                          @change="handlePaymentStatusChange('wechat')" />
                      </div>
                    </template>

                    <el-form label-position="top" class="payment-form">
                      <el-form-item label="应用ID">
                        <el-input v-model="paymentConfig.wechat.appid" placeholder="请输入应用ID" />
                      </el-form-item>
                      <el-form-item label="商户号">
                        <el-input v-model="paymentConfig.wechat.mchid" placeholder="请输入商户号" />
                      </el-form-item>
                      <el-form-item label="商户API序列号">
                        <el-input v-model="paymentConfig.wechat.serialNo" placeholder="请输入商户API序列号" />
                      </el-form-item>
                      <el-form-item label="商户私钥">
                        <el-input v-model="paymentConfig.wechat.privateKey" type="password" placeholder="请输入商户私钥"
                          show-password />
                      </el-form-item>
                      <el-form-item label="API V3密钥">
                        <el-input v-model="paymentConfig.wechat.apiV3Key" type="password" placeholder="请输入API V3密钥"
                          show-password />
                      </el-form-item>
                      <el-button type="primary" @click="handleUpdatePaymentConfig('wechat')" class="save-btn">
                        <el-icon>
                          <Check />
                        </el-icon> 保存配置
                      </el-button>
                    </el-form>
                  </el-card>
                </el-col>
              </el-row>
            </el-tab-pane>

            <!-- 订阅管理标签页 -->
            <el-tab-pane label="订阅管理" name="subscription">
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
                        <div class="plan-card-header">
                          <h4>{{ plan.name }}</h4>
                          <el-tag v-if="plan.isRecommended" type="warning" effect="dark" size="small">推荐</el-tag>
                        </div>
                        <div class="plan-card-price">
                          <span class="price-value">¥{{ plan.price }}</span>
                          <span class="price-period">/ {{ getPeriodText(plan.period) }}</span>
                        </div>
                        <div class="plan-card-features">
                          <div v-for="(feature, index) in getFeaturesList(plan.features)" :key="index"
                            class="feature-item">
                            <el-icon>
                              <Check />
                            </el-icon>
                            <span>{{ feature }}</span>
                          </div>
                        </div>
                        <el-button type="primary" class="subscribe-btn" :disabled="isCurrentPlan(plan.id)"
                          @click="showSubscriptionDialog(plan)">
                          {{ isCurrentPlan(plan.id) ? '当前套餐' : '选择套餐' }}
                        </el-button>
                      </el-card>
                    </el-col>
                  </el-row>
                </div>
              </div>
            </el-tab-pane>
          </el-tabs>
        </el-card>
      </el-col>
    </el-row>

    <!-- 订阅套餐付款弹窗 -->
    <subscription-payment v-model:visible="subscriptionDialogVisible" :plan-data="selectedPlan"
      @payment-success="handlePaymentSuccess" @payment-cancel="handlePaymentCancel" />
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { getProfile, updateProfile, updatePaymentConfig } from '@/api/system/profile'
import { formatDate as formatDateUtil } from '@/utils/index'
import { getAllPlans } from '@/api/system/subscription'
import useUserStore from '@/store/modules/user'
import SubscriptionPayment from '@/components/SubscriptionPayment/index.vue';
import WechatPayIcon from '../../icons/wechatPayIcon.vue'
import AliPayIcon from '../../icons/aliPayIcon.vue'

const activeTab = ref('basic')
const profileData = ref({})
const updating = ref(false)

// 订阅套餐相关
const subscriptionDialogVisible = ref(false);
const selectedPlan = ref({});

// 支付配置数据
const paymentConfig = ref({
  alipay: {
    appId: '',
    privateKey: '',
    alipayPublicKey: '',
    sellerId: '',
    isActive: false
  },
  wechat: {
    appid: '',
    mchid: '',
    serialNo: '',
    privateKey: '',
    wechatCert: '',
    apiV3Key: '',
    isActive: false
  }
})

// 订阅数据
const subscriptionData = ref({
  plan: null,
  expiryDate: null,
  autoRenew: false,
  status: 'active' // active, expired, pending
})

// 可用套餐列表
const availablePlans = ref([])

// 显示订阅套餐弹窗
const showSubscriptionDialog = async (plan) => {
  selectedPlan.value = plan;
  subscriptionDialogVisible.value = true;

};

// 处理支付成功
const handlePaymentSuccess = (paymentInfo) => {
  console.log('支付成功', paymentInfo);
  // 这里可以添加支付成功后的逻辑，比如更新用户订阅状态等
};

// 处理支付取消
const handlePaymentCancel = () => {
  console.log('支付已取消');
  subscriptionDialogVisible.value = false;
};

// 获取个人信息和配置
onMounted(async () => {
  getAllPlans().then(res => {
    availablePlans.value = res;
  });

  useUserStore().getInfo().then(res => {
    profileData.value = res.user;
  });
})

// 更新个人信息
const handleUpdateProfile = async () => {
  try {
    updating.value = true
    await updateProfile(profileData.value)
    ElMessage.success('个人信息更新成功')
  } catch (error) {
    ElMessage.error('更新失败：' + (error.message || '未知错误'))
  } finally {
    updating.value = false
  }
}

// 重置表单
const resetForm = () => {
  ElMessageBox.confirm('确定要重置表单吗？所有未保存的修改将丢失。', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(async () => {
    try {
      const res = await getProfile()
      profileData.value = res.data
      ElMessage.success('表单已重置')
    } catch (error) {
      console.error(error)
    }
  }).catch(() => { })
}

// 更新支付配置
const handleUpdatePaymentConfig = async (type) => {
  try {
    const config = type === 'alipay' ? paymentConfig.value.alipay : paymentConfig.value.wechat
    await updatePaymentConfig({ type, config })
    ElMessage.success(`${type === 'alipay' ? '支付宝' : '微信支付'}配置更新成功`)
  } catch (error) {
    ElMessage.error('配置更新失败：' + (error.message || '未知错误'))
  }
}

// 处理支付状态变更
const handlePaymentStatusChange = async (type) => {
  try {
    const config = type === 'alipay' ? paymentConfig.value.alipay : paymentConfig.value.wechat
    await updatePaymentConfig({ type, config: { ...config, isActive: config.isActive } })
    ElMessage.success(`${type === 'alipay' ? '支付宝' : '微信支付'}状态已${config.isActive ? '启用' : '禁用'}`)
  } catch (error) {
    // 恢复状态
    if (type === 'alipay') {
      paymentConfig.value.alipay.isActive = !paymentConfig.value.alipay.isActive
    } else {
      paymentConfig.value.wechat.isActive = !paymentConfig.value.wechat.isActive
    }
    ElMessage.error('状态更新失败：' + (error.message || '未知错误'))
  }
}

// 处理头像上传成功
const handleAvatarSuccess = (response) => {
  profileData.value.avatar = response.url
}

// 敏感信息脱敏
const maskSensitiveInfo = (str) => {
  if (!str) return ''
  if (str.length <= 8) return '*'.repeat(str.length)
  return str.substring(0, 4) + '*'.repeat(str.length - 8) + str.substring(str.length - 4)
}

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

// 判断是否为当前套餐
const isCurrentPlan = (planId) => {
  return subscriptionData.value.plan && subscriptionData.value.plan.id === planId
}

// 处理自动续费变更
const handleAutoRenewChange = async () => {
  try {
    // 实际项目中应调用API更新自动续费状态
    // await updateSubscription({ autoRenew: subscriptionData.value.autoRenew })
    ElMessage.success(`自动续费已${subscriptionData.value.autoRenew ? '开启' : '关闭'}`)
  } catch (error) {
    subscriptionData.value.autoRenew = !subscriptionData.value.autoRenew
    ElMessage.error('设置失败：' + (error.message || '未知错误'))
  }
}

// 处理续费
const handleRenewSubscription = () => {
  ElMessageBox.confirm('确定要续费当前套餐吗？', '续费确认', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'info'
  }).then(() => {
    // 实际项目中应跳转到支付页面或调用续费API
    ElMessage.success('续费成功，订阅已延长')
    // 模拟更新到期时间
    subscriptionData.value.expiryDate = Date.now() + 30 * 24 * 60 * 60 * 1000
  }).catch(() => { })
}

// 处理升级
const handleUpgradeSubscription = () => {
  activeTab.value = 'subscription'
}

// 处理订阅
const handleSubscribe = () => {
  activeTab.value = 'subscription'
}

// 处理选择套餐
const handleSelectPlan = (plan) => {
  ElMessageBox.confirm(`确定要选择「${plan.name}」套餐吗？`, '套餐选择', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'info'
  }).then(() => {
    // 实际项目中应跳转到支付页面或调用订阅API
    ElMessage.success(`已成功订阅「${plan.name}」套餐`)
    // 模拟更新订阅信息
    subscriptionData.value.plan = plan
    subscriptionData.value.expiryDate = Date.now() + 30 * 24 * 60 * 60 * 1000
  }).catch(() => { })
}
</script>

<style scoped>
/* 整体容器样式 */
.app-container {
  width: 100%;
  height: 100%;
  padding: 1rem;
  background-color: var(--el-bg-color-page);
}

/* 页面标题区域 */
.profile-header {
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 1px solid #ebeef5;
}

.profile-title h2 {
  font-size: 24px;
  font-weight: 600;
  margin-bottom: 8px;
}

.profile-title p {
  font-size: 14px;
  color: var(--el-text-color-regular);
  margin: 0;
}

/* 左侧个人信息卡片 */
.profile-card {
  margin-bottom: 20px;
  border-radius: 8px;
  transition: all 0.3s;
}

.profile-avatar-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 20px 0;
}

.avatar-wrapper {
  position: relative;
  width: 120px;
  height: 120px;
  border-radius: 50%;
  overflow: hidden;
  margin-bottom: 16px;
  border: 4px solid #fff;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
  cursor: pointer;
}

.avatar {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.avatar-icon {
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  font-size: 48px;
  background-color: #e6f7ff;
  color: #1890ff;
}

.avatar-mask {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  color: #fff;
  opacity: 0;
  transition: opacity 0.3s;
}

.avatar-wrapper:hover .avatar-mask {
  opacity: 1;
}

.avatar-mask .el-icon {
  font-size: 24px;
  margin-bottom: 4px;
}

.profile-name {
  font-size: 18px;
  font-weight: 600;
  margin: 0 0 4px;
  color: var(--el-text-color-primary);
}

.profile-store {
  font-size: 14px;
  color: var(--el-text-color-secondary);
  margin: 0 0 16px;
}

.profile-info-list {
  padding: 0 16px;
}

.info-item {
  display: flex;
  align-items: center;
  padding: 8px 0;
  border-bottom: 1px solid #f0f0f0;
}

.info-item:last-child {
  border-bottom: none;
}

.info-item .el-icon {
  font-size: 16px;
  color: #409eff;
  margin-right: 8px;
}

.info-label {
  font-weight: 500;
  color: var(--el-text-color-regular);
  margin-right: 8px;
}

.info-value {
  color: var(--el-text-color-primary);
  flex: 1;
  text-align: right;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

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

/* 右侧内容区域 */
.content-card {
  border-radius: 8px;
  height: 100%;
}

.profile-tabs {
  padding: 0 16px;
}

.profile-form {
  padding: 16px 0;
}

.gender-radio {
  width: 100%;
  display: flex;
  justify-content: space-around;
}

/* 支付配置区域 */
.payment-config-container {
  padding: 16px 0;
}

.payment-card {
  height: 100%;
  border-radius: 8px;
  transition: all 0.3s;
}

.payment-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.payment-logo {
  height: 32px;
  object-fit: contain;
}

.payment-form {
  padding: 16px 0;
}

.save-btn {
  width: 100%;
  margin-top: 16px;
}

/* 订阅管理区域 */
.subscription-management {
  padding: 16px 0;
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
