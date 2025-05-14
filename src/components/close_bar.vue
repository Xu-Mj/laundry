<template>
    <div data-tauri-drag-region class="close-bar">
        <el-button link icon="Minus" @click="minimizeWindow" />
        <el-button link :icon="isMaximized ? 'FullScreen' : 'FullScreen'" @click="maximizeOrRestoreWindow" />
        <el-button link icon="Close" @click="closeWindow" />
    </div>
</template>

<script setup name="CloseBar">
import { Window } from '@tauri-apps/api/window';
import useUserStore from '@/store/modules/user';
import { removeToken } from '@/utils/auth';

const appWindow = new Window('main');
const userStore = useUserStore();

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
const closeWindow = async () => {
    try {
        // 执行登出操作以清除当前用户的登录状态
        await userStore.logOut();
        
        // 移除Token，确保下次启动时需要重新登录
        removeToken();
        
        // 关闭窗口
        appWindow.close();
    } catch (error) {
        console.error('关闭窗口时出错:', error);
        // 即使登出失败，也尝试关闭窗口
        appWindow.close();
    }
};
</script>

<style scoped>
.close-bar {
    position: fixed;
    right: 0;
    top: 0;
    display: flex;
    justify-content: flex-end;
    align-items: center;
    padding-right: 1rem;
    height: 30px;
    width: calc(100% - var(--sidebar-width));
    z-index: 9999;
}
</style>