<template>
    <!-- show sell coupon -->
    <el-form ref="sellFormRef" :model="sellForm" label-width="90px">
        <el-form-item label="会员身份" prop="userId">
            {{ user.nickName }} - {{ user.phonenumber }}
        </el-form-item>
        <el-row>
            <h3 class="title">卡券信息</h3>
        </el-row>
        <el-table :data="couponList" border @selection-change="handleSelectionChange">
            <el-table-column type="selection" width="55" align="center" />
            <el-table-column label="卡券名称" align="center" key="couponTitle" prop="couponTitle" />
            <el-table-column label="有效期" align="center" key="validTo" prop="validTo" />
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
                    <el-radio v-for="dict in sys_payment_method" :key="dict.value" :label="dict.label"
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
</template>

<script setup name="CouponSale">
import { listCoupon, buyCoupon } from "@/api/system/coupon";
import { getUser } from "@/api/system/user";
import { listClothing } from "@/api/system/clothing";
import { ref, computed } from "vue";

const props = defineProps({
    userId: {
        type: String,
        required: true
    },
    submit: {
        type: Function,
        required: true
    }
});

const { proxy } = getCurrentInstance();

const {
    sys_payment_method
} =
    proxy.useDict(
        "sys_payment_method"
    );

const couponList = ref([]);
const clothList = ref([]);
const clothListloading = ref(false);
const loading = ref(true);
const user = ref({});

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
});

const { sellForm, selectedList } = toRefs(data);


/* 动态计算销售卡券时的总金额 */
const totalPrice = computed(() => {
    return selectedList.value.reduce((accumulator, curItem) => {
        return accumulator + curItem.couponValue * curItem.count;
    }, 0);
});

/** 查询卡券列表 */
function getList() {
    loading.value = true;
    getUser(props.userId).then(response => {
        user.value = response.data;
    });
    listCoupon().then(response => {
        couponList.value = response.rows;
        // total.value = response.total;
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
    proxy.$refs["sellFormRef"].validate(valid => {
        if (valid) {
            const coupons = selectedList.value.filter(item => item.count > 0).map(({ couponId, count }) => ({ couponId, count }));
            if (coupons.length === 0) {
                proxy.$modal.msgWarning("请选择购买卡券");
                return;
            }
            sellForm.value.coupons = coupons;
            console.log(sellForm.value);
            buyCoupon(sellForm.value).then(res => {
                proxy.$modal.msgSuccess("购买成功");
                props.submit(sellForm.value);
            }).catch();
        }
    });
}

/* 获取衣物列表 */
function getClothingList(name) {
    clothListloading.value = true;
    listClothing({ clothingName: name }).then(res => {
        clothList.value = res.rows;
        clothListloading.value = false;
    });
}

resetSellForm();
getList();
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