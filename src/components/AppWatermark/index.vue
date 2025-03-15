<template>
  <el-watermark
    v-if="showWatermark"
    :content="watermarkContent"
    :font="font"
    :zIndex="9999"
    class="app-watermark"
  >
  <slot></slot>
  </el-watermark>
  <template v-else>
    <slot></slot>
  </template>
</template>

<script setup>
import { computed, onMounted } from 'vue'
import useSubscriptionStore from '@/store/modules/subscription'
import { useDark } from '@vueuse/core';

const isDark = useDark();
const subscriptionStore = useSubscriptionStore();

const font = reactive({
  color: 'rgba(0, 0, 0, .15)',
});

// 是否显示水印
const showWatermark = computed(() => subscriptionStore.showWatermark)

// 水印内容
const watermarkContent = computed(() => {
  if (!subscriptionStore.isInTrial) return ''
  
  if (subscriptionStore.isGuest) {
    return `游客模式 - 剩余${subscriptionStore.trialRemaining}天`
  } else {
    return `试用版 - 剩余${subscriptionStore.trialRemaining}天`
  }
})

watch(
  isDark,
  () => {
    font.color = isDark.value
      ? 'rgba(255, 255, 255, .15)'
      : 'rgba(0, 0, 0, .15)'
  },
  {
    immediate: true,
  }
)

// 组件挂载时获取订阅信息
onMounted(async () => {
  await subscriptionStore.getSubscriptionInfo()
})
</script>

<style scoped>
.app-watermark {
  width: 100%;
  height: 100%;
}
</style>