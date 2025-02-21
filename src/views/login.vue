<template>
  <div class="close-btn" style="padding: .3rem;">
    <el-button link icon="Close" size="large" style="color: #000;" @click="closeWindow" />
  </div>
  <WaveBackground />
  <div class="login" data-tauri-drag-region >
    <el-form ref="loginRef" :model="loginForm" :rules="loginRules" class="login-form">
      <h3 class="title">LAUNDRY</h3>
      <el-form-item prop="username">
        <el-input v-model="loginForm.username" type="text" size="large" auto-complete="off" placeholder="账号">
          <template #prefix><svg-icon icon-class="user" class="el-input__icon input-icon" /></template>
        </el-input>
      </el-form-item>
      <el-form-item prop="password">
        <el-input v-model="loginForm.password" type="password" size="large" auto-complete="off" placeholder="密码"
          @keyup.enter="handleLogin">
          <template #prefix><svg-icon icon-class="password" class="el-input__icon input-icon" /></template>
        </el-input>
      </el-form-item>
      <el-form-item prop="code" v-if="captchaEnabled">
        <el-input v-model="loginForm.code" size="large" auto-complete="off" placeholder="验证码" style="width: 63%"
          @keyup.enter="handleLogin">
          <template #prefix><svg-icon icon-class="validCode" class="el-input__icon input-icon" /></template>
        </el-input>
        <div class="login-code">
          <img :src="codeUrl" @click="getCode" class="login-code-img" />
        </div>
      </el-form-item>
      <el-checkbox v-model="loginForm.rememberMe" style="margin:0px 0px 25px 0px;">记住密码</el-checkbox>
      <el-form-item style="width:100%;">
        <el-button :loading="loading" size="large" type="primary" style="width:100%;" @click.prevent="handleLogin">
          <span v-if="!loading">登 录</span>
          <span v-else>登 录 中...</span>
        </el-button>
        <div style="float: right;" v-if="register">
          <router-link class="link-type" :to="'/register'">立即注册</router-link>
        </div>
      </el-form-item>
    </el-form>
    <!--  底部  -->
    <div class="el-login-footer">
      <!-- <span>Copyright © 2018-2024 ruoyi.vip All Rights Reserved.</span> -->
    </div>
  </div>
  <el-dialog v-model="pwdDialogVisible" title="首次登录修改密码" :close-on-click-modal="false" :show-close="false"
    width="400px">
    <el-form ref="pwdFormRef" :model="pwdForm" :rules="pwdRules" label-width="80px">
      <el-form-item label="新密码" prop="newPassword">
        <el-input v-model="pwdForm.newPassword" type="password" show-password placeholder="请输入新密码" />
      </el-form-item>
      <el-form-item label="确认密码" prop="confirmPassword">
        <el-input v-model="pwdForm.confirmPassword" type="password" show-password placeholder="请确认新密码" />
      </el-form-item>
    </el-form>
    <template #footer>
      <el-button type="primary" @click="submitNewPwd">提交</el-button>
    </template>
  </el-dialog>
</template>

<script setup>
import WaveBackground from '@/layout/components/WaveBackground.vue';
import { getCodeImg } from "@/api/login";
import Cookies from "js-cookie";
import { encrypt, decrypt } from "@/utils/jsencrypt";
import useUserStore from '@/store/modules/user';
import {LogicalSize, Window } from "@tauri-apps/api/window";
import { onMounted } from 'vue';
const appWindow = new Window('main');

const userStore = useUserStore()
const route = useRoute();
const router = useRouter();
const { proxy } = getCurrentInstance();

const loginForm = ref({
  username: null,
  password: null,
  rememberMe: false,
  code: "",
  uuid: ""
});

const loginRules = {
  username: [{ required: true, trigger: "blur", message: "请输入您的账号" }],
  password: [{ required: true, trigger: "blur", message: "请输入您的密码" }],
  code: [{ required: true, trigger: "change", message: "请输入验证码" }]
};

const codeUrl = ref("");
const loading = ref(false);
// 验证码开关
const captchaEnabled = ref(true);
// 注册开关
const register = ref(false);
const redirect = ref(undefined);

watch(route, (newRoute) => {
  redirect.value = newRoute.query && newRoute.query.redirect;
}, { immediate: true });

// 关闭窗口
const closeWindow = () => {
  appWindow.close()
};

const pwdDialogVisible = ref(false);
const pwdForm = ref({
  newPassword: '',
  confirmPassword: ''
});

// 密码验证规则
const pwdRules = {
  newPassword: [
    { required: true, message: '请输入新密码' },
    { min: 6, message: '密码长度不能少于6位' }
  ],
  confirmPassword: [
    { required: true, message: '请确认新密码' },
    {
      validator: (rule, value, callback) => {
        if (value !== pwdForm.value.newPassword) {
          callback(new Error('两次输入密码不一致'));
        } else {
          callback();
        }
      }
    }
  ]
};

// 提交新密码
const submitNewPwd = async () => {
  try {
    await proxy.$refs.pwdFormRef.validate();
    await userStore.updatePassword({
      account: loginForm.value.username,
      oldPassword: loginForm.value.password,
      newPassword: pwdForm.value.newPassword,
      confirmPassword: pwdForm.value.confirmPassword,
    });
    pwdDialogVisible.value = false;
    proxy.$modal.msgSuccess("密码修改成功，请重新登录");

    pwdForm.value.newPassword = '';
    pwdForm.value.confirmPassword = '';

    // 清空登录信息
    loading.value = false;
    loginForm.value.password = '';
    if (loginForm.value.rememberMe) {
      Cookies.set("password", encrypt(pwdForm.value.newPassword)); // 更新记住的密码
    }

    // 强制跳转登录页
    router.replace('/login');
  } catch (err) {
    proxy.$modal.msgError(err.message || '密码修改失败');
  }
};

function handleLogin() {
  proxy.$refs.loginRef.validate(valid => {
    if (valid) {
      loading.value = true;
      // 勾选了需要记住密码设置在 cookie 中设置记住用户名和密码
      if (loginForm.value.rememberMe) {
        Cookies.set("username", loginForm.value.username, { expires: 30 });
        Cookies.set("password", encrypt(loginForm.value.password), { expires: 30 });
        Cookies.set("rememberMe", loginForm.value.rememberMe, { expires: 30 });
      } else {
        // 否则移除
        Cookies.remove("username");
        Cookies.remove("password");
        Cookies.remove("rememberMe");
      }
      // 调用action的登录方法
      userStore.login(loginForm.value).then((res) => {
        if (res.user.isFirstLogin) { // 根据实际响应字段判断
          pwdDialogVisible.value = true; // 显示修改密码弹窗
        } else {
          routeJump();
        }
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

async function routeJump() {
  // 原有跳转逻辑
  const query = route.query;
  const otherQueryParams = Object.keys(query).reduce((acc, cur) => {
    if (cur !== "redirect") {
      acc[cur] = query[cur];
    }
    return acc;
  }, {});
  router.push({ path: redirect.value || "/", query: otherQueryParams });
  // 应用过渡动画
  document.body.classList.add('window-resize-animation');
  appWindow.hide();
  setTimeout(() => {
    appWindow.setSize(new LogicalSize(2160, 1080));
    appWindow.show();
    document.body.classList.remove('window-resize-animation');
  }, 500); // 动画持续时间为500ms
}

function getCode() {
  getCodeImg().then(res => {
    captchaEnabled.value = res.captchaEnabled === undefined ? true : res.captchaEnabled;
    if (captchaEnabled.value) {
      codeUrl.value = "data:image/gif;base64," + res.img;
      loginForm.value.uuid = res.uuid;
    }
  });
}

function getCookie() {
  const username = Cookies.get("username");
  const password = Cookies.get("password");
  const rememberMe = Cookies.get("rememberMe");
  loginForm.value = {
    username: username === undefined ? loginForm.value.username : username,
    password: password === undefined ? loginForm.value.password : decrypt(password),
    rememberMe: rememberMe === undefined ? false : Boolean(rememberMe)
  };
}
onMounted(async () => {
  await appWindow.setSize(new LogicalSize(1000, 650))
  appWindow.show();
  getCode();
  getCookie();
})
</script>

<style lang='scss' scoped>
.login {
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
  color: black;
}
.title {
  margin: 0px auto 30px auto;
  text-align: center;
  color: #707070;
}

.login-form {
  border-radius: 6px;
  // background: #ffffff;
  width: 400px;
  padding: 25px 25px 5px 25px;

  .el-input {
    height: 40px;

    input {
      height: 40px;
    }
  }

  .input-icon {
    height: 39px;
    width: 14px;
    margin-left: 0px;
  }
}

.login-tip {
  font-size: 13px;
  text-align: center;
  color: #bfbfbf;
}

.login-code {
  width: 33%;
  height: 40px;
  float: right;
  overflow: hidden;

  img {
    cursor: pointer;
    vertical-align: middle;
  }
}

.el-login-footer {
  height: 40px;
  line-height: 40px;
  position: fixed;
  bottom: 0;
  width: 100%;
  text-align: center;
  color: #fff;
  font-family: Arial;
  font-size: 12px;
  letter-spacing: 1px;
}

.login-code-img {
  height: 40px;
  padding-left: 12px;
}
/* 添加窗口大小变化动画 */
.window-resize-animation {
  transition: width 0.5s ease, height 0.5s ease;
}
</style>
