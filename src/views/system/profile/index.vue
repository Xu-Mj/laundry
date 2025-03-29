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
            <!-- <el-upload class="avatar-uploader" :show-file-list="false" :on-success="handleAvatarSuccess"> -->
              <div class="avatar-wrapper">
                <img v-if="profileData.avatar" :src="profileData.avatar" class="avatar">
                <el-icon v-else class="avatar-icon">
                  <UserFilled />
                </el-icon>
                <!-- <div class="avatar-mask">
                  <el-icon>
                    <Camera />
                  </el-icon>
                  <span>更换头像</span>
                </div> -->
              </div>
            <!-- </el-upload> -->
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
        <SubscriptionCard title="当前订阅" :subscription-data="subscriptionData" :subscriptions="allSubscriptions"
          @renew="handleRenewSubscription" @upgrade="handleUpgradeSubscription" @subscribe="handleSubscribe" />

        <!-- 短信订阅信息卡片 -->
        <SmsSubscriptionCard title="短信订阅" :sms-subscription-data="smsSubscriptionData"
          @renew="handleRenewSmsSubscription" @upgrade="handleUpgradeSmsSubscription" @subscribe="handleSmsSubscribe"
          class="mt-20" />
      </el-col>

      <!-- 右侧内容区域 -->
      <el-col :span="18" :xs="24">
        <el-card class="content-card" shadow="hover">
          <el-tabs v-model="activeTab" class="profile-tabs">
            <!-- 基本信息标签页 -->
            <el-tab-pane label="基本信息" name="basic">
              <ProfileForm :profile-data="profileData" :user-store="userStore" :isGuest="isGuest"
                @update:profile-data="profileData = $event" @profile-updated="handleProfileUpdated" />
            </el-tab-pane>

            <!-- 支付配置标签页 -->
            <el-tab-pane label="支付配置" name="payment">
              <el-row :gutter="20" class="payment-config-container">
                <!-- 支付宝配置 -->
                <el-col :span="12">
                  <AlipayConfig :config="paymentConfig.alipay" @config-updated="handleConfigUpdated"
                    @status-changed="handleStatusChanged" />
                </el-col>

                <!-- 微信支付配置 -->
                <el-col :span="12">
                  <WechatPayConfig :config="paymentConfig.wechat" @config-updated="handleConfigUpdated"
                    @status-changed="handleStatusChanged" />
                </el-col>
              </el-row>
            </el-tab-pane>

            <!-- 订阅管理标签页 -->
            <el-tab-pane label="订阅管理" name="subscription">
              <SubscriptionManagement :subscription-data="subscriptionData" :all-subscriptions="allSubscriptions" />
            </el-tab-pane>
            <el-tab-pane label="短信订阅管理" name="subscription-sms">
              <SmsSubscriptionManagement :subscription-data="subscriptionData" />
            </el-tab-pane>
          </el-tabs>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup>
import { ElMessageBox } from 'element-plus';
import { getAlipayConfig, getWechatConfig } from '@/api/system/profile';
import useUserStore from '@/store/modules/user';
import SubscriptionManagement from '@/components/SubscriptionManagement/index.vue';
import SmsSubscriptionManagement from '@/components/SmsSubscriptionManagement/index.vue';
import SubscriptionCard from '@/components/SubscriptionCard/index.vue';
import SmsSubscriptionCard from '@/components/SmsSubscriptionCard/index.vue';
import AlipayConfig from '@/components/AlipayConfig/index.vue';
import WechatPayConfig from '@/components/WechatPayConfig/index.vue';
import ProfileForm from '@/components/ProfileForm/index.vue';
import { useRoute } from 'vue-router';

const { proxy } = getCurrentInstance();
const userStore = useUserStore();
const route = useRoute();

const activeTab = ref('basic')
const profileData = ref(userStore.user || {})
const isGuest = userStore.isGuest;
// 支付配置数据
const paymentConfig = ref({
  alipay: {
    appId: '',
    privateKey: '',
    privateKeyFile: null,
    alipayPublicKey: '',
    alipayPublicKeyFile: null,
    isActive: false,
    isSandbox: false,
    storeId: useUserStore.id || 0
  },
  wechat: {
    appId: '',
    mchid: '',
    serialNo: '',
    privateKey: '',
    apiV3Key: '',
    spAppid: '',
    spMchid: '',
    mchKey: '',
    apiclientKey: '',
    apiclientCert: '',
    isActive: false,
    storeId: useUserStore.id || 0
  }
})

// 文件上传相关
const privateKeyFileList = ref([])
const publicKeyFileList = ref([])
const wechatPrivateKeyFileList = ref([])
const wechatCertFileList = ref([])

// 处理配置更新事件
const handleConfigUpdated = (type, config) => {
  if (type === 'alipay') {
    paymentConfig.value.alipay = config;
    proxy.notify.success('支付宝配置已更新');
  } else if (type === 'wechat') {
    paymentConfig.value.wechat = config;
    proxy.notify.success('微信支付配置已更新');
  }
}

// 处理状态变更事件
const handleStatusChanged = (type, isActive) => {
  if (type === 'alipay') {
    paymentConfig.value.alipay.isActive = isActive;
    proxy.notify.success(`支付宝支付已${isActive ? '启用' : '禁用'}`);
  } else if (type === 'wechat') {
    paymentConfig.value.wechat.isActive = isActive;
    proxy.notify.success(`微信支付已${isActive ? '启用' : '禁用'}`);
  }
}

// 订阅数据
const subscriptionData = ref(userStore.sub.subscription)

// 短信订阅数据
const smsSubscriptionData = ref(userStore.sub.smsSub || {})
const allSubscriptions = ref(userStore.sub.allSubscriptions);

// 获取个人信息和配置
onMounted(async () => {
  // 检查URL参数，设置激活的标签页
  if (route.query.tab) {
    // 处理从login.vue传递过来的tab参数
    const tabParam = route.query.tab;
    if (tabParam === 'subscription') {
      activeTab.value = 'subscription';
    } else if (tabParam === 'sms') {
      activeTab.value = 'subscription-sms';
    }
  }

  // 获取用户的商店ID
  const storeId = userStore.id || 0;
  paymentConfig.value.alipay.storeId = storeId;
  paymentConfig.value.wechat.storeId = storeId;

  // 获取支付宝配置
  if (storeId > 0) {
    getAlipayConfig(storeId).then(config => {
      if (config) {
        paymentConfig.value.alipay = {
          appId: config.appId || '',
          privateKey: config.privateKey || '',
          alipayPublicKey: config.alipayPublicKey || '',
          isActive: config.isActive || false,
          isSandbox: config.isSandbox || false,
        };

        // 如果有私钥文件，更新文件列表
        if (config.privateKeyFile) {
          const fileName = config.privateKeyFile.split('/').pop();
          privateKeyFileList.value = [{ name: fileName, url: config.privateKeyFile }];
        }

        // 如果有公钥文件，更新文件列表
        if (config.alipayPublicKeyFile) {
          const fileName = config.alipayPublicKeyFile.split('/').pop();
          publicKeyFileList.value = [{ name: fileName, url: config.alipayPublicKeyFile }];
        }
      }
    }).catch(error => {
      console.error('获取支付宝配置失败:', error);
    });

    // 获取微信支付配置
    getWechatConfig(storeId).then(config => {
      if (config) {
        paymentConfig.value.wechat = {
          appId: config.appId || '',
          mchid: config.mchid || '',
          serialNo: config.serialNo || '',
          privateKey: config.privateKey || '',
          apiV3Key: config.apiV3Key || '',
          spAppid: config.spAppid || '',
          spMchid: config.spMchid || '',
          mchKey: config.mchKey || '',
          apiclientKey: config.apiclientKey || '',
          apiclientCert: config.apiclientCert || '',
          isActive: config.isActive || false,
        };

        // 如果有私钥文件，更新文件列表
        if (config.apiclientKey) {
          const fileName = config.apiclientKey.split('/').pop();
          wechatPrivateKeyFileList.value = [{ name: fileName, url: config.apiclientKey }];
        }

        // 如果有证书文件，更新文件列表
        if (config.apiclientCert) {
          const fileName = config.apiclientCert.split('/').pop();
          wechatCertFileList.value = [{ name: fileName, url: config.apiclientCert }];
        }
      }
    }).catch(error => {
      console.error('获取微信支付配置失败:', error);
    });
  }
})

// 处理ProfileForm组件更新个人信息成功事件
const handleProfileUpdated = (user) => {
  proxy.notify.success('个人信息更新成功');
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

// 处理续费
const handleRenewSubscription = () => {
  ElMessageBox.confirm('确定要续费当前套餐吗？', '续费确认', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'info'
  }).then(() => {
    // 实际项目中应跳转到支付页面或调用续费API
    proxy.notify.success('续费成功，订阅已延长')
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
    proxy.notify.success('续费成功，短信订阅已延长')
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
