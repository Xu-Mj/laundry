<template>
  <div class="navbar">
    <CloseBar />
    <hamburger id="hamburger-container" :is-active="appStore.sidebar.opened" class="hamburger-container"
      @toggleClick="toggleSideBar" />
    <breadcrumb id="breadcrumb-container" class="breadcrumb-container" v-if="!settingsStore.topNav" />
    <top-nav id="topmenu-container" class="topmenu-container" v-if="settingsStore.topNav" />

    <div class="right-menu">
      <template v-if="appStore.device !== 'mobile'">
        <header-search id="header-search" class="right-menu-item" />


        <!-- <el-tooltip content="布局大小" effect="dark" placement="bottom">
          <size-select id="size-select" class="right-menu-item hover-effect" />
        </el-tooltip> -->
      </template>
      <el-button class="right-menu-item" icon="Setting" type="text" @click="showSetting = !showSetting">
      </el-button>
      <div class="avatar-container">
        <el-dropdown @command="handleCommand" class="right-menu-item hover-effect" trigger="click">
          <div class="avatar-wrapper">
            <img :src="userStore.avatar" class="user-avatar" />
            <el-icon><caret-bottom /></el-icon>
          </div>
          <template #dropdown>
            <el-dropdown-menu>
              <!-- <router-link to="/user/profile"> -->
              <el-dropdown-item @click="showChangePwd = true">修改密码</el-dropdown-item>
              <!-- </router-link> -->
              <el-dropdown-item command="setLayout" v-if="settingsStore.showSettings">
                <span>布局设置</span>
              </el-dropdown-item>
              <el-dropdown-item divided command="logout">
                <span>退出登录</span>
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
    </div>

    <Setting :visible="showSetting" :key="showSetting" :taggle="() => { showSetting = !showSetting }" />

    <el-dialog v-model="showChangePwd" title="修改密码" :show-close="false" width="300px">
      <el-form ref="formRef" :model="form" :rules="rules" label-width="80px">
        <el-form-item label="旧密码" prop="oldPassword">
          <el-input v-model="form.oldPassword" type="password" />
        </el-form-item>
        <el-form-item label="新密码" prop="newPassword">
          <el-input v-model="form.newPassword" type="password" />
        </el-form-item>
        <el-form-item label="确认密码" prop="confirmPassword">
          <el-input v-model="form.confirmPassword" type="password" />
        </el-form-item>
      </el-form>
      <template #footer center>
        <el-button type="" @click="cancelChangePwd">取消</el-button>
        <el-button type="primary" @click="submitPwd">提交</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ElMessageBox } from 'element-plus'
import Breadcrumb from '@/components/Breadcrumb'
import TopNav from '@/components/TopNav'
import Hamburger from '@/components/Hamburger'
// import Screenfull from '@/components/Screenfull'
import SizeSelect from '@/components/SizeSelect'
import HeaderSearch from '@/components/HeaderSearch'
import useAppStore from '@/store/modules/app'
import useUserStore from '@/store/modules/user'
import useSettingsStore from '@/store/modules/settings'
import Setting from '@/views/setting/index.vue';
import CloseBar from '@/components/close_bar'
import { updatePwd } from '@/api/system/user'

const { proxy } = getCurrentInstance();

const appStore = useAppStore()
const userStore = useUserStore()
const settingsStore = useSettingsStore()
const showSetting = ref(false);
const showChangePwd = ref(false);
const form = ref({
  oldPassword: '',
  newPassword: '',
  confirmPassword: ''
});
const formRef = ref();

function reset() {
  form.value = {
    oldPassword: '',
    newPassword: '',
    confirmPassword: ''
  };
  proxy.resetForm("formRef");
}

// 验证规则
const validateOldPass = (rule, value, callback) => {
  if (value === '') {
    callback(new Error('请输入旧密码'));
  } else if (value === form.value.newPassword) {
    callback(new Error('新密码不能与旧密码相同'));
  } else {
    callback();
  }
};

const validateNewPass = (rule, value, callback) => {
  if (value === '') {
    callback(new Error('请输入新密码'));
  } else if (value.length < 6 || value.length > 20) {
    callback(new Error('长度在 6 到 20 个字符'));
  } else {
    callback();
  }
};

const validateConfirmPass = (rule, value, callback) => {
  if (value === '') {
    callback(new Error('请再次输入密码'));
  } else if (value !== form.value.newPassword) {
    callback(new Error('两次输入密码不一致!'));
  } else {
    callback();
  }
};

const rules = reactive({
  oldPassword: [
    { validator: validateOldPass, trigger: 'blur' },
  ],
  newPassword: [
    { validator: validateNewPass, trigger: 'blur' },
  ],
  confirmPassword: [
    { validator: validateConfirmPass, trigger: 'blur' },
  ]
});
function toggleSideBar() {
  appStore.toggleSideBar()
}

function submitPwd() {
  proxy.$refs["formRef"].validate(valid => {
    if (valid) {
      form.value.account = userStore.account;
      updatePwd(form.value).then(() => {
        // 修改成功，正在跳转登录页面
        proxy.$modal.msgSuccess('密码修改成功，正在退出登录');
        userStore.logOut().then(() => {
          location.href = '/index';
        })
      }).catch(err => {})
    }
  })
}

function cancelChangePwd() {
  reset();
  showChangePwd.value = false;
}

function handleCommand(command) {
  switch (command) {
    case "setLayout":
      setLayout();
      break;
    case "logout":
      logout();
      break;
    default:
      break;
  }
}

function logout() {
  ElMessageBox.confirm('确定注销并退出系统吗？', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(() => {
    userStore.logOut().then(() => {
      location.href = '/index';
    })
  }).catch(() => { });
}

const emits = defineEmits(['setLayout'])
function setLayout() {
  emits('setLayout');
}

import { resourceDir } from '@tauri-apps/api/path';
import { convertFileSrc } from '@tauri-apps/api/core';

const imageUrl = ref(null);

onMounted(async () => {
  try {
    // 获取应用数据目录
    const appDir = await resourceDir();

    const avatar = userStore.avatar;
    // 构建完整的文件路径
    const fullPath = `${appDir}/${avatar}`;

    // 使用 convertFileSrc 将文件路径转换为可访问的 URL
    imageUrl.value = convertFileSrc(fullPath);
  } catch (error) {
    console.error('Error fetching image URL:', error);
  }
});
</script>

<style lang='scss' scoped>
.navbar {
  height: 80px;
  overflow: hidden;
  position: relative;
  background: #fff;
  box-shadow: 0 1px 4px rgba(0, 21, 41, 0.08);

  .hamburger-container {
    line-height: 46px;
    height: 100%;
    float: left;
    cursor: pointer;
    transition: background 0.3s;
    -webkit-tap-highlight-color: transparent;

    &:hover {
      background: rgba(0, 0, 0, 0.025);
    }
  }

  .breadcrumb-container {
    float: left;
  }

  .topmenu-container {
    position: absolute;
    left: 50px;
  }

  .errLog-container {
    display: inline-block;
    vertical-align: top;
  }

  .right-menu {
    float: right;
    height: 100%;
    line-height: 50px;
    display: flex;

    &:focus {
      outline: none;
    }

    .right-menu-item {
      display: inline-block;
      padding: 0 8px;
      height: 100%;
      font-size: 18px;
      color: #5a5e66;
      vertical-align: text-bottom;

      &.hover-effect {
        cursor: pointer;
        transition: background 0.3s;

        &:hover {
          background: rgba(0, 0, 0, 0.025);
        }
      }
    }

    .avatar-container {
      margin-right: 40px;

      .avatar-wrapper {
        margin-top: 5px;
        position: relative;

        .user-avatar {
          cursor: pointer;
          width: 40px;
          height: 40px;
          border-radius: 10px;
        }

        i {
          cursor: pointer;
          position: absolute;
          right: -20px;
          top: 25px;
          font-size: 12px;
        }
      }
    }
  }
}
</style>
