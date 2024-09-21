<template>
    <el-dialog v-model="showOrderDialog" width="1280px" append-to-body @closed="taggle">
        <el-form :model="queryParams" ref="queryRef" :inline="true" label-width="68px">
            <el-form-item label="取件码" prop="pickupCode">
                <el-input v-model="queryParams.pickupCode" placeholder="请输入取件码" clearable @keyup.enter="handleQuery" />
            </el-form-item>
            <el-form-item label="手机号" prop="phonenumber">
                <el-input v-model="queryParams.phonenumber" placeholder="请输入会员手机号" clearable
                    @keyup.enter="handleQuery" />
            </el-form-item>
            <el-form-item label="订单编码" prop="orderNumber">
                <el-input v-model="queryParams.orderNumber" placeholder="请输入订单编码" clearable
                    @keyup.enter="handleQuery" />
            </el-form-item>
            <el-form-item>
                <el-button type="primary" icon="Search" @click="handleQuery">搜索</el-button>
                <el-button icon="Refresh" @click="resetQuery">重置</el-button>
            </el-form-item>
        </el-form>
        <el-table ref="orderTableRef" :data="ordersList" :show-header="false" style="width: 100%" row-key="orderId"
            @row-click="handleRowClick" :row-style="{ border: '1px dashed' }" @expand-change="handleExpandChange"
            :row-class-name="rowClassName" v-model:expanded-row-keys="expandedRows"
            @select="handleOrderSelectionChange">

            <!-- 父行选择框 -->
            <el-table-column type="selection" width="55" align="center" />

            <!-- 可展开行 -->
            <el-table-column type="expand">
                <template #default="props">
                    <el-table class="cloths-table" :data="props.row.clothList" :loading="props.row.loading"
                        @selection-change="selectedItems => handleClothSelectionChange(selectedItems, props.row)"
                        ref="clothsTableRef" border="dash">
                        <el-table-column type="selection" width="55" align="center" />
                        <el-table-column label="衣物" align="center">
                            <template #default="scope">
                                {{ scope.row.clothInfo.clothingName }}
                                {{ scope.row.clothingColor ? '-' + colorList.find(item => item.tagId ==
                                    scope.row.clothingColor).tagName : '' }}
                            </template>
                        </el-table-column>
                        <el-table-column label="服务类型" :width="120" align="center">
                            <template #default="scope">
                                <span class="service-type">
                                    <dict-tag :options="sys_service_type" :value="scope.row.serviceType" />
                                    -
                                    <dict-tag :options="sys_service_requirement"
                                        :value="scope.row.serviceRequirement" />
                                </span>
                            </template>
                        </el-table-column>
                        <el-table-column label="洗护价格" align="center" prop="priceValue" />
                        <el-table-column label="工艺加价" align="center" prop="processMarkup" />
                        <el-table-column label="衣物瑕疵" align="center" prop="clothingFlaw">
                            <template #default="scope">
                                <el-tag v-for="tagId in scope.row.clothingFlaw ? scope.row.clothingFlaw.split(',') : []"
                                    :key="item" type="danger">
                                    {{ flawList.find(item => item.tagId == tagId).tagName }}
                                </el-tag>
                            </template>
                        </el-table-column>
                        <el-table-column label="洗后预估" align="center" prop="estimate">
                            <template #default="scope">
                                <el-tag v-for="tagId in scope.row.estimate ? scope.row.estimate.split(',') : []"
                                    :key="item" type="primary">
                                    {{ estimateList.find(item => item.tagId == tagId).tagName }}
                                </el-tag>
                            </template>
                        </el-table-column>
                        <el-table-column label="衣物品牌" align="center" prop="clothingBrand">
                            <template #default="scope">
                                <el-tag v-if="scope.row.clothingBrand" type="primary">
                                    {{ brandList.find(item => item.tagId == scope.row.clothingBrand).tagName }}
                                </el-tag>
                            </template>
                        </el-table-column>
                        <el-table-column label="图片" align="center" :width="95" class-name="small-padding fixed-width">
                            <template #default="scope">
                                <el-button link type="primary" @click="handleShowPicture(scope.row, true)"
                                    v-hasPermi="['system:cloths:edit']">洗前</el-button>
                                <el-button link type="primary" @click="handleShowPicture(scope.row, false)"
                                    v-hasPermi="['system:cloths:edit']">洗后</el-button>
                            </template>
                        </el-table-column>
                        <el-table-column label="状态" align="center" prop="clothingStatus">
                            <template #default="scope">
                                <dict-tag :options="sys_clothing_status" :value="scope.row.clothingStatus" />
                            </template>
                        </el-table-column>
                        <el-table-column label="取回方式" align="center" prop="pickupMethod">
                            <template #default="scope">
                                <dict-tag :options="sys_delivery_mode" :value="scope.row.pickupMethod" />
                            </template>
                        </el-table-column>
                        <el-table-column label="取回时间" align="center" prop="pickupTime" width="180">
                            <template #default="scope">
                                <span>{{ parseTime(scope.row.pickupTime, '{y}-{m}-{d}') }}</span>
                            </template>
                        </el-table-column>
                        <el-table-column label="上挂位置" align="center">
                            <template #default="scope">
                                {{
                                    scope.row.hangLocationCode ?
                                        scope.row.hangerName + '-' + scope.row.hangerNumber : ''
                                }}
                            </template>
                        </el-table-column>
                    </el-table>
                </template>
            </el-table-column>
            <el-table-column label="订单编码" width="240" prop="orderNumber">
                <template #default="scope">
                    订单编码: {{ scope.row.orderNumber }}
                </template>
            </el-table-column>
            <el-table-column label="所属会员" width="150">
                <template #default="scope">
                    <el-tooltip :content="scope.row.phonenumber">
                        所属会员: {{ scope.row.nickName }}
                    </el-tooltip>
                </template>
            </el-table-column>
            <el-table-column label="实际支付金额" width="140">
                <template #default="scope">
                    实际支付金额: {{ scope.row.payment ? scope.row.payment.paymentAmount : '-' }}
                </template>
            </el-table-column>
            <el-table-column label="取件码" prop="pickupCode" width="140">
                <template #default="scope">
                    取件码: {{ scope.row.pickupCode }}
                </template>
            </el-table-column>
            <el-table-column label="支付状态" prop="paymentStatus">
                <template #default="scope">
                    <div class="payment-status">
                        支付状态: <dict-tag :options="sys_payment_status" :value="scope.row.paymentStatus" />
                    </div>
                </template>
            </el-table-column>
        </el-table>
        <pagination v-show="total > 10" :total="total" v-model:page="queryParams.pageNum"
            v-model:limit="queryParams.pageSize" @pagination="getList" />
        <!--footer包含 四个button -->
        <template #footer>
            <el-button type="primary" @click="pickup">取衣</el-button>
            <el-button @click="handlePay">取衣收款</el-button>
            <el-button @click="handleDelivery">上门派送</el-button>
            <el-button @click="">补打小票</el-button>
        </template>
    </el-dialog>

    <!-- 展示照片 -->
    <el-dialog title="照片" v-model="showPicture" width="400px" append-to-body>
        <div class="img-container">
            <el-image class="img-item" :preview-src-list="pictureList" :src="item" v-for="(item, index) in pictureList"
                :key="index" fit="contain" />
        </div>
    </el-dialog>

    <!-- 派送对话框 -->
    <el-dialog title="派送" v-model="showDeliveryDialog" width="500px" append-to-body>
        <el-form ref="pickupRef" :model="deliveryForm" :rules="pickupRules" label-width="80px">
            <!-- 配送地址/配送时间/备注信息 -->
            <el-form-item label="配送地址" prop="deliveryAddress">
                <el-input v-model="deliveryForm.deliveryAddress" placeholder="请输入配送地址" />
            </el-form-item>
            <el-form-item label="配送时间" prop="deliveryTime">
                <el-date-picker v-model="deliveryForm.deliveryTime" type="date" placeholder="选择日期" />
            </el-form-item>
            <el-form-item label="备注信息" prop="remark">
                <el-input v-model="deliveryForm.remark" placeholder="请输入备注信息" />
            </el-form-item>
        </el-form>
        <!-- 取消确认 -->
        <template #footer>
            <div class="pickup-footer">
                <el-button type="primary" @click="submitDelivery">确认取走</el-button>
                <el-button type="primary" @click="cancelPickup">取消</el-button>
            </div>
        </template>
    </el-dialog>


    <!-- 付款弹窗 -->
    <el-dialog title="付款" v-model="showPaymentDialog" width="600px" append-to-body lock-scroll modal
        :close-on-click-modal="false">
        <el-form ref="paymentRef" :model="paymentForm" :rules="paymentRules" label-width="80px">
            <el-form-item label="订单编号">
                {{ selectedOrders.map(item => item.orderNumber).join(' | ') }}
            </el-form-item>
            <el-form-item label="支付方式">
                <el-radio-group v-model="paymentForm.paymentMethod">
                    <el-radio v-for="dict in sys_payment_method" :key="dict.value" :label="dict.value">
                        {{ dict.label }}
                    </el-radio>
                </el-radio-group>
            </el-form-item>
            <el-form-item label="储值卡">
                <!-- 列出储值卡列表 -->
                <el-checkbox-group v-if="selectedOrders.length<2" v-model="couponStorageCardId" @change="changeCoupon(1)">
                    <el-checkbox v-for="card in userCouponList.filter(item => item.coupon.couponType == '000')"
                        :disabled="!card.isValid" :key="card.ucId" :value="card.ucId">
                        {{ card.coupon.couponTitle }}
                        -余额
                        {{ card.availableValue }}
                        {{ card.coupon.couponType == '000' ? '元' : '次' }}
                        {{ card.isValid ? '' : '(' + card.unValidReason + ')' }}
                    </el-checkbox>
                </el-checkbox-group>
                <span v-else>多个订单暂时不支持使用卡券</span>
            </el-form-item>
            <el-form-item label="优惠券">
                <el-radio-group v-if="selectedOrders.length<2" v-model="paymentForm.couponId" @change="changeCoupon(2)">
                    <el-radio v-for="card in userCouponList.filter(item => item.coupon.couponType !== '000')"
                        :disabled="!card.isValid" :key="card.ucId" :value="card.ucId">
                        {{ card.coupon.couponTitle }}
                        <!-- - -->
                        <!-- {{ card.ucCount }} -->
                        <!-- 张 -->
                        {{ card.isValid ? '' : '(' + card.unValidReason + ')' }}
                    </el-radio>
                </el-radio-group>
                <span v-else>多个订单暂时不支持使用卡券</span>
            </el-form-item>
            <el-row>
                <el-col :span="8">
                    <el-form-item label="订单金额">
                        {{ paymentForm.totalAmount }}
                    </el-form-item>
                </el-col>
                <el-col :span="8">
                    <el-form-item label="优惠金额">
                        {{ paymentForm.bonusAmount }}
                    </el-form-item>
                </el-col>
                <el-col :span="8">
                    <el-form-item label-width="auto" label="优惠后金额">
                        {{ paymentForm.paymentAmount }}
                    </el-form-item>
                </el-col>
            </el-row>
            <el-row>
                <el-form-item label="补差价" v-if="priceDiff > 0">
                    {{ priceDiff }}
                </el-form-item>
            </el-row>
        </el-form>
        <template #footer>
            <div class="payment-footer">
                <el-button type="primary" @click="submitPaymentForm">确认收款</el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup name="OderContent">
import { selectListExceptCompleted, pay } from "@/api/system/orders";
import { listCloths, getCloths } from "@/api/system/cloths";
import { listTags } from "@/api/system/tags";
import { onMounted } from "vue";
import { delivery, pickUp } from ".@/api/system/cloths";
import { listUserCoupon } from '@/api/system/user_coupon';
import { isCurrentTimeWithinRange } from "@/utils";

const props = defineProps({
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
    sys_cost_time_alarm,
    sys_payment_status,
    sys_price_order_type,
    sys_business_type,
    sys_delivery_mode,
    sys_order_type,
    sys_order_status,
    sys_payment_method,
    sys_notice_method,
    sys_service_requirement,
    sys_service_type,
    sys_clothing_status
} =
    proxy.useDict(
        'sys_cost_time_alarm',
        'sys_payment_status',
        "sys_price_order_type",
        "sys_business_type",
        "sys_delivery_mode",
        "sys_order_type",
        "sys_order_status",
        "sys_payment_method",
        "sys_notice_method",
        "sys_service_requirement",
        "sys_service_type",
        "sys_clothing_status",
    );

const showOrderDialog = ref(props.visible);

const rowClassName = ref("row-class-name");

// 订单列表
const ordersList = ref([]);
const showPaymentDialog = ref(false);
const loading = ref(true);
const total = ref(0);
const colorList = ref([]);
const flawList = ref([]);
const estimateList = ref([]);
const brandList = ref([]);
const pictureList = ref([]);
const selectedCloths = ref([]);
const selectedOrders = ref([]);
// 展开的父行
const expandedRows = ref([]);
const orderSelectedList = ref([]);

const orderTableRef = ref();
const clothsTableRef = ref();
// 用户卡券列表
const userCouponList = ref([]);
const showPicture = ref(false);
const showDeliveryDialog = ref(false);
const baseUrl = import.meta.env.VITE_APP_BASE_API;
const pictureUrl = ref(baseUrl + "/system/cloths/download/");
// 总价格
const totalPrice = ref(0);

const data = reactive({
    deliveryForm: {},
    paymentForm: {},
    pickupRules: {},
    queryParams: {
        pageNum: 1,
        pageSize: 10,
        orderNumber: null,
        phonenumber: null,
        pickupCode: null,
    },
});

const { deliveryForm, paymentForm, pickupRules, queryParams } = toRefs(data);

function handlePay() {
    const orders = selectedOrders.value.filter(item => item.paymentStatus === '01');
    if (orders.length == 0) {
        proxy.$modal.msgWarning("没有选中为支付的订单");
        return;
    }

    // 判断是否是同一个会员的衣物
    const userIdList = [...new Set(orders.map(item => item.userId))];
    if (userIdList.length === 0) {
        proxy.$modal.msgError("错误");
        return;
    }
    if (userIdList.length > 1) {
        proxy.$modal.msgWarning("请选择同一会员的订单");
        return;
    }

    // 计算总价格
    totalPrice.value = selectedCloths.value.reduce((acc, cur) => acc + cur.priceValue+cur.processMarkup, 0);
    // 获取用户卡券列表
    listUserCoupon({ userId: userIdList[0] }).then(response => {
        userCouponList.value = response.rows;
        initPaymentForm();
        checkCoupon();
        showPaymentDialog.value = true;
    });
}


/* 判断卡券是否有效 */
function checkCoupon() {
    // 判断每个卡券是否有效
    for (const item of userCouponList.value) {
        item.isValid = true;
        item.unValidReason = '';
        // 判断有效期
        if (!isCurrentTimeWithinRange(item.coupon.validFrom, item.coupon.validTo)) {
            item.isValid = false;
            item.unValidReason = "不在有效期内";
            continue;
        }

        // 判断最低消费金额
        if ((item.coupon.couponType == '003' || item.coupon.couponType == '004') && item.coupon.minSpend > totalPrice.value) {
            item.isValid = false;
            item.unValidReason = "最低消费金额不足";
            continue;
        }
        // 适用衣物列表
        const applicableClothsList = item.coupon.applicableCloths ? item.coupon.applicableCloths.split(',') : [];
        // 适用分类列表
        const applicableStyleList = item.coupon.applicableStyle ? item.coupon.applicableStyle.split(',') : [];
        // 适用品类列表
        const applicableCategoryList = item.coupon.applicableCategory ? item.coupon.applicableCategory.split(',') : [];
        // 判断品类
        for (const cloth of selectedCloths.value) {
            // 先判断适用衣物
            if (applicableClothsList.length != 0 && !applicableClothsList.includes(cloth.clothInfo.clothingId)) {
                item.isValid = false;
                item.unValidReason = "适用衣物不匹配";
                break;
            }

            // 判断适用分类
            if (applicableStyleList.length != 0 && applicableStyleList.includes(cloth.applicableStyle)) {
                item.isValid = false;
                item.unValidReason = "适用分类不匹配";
                break;
            }
            // 判断适用品类
            if (applicableCategoryList.length != 0 && applicableCategoryList.includes(cloth.applicableCategory)) {
                item.isValid = false;
                item.unValidReason = "适用品类不匹配";
                break;
            }
        }
    }
}
/* 初始化支付表单数据 */
function initPaymentForm() {
    paymentForm.value = {
        paymentMethod: '02',
        orderType: '1',
        totalAmount: totalPrice.value,
    }

}

/* 收款 */
function submitPaymentForm() {
    // 判断是否使用了优惠券
    if (!paymentForm.value.couponId) {
        if (couponStorageCardId.value.length > 0) {
            // 计算使用了多少储值卡
            let storageCardPrice = 0;
            userCouponList.value.forEach(item => {
                if (couponStorageCardId.value.includes(item.ucId)) {
                    storageCardPrice += item.availableValue;
                }
            });
            paymentForm.value.paymentAmountVip = storageCardPrice;
            paymentForm.value.ucId = couponStorageCardId.value.join(',');
            // 使用了储值卡，那么实际从微信/或其他支付方式中扣除的金额为差价
            paymentForm.value.paymentAmountMv = priceDiff.value;
        } else {
            // 什么卡券都没用
            paymentForm.value.ucId = null;
            paymentForm.value.paymentAmountMv = totalPrice.value;
        }
    } else {
        paymentForm.value.ucId = String(paymentForm.value.couponId);
        // 用了优惠券，那么实际从微信/或其他支付方式中扣除的金额为优惠后的总金额
        paymentForm.value.paymentAmountMv = paymentForm.value.paymentAmount;
    }
    paymentForm.value.totalAmount = Number(paymentForm.value.totalAmount);
    // 
    console.log(paymentForm.value)
    pay(paymentForm.value).then(res => {
        proxy.$modal.msgSuccess('支付成功');
        showPaymentDialog.value = false;
        open.value = false;
    })
}

function changeCoupon(couponType) {
    if (couponType == 1) {
        paymentForm.value.couponId = null;
        paymentForm.value.bonusAmount = 0;
        // 计算差价
        let storageCardPrice = 0;
        userCouponList.value.forEach(item => {
            if (couponStorageCardId.value.includes(item.ucId)) {
                storageCardPrice += item.availableValue;
            }
        });
        if (storageCardPrice < totalPrice.value) {
            priceDiff.value = totalPrice.value - storageCardPrice;
        }
    }
    //计算优惠金额
    if (couponType == 2) {
        couponStorageCardId.value = [];
        const coupon = userCouponList.value.find(item => item.couponId == paymentForm.value.couponId);
        // 满减券
        if (coupon.coupon.couponType == '004') {
            paymentForm.value.bonusAmount = coupon.coupon.usageValue;
        }
        // 折扣券
        if (coupon.coupon.couponType == '003') {
            paymentForm.value.bonusAmount = totalPrice.value * coupon.coupon.usageValue / 100;
        }

    }
    paymentForm.value.paymentAmount = totalPrice.value - (paymentForm.value.bonusAmount ? paymentForm.value.bonusAmount : 0);
}

// 显示取走
function pickup() {
    const cloths = selectedCloths.value.filter(item => item.clothingStatus === '02');
    if (cloths.length == 0) {
        proxy.$modal.msgWarning("没有选中符合条件的衣物");
        return;
    }

    const ids = cloths.map(item => item.clothId);

    const orderIds = cloths.map(item => item.orderClothId);
    // 判断是否是同一个会员的衣物
    const userIdList = [...new Set(ordersList.value.filter(item => orderIds.includes(item.orderId)).map(item => item.userId))];
    if (userIdList.length === 0) {
        proxy.$modal.msgError("错误");
        return;
    }
    if (userIdList.length > 1) {
        proxy.$modal.msgWarning("请选择同一会员的衣物");
        return;
    }
    // return;
    pickUp(ids).then(res => {
        proxy.$modal.msgSuccess("取走成功");
        selectedCloths.value = [];
        getList();
    }).catch(err => {
        proxy.$modal.msgError(err.message);
    })
}

function handleDelivery() {
    const ids = selectedCloths.value.filter(item => item.clothingStatus === '02').map(item => item.clothId);
    if (ids.length == 0) {
        proxy.$modal.msgWarning("没有选中符合条件的衣物");
        return;
    }
    showDeliveryDialog.value = true;
}

function submitDelivery() {
    const ids = selectedCloths.value.filter(item => item.clothingStatus === '02').map(item => item.clothId);
    deliveryForm.value.clothingIds = ids.join(',');
    delivery(deliveryForm.value).then(res => {
        showDeliveryDialog.value = false;
        proxy.$modal.msgSuccess("操作成功");
    })
}

// 处理父行选择变化
async function handleOrderSelectionChange(selection, row) {
    selectedOrders.value = selection;
    // 获取选中的父行（订单）
    const isSelected = selection.includes(row);

    if (isSelected) {
        // 选中父行时，自动展开并选中其子行
        await selection.forEach(async order => {
            // 自动展开父行
            if (!expandedRows.value.includes(order.orderId)) {
                expandedRows.value.push(order.orderId);
                order.loading = true;
                await listCloths({ orderClothId: order.orderId }).then(res => {
                    order.clothList = res.rows;
                    order.loading = false;
                    // 展开行
                    orderTableRef.value.toggleRowExpansion(order);
                })
            }
            // 选中该父行的所有子行
            order.clothList.forEach(cloth => {
                // 选中子行
                clothsTableRef.value.toggleRowSelection(cloth, true);
                // 将子行加入 selectedCloths 数组
                if (!selectedCloths.value.find(c => c.clothId === cloth.clothId)) {
                    selectedCloths.value.push(cloth);
                }

            });
        });
    } else {

        // 当取消选择父行时，取消该行所有子行的选中状态
        const selectedOrderIds = selection.map(order => order.orderId);
        row.clothList.forEach(cloth => clothsTableRef.value.toggleRowSelection(cloth, false));
        selectedCloths.value = selectedCloths.value.filter(cloth => selectedOrderIds.includes(cloth.orderId));
        console.log('result', selectedCloths.value)
    }
};

// 衣物列表多选框选中数据
function handleClothSelectionChange(selectedItems, row) {
    // console.log('cloth selection change', selectedItems, row);
    // 清空当前订单下的选中数据
    const orderId = row.orderId;
    selectedCloths.value = selectedCloths.value.filter(cloth => cloth.orderClothId !== orderId);

    // 将新的选中项合并到 shared array
    selectedCloths.value.push(...selectedItems);
    if (!selectedCloths.value.find(item => item.orderClothId === orderId)) {
        // 说明该订单下已经没有选中项，则删除该订单
        orderTableRef.value.toggleRowSelection(row, false);
    } else {
        if (selectedCloths.value.filter(item => item.orderClothId === orderId).length === row.clothList.length) {
            orderTableRef.value.toggleRowSelection(row, true);
        }
    }
}

function cancelPickup() {
    showDeliveryDialog.value = false;
    resetPickupForm();
}
// 重置取走form
function resetPickupForm() {
    deliveryForm.value = {
        orderClothId: null,
        clothingId: null,
        clothingCategory: null,
        clothingStyle: null,
        clothingColor: null,
        clothingFlaw: null,
        estimate: null,
        clothingBrand: null,
        serviceType: null,
        serviceRequirement: null,
        beforePics: null,
    }
    proxy.resetForm("pickupRef");
}

function handleExpandChange(row, expanded) {
    if (expanded && !row.clothList) {
        row.loading = true;
        listCloths({ orderClothId: row.orderId }).then(res => {
            row.clothList = res.rows;
            row.loading = false;
        });
    }
}

function handleRowClick(row) {
    orderTableRef.value.toggleRowExpansion(row);
}

// 监听传入的visible属性
watch(() => props.visible, async (newVal) => {
    if (newVal) {
        showOrderDialog.value = true;
        await initList();
        getList(); // 当 visible 变为 true 时，加载数据
    } else {
        showOrderDialog.value = false; // 隐藏对话框
        ordersList.value = [];
    }
});

/* 初始化列表数据 */
async function initList() {
    const promises = [];

    // 获取颜色列表
    if (colorList.value.length === 0) {
        const colorPromise = listTags({ tagOrder: '003' }).then(response => {
            colorList.value = response.rows;
        });
        promises.push(colorPromise);
    }

    // 获取瑕疵列表
    if (flawList.value.length === 0) {
        const flawPromise = listTags({ tagOrder: '001' }).then(response => {
            flawList.value = response.rows;
        });
        promises.push(flawPromise);
    }

    // 获取预估列表
    if (estimateList.value.length === 0) {
        const estimatePromise = listTags({ tagOrder: '002' }).then(response => {
            estimateList.value = response.rows;
        });
        promises.push(estimatePromise);
    }

    // 获取品牌列表
    if (brandList.value.length === 0) {
        const brandPromise = listTags({ tagOrder: '004' }).then(response => {
            brandList.value = response.rows;
        });
        promises.push(brandPromise);
    }

    // 等待所有异步操作完成防止衣物列表数据加载完后这里的数据没有准备好而出错
    await Promise.all(promises);
}

/** 查询洗护服务订单列表 */
function getList() {
    if (!showOrderDialog.value) {
        return;
    }
    loading.value = true;
    selectListExceptCompleted(queryParams.value).then(response => {
        ordersList.value = response.rows;
        if (ordersList.value.length < 4) {
            ordersList.value.map(item => {
                item.loading = true;
                listCloths({ orderClothId: item.orderId }).then(res => {
                    item.clothList = res.rows;
                    item.loading = false;
                    // 展开行
                    orderTableRef.value.toggleRowExpansion(item);
                })
            })
        } else {
            ordersList.value.map(item => item.loading = false);
        }
        total.value = response.total;
    });
}

/* 获取图片列表id */
function handleShowPicture(row, flag) {
    showPicture.value = true;
    getCloths(row.clothId).then(response => {
        if (flag) {
            pictureList.value = response.data.beforePics ?
                response.data.beforePics.split(',').map(item => pictureUrl.value + item) : [];
        } else {
            pictureList.value = response.data.afterPics ?
                response.data.afterPics.split(',').map(item => pictureUrl.value + item) : [];
        }
        console.log(pictureList.value)
    });
}
/** 搜索按钮操作 */
function handleQuery() {
    queryParams.value.pageNum = 1;
    getList();
}

/** 重置按钮操作 */
function resetQuery() {
    proxy.resetForm("queryRef");
    handleQuery();
}

// 如果初始化时visible是true，则直接加载数据
onMounted(async () => {
    if (props.visible) {
        await initList();
        getList();
    }
});
</script>
<style scoped>
.el-table {
    border: none;

    tr {
        border: 1px dashed;
        border-radius: .4rem;
    }
}

.cloths-table {
    border-radius: .4rem;
    border: 1px dashed;
}

.payment-status {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.img-container {
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-start;
    align-items: center;
    gap: 1rem;
}

.img-item {
    flex: 1 1 calc(33.333% - 1rem);
    /* 每行 3 个元素 */
    box-sizing: border-box;
}

.payment-footer {
  text-align: center;
}

.el-dialog .pagination-container {
    position: relative !important;
    margin-top: 0;
}
</style>
<style>
.row-class-name {
    background-color: rgb(5, 252, 169) !important;
    cursor: pointer;
    border: 1px dashed !important;
    border-radius: .4rem !important;
}
</style>