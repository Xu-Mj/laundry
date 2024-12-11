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
        <!-- 渲染订单抖索结果列表 -->
        <div class="search-result-list">

            <div class="result-item" v-for="order in ordersList" :key="order.orderId">
                <!-- 信息行 -->
                <div class="result-item-info">
                    <!-- 订单编码 -->
                    <span>订单编码: {{ order.orderNumber }}</span>
                    <!-- 所属会员 -->
                    <span>所属会员: {{ order.nickName }}</span>
                    <!-- 实际支付金额 -->
                    <span style="display: flex; align-items: center; gap: .5rem;">实际支付金额:
                        <span style="color: red;font-weight: bold; align-items: center;">
                            {{ order.mount }}
                        </span>
                    </span>
                    <!-- 取件码 -->
                    <span>取件码: {{ order.pickupCode }}</span>
                    <!-- 支付状态 -->
                    <span style="display: flex; align-items: center; gap: .5rem;">支付状态:
                        <!-- <dict-tag :options="sys_payment_status" :value="order.paymentStatus" /> -->
                        <dict-tag v-if="order.paymentStatus === '01'" style="cursor: pointer;" @click="go2pay(order)"
                            :options="sys_payment_status" :value="order.paymentStatus" />
                        <dict-tag v-else :options="sys_payment_status" :value="order.paymentStatus" />
                    </span>
                </div>
                <!-- 订单包含的衣物列表 -->
                <el-table v-if="order.clothList && order.clothList.length > 0" class="cloths-table"
                    :data="order.clothList" :loading="order.loading" row-key="clothingId"
                    @selection-change="selectedItems => handleClothSelectionChange(selectedItems, order)"
                    ref="clothsTableRef" border="dash">
                    <el-table-column type="selection" width="55" align="center" />
                    <el-table-column label="衣物" align="center">
                        <template #default="scope">
                            {{ scope.row.clothInfo.clothingName }}
                            {{ scope.row.clothingColor ? '-' + colorList.find(item => item.tagId ==
                                scope.row.clothingColor).tagName : '' }}
                        </template>
                    </el-table-column>
                    <el-table-column label="衣物编码" align="center" prop="clothingColor" width="110">
                        <template #default="scope">
                            {{ scope.row.hangClothCode }}
                        </template>
                    </el-table-column>
                    <el-table-column label="服务类型" align="center">
                        <template #default="scope">
                            <span class="service-type">
                                <dict-tag :options="sys_service_type" :value="scope.row.serviceType" />
                                -
                                <dict-tag :options="sys_service_requirement" :value="scope.row.serviceRequirement" />
                            </span>
                        </template>
                    </el-table-column>
                    <el-table-column label="洗护价格" align="center" prop="priceValue" />
                    <el-table-column label="工艺加价" align="center" prop="processMarkup" />
                    <el-table-column label="衣物瑕疵" align="center" prop="clothingFlaw">
                        <template #default="scope">
                            <el-tag v-for="tagId in scope.row.clothingFlaw ? scope.row.clothingFlaw.split(',') : []"
                                :key="tagId" type="danger">
                                {{ flawList.find(item => item.tagId == tagId).tagName }}
                            </el-tag>
                        </template>
                    </el-table-column>
                    <el-table-column label="洗后预估" align="center" prop="estimate">
                        <template #default="scope">
                            <el-tag v-for="tagId in scope.row.estimate ? scope.row.estimate.split(',') : []" :key="tagId"
                                type="primary">
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
                    <el-table-column label="图片" align="center" class-name="small-padding fixed-width">
                        <template #default="scope">
                            <el-button link type="primary"
                                :disabled="scope.row.beforePics == null || scope.row.beforePics.length == 0"
                                @click="handleShowPicture(scope.row, true)"
                                v-hasPermi="['system:cloths:edit']">洗前</el-button>
                            <el-button link type="primary"
                                :disabled="scope.row.afterPics == null || scope.row.afterPics.length == 0"
                                @click="handleShowPicture(scope.row, false)"
                                v-hasPermi="['system:cloths:edit']">洗后</el-button>
                        </template>
                    </el-table-column>
                    <el-table-column label="洗护状态" align="center" prop="clothingStatus">
                        <template #default="scope">
                            <dict-tag :options="sys_clothing_status" :value="scope.row.clothingStatus" />
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
                    <el-table-column label="上挂备注" align="center" prop="hangRemark" />
                </el-table>

            </div>
        </div>

        <!--footer包含 四个button -->
        <template #footer>
            <el-button type="primary" @click="pickup">取衣</el-button>
            <el-button @click="handlePay">取衣收款</el-button>
            <el-button @click="handleDelivery">上门派送</el-button>
            <el-button @click="handleReWash">售后复洗</el-button>
            <el-button @click="() => {}">补打小票</el-button>
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
    <el-dialog v-model="showDeliveryDialog" width="500px" :show-close="false" append-to-body>
        <el-form ref="pickupRef" :model="deliveryForm" :rules="pickupRules" label-width="80px">
            <!-- 配送地址/配送时间/备注信息 -->
            <el-form-item label="配送地址" prop="address">
                <div class="address">
                    <el-input v-model="deliveryForm.address" @input="needSync = true" placeholder="请输入配送地址" />
                    <el-checkbox v-if="needSync" v-model="deliveryForm.needSync">更新会员默认地址</el-checkbox>
                </div>
            </el-form-item>
            <el-form-item label="配送时间" prop="dispatchTime">
                <el-date-picker v-model="deliveryForm.dispatchTime" type="date" placeholder="选择日期" />
            </el-form-item>
            <el-form-item label="备注信息" prop="remark">
                <el-input type="textarea" v-model="deliveryForm.remark" placeholder="请输入备注信息" />
            </el-form-item>
        </el-form>
        <!-- 取消确认 -->
        <template #footer>
            <div class="pickup-footer">
                <el-button type="primary" @click="submitDelivery">确认派送</el-button>
                <el-button type="primary" @click="cancelDelivery">取消</el-button>
            </div>
        </template>
    </el-dialog>


    <!-- 付款弹窗 -->
    <el-dialog title="付款" v-model="showPaymentDialog" width="600px" append-to-body lock-scroll modal
        :close-on-click-modal="false">
        <el-form ref="paymentRef" :model="paymentForm" :rules="paymentRules" label-width="80px">
            <el-form-item label="订单编号">
                {{ paymentForm.titles }}
            </el-form-item>
            <el-form-item label="支付方式">
                <el-radio-group v-model="paymentForm.paymentMethod">
                    <template v-for="dict in sys_payment_method" :key="dict.value">
                        <template v-if="dict.value == '06'">
                            <el-radio v-if="couponTypeList.has('000')" :value="dict.value">
                                {{ dict.label }}
                            </el-radio>
                        </template>
                        <template v-else-if="dict.value == '07'">
                            <el-radio v-if="couponTypeList.has('002')" :value="dict.value">
                                {{ dict.label }}
                            </el-radio>
                        </template>
                        <el-radio v-else-if="dict.value !== '03' && dict.value !== '04'" :value="dict.value">
                            {{ dict.label }}
                        </el-radio>
                    </template>
                </el-radio-group>
            </el-form-item>
            <template v-if="showCoupons">
                <el-form-item v-if="userCouponList.filter(item => item.coupon.couponType == '000').length !== 0"
                    label="储值卡">
                    <el-checkbox-group v-model="couponStorageCardId" @change="changeCoupon(1)">
                        <el-checkbox v-for="card in userCouponList.filter(item => item.coupon.couponType == '000')"
                            :disabled="!card.isValid" :key="card.ucId" :value="card.ucId">
                            {{ card.coupon.couponTitle }}
                            -余额
                            {{ card.availableValue }}
                            {{ card.coupon.couponType == '000' ? '元' : '次' }}
                            {{ card.isValid ? '' : '(' + card.unValidReason + ')' }}
                        </el-checkbox>
                    </el-checkbox-group>
                </el-form-item>
                <el-form-item v-if="userCouponList.filter(item => item.coupon.couponType == '002').length != 0"
                    label="次卡">
                    <div class="coupon-times">
                        <div class="coupon-times-item"
                            v-for="card in userCouponList.filter(item => item.coupon.couponType == '002')"
                            :key="card.ucId">
                            <el-checkbox @change="changeCoupon(2, card)" :disabled="!card.isValid"
                                v-model="card.selected" :value="card.ucId">
                                {{ card.coupon.couponTitle }}
                                {{ card.isValid ? '' : '(' + card.unValidReason + ')' }}
                                {{ '(剩余: ' + card.availableValue + '次)' }}
                            </el-checkbox>
                            <el-input-number controls-position="right" v-if="card.selected" v-model="card.count"
                                @change="changeCouponCount(card)" :min="1" :max="card.availableValue"
                                placeholder="请输入次卡数量" />
                        </div>
                    </div>
                </el-form-item>
                <el-form-item
                    v-if="userCouponList.filter(item => item.coupon.couponType !== '002' && item.coupon.couponType !== '000').length != 0"
                    label="优惠券">
                    <el-radio-group v-model="paymentForm.couponId" @change="changeCoupon(3)">
                        <el-radio
                            v-for="card in userCouponList.filter(item => item.coupon.couponType !== '000' && item.coupon.couponType !== '002')"
                            :disabled="!card.isValid" :key="card.ucId" :value="card.ucId">
                            {{ card.coupon.couponTitle }}
                            {{ card.isValid ? '' : '(' + card.unValidReason + ')' }}
                            {{ '(剩余: ' + card.ucCount + '张)' }}
                        </el-radio>
                    </el-radio-group>
                </el-form-item>
            </template>
            <el-row>
                <el-col :span="8">
                    <el-form-item label="订单金额">
                        <span class="payment-amount">
                            {{ paymentForm.totalAmount }}
                        </span>
                    </el-form-item>
                </el-col>
                <el-col :span="8">
                    <el-form-item label="优惠金额">
                        {{ paymentForm.bonusAmount }}
                    </el-form-item>
                </el-col>
                <el-col :span="8">
                    <el-form-item label-width="auto" label="优惠后金额">
                        <span class="payment-amount">
                            {{ paymentForm.paymentAmount }}
                        </span>
                    </el-form-item>
                </el-col>
            </el-row>
            <el-row>
                <el-form-item label="补差价" v-if="paymentForm.priceDiff > 0">
                    <el-input-number v-model="paymentForm.priceDiff" controls-position="right" :min="0"
                        :max="paymentForm.paymentAmount" placeholder="请输入补差价" />
                </el-form-item>
            </el-row>
        </el-form>
        <template #footer>
            <div class="payment-footer">
                <el-button type="primary" @click="submitPaymentForm(false)">确认收款</el-button>
                <el-button type="primary" @click="submitPaymentForm(true)">收款并取衣</el-button>
            </div>
        </template>
    </el-dialog>

    <!-- 复洗 -->
    <ReWash :visible="showRewashDialog" :order="rewashOrder" :clothes="rewashClothesId"
        :refresh="() => { selectedCloths = []; getList(); }" :key="showRewashDialog"
        :toggle="() => { showRewashDialog = !showRewashDialog }" />

</template>

<script setup name="OderContent">
import { pay } from "@/api/system/orders";
import { listCloths, getCloths } from "@/api/system/cloths";
import { listTagsNoLimit } from "@/api/system/tags";
import { onMounted } from "vue";
import { delivery, pickUp } from "@/api/system/cloths";
import { listUserCouponWithValidTime } from '@/api/system/user_coupon';
import { getUser } from '@/api/system/user';
import { isCurrentTimeWithinRange } from "@/utils";
import { selectListExceptCompleted } from "@/api/system/orders";
import { getPrice } from "@/api/system/price";
import ReWash from "./rewash.vue";
import { ElMessageBox } from 'element-plus'

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
    sys_payment_status,
    sys_delivery_mode,
    sys_payment_method,
    sys_service_requirement,
    sys_service_type,
    sys_clothing_status
} =
    proxy.useDict(
        'sys_payment_status',
        "sys_delivery_mode",
        "sys_payment_method",
        "sys_service_requirement",
        "sys_service_type",
        "sys_clothing_status",
    );

const showOrderDialog = ref(props.visible);

// 订单列表
const ordersList = ref([]);
const showPaymentDialog = ref(false);
const showRewashDialog = ref(false);
const loading = ref(true);
const colorList = ref([]);
const flawList = ref([]);
const estimateList = ref([]);
const brandList = ref([]);
const pictureList = ref([]);
const selectedCloths = ref([]);
const selectedOrders = ref([]);
// 展开的父行
const expandedRows = ref([]);
// 选中的储值卡列表
const couponStorageCardId = ref([]);

const orderTableRef = ref();
const clothsTableRef = ref();
// 用户卡券列表
const userCouponList = ref([]);
// 用户卡券种类列表
const couponTypeList = ref();

const showPicture = ref(false);
const showDeliveryDialog = ref(false);
const baseUrl = import.meta.env.VITE_APP_BASE_API;
const pictureUrl = ref(baseUrl + "/system/cloths/download/");
// 总价格
const totalPrice = ref(0);

// 选中的次卡数量总数
const timeCardCount = ref(0);
// 当前需要处理的衣物列表
const clothsList = ref([]);

// 当前用户信息
const currentUser = ref(null);

// 显示更新会员默认地址
const needSync = ref(false);

const showCoupons = ref(true);

const rewashOrder = ref(null);
const rewashClothesId = ref([]);


const data = reactive({
    deliveryForm: {},
    paymentForm: {},
    pickupRules: {},
    queryParams: {
        orderNumber: null,
        phonenumber: null,
        pickupCode: null,
    },
});

const { deliveryForm, paymentForm, pickupRules, queryParams } = toRefs(data);

async function go2pay(row) {
    initPaymentForm();
    paymentForm.value.orders = [row];
    // 获取用户的卡券列表
    await listUserCouponWithValidTime({ userId: row.userId }).then(response => {
        userCouponList.value = response;
        // 初始化次卡信息
        userCouponList.value.filter(item => item.coupon.couponType == '002').map(item => {
            item.selected = false;
            item.count = 1;
        })
    });
    // 计算订单标题栏以及订单总金额
    paymentForm.value.titles = paymentForm.value.orders.map(item => item.orderNumber + `(` + item.mount + `元)`).join(' | ');
    paymentForm.value.totalAmount = paymentForm.value.orders.reduce((acc, cur) => acc + cur.mount, 0);

    // 校验卡券
    checkCoupon();
    // 判断储值卡金额是否能够覆盖订单金额
    const storageCardValue = userCouponList.value.filter(item => item.coupon.couponType === "000" && item.isValid).reduce((acc, cur) => acc + cur.availableValue, 0);
    if (paymentForm.value.totalAmount < storageCardValue) {
        paymentForm.value.paymentMethod = '06';
    }
    paymentForm.value.bonusAmount = 0;
    paymentForm.value.paymentAmount = paymentForm.value.totalAmount;
    showPaymentDialog.value = true;
}

// 显示售后复洗
function handleReWash() {
    if (selectedCloths.value.length == 0) {
        proxy.$message.error("请先选择衣物");
        return;
    }

    const orders = new Set(selectedCloths.value.map(item => item.orderClothId));
    if (orders.length > 1) {
        proxy.$message.error("不支持跨订单复洗");
        return;
    }
    rewashClothesId.value = selectedCloths.value.map(item => item.clothId);
    rewashOrder.value = ordersList.value.find(item => item.orderId == orders.values().next().value);

    showRewashDialog.value = true;
}

function changeCouponCount() {
    // 计算默认数量
    // 计算选中的次卡数量
    const count = userCouponList.value.filter(item => item.selected).reduce((acc, item) => {
        if (item.coupon.couponType == '002') {
            acc += item.count;
        }
        return acc;
    }, 0);
    // paymentForm.value.couponCount = count;
    if (count == 0) {
        paymentForm.value.paymentMethod = '02';
        paymentForm.value.bonusAmount = 0;
    } else {
        // 计算差价
        if (clothsList.value.length > count) {
            // 需要补充差价
            const diffCount = clothsList.value.length - count;
            // 获取diffCount数量的衣物
            const diffCloths = clothsList.value.slice(0, diffCount);
            // 计算差价
            paymentForm.value.priceDiff = diffCloths.reduce((acc, cloth) => acc + cloth.priceValue, 0);
            paymentForm.value.bonusAmount = paymentForm.value.totalAmount - paymentForm.value.priceDiff;
            paymentForm.value.paymentMethod = '02';
        } else {
            paymentForm.value.priceDiff = 0;
            paymentForm.value.paymentMethod = '07';
            paymentForm.value.bonusAmount = paymentForm.value.totalAmount;
        }
    }
    paymentForm.value.paymentAmount = paymentForm.value.totalAmount - (paymentForm.value.bonusAmount ? paymentForm.value.bonusAmount : 0);
    // console.log(paymentForm.value)
}

async function handlePay() {
    if (ordersList.value.length == 0) {
        proxy.$modal.msgWarning("没有可支付的订单或可取走的衣物");
        return;
    }
    initPaymentForm();
    // 获取用户的卡券列表
    await listUserCouponWithValidTime({ userId: ordersList.value[0].userId }).then(response => {
        userCouponList.value = response;
        // 初始化次卡信息
        userCouponList.value.filter(item => item.coupon.couponType == '002').map(item => {
            item.selected = false;
            item.count = 1;
        })
    });
    // 遍历订单列表
    // 1. 没有选中衣物，只是支付
    if (selectedCloths.value.length == 0) {
        // 遍历所有的查询结果
        paymentForm.value.orders = ordersList.value.filter(item => item.paymentStatus === '01');
        clothsList.value = paymentForm.value.orders.flatMap(order => order.clothList) // 展开每个订单的衣物列表
            .sort((a, b) => b.priceValue - a.priceValue);
    } else {
        // 查询选中衣物所属的订单
        const orderIds = new Set(selectedCloths.value.map(item => item.orderClothId));
        paymentForm.value.orders = ordersList.value.filter(item => orderIds.has(item.orderId) && item.paymentStatus === '01');
        const ids = paymentForm.value.orders.map(item => item.orderId);
        // 排序
        clothsList.value = selectedCloths.value.filter(item => ids.includes(item.orderClothId)).sort((a, b) => b.priceValue - a.priceValue);
    }
    if (paymentForm.value.orders.length == 0) {
        proxy.$modal.msgWarning("没有选中未支付的订单");
        return;
    }

    // 计算订单标题栏以及订单总金额
    paymentForm.value.titles = paymentForm.value.orders.map(item => item.orderNumber + `(` + item.mount + `元)`).join(' | ');
    paymentForm.value.totalAmount = paymentForm.value.orders.reduce((acc, cur) => acc + cur.mount, 0);

    // 校验订单列表中是否包含选择了价格标签的叮当
    // const hasSelectedPrice = paymentForm.value.orders.some(item => item.priceId != null);
    // if (hasSelectedPrice) {
    //     showCoupons.value = false;
    // } else {
    //     showCoupons.value = true;
    //     // 校验卡券
    //     checkCoupon();
    //     // 判断储值卡金额是否能够覆盖订单金额
    //     const storageCardValue = userCouponList.value.filter(item => item.coupon.couponType === "000" && item.isValid).reduce((acc, cur) => acc + cur.availableValue, 0);
    //     if (paymentForm.value.totalAmount < storageCardValue) {
    //         paymentForm.value.paymentMethod = '06';
    //     }
    // }
    // 校验卡券
    checkCoupon();
    // 判断储值卡金额是否能够覆盖订单金额
    const storageCardValue = userCouponList.value.filter(item => item.coupon.couponType === "000" && item.isValid).reduce((acc, cur) => acc + cur.availableValue, 0);
    if (paymentForm.value.totalAmount < storageCardValue) {
        paymentForm.value.paymentMethod = '06';
    }
    paymentForm.value.bonusAmount = 0;
    paymentForm.value.paymentAmount = paymentForm.value.totalAmount;
    showPaymentDialog.value = true;
    // console.log(paymentForm.value)
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

        let allOrdersInvalid = true;
        // Loop through each order to check coupon validation
        for (const order of paymentForm.value.orders) {
            let orderValid = true;

            // 判断最低消费金额 (Minimum spend for order type '004')
            if (item.coupon.couponType == '004' && item.coupon.minSpend > order.totalPrice) {
                orderValid = false;
                item.unValidReason = "最低消费金额不足";
            }

            // 判断订单类型'003'的金额限制 (Order type '003')
            if (item.coupon.couponType == '003') {
                if (item.coupon.minSpend > order.totalPrice) {
                    orderValid = false;
                    item.unValidReason = "最低消费金额不足";
                }
                if (item.coupon.usageLimit < order.totalPrice) {
                    orderValid = false;
                    item.unValidReason = "订单金额超过使用上限";
                }
            }

            // If at least one order passes, the coupon is valid
            if (orderValid) {
                allOrdersInvalid = false;
                break;
            }
        }

        // If all orders are invalid, mark coupon as invalid
        if (allOrdersInvalid) {
            item.isValid = false;
        }
    }
}

/* 初始化支付表单数据 */
function initPaymentForm() {
    paymentForm.value = {
        paymentMethod: '02',
        orderType: '1',
    }

}

/* 收款 */
async function submitPaymentForm(isPickup) {
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
            // 拆分为单个订单

            // 使用了储值卡，那么实际从微信/或其他支付方式中扣除的金额为差价
            paymentForm.value.paymentAmountMv = paymentForm.value.priceDiff;
            if (paymentForm.value.priceDiff > 0) {
                // 需要补充差价，那么就是组合支付
                if (paymentForm.value.paymentMethod == '01') {
                    paymentForm.value.paymentMethod = '16';
                } else if (paymentForm.value.paymentMethod == '02') {
                    paymentForm.value.paymentMethod = '26';
                } else if (paymentForm.value.paymentMethod == '05') {
                    paymentForm.value.paymentMethod = '56';
                }
            }
        } else if (userCouponList.value.filter(item => item.coupon.couponType == '002' && item.selected).length > 0) {
            // 使用了次卡
            const list = userCouponList.value.filter(item => item.coupon.couponType == '002' && item.selected).map(item => ({
                ucId: item.ucId,
                count: item.count,
            }));
            paymentForm.value.timeBased = list;
            if (paymentForm.value.priceDiff > 0) {
                // 需要补充差价，那么就是组合支付
                if (paymentForm.value.paymentMethod == '01') {
                    paymentForm.value.paymentMethod = '17';
                } else if (paymentForm.value.paymentMethod == '02') {
                    paymentForm.value.paymentMethod = '27';
                } else if (paymentForm.value.paymentMethod == '05') {
                    paymentForm.value.paymentMethod = '57';
                }
            }
        } else {
            // 什么卡券都没用
            paymentForm.value.ucId = null;
            paymentForm.value.paymentAmountMv = totalPrice.value;
        }
    } else {
        const coupon = userCouponList.value.find(item => item.ucId == paymentForm.value.couponId);
        if (coupon && coupon.coupon.couponType == '003') {
            // 折扣券

            if (paymentForm.value.paymentMethod == '01') {
                paymentForm.value.paymentMethod = '18';
            } else if (paymentForm.value.paymentMethod == '02') {
                paymentForm.value.paymentMethod = '28';
            } else if (paymentForm.value.paymentMethod == '05') {
                paymentForm.value.paymentMethod = '58';
            }
        } else if (coupon && coupon.coupon.couponType == '004') {
            // 满减券
            if (paymentForm.value.paymentMethod == '01') {
                paymentForm.value.paymentMethod = '19';
            } else if (paymentForm.value.paymentMethod == '02') {
                paymentForm.value.paymentMethod = '29';
            } else if (paymentForm.value.paymentMethod == '05') {
                paymentForm.value.paymentMethod = '59';
            }
        }
        paymentForm.value.ucId = String(paymentForm.value.couponId);
        // 用了优惠券，那么实际从微信/或其他支付方式中扣除的金额为优惠后的总金额
        paymentForm.value.paymentAmountMv = paymentForm.value.paymentAmount;
    }
    paymentForm.value.totalAmount = Number(paymentForm.value.totalAmount);
    // pay(paymentForm.value).then(async res => {
    //     proxy.$modal.msgSuccess('支付成功');
    //     // 如果选中了衣物，那么同时需要取走
    //     if (selectedCloths.value.length !== 0 && isPickup) {
    //         await pickup();
    //     }
    //     showPaymentDialog.value = false;
    //     getList();
    // })
    try {
        // 等待支付完成
        await pay(paymentForm.value);

        // 支付成功后提示
        proxy.$modal.msgSuccess('支付成功');

        // 修改订单支付状态
        paymentForm.value.orders.forEach(item => item.paymentStatus = '00')
        // 如果选中了衣物，并且需要取走
        if (selectedCloths.value.length !== 0 && isPickup) {
            // 等待 pickup 完成
            await pickup();
        }

        // 关闭支付对话框
        showPaymentDialog.value = false;

        // 获取最新列表
        getList();
    } catch (err) {
        // 错误处理
        console.error('支付失败:', err);
    }
}

function changeCoupon(couponType, card) {
    if (couponType == 1) {
        paymentForm.value.couponId = null;
        paymentForm.value.bonusAmount = 0;
        // 清空次卡选择列表
        userCouponList.value.filter(item => item.coupon.couponType === "002").map(item => item.selected = false)
        // 计算差价
        let storageCardPrice = 0;
        userCouponList.value.forEach(item => {
            if (couponStorageCardId.value.includes(item.ucId)) {
                storageCardPrice += item.availableValue;
            }
        });
        if (storageCardPrice == 0) {
            // 什么都没选中
            paymentForm.value.priceDiff = 0;
        } else if (storageCardPrice < paymentForm.value.totalAmount) {
            paymentForm.value.priceDiff = paymentForm.value.totalAmount - storageCardPrice;
            paymentForm.value.paymentMethod = '02';
        } else {
            paymentForm.value.priceDiff = 0;
            paymentForm.value.paymentMethod = '06';
        }
    } else if (couponType == 2) {
        // 次卡
        // 清空储值卡选择列表
        couponStorageCardId.value = [];
        paymentForm.value.couponId = null;

        // 计算默认数量
        let count = 0;
        if (card.selected) {
            // 计算选中的次卡数量
            count = userCouponList.value.filter(item => item.selected).reduce((acc, item) => {
                if (item.coupon.couponType == '002' && item.ucId !== card.ucId) {
                    acc += item.count;
                }
                return acc;
            }, 0);
            // 计算输入框的数量
            card.count = clothsList.value.length > count ? clothsList.value.length - count > card.availableValue ? card.availableValue : clothsList.value.length - count : clothsList.value.length;
            // 需要再加上card.count
            count += card.count;
        } else {
            count = userCouponList.value.filter(item => item.selected).reduce((acc, item) => {
                if (item.coupon.couponType == '002' && item.ucId !== card.ucId) {
                    acc += item.count;
                }
                return acc;
            }, 0);
        }

        timeCardCount.value = count;
        if (count == 0) {
            paymentForm.value.paymentMethod = '02';
            paymentForm.value.bonusAmount = 0;
            paymentForm.value.priceDiff = 0;
        } else {

            // 计算差价
            if (clothsList.value.length > count) {
                // 需要补充差价
                const diffCount = clothsList.value.length - count;
                // 获取diffCount数量的衣物
                const diffCloths = clothsList.value.slice(0, diffCount);
                // 计算差价
                paymentForm.value.priceDiff = diffCloths.reduce((acc, cloth) => acc + cloth.priceValue, 0);
                paymentForm.value.bonusAmount = paymentForm.value.totalAmount - paymentForm.value.priceDiff;
                paymentForm.value.paymentMethod = '02';
            } else {
                paymentForm.value.priceDiff = 0;
                paymentForm.value.paymentMethod = '07';
                paymentForm.value.bonusAmount = paymentForm.value.totalAmount;
            }
        }


    } else if (couponType == 3) {
        //计算优惠金额
        couponStorageCardId.value = [];
        userCouponList.value.filter(item => item.coupon.couponType === "002").map(item => item.selected = false)
        const coupon = userCouponList.value.find(item => item.ucId == paymentForm.value.couponId);

        // 满减券
        if (coupon.coupon.couponType == '004') {
            paymentForm.value.bonusAmount = coupon.coupon.usageValue;
            paymentForm.value.paymentMethod = '02';
        }
        // 折扣券
        if (coupon.coupon.couponType == '003') {
            let bonusAmount = parseFloat((paymentForm.value.totalAmount * (1 - coupon.coupon.usageValue / 100)).toFixed(2));

            // 进一步处理，不保留小数点后的0
            // if (bonusAmount % 1 === 0) {
            //     bonusAmount = Math.floor(bonusAmount); // 变为整数
            // }

            if (coupon.coupon.usageLimit != 0 && bonusAmount > coupon.coupon.usageLimit) {
                bonusAmount = coupon.coupon.usageLimit;
            }
            paymentForm.value.bonusAmount = bonusAmount;
            // 动态修改支付方式
            paymentForm.value.paymentMethod = '02';
        }

    }
    paymentForm.value.paymentAmount = paymentForm.value.totalAmount - (paymentForm.value.bonusAmount ? paymentForm.value.bonusAmount : 0);
}

// 显示取走
async function pickup() {
    const cloths = selectedCloths.value.filter(item => item.clothingStatus !== '00');
    if (cloths.length == 0) {
        proxy.$modal.msgWarning("没有选中符合条件的衣物");
        return;
    }

    // 筛选正在洗护中的衣物进行提示
    const washCloths = cloths.filter(item => item.clothingStatus === '01');
    if (washCloths.length > 0) {
        try {
            // 显示确认弹窗，用户点击确认后才会执行后续逻辑
            await ElMessageBox.confirm('选择衣物中包含' + washCloths.length + '件正在洗护中的衣物，确认取衣？', 'Warning', {
                confirmButtonText: '取衣',
                cancelButtonText: '取消',
                type: 'warning'
            });
        } catch (error) {
            // 用户点击取消时捕获错误，直接返回，流程不会继续
            return;
        }
    }

    const ids = cloths.map(item => item.clothId);

    const orderIds = cloths.map(item => item.orderClothId);

    // 判断是否包含未支付的订单
    const unpaidOrders = ordersList.value.filter(item => orderIds.includes(item.orderId) && item.paymentStatus !== '00');
    if (unpaidOrders.length > 0) {
        // 弹出询问是否确认取走
        proxy.$modal.confirm("当前选中衣物有未支付订单，是否确认取走？").then(async () => {
            await pickUp(ids).then(res => {
                proxy.$modal.msgSuccess("取走成功");
                selectedCloths.value = [];
                getList();
            }).catch(err => {
                proxy.$modal.msgError(err.message);
            })
        }).catch(() => { })
        return;
    }
    await pickUp(ids).then(res => {
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

    // 初始化配送表单
    deliveryForm.value = {
        address: currentUser.value.address,
        needSync: false,
        dispatchTime: getDate(),
    }
    showDeliveryDialog.value = true;
}

function getDate() {
    const today = new Date();
    const year = today.getFullYear();
    const month = String(today.getMonth() + 1).padStart(2, '0'); // 月份是从0开始的，所以要加1
    const day = String(today.getDate()).padStart(2, '0');

    const currentDate = `${year}-${month}-${day}`;
    return currentDate;
}

function submitDelivery() {
    const cloths = selectedCloths.value.filter(item => item.clothingStatus === '02');
    deliveryForm.value.clothId = cloths.map(item => item.clothId).join(',');
    const orderIds = [...new Set(cloths.map(item => item.orderClothId))];
    deliveryForm.value.orderId = orderIds.join(',');
    console.log(deliveryForm.value)
    delivery(deliveryForm.value).then(res => {
        showDeliveryDialog.value = false;
        proxy.$modal.msgSuccess("操作成功");
        getList();
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
    }
};

// 衣物列表多选框选中数据
function handleClothSelectionChange(selectedItems, row) {
    // console.log('cloth selection change', selectedItems, row);
    // selectedCloths.value = selectedItems;
    // selectedOrders.value = row;
    // 清空当前订单下的选中数据
    const orderId = row.orderId;
    selectedCloths.value = selectedCloths.value.filter(cloth => cloth.orderClothId !== orderId);

    // 将新的选中项合并到 shared array
    selectedCloths.value.push(...selectedItems);
    // if (!selectedCloths.value.find(item => item.orderClothId === orderId)) {
    //     // 说明该订单下已经没有选中项，则删除该订单
    //     orderTableRef.value.toggleRowSelection(row, false);
    // } else {
    //     if (selectedCloths.value.filter(item => item.orderClothId === orderId).length === row.clothList.length) {
    //         orderTableRef.value.toggleRowSelection(row, true);
    //     }
    // }
}

function cancelDelivery() {
    showDeliveryDialog.value = false;
    resetDeliveryForm();
}

// 重置取走form
function resetDeliveryForm() {
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

/* 初始化列表数据 */
async function initList() {
    const promises = [];

    // 获取颜色列表
    if (colorList.value.length === 0) {
        const colorPromise = listTagsNoLimit({ tagOrder: '003' }).then(response => {
            colorList.value = response;
        });
        promises.push(colorPromise);
    }

    // 获取瑕疵列表
    if (flawList.value.length === 0) {
        const flawPromise = listTagsNoLimit({ tagOrder: '001' }).then(response => {
            flawList.value = response;
        });
        promises.push(flawPromise);
    }

    // 获取预估列表
    if (estimateList.value.length === 0) {
        const estimatePromise = listTagsNoLimit({ tagOrder: '002' }).then(response => {
            estimateList.value = response;
        });
        promises.push(estimatePromise);
    }

    // 获取品牌列表
    if (brandList.value.length === 0) {
        const brandPromise = listTagsNoLimit({ tagOrder: '004' }).then(response => {
            brandList.value = response;
        });
        promises.push(brandPromise);
    }

    // 等待所有异步操作完成防止衣物列表数据加载完后这里的数据没有准备好而出错
    await Promise.all(promises);
}

/** 查询洗护服务订单列表 */
async function getList() {
    if (!showOrderDialog.value) return;

    if (queryParams.value.pickupCode === '' && queryParams.value.phonenumber === '' && queryParams.value.orderNumber === '') {
        return;
    }
    loading.value = true;
    ordersList.value = await selectListExceptCompleted(queryParams.value);

    if (ordersList.value.length === 0) {
        proxy.$modal.msgWarning('没有找到相关订单');
    return;
    } 

    // 查询用户信息
    getUser(ordersList.value[0].userId).then(res => {
        currentUser.value = res;
    });

    // 获取用户卡券列表
    await listUserCouponWithValidTime({ userId: ordersList.value[0].userId }).then(res => {
        userCouponList.value = res;
        // 计算用户卡券种类
        couponTypeList.value = new Set(userCouponList.value.map(coupon => coupon.coupon.couponType));
    });

    // 遍历计算订单价格
    for (const item of ordersList.value) {
        item.loading = true;

        item.clothList = await listCloths({ orderId: item.orderId });
        item.loading = false;

        let price = 0;

        // 优先处理 `adjust` 的情况
        if (item.adjust) {
            if (item.adjust.adjustTotal) {
                item.mount = item.adjust.adjustTotal;
            } else {
                price = await calculatePrice(item);
                price +=
                    Number(item.adjust.adjustValueAdd || 0) -
                    Number(item.adjust.adjustValueSub || 0);
                item.mount = price > 0 ? price : 0;
            }
        } else {
            // 没有 `adjust` 的情况下计算价格
            price = await calculatePrice(item);
            item.mount = price > 0 ? price : 0;
        }

        // 过滤已取走的衣物
        if (item.paymentStatus == '00') {
            item.clothList = item.clothList.filter(cloth => cloth.clothingStatus !== '00');
        }
    }
    // console.log('ordersList', ordersList.value);
}

// 提取出价格计算逻辑
async function calculatePrice(item) {
    if (item.priceId) {
        const { data: priceItem } = await getPrice(item.priceId);
        return priceItem ? priceItem.priceValue : 0;
    } else {
        return item.clothList.reduce((acc, cur) => {
            let priceValue = cur.priceValue;
            if (cur.serviceRequirement === '001') {
                priceValue *= 2;
            } else if (cur.serviceRequirement === '002') {
                priceValue *= 1.5;
            }
            return acc + priceValue + cur.processMarkup;
        }, 0);
    }
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
    });
}
/** 搜索按钮操作 */
function handleQuery() {
    queryParams.value = {
        pickupCode: queryParams.value.pickupCode ? queryParams.value.pickupCode.trim() : null,
        orderNumber: queryParams.value.orderNumber ? queryParams.value.orderNumber.trim() : null,
        phonenumber: queryParams.value.phonenumber ? queryParams.value.phonenumber.trim() : null,
    };
    if (isEmpty(queryParams.value.pickupCode) &&
        isEmpty(queryParams.value.phonenumber) &&
        isEmpty(queryParams.value.orderNumber)) {
        ordersList.value = []
        return;
    }

    // check tel surfix
    if (!isEmpty(queryParams.value.phonenumber)) {
        if(queryParams.value.phonenumber.length < 4) {
            proxy.$modal.msgError('请输入正确的手机后四位,或完整的手机号');
            return;
        }
    }
    getList();
}

/** 重置按钮操作 */
function resetQuery() {
    proxy.resetForm("queryRef");
    handleQuery();
}

function isEmpty(value) {
    return value === null || value === undefined || value === '';
}

// 如果初始化时visible是true，则直接加载数据
onMounted(async () => {
    if (props.visible) {
        await initList();
        if (isEmpty(queryParams.value.pickupCode) &&
            isEmpty(queryParams.value.phonenumber) &&
            isEmpty(queryParams.value.orderNumber)) {
            return;
        }
        getList();
        showOrderDialog.value = true;
    }
});
</script>
<style scoped>
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

.search-result-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.result-item {
    display: flex;
    flex-direction: column;
    gap: .5rem;
}

.result-item-info {
    width: 100%;
    display: flex;
    justify-content: flex-start;
    align-items: center;
    gap: 3rem;
    padding: .5rem;
    border: 1px dashed;
    border-radius: .5rem;
    background-color: aquamarine;

    :last-child {
        display: flex;
        gap: .5rem;
    }
}

.address {
    display: flex;
    gap: 2rem;
}

.coupon-times {
    display: flex;
    flex-direction: column;
    gap: .5rem;

    .coupon-times-item {
        display: flex;
        gap: .5rem;
    }
}

.payment-amount {
    color: red;
    font-size: large;
    font-weight: bold;
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