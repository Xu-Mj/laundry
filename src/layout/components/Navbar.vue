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
              <el-dropdown-item @click="showChangePwd = true" v-if="!isGuest">修改密码</el-dropdown-item>
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
      <span class="setting-icon">
        <el-icon size="20">
          <component is="Message" />
        </el-icon>
      </span>
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
              <span>{{ props.isAdmin ? '返回' : '后台管理' }}</span>
            </el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
    </div>
    <Setting :visible="showSetting" :key="showSetting" :taggle="() => { showSetting = !showSetting }" />

    </div>
    <ChangePwdDialog :visible="showChangePwd" :account="userStore.account"
      @update:visible="(val) => showChangePwd = val" @success="handlePwdSuccess"
      @cancel="() => showChangePwd = false" />
</template>

<script setup>
import ThemeSwitch from '@/components/ThemeSwitch.vue';
import { ElMessageBox } from 'element-plus'
import useUserStore from '@/store/modules/user'
import Setting from '@/views/setting/index.vue';
import ChangePwdDialog from '@/components/ChangePwdDialog/index.vue';

const props = defineProps({
  switch: Function,
  isAdmin: Boolean,
})

const { proxy } = getCurrentInstance();

const userStore = useUserStore()
const isGuest = ref(userStore.trial.isGuest);
const showSetting = ref(false);
const showChangePwd = ref(false);

// 处理密码修改成功事件
function handlePwdSuccess() {
  proxy.notify.success('密码修改成功，正在退出登录');
  userStore.logOut().then(() => {
    location.href = '/index';
  });
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
      location.href = '/';
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
    justify-content: space-between;
  }

  .controls-row {
    width: 100%;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .setting-icon {
    display: flex;
    justify-content: center;
    align-items: center;
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
