<template>
  <div class="close-btn">
    <el-button link icon="Close" size="large" @click="closeWindow" />
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
          <el-input v-model="registerForm.name" type="text" size="large" auto-complete="off" placeholder="商家名称"
            class="custom-input">
            <template #prefix><el-icon class="input-icon">
                <OfficeBuilding />
              </el-icon></template>
          </el-input>
        </el-form-item>

        <!-- 联系人姓名 -->
        <el-form-item prop="ownerName" class="custom-form-item">
          <el-input v-model="registerForm.ownerName" type="text" size="large" auto-complete="off" placeholder="店主姓名"
            class="custom-input">
            <template #prefix><svg-icon icon-class="user" class="input-icon" /></template>
          </el-input>
        </el-form-item>

        <!-- 手机号 -->
        <el-form-item prop="ownerPhone" class="custom-form-item">
          <el-input v-model="registerForm.ownerPhone" type="text" size="large" auto-complete="off" placeholder="手机号码"
            class="custom-input">
            <template #prefix><svg-icon icon-class="phone" class="input-icon" /></template>
          </el-input>
        </el-form-item>

        <!-- 短信验证码 -->
        <el-form-item prop="verificationCode" class="custom-form-item sms-code-item">
          <el-input v-model="registerForm.verificationCode" type="text" size="large" auto-complete="off"
            placeholder="短信验证码" class="custom-input sms-input">
            <template #prefix><svg-icon icon-class="validCode" class="input-icon" /></template>
          </el-input>
          <el-button :disabled="smsCodeTimer > 0" type="primary" class="sms-code-button" @click="getSmsCode">
            {{ smsCodeTimer > 0 ? `${smsCodeTimer}秒后重试` : '获取验证码' }}
          </el-button>
        </el-form-item>

        <!-- 商家地址 -->
        <el-form-item prop="location" class="custom-form-item">
          <el-input v-model="registerForm.location" type="text" size="large" auto-complete="off" placeholder="商家地址"
            class="custom-input">
            <template #prefix><svg-icon icon-class="tree" class="input-icon" /></template>
          </el-input>
        </el-form-item>

        <!-- 密码 -->
        <el-form-item prop="password" class="custom-form-item">
          <el-input v-model="registerForm.password" type="password" size="large" auto-complete="off" placeholder="密码"
            class="custom-input">
            <template #prefix><svg-icon icon-class="password" class="input-icon" /></template>
          </el-input>
        </el-form-item>

        <!-- 确认密码 -->
        <el-form-item prop="confirmPassword" class="custom-form-item">
          <el-input v-model="registerForm.confirmPassword" type="password" size="large" auto-complete="off"
            placeholder="确认密码" class="custom-input">
            <template #prefix><svg-icon icon-class="password" class="input-icon" /></template>
          </el-input>
        </el-form-item>

        <!-- 图形验证码 -->
        <el-form-item prop="code" v-if="captchaEnabled" class="custom-form-item captcha-item">
          <el-input v-model="registerForm.code" ref="codeInput" size="large" auto-complete="off" placeholder="验证码"
            class="custom-input captcha-input">
            <template #prefix><svg-icon icon-class="validCode" class="input-icon" /></template>
          </el-input>
          <div class="register-code">
            <img :src="codeUrl" @click="getCode" class="register-code-img" />
          </div>
        </el-form-item>

        <!-- 注册按钮 -->
        <el-form-item class="register-button-item">
          <el-button :loading="loading" size="large" type="primary" class="register-button"
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
    <el-dialog title="提示" :visible.sync="dialogVisible" width="30%">
      <div style="text-align:center; padding: 30px 20px;">
        <div style="margin-bottom:30px;">
          <div
            style="display:inline-flex; justify-content:center; align-items:center; width:80px; height:80px; border-radius:50%; background-color:#67C23A; box-shadow:0 6px 16px rgba(103, 194, 58, 0.4);">
            <i class="el-icon-check" style="font-size:40px; color:white;"></i>
          </div>
        </div>
        <div style="font-size:24px; color:#303133; margin-bottom:20px; font-weight:600">注册申请已提交</div>
        <div style="color:#606266; font-size:16px; line-height:1.8; margin-bottom:8px;">您的注册申请已提交，请等待管理员审核。</div>
        <div style="color:#606266; font-size:16px; line-height:1.8; margin-top:8px;">审核通过后，您将收到短信通知。</div>
      </div>
    </el-dialog>
  </div>

</template>

<script setup>
import WaveBackground from '@/layout/components/WaveBackground.vue';
import SunRays from '@/layout/components/SunRays.vue';
import { getCodeImg, register, getMsgCode } from "@/api/login";
import { Window } from "@tauri-apps/api/window";
import { ElMessageBox } from "element-plus";
import { convertFileSrc } from '@tauri-apps/api/core'

const appWindow = new Window('main');
const router = useRouter();
const { proxy } = getCurrentInstance();

// 注册表单数据
const registerForm = ref({
  avatar: "images/avatars/avatar1.png", // 默认选择第一个头像
  name: "",
  ownerName: "",
  ownerPhone: "",
  verificationCode: "",
  location: "",
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
  location: [
    { required: true, trigger: "blur", message: "请输入商家地址" },
    { min: 5, max: 100, message: "地址长度必须介于 5 和 100 之间", trigger: "blur" }
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
      title: "验证失败",
      message: error.message || "请输入正确的手机号码",
      duration: 3000
    });
  }
};

// 提交注册
const handleRegister = () => {
  proxy.$refs.registerRef.validate(valid => {
    if (valid) {
      loading.value = true;

      // 调用注册API
      register(registerForm.value).then(res => {
        loading.value = false;
        ElMessageBox.alert(
          `<div style="text-align:center; padding: 30px 20px;">
            <div style="margin-bottom:30px;">
              <div style="display:inline-flex; justify-content:center; align-items:center; width:80px; height:80px; border-radius:50%; background-color:#67C23A; box-shadow:0 6px 16px rgba(103, 194, 58, 0.4);">
                <i class="el-icon-check" style="font-size:40px; color:white;"></i>
              </div>
            </div>
            <div style="font-size:24px; color:#303133; margin-bottom:20px; font-weight:600">注册申请已提交</div>
            <div style="color:#606266; font-size:16px; line-height:1.8; margin-bottom:8px;">您的注册申请已提交，请等待管理员审核。</div>
            <div style="color:#606266; font-size:16px; line-height:1.8; margin-top:8px;">审核通过后，您将收到短信通知。</div>
          </div>`,
          "提示",
          {
            dangerouslyUseHTMLString: true,
            confirmButtonText: "返回登录页",
            customClass: {
              container: "register-success-dialog",
              confirmButton: "register-success-confirm-btn"
            },
            callback: () => {
              router.push("/login");
            }
          }
        );
      }).catch(() => {
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
  .custom-form-item {
    margin-bottom: 24px;
  }

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

  .custom-input {
    height: 50px;
    border-radius: 8px;
    transition: all 0.3s;

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

  .sms-code-item {
    display: flex;
    gap: 10px;

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