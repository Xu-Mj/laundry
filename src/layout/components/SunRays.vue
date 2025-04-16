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

.top-right {
  top: -150px;
  right: -150px;
}

.top-left {
  top: -150px;
  left: -150px;
}

/* 太阳光晕 */
.sun-halo {
  position: absolute;
  width: 500px;
  height: 500px;
  border-radius: 50%;
  background: radial-gradient(
    circle,
    rgba(255, 255, 255, 0.9) 0%,
    rgba(255, 236, 184, 0.7) 15%,
    rgba(255, 213, 79, 0.5) 30%,
    rgba(255, 152, 0, 0.3) 50%,
    rgba(255, 152, 0, 0) 80%
  );
  filter: blur(15px);
  opacity: 0.9;
  transform: translate(-150px, -150px);
  animation: pulse-halo 8s infinite alternate;
}

/* 太阳主体 */
.sun {
  width: 200px;
  height: 200px;
  position: relative;
  border-radius: 50%;
  background: radial-gradient(
    circle,
    rgba(255, 255, 255, 1) 0%,
    rgba(255, 236, 184, 1) 20%,
    rgba(255, 213, 79, 1) 50%,
    rgba(255, 152, 0, 1) 100%
  );
  box-shadow: 
    0 0 80px rgba(255, 235, 59, 0.9),
    0 0 150px rgba(255, 152, 0, 0.6),
    0 0 200px rgba(255, 152, 0, 0.3);
  animation: pulse 6s infinite alternate;
}

.sun-core {
  position: absolute;
  width: 120px;
  height: 120px;
  top: 40px;
  left: 40px;
  border-radius: 50%;
  background: radial-gradient(
    circle,
    rgba(255, 255, 255, 1) 0%,
    rgba(255, 236, 184, 0.8) 40%,
    rgba(255, 213, 79, 0) 100%
  );
  filter: blur(4px);
  animation: core-pulse 4s infinite alternate;
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

/* 主光柱 */
.ray-group {
  position: absolute;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
}

.ray {
  position: absolute;
  background: linear-gradient(
    to bottom,
    rgba(255, 255, 255, 0.9) 0%,
    rgba(255, 236, 184, 0.7) 20%,
    rgba(255, 213, 79, 0.5) 40%,
    rgba(255, 152, 0, 0.3) 60%,
    rgba(255, 152, 0, 0) 100%
  );
  width: 20px;
  height: 100vh; /* 延伸到整个视口高度 */
  left: 90px;
  top: -50px;
  transform-origin: bottom center;
  border-radius: 10px;
  filter: blur(3px);
  animation: flicker 5s infinite alternate;
  overflow: visible;
}

.main-rays .ray {
  height: 120vh; /* 超出视口高度 */
  width: 25px;
}

.secondary-rays .ray {
  width: 15px;
  height: 100vh;
  opacity: 0.7;
  left: 92px;
  animation-duration: 7s;
}

/* 丁达尔效应光束 */
.tyndall-rays {
  position: absolute;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
}

.tyndall-ray {
  position: absolute;
  width: 60px;
  height: 150vh; /* 超出视口高度 */
  background: linear-gradient(
    to bottom,
    rgba(255, 255, 255, 0.8) 0%,
    rgba(255, 236, 184, 0.6) 10%,
    rgba(255, 213, 79, 0.4) 30%,
    rgba(255, 152, 0, 0.2) 60%,
    rgba(255, 152, 0, 0) 100%
  );
  transform-origin: top center;
  border-radius: 30px;
  filter: blur(15px);
  opacity: 0.6;
  left: 70px;
  top: 0;
  animation: tyndall-flicker 8s infinite alternate;
  overflow: visible;
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

/* 光柱上的光晕 */
.ray::after {
  content: '';
  position: absolute;
  width: 15px;
  height: 15px;
  background-color: rgba(255, 255, 255, 0.9);
  border-radius: 50%;
  filter: blur(5px);
  box-shadow: 0 0 15px rgba(255, 255, 255, 0.8);
  animation: ray-glow 8s infinite alternate;
  opacity: 0;
}

/* 光线角度 - 更随机的分布 */
.ray1 { transform: rotate(0deg); animation-delay: 0.1s; }
.ray2 { transform: rotate(35deg); animation-delay: 0.2s; }
.ray3 { transform: rotate(75deg); animation-delay: 0.3s; }
.ray4 { transform: rotate(120deg); animation-delay: 0.4s; }
.ray5 { transform: rotate(165deg); animation-delay: 0.5s; }
.ray6 { transform: rotate(210deg); animation-delay: 0.6s; }
.ray7 { transform: rotate(255deg); animation-delay: 0.7s; }
.ray8 { transform: rotate(300deg); animation-delay: 0.8s; }

/* 丁达尔光束角度 - 更广泛的分布 */
.tyndall1 { transform: rotate(15deg); animation-delay: 0.2s; }
.tyndall2 { transform: rotate(55deg); animation-delay: 0.5s; }
.tyndall3 { transform: rotate(95deg); animation-delay: 0.8s; }
.tyndall4 { transform: rotate(135deg); animation-delay: 1.1s; }
.tyndall5 { transform: rotate(175deg); animation-delay: 1.4s; }

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
@keyframes pulse {
  0% { transform: scale(1); opacity: 0.9; }
  50% { transform: scale(1.05); opacity: 1; }
  100% { transform: scale(1); opacity: 0.95; }
}

@keyframes pulse-halo {
  0% { transform: translate(-50px, -50px) scale(1); opacity: 0.7; }
  50% { transform: translate(-50px, -50px) scale(1.1); opacity: 0.9; }
  100% { transform: translate(-50px, -50px) scale(1); opacity: 0.8; }
}

@keyframes core-pulse {
  0% { transform: scale(1); opacity: 0.9; }
  100% { transform: scale(1.1); opacity: 1; }
}

@keyframes flicker {
  0% { opacity: 0.7; height: 90vh; }
  50% { opacity: 0.9; height: 110vh; }
  100% { opacity: 0.8; height: 100vh; }
}

@keyframes tyndall-flicker {
  0% { opacity: 0.4; height: 140vh; }
  50% { opacity: 0.7; height: 160vh; }
  100% { opacity: 0.5; height: 150vh; }
}

@keyframes ray-glow {
  0% { opacity: 0; top: 20%; }
  25% { opacity: 0.8; }
  50% { opacity: 0.5; top: 50%; }
  75% { opacity: 0.9; }
  100% { opacity: 0; top: 80%; }
}

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