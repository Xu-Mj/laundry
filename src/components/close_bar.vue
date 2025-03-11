<template>
    <div data-tauri-drag-region class="close-bar">
        <el-button link icon="Minus" @click="minimizeWindow" />
        <el-button link :icon="isMaximized ? 'FullScreen' : 'FullScreen'" @click="maximizeOrRestoreWindow" />
        <el-button link icon="Close" @click="closeWindow" />
    </div>
</template>

<script setup name="CloseBar">
import { Window } from '@tauri-apps/api/window';

const appWindow = new Window('main');

// 定义一个响应式的引用，用于跟踪窗口是否最大化
const isMaximized = ref(false);


// 窗口最小化
const minimizeWindow = () => {
    appWindow.minimize();
};

// 窗口最大化或还原
const maximizeOrRestoreWindow = () => {
    // 查询当前状态
    isMaximized.value = appWindow.isMaximized()
    appWindow.toggleMaximize()
};

// 关闭窗口
const closeWindow = () => {
    appWindow.close()
};
</script>

<style scoped>
.close-bar {
    position: fixed;
    left: 0;
    right: 0;
    top: 0;
    display: flex;
    justify-content: flex-end;
    align-items: center;
    padding-right: 1rem;
    height: 30px;
    width: 100%;
    z-index: 9999;
}
</style>