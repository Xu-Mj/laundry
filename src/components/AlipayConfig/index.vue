<template>
  <el-card class="payment-card alipay-card" shadow="hover">
    <template #header>
      <div class="payment-header">
        <span style="display: flex; align-items: center; gap: .3rem;">
          <AliPayIcon />
          <span>支付宝</span>
        </span>
        <el-switch v-model="paymentConfig.isActive" active-text="已启用" inactive-text="未启用"
          @change="handlePaymentStatusChange" />
      </div>
    </template>

    <el-form label-position="top" class="payment-form">
      <el-form-item label="应用ID">
        <el-input v-model="paymentConfig.appId" placeholder="请输入应用ID" />
      </el-form-item>
      <el-form-item label="私钥">
        <el-input v-model="paymentConfig.privateKey" type="text" placeholder="请上传私钥文件" readonly />
      </el-form-item>
      <el-form-item label="私钥文件">
        <el-upload class="upload-demo" :auto-upload="false" :limit="1" :on-change="handlePrivateKeyFileChange"
          :file-list="privateKeyFileList">
          <el-button type="primary">选择私钥文件</el-button>
          <template #tip>
            <div class="el-upload__tip">上传私钥文件，支持.pem格式</div>
          </template>
        </el-upload>
      </el-form-item>
      <el-form-item label="支付宝公钥">
        <el-input v-model="paymentConfig.alipayPublicKey" type="text" placeholder="请上传支付宝公钥文件" readonly />
      </el-form-item>
      <el-form-item label="支付宝公钥文件">
        <el-upload class="upload-demo" :auto-upload="false" :limit="1" :on-change="handlePublicKeyFileChange"
          :file-list="publicKeyFileList">
          <el-button type="primary">选择公钥文件</el-button>
          <template #tip>
            <div class="el-upload__tip">上传支付宝公钥文件，支持.pem格式</div>
          </template>
        </el-upload>
      </el-form-item>
      <el-form-item label="沙箱环境">
        <el-switch v-model="paymentConfig.isSandbox" active-text="开启" inactive-text="关闭" />
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
import { updateAlipayConfig, uploadFile } from '@/api/system/profile';
import AliPayIcon from '@/views/icons/aliPayIcon.vue';

const props = defineProps({
  config: {
    type: Object,
    required: true
  }
});

const emit = defineEmits(['config-updated', 'status-changed']);

const { proxy } = getCurrentInstance();

// 支付配置数据
const paymentConfig = ref({
  appId: '',
  privateKey: '',
  alipayPublicKey: '',
  isActive: false,
  isSandbox: false,
  storeId: 0
});

// 文件上传相关
const privateKeyFileList = ref([]);
const publicKeyFileList = ref([]);

// 监听props变化，更新本地数据
watch(() => props.config, (newConfig) => {
  if (newConfig) {
    paymentConfig.value = {
      appId: newConfig.appId || '',
      privateKey: newConfig.privateKey || '',
      alipayPublicKey: newConfig.alipayPublicKey || '',
      isActive: newConfig.isActive || false,
      isSandbox: newConfig.isSandbox || false,
      storeId: newConfig.storeId || 0
    };

    console.log('newConfig', newConfig);
    // 如果有私钥文件，更新文件列表
    if (newConfig.privateKey) {
      const fileName = newConfig.privateKey.split('/').pop();
      privateKeyFileList.value = [{ name: fileName, url: newConfig.privateKey }];
    }

    // 如果有公钥文件，更新文件列表
    if (newConfig.alipayPublicKey) {
      const fileName = newConfig.alipayPublicKey.split('/').pop();
      publicKeyFileList.value = [{ name: fileName, url: newConfig.alipayPublicKey }];
    }
  }
}, { immediate: true, deep: true });

// 处理支付宝私钥文件上传
const handlePrivateKeyFileChange = async (file) => {
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
        proxy.notify.success('支付宝私钥文件上传成功');
      };
    } catch (error) {
      proxy.notify.error('支付宝私钥文件上传失败：' + error.message);
    }
  }
};

// 处理支付宝公钥文件上传
const handlePublicKeyFileChange = async (file) => {
  if (file && file.raw) {
    try {
      // 读取文件内容
      const reader = new FileReader();
      reader.readAsArrayBuffer(file.raw);
      reader.onload = async (e) => {
        const data = new Uint8Array(e.target.result);
        // 上传文件
        const result = await uploadFile(file.name, Array.from(data));
        // 显示文件路径到输入框
        paymentConfig.value.alipayPublicKey = result;
        proxy.notify.success('支付宝公钥文件上传成功');
      };
    } catch (error) {
      proxy.notify.error('支付宝公钥文件上传失败：' + error.message);
    }
  }
};

// 更新支付配置
const handleUpdatePaymentConfig = async () => {
  try {
    // 确保storeId存在
    if (!paymentConfig.value.storeId || paymentConfig.value.storeId <= 0) {
      proxy.notify.error('商家ID不能为空，请先完善店铺信息');
      return;
    }

    // 验证所有字段不能为空
    const requiredFields = {
      'appId': '应用ID',
      'privateKey': '私钥',
      'alipayPublicKey': '支付宝公钥'
    };

    for (const [key, label] of Object.entries(requiredFields)) {
      if (!paymentConfig.value[key]) {
        proxy.notify.error(`${label}不能为空`);
        return;
      }
    }

    // 转换字段名称以匹配后端结构
    const alipayConfig = {
      storeId: paymentConfig.value.storeId,
      appId: paymentConfig.value.appId,
      privateKey: paymentConfig.value.privateKey,
      alipayPublicKey: paymentConfig.value.alipayPublicKey,
      isActive: paymentConfig.value.isActive,
      isSandbox: paymentConfig.value.isSandbox
    };

    await updateAlipayConfig(alipayConfig);
    proxy.notify.success('支付宝配置更新成功');
    emit('config-updated', 'alipay', alipayConfig);
  } catch (error) {
    proxy.notify.error('配置更新失败：' + (error.message || '未知错误'));
  }
};

// 处理支付状态变更
const handlePaymentStatusChange = async () => {
  try {
    // 确保storeId存在
    if (!paymentConfig.value.storeId || paymentConfig.value.storeId <= 0) {
      proxy.notify.error('商家ID不能为空，请先完善店铺信息');
      // 恢复状态
      paymentConfig.value.isActive = !paymentConfig.value.isActive;
      return;
    }

    // 如果正在启用支付，验证所有字段不能为空
    if (paymentConfig.value.isActive) {
      const requiredFields = {
        'appId': '应用ID',
        'privateKey': '私钥',
        'alipayPublicKey': '支付宝公钥'
      };

      for (const [key, label] of Object.entries(requiredFields)) {
        if (!paymentConfig.value[key]) {
          proxy.notify.error(`${label}不能为空，无法启用支付`);
          // 恢复状态
          paymentConfig.value.isActive = !paymentConfig.value.isActive;
          return;
        }
      }
    }

    // 转换字段名称以匹配后端结构
    const alipayConfig = {
      storeId: paymentConfig.value.storeId,
      appId: paymentConfig.value.appId,
      privateKey: paymentConfig.value.privateKey,
      alipayPublicKey: paymentConfig.value.alipayPublicKey,
      isActive: paymentConfig.value.isActive,
      isSandbox: paymentConfig.value.isSandbox
    };

    await updateAlipayConfig(alipayConfig);
    proxy.notify.success(`支付宝状态已${paymentConfig.value.isActive ? '启用' : '禁用'}`);
    emit('status-changed', 'alipay', paymentConfig.value.isActive);
  } catch (error) {
    // 恢复状态
    paymentConfig.value.isActive = !paymentConfig.value.isActive;
    proxy.notify.error('状态更新失败：' + (error.message || '未知错误'));
  }
};
</script>

<style scoped>
.payment-card {
  height: 100%;
  border-radius: 8px;
  transition: all 0.3s;
}

.alipay-card {
  border-top: 4px solid #1677ff;
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