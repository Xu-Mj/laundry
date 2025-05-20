<template>
    <el-dialog v-model="open" width="520px" :align-center="true" :show-close="false" append-to-body
        @closed="props.taggle()" class="expenditure-dialog">
        <template #header>
            <div class="dialog-header hover-flow">
                <div class="dialog-title">{{ props.title }}</div>
                <el-button circle @click="cancel" icon="Close" />
            </div>
        </template>
        <div class="expenditure-container">
            <!-- 表单头部 - 支出类型选择 -->
            <div class="form-section hover-flow">
                <div class="section-title">
                    <el-icon>
                        <Money />
                    </el-icon>
                    <span>支出类型</span>
                </div>
                <el-form-item prop="expType" class="type-selector">
                    <el-radio-group v-model="form.expType" size="large">
                        <el-radio-button v-for="dict in sys_exp_type" :key="dict.value" :label="dict.value">
                            {{ dict.label }}
                        </el-radio-button>
                    </el-radio-group>
                </el-form-item>
            </div>

            <!-- 主表单区域 -->
            <el-form ref="expenditureRef" :model="form" :rules="rules" label-position="top" class="expenditure-form">
                <div class="form-section hover-flow">
                    <!-- 支出账目 -->
                    <el-form-item label="支出账目" prop="expTitle">
                        <el-input v-model="form.expTitle" placeholder="请输入支出账目" prefix-icon="Document" />
                    </el-form-item>

                    <!-- 对方账户 -->
                    <el-form-item label="对方账户" prop="recvAccountTitle">
                        <el-select v-model="form.recvAccount" filterable :clearable="true" remote reserve-keyword
                            placeholder="请选择或输入对方账户" allow-create @blur="handleBlur" remote-show-suffix
                            :remote-method="searchUserByTel" value-key="recvAccount" style="width: 100%"
                            prefix-icon="User">
                            <el-option v-for="item in userListRes" :key="item.userId"
                                :label="item.nickName + ' · ' + item.phonenumber" :value="item.userId" />
                            <template #prefix>
                                <el-icon>
                                    <User />
                                </el-icon>
                            </template>
                        </el-select>
                    </el-form-item>

                    <!-- 支出金额 -->
                    <el-form-item label="支出金额" prop="expAmount" class="amount-item">
                        <el-input-number v-model="form.expAmount" :min="0" :max="92233720368.54" controls-position="right"
                            placeholder="请输入支出金额" class="amount-input" :precision="2" :step="10" style="width: 100%">
                            <template #prefix>
                                <el-icon>
                                    <Money />
                                </el-icon>
                            </template>
                        </el-input-number>
                        <div class="amount-display" v-if="form.expAmount">
                            {{ form.expAmount }}
                        </div>
                        <div class="amount-hint">金额单位：元</div>
                    </el-form-item>

                    <!-- 备注信息 -->
                    <el-form-item label="备注信息" prop="remark">
                        <el-input type="textarea" v-model="form.remark" placeholder="请输入备注信息" :rows="3" resize="none" />
                    </el-form-item>
                </div>
            </el-form>
        </div>

        <!-- 底部按钮 -->
        <template #footer>
            <div class="dialog-footer">
                <el-button type="primary" @click="submitForm" :loading="submitting">
                    <el-icon v-if="!submitting">
                        <Check />
                    </el-icon>
                    <span>{{ submitting ? '提交中...' : '确 定' }}</span>
                </el-button>
                <el-button type="danger" @click="cancel" icon="Close" plain>取 消</el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup>
import { addExpenditure, updateExpenditure } from "@/api/system/expenditure";
import { listUserWithNoLimit } from "@/api/system/user";

const { proxy } = getCurrentInstance();
const { sys_exp_type } = proxy.useDict("sys_exp_type");
const props = defineProps({
    visible: {
        type: Boolean,
        required: true,
        default: false,
    },
    taggle: {
        type: Function,
        required: true,
    },
    data: {
        type: Object,
        required: false,
    },
    title: {
        type: String,
        required: true,
    }
});

const userList = ref([]);
const userListRes = ref([]);
const notACount = ref(false);
const open = ref(false);
const submitting = ref(false);

const rules = ref({
    expTitle: [
        { required: true, message: "支出账目不能为空", trigger: "blur" }
    ],
    expType: [
        { required: true, message: "支出类型不能为空", trigger: "change" }
    ],
    expAmount: [
        { required: true, message: "支出金额不能为空", trigger: "blur" }
    ],
});

// 初始化表单数据，如果是编辑模式，需要将后端返回的金额（分）转换为前端显示的金额（元）
const form = ref({
    ...props.data,
    // 如果是编辑模式且金额存在，将分转换为元
    expAmount: props.data && props.data.expAmount ? props.data.expAmount / 100 : null
})

// 格式化货币显示
const formatCurrency = (value) => {
    // 前端显示时使用原始输入值（元）
    return new Intl.NumberFormat('zh-CN', {
        style: 'currency',
        currency: 'CNY',
        minimumFractionDigits: 2
    }).format(value);
};

// 处理失去焦点的情况，保留用户输入
const handleBlur = (event) => {
    const inputValue = event.target.value;
    // 如果用户没有输入的话，不进行搜索
    if (!inputValue) return;
    if (!userListRes.value.some(item => item.userId === form.value.recvAccount)) {
        // 没有搜索结果且没有选择项时，保留输入
        form.value.recvAccount = inputValue;
        notACount.value = true;
    } else {
        notACount.value = false;
    }
};

/* 根据手机号搜索用户列表 */
function searchUserByTel(tel) {
    userListRes.value = userList.value.filter(item => item.phonenumber.includes(tel));
    if (userListRes.value.length == 0) {
        // 没找到，需要创建用户
        notACount.value = true;
    } else {
        notACount.value = false;
    }
}

// 取消按钮
function cancel() {
    open.value = false;
    reset();
}

// 表单重置
function reset() {
    form.value = {
        expId: null,
        orderId: null,
        clothIds: null,
        expTitle: null,
        recvAccount: null,
        recvAccountTitle: null,
        expType: null,
        expAmount: null,
        createTime: null,
        remark: null
    };
    proxy.resetForm("expenditureRef");
}

/** 提交按钮 */
function submitForm() {
    proxy.$refs["expenditureRef"].validate(valid => {
        if (valid) {
            submitting.value = true;
            
            // 将金额转换为整数（单位：分），因为后端使用i64类型存储
            if (form.value.expAmount !== null && form.value.expAmount !== undefined) {
                // 将元转换为分（乘以100并取整）
                form.value.expAmount = Math.round(form.value.expAmount * 100);
            }

            if (notACount.value) {
                form.value.recvAccountTitle = form.value.recvAccount;
                form.value.recvAccount = null;
            } else if (form.value.recvAccount) {
                // 查找用户，防止找不到用户导致报错
                const foundUser = userList.value.find(item => item.userId === form.value.recvAccount);
                if (foundUser) {
                    form.value.recvAccountTitle = foundUser.nickName;
                } else {
                    // 如果找不到对应的用户，将recvAccount值赋给recvAccountTitle并设置recvAccount为null
                    form.value.recvAccountTitle = form.value.recvAccount;
                    form.value.recvAccount = null;
                }
            }

            const request = form.value.expId != null ?
                updateExpenditure(form.value) :
                addExpenditure(form.value);

            request.then(response => {
                proxy.notify.success(form.value.expId != null ? "修改成功" : "新增成功");
                open.value = false;
                reset();
                props.taggle();
            }).finally(() => {
                submitting.value = false;
            });
        }
    });
}

onMounted(() => {
    if (props.visible) {
        listUserWithNoLimit().then(res => {
            userList.value = res;
            open.value = true;
        })
    }
});
</script>

<style scoped>
.expenditure-dialog :deep(.el-dialog__header) {
    margin-right: 0;
    padding: 16px 20px;
    border-bottom: 1px solid var(--el-border-color-lighter);
}

.expenditure-dialog :deep(.el-dialog__title) {
    font-size: 18px;
    font-weight: 600;
    color: var(--el-color-primary);
}

.expenditure-dialog :deep(.el-dialog__body) {
    padding: 20px;
}

.expenditure-container {
    display: flex;
    flex-direction: column;
    gap: 20px;
}

.form-section {
    background-color: var(--el-fill-color-light);
    border-radius: 8px;
    padding: 16px;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.04);
    transition: all 0.3s ease;
}

.section-title {
    display: flex;
    align-items: center;
    margin-bottom: 16px;
    font-size: 16px;
    font-weight: 500;
    color: var(--el-text-color-primary);
}

.section-title .el-icon {
    margin-right: 8px;
    color: var(--el-color-primary);
}

.type-selector {
    margin-bottom: 0;
}

.type-selector :deep(.el-radio-button__inner) {
    padding: 8px 16px;
}

.expenditure-form {
    display: flex;
    flex-direction: column;
    gap: 16px;
}

.expenditure-form :deep(.el-form-item__label) {
    padding-bottom: 8px;
    font-weight: 500;
}

.amount-item {
    position: relative;
}

.amount-display {
    position: absolute;
    right: 0;
    bottom: -22px;
    font-size: 14px;
    color: var(--el-color-success);
    font-weight: 500;
}

.amount-hint {
    font-size: 12px;
    color: var(--el-text-color-secondary);
    margin-top: 4px;
}

.dialog-footer {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    padding-top: 8px;

    button {
        transition: all 0.3s;
    }

    button:hover {
        transform: translateY(-2px);
    }
}

.dialog-footer .el-button {
    min-width: 100px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
}
</style>