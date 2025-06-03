<template>
  <div class="sun-container" :class="position">
    <!-- <div class="sun-halo"></div> -->
    <!-- <div class="sun">
      <div class="sun-core"></div>
    </div> -->
    <div class="rays-container">

      <!-- 光晕粒子 -->
      <div class="particles">
        <div class="particle p1"></div>
        <div class="particle p2"></div>
        <div class="particle p3"></div>
        <div class="particle p4"></div>
        <div class="particle p5"></div>
        <div class="particle p6"></div>
        <div class="particle p7"></div>
        <div class="particle p8"></div>
        <div class="particle p9"></div>
        <div class="particle p10"></div>
      </div>
    </div>
  </div>
</template>

<script setup>
const props = defineProps({
  position: {
    type: String,
    default: 'top-right',
    validator: (value) => ['top-right', 'top-left'].includes(value)
  }
});
</script>

<style scoped>
.sun-container {
  position: fixed;
  z-index: 0;
  pointer-events: none;
  overflow: visible; /* 改为visible以允许光线溢出容器 */
  width: 100vw;
  height: 100vh;
}

/* 光线容器 */
.rays-container {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  transform-origin: center center;
  animation: slow-rotate 180s linear infinite;
}

/* 光晕粒子 */
.particles {
  position: absolute;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
}

.particle {
  position: absolute;
  width: 8px;
  height: 8px;
  background-color: rgba(255, 255, 255, 0.8);
  border-radius: 50%;
  filter: blur(2px);
  box-shadow: 0 0 10px rgba(255, 255, 255, 0.8);
  animation: float 10s infinite linear;
}

/* 粒子位置 - 更分散的分布 */
.p1 { top: 50px; left: 100px; animation-delay: 0s; }
.p2 { top: 120px; left: 200px; animation-delay: 1s; }
.p3 { top: 180px; left: 80px; animation-delay: 2s; }
.p4 { top: 90px; left: 250px; animation-delay: 3s; }
.p5 { top: 220px; left: 150px; animation-delay: 4s; }
.p6 { top: 300px; left: 100px; animation-delay: 5s; }
.p7 { top: 250px; left: 300px; animation-delay: 6s; }
.p8 { top: 150px; left: 350px; animation-delay: 7s; }
.p9 { top: 350px; left: 200px; animation-delay: 8s; }
.p10 { top: 200px; left: 50px; animation-delay: 9s; }

/* 动画效果 */
@keyframes slow-rotate {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

@keyframes float {
  0% { transform: translate(0, 0); opacity: 0; }
  10% { opacity: 0.8; }
  50% { transform: translate(50px, 50px); opacity: 1; }
  90% { opacity: 0.8; }
  100% { transform: translate(0, 0); opacity: 0; }
}
</style>