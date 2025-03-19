<template>
  <AppWatermark>
    <SidebarNew class="sidebar-container" />
    <div :class="{ hasTagsView: needTagsView, sidebarHide: sidebar.hide }" class="main-container">
      <CloseBar />
      <app-main />
    </div>
    <LockScreen @unlock="handleUnlock" />
  </AppWatermark>
</template>

<script setup>
import { useWindowSize } from '@vueuse/core'
import SidebarNew from './components/SidebarNew/index.vue'
import { AppMain } from './components'
import CloseBar from '@/components/close_bar'
import AppWatermark from '@/components/AppWatermark/index.vue'
import LockScreen from '@/components/LockScreen/index.vue'

import useAppStore from '@/store/modules/app'
import useSettingsStore from '@/store/modules/settings'

const settingsStore = useSettingsStore()
const sidebar = computed(() => useAppStore().sidebar);
const device = computed(() => useAppStore().device);
const needTagsView = computed(() => settingsStore.tagsView);

const { width } = useWindowSize();
const WIDTH = 992; // refer to Bootstrap's responsive design

watch(() => device.value, () => {
  if (device.value === 'mobile' && sidebar.value.opened) {
    useAppStore().closeSideBar({ withoutAnimation: false })
  }
})

watchEffect(() => {
  if (width.value - 1 < WIDTH) {
    useAppStore().toggleDevice('mobile')
    useAppStore().closeSideBar({ withoutAnimation: true })
  } else {
    useAppStore().toggleDevice('desktop')
  }
})

// 锁屏功能 - 简化后的解锁处理函数
const handleUnlock = () => {
  // 用户成功解锁后的回调
  console.log('用户已解锁');
};
</script>

<style lang="scss" scoped>
@import "@/assets/styles/mixin.scss";
@import "@/assets/styles/variables.module.scss";

.app-wrapper {
  @include clearfix;
  position: relative;
  height: 100%;
  width: 100%;

  &.mobile.openSidebar {
    position: fixed;
    top: 0;
  }
}
</style>