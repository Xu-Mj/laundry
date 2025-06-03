<template>
    <el-dialog v-model="dialogVisible" :width="width" align-center :show-close="false" append-to-body
        class="ref-count-editor">
        <template #header>
            <div class="dialog-header">
                <div class="dialog-title">
                    <el-icon class="header-icon">
                        <EditPen />
                    </el-icon>
                    <span>{{ title || '修改使用次数' }}</span>
                </div>
                <el-button icon="Close" circle @click="cancel" />
            </div>
        </template>

        <div class="dialog-content">
            <el-form ref="formRef" :model="form" :rules="rules" label-width="80px">
                <el-form-item label="使用次数" prop="refNumber">
                    <el-input-number v-model="form.refNumber" :min="0" controls-position="right" style="width: 100%"
                        placeholder="请输入使用次数">
                        <template #prefix>
                            <el-icon>
                                <Odometer />
                            </el-icon>
                        </template>
                    </el-input-number>
                </el-form-item>

                <div class="count-tip" v-if="description">
                    <el-alert :title="description" type="info" :closable="false" show-icon class="custom-alert" />
                </div>
            </el-form>

            <div class="value-display" v-if="form.refNumber !== null && form.refNumber !== undefined">
                <div class="value-circle">
                    <span class="value-number">{{ form.refNumber }}</span>
                </div>
                <div class="value-label">当前设置值</div>
            </div>
        </div>

        <template #footer>
            <div class="dialog-footer">
                <el-button class="hover-flow" type="primary" @click="submitForm" :loading="loading" icon="Check"> 确
                    定</el-button>
                <el-button class="hover-flow" type="danger" @click="cancel" icon="Close">取 消</el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup>

const props = defineProps({
    // 对话框标题
    title: {
        type: String,
        default: '修改使用次数'
    },
    // 对话框宽度
    width: {
        type: String,
        default: '450px'
    },
    // 描述信息
    description: {
        type: String,
        default: ''
    },
    // 初始值
    initialValue: {
        type: Number,
        default: 0
    },
    // 是否显示对话框
    modelValue: {
        type: Boolean,
        default: false
    }
});

const emit = defineEmits(['update:modelValue', 'confirm', 'cancel']);

// 对话框可见性
const dialogVisible = ref(false);

// 监听modelValue变化
watch(() => props.modelValue, (val) => {
    dialogVisible.value = val;
    if (val) {
        // 打开对话框时重置表单
        resetForm();
    }
});

// 监听dialogVisible变化
watch(() => dialogVisible.value, (val) => {
    emit('update:modelValue', val);
});

// 表单引用
const formRef = ref(null);

// 加载状态
const loading = ref(false);

// 表单数据
const form = reactive({
    refNumber: 0
});

// 表单校验规则
const rules = {
    refNumber: [
        { required: true, message: "使用次数不能为空", trigger: "blur" }
    ]
};

// 重置表单
function resetForm() {
    form.refNumber = props.initialValue;
    if (formRef.value) {
        formRef.value.resetFields();
    }
}

// 提交表单
function submitForm() {
    if (!formRef.value) return;

    formRef.value.validate(valid => {
        if (valid) {
            loading.value = true;
            // 触发确认事件，将表单数据传递给父组件
            emit('confirm', form.refNumber);
            // 父组件应该处理关闭对话框
        } else {
            return false;
        }
    });
}

// 取消操作
function cancel() {
    emit('cancel');
    dialogVisible.value = false;
}
</script>

<style scoped>
.ref-count-editor :deep(.el-input-number .el-input__wrapper) {
    width: 100%;
}

.dialog-content {
    padding: 10px 0;
    display: flex;
    flex-direction: column;
    gap: 20px;
}

.count-tip {
    margin-top: 15px;
}

.custom-alert {
    border-radius: 8px;
    border-left: 3px solid var(--el-color-primary);
}

.value-display {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin: 10px 0;
}

.value-circle {
    width: 80px;
    height: 80px;
    border-radius: 50%;
    background-color: var(--el-color-primary-light-8);
    display: flex;
    justify-content: center;
    align-items: center;
    margin-bottom: 10px;
    border: 2px solid var(--el-color-primary);
}

.value-number {
    font-size: 24px;
    font-weight: bold;
    color: var(--el-color-primary);
}

.value-label {
    font-size: 14px;
    color: var(--el-text-color-secondary);
}
</style>