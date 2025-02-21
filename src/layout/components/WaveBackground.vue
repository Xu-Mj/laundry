<!-- WaveBackground.vue -->
<template>
    <div class="wave-background">
        <canvas ref="waveCanvas"></canvas>
    </div>
</template>

<script setup>
import { onMounted, ref, onBeforeUnmount } from 'vue';

const waveCanvas = ref(null);
let canvasContext;
let waves = [];

onMounted(() => {
    if (waveCanvas.value) {
        const canvas = waveCanvas.value;
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        canvasContext = canvas.getContext('2d');

        initWaves();
        animate();

        window.addEventListener('resize', resizeCanvas);
    } else {
        console.error('Canvas element not found');
    }
});

onBeforeUnmount(() => {
    window.removeEventListener('resize', resizeCanvas);
});
const colorSet = [
    'rgba(241, 251, 247, .5)',
    'rgba(244, 247, 239, .6)',
    'rgba(240, 249, 252, .7)',
    'rgba(240, 249, 252, .8)',
];

function initWaves() {
  waves = [];
  for (let i = 0; i < 4; i++) {
    waves.push({
      amplitude: Math.random() * 50 + 30,
      wavelength: Math.random() * 100 + 50,
      speed: Math.random()  - 0.05,
      phase: Math.random() * Math.PI * 2 +i * Math.PI / 3,
      color: colorSet[i]
    //   color: `rgba(${Math.random() * 255}, ${Math.random() * 255}, ${Math.random() * 255}, ${0.5 + (i * 0.1)})` // 完全随机的颜色
    //   color: `rgba(${Math.random() * 255}, ${Math.random() * 255}, ${Math.random() * 255}, ${0.5 + (i * 0.1)})` // 完全随机的颜色
    });
  }
}

function drawWave(wave, canvasWidth, canvasHeight) {
    canvasContext.beginPath();
    canvasContext.moveTo(0, wave.amplitude * Math.sin((0 + wave.phase) / wave.wavelength) + canvasHeight / 2);

    for (let x = 0; x <= canvasWidth; x++) {
        canvasContext.lineTo(
            x,
            wave.amplitude * Math.sin((x + wave.phase) / wave.wavelength) + canvasHeight / 2
        );
    }

    canvasContext.lineTo(canvasWidth, canvasHeight);
    canvasContext.lineTo(0, canvasHeight);
    canvasContext.closePath();
    canvasContext.fillStyle = wave.color;
    canvasContext.fill();
}

function animate() {
    if (!waveCanvas.value) {
        console.error('Canvas element not found in animate');
        return;
    }

    const canvas = waveCanvas.value;
    const canvasWidth = canvas.width;
    const canvasHeight = canvas.height;

    canvasContext.clearRect(0, 0, canvasWidth, canvasHeight);

    waves.forEach(wave => {
        drawWave(wave, canvasWidth, canvasHeight);
        wave.phase += wave.speed;
    });

    requestAnimationFrame(animate);
}

function resizeCanvas() {
    if (waveCanvas.value) {
        const canvas = waveCanvas.value;
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        initWaves();
    }
}
</script>

<style scoped>
.wave-background {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: -1;
    /* 确保背景在最底层 */
    background-color: #08d3f3;
    /* 设置背景颜色为黑色 */
}
</style>