<template>
    <!-- show sell coupon -->
    <!-- <el-dialog :title="title" v-model="open" width="780px" :show-close="true" append-to-body @closed="closeHangUpDialog" > -->
    <el-form ref="sellFormRef" :model="sellForm" :rules="rules" label-width="90px">
        <el-row v-if="props.userId && props.userId != 0">
            <el-col :span="12">
                <el-form-item label="会员信息:">
                    {{ user.nickName }} - {{ user.phonenumber }}
                </el-form-item>
            </el-col>
            <el-col :span="12">
                <el-form-item label="会员积分:">
                    {{ user.integral }}
                </el-form-item>
            </el-col>
        </el-row>

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
        <el-table :data="couponList" max-height="15rem" border @selection-change="handleSelectionChange">
            <el-table-column type="selection" width="35" align="center" />
            <el-table-column label="卡券名称" align="center" key="couponTitle" prop="couponTitle" />
            <el-table-column label="卡券类型" align="center" key="couponType" prop="couponType">
                <template #default="scope">
                    <dict-tag :options="sys_coupon_type" :value="scope.row.couponType" />
                </template>
            </el-table-column>
            <!-- <el-table-column label="有效时间-起" align="center" prop="validFrom" v-if="columns[9].visible">
                <template #default="scope">
                    <span>{{ parseTime(scope.row.validFrom, '{y}-{m}-{d}') }}</span>
                </template>
            </el-table-column>
            <el-table-column label="有效时间-止" align="center" prop="validTo" v-if="columns[10].visible">
                <template #default="scope">
                    <span>{{ parseTime(scope.row.validTo, '{y}-{m}-{d}') }}</span>
                </template>
            </el-table-column>
            <el-table-column label="限制条件" align="center" prop="usageLimit" v-if="columns[13].visible">
                <template #default="scope">
                    {{ scope.row.couponType === '003' ? '最高消费金额限制' + scope.row.usageLimit + '元' :
                        scope.row.usageLimit == 0 ? '无限制' : scope.row.usageLimit }}
                </template>
            </el-table-column> -->
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
        <el-row>
            <el-form-item>
                <el-radio-group v-model="sellForm.paymentMethod">
                    <el-radio v-for="dict in sys_coupon_payment_method" :key="dict.value" :label="dict.label"
                        :value="dict.value" />
                </el-radio-group>
            </el-form-item>
        </el-row>
        <el-row>
            <h3 class="title">备注信息</h3>
        </el-row>
        <el-row>
            <el-form-item style="width: 100%;">
                <el-input type="textarea" v-model="sellForm.remark" placeholder="备注信息" />
            </el-form-item>
        </el-row>

        <el-row>
            <h3 class="title">订单金额: {{ totalPrice }} 元</h3>
        </el-row>
        <el-row style="margin-top: 1rem; display: flex; justify-content: center; align-items: center; gap: 1rem;">
            <el-button type="primary" style="width: 6rem; height: 2rem;" @click="props.taggle()">返回</el-button>
            <el-button type="primary" style="width: 6rem; height: 2rem;" @click="buy">立即购买</el-button>
        </el-row>
    </el-form>
    <Information :user="currentUser" :visible="showInfoDialog" :key="showInfoDialog"
        :toggle="() => { showInfoDialog = !showInfoDialog }" />
    <!-- </el-dialog> -->
</template>

<script setup name="CouponSale">
import { listCoupon4sale, buyCoupon } from "@/api/system/coupon";
import { getUser, listUserWithNoLimit, addUser } from "@/api/system/user";
import Information from "@/views/system/user/information.vue";
import { ref, computed } from "vue";

const router = useRouter();
const props = defineProps({
    userId: {
        type: String,
    },
    submit: {
        type: Function,
        default: (data) => { }
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
                proxy.$modal.msgWarning("请选择购买卡券");
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
                    proxy.$modal.msgError(err);
                    return; // 当 addUser 出错时，中断执行
                }
            }

            sellForm.value.coupons = coupons;
            buyCoupon(sellForm.value).then(res => {
                proxy.$modal.msgSuccess("购买成功");
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
<!-- <style>
.el-dialog::before{
    content: "";
    width: 100%;
    height: 100%;
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    backdrop-filter: blur(40px);
    background-color: rgba(255, 255, 255, 0.2);
    /* 半透明白色 */
    z-index: -1;
    transition: background-color 0.3s ease;
}
</style> -->
<style scoped>
.title {
    border-bottom: 1px solid gray;
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