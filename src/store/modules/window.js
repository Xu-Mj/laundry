import { defineStore } from 'pinia';
import { Window } from '@tauri-apps/api/window';
import { PhysicalSize } from '@tauri-apps/api/window';

export const useWindowStore = defineStore('window', {
    state: () => ({
        isMaximized: false,
        showMaximizeButton: true,
    }),
    actions: {
        async initWindow() {
            const appWindow = new Window('main');
            try {
                const { width, height } = await appWindow.innerSize();
                const scaleFactor = await appWindow.scaleFactor();
                
                const screenWidth = width * scaleFactor;
                const screenHeight = height * scaleFactor;
                
                // 如果屏幕尺寸小于1920*1080，自动全屏并隐藏最大化按钮
                if (screenWidth < 1920 || screenHeight < 1080) {
                    await appWindow.maximize();
                    this.showMaximizeButton = false;
                } else {
                    // 否则设置窗口大小为1920*1080
                    await appWindow.setSize(new PhysicalSize(1920, 1080));
                    this.showMaximizeButton = true;
                }
            } catch (error) {
                console.error('设置窗口大小时出错:', error);
            }
        },
        async toggleMaximize() {
            const appWindow = new Window('main');
            this.isMaximized = await appWindow.isMaximized();
            await appWindow.toggleMaximize();
        },
        async minimize() {
            const appWindow = new Window('main');
            await appWindow.minimize();
        },
        async close() {
            const appWindow = new Window('main');
            await appWindow.close();
        }
    }
}); 