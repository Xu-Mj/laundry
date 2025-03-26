<template>
    <el-dialog :align-center="true" v-model="dialogVisible" title="修改密码" :show-close="false" width="400px"
        class="modern-dialog">
        <template #header>
            <div class="dialog-header">
                <div class="dialog-title">
                    <el-icon style="color: var(--el-color-primary);">
                        <Lock />
                    </el-icon>
                    <span>修改密码</span>
                </div>
                <el-button circle @click="cancel">
                    <el-icon>
                        <Close />
                    </el-icon>
                </el-button>
            </div>
        </template>

        <div class="form-container">

            <el-form ref="formRef" :model="form" :rules="rules" label-width="80px">
                <div class="form-section hover-flow">
                    <div class="section-header">
                        <el-icon>
                            <Key />
                        </el-icon>
                        <span>修改内容</span>
                    </div>
                    <div class="section-content">
                        <el-form-item label="旧密码" prop="oldPassword">
                            <el-input v-model="form.oldPassword" type="password" placeholder="请输入旧密码"
                                :prefix-icon="Key" />
                        </el-form-item>
                        <el-form-item label="新密码" prop="newPassword">
                            <el-input v-model="form.newPassword" type="password" placeholder="请输入新密码"
                                :prefix-icon="Edit" />
                        </el-form-item>
                        <el-form-item label="确认密码" prop="confirmPassword">
                            <el-input v-model="form.confirmPassword" type="password" placeholder="请再次输入新密码"
                                :prefix-icon="Check" />
                        </el-form-item>
                    </div>
                </div>
            </el-form>
        </div>

        <template #footer>
            <div class="dialog-footer">
                <el-button class="hover-flow" type="danger" @click="cancel">取消</el-button>
                <el-button class="hover-flow" type="primary" @click="submitPwd" :loading="loading">提交</el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup>
import { ref, reactive, defineProps, defineEmits, watch } from 'vue';
import { Lock, Key, Edit, Check, Close } from '@element-plus/icons-vue';
import { updatePwd } from '@/api/system/user';

const props = defineProps({
    visible: {
        type: Boolean,
        default: false
    },
    account: {
        type: String,
        default: ''
    }
});

const emit = defineEmits(['update:visible', 'success', 'cancel']);

// 对话框可见性
const dialogVisible = ref(false);
// 表单加载状态
const loading = ref(false);
// 表单引用
const formRef = ref(null);

// 表单数据
const form = ref({
    oldPassword: '',
    newPassword: '',
    confirmPassword: ''
});

// 重置表单
function reset() {
    form.value = {
        oldPassword: '',
        newPassword: '',
        confirmPassword: ''
    };
    formRef.value?.resetFields();
}

// 验证规则
const validateOldPass = (rule, value, callback) => {
    if (value === '') {
        callback(new Error('请输入旧密码'));
    } else if (value === form.value.newPassword) {
        callback(new Error('新密码不能与旧密码相同'));
    } else {
        callback();
    }
};

const validateNewPass = (rule, value, callback) => {
    if (value === '') {
        callback(new Error('请输入新密码'));
    } else if (value.length < 6 || value.length > 20) {
        callback(new Error('长度在 6 到 20 个字符'));
    } else {
        callback();
    }
};

const validateConfirmPass = (rule, value, callback) => {
    if (value === '') {
        callback(new Error('请再次输入密码'));
    } else if (value !== form.value.newPassword) {
        callback(new Error('两次输入密码不一致!'));
    } else {
        callback();
    }
};

const rules = reactive({
    oldPassword: [
        { validator: validateOldPass, trigger: 'blur' },
    ],
    newPassword: [
        { validator: validateNewPass, trigger: 'blur' },
    ],
    confirmPassword: [
        { validator: validateConfirmPass, trigger: 'blur' },
    ]
});

// 监听visible属性变化
watch(() => props.visible, (newVal) => {
    dialogVisible.value = newVal;
    if (newVal) {
        reset();
    }
});

// 监听对话框可见性变化
watch(() => dialogVisible.value, (newVal) => {
    emit('update:visible', newVal);
});

// 提交密码修改
function submitPwd() {
    formRef.value.validate(valid => {
        if (valid) {
            loading.value = true;
            const submitForm = { ...form.value, account: props.account };

            updatePwd(submitForm).then(() => {
                emit('success');
                cancel();
            }).catch(err => {
                console.error(err);
            }).finally(() => {
                loading.value = false;
            });
        }
    });
}

// 取消操作
function cancel() {
    reset();
    dialogVisible.value = false;
    emit('cancel');
}
</script>

<style lang="scss" scoped>
.modern-dialog {
    :deep(.el-dialog__header) {
        padding: 0;
        margin: 0;
    }

    :deep(.el-dialog__body) {
        padding: 20px 20px 10px;
    }

    :deep(.el-dialog__footer) {
        padding: 10px 20px 20px;
    }
}

.dialog-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background-color: var(--el-color-primary-light-9);
    border-bottom: 1px solid var(--el-border-color-lighter);
    border-radius: 8px 8px 0 0;
}

.dialog-title {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 16px;
    font-weight: 600;
    color: var(--el-color-primary);
}

.form-container {
    padding: 10px 0;
}

.form-section {
    background-color: var(--el-bg-color-overlay);
    border-radius: 8px;
    box-shadow: var(--el-box-shadow-light);
    overflow: hidden;
}

.section-header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 16px;
    background-color: var(--el-fill-color-light);
    border-bottom: 1px solid var(--el-border-color-lighter);
    font-weight: 600;
    color: var(--el-text-color-primary);
}

.section-content {
    padding: 16px;
}

.dialog-footer {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
}
</style>