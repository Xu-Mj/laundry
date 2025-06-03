<template>
  <div class="close-btn">
    <el-button link icon="Close" @click="closeWindow" />
  </div>
  <WaveBackground />
  <SunRays position="top-right" />
  <div class="register-container" data-tauri-drag-region>
    <div class="register-card">
      <div class="register-header">
        <h2 class="title">CleanWave</h2>
        <p class="subtitle">商家注册</p>
      </div>
      <el-form ref="registerRef" :model="registerForm" :rules="registerRules" class="register-form">
        <!-- 头像选择 -->
        <el-form-item prop="avatar" class="custom-form-item avatar-selection">
          <!-- <p class="avatar-label">选择头像</p> -->
          <div class="avatar-group">
            <div class="avatar-container" v-for="(avatar, index) in avatars" :key="index"
              :class="{ 'selected': registerForm.avatar === avatar }" @click="registerForm.avatar = avatar">
              <img :src="convertFileSrc(avatar)" alt="头像" class="avatar-image">
            </div>
          </div>
        </el-form-item>

        <!-- 商家名称 -->
        <el-form-item prop="name" class="custom-form-item">
          <el-input v-model="registerForm.name" type="text" auto-complete="off" placeholder="商家名称"
            class="custom-input">
            <template #prefix><el-icon class="input-icon">
                <OfficeBuilding />
              </el-icon></template>
          </el-input>
        </el-form-item>

        <!-- 联系人姓名 -->
        <el-form-item prop="ownerName" class="custom-form-item">
          <el-input v-model="registerForm.ownerName" type="text" auto-complete="off" placeholder="店主姓名"
            class="custom-input">
            <template #prefix><svg-icon icon-class="user" class="input-icon" /></template>
          </el-input>
        </el-form-item>

        <!-- 手机号 -->
        <el-form-item prop="ownerPhone" class="custom-form-item">
          <el-input v-model="registerForm.ownerPhone" type="text" auto-complete="off" placeholder="手机号码"
            class="custom-input">
            <template #prefix><svg-icon icon-class="phone" class="input-icon" /></template>
          </el-input>
        </el-form-item>

        <!-- 短信验证码 -->
        <el-form-item prop="verificationCode" class="">
          <div class="sms-code-item">
            <el-input v-model="registerForm.verificationCode" type="text" auto-complete="off"
              placeholder="短信验证码" class="custom-input sms-input">
              <template #prefix><svg-icon icon-class="validCode" class="input-icon" /></template>
            </el-input>
            <el-button :disabled="smsCodeTimer > 0" type="primary" class="sms-code-button" @click="getSmsCode">
              {{ smsCodeTimer > 0 ? `${smsCodeTimer}秒后重试` : '获取验证码' }}
            </el-button>
          </div>
        </el-form-item>

        <!-- 商家地址 - 省市区选择 -->
        <el-form-item prop="addressRegion" class="custom-form-item">
          <el-cascader v-model="registerForm.addressRegion" :options="regionData" placeholder="请选择省/市/区"
            class="custom-cascader" :props="{
              checkStrictly: false,
              value: 'value',
              label: 'label',
              children: 'children',
              expandTrigger: 'hover'
            }" clearable @change="handleAddressChange">
            <template #prefix><el-icon>
                <Location />
              </el-icon></template>
          </el-cascader>
        </el-form-item>

        <!-- 详细地址 -->
        <el-form-item prop="addressdetail" class="custom-form-item">
          <el-input v-model="registerForm.addressdetail" type="text" auto-complete="off"
            placeholder="请输入详细地址（街道、门牌号等）" class="custom-input">
            <template #prefix><el-icon class="input-icon">
                <Location />
              </el-icon></template>
          </el-input>
        </el-form-item>

        <!-- 密码 -->
        <el-form-item prop="password" class="custom-form-item">
          <el-input v-model="registerForm.password" type="password" auto-complete="off" placeholder="密码"
            class="custom-input">
            <template #prefix><svg-icon icon-class="password" class="input-icon" /></template>
          </el-input>
        </el-form-item>

        <!-- 确认密码 -->
        <el-form-item prop="confirmPassword" class="custom-form-item">
          <el-input v-model="registerForm.confirmPassword" type="password" auto-complete="off"
            placeholder="确认密码" class="custom-input">
            <template #prefix><svg-icon icon-class="password" class="input-icon" /></template>
          </el-input>
        </el-form-item>

        <!-- 图形验证码 -->
        <el-form-item prop="code" v-if="captchaEnabled" class="custom-form-item captcha-item">
          <el-input v-model="registerForm.code" ref="codeInput" auto-complete="off" placeholder="验证码"
            class="custom-input captcha-input">
            <template #prefix><svg-icon icon-class="validCode" class="input-icon" /></template>
          </el-input>
          <div class="register-code">
            <img :src="codeUrl" @click="getCode" class="register-code-img" />
          </div>
        </el-form-item>

        <!-- 注册按钮 -->
        <el-form-item class="register-button-item">
          <el-button :loading="loading" type="primary" class="register-button"
            @click.prevent="handleRegister">
            <span v-if="!loading">提交注册</span>
            <span v-else>提交中...</span>
          </el-button>
        </el-form-item>

        <!-- 登录链接 -->
        <div class="login-link">
          <span>已有账号？</span>
          <router-link class="link-type" :to="'/login'">立即登录</router-link>
        </div>
      </el-form>
    </div>
    <!-- <el-dialog title="提示" v-model="dialogVisible" width="30%" align-center>
      <div style="text-align:center; padding: 30px 20px;">
        <div style="margin-bottom:30px;">
          <div
            style="display:inline-flex; justify-content:center; align-items:center; width:80px; height:80px; border-radius:50%; background-color:#67C23A; box-shadow:0 6px 16px rgba(103, 194, 58, 0.4);">
            <el-icon class="el-icon-check" style="font-size:98px; color: #fff; width: 100%; height: 100%;">
              <CircleCheck />
            </el-icon>
          </div>
        </div>
        <div style="font-size:24px; color:#303133; margin-bottom:20px; font-weight:600">注册申请已提交</div>
        <div style="color:#606266; font-size:16px; line-height:1.8; margin-bottom:8px;">您的注册申请已提交，请等待管理员审核。</div>
        <div style="color:#606266; font-size:16px; line-height:1.8; margin-top:8px;">审核通过后，您将收到短信通知。</div>
      </div>
      <div style="display: flex; justify-content: center;">
        <el-button @click="router.push('/login')" type="primary">确定</el-button>
      </div>
    </el-dialog>
     -->
    <el-dialog title="提示" v-model="dialogVisible" width="30%" align-center>
      <div style="text-align:center; padding: 30px 20px;">
        <div style="margin-bottom:30px;">
          <div
            style="display:inline-flex; justify-content:center; align-items:center; width:80px; height:80px; border-radius:50%; background-color:#67C23A; box-shadow:0 6px 16px rgba(103, 194, 58, 0.4);">
            <el-icon class="el-icon-check" style="font-size:98px; color: #fff; width: 100%; height: 100%;">
              <CircleCheck />
            </el-icon>
          </div>
        </div>
        <div style="font-size:24px; color:#303133; margin-bottom:20px; font-weight:600">注册成功~</div>
      </div>
      <div style="display: flex; justify-content: center;">
        <el-button @click="router.push('/login')" type="primary">前往登录</el-button>
      </div>
    </el-dialog>
  </div>

</template>

<script setup>
import WaveBackground from '@/layout/components/WaveBackground.vue';
import SunRays from '@/layout/components/SunRays.vue';
import { getCodeImg, register, getMsgCode, getDeviceInfo } from "@/api/login";
import { Window } from "@tauri-apps/api/window";
import { convertFileSrc } from '@tauri-apps/api/core';
import { regionData, codeToText } from 'element-china-area-data';
import { Location } from '@element-plus/icons-vue'

const appWindow = new Window('main');
const router = useRouter();
const { proxy } = getCurrentInstance();

const dialogVisible = ref(false);

// 注册表单数据
const registerForm = ref({
  avatar: "images/avatars/avatar1.png", // 默认选择第一个头像
  name: "",
  ownerName: "",
  ownerPhone: "",
  verificationCode: "",
  addressRegion: [], // 省市区选择（用于级联选择器）
  // 后端对应的字段
  province: "", // 省份
  city: "", // 城市
  district: "", // 区/县
  addressdetail: "", // 详细地址
  password: "",
  confirmPassword: "",
  code: "",
  uuid: ""
});

// 密码确认验证
const equalToPassword = (rule, value, callback) => {
  if (registerForm.value.password !== value) {
    callback(new Error("两次输入的密码不一致"));
  } else {
    callback();
  }
};

// 手机号验证
const validatePhone = (rule, value, callback) => {
  const phoneRegex = /^1[3-9]\d{9}$/;
  if (!phoneRegex.test(value)) {
    callback(new Error("请输入正确的手机号码"));
  } else {
    callback();
  }
};

// 表单验证规则
const registerRules = {
  avatar: [
    { required: true, message: "请选择头像", trigger: "change" }
  ],
  name: [
    { required: true, trigger: "blur", message: "请输入商家名称" },
    { min: 2, max: 50, message: "商家名称长度必须介于 2 和 50 之间", trigger: "blur" }
  ],
  ownerName: [
    { required: true, trigger: "blur", message: "请输入联系人姓名" },
    { min: 2, max: 20, message: "联系人姓名长度必须介于 2 和 20 之间", trigger: "blur" }
  ],
  ownerPhone: [
    { required: true, trigger: "blur", message: "请输入手机号码" },
    { validator: validatePhone, trigger: "blur" }
  ],
  verificationCode: [
    { required: true, trigger: "blur", message: "请输入短信验证码" },
    { min: 4, max: 6, message: "验证码长度不正确", trigger: "blur" }
  ],
  addressRegion: [
    { required: true, trigger: "change", message: "请选择省/市/区" },
    { type: 'array', min: 3, message: "请完整选择省/市/区", trigger: "change" }
  ],
  addressdetail: [
    { required: true, trigger: "blur", message: "请输入详细地址" },
    { min: 3, max: 100, message: "详细地址长度必须介于 3 和 100 之间", trigger: "blur" }
  ],
  password: [
    { required: true, trigger: "blur", message: "请输入密码" },
    { min: 6, max: 20, message: "密码长度必须介于 6 和 20 之间", trigger: "blur" },
    { pattern: /^[^<>"'|\\]+$/, message: "不能包含非法字符：< > \" ' \\ |", trigger: "blur" }
  ],
  confirmPassword: [
    { required: true, trigger: "blur", message: "请再次输入密码" },
    { required: true, validator: equalToPassword, trigger: "blur" }
  ],
  code: [{ required: true, trigger: "change", message: "请输入验证码" }]
};

// 状态变量
const codeUrl = ref("");
const loading = ref(false);
const captchaEnabled = ref(true);
const smsCodeTimer = ref(0);
const smsCodeInterval = ref(null);
const avatars = [
  "images/avatars/avatar1.png",
  "images/avatars/avatar2.png",
  "images/avatars/avatar3.png",
];
// 关闭窗口
const closeWindow = () => {
  appWindow.close();
};

// 处理地址选择变化
const handleAddressChange = (value) => {
  if (value && value.length >= 3) {
    // 从级联选择器的值中获取省市区文本
    const [provinceCode, cityCode, districtCode] = value;
    registerForm.value.province = codeToText[provinceCode];
    registerForm.value.city = codeToText[cityCode];
    registerForm.value.district = codeToText[districtCode];
  } else {
    // 如果清空选择，也清空对应字段
    registerForm.value.province = "";
    registerForm.value.city = "";
    registerForm.value.district = "";
  }
};

// 获取短信验证码
const getSmsCode = async () => {
  // 验证手机号
  try {
    await proxy.$refs.registerRef.validateField('ownerPhone');

    // 开始倒计时
    smsCodeTimer.value = 60;
    smsCodeInterval.value = setInterval(() => {
      smsCodeTimer.value--;
      if (smsCodeTimer.value <= 0) {
        clearInterval(smsCodeInterval.value);
      }
    }, 1000);

    // 这里应该调用发送短信验证码的API
    // 由于目前没有实际的短信API，这里只做模拟
    await getMsgCode(registerForm.value.ownerPhone);
    proxy.$notify.success({
      title: "验证码已发送",
      message: `验证码已发送至手机号: ${registerForm.value.ownerPhone}`,
      duration: 3000
    });

    // 实际项目中应该调用类似的API
    // await sendSmsCode(registerForm.value.ownerPhone);
  } catch (error) {
    proxy.$notify.error({
      title: "获取验证码失败",
      message: error.message || "请输入正确的手机号码",
      duration: 3000
    });
  }
};

// 提交注册
const handleRegister = () => {
  console.log("注册表单数据：", registerForm.value);
  proxy.$refs.registerRef.validate(valid => {
    if (valid) {
      loading.value = true;

      // 处理地址数据，从级联选择器中提取省市区信息
      const formData = { ...registerForm.value };
      
      // 如果有选择省市区，则更新对应字段
      if (formData.addressRegion && formData.addressRegion.length >= 3) {
        // 从级联选择器的值中获取省市区文本
        const [provinceCode, cityCode, districtCode] = formData.addressRegion;
        formData.province = codeToText[provinceCode];
        formData.city = codeToText[cityCode];
        formData.district = codeToText[districtCode];
      }
      
      // 删除中间字段，不需要传给后端
      delete formData.addressRegion;
      delete formData.confirmPassword; // 确认密码不需要传给后端

      // 获取设备信息并添加到注册数据中
      getDeviceInfo().then(deviceInfo => {
        // 添加设备信息到注册数据
        formData.deviceId = deviceInfo.deviceId;
        formData.deviceName = deviceInfo.deviceName || '';
        formData.deviceMac = deviceInfo.macAddress || '';

        // 调用注册API
        return register(formData);
      }).then(res => {
        loading.value = false;
        dialogVisible.value = true;
      }).catch((error) => {
        console.error('注册失败:', error);
        loading.value = false;
        if (captchaEnabled.value) {
          getCode();
        }
      });
    }
  });
};

// 获取图形验证码
const getCode = () => {
  getCodeImg().then(res => {
    captchaEnabled.value = res.captchaEnabled === undefined ? true : res.captchaEnabled;
    if (captchaEnabled.value) {
      codeUrl.value = "data:image/gif;base64," + res.img;
      registerForm.value.uuid = res.uuid;
    }
  });
};

onMounted(async () => {
  getCode();
});
</script>

<style lang='scss' scoped>
.register-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  background-color: transparent;
}

.close-btn {
  position: fixed;
  top: 0;
  right: 0;
  z-index: 10;
  padding: 0.5rem;

  .el-button {
    color: #333;
  }
}

.register-card {
  transition: color 0.3s ease;

  &:hover {
    color: #000;
    transform: scale(1.1);
  }
}



.register-card {
  background: rgba(255, 255, 255, 0.85);
  backdrop-filter: blur(10px);
  border-radius: 16px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  padding: 40px;
  width: 480px;
  transition: all 0.3s ease;

  &:hover {
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.15);
    transform: translateY(-5px);
  }
}

.register-header {
  text-align: center;
  margin-bottom: 30px;

  .title {
    font-size: 28px;
    font-weight: 600;
    margin: 0 0 10px 0;
    background: linear-gradient(45deg, #3498db, #2c3e50);
    background-clip: text;
    -webkit-text-fill-color: transparent;
    letter-spacing: 1px;
  }

  .subtitle {
    color: #606266;
    font-size: 14px;
    margin: 0;
  }
}

.register-form {

  .avatar-selection {
    text-align: center;

    .avatar-label {
      font-size: 14px;
      color: #606266;
      margin-bottom: 12px;
    }

    .avatar-group {
      width: 100%;
      display: flex;
      justify-content: center;
      gap: 20px;

      .avatar-container {
        width: 80px;
        height: 80px;
        border-radius: 50%;
        overflow: hidden;
        cursor: pointer;
        border: 3px solid transparent;
        transition: all 0.3s ease;

        &:hover {
          transform: scale(1.05);
          box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
        }

        &.selected {
          border-color: #409EFF;
          box-shadow: 0 0 15px rgba(64, 158, 255, 0.5);
        }

        .avatar-image {
          width: 100%;
          height: 100%;
          object-fit: cover;
        }
      }
    }
  }

  .custom-input,
  .custom-cascader {
    height: 50px;
    border-radius: 8px;
    transition: all 0.3s;
    width: 100%;

    &:hover,
    &:focus {
      box-shadow: 0 0 0 2px rgba(64, 158, 255, 0.1);
    }

    input {
      height: 50px;
      padding-left: 15px;
    }

    .input-icon {
      height: 20px;
      width: 20px;
      margin: 0 10px 0 0;
      color: #909399;
    }
  }

  // 级联选择器特殊样式
  .custom-cascader {
    :deep(.el-input__wrapper) {
      padding: 0 15px;
    }

    :deep(.el-input__inner) {
      height: 50px;
      line-height: 50px;
    }
  }

  .sms-code-item {
    width: 100%;
    display: flex;
    gap: 1rem;

    .sms-input {
      flex: 1;
    }

    .sms-code-button {
      height: 50px;
      border-radius: 8px;
      min-width: 120px;
      font-size: 14px;
      background: linear-gradient(45deg, #409EFF, #007bff);
      border: none;
      transition: all 0.3s ease;

      &:hover {
        background: linear-gradient(45deg, #007bff, #0056b3);
        transform: translateY(-2px);
        box-shadow: 0 5px 15px rgba(0, 105, 217, 0.3);
      }

      &:active {
        transform: translateY(0);
      }

      &:disabled {
        background: #a0cfff;
        transform: none;
        box-shadow: none;
      }
    }
  }

  .captcha-item {
    display: flex;
    gap: 10px;

    .captcha-input {
      flex: 1;
    }
  }

  .register-button-item {
    margin-bottom: 20px;
    text-align: center;
  }

  .register-button {
    width: 100%;
    height: 50px;
    border-radius: 8px;
    font-size: 16px;
    font-weight: 500;
    letter-spacing: 1px;
    background: linear-gradient(45deg, #409EFF, #007bff);
    border: none;
    transition: all 0.3s ease;

    &:hover {
      background: linear-gradient(45deg, #007bff, #0056b3);
      transform: translateY(-2px);
      box-shadow: 0 5px 15px rgba(0, 105, 217, 0.3);
    }

    &:active {
      transform: translateY(0);
    }
  }

  .login-link {
    text-align: center;
    margin-top: 15px;
    font-size: 14px;
    color: #606266;

    .link-type {
      color: #409EFF;
      margin-left: 5px;
      transition: color 0.3s;

      &:hover {
        color: #007bff;
        text-decoration: underline;
      }
    }
  }
}

.register-code {
  height: 50px;
  border-radius: 8px;
  overflow: hidden;
  background: #f5f7fa;
  display: flex;
  align-items: center;
  justify-content: center;

  img {
    cursor: pointer;
    height: 40px;
    transition: transform 0.3s;

    &:hover {
      transform: scale(1.05);
    }
  }
}

.el-register-footer {
  height: 40px;
  line-height: 40px;
  position: fixed;
  bottom: 0;
  width: 100%;
  text-align: center;
  color: rgba(255, 255, 255, 0.7);
  font-family: Arial;
  font-size: 12px;
  letter-spacing: 1px;
}
</style>