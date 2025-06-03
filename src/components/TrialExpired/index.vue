<template>
    <div v-if="showExpiredOverlay" class="trial-expired-overlay">
        <!-- 订阅购买窗口 -->
        <SubscriptionManagement v-if="showSubscriptionDialog" />
        <div v-else class="trial-expired-content">
            <div class="expired-icon">
                <el-icon :size="64">
                    <WarningFilled />
                </el-icon>
            </div>
            <h2 class="expired-title">试用期已结束</h2>
            <p class="expired-message">{{ expiredMessage }}</p>
            <el-button type="primary" class="action-button" @click="handleAction">
                {{ actionButtonText }}
            </el-button>
        </div>
    </div>

</template>

<script setup>
import { computed, ref } from 'vue';
import useUserStore from '@/store/modules/user';
import SubscriptionManagement from '@/components/SubscriptionManagement/index.vue';

const userStore = useUserStore();

// 控制蒙版显示
const showExpiredOverlay = computed(() => {
    // return true
    // 当用户没有token时不显示（正在退出登录）
    if (!userStore.token) {
        return false;
    }
    // 当试用期结束且没有激活订阅时显示
    return userStore.trial.remainingDays <= 0 &&
        !userStore.subscription.isActive;
});

// 根据用户类型显示不同的消息
const expiredMessage = computed(() => {
    if (userStore.trial.isGuest) {
        return '游客试用时间已结束，请登录账号继续使用或购买订阅。';
    } else {
        return '您的试用期已结束，请购买订阅以继续使用完整功能。';
    }
});

// 根据用户类型显示不同的按钮文本
const actionButtonText = computed(() => {
    return userStore.trial.isGuest ? '前往登录' : '购买订阅';
});

// 订阅对话框控制
const showSubscriptionDialog = ref(false);

// 处理按钮点击事件
const handleAction = () => {
    if (userStore.trial.isGuest) {
        // 游客用户跳转到登录页
        userStore.logOut().then(async () => {
            location.href = '/index';
        })
    } else {
        // 普通用户打开订阅购买窗口
        showSubscriptionDialog.value = true;
    }
};
</script>

<style scoped>
.trial-expired-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.85);
    z-index: 3;
    display: flex;
    justify-content: center;
    align-items: center;
    backdrop-filter: blur(5px);
}

.trial-expired-content {
    background-color: var(--el-bg-color);
    border-radius: 12px;
    padding: 40px;
    max-width: 500px;
    width: 90%;
    text-align: center;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
    animation: fadeIn 0.5s ease-out;
}

@keyframes fadeIn {
    from {
        opacity: 0;
        transform: translateY(-20px);
    }

    to {
        opacity: 1;
        transform: translateY(0);
    }
}

.expired-icon {
    margin-bottom: 20px;
    color: var(--el-color-warning);
}

.expired-title {
    font-size: 28px;
    font-weight: 600;
    margin-bottom: 16px;
    color: var(--el-text-color-primary);
}

.expired-message {
    font-size: 16px;
    line-height: 1.6;
    margin-bottom: 30px;
    color: var(--el-text-color-secondary);
}

.action-button {
    padding: 12px 30px;
    font-size: 16px;
    font-weight: 500;
    transition: transform 0.2s;
}

.action-button:hover {
    transform: translateY(-2px);
}
</style>