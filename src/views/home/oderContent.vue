<template>
    <div class="result-container">
        <el-form :model="queryParams" class="top-bar" ref="queryRef" :inline="true" label-width="68px">
            <el-form-item label="取件码" prop="pickupCode" size="large">
                <el-input v-model="queryParams.pickupCode" placeholder="请输入取件码" clearable @keyup.enter="handleQuery"
                    size="large">
                    <template #prefix>
                        <el-icon>
                            <Ticket />
                        </el-icon>
                    </template>
                </el-input>
            </el-form-item>
            <el-form-item label="手机号" prop="phonenumber" size="large">
                <el-input ref="phonenumber" v-model="queryParams.phonenumber" placeholder="请输入会员手机号" clearable
                    @keyup.enter="handleQuery" size="large">
                    <template #prefix>
                        <el-icon>
                            <Phone />
                        </el-icon>
                    </template>
                </el-input>
            </el-form-item>
            <el-form-item label="订单编码" prop="orderNumber" size="large">
                <el-input v-model="queryParams.orderNumber" placeholder="请输入订单编码" clearable @keyup.enter="handleQuery"
                    size="large">
                    <template #prefix>
                        <el-icon>
                            <Document />
                        </el-icon>
                    </template>
                </el-input>
            </el-form-item>
            <el-form-item>
                <el-button type="primary" icon="Search" @click="handleQuery" size="large" round>搜索</el-button>
                <el-button icon="Refresh" @click="resetQuery" size="large" round>重置</el-button>
            </el-form-item>
        </el-form>
        <!-- 渲染订单抖索结果列表 -->
        <div class="search-result-list">
            <div v-if="ordersList.length === 0" class="no-result">
                <!-- <h1 style="color: #ccc;">暂无数据</h1> -->
                <el-empty description="暂无数据" />
            </div>
            <div v-else class="result-item" v-for="order in ordersList" :key="order.orderId">
                <div class="result-item-order-num">
                    <span>
                        订单编码: {{ order.orderNumber }}
                    </span>
                    <el-button type="primary" size="small">补打小票</el-button>
                </div>
                <div class="result-item-info">
                    <div class="info-item">
                        <el-icon>
                            <User />
                        </el-icon>
                        <span>会员身份: <strong>{{ order.nickName }}</strong> ({{ order.phonenumber }})</span>
                    </div>
                    <div class="info-item">
                        <el-icon>
                            <Wallet />
                        </el-icon>
                        <span>支付状态:</span>
                        <dict-tag v-if="order.paymentStatus === '01'" style="cursor: pointer;" @click="go2pay(order)"
                            :options="sys_payment_status" :value="order.paymentStatus" />
                        <dict-tag v-else :options="sys_payment_status" :value="order.paymentStatus" />
                    </div>
                    <div class="info-item">
                        <el-icon>
                            <Money />
                        </el-icon>
                        <span>{{ order.paymentStatus === '00' ? '实际支付金额:' : '应支付金额:' }}</span>
                        <span class="payment-amount">
                            {{ order.mount }}元
                        </span>
                    </div>
                    <div class="info-item">
                        <el-icon>
                            <Ticket />
                        </el-icon>
                        <span>取件码: <strong>{{ order.pickupCode }}</strong></span>
                    </div>
                    <div class="info-item">
                        <el-icon>
                            <InfoFilled />
                        </el-icon>
                        <span>订单状态:</span>
                        <dict-tag :options="sys_order_status" :value="order.status" />
                    </div>
                </div>
                <!-- 订单包含的衣物列表 -->
                <el-table v-if="order.clothList && order.clothList.length > 0" :data="order.clothList"
                    :loading="order.loading" row-key="clothingId"
                    @selection-change="selectedItems => handleClothSelectionChange(selectedItems, order)"
                    ref="clothsTableRef" class="modern-table" border="dash" stripe>
                    <el-table-column type="selection" width="50" align="center" />
                    <el-table-column label="衣物" align="center" min-width="120">
                        <template #default="scope">
                            <div class="cloth-name">
                                {{ scope.row.clothInfo.clothingName }}
                                <span v-if="scope.row.clothingColor" class="cloth-color">
                                    {{colorList.find(item => item.tagId == scope.row.clothingColor).tagName}}
                                </span>
                            </div>
                        </template>
                    </el-table-column>
                    <el-table-column label="衣物编码" align="center" prop="clothingColor" width="110">
                        <template #default="scope">
                            <span class="code-badge">{{ scope.row.hangClothCode }}</span>
                        </template>
                    </el-table-column>
                    <el-table-column label="服务类型" align="center" width="150">
                        <template #default="scope">
                            <span class="service-type">
                                <dict-tag :options="sys_service_type" :value="scope.row.serviceType" />
                                <el-divider direction="vertical" class="vertical-divider" />
                                <dict-tag :options="sys_service_requirement" :value="scope.row.serviceRequirement" />
                            </span>
                        </template>
                    </el-table-column>
                    <el-table-column label="洗护价格" align="center" prop="priceValue" width="90">
                        <template #default="scope">
                            <span class="price-value">¥{{ scope.row.priceValue }}</span>
                        </template>
                    </el-table-column>
                    <el-table-column label="工艺加价" align="center" prop="processMarkup" width="90">
                        <template #default="scope">
                            <span v-if="scope.row.processMarkup && scope.row.processMarkup > 0" class="markup-value">+{{
                                scope.row.processMarkup }}</span>
                            <span v-else>-</span>
                        </template>
                    </el-table-column>
                    <el-table-column label="衣物瑕疵" align="center" prop="clothingFlaw" min-width="120">
                        <template #default="scope">
                            <div class="tag-container">
                                <el-tag v-for="tagId in scope.row.clothingFlaw ? scope.row.clothingFlaw.split(',') : []"
                                    :key="tagId" type="danger" size="small" effect="light">
                                    {{flawList.find(item => item.tagId == tagId).tagName}}
                                </el-tag>
                                <span v-if="!scope.row.clothingFlaw">-</span>
                            </div>
                        </template>
                    </el-table-column>
                    <el-table-column label="洗后预估" align="center" prop="estimate" min-width="120">
                        <template #default="scope">
                            <div class="tag-container">
                                <el-tag v-for="tagId in scope.row.estimate ? scope.row.estimate.split(',') : []"
                                    :key="tagId" type="info" size="small" effect="light">
                                    {{estimateList.find(item => item.tagId == tagId).tagName}}
                                </el-tag>
                                <span v-if="!scope.row.estimate">-</span>
                            </div>
                        </template>
                    </el-table-column>
                    <el-table-column label="衣物品牌" align="center" prop="clothingBrand" width="100">
                        <template #default="scope">
                            <el-tag v-if="scope.row.clothingBrand" type="success" size="small" effect="light">
                                {{brandList.find(item => item.tagId == scope.row.clothingBrand).tagName}}
                            </el-tag>
                            <span v-else>-</span>
                        </template>
                    </el-table-column>
                    <el-table-column label="图片" align="center" width="120">
                        <template #default="scope">
                            <div class="image-buttons">
                                <el-button link type="primary" size="small"
                                    :disabled="scope.row.beforePics == null || scope.row.beforePics.length == 0"
                                    @click="handleShowPicture(scope.row, true)">
                                    <el-icon>
                                        <Picture />
                                    </el-icon> 洗前
                                </el-button>
                                <el-button link type="primary" size="small"
                                    :disabled="scope.row.afterPics == null || scope.row.afterPics.length == 0"
                                    @click="handleShowPicture(scope.row, false)">
                                    <el-icon>
                                        <Picture />
                                    </el-icon> 洗后
                                </el-button>
                            </div>
                        </template>
                    </el-table-column>
                    <el-table-column label="洗护状态" align="center" prop="clothingStatus" width="100">
                        <template #default="scope">
                            <dict-tag :options="sys_clothing_status" :value="scope.row.clothingStatus" />
                        </template>
                    </el-table-column>
                    <el-table-column label="上挂位置" align="center" width="120">
                        <template #default="scope">
                            <el-tag v-if="scope.row.hangLocationCode" type="info">{{ scope.row.hangerName + '-' +
                                scope.row.hangerNumber }}
                            </el-tag>
                            <span v-else>-</span>
                        </template>
                    </el-table-column>
                    <el-table-column label="操作" align="center" width="120" fixed="right">
                        <template #default="scope">
                            <div v-if="scope.row.clothingStatus == '02'" class="action-buttons">
                                <el-button type="primary" size="small" plain round @click="pickup(scope.row)">
                                    <el-icon>
                                        <TakeawayBox />
                                    </el-icon> 取衣
                                </el-button>
                                <el-button type="warning" size="small" plain round @click="handleReWash(scope.row)">
                                    <el-icon>
                                        <Refresh />
                                    </el-icon> 复洗
                                </el-button>
                            </div>
                            <span v-else>-</span>
                        </template>
                    </el-table-column>
                </el-table>

            </div>
        </div>

        <div class="footer">
            <el-button @click="props.taggle()" plain icon="Close" round>关闭</el-button>
            <el-button type="primary" @click="pickup()" icon="TakeawayBox" round>取衣</el-button>
            <el-button type="success" @click="handlePay" icon="Wallet" round>取衣收款</el-button>
            <el-button type="warning" @click="handleDelivery" icon="Van" round>上门派送</el-button>
            <el-button type="info" @click="() => { }" icon="Printer" round>补打小票</el-button>
        </div>
    </div>

    <!-- 展示照片 -->
    <el-dialog title="照片" v-model="showPicture" width="400px" append-to-body>
        <div class="img-container">
            <el-image class="img-item" show-progress :zoom-rate="1.2" :max-scale="7" :min-scale="0.2"
                :preview-src-list="pictureList" :src="item" v-for="(item, index) in pictureList" :key="index"
                fit="cover" />
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
    <el-dialog v-model="showPaymentDialog" width="600px" append-to-body lock-scroll modal :close-on-click-modal="false"
        :show-close="false" class="payment-dialog">
        <template #header>
            <div class="dialog-header">
                <div class="order-info">
                    <el-icon>
                        <Ticket />
                    </el-icon>
                    <span>订单信息</span>
                </div>
                <div class="order-number">{{ paymentForm.titles }}</div>
            </div>
        </template>

        <!-- 会员信息卡片 -->
        <div class="member-card" v-if="currentUser">
            <div class="member-avatar">
                <el-avatar :size="50" icon="UserFilled" />
            </div>
            <div class="member-details">
                <div class="member-name">{{ currentUser.nickName || currentUser.userName }}</div>
                <div class="member-phone">{{ currentUser.phonenumber }}</div>
            </div>
            <div class="member-points" v-if="currentUser.integral !== undefined">
                <div class="points-label">积分</div>
                <div class="points-value">{{ currentUser.integral }}</div>
            </div>
        </div>

        <el-form ref="paymentRef" :model="paymentForm" :rules="paymentRules" label-width="80px" class="payment-form">
            <!-- 支付方式选择 -->
            <div class="section-title">支付方式</div>
            <el-form-item class="payment-method-section">
                <el-radio-group v-model="paymentForm.paymentMethod" class="payment-method-group">
                    <template v-for="dict in sys_payment_method" :key="dict.value">
                        <template v-if="dict.value == '06'">
                            <el-radio v-if="couponTypeList && couponTypeList.has('000')" :value="dict.value"
                                class="payment-method-radio">
                                <div class="payment-method-card"
                                    :class="{ 'selected': paymentForm.paymentMethod === dict.value }">
                                    <el-icon>
                                        <CreditCard />
                                    </el-icon>
                                    <span>{{ dict.label }}</span>
                                </div>
                            </el-radio>
                        </template>
                        <template v-else-if="dict.value == '07'">
                            <el-radio v-if="couponTypeList && couponTypeList.has('002')" :value="dict.value"
                                class="payment-method-radio">
                                <div class="payment-method-card"
                                    :class="{ 'selected': paymentForm.paymentMethod === dict.value }">
                                    <el-icon>
                                        <Ticket />
                                    </el-icon>
                                    <span>{{ dict.label }}</span>
                                </div>
                            </el-radio>
                        </template>
                        <el-radio v-else-if="dict.value !== '03' && dict.value !== '04'" :value="dict.value"
                            class="payment-method-radio">
                            <div class="payment-method-card"
                                :class="{ 'selected': paymentForm.paymentMethod === dict.value }">
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

            <!-- 卡券选择区域 -->
            <template v-if="showCoupons">
                <div class="section-title">选择优惠</div>
                <div class="coupon-section">
                    <!-- 储值卡 -->
                    <el-collapse v-if="userCouponList.filter(item => item.coupon.couponType == '000').length !== 0">
                        <el-collapse-item title="储值卡" name="storage-card">
                            <el-checkbox-group v-model="couponStorageCardId" @change="changeCoupon(1)"
                                class="coupon-checkbox-group">
                                <el-checkbox
                                    v-for="card in userCouponList.filter(item => item.coupon.couponType == '000')"
                                    :disabled="!card.isValid" :key="card.ucId" :value="card.ucId"
                                    class="coupon-checkbox">
                                    <div class="coupon-card" :class="{ 'disabled': !card.isValid }">
                                        <div class="coupon-title">{{ card.coupon.couponTitle }}</div>
                                        <div class="coupon-value">余额: {{ card.availableValue }}元</div>
                                        <div v-if="!card.isValid" class="coupon-invalid">{{ card.unValidReason }}</div>
                                    </div>
                                </el-checkbox>
                            </el-checkbox-group>
                        </el-collapse-item>
                    </el-collapse>

                    <!-- 次卡 -->
                    <el-collapse v-if="userCouponList.filter(item => item.coupon.couponType == '002').length != 0">
                        <el-collapse-item title="次卡" name="time-card">
                            <div class="coupon-times">
                                <div class="coupon-times-item"
                                    v-for="card in userCouponList.filter(item => item.coupon.couponType == '002')"
                                    :key="card.ucId">
                                    <el-checkbox @change="changeCoupon(2, card)" :disabled="!card.isValid"
                                        v-model="card.selected" :value="card.ucId" class="coupon-checkbox">
                                        <div class="coupon-card" :class="{ 'disabled': !card.isValid }">
                                            <div class="coupon-title">{{ card.coupon.couponTitle }}</div>
                                            <div class="coupon-value">剩余: {{ card.availableValue }}次</div>
                                            <div v-if="!card.isValid" class="coupon-invalid">{{ card.unValidReason }}
                                            </div>
                                        </div>
                                    </el-checkbox>
                                    <el-input-number v-if="card.selected" v-model="card.count"
                                        @change="changeCouponCount(card)" :min="1" :max="card.availableValue"
                                        controls-position="right" size="small" class="count-input" />
                                </div>
                            </div>
                        </el-collapse-item>
                    </el-collapse>

                    <!-- 优惠券 -->
                    <el-collapse
                        v-if="userCouponList.filter(item => item.coupon.couponType !== '002' && item.coupon.couponType !== '000').length != 0">
                        <el-collapse-item title="优惠券" name="discount-coupon">
                            <el-radio-group v-model="paymentForm.couponId" @change="changeCoupon(3)"
                                class="coupon-radio-group">
                                <el-radio
                                    v-for="card in userCouponList.filter(item => item.coupon.couponType !== '000' && item.coupon.couponType !== '002')"
                                    :disabled="!card.isValid" :key="card.ucId" :value="card.ucId" class="coupon-radio">
                                    <div class="coupon-card"
                                        :class="{ 'disabled': !card.isValid, 'selected': paymentForm.couponId === card.ucId }">
                                        <div class="coupon-title">{{ card.coupon.couponTitle }}</div>
                                        <div class="coupon-value">剩余: {{ card.ucCount }}张</div>
                                        <div v-if="!card.isValid" class="coupon-invalid">{{ card.unValidReason }}</div>
                                    </div>
                                </el-radio>
                            </el-radio-group>
                        </el-collapse-item>
                    </el-collapse>
                </div>
            </template>

            <!-- 价格信息区域 -->
            <div class="price-summary-card">
                <div class="price-row">
                    <span class="price-label">订单金额</span>
                    <span class="price-value">¥ {{ paymentForm.totalAmount }}</span>
                </div>
                <div class="price-row" v-if="paymentForm.bonusAmount">
                    <span class="price-label">优惠金额</span>
                    <span class="price-value discount">- ¥ {{ paymentForm.bonusAmount }}</span>
                </div>
                <div class="price-divider"></div>
                <div class="price-row total">
                    <span class="price-label">应付金额</span>
                    <span class="price-value total-amount">¥ {{ paymentForm.paymentAmount }}</span>
                </div>
            </div>

            <!-- 补差价区域 -->
            <el-form-item label="补差价" v-if="paymentForm.priceDiff > 0" class="price-diff-section">
                <el-input-number v-model="paymentForm.priceDiff" controls-position="right" :min="0"
                    :max="paymentForm.paymentAmount" placeholder="请输入补差价" />
            </el-form-item>
        </el-form>

        <template #footer>
            <div class="payment-footer">
                <el-button size="large" @click="showPaymentDialog = false" plain>取消</el-button>
                <el-button size="large" type="primary" @click="submitPaymentForm(false)">确认收款</el-button>
                <el-button size="large" type="primary" @click="submitPaymentForm(true)">收款并取衣</el-button>
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
import { listCloths } from "@/api/system/cloths";
import { listTagsNoLimit } from "@/api/system/tags";
import { onMounted } from "vue";
import { delivery, pickUp } from "@/api/system/cloths";
import { listUserCouponWithValidTime } from '@/api/system/user_coupon';
import { getUser } from '@/api/system/user';
import { isCurrentTimeWithinRange } from "@/utils";
import { selectListExceptCompleted } from "@/api/system/orders";
import { getPrice } from "@/api/system/price";
import ReWash from "./rewash.vue";
import { ElMessageBox } from 'element-plus';
import { invoke } from '@tauri-apps/api/core';


const props = defineProps({
    taggle: {
        type: Function,
        required: true,
    }
});

const { proxy } = getCurrentInstance();
const {
    sys_payment_status,
    sys_order_status,
    sys_payment_method,
    sys_service_requirement,
    sys_service_type,
    sys_clothing_status
} =
    proxy.useDict(
        'sys_payment_status',
        "sys_order_status",
        "sys_payment_method",
        "sys_service_requirement",
        "sys_service_type",
        "sys_clothing_status",
    );


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
// 选中的储值卡列表
const couponStorageCardId = ref([]);

const clothsTableRef = ref();
// 用户卡券列表
const userCouponList = ref([]);
// 用户卡券种类列表
const couponTypeList = ref();

const showPicture = ref(false);
const showDeliveryDialog = ref(false);
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

const phonenumber = ref();

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
    await listUserCouponWithValidTime(row.userId).then(response => {
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
function handleReWash(cloth) {
    if (cloth) {
        selectedCloths.value = [cloth];
    }
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
}

async function handlePay() {
    if (ordersList.value.length == 0) {
        proxy.notify.warning("没有可支付的订单或可取走的衣物");
        return;
    }
    initPaymentForm();
    // 获取用户的卡券列表
    await listUserCouponWithValidTime(ordersList.value[0].userId).then(response => {
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
        proxy.notify.warning("没有选中未支付的订单");
        return;
    }

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
    try {
        // 等待支付完成
        await pay(paymentForm.value);

        // 支付成功后提示
        proxy.notify.success('支付成功');

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
async function pickup(cloth) {
    if (cloth) {
        selectedCloths.value = [cloth];
    }
    console.log(selectedCloths.value)
    const cloths = selectedCloths.value.filter(item => item.clothingStatus !== '00');
    if (cloths.length == 0) {
        proxy.notify.warning("没有选中符合条件的衣物");
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
    console.log(ids)

    // 判断是否包含未支付的订单
    const unpaidOrders = ordersList.value.filter(item => orderIds.includes(item.orderId) && item.paymentStatus !== '00');
    if (unpaidOrders.length > 0) {
        // 弹出询问是否确认取走
        proxy.$modal.confirm("当前选中衣物有未支付订单，是否确认取走？").then(async () => {
            await pickUp(ids).then(res => {
                proxy.notify.success("取走成功");
                selectedCloths.value = [];
                getList();
            }).catch(err => {
                proxy.notify.error(err.message);
            })
        }).catch(() => { })
        return;
    }
    await pickUp(ids).then(res => {
        proxy.notify.success("取走成功");
        selectedCloths.value = [];
        getList();
    }).catch(err => {
        proxy.notify.error(err.message);
    })
}

function handleDelivery() {
    const ids = selectedCloths.value.filter(item => item.clothingStatus === '02').map(item => item.clothId);
    if (ids.length == 0) {
        proxy.notify.warning("没有选中符合条件的衣物");
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
        proxy.notify.success("操作成功");
        getList();
    })
}

// 衣物列表多选框选中数据
function handleClothSelectionChange(selectedItems, row) {
    // 清空当前订单下的选中数据
    const orderId = row.orderId;
    selectedCloths.value = selectedCloths.value.filter(cloth => cloth.orderClothId !== orderId);

    // 将新的选中项合并到 shared array
    selectedCloths.value.push(...selectedItems);
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

    if (queryParams.value.pickupCode === '' && queryParams.value.phonenumber === '' && queryParams.value.orderNumber === '') {
        return;
    }
    loading.value = true;
    ordersList.value = await selectListExceptCompleted(queryParams.value);

    if (ordersList.value.length === 0) {
        proxy.notify.warning('没有找到相关订单');
        return;
    }

    // 查询用户信息
    getUser(ordersList.value[0].userId).then(res => {
        currentUser.value = res;
    });

    // 获取用户卡券列表
    await listUserCouponWithValidTime(ordersList.value[0].userId).then(res => {
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
const loadImage = async (id) => {
    try {
        // 调用 Tauri 后端命令获取图片二进制数据
        const imageData = await invoke('get_image', { id });

        // 将二进制数据转换为 Blob
        const blob = new Blob([new Uint8Array(imageData)], { type: 'image/png' });

        // 生成图片 URL
        return URL.createObjectURL(blob);

        // 提示加载成功
    } catch (error) {
        // 提示加载失败
    }
};
/* 获取图片列表id */
async function handleShowPicture(row, flag) {
    showPicture.value = true;
    try {
        // 获取图片 ID 列表
        const picIds = flag ? row.beforePics?.split(',') : row.afterPics?.split(',');

        if (picIds && picIds.length > 0) {
            // 使用 Promise.all 等待所有图片加载完成
            const imageUrls = await Promise.all(picIds.map(id => loadImage(Number(id))));

            // 过滤掉加载失败的图片（null）
            pictureList.value = imageUrls.filter(url => url !== null);
        } else {
            pictureList.value = []; // 如果没有图片 ID，清空列表
        }
    } catch (error) {
        console.error(`处理图片列表失败: ${error}`);
        pictureList.value = []; // 出错时清空列表
    }
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
        if (queryParams.value.phonenumber.length < 4) {
            proxy.notify.error('请输入正确的手机后四位,或完整的手机号');
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
    phonenumber.value.focus();
    await initList();
    if (isEmpty(queryParams.value.pickupCode) &&
        isEmpty(queryParams.value.phonenumber) &&
        isEmpty(queryParams.value.orderNumber)) {
        return;
    }
    getList();
});
</script>
<style scoped>
.result-container {
    height: 100%;
    width: 100%;
    position: relative;
    padding-top: 3.5rem;
    background-color: var(--el-bg-color-page);
}

.top-bar {
    position: absolute;
    top: 1.7rem;
    left: 1rem;
    right: 1rem;
    padding: 0.75rem;
    background-color: var(--el-bg-color);
    border-radius: 0.5rem;
    box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.05);
    z-index: 10;

    .el-form-item {
        margin-bottom: 0 !important;
    }
}

.footer {
    position: fixed;
    padding: 0.75rem 1rem;
    bottom: 0.75rem;
    right: 1rem;
    z-index: 999;
    background-color: var(--el-bg-color);
    border-radius: 0.5rem;
    box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
    display: flex;
    gap: 0.75rem;
    transition: all 0.3s;

    button:hover {
        transform: translateY(-2px);
    }
}

.payment-status {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.img-container {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
    gap: 1rem;
    padding: 0.5rem;
}

.img-item {
    border-radius: 0.5rem;
    overflow: hidden;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    transition: transform 0.2s;
}

.img-item:hover {
    transform: scale(1.05);
}

.payment-footer {
    text-align: center;
    padding: 1rem 0;
}

.search-result-list {
    width: 100%;
    height: calc(100% - 3rem);
    overflow-y: overlay;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
    margin-top: 3rem;
    padding: 0rem 1rem 5rem 1rem;
    scrollbar-gutter: stable;
}

/* 取消滚动条显示，因为tauri中无论如何设置都会挤压内部元素 */
.search-result-list::-webkit-scrollbar {
    width: 0;
}

.no-result {
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
}

.result-item {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    border: none;
    border-radius: 0.75rem;
    padding: 1rem;
    background-color: var(--el-bg-color);
    box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.05);
    transition: transform 0.2s, box-shadow 0.2s;
}

.result-item:hover {
    box-shadow: 0 4px 16px 0 rgba(0, 0, 0, 0.1);
}

.result-item-order-num {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem;
    border-radius: .5rem;
    color: #409EFF;
    font-weight: 500;
    font-size: 1.1rem;
    background-color: var(--el-color-primary-light-9);
}

:root.dark .result-item-order-num {
    --el-color-primary-light-9: #1d2c40;
}

.result-item-info {
    width: 100%;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    font-size: 0.9rem;
    color: var(--el-text-color-regular);
    padding: 0.5rem 0;

    :last-child {
        display: flex;
        gap: .5rem;
    }
}

.service-type {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    gap: .5rem;
    /* background-color: #f8f9fa; */
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;

    .vertical-divider::after {
        content: "" !important;
    }
}

.address {
    display: flex;
    gap: 2rem;
    align-items: center;
}

.coupon-times {
    display: flex;
    flex-direction: column;
    gap: .75rem;
    padding: 0.5rem;
    background-color: #f8f9fa;
    border-radius: 0.5rem;

    .coupon-times-item {
        display: flex;
        gap: .75rem;
        align-items: center;
    }
}

.payment-amount {
    color: #F56C6C;
    font-size: 1.2rem;
    font-weight: bold;
}

.row-class-name {
    background-color: rgba(64, 158, 255, 0.1) !important;
    cursor: pointer;
}

.modern-table {
    --el-table-border-color: transparent;
    /* --el-table-header-bg-color: #f5f7fa; */
    /* --el-table-row-hover-bg-color: #f5f7fa; */
    border-radius: 0.5rem;
    overflow: hidden;
    margin-top: 0.5rem;
    box-shadow: var(--el-box-shadow-lighter);
}

.modern-table :deep(th) {
    background-color: var(--el-fill-color-light);
    color: var(--el-text-color-primary);
    font-weight: 600;
}

.cloth-name {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

.cloth-color {
    font-size: 0.8rem;
    color: #909399;
}

.code-badge {
    color: #67c23a;
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
}

.price-value {
    font-weight: 500;
    color: #606266;
}

.markup-value {
    color: #F56C6C;
    font-weight: 500;
}

.tag-container {
    display: flex;
    flex-wrap: wrap;
    gap: 0.25rem;
    justify-content: center;
}

.location-badge {
    background-color: #f4f4f5;
    color: #909399;
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
}

.image-buttons {
    display: flex;
    flex-direction: column;
    gap: 8px;
    width: 100%;

    .el-button {
        margin-left: 0 !important;
    }
}

.action-buttons {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 0.5rem;

    .el-button {
        margin-left: 0 !important;
    }
}

.info-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    border-radius: 0.25rem;
    transition: background-color 0.2s;
}

.info-item:hover {
    background-color: var(--el-fill-color-light);
}

.info-item .el-icon {
    color: #909399;
}

/* 支付弹窗样式 */
.payment-dialog :deep(.el-dialog__header) {
    padding: 16px;
    margin-right: 0;
    border-bottom: 1px solid var(--el-border-color-lighter);
}

.dialog-header {
    display: flex;
    flex-direction: column;
    gap: 8px;
}

.order-info {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 16px;
    font-weight: 600;
    color: var(--el-color-primary);
}

.order-number {
    font-size: 14px;
    color: var(--el-text-color-secondary);
    margin-left: 24px;
}

.member-card {
    display: flex;
    align-items: center;
    background: linear-gradient(135deg, var(--el-fill-color-light) 0%, var(--el-fill-color-dark) 100%);
    border-radius: 12px;
    padding: 16px;
    margin: 16px 0 24px 0;
    box-shadow: var(--el-box-shadow-light);
    transition: transform 0.3s ease;
}

.member-card:hover {
    transform: translateY(-2px);
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
    border-left: 1px solid var(--el-border-color-lighter);
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

.payment-form {
    padding: 0 16px;
}

.section-title {
    font-size: 16px;
    font-weight: 600;
    margin: 16px 0 12px 0;
    color: var(--el-text-color-primary);
    position: relative;
    padding-left: 12px;
}

.section-title::before {
    content: '';
    position: absolute;
    left: 0;
    top: 50%;
    transform: translateY(-50%);
    width: 4px;
    height: 16px;
    background-color: var(--el-color-primary);
    border-radius: 2px;
}

.payment-method-section {
    margin-bottom: 24px;
}

.payment-method-group {
    display: flex;
    flex-wrap: wrap;
    gap: 16px;
    margin-bottom: 10px;
}

.payment-method-radio {
    margin-right: 0 !important;
    margin-bottom: 10px;
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
    background-color: var(--el-color-primary-light-9);
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

.coupon-section {
    margin-bottom: 24px;
}

.coupon-checkbox-group,
.coupon-radio-group {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
}

.coupon-checkbox,
.coupon-radio {
    margin-right: 0 !important;
    margin-bottom: 12px;
}

.coupon-card {
    width: 200px;
    padding: 12px;
    border-radius: 8px;
    border: 1px solid var(--el-border-color-lighter);
    background: linear-gradient(135deg, #ffffff 0%, #f5f7fa 100%);
    transition: all 0.3s;
}

.coupon-card:hover {
    border-color: var(--el-color-primary);
    transform: translateY(-2px);
    box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
}

.coupon-card.selected {
    border-color: var(--el-color-primary);
    background: linear-gradient(135deg, var(--el-color-primary-light-9) 0%, #f5f7fa 100%);
    box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
}

.coupon-card.disabled {
    opacity: 0.6;
    background: #f5f7fa;
}

.coupon-title {
    font-size: 14px;
    font-weight: 600;
    margin-bottom: 8px;
    color: var(--el-text-color-primary);
}

.coupon-value {
    font-size: 14px;
    color: var(--el-text-color-secondary);
}

.coupon-invalid {
    font-size: 12px;
    color: #f56c6c;
    margin-top: 4px;
}

.coupon-times {
    display: flex;
    flex-direction: column;
    gap: 12px;
}

.coupon-times-item {
    display: flex;
    align-items: center;
    gap: 12px;
}

.count-input {
    width: 120px;
}

.price-summary-card {
    background-color: var(--el-fill-color-light);
    border-radius: 8px;
    padding: 16px;
    margin: 24px 0;
    box-shadow: var(--el-box-shadow-light);
    transition: transform 0.3s ease;
}

.price-summary-card:hover {
    transform: translateY(-2px);
    box-shadow: var(--el-box-shadow);
}

.price-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
}

.price-row.total {
    margin-top: 12px;
    margin-bottom: 0;
}

.price-label {
    font-size: 14px;
    color: var(--el-text-color-secondary);
}

.price-value {
    font-size: 16px;
    font-weight: 600;
    color: var(--el-text-color-primary);
}

.price-value.discount {
    color: #f56c6c;
}

.price-value.total-amount {
    font-size: 24px;
    color: #f56c6c;
}

.price-divider {
    height: 1px;
    background-color: var(--el-border-color-lighter);
    margin: 12px 0;
}

.price-diff-section {
    margin-top: 16px;
}

.payment-footer {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
}

.payment-footer button {
    transition: all 0.3s;
}

.payment-footer button:hover {
    transform: translateY(-2px);
}

/* 添加一些动画效果 */
.el-dialog__body {
    transition: all 0.3s;
}

.el-collapse-item__header {
    font-weight: 600;
    color: var(--el-text-color-primary);
}

/* 响应式调整 */
@media (max-width: 768px) {
    .payment-method-group {
        justify-content: center;
    }

    .coupon-checkbox-group,
    .coupon-radio-group {
        justify-content: center;
    }
}
</style>
