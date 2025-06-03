<template>
    <div data-tauri-drag-region class="close-bar">
        <el-button link icon="Minus" @click="minimizeWindow" />
        <el-button v-if="showMaximizeButton" link :icon="isMaximized ? 'FullScreen' : 'CopyDocument'"
            @click="maximizeOrRestoreWindow" />
        <el-button link icon="Close" @click="closeWindow" />
    </div>
</template>

<script setup name="CloseBar">
import useUserStore from '@/store/modules/user';
import { removeToken } from '@/utils/auth';
import { getCurrentWindow } from '@tauri-apps/api/window';

const window = getCurrentWindow();
const userStore = useUserStore();

const showMaximizeButton = computed(async () => {
    try {
        // 获取当前显示器信息
        const size = await window.innerSize();
        const screenWidth = size.width;
        const screenHeight = size.height;

        return screenWidth < 1920 || screenHeight < 1080;
    } catch (error) {
        console.error('设置窗口大小时出错:', error);
    }
    return false;
});

const isMaximized = ref(window.isMaximized());
// 窗口最小化
const minimizeWindow = () => {
    window.minimize();
};

// 窗口最大化或还原
const maximizeOrRestoreWindow = async () => {
    isMaximized.value = await window.isMaximized();
    window.toggleMaximize();
};

// 关闭窗口
const closeWindow = async () => {
    try {
        await userStore.logOut();
        removeToken();
        window.close();
    } catch (error) {
        console.error('关闭窗口时出错:', error);
        window.close();
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