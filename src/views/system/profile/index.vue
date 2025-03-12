<template>
  <div class="app-container">
    <el-card>
      <el-tabs v-model="activeTab">
        <el-tab-pane label="基本信息" name="basic">
          <el-form ref="profileForm" :model="profileData" label-width="120px">
            <el-form-item label="头像">
              <el-upload
                class="avatar-uploader"
                :show-file-list="false"
                :on-success="handleAvatarSuccess"
              >
                <img v-if="profileData.avatar" :src="profileData.avatar" class="avatar">
                <el-icon v-else><Plus /></el-icon>
              </el-upload>
            </el-form-item>
            <el-form-item label="昵称">
              <el-input v-model="profileData.nickname" />
            </el-form-item>
            <el-form-item label="店铺名称">
              <el-input v-model="profileData.storeName" />
            </el-form-item>
            <el-form-item label="店铺地址">
              <el-input v-model="profileData.storeLocation" />
            </el-form-item>
            <el-form-item label="店主姓名">
              <el-input v-model="profileData.ownerName" />
            </el-form-item>
            <el-form-item label="联系电话">
              <el-input v-model="profileData.ownerPhone" />
            </el-form-item>
            <el-form-item label="电子邮箱">
              <el-input v-model="profileData.email" />
            </el-form-item>
            <el-form-item label="性别">
              <el-select v-model="profileData.ownerGender">
                <el-option label="男" value="male" />
                <el-option label="女" value="female" />
              </el-select>
            </el-form-item>
            <el-form-item>
              <el-button type="primary" @click="handleUpdateProfile">保存修改</el-button>
            </el-form-item>
          </el-form>
        </el-tab-pane>

        <el-tab-pane label="支付配置" name="payment">
          <el-descriptions :column="1" border :label-width="120">
            <el-descriptions-item label="支付宝配置">
              <el-descriptions :column="1" border :label-width="120">
                <el-descriptions-item label="应用ID">{{ paymentConfig.alipay.appId }}</el-descriptions-item>
                <el-descriptions-item label="私钥">{{ maskSensitiveInfo(paymentConfig.alipay.privateKey) }}</el-descriptions-item>
                <el-descriptions-item label="支付宝公钥">{{ maskSensitiveInfo(paymentConfig.alipay.alipayPublicKey) }}</el-descriptions-item>
                <el-descriptions-item label="商户ID">{{ paymentConfig.alipay.sellerId }}</el-descriptions-item>
                <el-descriptions-item label="状态">
                  <el-tag :type="paymentConfig.alipay.isActive ? 'success' : 'danger'">
                    {{ paymentConfig.alipay.isActive ? '已启用' : '未启用' }}
                  </el-tag>
                </el-descriptions-item>
              </el-descriptions>
            </el-descriptions-item>

            <el-descriptions-item label="微信支付配置">
              <el-descriptions :column="1" border :label-width="120">
                <el-descriptions-item label="应用ID">{{ paymentConfig.wechat.appid }}</el-descriptions-item>
                <el-descriptions-item label="商户号">{{ paymentConfig.wechat.mchid }}</el-descriptions-item>
                <el-descriptions-item label="商户API序列号">{{ paymentConfig.wechat.serialNo }}</el-descriptions-item>
                <el-descriptions-item label="商户私钥">{{ maskSensitiveInfo(paymentConfig.wechat.privateKey) }}</el-descriptions-item>
                <el-descriptions-item label="API V3密钥">{{ maskSensitiveInfo(paymentConfig.wechat.apiV3Key) }}</el-descriptions-item>
                <el-descriptions-item label="状态">
                  <el-tag :type="paymentConfig.wechat.isActive ? 'success' : 'danger'">
                    {{ paymentConfig.wechat.isActive ? '已启用' : '未启用' }}
                  </el-tag>
                </el-descriptions-item>
              </el-descriptions>
            </el-descriptions-item>
          </el-descriptions>
        </el-tab-pane>
      </el-tabs>
    </el-card>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { getProfile, updateProfile } from '@/api/system/profile'

const activeTab = ref('basic')
const profileData = ref({})
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

onMounted(async () => {
  try {
    const res = await getProfile()
    profileData.value = res.data
    if (res.data.paymentConfig) {
      paymentConfig.value = res.data.paymentConfig
    }
  } catch (error) {
    // ElMessage.error('获取个人信息失败')
    console.error(error)
  }
})

const handleUpdateProfile = async () => {
  try {
    await updateProfile(profileData.value)
    ElMessage.success('更新成功')
  } catch (error) {
    ElMessage.error('更新失败')
  }
}

const maskSensitiveInfo = (str) => {
  if (!str) return ''
  if (str.length <= 8) return '*'.repeat(str.length)
  return str.substring(0, 4) + '*'.repeat(str.length - 8) + str.substring(str.length - 4)
}

const handleAvatarSuccess = (response) => {
  profileData.value.avatar = response.url
}
</script>

<style scoped>
.avatar-uploader {
  width: 178px;
  height: 178px;
  border: 1px dashed var(--el-border-color);
  border-radius: 6px;
  cursor: pointer;
  overflow: hidden;
}
.avatar {
  width: 178px;
  height: 178px;
  display: block;
}
:deep(.el-descriptions__label) {
  width: 100px;
  justify-content: flex-end;
}
</style>
