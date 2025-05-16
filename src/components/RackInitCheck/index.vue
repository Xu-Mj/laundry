<template>
    <el-dialog v-model="dialogVisible" title="衣挂信息设置" width="500px" :close-on-click-modal="false"
        :close-on-press-escape="false" :show-close="false" :align-center="true">
        <template #header>
            <div class="dialog-header">
                <div>
                    <h2 class="dialog-title">衣挂信息设置</h2>
                </div>
            </div>
        </template>
        <div class="rack-init-content">
            <el-alert title="您尚未设置衣挂信息" type="warning" description="衣挂信息是系统正常运行的必要数据，否则无法收取衣物。" :closable="false"
                show-icon />
            <div class="rack-init-image">
                <el-image :src="rackSetupImage" fit="contain">
                    <template #error>
                        <div class="image-placeholder">
                            <el-icon>
                                <Picture />
                            </el-icon>
                            <span>衣挂设置示意图</span>
                        </div>
                    </template>
                </el-image>
            </div>
            <div class="rack-init-form">
                <el-form ref="rackFormRef" :model="rackForm" :rules="rackRules" label-width="100px">
                    <div v-for="(rack, index) in rackForm.racks" :key="index" class="rack-item">
                        <div class="rack-header">
                            <h4>衣挂 #{{ index + 1 }}</h4>
                            <el-button v-if="rackForm.racks.length > 1" type="danger" circle size="small"
                                @click="removeRack(index)">
                                <el-icon>
                                    <Delete />
                                </el-icon>
                            </el-button>
                        </div>
                        <el-form-item :label="'架子名称'" :prop="`racks.${index}.name`">
                            <el-input v-model="rack.name" placeholder="请输入架子名称" />
                        </el-form-item>
                        <el-form-item :label="'架子类型'" :prop="`racks.${index}.rackType`">
                            <el-radio-group v-model="rack.rackType">
                                <el-radio value="1">输送线</el-radio>
                                <el-radio value="3">鞋柜</el-radio>
                                <el-radio value="2">其他</el-radio>
                            </el-radio-group>
                        </el-form-item>
                        <el-form-item :label="'容量'" :prop="`racks.${index}.capacity`">
                            <el-input-number v-model="rack.capacity" :min="1" :max="1000" controls-position="right"
                                placeholder="请输入容量" />
                        </el-form-item>
                    </div>
                    <div class="add-rack-button">
                        <el-button type="primary" plain icon="Plus" @click="addNewRack">添加衣挂</el-button>
                    </div>
                </el-form>
            </div>
        </div>
        <template #footer>
            <div class="dialog-footer">
                <el-button type="primary" @click="submitForm">确认设置</el-button>
                <el-button @click="skipSetup">稍后设置</el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup>
import { ElMessageBox } from 'element-plus';
import { addRack } from '@/api/system/rack';
import { useRouter } from 'vue-router';
import { Picture } from '@element-plus/icons-vue';
import rackSetupImage from '@/assets/images/rack-setup.svg';

const props = defineProps({
    visible: {
        type: Boolean,
        default: false
    }
});

const emit = defineEmits(['update:visible', 'setup-complete']);

const { proxy } = getCurrentInstance();

const router = useRouter();
const dialogVisible = ref(props.visible);
const rackFormRef = ref(null);

// 创建默认衣挂对象的函数
const createDefaultRack = () => {
    return {
        name: ' ', // 添加空格作为初始值，避免立即触发验证
        rackType: '1',
        capacity: 100,
        remainingCapacity: 100,
        position: 1
    };
};

// 表单数据
const data = reactive({
    rackForm: {
        racks: [createDefaultRack()]
    },
    rackRules: {
        // 这里留空，我们将使用动态validateField而不是声明式验证规则
    }
});

// 添加新衣挂
const addNewRack = () => {
    const newRack = createDefaultRack();
    rackForm.value.racks.push(newRack);

    // 延迟聚焦并确保不触发验证
    setTimeout(() => {
        // 重置整个表单的验证
        if (rackFormRef.value) {
            rackFormRef.value.clearValidate();
        }

        const index = rackForm.value.racks.length - 1;
        const inputSelector = `.rack-item:nth-child(${index + 1}) input[placeholder="请输入架子名称"]`;
        const inputElement = document.querySelector(inputSelector);
        if (inputElement) {
            inputElement.focus();
        }
    }, 300);
};

// 删除衣挂
const removeRack = (index) => {
    rackForm.value.racks.splice(index, 1);

    // 更新剩余衣挂的验证规则
    Object.keys(rackRules.value).forEach(key => {
        if (key.startsWith('racks.')) {
            delete rackRules.value[key];
        }
    });

    // 重新添加验证规则
    rackForm.value.racks.forEach((_, idx) => {
        rackRules.value[`racks.${idx}.name`] = [
            { required: true, message: "架子名称不能为空", trigger: "blur" }
        ];
        rackRules.value[`racks.${idx}.capacity`] = [
            { required: true, message: "容量不能为空", trigger: "blur" }
        ];
        rackRules.value[`racks.${idx}.rackType`] = [
            { required: true, message: "架子类型不能为空", trigger: "blur" }
        ];
    });
};

const { rackForm, rackRules } = toRefs(data);

// 监听visible属性变化
watch(() => props.visible, (newVal) => {
    dialogVisible.value = newVal;
    if (newVal) {
        // 当对话框显示时，重置表单并聚焦第一个输入框
        setTimeout(() => {
            if (rackFormRef.value) {
                rackFormRef.value.clearValidate();

                if (rackForm.value.racks.length > 0) {
                    const inputSelector = `.rack-item:nth-child(1) input[placeholder="请输入架子名称"]`;
                    const inputElement = document.querySelector(inputSelector);
                    if (inputElement) {
                        inputElement.focus();
                    }
                }
            }
        }, 300);
    }
});

// 监听对话框可见性变化
watch(dialogVisible, (newVal) => {
    emit('update:visible', newVal);
});

// 提交表单
const submitForm = () => {
    // 手动验证所有字段
    let hasError = false;

    rackForm.value.racks.forEach((rack, index) => {
        // 检查每个必填字段
        if (!rack.name || rack.name.trim() === '') {
            proxy.notify.error(`衣挂 #${index + 1} 的架子名称不能为空`);
            hasError = true;
        }
        if (!rack.capacity) {
            proxy.notify.error(`衣挂 #${index + 1} 的容量不能为空`);
            hasError = true;
        }
        if (!rack.rackType) {
            proxy.notify.error(`衣挂 #${index + 1} 的架子类型不能为空`);
            hasError = true;
        }
    });

    if (!hasError) {
        // 设置每个衣挂的剩余容量等于总容量
        rackForm.value.racks.forEach(rack => {
            rack.remainingCapacity = rack.capacity;
        });

        // 创建提交的Promise数组
        const promises = rackForm.value.racks.map(rack => addRack(rack));

        // 并行提交所有衣挂信息
        Promise.all(promises)
            .then(responses => {
                proxy.notify.success(`成功设置 ${responses.length} 个衣挂信息`);
                dialogVisible.value = false;
                emit('setup-complete', true);

                // 询问是否前往衣挂管理页面
                ElMessageBox.confirm(
                    '衣挂信息设置成功，是否前往衣挂管理页面查看更多设置？',
                    '操作成功',
                    {
                        confirmButtonText: '前往查看',
                        cancelButtonText: '稍后再说',
                        type: 'success',
                    }
                ).then(() => {
                    router.push('/system/rack');
                }).catch(() => {
                    // 用户选择稍后再说，不做任何操作
                });
            })
            .catch(error => {
                console.error('设置衣挂信息失败:', error);
                proxy.notify.error('设置衣挂信息失败，请稍后重试');
            });
    }
};

// 跳过设置
const skipSetup = () => {
    ElMessageBox.confirm(
        '衣挂信息是系统正常运行的必要数据，确定要稍后设置吗？',
        '提示',
        {
            confirmButtonText: '确定',
            cancelButtonText: '取消',
            type: 'warning',
        }
    ).then(() => {
        dialogVisible.value = false;
        emit('setup-complete', false);
        proxy.notify.info('您可以稍后在「衣挂管理」中完成设置');
    }).catch(() => {
        // 用户取消，对话框保持打开状态
    });
};

// 聚焦第一个名称输入框（保留但不再使用）
const focusFirstNameInput = () => {
    focusNameInput(0);
};

// 聚焦指定索引的名称输入框（保留但不再使用）
const focusNameInput = (index) => {
    // 使用更具体的选择器，明确选择架子名称输入框
    const inputSelector = `.rack-item:nth-child(${index + 1}) input[placeholder="请输入架子名称"]`;
    nextTick(() => {
        setTimeout(() => {
            const inputElement = document.querySelector(inputSelector);
            if (inputElement) {
                inputElement.focus();
            }
        }, 100); // 添加小延迟确保DOM已完全渲染
    });
};
</script>

<style lang="scss" scoped>
.dialog-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    text-align: left;
    padding: 1rem;
    background: linear-gradient(135deg, var(--el-color-primary-light-9) 0%, var(--el-color-primary-light-8) 100%);
    border-radius: .5rem;
}

:root.dark .dialog-header {
    --el-color-primary-light-9: #1d2c40;
    --el-color-primary-light-8: #2b6095;
}

.dialog-title {
    margin: 0;
    font-size: 18px;
    /* color: #303133; */
    font-weight: 600;
}

.rack-init-content {
    padding: 20px 0;
}

.rack-init-image {
    margin: 20px 0;
    text-align: center;

    .image-placeholder {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 150px;
        background-color: #f5f7fa;
        border-radius: 4px;
        color: #909399;

        .el-icon {
            font-size: 48px;
            margin-bottom: 10px;
        }
    }
}

.rack-init-form {
    margin-top: 20px;
}

.dialog-footer {
    display: flex;
    justify-content: center;
    gap: 20px;
}

.rack-item {
    margin-bottom: 10px;
    padding: 15px;
    border-radius: 8px;
    background-color: var(--el-fill-color-light);
}

.rack-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;

    h4 {
        margin: 0;
        color: #409EFF;
    }
}

.add-rack-button {
    display: flex;
    justify-content: center;
    margin-top: 20px;
}
</style>