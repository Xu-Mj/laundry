<template>
  <div style="height: 100%;">
    <OrderContent :taggle="() => { router.push('/') }" />
    <el-dialog v-model="showWarning" :show-close="false" width="300px" center="true">
      <p style="text-align: center;">您将在 {{ countdown }} 秒后自动跳转至首页。</p>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue';
import OrderContent from '@/views/home/oderContent.vue';
import { useRouter } from 'vue-router';

const router = useRouter();
const showWarning = ref(false);
const timeoutLength = 60 * 5; // 5 minutes
const countdownInit = 30; // 5 minutes
const countdown = ref(countdownInit);
let warningTimeoutId;
let redirectTimeoutId;
let countdownIntervalId;

const resetTimeout = () => {
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
        clearInterval(countdownIntervalId);
      }
    }, 1000);

    redirectTimeoutId = setTimeout(() => {
      router.push('/');
    }, 30000); // 30 seconds
  }, (timeoutLength - countdownInit) * 1000); // 4 minutes 30 seconds
};

const handleUserInteraction = () => {
  resetTimeout();
};

onMounted(() => {
  resetTimeout();
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