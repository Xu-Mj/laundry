<template>
  <section class="app-main">
    <router-view v-slot="{ Component, route }">
      <transition name="fade-transform" mode="out-in" enter-from-class="fade-transform-enter">
        <keep-alive :include="tagsViewStore.cachedViews">
          <component :is="Component" :key="route.path" />
        </keep-alive>
      </transition>
    </router-view>
  </section>
  <el-dialog v-model="showWarning" :show-close="false" width="300px" center="true">
    <p style="text-align: center;">您将在 {{ countdown }} 秒后自动注销当前登录。</p>
  </el-dialog>
</template>

<script setup>
import useTagsViewStore from '@/store/modules/tagsView'
import { getConfigKey } from '@/api/system/config';
import useUserStore from '@/store/modules/user';

const userStore = useUserStore();
const tagsViewStore = useTagsViewStore()
console.log('tagsViewStore', tagsViewStore)
const showWarning = ref(false);
const defaultTimeoutLength = 300; // 5 minutes
const timeOut = ref(defaultTimeoutLength);
const countdownInit = 30; // 30s
const countdown = ref(countdownInit);
let warningTimeoutId;
let redirectTimeoutId;
let countdownIntervalId;

const resetTimeout = (timeoutLength = defaultTimeoutLength) => {
  if (warningTimeoutId) {
    clearTimeout(warningTimeoutId);
  }
  if (redirectTimeoutId) {
    clearTimeout(redirectTimeoutId);
  }
  if (countdownIntervalId) {
    clearInterval(countdownIntervalId);
  }
  showWarning.value = false;
  countdown.value = countdownInit;

  warningTimeoutId = setTimeout(() => {
    showWarning.value = true;
    countdownIntervalId = setInterval(() => {
      countdown.value -= 1;
      if (countdown.value <= 0) {
        showWarning.value = false;
        clearInterval(countdownIntervalId);
        userStore.logOut().then(() => {
          location.href = '/index';
        })
      }
    }, 1000);
  }, (timeoutLength - countdownInit) * 1000); // 4 minutes 30 seconds
};

const debounce = (func, wait) => {
  let timeout;
  return (...args) => {
    clearTimeout(timeout);
    timeout = setTimeout(() => func.apply(this, args), wait);
  };
};

const handleUserInteraction = debounce(() => {
  resetTimeout(timeOut.value);
}, 300); // Debounce with 300ms delay

onMounted(async () => {
  // get timeoutLength from server
  const config = await getConfigKey('logout_timeout');
  timeOut.value = config ? Number(config.configValue) : defaultTimeoutLength;
  resetTimeout(timeOut.value);
  window.addEventListener('mousemove', handleUserInteraction);
  window.addEventListener('keydown', handleUserInteraction);
});

onBeforeUnmount(() => {
  if (warningTimeoutId) {
    clearTimeout(warningTimeoutId);
  }
  if (redirectTimeoutId) {
    clearTimeout(redirectTimeoutId);
  }
  if (countdownIntervalId) {
    clearInterval(countdownIntervalId);
  }
  window.removeEventListener('mousemove', handleUserInteraction);
  window.removeEventListener('keydown', handleUserInteraction);
});
</script>

<style lang="scss" scoped>
.app-main {
  /* 50= navbar  50  */
  // min-height: calc(100vh - 35px);
  width: 100%;
  height: 100%;
  position: relative;
  // overflow: hidden;
  padding: 2rem 1rem;
}

.fixed-header+.app-main {
  padding-top: 50px;
}

.hasTagsView {
  .fixed-header+.app-main {
    padding-top: 114px;
  }
}
</style>

<style lang="scss">
// fix css style bug in open el-dialog
.el-popup-parent--hidden {
  .fixed-header {
    padding-right: 6px;
  }
}

::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

::-webkit-scrollbar-track {
  background-color: #f1f1f1;
}

::-webkit-scrollbar-thumb {
  background-color: #c0c0c0;
  border-radius: 3px;
}
</style>
