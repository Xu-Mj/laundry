<template>
    <el-dialog v-model="props.visible" :title="props.user.nickName + '-' + props.user.phonenumber" width="66.66%"
        top="0" append-to-body class="modern-user-dialog" @closed="props.toggle" :show-close="false" align-center>
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

<style>
.modern-user-dialog {
    max-height: 100%;
    overflow: hidden;
}
</style>

<style scoped>
.dialog-header {
    background: linear-gradient(135deg, var(--el-color-primary-light-9) 0%, var(--el-color-primary-light-8) 100%);
    padding: 1rem;
    display: flex;
    align-items: center;
    border-radius: .5rem;
    transition: all 0.3s ease;
}

.dialog-header:hover {
    transform: translateY(-2px);
    box-shadow: var(--el-box-shadow-light);
}

:root.dark .dialog-header {
    --el-color-primary-light-9: #1d2c40;
    --el-color-primary-light-8: #2b6095;
}

.user-avatar {
    margin-right: 1rem;
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
    height: 100%;
    margin-top: 1rem;
}

.modern-tabs :deep(.el-tabs__header) {
    margin-bottom: 20px;
}

.modern-tabs :deep(.el-tabs__item) {
    height: 40px;
    line-height: 40px;
    font-size: 15px;
}
</style>