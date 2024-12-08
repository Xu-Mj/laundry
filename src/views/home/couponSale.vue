<template>
    <!-- show sell coupon -->
    <el-dialog :title="title" v-model="open" width="1080px" :show-close="false" append-to-body @closed="closeHangUpDialog" >
    <el-form ref="sellFormRef"
        :model="sellForm" :rules="rules" label-width="90px">
        <el-form-item v-if="props.userId && props.userId != 0" label="会员身份">
            {{ user.nickName }} - {{ user.phonenumber }}
        </el-form-item>
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
        <el-row>
            <h3 class="title">卡券信息</h3>
        </el-row>
        <el-table :data="couponList" max-height="15rem" border @selection-change="handleSelectionChange">
            <el-table-column type="selection" width="55" align="center" />
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
                        controls-position="right" />
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
            <el-col :span="21" class="cash">
                总价：{{ totalPrice }}
            </el-col>
            <el-col :span="3" justify="center" align="right">
                <el-button type="primary" @click="buy">立即购买</el-button>
            </el-col>
        </el-row>
        </el-form>
    </el-dialog>
</template>

<script setup name="CouponSale">
import { listCoupon4sale, buyCoupon } from "@/api/system/coupon";
import { getUser, listUserWithNoLimit, addUser } from "@/api/system/user";
import { listClothing } from "@/api/system/clothing";
import { ref, computed } from "vue";

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

const userListRes = ref([]);
const userList = ref([]);
const sellFormRef = ref();
const phoneRegex = /^1[3-9]\d{9}$/;

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
console.log(props)
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

/* 选择会员信息 */
function selectUser(userId) {
    if (!userId || userId.length == 0) {
        sellForm.value.nickName = null;
        return;
    }
    const item = userList.value.find(item => { return item.userId === userId });
    sellForm.value.nickName = item.nickName;
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
    listCoupon4sale().then(response => {
        couponList.value = response;
        couponList.value.forEach(item => item.count = 1);
        loading.value = false;
    });
}

// 多选框选中数据
function handleSelectionChange(selection) {
    selection.forEach(item => item.count = 1);
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

<style scoped>
.title {
    border-bottom: 1px solid gray;
}

.cash {
    display: flex;
    justify-content: right;
    align-items: center;
}
</style>