<template>
  <router-view />
</template>

<script setup>
import useSettingsStore from '@/store/modules/settings'
import { handleThemeStyle } from '@/utils/theme'

onMounted(() => {
  nextTick(() => {
    // 初始化主题样式
    handleThemeStyle(useSettingsStore().theme)
  })
})
</script>

<style>
:root {
  view-transition-name: root;
}

@supports not (view-transition-name: root) {
  /* 当浏览器不支持 View Transitions API 时的降级处理 */
  :root {
    --transition-duration: 0.5s;
  }
}
</style>
