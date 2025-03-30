<template>
  <AppWatermark>
    <Sidebar class="sidebar-container" />
    <div class="main-container">
      <CloseBar />
      <app-main />
    </div>
    <LockScreen @unlock="handleUnlock" />
    <TrialExpired />
  </AppWatermark>
</template>

<script setup>
import Sidebar from './components/Sidebar/index.vue'
import { AppMain } from './components'
import CloseBar from '@/components/close_bar'
import AppWatermark from '@/components/AppWatermark/index.vue'
import LockScreen from '@/components/LockScreen/index.vue'
import TrialExpired from '@/components/TrialExpired/index.vue'

// 锁屏功能 - 简化后的解锁处理函数
const handleUnlock = () => {
  // 用户成功解锁后的回调
  console.log('用户已解锁');
};
</script>

<style lang="scss" scoped>
@import "@/assets/styles/variables.module.scss";

.app-wrapper {
  position: relative;
  height: 100%;
  width: 100%;
}
</style>