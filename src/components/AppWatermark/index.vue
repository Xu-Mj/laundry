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
import useUserStore from '@/store/modules/user';
import { useDark } from '@vueuse/core';

const isDark = useDark();
const userStore = useUserStore();

const font = reactive({
  color: 'rgba(0, 0, 0, .15)',
});

// 是否显示水印
const showWatermark = computed(() => userStore.sub.showWatermark)
// const showWatermark = false

// 水印内容
const watermarkContent = computed(() => {
  if (!userStore.sub.isInTrial) return ''
  
  if (userStore.isGuest) {
    return `游客模式 - 剩余${userStore.sub.trialRemaining}天`
  } else {
    return `试用版 - 剩余${userStore.sub.trialRemaining}天`
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

</script>

<style scoped>
.app-watermark {
  width: 100%;
  height: 100%;
}
</style>