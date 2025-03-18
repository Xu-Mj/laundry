<template>
  <div class="qrcode-scanner">
    <div class="scanner-container">
      <video ref="videoRef" class="scanner-video" autoplay playsinline></video>
      <canvas ref="canvasRef" class="scanner-canvas"></canvas>
      <div class="scanner-overlay">
        <div class="scanner-frame"></div>
        <div class="scanner-line"></div>
        <div v-if="scanSuccess" class="scanner-success-overlay">
          <el-icon class="success-icon">
            <Check />
          </el-icon>
          <div class="success-text">扫描成功</div>
        </div>
      </div>
    </div>
    <div v-if="errorMessage" class="scanner-error">
      {{ errorMessage }}
      <el-button size="small" type="primary" @click="retryScanner">重试</el-button>
    </div>
    <div v-if="!autoStart && !isScanning && !scanSuccess" class="scanner-tip">
      <el-button type="primary" @click="startScanner">开始扫码</el-button>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount, watch } from 'vue';
import jsQR from 'jsqr';
import { Check } from '@element-plus/icons-vue';
import { ElLoading } from 'element-plus';

const props = defineProps({
  onScanSuccess: {
    type: Function,
    required: true
  },
  autoStart: {
    type: Boolean,
    default: true
  },
  scanTimeout: {
    type: Number,
    default: 30000 // 默认30秒超时
  }
});

const emit = defineEmits(['close', 'scan-success', 'scan-timeout', 'scan-error']);

const videoRef = ref(null);
const canvasRef = ref(null);
const isScanning = ref(false);
const errorMessage = ref('');
const scanSuccess = ref(false);
let videoStream = null;
let scanInterval = null;
let timeoutTimer = null;
let loadingInstance = null;

// 开始扫码
const startScanner = async () => {
  errorMessage.value = '';
  scanSuccess.value = false;
  try {
    // 显示加载中状态
    loadingInstance = ElLoading.service({
      target: '.scanner-container',
      text: '正在启动摄像头...',
      background: 'rgba(0, 0, 0, 0.7)'
    });

    const devices = await navigator.mediaDevices.enumerateDevices();
    const videoDevices = devices.filter(device => device.kind === 'videoinput');

    if (videoDevices.length === 0) {
      errorMessage.value = '没有可用的摄像头设备';
      if (loadingInstance) loadingInstance.close();
      emit('scan-error', '没有可用的摄像头设备');
      return;
    }

    const stream = await navigator.mediaDevices.getUserMedia({
      video: {
        deviceId: videoDevices[0].deviceId,
        width: { ideal: 1280 },
        height: { ideal: 720 },
        facingMode: 'environment' // 优先使用后置摄像头
      }
    });

    // const stream = await navigator.mediaDevices.getDisplayMedia({
    //   video: true
    // });
    // videoRef.value.srcObject = stream;

    videoRef.value.srcObject = stream;
    videoStream = stream;
    isScanning.value = true;

    // 关闭加载状态
    if (loadingInstance) loadingInstance.close();

    // 开始定时扫描视频帧
    scanInterval = setInterval(scanVideoFrame, 200); // 每200ms扫描一次

    // 设置超时定时器
    if (props.scanTimeout > 0) {
      timeoutTimer = setTimeout(() => {
        if (isScanning.value) {
          stopScanner();
          errorMessage.value = '扫描超时，请重试';
          emit('scan-timeout');
        }
      }, props.scanTimeout);
    }
  } catch (error) {
    console.error('无法访问摄像头:', error);
    errorMessage.value = `无法访问摄像头: ${error.message}`;
    emit('scan-error', error.message);
    if (loadingInstance) loadingInstance.close();
  }
};

// 重试扫描
const retryScanner = () => {
  errorMessage.value = '';
  startScanner();
};

// 停止扫码
const stopScanner = () => {
  if (videoStream) {
    const tracks = videoStream.getTracks();
    tracks.forEach(track => track.stop());
    videoRef.value.srcObject = null;
    videoStream = null;
  }

  if (scanInterval) {
    clearInterval(scanInterval);
    scanInterval = null;
  }

  if (timeoutTimer) {
    clearTimeout(timeoutTimer);
    timeoutTimer = null;
  }

  isScanning.value = false;
};

// 扫描视频帧
const scanVideoFrame = () => {
  if (!isScanning.value || !videoRef.value || !canvasRef.value) return;

  const video = videoRef.value;
  const canvas = canvasRef.value;
  const context = canvas.getContext('2d');

  // 确保视频已经加载
  if (video.readyState !== video.HAVE_ENOUGH_DATA) return;

  // 设置canvas尺寸与视频一致
  canvas.width = video.videoWidth;
  canvas.height = video.videoHeight;

  // 在canvas上绘制当前视频帧
  context.drawImage(video, 0, 0, canvas.width, canvas.height);

  // 获取图像数据
  const imageData = context.getImageData(0, 0, canvas.width, canvas.height);

  // 使用jsQR库识别二维码
  const code = jsQR(imageData.data, imageData.width, imageData.height, {
    inversionAttempts: 'dontInvert' // 不尝试反转图像
  });

  if (code) {
    console.log('扫描到二维码:', code.data);
    // 标记扫描成功
    scanSuccess.value = true;

    // 发出扫描成功事件
    emit('scan-success', code.data);

    // 延迟一小段时间以显示成功动画，然后处理结果
    setTimeout(() => {
      props.onScanSuccess(code.data);
      // 停止扫描
      stopScanner();
    }, 800);
  }
};

// 组件挂载后自动启动扫描
onMounted(() => {
  if (props.autoStart) {
    startScanner();
  }
});

// 组件卸载前清理资源
onBeforeUnmount(() => {
  stopScanner();
  if (loadingInstance) {
    loadingInstance.close();
  }
});
</script>

<style scoped>
.qrcode-scanner {
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 100%;
  height: 100%;
}

.scanner-container {
  position: relative;
  width: 100%;
  max-width: 500px;
  height: 350px;
  overflow: hidden;
  border-radius: 8px;
  margin-bottom: 16px;
  background-color: #000;
}

.scanner-video {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.scanner-canvas {
  display: none;
}

.scanner-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
}

.scanner-frame {
  position: absolute;
  top: 50%;
  left: 50%;
  width: 200px;
  height: 200px;
  transform: translate(-50%, -50%);
  border: 2px solid var(--el-color-primary);
  border-radius: 8px;
  box-shadow: 0 0 0 100vmax rgba(0, 0, 0, 0.5);
}

.scanner-line {
  position: absolute;
  top: 50%;
  left: 50%;
  width: 196px;
  height: 2px;
  background-color: var(--el-color-primary);
  transform: translate(-50%, -50%);
  animation: scan-line 2s linear infinite;
}

.scanner-success-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.7);
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  color: #fff;
  animation: fade-in 0.3s ease-in-out;
}

.success-icon {
  font-size: 48px;
  color: var(--el-color-success);
  margin-bottom: 16px;
}

.success-text {
  font-size: 18px;
  font-weight: 500;
}

.scanner-error {
  margin-top: 16px;
  color: var(--el-color-danger);
  text-align: center;
}

.scanner-tip {
  margin-top: 16px;
  text-align: center;
}

@keyframes fade-in {
  from {
    opacity: 0;
  }

  to {
    opacity: 1;
  }
}

@keyframes scan-line {
  0% {
    transform: translate(-50%, -100px);
  }

  50% {
    transform: translate(-50%, 100px);
  }

  100% {
    transform: translate(-50%, -100px);
  }
}

.scanner-controls {
  display: flex;
  gap: 12px;
  margin-top: 16px;
}

.scanner-error {
  color: var(--el-color-danger);
  margin-top: 12px;
  text-align: center;
}
</style>