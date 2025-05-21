<template>
    <div data-tauri-drag-region class="close-bar">
        <el-button link icon="Minus" @click="minimizeWindow" />
        <el-button v-if="windowStore.showMaximizeButton" link :icon="windowStore.isMaximized ? 'FullScreen' : 'FullScreen'"
            @click="maximizeOrRestoreWindow" />
        <el-button link icon="Close" @click="closeWindow" />
    </div>
</template>

<script setup name="CloseBar">
import { useWindowStore } from '@/store/modules/window';
import useUserStore from '@/store/modules/user';
import { removeToken } from '@/utils/auth';

const windowStore = useWindowStore();
const userStore = useUserStore();

// 窗口最小化
const minimizeWindow = () => {
    windowStore.minimize();
};

// 窗口最大化或还原
const maximizeOrRestoreWindow = () => {
    windowStore.toggleMaximize();
};

// 关闭窗口
const closeWindow = async () => {
    try {
        await userStore.logOut();
        removeToken();
        windowStore.close();
    } catch (error) {
        console.error('关闭窗口时出错:', error);
        windowStore.close();
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