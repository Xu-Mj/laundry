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
        <SubscriptionCard 
          title="当前订阅"
          :subscription-data="subscriptionData"
          @renew="handleRenewSubscription"
          @upgrade="handleUpgradeSubscription"
          @subscribe="handleSubscribe"
        />
        
        <!-- 短信订阅信息卡片 -->
        <SmsSubscriptionCard 
          title="短信订阅"
          :sms-subscription-data="smsSubscriptionData"
          @renew="handleRenewSmsSubscription"
          @upgrade="handleUpgradeSmsSubscription"
          @subscribe="handleSmsSubscribe"
          class="mt-20"
        />
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
              <SubscriptionManagement :subscription-data="subscriptionData" />
            </el-tab-pane>
            <el-tab-pane label="短信订阅管理" name="subscription-sms">
              <SmsSubscriptionManagement :subscription-data="subscriptionData" />
            </el-tab-pane>
          </el-tabs>
        </el-card>
      </el-col>
    </el-row>

    <!-- 订阅套餐付款弹窗 -->
    <subscription-payment v-model:visible="subscriptionDialogVisible" :plan-data="selectedPlan"
      @payment-success="handlePaymentSuccess" @payment-cancel="handlePaymentCancel" />

    <!-- 订阅成功贺卡 -->
    <subscription-congrats v-model:visible="congratsVisible" :plan-data="selectedPlan"
      :expiry-date="subscriptionExpiryDate" @confirmed="handleCongratsConfirmed" />
  </div>
</template>

<script setup>
import { ElMessage, ElMessageBox } from 'element-plus';
import { getProfile, updateProfile, updatePaymentConfig } from '@/api/system/profile';
import { formatDate as formatDateUtil } from '@/utils/index';
import useUserStore from '@/store/modules/user';
import SubscriptionManagement from '@/components/SubscriptionManagement/index.vue';
import SmsSubscriptionManagement from '@/components/SmsSubscriptionManagement/index.vue';
import SubscriptionCard from '@/components/SubscriptionCard/index.vue';
import SmsSubscriptionCard from '@/components/SmsSubscriptionCard/index.vue';
import WechatPayIcon from '@/views/icons/wechatPayIcon.vue';
import AliPayIcon from '@/views/icons/aliPayIcon.vue';
import { getSubscription, saveSubscription } from '@/api/system/subscription';

const activeTab = ref('basic')
const profileData = ref({})
const updating = ref(false)

// 订阅套餐相关
const subscriptionDialogVisible = ref(false);
const congratsVisible = ref(false);
const selectedPlan = ref({});
const subscriptionExpiryDate = ref(null);

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
const subscriptionData = ref(useUserStore().sub.subscription)

// 短信订阅数据
const smsSubscriptionData = ref({
  plan: null,
  expiryDate: null,
  autoRenew: false,
  totalSmsCount: 0,
  usedSmsCount: 0,
  remainingSmsCount: 0,
  status: 'Active'
})

// 可用套餐列表
const availablePlans = ref([])

// 处理支付成功
const handlePaymentSuccess = (paymentInfo) => {
  console.log('支付成功', selectedPlan.value);
  // 更新订阅状态
  // subscriptionData.value.plan = selectedPlan.value;
  // subscriptionExpiryDate.value = Date.now() + 30 * 24 * 60 * 60 * 1000; // 设置30天后到期
  // subscriptionData.value.expiryDate = subscriptionExpiryDate.value;
  // subscriptionData.value.status = 'active';

  // get subscription
  getSubscription(useUserStore().user.id, paymentInfo.planId, paymentInfo.subscriptionId).then(res => {
    saveSubscription(res.subscription, res.plan).catch(err => { })
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
  // activeTab.value = 'basic'; // 可选：切换回基本信息标签页
};

// 处理支付取消
const handlePaymentCancel = () => {
  console.log('支付已取消');
  subscriptionDialogVisible.value = false;
};

// 获取个人信息和配置
onMounted(async () => {
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

// 处理短信续费
const handleRenewSmsSubscription = () => {
  ElMessageBox.confirm('确定要续费当前短信套餐吗？', '续费确认', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'info'
  }).then(() => {
    // 实际项目中应跳转到支付页面或调用续费API
    ElMessage.success('续费成功，短信订阅已延长')
    // 模拟更新到期时间
    smsSubscriptionData.value.expiryDate = Date.now() + 30 * 24 * 60 * 60 * 1000
  }).catch(() => { })
}

// 处理短信升级
const handleUpgradeSmsSubscription = () => {
  activeTab.value = 'subscription-sms'
}

// 处理短信订阅
const handleSmsSubscribe = () => {
  activeTab.value = 'subscription-sms'
}
</script>

<style scoped>
/* 工具类 */
.mt-20 {
  margin-top: 20px;
}

/* 整体容器样式 */
.app-container {
  width: 100%;
  height: 100%;
  padding: 1rem;
  background-color: var(--el-bg-color-page);
  overflow-y: auto;
}

.app-container::-webkit-scrollbar-button {
  display: none;
}

.app-container::-webkit-scrollbar-thumb {
  display: none;
}

.app-container:hover::-webkit-scrollbar-thumb {
  display: block;
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

</style>

