<template>
    <!-- show sell coupon -->
    <!-- <el-dialog :title="title" v-model="open" width="780px" :show-close="true" append-to-body @closed="closeHangUpDialog" > -->
    <el-form ref="sellFormRef" :model="sellForm" :rules="rules" label-width="90px">
        <div v-if="props.userId && props.userId != 0">
            <!-- 会员信息卡片 -->
            <div class="member-card">
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
        </div>

        <el-row v-else>
            <el-col :span="6">
                <el-form-item label="会员身份" prop="userId">
                    <el-select v-model="sellForm.userId" filterable :clearable="true" remote reserve-keyword
                        placeholder="请输入手机号码搜索" allow-create @blur="handleBlur" remote-show-suffix
                        :remote-method="searchUserByTel" @change="selectUser" value-key="userId" style="width: 240px">
                        <el-option v-for="item in userListRes" :key="item.userId"
                            :label="item.nickName + '\t' + item.phonenumber" :value="item.userId" />
                    </el-select>
                </el-form-item>
            </el-col>
            <el-col :span="6">
                <el-form-item label="会员姓名" prop="nickName">
                    <el-input v-model="sellForm.nickName" placeholder="请输入会员姓名" />
                </el-form-item>
            </el-col>
        </el-row>
        <!-- <el-form style="margin-top: 1rem;" :model="queryParams" ref="queryRef" :inline="true" label-width="68px">
            <el-form-item label="卡券名称" prop="couponTitle">
                <el-input v-model="queryParams.couponTitle" placeholder="请输入卡券名称" clearable
                    @keyup.enter="handleQuery" />
            </el-form-item>
            <el-form-item label="卡券类型" prop="couponType">
                <el-select v-model="queryParams.couponType" @change="handleQuery" placeholder="卡券类型" clearable
                    style="width: 120px">
                    <el-option v-for="dict in sys_coupon_type" :key="dict.value" :label="dict.label"
                        :value="dict.value" />
                </el-select>
            </el-form-item>
            <el-form-item>
                <el-button type="primary" icon="Search" @click="handleQuery">搜索</el-button>
                <el-button icon="Refresh" @click="resetQuery">重置</el-button>
            </el-form-item>
        </el-form> -->
        <el-row>
            <h3 class="title">在售会员卡列表:</h3>
        </el-row>
        <el-table class="info-card" :data="couponList" max-height="15rem" border
            @selection-change="handleSelectionChange">
            <el-table-column type="selection" width="35" align="center" />
            <el-table-column label="卡券名称" align="center" key="couponTitle" prop="couponTitle" />
            <el-table-column label="卡券类型" align="center" key="couponType" prop="couponType">
                <template #default="scope">
                    <dict-tag :options="sys_coupon_type" :value="scope.row.couponType" />
                </template>
            </el-table-column>
            <el-table-column label="价格" align="center" key="couponValue" prop="couponValue" />
            <el-table-column label="数量" align="center">
                <template #default="scope">
                    <el-input-number v-model="scope.row.count" :min="0" :max="(scope.row.customerSaleCount != -1 && scope.row.customerSaleTotal != -1)
                        ? Math.min(scope.row.customerSaleCount, scope.row.customerSaleTotal)
                        : (scope.row.customerSaleTotal != -1 ? scope.row.customerSaleTotal
                            : (scope.row.customerSaleCount != -1 ? scope.row.customerSaleCount : Infinity))"
                        controls-position="right" style="width: 6rem;" />
                </template>
            </el-table-column>
        </el-table>
        <el-row>
            <h3 class="title">支付方式</h3>
        </el-row>
        <el-form-item class="payment-method-section">
            <el-radio-group v-model="sellForm.paymentMethod" class="payment-method-group">
                <template v-for="dict in sys_coupon_payment_method" :key="dict.value">
                    <template v-if="dict.value == '06'">
                        <el-radio v-if="props.couponTypeList.has('000')" :value="dict.value"
                            class="payment-method-radio">
                            <div class="payment-method-card"
                                :class="{ 'selected': sellForm.paymentMethod === dict.value }">
                                <el-icon>
                                    <CreditCard />
                                </el-icon>
                                <span>{{ dict.label }}</span>
                            </div>
                        </el-radio>
                    </template>
                    <template v-else-if="dict.value == '07'">
                        <el-radio v-if="props.couponTypeList.has('002')" :value="dict.value"
                            class="payment-method-radio">
                            <div class="payment-method-card"
                                :class="{ 'selected': sellForm.paymentMethod === dict.value }">
                                <el-icon>
                                    <Ticket />
                                </el-icon>
                                <span>{{ dict.label }}</span>
                            </div>
                        </el-radio>
                    </template>
                    <el-radio v-else-if="dict.value !== '03' && dict.value !== '04'" :value="dict.value"
                        class="payment-method-radio">
                        <div class="payment-method-card" :class="{ 'selected': sellForm.paymentMethod === dict.value }">
                            <el-icon v-if="dict.value === '01'">
                                <Money />
                            </el-icon>
                            <el-icon v-else-if="dict.value === '02'">
                                <ChatDotRound />
                            </el-icon>
                            <el-icon v-else-if="dict.value === '05'">
                                <Wallet />
                            </el-icon>
                            <el-icon v-else>
                                <More />
                            </el-icon>
                            <span>{{ dict.label }}</span>
                        </div>
                    </el-radio>
                </template>
            </el-radio-group>
        </el-form-item>
        <el-row>
            <h3 class="title">备注信息</h3>
        </el-row>
        <div class="payment-method-section remark-section">
            <el-input type="textarea" v-model="sellForm.remark" placeholder="备注信息" />
        </div>
        <!-- 价格信息区域 -->
        <div class="price-summary-card">
            <div class="price-row total">
                <span class="price-label">订单金额：</span>
                <span class="total-amount">¥ {{ totalPrice }}</span>
            </div>
        </div>

        <div class="footer-btn">
            <el-button size="large" @click="props.taggle()">取消</el-button>
            <el-button type="primary" size="large" @click="buy">立即购买</el-button>
        </div>
    </el-form>
    <Information :user="currentUser" :visible="showInfoDialog" :key="showInfoDialog"
        :toggle="() => { showInfoDialog = !showInfoDialog }" />
</template>

<script setup name="CouponSale">
import { listCoupon4sale, buyCoupon } from "@/api/system/coupon";
import { getUser, listUserWithNoLimit, addUser } from "@/api/system/user";
import Information from "@/views/system/user/information.vue";
import { ref, computed } from "vue";

const props = defineProps({
    userId: {
        type: String,
    },
    submit: {
        type: Function,
        default: (data) => { }
    },
    couponTypeList: {
        type: Set,
        required: true,
    },
    visible: {
        type: Boolean,
        required: true,
        default: false,
    },
    taggle: {
        type: Function,
        required: true,
    }
});

const { proxy } = getCurrentInstance();

const {
    sys_coupon_payment_method,
    sys_coupon_type
} =
    proxy.useDict(
        "sys_coupon_payment_method",
        "sys_coupon_type"
    );

const couponList = ref([]);
const loading = ref(true);
const open = ref(false);
const user = ref({});
const needCreateUser = ref(false);
const searchUserloading = ref(false);
const showInfoDialog = ref(false);
const currentUser = ref({});

const userListRes = ref([]);
const userList = ref([]);
const sellFormRef = ref();
const phoneRegex = /^1[3-9]\d{9}$/;
const queryParams = ref({
    pageNum: 1,
    pageSize: 10,
    couponType: null,
    couponTitle: null,
});
// 列显隐信息
const columns = ref([
    { key: 0, label: `卡券名称`, visible: true },
    { key: 1, label: `卡券编码`, visible: false },
    { key: 2, label: `卡券类别`, visible: true },
    { key: 3, label: `卡券面值`, visible: true },
    { key: 4, label: `最低消费金额`, visible: true },
    { key: 5, label: `客户可见`, visible: true },
    { key: 6, label: `总量限制`, visible: true },
    { key: 7, label: `单用户数量限制`, visible: true },
    { key: 8, label: `有效期-起`, visible: true },
    { key: 9, label: `有效期-止`, visible: true },
    { key: 10, label: `自动延期`, visible: true },
    { key: 11, label: `卡券价值`, visible: true },
    { key: 12, label: `限制条件`, visible: true },
    { key: 13, label: `适用品类`, visible: true },
    { key: 14, label: `适用分类`, visible: true },
    { key: 15, label: `适用衣物`, visible: true },
    { key: 16, label: `卡券状态`, visible: true },
    { key: 17, label: `描述`, visible: true },
]);

const data = reactive({
    form: {},
    sellForm: {
        userId: props.userId,
        paymentMethod: "05"
    },
    selectedList: [],
    rules: {
        userId: [
            { required: true, message: "所属会员不能为空", trigger: "blur" },
            {
                validator: (rule, value, callback) => {
                    // 当没有匹配到任何会员时才进行手机号格式校验
                    const isNewUser = !userListRes.value.some(item => item.userId === sellForm.value.userId);
                    if (isNewUser && !phoneRegex.test(value)) {
                        callback(new Error("请输入正确的手机号"));
                    } else {
                        callback();
                    }
                },
                trigger: 'blur'
            }
        ],
        nickName: [
            { required: true, message: "所属会员姓名不能为空", trigger: "blur" }
        ],
    }
});

const { sellForm, selectedList, rules } = toRefs(data);

function closeHangUpDialog() {
    props.taggle();
}
/** 搜索按钮操作 */
function handleQuery() {
    queryParams.value.pageNum = 1;
    getList();
}

/** 重置按钮操作 */
function resetQuery() {
    proxy.resetForm("queryRef");
    queryParams.value.delFlag = null;
    handleQuery();
}

/* 选择会员信息 */
async function selectUser(userId) {
    if (!userId || userId.length == 0) {
        sellForm.value.nickName = null;
        return;
    }
    const item = userList.value.find(item => { return item.userId === userId });
    sellForm.value.nickName = item.nickName;
    currentUser.value = await getUser(userId);

}

/* 根据手机号搜索用户列表 */
function searchUserByTel(tel) {
    userListRes.value = userList.value.filter(item => item.phonenumber.includes(tel));
    if (userListRes.value.length == 0) {
        // 没找到，需要创建用户
        needCreateUser.value = true;
        sellForm.value.nickName = null;
    } else {
        needCreateUser.value = false;
    }
}

// 处理失去焦点的情况，保留用户输入
const handleBlur = (event) => {
    const inputValue = event.target.value;
    if (!userListRes.value.some(item => item.userId === sellForm.value.userId)) {
        // 没有搜索结果且没有选择项时，保留输入
        sellForm.value.userId = inputValue;
    }
    sellFormRef.value.validateField('userId');
};


/* 动态计算销售卡券时的总金额 */
const totalPrice = computed(() => {
    return selectedList.value.reduce((accumulator, curItem) => {
        return accumulator + curItem.couponValue * curItem.count;
    }, 0);
});

/** 查询卡券列表 */
function getList() {
    loading.value = true;
    if (!props.userId || props.userId == 0) {
        searchUserloading.value = true;
        listUserWithNoLimit().then(res => {
            searchUserloading.value = false;
            userList.value = res;
        });
    } else {
        getUser(props.userId).then(response => {
            user.value = response;
        });
    }
    listCoupon4sale({ title: queryParams.value.couponTitle, tp: queryParams.value.couponType }).then(response => {
        couponList.value = response;
        couponList.value.forEach(item => item.count = 1);
        loading.value = false;
    });
}

// 多选框选中数据
function handleSelectionChange(selection) {
    selection.forEach(item => {
        if (!item.count) {
            item.count = 1;
        }
    });
    selectedList.value = selection;
}

function resetSellForm() {
    sellForm.value = {
        userId: props.userId,
        coupons: null,
        remark: null,
        paymentMethod: "05"
    };
    proxy.resetForm("sellFormRef");
}

/* 购买卡券 */
function buy() {
    proxy.$refs["sellFormRef"].validate(async valid => {
        if (valid) {
            const coupons = selectedList.value.filter(item => item.count > 0).map(({ couponId, count }) => ({ couponId, count }));
            if (coupons.length === 0) {
                proxy.notify.warning("请选择购买卡券");
                return;
            }

            if (needCreateUser.value) {
                try {
                    const res = await addUser({
                        phonenumber: sellForm.value.userId,
                        nickName: sellForm.value.nickName
                    });

                    sellForm.value.userId = res.data; // 设置返回的用户ID
                } catch (err) {
                    proxy.notify.error(err);
                    return; // 当 addUser 出错时，中断执行
                }
            }

            sellForm.value.coupons = coupons;
            buyCoupon(sellForm.value).then(res => {
                proxy.notify.success("购买成功");
                props.submit(sellForm.value);
                resetSellForm();
                selectedList.value = [];
                getList();
            }).catch();
        }
    });
}
onMounted(async () => {
    if (props.visible) {
        resetSellForm();
        getList();
        open.value = true;
    }
});
</script>

<style scoped>
.title {
    font-size: 16px;
    font-weight: 600;
    margin: 16px 0 12px 0;
    color: var(--el-text-color-primary);
    padding-bottom: 8px;
    border-bottom: 1px solid #ebeef5;
}

.member-card {
    display: flex;
    align-items: center;
    background: linear-gradient(135deg, var(--el-fill-color-light) 0%, var(--el-fill-color-dark) 100%);
    border-radius: 12px;
    padding: 16px;
    margin-bottom: 24px;
    box-shadow: var(--el-box-shadow);
}

.member-avatar {
    margin-right: 16px;
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
    padding: 0 16px;
    border-left: 1px solid #e4e7ed;
}

.points-label {
    font-size: 12px;
    color: var(--el-text-color-secondary);
}

.points-value {
    font-size: 20px;
    font-weight: 600;
    color: #f56c6c;
}

.info-card {
    background-color: var(--el-fill-color);
    border-radius: 8px;
    padding: 15px;
    box-shadow: var(--el-box-shadow);
}

.section-title {
    font-size: 16px;
    font-weight: 600;
    margin: 16px 0 12px 0;
    color: var(--el-text-color-primary);
}

.payment-method-section {
    background-color: var(--el-fill-color);
    border-radius: 8px;
    padding: 15px;
    box-shadow: var(--el-box-shadow);
}

.payment-method-group {
    display: flex;
    flex-wrap: wrap;
    gap: 16px;
}

.payment-method-radio {
    margin-right: 0 !important;
    height: auto;
}

.payment-method-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 100px;
    height: 80px;
    border-radius: 8px;
    border: 1px solid var(--el-border-color);
    transition: all 0.3s;
    cursor: pointer;
    background-color: var(--el-bg-color-overlay);
}

.payment-method-card:hover {
    border-color: var(--el-color-primary);
    transform: translateY(-2px);
    box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
}

.payment-method-card.selected {
    border-color: var(--el-color-primary);
    background-color: var(--el-fill-color-light);
    box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
}

.payment-method-card .el-icon {
    font-size: 24px;
    margin-bottom: 8px;
    color: var(--el-color-primary);
}

.payment-method-card span {
    font-size: 14px;
}

.remark-section {
    margin-bottom: 20px;
}

.price-summary-card {
    background-color: var(--el-fill-color-light);
    border-radius: 8px;
    padding: 16px;
    margin: 24px 0;
    box-shadow: var(--el-box-shadow);
}

.price-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
}

.price-row.total {
    margin-bottom: 0;
}

.price-label {
    font-size: 14px;
    color: var(--el-text-color-primary);
}

.total-amount {
    font-weight: 600;
    font-size: 24px;
    color: #f56c6c;
}

.footer-btn {
    margin-top: 1rem;
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 1rem;

    button {
        transition: all 0.3s;
    }

    button:hover {
        transform: translateY(-2px);
    }
}

/* 美化表单元素 */
:deep(.el-input__inner),
:deep(.el-select),
:deep(.el-input-number) {
    border-radius: 6px;
}

:deep(.el-table) {
    border-radius: 8px;
    overflow: hidden;
    margin-bottom: 20px;
}

:deep(.el-textarea__inner) {
    border-radius: 6px;
    min-height: 80px;
}

:deep(.el-button) {
    border-radius: 6px;
}

.cash {
    display: flex;
    justify-content: right;
    align-items: center;
}

.equal-width-button {
    width: 120px;
}
</style>