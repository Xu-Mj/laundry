<template>
  <div class="sidebar-logo-container">
    <transition name="sidebarLogoFade">
      <router-link key="collapse" class="sidebar-logo-link" to="/">
        <img :src="logo" class="sidebar-logo" />
      </router-link>
    </transition>
  </div>
</template>

<script setup>
import logo from '@/assets/logo/logo.svg'
</script>

<style lang="scss" scoped>
.sidebar-logo-container {
  position: relative;
  height: 50px;
  line-height: 50px;
  text-align: center;
  overflow: hidden;
  margin: .5rem;
  border-radius: .4rem;

  &::before {
    content: "";
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    backdrop-filter: blur(40px);
    z-index: -1;
  }

  & .sidebar-logo-link {
    height: 100%;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;

    & .sidebar-logo {
      width: 32px;
      height: 32px;
      vertical-align: middle;
      border-radius: .3rem;
    }
  }
}
</style>