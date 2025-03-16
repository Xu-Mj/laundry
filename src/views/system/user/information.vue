<template>
    <el-dialog v-model="props.visible" :title="props.user.nickName + '-' + props.user.phonenumber" width="66.66%"
        :align-center="true" append-to-body class="modern-user-dialog" @closed="props.toggle" :show-close="false">
        <template #header>
            <div class="dialog-header">
                <div class="user-avatar">
                    <el-avatar :size="50" icon="UserFilled" />
                </div>
                <div class="user-title">
                    <h3>{{ props.user.nickName }}</h3>
                    <p>{{ props.user.phonenumber }}</p>
                </div>
                <div class="dialog-actions">
                    <el-button circle @click="props.toggle">
                        <el-icon>
                            <Close />
                        </el-icon>
                    </el-button>
                </div>
            </div>
        </template>

        <el-tabs v-model="currentTab" @tab-click="handleTabClick" class="modern-tabs">
            <el-tab-pane label="基本信息" name="basicInfo">
                <Info :user="props.user" />
            </el-tab-pane>
            <el-tab-pane label="消费历史" name="consumptionHistory">
                <History v-if="currentTab === 'consumptionHistory'" :userId="props.user.userId" />
            </el-tab-pane>
        </el-tabs>
    </el-dialog>
</template>

<script setup>
import History from './history.vue';
import Info from './info.vue';
import { ref } from 'vue';

const props = defineProps({
    user: {
        type: Object,
        required: true,
    },
    visible: {
        type: Boolean,
        required: true,
    },
    toggle: {
        type: Function,
        required: true,
    }
});

const currentTab = ref('basicInfo');
function handleTabClick(tab) {
    currentTab.value = tab.props.name;
}
</script>
<style scoped>
.modern-user-dialog {
    height: 100%;
    margin-left: auto;
    margin-right: 0;
    margin-top: 0 !important;
    display: flex;
    flex-direction: column;
    overflow: auto;
}

.modern-user-dialog :deep(.el-dialog__header) {
    padding: 0;
    margin: 0;
}

.modern-user-dialog :deep(.el-dialog__body) {
    padding: 20px;
}

.dialog-header {
    background: linear-gradient(135deg, var(--el-color-primary-light-9) 0%, var(--el-color-primary-light-8) 100%);
    padding: 1rem;
    display: flex;
    align-items: center;
    border-radius: .5rem;
}

:root.dark .dialog-header {
    --el-color-primary-light-9: #1d2c40;
    --el-color-primary-light-8: #2b6095;
}

.user-avatar {
    margin-right: 16px;
}

.user-title {
    flex: 1;
}

.user-title h3 {
    margin: 0 0 4px 0;
    font-size: 18px;
    font-weight: 600;
    color: var(--el-text-color-primary);
}

.user-title p {
    margin: 0;
    font-size: 14px;
    color: var(--el-text-color-secondary);
}

.dialog-actions {
    display: flex;
    gap: 8px;
}

.modern-tabs {
    margin-top: 16px;
}

.modern-tabs :deep(.el-tabs__header) {
    margin-bottom: 20px;
}

.modern-tabs :deep(.el-tabs__item) {
    height: 40px;
    line-height: 40px;
    font-size: 15px;
}

/* 在全局样式文件中 */
:deep(.el-dialog.modern-user-dialog:not(.is-fullscreen)) {
    margin-top: 0 !important;
    border-radius: 8px 0 0 8px;
    overflow: hidden;
    box-shadow: -5px 0 15px rgba(0, 0, 0, 0.1);
}
</style>