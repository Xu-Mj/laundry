<template>
  <div class="navbar">
    <div class="avatar-row">
      <div class="avatar-container">
        <el-dropdown @command="handleCommand" class="right-menu-item hover-effect" trigger="click">
          <div class="avatar-wrapper">
            <img :src="userStore.avatar" class="user-avatar" />
          </div>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item @click="showChangePwd = true">修改密码</el-dropdown-item>
              <router-link to="/profile">
                <el-dropdown-item>个人中心</el-dropdown-item>
              </router-link>
              <el-dropdown-item divided command="logout">
                <span>退出登录</span>
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
    </div>
    <div class="controls-row">
      <ThemeSwitch />
      <el-dropdown @command="handleCommand" class="right-menu-item hover-effect" trigger="click">
        <span class="setting-icon">
          <el-icon size="20">
            <component is="Setting" />
          </el-icon>
        </span>
        <template #dropdown>
          <el-dropdown-menu>
            <el-dropdown-item @click="showSetting = true">选择打印机</el-dropdown-item>
            <el-dropdown-item command="goAdmin">
              <span>{{ props.isAdmin ? '后台管理' : '返回' }}</span>
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
import ThemeSwitch from '@/components/ThemeSwitch.vue';
import { ElMessageBox } from 'element-plus'
import useUserStore from '@/store/modules/user'
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

</script>

<style lang='scss' scoped>
.navbar {
  width: 100%;
  overflow: hidden;
  position: absolute;
  bottom: 0;
  left: 0;
  padding: 1rem;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;

  .avatar-row {
    width: 100%;
    display: flex;
    justify-content: flex-start;
  }

  .controls-row {
    width: 100%;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .setting-icon {
    cursor: pointer;
    border-radius: 4px;
    transition: background-color 0.3s;
    color: var(--el-color-primary);
  }

  .setting-icon:hover {
    background-color: var(--el-fill-color-light);
  }

  :deep(.el-icon) {
    transition: transform 0.3s;
  }

  .setting-icon:hover :deep(.el-icon) {
    transform: scale(1.2);
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
}
</style>
