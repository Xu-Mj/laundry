<!-- <template>
    <div class="object-detection">
        <h1>实时物体识别</h1>
        <video ref="videoRef" autoplay playsinline muted></video>
        <canvas ref="canvasRef" class="overlay"></canvas>
        <button @click="startDetection" :disabled="isDetecting">开始检测</button>
        <button @click="stopDetection" :disabled="!isDetecting">停止检测</button>
        <div v-if="detections.length > 0" class="results">
            <h3>检测结果：</h3>
            <ul>
                <li v-for="(detection, index) in detections" :key="index">
                    {{ detection.class }} ({{ (detection.score * 100).toFixed(1) }}%)
                </li>
            </ul>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue'
import * as cocoSsd from '@tensorflow-models/coco-ssd'
import '@tensorflow/tfjs'

// Refs
const videoRef = ref(null)
const canvasRef = ref(null)
const isDetecting = ref(false)
const detections = ref([])

// 模型和视频流
let model = null
let stream = null


// 初始化摄像头
async function initCamera() {
    try {
        const stream = await navigator.mediaDevices.getUserMedia({
            video: {
                width: { ideal: 1920 },
                height: { ideal: 1080 }
            }
        });
        videoRef.value.srcObject = stream;
    } catch (error) {
        console.error('无法访问摄像头: ' + error);
        try {
            const stream = await navigator.mediaDevices.getDisplayMedia({
                video: true
            });
            videoRef.value.srcObject = stream;
        } catch (error) {
            console.error('无法访问桌面: ' + error);
        }
    }
}
// 加载 COCO-SSD 模型
async function loadModel() {
    model = await cocoSsd.load()
    console.log('模型加载完成')
}

// 执行检测
async function detectFrame() {
    if (!isDetecting.value || !model || !videoRef.value || !canvasRef.value) return

    // 执行检测
    const predictions = await model.detect(videoRef.value)

    // 更新检测结果
    detections.value = predictions.map((pred) => ({
        class: pred.class,
        score: pred.score,
    }))

    // 绘制检测框
    const ctx = canvasRef.value.getContext('2d')
    if (ctx) {
        ctx.clearRect(0, 0, canvasRef.value.width, canvasRef.value.height)
        predictions.forEach((pred) => {
            const [x, y, width, height] = pred.bbox
            ctx.strokeStyle = '#00FF00'
            ctx.lineWidth = 2
            ctx.strokeRect(x, y, width, height)
            ctx.fillStyle = '#00FF00'
            ctx.font = '16px Arial'
            ctx.fillText(`${pred.class} (${(pred.score * 100).toFixed(1)}%)`, x, y > 10 ? y - 5 : 10)
        })
    }

    // 循环检测
    requestAnimationFrame(detectFrame)
}

// 开始检测
function startDetection() {
    if (!model) {
        alert('模型未加载，请稍后再试。')
        return
    }
    isDetecting.value = true
    detectFrame()
}

// 停止检测
function stopDetection() {
    isDetecting.value = false
}

// 生命周期钩子
onMounted(async () => {
    await initCamera()
    await loadModel()
})

onBeforeUnmount(() => {
    if (stream) {
        stream.getTracks().forEach((track) => track.stop())
    }
})
</script>

<style scoped>
.object-detection {
    text-align: center;
    margin: 20px;
}

video,
canvas {
    width: 640px;
    height: 480px;
    border: 1px solid #ccc;
    margin: 10px 0;
}

.overlay {
    position: absolute;
    top: 0;
    left: 50%;
    transform: translateX(-50%);
    pointer-events: none;
}

button {
    margin: 5px;
    padding: 10px 20px;
    font-size: 16px;
}

.results {
    margin-top: 20px;
    text-align: left;
}
</style> -->