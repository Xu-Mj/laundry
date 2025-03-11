<template>
  <div class="navbar">
    <el-button @click="toggleDarkMode" class="right-menu-item" icon="Moon" type="text" />
    <el-dropdown @command="handleCommand" class="right-menu-item hover-effect" trigger="click">
      <el-button class="right-menu-item" icon="Setting" type="text" />
      <!-- <div class="avatar-wrapper"> -->
      <!-- <img :src="userStore.avatar" class="user-avatar" />
        </div> -->
      <template #dropdown>
        <el-dropdown-menu>
          <el-dropdown-item @click="showSetting = true">选择打印机</el-dropdown-item>
          <el-dropdown-item command="goAdmin">
            <span>{{ props.isAdmin ? '后台管理' : '返回' }}</span>
          </el-dropdown-item>
        </el-dropdown-menu>
      </template>
    </el-dropdown>
    <div class="avatar-container">
      <el-dropdown @command="handleCommand" class="right-menu-item hover-effect" trigger="click">
        <div class="avatar-wrapper">
          <img :src="userStore.avatar" class="user-avatar" />
        </div>
        <template #dropdown>
          <el-dropdown-menu>
            <!-- <router-link to="/user/profile"> -->
            <el-dropdown-item @click="showChangePwd = true">修改密码</el-dropdown-item>
            <!-- </router-link> -->
            <!-- <el-dropdown-item command="setLayout" v-if="settingsStore.showSettings">
              <span>布局设置</span> -->
            <!-- </el-dropdown-item> -->
            <el-dropdown-item divided command="logout">
              <span>退出登录</span>
            </el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
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
import useUserStore from '@/store/modules/user'
import useSettingsStore from '@/store/modules/settings'
import Setting from '@/views/setting/index.vue';
import { updatePwd } from '@/api/system/user';
import { Window } from '@tauri-apps/api/window';

const props = defineProps({
  switch: Function,
  isAdmin: Boolean,
})

const { proxy } = getCurrentInstance();

const appWindow = new Window('main')
const userStore = useUserStore()
const showSetting = ref(false);
const showChangePwd = ref(false);
const form = ref({
  oldPassword: '',
  newPassword: '',
  confirmPassword: ''
});
const formRef = ref();

const isDarkMode = ref(localStorage.getItem('darkMode') === 'true');

function toggleDarkMode() {
  isDarkMode.value = !isDarkMode.value;
  document.documentElement.classList.toggle('dark', isDarkMode.value);
  localStorage.setItem('darkMode', isDarkMode.value);
}

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
      }).catch(err => { })
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
    case "goAdmin":
      props.switch();
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
    userStore.logOut().then(async () => {
      await appWindow.hide();
      location.href = '/index';
    })
  }).catch(() => { });
}

const emits = defineEmits(['setLayout'])
function setLayout() {
  emits('setLayout');
}

onMounted(() => {
  if (isDarkMode.value) {
    document.documentElement.classList.add('dark');
  }
});

</script>

<style lang='scss' scoped>
.navbar {
  // height: 80px;
  width: 100%;
  overflow: hidden;
  position: absolute;
  bottom: 0;
  left: 0;
  padding-bottom: 1rem;
  display: flex;
  align-items: end;
  justify-content: space-around;
  // gap: 1rem;


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

  .avatar-container {
    .avatar-wrapper {
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


  }
}
</style>
