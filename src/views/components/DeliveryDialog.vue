<template>
    <el-dialog v-model="dialogVisible" width="600px" :show-close="false" :align-center="true" append-to-body
        class="modern-dialog">
        <template #header>
            <div class="dialog-header">
                <div class="dialog-title">
                    <el-icon style="color: var(--el-color-warning);">
                        <Van />
                    </el-icon>
                    <span>上门派送</span>
                </div>
                <el-button circle @click="cancel">
                    <el-icon>
                        <Close />
                    </el-icon>
                </el-button>

            </div>
        </template>

        <div class="form-container">
            <el-form ref="deliveryFormRef" class="modern-form" :model="deliveryForm" :rules="deliveryRules"
                label-width="90px">
                <div class="member-card hover-flow">
                    <div class="member-avatar">
                        <el-avatar :size="50" icon="UserFilled" />
                    </div>
                    <div class="member-details">
                        <div class="member-name">{{ user.nickName }}</div>
                        <div class="member-phone">{{ user.phonenumber }}</div>
                    </div>
                    <div class="member-points">
                        <div class="points-label">积分</div>
                        <div class="points-value">{{ user.integral }}</div>
                    </div>
                </div>
                <!-- 配送信息区域 -->
                <div class="form-section hover-flow">
                    <div class="section-header">
                        <el-icon>
                            <Location />
                        </el-icon>
                        <span>配送地址</span>
                    </div>
                    <div class="section-content">
                        <el-form-item prop="address">
                            <el-input v-model="deliveryForm.address" @input="handleAddressInput" placeholder="请输入配送地址">
                                <template #prefix>
                                    <el-icon>
                                        <Location />
                                    </el-icon>
                                </template>
                            </el-input>
                            <el-checkbox v-if="needSync" v-model="deliveryForm.needSync"
                                class="sync-checkbox">更新会员默认地址</el-checkbox>
                        </el-form-item>
                    </div>
                </div>
                <div class="form-section hover-flow">
                    <div class="section-header">
                        <el-icon>
                            <Calendar />
                        </el-icon>
                        <span>配送时间</span>
                    </div>
                    <div class="section-content">
                        <el-form-item style="width: 100%; margin-bottom: 0;" prop="dispatchTime">
                            <el-date-picker v-model="deliveryForm.dispatchTime" type="date" placeholder="选择日期"
                                style="width: 100%" />
                        </el-form-item>
                    </div>
                </div>
                <div class="form-section hover-flow">
                    <div class="section-header">
                        <el-icon>
                            <Document />
                        </el-icon>
                        <span>备注信息</span>
                    </div>
                    <div class="section-content">
                        <el-form-item style="width: 100%; margin-bottom: 0;">
                            <el-input type="textarea" v-model="deliveryForm.remark" placeholder="备注信息" :rows="3" />
                        </el-form-item>
                    </div>
                </div>

            </el-form>
        </div>

        <template #footer>
            <div class="dialog-footer">
                <el-button class="hover-flow" size="large" type="danger" @click="cancelDelivery">取消</el-button>
                <el-button class="hover-flow" size="large" type="primary" @click="submitDelivery" :loading="loading">确认派送</el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup>
import { delivery } from "@/api/system/cloths";
import { ElMessage } from 'element-plus';
import { Van } from '@element-plus/icons-vue';

const props = defineProps({
    visible: {
        type: Boolean,
        default: false
    },
    // 当前用户信息
    user: {
        type: Object,
        default: () => ({})
    },
    // 选中的衣物列表
    selectedCloths: {
        type: Array,
        default: () => []
    }
});

const emit = defineEmits(['update:visible', 'success', 'cancel']);

// 对话框可见性
const dialogVisible = ref(false);
// 表单加载状态
const loading = ref(false);
// 表单引用
const deliveryFormRef = ref(null);
// 是否需要同步地址
const needSync = ref(false);

// 表单数据
const data = reactive({
    deliveryForm: {
        address: '',
        needSync: false,
        dispatchTime: getDefaultDate(),
        remark: ''
    },
    deliveryRules: {
        address: [
            { required: true, message: '请输入配送地址', trigger: 'blur' }
        ],
        dispatchTime: [
            { required: true, message: '请选择配送时间', trigger: 'change' }
        ]
    }
});

const { deliveryForm, deliveryRules } = toRefs(data);

// 监听visible属性变化
watch(() => props.visible, (newVal) => {
    dialogVisible.value = newVal;
    if (newVal) {
        initForm();
    }
});

// 监听对话框可见性变化
watch(() => dialogVisible.value, (newVal) => {
    emit('update:visible', newVal);
});

// 初始化表单
function initForm() {
    // 初始化配送表单
    deliveryForm.value = {
        address: props.user?.address || '',
        needSync: false,
        dispatchTime: getDefaultDate(),
        remark: ''
    };
    needSync.value = false;
}

// 获取当前日期
function getDefaultDate() {
    const today = new Date();
    return today;
}

// 处理地址输入
function handleAddressInput() {
    needSync.value = true;
}

// 提交派送
function submitDelivery() {
    deliveryFormRef.value.validate(valid => {
        if (!valid) return;

        loading.value = true;

        // 过滤出状态为'02'(已洗完)的衣物
        const cloths = props.selectedCloths.filter(item => item.clothingStatus === '02');
        if (cloths.length === 0) {
            ElMessage.warning("没有选中符合条件的衣物");
            loading.value = false;
            return;
        }

        // 准备提交数据
        const submitData = {
            ...deliveryForm.value,
            clothId: cloths.map(item => item.clothId).join(','),
            orderId: [...new Set(cloths.map(item => item.orderId))].join(',')
        };

        // 调用派送API
        delivery(submitData).then(res => {
            ElMessage.success("派送操作成功");
            dialogVisible.value = false;
            emit('success');
        }).catch(err => {
            ElMessage.error(err.message || "派送操作失败");
        }).finally(() => {
            loading.value = false;
        });
    });
}

// 取消派送
function cancelDelivery() {
    dialogVisible.value = false;
    emit('cancel');
}
</script>

<style scoped>
.dialog-header {
    background-color: var(--el-color-primary-light-9);
    padding: 1rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-bottom: 1px solid var(--el-border-color-lighter);
    border-radius: .5rem;
}

:root.dark .dialog-header {
    --el-color-primary-light-9: #1d2c40;
}

.dialog-title {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 18px;
    font-weight: 600;
    color: var(--el-color-primary);
}

.member-card {
    display: flex;
    align-items: center;
    background: linear-gradient(135deg, var(--el-color-primary-light-9) 0%, var(--el-color-primary-light-8) 100%);
    /* background: linear-gradient(135deg, var(--el-fill-color-light) 0%, var(--el-fill-color-dark) 100%); */
    border-radius: 12px;
    padding: 1rem;
    box-shadow: var(--el-box-shadow-lighter);
    margin-bottom: 1rem;
}

:root.dark .member-card {
    --el-color-primary-light-9: #1d2c40;
    --el-color-primary-light-8: #2b6095;
}

.member-avatar {
    margin-right: 1rem;
}

.member-details {
    flex: 1;
}

.member-name {
    font-size: 18px;
    font-weight: 600;
    margin-bottom: 4px;
}

.member-phone {
    font-size: 14px;
    color: var(--el-text-color-secondary);
}

.member-points {
    text-align: center;
    padding: 0 1rem;
    border-left: 1px solid #e4e7ed;
}

.form-section {
    margin-bottom: 24px;
    background-color: var(--el-bg-color-overlay);
    border-radius: 8px;
    box-shadow: var(--el-box-shadow-light);
    overflow: hidden;
}

.form-section:last-child {
    margin-bottom: 0;
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

.address-container {
    display: flex;
    flex-direction: column;
}

.sync-checkbox {
    margin-top: 8px;
    color: var(--el-text-color-secondary);
}

.dialog-footer {
    display: flex;
    justify-content: center;
    gap: 16px;
}
</style>