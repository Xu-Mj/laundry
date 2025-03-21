<template>
    <div data-tauri-drag-region class="lock-screen" @contextmenu.prevent="()=>{}" v-if="localIsLocked">
        <div class="lock-container">
            <div class="lock-box">
                <div class="avatar-box">
                    <img :src="userInfo.avatar" alt="avatar" class="avatar" />
                </div>
                <h3 class="welcome">欢迎回来，{{ userInfo.name }}</h3>
                <p class="hint">请输入密码解锁</p>
                <div v-if="isLockoutActive" class="lockout-message">
                    <el-alert title="密码输入次数过多" type="error"
                        :description="`账户已被锁定，请${Math.ceil(remainingLockoutTime / 60)}分钟后再试`" show-icon
                        :closable="false" />
                    <div class="lockout-timer">剩余锁定时间: {{ formatLockoutTime(remainingLockoutTime) }}</div>
                </div>
                <el-button style="width: 60%;" v-else-if="userInfo.isGuest" @click="localIsLocked = false" type="primary">点击解锁</el-button>
                <el-form @submit.prevent="handleUnlock" v-else>
                    <el-form-item>
                        <div class="password-container">
                            <el-input v-model="password" type="password" placeholder="请输入密码" autofocus
                                @keyup.enter="isPasswordValid && handleUnlock" size="large" ref="passwordInput"
                                :class="['password-input', { 'shake': isShaking }]" :disabled="isLockoutActive">
                                <template #prefix><svg-icon icon-class="password" class="input-icon" /></template>
                            </el-input>
                            <div class="arrow-button" @click="isPasswordValid && handleUnlock"
                                :class="{ 'disabled': isLockoutActive || loading || !isPasswordValid }">
                                <svg-icon icon-class="arrow-right" class="arrow-icon" />
                            </div>
                        </div>
                        <div v-if="failedAttempts > 0" class="attempts-warning">
                            密码错误，还有{{ maxFailedAttempts - failedAttempts }}次机会
                        </div>
                    </el-form-item>
                </el-form>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, computed, onMounted, onBeforeUnmount, watch, nextTick } from 'vue';
import useUserStore from '@/store/modules/user';
import { validatePwd } from '@/api/login';
import { getConfigKey } from '@/api/system/config';

const props = defineProps({
    isLocked: {
        type: Boolean,
        default: false
    }
});

const emit = defineEmits(['unlock']);

const userStore = useUserStore();
const password = ref('');
const loading = ref(false);
const localIsLocked = ref(props.isLocked);

// 密码错误尝试相关变量
const failedAttempts = ref(0);
const maxFailedAttempts = 5; // 最大尝试次数
const lockoutDuration = 30 * 60; // 锁定时间（秒）
const isLockoutActive = ref(false);
const remainingLockoutTime = ref(0);
const isShaking = ref(false);
let lockoutTimerId = null;

// 锁屏计时器配置
const defaultTimeoutLength = 300; // 默认5分钟
const timeOut = ref(defaultTimeoutLength);
let warningTimeoutId;
let lockTimeoutId;

// 监听props.isLocked的变化
watch(() => props.isLocked, (newVal) => {
    localIsLocked.value = newVal;
    if (newVal) {
        nextTick(() => {
            passwordInput.value?.focus();
        });
    }
});

// 监听localIsLocked的变化
watch(() => localIsLocked.value, (newVal) => {
    if (newVal) {
        nextTick(() => {
            passwordInput.value?.focus();
        });
    }
});

// 添加密码输入框的ref
const passwordInput = ref(null);
const userInfo = computed(() => {
    return {
        name: userStore.name,
        avatar: userStore.avatar,
        account: userStore.account,
        isGuest: userStore.isGuest,
    };
});

const resetTimeout = (timeoutLength = defaultTimeoutLength) => {
    if (warningTimeoutId) {
        clearTimeout(warningTimeoutId);
    }
    if (lockTimeoutId) {
        clearTimeout(lockTimeoutId);
    }

    // 如果已经锁定，不重置计时器
    if (localIsLocked.value) return;

    warningTimeoutId = setTimeout(() => {
        // 锁定屏幕
        localIsLocked.value = true;
    }, timeoutLength * 1000);
};

// 格式化锁定时间显示
const formatLockoutTime = (seconds) => {
    const minutes = Math.floor(seconds / 60);
    const remainingSeconds = seconds % 60;
    return `${minutes}:${remainingSeconds < 10 ? '0' : ''}${remainingSeconds}`;
};

// 开始锁定计时
const startLockout = () => {
    isLockoutActive.value = true;
    remainingLockoutTime.value = lockoutDuration;

    if (lockoutTimerId) {
        clearInterval(lockoutTimerId);
    }

    lockoutTimerId = setInterval(() => {
        remainingLockoutTime.value--;
        if (remainingLockoutTime.value <= 0) {
            clearInterval(lockoutTimerId);
            isLockoutActive.value = false;
            failedAttempts.value = 0;
        }
    }, 1000);
};

// 触发输入框抖动动画
const triggerShake = () => {
    isShaking.value = true;
    setTimeout(() => {
        isShaking.value = false;
    }, 500); // 动画持续时间
};

// 添加密码有效性检查的计算属性
const isPasswordValid = computed(() => {
    return password.value.length >= 6;
});

const handleUnlock = async () => {
    // 如果密码长度小于6，不执行解锁操作
    if (!isPasswordValid.value) {
        return;
    }

    // 如果处于锁定状态，不允许尝试
    if (isLockoutActive.value) {
        return;
    }

    loading.value = true;
    try {
        // 使用登录API验证密码
        await validatePwd(userInfo.value.account, password.value);
        localIsLocked.value = false;
        emit('unlock');
        password.value = '';
        failedAttempts.value = 0; // 重置错误计数
        resetTimeout(timeOut.value);
    } catch (error) {
        failedAttempts.value++;
        triggerShake(); // 触发抖动动画

        // 检查是否达到最大尝试次数
        if (failedAttempts.value >= maxFailedAttempts) {
            startLockout();
        }
    } finally {
        password.value = '';
        loading.value = false;
    }
};

const debounce = (func, wait) => {
    let timeout;
    return (...args) => {
        clearTimeout(timeout);
        timeout = setTimeout(() => func.apply(this, args), wait);
    };
};

const handleUserInteraction = debounce(() => {
    resetTimeout(timeOut.value);
}, 300); // Debounce with 300ms delay

const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
const modifierKey = isMac ? 'metaKey' : 'ctrlKey';

const handleKeyDown = (event) => {
  // 检查组合键 + KeyL
  if (event[modifierKey] && event.code === 'KeyL') {
    // 尝试阻止默认行为
    try {
      event.preventDefault();
      event.stopImmediatePropagation();
    } catch (e) {
      console.warn('无法阻止浏览器默认快捷键行为');
    }

    // 验证锁定条件
    if (!localIsLocked.value && !isLockoutActive.value) {
      localIsLocked.value = true;
      password.value = '';
      resetTimeout(timeOut.value);
      nextTick(() => passwordInput.value?.focus());
    }
  }
};

onMounted(async () => {
    // 从服务器获取超时时间
    try {
        const config = await getConfigKey('logout_timeout');
        timeOut.value = config ? Number(config.configValue) : defaultTimeoutLength;
        // timeOut.value = 5;
    } catch (error) {
        console.error('获取超时配置失败:', error);
    }

      resetTimeout(timeOut.value);
    window.addEventListener('mousemove', handleUserInteraction);
    window.addEventListener('keydown', handleUserInteraction);
    window.addEventListener('keydown', handleKeyDown);
});

onBeforeUnmount(() => {
    if (warningTimeoutId) {
        clearTimeout(warningTimeoutId);
    }
    if (lockTimeoutId) {
        clearTimeout(lockTimeoutId);
    }
    if (lockoutTimerId) {
        clearInterval(lockoutTimerId);
    }
    window.removeEventListener('mousemove', handleUserInteraction);
    window.removeEventListener('keydown', handleUserInteraction);
    window.removeEventListener('keydown', handleKeyDown);
});
</script>

<style lang="scss" scoped>
.lock-screen {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(18px);
    z-index: 9999;
    display: flex;
    justify-content: center;
    align-items: center;
    pointer-events: auto;
    animation: fadeIn 0.3s ease-in-out;
}

.lock-container {
    width: 100%;
    max-width: 400px;
    padding: 20px;
    position: relative;
}

.lock-box {
    border-radius: 8px;
    padding: 30px;
    text-align: center;
}

.avatar-box {
    margin-bottom: 20px;
}

.avatar {
    width: 80px;
    height: 80px;
    border-radius: 50%;
    border: 3px solid var(--el-color-primary);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.welcome {
    font-size: 1.5rem;
    margin-bottom: 5px;
    color: var(--el-text-color-primary);
}

.hint {
    color: var(--el-text-color-secondary);
    margin-bottom: 20px;
}

.password-container {
    width: 100%;
    position: relative;
    display: flex;
    align-items: center;
    margin-bottom: 10px;
}

.password-input {
    flex: 1;
    background-color: rgba(0, 0, 0, 0.5) !important;
    border-radius: 4px;
}

:deep(.el-input__wrapper) {
    background-color: transparent;
    box-shadow: none !important;
    border-radius: 4px;
}

:deep(.el-input__inner) {
    color: #fff;
}

:deep(.el-input__inner::placeholder) {
    color: rgba(255, 255, 255, 0.6);
}

.arrow-button {
    position: absolute;
    right: 10px;
    top: 50%;
    transform: translateY(-50%);
    cursor: pointer;
    z-index: 10;
    display: flex;
    align-items: center;
    justify-content: center;
}

.arrow-icon {
    font-size: 20px;
    color: #fff;
}

.arrow-button.disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.input-icon {
    color: rgba(255, 255, 255, 0.6);
}

.attempts-warning {
    width: 100%;
    color: var(--el-color-danger);
    font-size: 0.9rem;
    margin-bottom: 15px;
    text-align: center;
    position: absolute;
    bottom: -100%;
    left: 0;
    right: 0;
}

.lockout-message {
    margin-bottom: 20px;
}

.lockout-timer {
    margin-top: 10px;
    font-weight: bold;
    color: var(--el-color-danger);
}

/* 抖动动画 */
.shake {
    animation: shake 0.5s cubic-bezier(0.36, 0.07, 0.19, 0.97) both;
    transform: translate3d(0, 0, 0);
}

@keyframes shake {

    10%,
    90% {
        transform: translate3d(-1px, 0, 0);
    }

    20%,
    80% {
        transform: translate3d(2px, 0, 0);
    }

    30%,
    50%,
    70% {
        transform: translate3d(-4px, 0, 0);
    }

    40%,
    60% {
        transform: translate3d(4px, 0, 0);
    }
}

@keyframes fadeIn {
    from {
        opacity: 0;
    }

    to {
        opacity: 1;
    }
}
</style>