<template>
  <el-card class="payment-card wechat-card" shadow="hover">
    <template #header>
      <div class="payment-header">
        <span style="display: flex; align-items: center; gap: .3rem;">
          <WechatPayIcon />
          <span>微信支付</span>
        </span>
        <el-switch v-model="paymentConfig.isActive" active-text="已启用" inactive-text="未启用"
          @change="handlePaymentStatusChange" />
      </div>
    </template>

    <el-form label-position="top" class="payment-form">
      <el-form-item label="应用ID">
        <el-input v-model="paymentConfig.appId" placeholder="请输入应用ID" />
      </el-form-item>
      <el-form-item label="商户号">
        <el-input v-model="paymentConfig.mchid" placeholder="请输入商户号" />
      </el-form-item>
      <el-form-item label="商户API序列号">
        <el-input v-model="paymentConfig.serialNo" placeholder="请输入商户API序列号" />
      </el-form-item>
      <el-form-item label="商户私钥">
        <el-input v-model="paymentConfig.privateKey" type="password" placeholder="请输入商户私钥"
          show-password readonly />
      </el-form-item>
      <el-form-item label="商户私钥文件">
        <el-upload class="upload-demo" :auto-upload="false" :limit="1"
          :on-change="handleWechatPrivateKeyFileChange" :file-list="wechatPrivateKeyFileList">
          <el-button type="primary">选择私钥文件</el-button>
          <template #tip>
            <div class="el-upload__tip">上传商户私钥文件，支持.pem格式</div>
          </template>
        </el-upload>
      </el-form-item>
      <el-form-item label="API V3密钥">
        <el-input v-model="paymentConfig.apiV3Key" type="password" placeholder="请输入API V3密钥"
          show-password />
      </el-form-item>
      <el-form-item label="API证书文件">
        <el-upload class="upload-demo" :auto-upload="false" :limit="1"
          :on-change="handleWechatCertFileChange" :file-list="wechatCertFileList">
          <el-button type="primary">选择证书文件</el-button>
          <template #tip>
            <div class="el-upload__tip">上传API证书文件，支持.pem格式</div>
          </template>
        </el-upload>
      </el-form-item>
      <el-button type="primary" @click="handleUpdatePaymentConfig" class="save-btn">
        <el-icon>
          <Check />
        </el-icon> 保存配置
      </el-button>
    </el-form>
  </el-card>
</template>

<script setup>
import { ref, defineProps, defineEmits, watch } from 'vue';
import { ElMessage } from 'element-plus';
import { updateWechatConfig, uploadFile } from '@/api/system/profile';
import WechatPayIcon from '@/views/icons/wechatPayIcon.vue';

const props = defineProps({
  config: {
    type: Object,
    required: true
  }
});

const emit = defineEmits(['config-updated', 'status-changed']);

// 支付配置数据
const paymentConfig = ref({
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
  storeId: 0
});

// 文件上传相关
const wechatPrivateKeyFileList = ref([]);
const wechatCertFileList = ref([]);

// 监听props变化，更新本地数据
watch(() => props.config, (newConfig) => {
  if (newConfig) {
    paymentConfig.value = {
      appId: newConfig.appId || '',
      mchid: newConfig.mchid || '',
      serialNo: newConfig.serialNo || '',
      privateKey: newConfig.privateKey || '',
      apiV3Key: newConfig.apiV3Key || '',
      spAppid: newConfig.spAppid || '',
      spMchid: newConfig.spMchid || '',
      mchKey: newConfig.mchKey || '',
      apiclientKey: newConfig.apiclientKey || '',
      apiclientCert: newConfig.apiclientCert || '',
      isActive: newConfig.isActive || false,
      storeId: newConfig.storeId || 0
    };

    // 如果有私钥文件，更新文件列表
    if (newConfig.apiclientKey) {
      const fileName = newConfig.apiclientKey.split('/').pop();
      wechatPrivateKeyFileList.value = [{ name: fileName, url: newConfig.apiclientKey }];
    }
    
    // 如果有证书文件，更新文件列表
    if (newConfig.apiclientCert) {
      const fileName = newConfig.apiclientCert.split('/').pop();
      wechatCertFileList.value = [{ name: fileName, url: newConfig.apiclientCert }];
    }
  }
}, { immediate: true, deep: true });

// 处理微信私钥文件上传
const handleWechatPrivateKeyFileChange = async (file) => {
  if (file && file.raw) {
    try {
      // 读取文件内容
      const reader = new FileReader();
      reader.readAsArrayBuffer(file.raw);
      reader.onload = async (e) => {
        const data = new Uint8Array(e.target.result);
        // 上传文件
        const result = await uploadFile(file.name, Array.from(data));
        // 保存文件路径
        paymentConfig.value.privateKey = result;
        ElMessage.success('微信私钥文件上传成功');
      };
    } catch (error) {
      ElMessage.error('微信私钥文件上传失败：' + error.message);
    }
  }
};

// 处理微信证书文件上传
const handleWechatCertFileChange = async (file) => {
  if (file && file.raw) {
    try {
      // 读取文件内容
      const reader = new FileReader();
      reader.readAsArrayBuffer(file.raw);
      reader.onload = async (e) => {
        const data = new Uint8Array(e.target.result);
        // 上传文件
        const result = await uploadFile(file.name, Array.from(data));
        // 保存文件路径
        paymentConfig.value.apiclientCert = result;
        ElMessage.success('微信证书文件上传成功');
      };
    } catch (error) {
      ElMessage.error('微信证书文件上传失败：' + error.message);
    }
  }
};

// 更新支付配置
const handleUpdatePaymentConfig = async () => {
  try {
    // 确保storeId存在
    if (!paymentConfig.value.storeId || paymentConfig.value.storeId <= 0) {
      ElMessage.error('商家ID不能为空，请先完善店铺信息');
      return;
    }

    // 转换字段名称以匹配后端结构
    const wechatConfig = {
      storeId: paymentConfig.value.storeId,
      appId: paymentConfig.value.appId,
      mchid: paymentConfig.value.mchid,
      serialNo: paymentConfig.value.serialNo || '',
      privateKey: paymentConfig.value.privateKey || '',
      apiV3Key: paymentConfig.value.apiV3Key || '',
      spAppid: paymentConfig.value.spAppid || '',
      spMchid: paymentConfig.value.spMchid || '',
      mchKey: paymentConfig.value.mchKey || '',
      apiclientKey: paymentConfig.value.apiclientKey || '',
      apiclientCert: paymentConfig.value.apiclientCert || '',
      isActive: paymentConfig.value.isActive
    };

    await updateWechatConfig(wechatConfig);
    ElMessage.success('微信支付配置更新成功');
    emit('config-updated', 'wechat', wechatConfig);
  } catch (error) {
    ElMessage.error('配置更新失败：' + (error.message || '未知错误'));
  }
};

// 处理支付状态变更
const handlePaymentStatusChange = async () => {
  try {
    // 确保storeId存在
    if (!paymentConfig.value.storeId || paymentConfig.value.storeId <= 0) {
      ElMessage.error('商家ID不能为空，请先完善店铺信息');
      // 恢复状态
      paymentConfig.value.isActive = !paymentConfig.value.isActive;
      return;
    }

    // 转换字段名称以匹配后端结构
    const wechatConfig = {
      storeId: paymentConfig.value.storeId,
      appId: paymentConfig.value.appId,
      mchid: paymentConfig.value.mchid,
      serialNo: paymentConfig.value.serialNo || '',
      privateKey: paymentConfig.value.privateKey || '',
      apiV3Key: paymentConfig.value.apiV3Key || '',
      spAppid: paymentConfig.value.spAppid || '',
      spMchid: paymentConfig.value.spMchid || '',
      mchKey: paymentConfig.value.mchKey || '',
      apiclientKey: paymentConfig.value.apiclientKey || '',
      apiclientCert: paymentConfig.value.apiclientCert || '',
      isActive: paymentConfig.value.isActive
    };

    await updateWechatConfig(wechatConfig);
    ElMessage.success(`微信支付状态已${paymentConfig.value.isActive ? '启用' : '禁用'}`);
    emit('status-changed', 'wechat', paymentConfig.value.isActive);
  } catch (error) {
    // 恢复状态
    paymentConfig.value.isActive = !paymentConfig.value.isActive;
    ElMessage.error('状态更新失败：' + (error.message || '未知错误'));
  }
};
</script>

<style scoped>
.payment-card {
  height: 100%;
  border-radius: 8px;
  transition: all 0.3s;
}

.wechat-card {
  border-top: 4px solid #07c160;
}

.payment-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 16px;
  font-weight: 600;
}

.payment-form {
  margin-top: 16px;
}

.save-btn {
  width: 100%;
  margin-top: 16px;
}

.el-upload__tip {
  font-size: 12px;
  color: #909399;
  margin-top: 4px;
}
</style>