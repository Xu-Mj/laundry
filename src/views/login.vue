<template>
  <div class="close-btn">
    <el-button link icon="Close" size="large" @click="closeWindow" />
  </div>
  <WaveBackground />
  <SunRays position="top-right" />
  <div class="login-container" data-tauri-drag-region>
    <div class="login-card">
      <div class="login-header">
        <h2 class="title">CleanWave</h2>
        <p class="subtitle">欢迎回来，请登录您的账号</p>
      </div>
      <el-form ref="loginRef" :model="loginForm" :rules="loginRules" class="login-form">
        <el-form-item prop="account" class="custom-form-item">
          <el-input v-model="loginForm.account" type="text" size="large" auto-complete="off" placeholder="账号"
            class="custom-input">
            <template #prefix><svg-icon icon-class="user" class="input-icon" /></template>
          </el-input>
        </el-form-item>
        <el-form-item prop="password" class="custom-form-item">
          <el-input v-model="loginForm.password" type="password" size="large" auto-complete="off" placeholder="密码"
            @keyup.enter="handleLogin" class="custom-input">
            <template #prefix><svg-icon icon-class="password" class="input-icon" /></template>
          </el-input>
        </el-form-item>
        <el-form-item prop="code" v-if="captchaEnabled" class="custom-form-item captcha-item">
          <el-input v-model="loginForm.code" ref="codeInput" size="large" auto-complete="off" placeholder="验证码"
            @keyup.enter="handleLogin" class="custom-input captcha-input">
            <template #prefix><svg-icon icon-class="validCode" class="input-icon" /></template>
          </el-input>
          <div class="login-code">
            <img :src="codeUrl" @click="getCode" class="login-code-img" />
          </div>
        </el-form-item>
        <div class="remember-row">
          <el-checkbox v-model="loginForm.rememberMe">记住密码</el-checkbox>
          <el-button type="primary" link icon="UserFilled" @click.prevent="handleGuestLogin">
            游客登录
          </el-button>
        </div>
        <el-form-item style="width: 100%;">
          <el-button id="loginButton" ref="loginButton" style="width: 100%;" :loading="loading" size="large"
            type="primary" class="login-button" @click.prevent="handleLogin">
            <span v-if="!loading">登录</span>
            <span v-else>登录中...</span>
          </el-button>
        </el-form-item>
        <div class="register-link" v-if="register">
          <span>还没有账号？</span>
          <router-link class="link-type" :to="'/register'">立即注册</router-link>
        </div>
      </el-form>
    </div>
    <div class="el-login-footer">
      <!-- <span>Copyright © 2018-2024 ruoyi.vip All Rights Reserved.</span> -->
    </div>
  </div>
</template>

<script setup>
import WaveBackground from '@/layout/components/WaveBackground.vue';
import SunRays from '@/layout/components/SunRays.vue';
import { getCodeImg } from "@/api/login";
import Cookies from "js-cookie";
import { encrypt, decrypt } from "@/utils/jsencrypt";
import useUserStore from '@/store/modules/user';
import { Window } from "@tauri-apps/api/window";
import { onMounted } from 'vue';
const appWindow = new Window('main');

const userStore = useUserStore()
const route = useRoute();
const router = useRouter();
const { proxy } = getCurrentInstance();

const loginForm = ref({
  account: null,
  password: null,
  rememberMe: false,
  code: "",
  uuid: ""
});

const loginRules = {
  account: [{ required: true, trigger: "blur", message: "请输入您的账号" }],
  password: [{ required: true, trigger: "blur", message: "请输入您的密码" }],
  code: [{ required: true, trigger: "change", message: "请输入验证码" }]
};

const codeInput = ref(null);
const loginButton = ref(null);

const codeUrl = ref("");
const loading = ref(false);
// 验证码开关
const captchaEnabled = ref(true);
// 注册开关
const register = ref(true);
const redirect = ref(undefined);

watch(route, (newRoute) => {
  redirect.value = newRoute.query && newRoute.query.redirect;
}, { immediate: true });

// 关闭窗口
const closeWindow = () => {
  appWindow.close()
};

function handleLogin() {
  proxy.$refs.loginRef.validate(valid => {
    if (valid) {
      loading.value = true;
      // 勾选了需要记住密码设置在 cookie 中设置记住用户名和密码
      if (loginForm.value.rememberMe) {
        Cookies.set("account", loginForm.value.account, { expires: 30 });
        Cookies.set("password", encrypt(loginForm.value.password), { expires: 30 });
        Cookies.set("rememberMe", loginForm.value.rememberMe, { expires: 30 });
      } else {
        // 否则移除
        Cookies.remove("account");
        Cookies.remove("password");
        Cookies.remove("rememberMe");
      }
      // 调用action的登录方法
      userStore.login(loginForm.value).then((res) => {
        routeJump();
      }).catch(() => {
        loading.value = false;
        // 重新获取验证码
        if (captchaEnabled.value) {
          getCode();
        }
      });
    }
  });
}

// 游客登录处理函数
function handleGuestLogin() {
  loading.value = true;

  // 调用登录方法
  userStore.guestLogin().then((res) => {
    routeJump();
  }).catch(() => {
    loading.value = false;
    proxy.notify.error('游客登录失败，请稍后再试');
  });
}

async function routeJump() {
  const query = route.query;
  const otherQueryParams = Object.keys(query).reduce((acc, cur) => {
    if (cur !== "redirect") {
      acc[cur] = query[cur];
    }
    return acc;
  }, {});
  router.push({ path: redirect.value || "/", query: otherQueryParams });
}

async function getCode() {
  await getCodeImg().then(res => {
    captchaEnabled.value = res.captchaEnabled === undefined ? true : res.captchaEnabled;
    if (captchaEnabled.value) {
      codeUrl.value = "data:image/gif;base64," + res.img;
      loginForm.value.uuid = res.uuid;
    }
  });
}

function getCookie() {
  const account = Cookies.get("account");
  const password = Cookies.get("password");
  const rememberMe = Cookies.get("rememberMe");
  loginForm.value = {
    account: account === undefined ? loginForm.value.account : account,
    password: password === undefined ? loginForm.value.password : decrypt(password),
    rememberMe: rememberMe === undefined ? false : Boolean(rememberMe)
  };
}

function focusElement() {
  if (loginForm.value.account && loginForm.value.password) {
    if (captchaEnabled.value) {
      codeInput.value.focus();
    } else {
      const button = document.getElementById('loginButton');
      button.focus();
    }
  }
}

onMounted(async () => {
  await getCode();
  getCookie();
  focusElement();
})
</script>

<style lang='scss' scoped>
.login-container {
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
    transition: color 0.3s ease;

    &:hover {
      color: #000;
      transform: scale(1.1);
    }
  }
}

.login-card {
  width: 400px;
  padding: 40px;
  margin: 0 auto;
  background-color: rgba(255, 255, 255, 0.9);
  border-radius: 10px;
  box-shadow: 0 0 20px rgba(0, 0, 0, 0.1);
  position: relative;
  overflow: visible;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  padding-top: 80px;
}

.login-header {
  text-align: center;
  margin-bottom: 30px;

  .title {
    font-size: 28px;
    font-weight: 600;
    margin: 0 0 10px 0;
    background: linear-gradient(45deg, #3498db, #2c3e50);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    letter-spacing: 1px;
  }

  .subtitle {
    color: #606266;
    font-size: 14px;
    margin: 0;
  }
}

.login-form {
  .custom-form-item {
    margin-bottom: 24px;
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

  .captcha-item {
    display: flex;
    gap: 10px;

    .captcha-input {
      flex: 1;
    }
  }

  .remember-row {
    display: flex;
    justify-content: space-between;
    margin-bottom: 24px;
  }

  .login-button-item {
    margin-bottom: 20px;
    text-align: center;
  }

  .guest-button {
    width: 100%;
    height: 40px;
    border-radius: 8px;
    font-size: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.3s ease;

    &:hover {
      transform: translateY(-2px);
      box-shadow: 0 3px 10px rgba(0, 0, 0, 0.1);
    }

  }

  .register-link {
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

.login-code {
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

.el-login-footer {
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

/* 添加窗口大小变化动画 */
.window-resize-animation {
  transition: width 0.5s ease, height 0.5s ease;
}

/* 密码修改弹窗样式 */
.pwd-dialog {
  :deep(.el-dialog) {
    border-radius: 16px;
    overflow: hidden;
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.2);
    background: rgba(255, 255, 255, 0.9);
    backdrop-filter: blur(10px);

    .el-dialog__header {
      display: none;
    }

    .el-dialog__body {
      padding: 0;
    }
  }
}

.pwd-card {
  padding: 30px;

  .pwd-header {
    text-align: center;
    margin-bottom: 25px;

    h3 {
      font-size: 22px;
      font-weight: 600;
      margin: 0 0 10px 0;
      background: linear-gradient(45deg, #3498db, #2c3e50);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      letter-spacing: 1px;
    }

    p {
      color: #606266;
      font-size: 14px;
      margin: 0;
    }
  }

  .pwd-form {
    .pwd-button-item {
      margin-top: 30px;
      margin-bottom: 0;
    }

    .pwd-button {
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
  }
}
</style>
