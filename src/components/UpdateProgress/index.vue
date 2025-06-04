<template>
  <div v-if="showProgress" class="update-progress-overlay">
    <div class="update-progress-container">
      <h3>{{ title }}</h3>
      <div class="progress-bar-container">
        <div class="progress-bar" :style="{width: `${percent}%`}"></div>
      </div>
      <div class="progress-text">
        {{ statusText }}
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { listen } from '@tauri-apps/api/event'

const showProgress = ref(false)
const percent = ref(0)
const title = ref('正在更新...')
const statusText = ref('准备下载更新...')

// 存储事件取消器
let unlisten = []

onMounted(async () => {
  // 监听更新开始事件
  unlisten.push(
    await listen('update-started', () => {
      showProgress.value = true
      percent.value = 0
      title.value = '正在更新...'
      statusText.value = '准备下载更新...'
    })
  )

  // 监听更新进度
  unlisten.push(
    await listen('update-progress', (event) => {
      const progress = event.payload
      percent.value = progress.percent
      
      // 格式化下载大小
      const downloadedSize = formatBytes(progress.downloaded)
      const totalSize = progress.total ? formatBytes(progress.total) : '未知'
      
      statusText.value = `下载中: ${downloadedSize} / ${totalSize} (${progress.percent.toFixed(2)}%)`
    })
  )

  // 监听下载完成
  unlisten.push(
    await listen('update-downloaded', () => {
      statusText.value = '下载完成，正在安装...'
    })
  )

  // 监听安装完成
  unlisten.push(
    await listen('update-installed', () => {
      title.value = '更新完成'
      statusText.value = '更新已安装完成，即将重启应用...'
      setTimeout(() => {
        showProgress.value = false
      }, 2000)
    })
  )

  // 监听更新失败
  unlisten.push(
    await listen('update-failed', () => {
      title.value = '更新失败'
      statusText.value = '更新安装失败，请稍后再试'
      setTimeout(() => {
        showProgress.value = false
      }, 3000)
    })
  )
})

onUnmounted(() => {
  // 清理所有事件监听
  unlisten.forEach(unlistenFn => unlistenFn())
})

// 格式化字节大小
function formatBytes(bytes, decimals = 2) {
  if (bytes === 0) return '0 Bytes'
  
  const k = 1024
  const dm = decimals < 0 ? 0 : decimals
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB']
  
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  
  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i]
}
</script>

<style scoped>
.update-progress-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.7);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 9999;
}

.update-progress-container {
  background-color: white;
  padding: 20px;
  border-radius: 8px;
  width: 400px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

h3 {
  margin-top: 0;
  margin-bottom: 15px;
  color: #333;
}

.progress-bar-container {
  height: 10px;
  background-color: #e9ecef;
  border-radius: 5px;
  overflow: hidden;
  margin-bottom: 10px;
}

.progress-bar {
  height: 100%;
  background-color: #1890ff;
  transition: width 0.3s ease;
}

.progress-text {
  font-size: 14px;
  color: #666;
}
</style> 