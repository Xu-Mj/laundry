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
                        <el-avatar :size="50" :src="user.avatar" icon="UserFilled" />
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
                            <el-date-picker v-model="deliveryForm.dispatchTime" type="datetime" placeholder="选择日期和时间"
                                style="width: 100%" :disabledDate="disabledDate" />
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

                <!-- Selected clothes section -->
                <div class="form-section hover-flow">
                    <div class="section-header">
                        <el-icon>
                            <Goods />
                        </el-icon>
                        <span>派送衣物 ({{ filteredCloths.length }}件)</span>
                    </div>
                    <div class="section-content clothes-list">
                        <div v-if="filteredCloths.length === 0" class="empty-clothes">
                            <el-empty description="没有可派送的衣物" />
                        </div>
                        <div v-else class="cloth-item" v-for="(cloth, index) in filteredCloths" :key="index">
                            <div class="cloth-info">
                                <div class="cloth-name">
                                    <span class="cloth-number">{{ index + 1 }}.</span>
                                    {{ cloth.clothInfo?.clothingName || '未知衣物' }}
                                </div>
                                <div class="cloth-detail">
                                    <el-tag size="small" type="success">
                                        {{ cloth.clothInfo?.clothingCategory || '未知类别' }}
                                    </el-tag>
                                    <el-tag size="small" type="primary">
                                        {{ cloth.clothInfo?.clothingStyle || '未知分类' }}
                                    </el-tag>
                                    <el-tag size="small" type="success" class="ml-1">
                                        {{ cloth.hangClothCode || '无编码' }}
                                    </el-tag>
                                </div>
                            </div>
                            <div class="cloth-price">￥{{ cloth.priceValue }}</div>
                        </div>
                    </div>
                </div>
            </el-form>
        </div>

        <template #footer>
            <div class="dialog-footer">
                <el-button class="hover-flow" size="large" @click="cancelDelivery">取消</el-button>
                <el-button class="hover-flow" size="large" type="primary" @click="submitDelivery"
                    :loading="loading">确认派送</el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup>
import { delivery, listDeliveryEligibleClothes } from "@/api/frontend/delivery";
import { ElMessage, ElMessageBox } from 'element-plus';
import { Van, Goods } from '@element-plus/icons-vue';

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
// 过滤后的衣物（只显示状态为'02'已洗完的衣物）
const filteredCloths = ref([]);

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
        loadEligibleClothes();
    }
});

// 监听对话框可见性变化
watch(() => dialogVisible.value, (newVal) => {
    emit('update:visible', newVal);
});

// 获取可派送的衣物（状态为'02'已洗完的衣物）
async function loadEligibleClothes() {
    if (!props.user.userId) return;

    try {
        // 如果有selectedCloths，先过滤它们
        if (props.selectedCloths && props.selectedCloths.length > 0) {
            filteredCloths.value = props.selectedCloths.filter(item => item.clothingStatus === '02' || item.clothingStatus === '01');
        } else {
            // 否则从服务器加载该用户的所有可派送衣物
            const res = await listDeliveryEligibleClothes(props.user.userId);
            filteredCloths.value = res || [];
        }
    } catch (error) {
        console.error('Failed to load eligible clothes:', error);
        ElMessage.error('加载可派送衣物失败');
    }
}

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
    // 设置默认时间为当前时间4小时后
    today.setHours(today.getHours() + 4);
    return today;
}

// 禁用过去的日期
function disabledDate(time) {
    return time.getTime() < Date.now() - 8.64e7; // 禁用今天之前的日期
}

// 处理地址输入
function handleAddressInput() {
    // 如果地址与用户原始地址不同，则显示同步选项
    if (deliveryForm.value.address !== props.user?.address) {
        needSync.value = true;
    } else {
        needSync.value = false;
        deliveryForm.value.needSync = false;
    }
}

// 提交派送
function submitDelivery() {
    deliveryFormRef.value.validate(async valid => {
        if (!valid) return;

        if (filteredCloths.value.length === 0) {
            ElMessage.warning("没有选中符合条件的衣物");
            return;
        }

        // 检查是否有洗护中的衣物需要确认
        const washingCloths = filteredCloths.value.filter(item => item.clothingStatus === '01');

        try {
            // 如果有洗护中的衣物，弹出确认框
            if (washingCloths.length > 0) {
                await ElMessageBox.confirm(
                    `选择衣物中包含${washingCloths.length}件正在洗护中的衣物，确认派送？`,
                    '确认派送',
                    {
                        confirmButtonText: '确认派送',
                        cancelButtonText: '取消',
                        type: 'warning'
                    }
                );
                // 用户确认继续，不做任何处理
            }

            // 没有洗护中衣物或用户已确认，继续处理
            loading.value = true;

            // 准备提交数据
            const submitData = {
                userId: props.user.userId,
                address: deliveryForm.value.address,
                dispatchTime: deliveryForm.value.dispatchTime,
                remark: deliveryForm.value.remark,
                clothId: filteredCloths.value.map(item => item.clothId).join(','),
                needSync: deliveryForm.value.needSync,
                orderId: [...new Set(filteredCloths.value.map(item => item.orderId))].join(',')
            };

            // 调用派送API
            await delivery(submitData);

            // 如果需要同步地址到用户信息
            // if (deliveryForm.value.needSync && props.user.userId) {
            //     try {
            //         await updateUserAddress(props.user.userId, deliveryForm.value.address);
            //     } catch (err) {
            //         console.error('Failed to update user address:', err);
            //     }
            // }

            ElMessage.success("派送操作成功");
            dialogVisible.value = false;
            emit('success');
        } catch (err) {
            // 如果是 MessageBox 的取消操作，err是一个 cancel 字符串
            if (err === 'cancel') {
                return; // 用户取消了确认，不执行后续操作
            }
            ElMessage.error(err.message || "派送操作失败");
        } finally {
            loading.value = false;
        }
    });
}

// 取消派送
function cancelDelivery() {
    dialogVisible.value = false;
    emit('cancel');
}

// 简单的取消，用于关闭对话框
function cancel() {
    dialogVisible.value = false;
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

.clothes-list {
    max-height: 200px;
    overflow-y: auto;
}

.cloth-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 12px;
    border-bottom: 1px solid var(--el-border-color-lighter);
}

.cloth-item:last-child {
    border-bottom: none;
}

.cloth-info {
    flex: 1;
}

.cloth-name {
    font-weight: 500;
    margin-bottom: 4px;
}

.cloth-number {
    color: var(--el-color-primary);
    font-weight: bold;
    margin-right: 4px;
}

.cloth-detail {
    display: flex;
    gap: 4px;
    font-size: 12px;
    color: var(--el-text-color-secondary);
}

.cloth-price {
    font-weight: bold;
    color: var(--el-color-danger);
}

.empty-clothes {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100px;
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

.ml-1 {
    margin-left: 4px;
}
</style>