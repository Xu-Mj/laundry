<template>
    <div class="theme-switch" @click="toggleTheme">
        <el-icon size="20">
            <component :is="isDark ? 'Sunny' : 'Moon'" />
        </el-icon>
    </div>
</template>

<script setup>
import { useDark, useToggle } from '@vueuse/core';

const isDark = useDark();
const toggleDark = useToggle(isDark);

const toggleTheme = async (e) => {
    if (!document.startViewTransition) {
        toggleDark();
        return;
    }

    const { clientX, clientY } = e;

    const transition = document.startViewTransition(async () => {
        // 同时处理 useDark 状态和 classList
        toggleDark();
        document.documentElement.classList.toggle('dark');
    });

    transition.ready.then(() => {
        const radius = Math.hypot(
            Math.max(clientX, innerWidth - clientX),
            Math.max(clientY, innerHeight - clientY)
        );
        const clipPath = [
            `circle(0px at ${clientX}px ${clientY}px)`,
            `circle(${radius}px at ${clientX}px ${clientY}px)`
        ];
        
        document.documentElement.animate(
            {
                clipPath: isDark.value ? clipPath.reverse() : clipPath
            },
            {
                duration: 500,
                pseudoElement: isDark.value
                    ? '::view-transition-old(root)'
                    : '::view-transition-new(root)'
            }
        );
    });
};
</script>

<style>
/* 这段样式需要是全局的，不能加 scoped */
::view-transition-old(root),
::view-transition-new(root) {
    animation: none;
    /* mix-blend-mode: normal; */
}

.dark::view-transition-old(root) {
    z-index: 9999;
}

</style>

<style scoped>
.theme-switch {
    cursor: pointer;
    border-radius: 4px;
    transition: background-color 0.3s;
    color: var(--el-color-primary);
}

.theme-switch:hover {
    background-color: var(--el-fill-color-light);
}

:deep(.el-icon) {
    transition: transform 0.3s;
}

.theme-switch:hover :deep(.el-icon) {
    transform: scale(1.2);
}
</style>
