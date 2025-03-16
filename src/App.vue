<template>
    <router-view />
</template>

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
