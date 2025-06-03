<template>
    <!-- 付款弹窗 -->
    <el-dialog v-model="showPaymentDialog" width="600px" append-to-body lock-scroll modal :align-center="true"
        :close-on-click-modal="false" :show-close="false" @closed="close" class="payment-dialog">
        <!-- 扫码支付弹窗 -->
        <el-dialog v-model="showScannerDialog" title="扫描二维码" width="500px" lock-scroll modal :align-center="true"
            :close-on-click-modal="false" :show-close="true" class="scanner-dialog">
            <QrCodeScanner :onScanSuccess="handleScanResult" :autoStart="true" @scan-error="handleScanError"
                @scan-timeout="handleScanTimeout" />
        </el-dialog>
        <template #header>
            <div class="dialog-header">
                <div class="order-info">
                    <el-icon>
                        <Ticket />
                    </el-icon>
                    <span v-if="paymentForm.titles">{{ paymentForm.titles }}</span>
                    <span v-else>{{ paymentForm.payNumber ? `订单 - ${paymentForm.payNumber}` : '订单支付' }}</span>
                </div>
                <el-button type="primary" size="small" @click="showCouponSale = true">
                    <el-icon>
                        <Plus />
                    </el-icon>购买卡券
                </el-button>
            </div>
        </template>

        <!-- 会员信息卡片 -->
        <div class="member-card hover-flow">
            <div class="member-avatar">
                <el-avatar :size="50" icon="UserFilled" />
            </div>
            <div class="member-details">
                <div class="member-name">{{ currentUser ? (currentUser.nickName || currentUser.userName) : user.nickName
                }}
                </div>
                <div class="member-phone">{{ currentUser ? currentUser.phonenumber : user.phonenumber }}</div>
            </div>
            <div class="member-points">
                <div class="points-label">积分</div>
                <div class="points-value">{{ currentUser ? currentUser.integral : user.integral }}</div>
            </div>
        </div>

        <el-form ref="paymentRef" :model="paymentForm" :rules="paymentRules" label-width="80px" class="payment-form">
            <!-- 支付方式选择 -->
            <div class="section-title">支付方式</div>
            <el-form-item class="payment-method-section">
                <template v-if="isExternalOrder">
                    <div class="payment-method-card selected">
                        <el-icon>
                            <Promotion />
                        </el-icon>
                        <span>{{ externalSourceLabel }}</span>
                    </div>
                </template>
                <template v-else>
                    <el-radio-group v-model="paymentForm.paymentMethod" class="payment-method-group"
                        @change="handlePaymentMethodChange">
                        <template v-for="dict in PaymentMethod" :key="dict.value">
                            <template v-if="dict.value == 'StoredValueCard'">
                                <el-radio v-if="couponTypeList.has('StoredValueCard')" :value="dict.value"
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
                            <template v-else-if="dict.value == 'SessionCard'">
                                <el-radio v-if="couponTypeList.has('SessionCard')" :value="dict.value"
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
                            <template v-else-if="dict.value == 'DiscountCard'">
                                <el-radio v-if="couponTypeList.has('DiscountCard')" :value="dict.value"
                                    class="payment-method-radio">
                                    <div class="payment-method-card"
                                        :class="{ 'selected': paymentForm.paymentMethod === dict.value }">
                                        <el-icon>
                                            <Discount />
                                        </el-icon>
                                        <span>{{ dict.label }}</span>
                                    </div>
                                </el-radio>
                            </template>
                            <el-radio v-else-if="dict.value !== 'Meituan' && dict.value !== 'Douyin'"
                                :value="dict.value" class="payment-method-radio">
                                <div class="payment-method-card"
                                    :class="{ 'selected': paymentForm.paymentMethod === dict.value }">
                                    <el-icon v-if="dict.value === 'Alipay'">
                                        <Money />
                                    </el-icon>
                                    <el-icon v-else-if="dict.value === 'WechatPay'">
                                        <ChatDotRound />
                                    </el-icon>
                                    <el-icon v-else-if="dict.value === 'Cash'">
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

                    <!-- 扫码支付按钮 -->
                    <el-button type="primary" @click="startScanPayment" class="scan-code-btn"
                        v-if="paymentForm.paymentMethod === 'Alipay' || paymentForm.paymentMethod === 'WechatPay'">
                        <el-icon>
                            <ScanningFilled />
                        </el-icon>
                        扫描付款码
                    </el-button>
                </template>
            </el-form-item>

            <!-- 卡券选择区域 -->
            <template v-if="showCoupons">
                <div class="section-title">选择优惠</div>
                <div class="coupon-section">
                    <!-- 当没有任何卡券时显示提示信息 -->
                    <div v-if="userCouponList.length === 0" class="no-coupons-tip">
                        <el-empty description="该用户名下没有任何卡券" :image-size="50" :style="{ padding: '1rem' }">
                            <el-button type="primary" size="small" plain @click="showCouponSale = true">
                                <el-icon>
                                    <Plus />
                                </el-icon>购买卡券
                            </el-button>
                        </el-empty>
                    </div>

                    <!-- 当有卡券但没有有效卡券时显示提示 -->
                    <div v-else-if="userCouponList.filter(item => item.isValid).length === 0" class="no-coupons-tip">
                        <el-empty description="该用户名下没有可用的卡券" :image-size="50" :style="{ padding: '1rem' }">
                            <el-button type="primary" size="small" plain @click="showCouponSale = true">
                                <el-icon>
                                    <Plus />
                                </el-icon>购买卡券
                            </el-button>
                        </el-empty>
                    </div>

                    <!-- 储值卡、次卡、优惠券 -->
                    <el-collapse v-model="activeCollapseItem" accordion>
                        <!-- 储值卡 -->
                        <el-collapse-item name="storage-card"
                            v-if="userCouponList.length > 0 && userCouponList.filter(item => item.isValid).length > 0 && userCouponList.filter(item => item.coupon.couponType == 'StoredValueCard').length !== 0">
                            <template #title>
                                <span>储值卡</span>
                                <span class="coupon-count-badge">
                                    {{userCouponList.filter(item => item.coupon.couponType == 'StoredValueCard' &&
                                        item.isValid).length}}
                                </span>
                            </template>
                            <el-checkbox-group v-model="couponStorageCardId" @change="changeCoupon(1)"
                                class="coupon-checkbox-group storage-card-group">
                                <el-checkbox
                                    v-for="card in userCouponList.filter(item => item.coupon.couponType == 'StoredValueCard')"
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

                        <!-- 折扣卡 -->
                        <el-collapse-item name="discount-card"
                            v-if="userCouponList.length > 0 && userCouponList.filter(item => item.isValid).length > 0 && userCouponList.filter(item => item.coupon.couponType == 'DiscountCard').length !== 0">
                            <template #title>
                                <span>折扣卡</span>
                                <span class="coupon-count-badge">
                                    {{userCouponList.filter(item => item.coupon.couponType == 'DiscountCard' &&
                                        item.isValid).length}}
                                </span>
                            </template>
                            <el-checkbox-group v-model="couponDiscountCardId"
                                @change="(val) => handleDiscountCardChange(val)"
                                class="coupon-checkbox-group storage-card-group">
                                <el-checkbox
                                    v-for="card in userCouponList.filter(item => item.coupon.couponType == 'DiscountCard')"
                                    :disabled="!card.isValid" :key="card.ucId" :value="card.ucId"
                                    class="coupon-checkbox">
                                    <div class="coupon-card" :class="{ 'disabled': !card.isValid }">
                                        <div class="coupon-title">{{ card.coupon.couponTitle }}</div>
                                        <div class="coupon-value">
                                            余额: {{ card.availableValue }}元
                                            ({{ (card.coupon.usageValue / 10).toFixed(1) }}折)
                                        </div>
                                        <div v-if="!card.isValid" class="coupon-invalid">{{ card.unValidReason }}</div>
                                    </div>
                                </el-checkbox>
                            </el-checkbox-group>
                        </el-collapse-item>

                        <!-- 次卡 -->
                        <el-collapse-item name="time-card"
                            v-if="userCouponList.length > 0 && userCouponList.filter(item => item.isValid).length > 0 && userCouponList.filter(item => item.coupon.couponType == 'SessionCard').length != 0">
                            <template #title>
                                <span>次卡</span>
                                <span class="coupon-count-badge">{{userCouponList.filter(item => item.coupon.couponType
                                    == 'SessionCard' && item.isValid).length}}</span>
                            </template>
                            <div class="coupon-times">
                                <div class="coupon-times-item" v-for="card in groupedTimeCards"
                                    :key="card.coupon.couponId">
                                    <el-checkbox @change="changeCoupon(2, card)" :disabled="!card.isValid"
                                        v-model="card.selected" :label="card.ucId" class="coupon-checkbox">
                                        <div class="coupon-card" :class="{ 'disabled': !card.isValid }">
                                            <div class="coupon-title">{{ card.coupon.couponTitle }}</div>
                                            <div class="coupon-value">剩余: {{ card.totalAvailableValue }}次</div>
                                            <!-- <div v-if="card.groupCount > 1" class="coupon-group-info">
                                                已合并 {{ card.groupCount }} 张同类卡
                                            </div> -->
                                            <div v-if="!card.isValid" class="coupon-invalid">{{ card.unValidReason }}
                                            </div>
                                        </div>
                                    </el-checkbox>
                                    <!-- <el-input-number v-if="card.selected" v-model="card.count"
                                        @change="changeCouponCount(card)" :min="1" :max="card.totalAvailableValue"
                                        controls-position="right" class="count-input" /> -->
                                </div>
                            </div>
                        </el-collapse-item>

                        <!-- 优惠券 -->
                        <el-collapse-item name="discount-coupon"
                            v-if="userCouponList.length > 0 && userCouponList.filter(item => item.isValid).length > 0 && userCouponList.filter(item => item.coupon.couponType !== 'SessionCard' && item.coupon.couponType !== 'StoredValueCard').length != 0">
                            <template #title>
                                <span>优惠券</span>
                                <span class="coupon-count-badge">{{userCouponList.filter(item => item.coupon.couponType
                                    !== 'StoredValueCard' && item.coupon.couponType !== 'SessionCard' &&
                                    item.isValid).length}}</span>
                            </template>
                            <el-radio-group v-model="paymentForm.couponId" @change="changeCoupon(3)"
                                class="coupon-radio-group storage-card-group">
                                <el-radio
                                    v-for="card in userCouponList.filter(item => item.coupon.couponType !== 'StoredValueCard' && item.coupon.couponType !== 'SessionCard')"
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
                    <span class="price-value">¥ {{ getOrderAmount() }}</span>
                </div>
                <!-- 店主调价信息 -->
                <template v-if="isOrderWithAdjustment">
                    <div class="price-row adjust-price-row" v-if="props.order.adjust.adjustTotal">
                        <span class="price-label">店主调价</span>
                        <span class="price-value adjust">总价调整为 ¥ {{ (Math.floor(props.order.adjust.adjustTotal * 100) /
                            100).toFixed(2) }}</span>
                    </div>
                    <div class="price-row adjust-price-row" v-else>
                        <span class="price-label">店主调价</span>
                        <div class="adjust-details">
                            <span v-if="props.order.adjust.adjustValueAdd" class="price-value adjust-add">
                                + ¥{{ (Math.floor(props.order.adjust.adjustValueAdd * 100) / 100).toFixed(2) }}
                            </span>
                            <span v-if="props.order.adjust.adjustValueSub" class="price-value adjust-sub">
                                - ¥{{ (Math.floor(props.order.adjust.adjustValueSub * 100) / 100).toFixed(2) }}
                            </span>
                        </div>
                    </div>
                    <div class="price-row adjust-price-row" v-if="props.order.adjust.remark">
                        <span class="price-label">调价备注</span>
                        <span class="price-value adjust-remark">{{ props.order.adjust.remark }}</span>
                    </div>
                </template>
                <div class="price-row" v-if="paymentForm.bonusAmount">
                    <span class="price-label">优惠金额</span>
                    <span class="price-value discount">
                        - ¥ {{ (Math.floor(paymentForm.bonusAmount * 100) / 100).toFixed(2) }}
                    </span>
                </div>
                <div class="price-divider"></div>
                <div class="price-row total">
                    <span class="price-label">应付金额</span>
                    <span class="price-value total-amount">
                        ¥ {{ (Math.floor(paymentForm.paymentAmount * 100) / 100).toFixed(2) }}
                    </span>
                </div>
            </div>

            <!-- 补差价区域 -->
            <div v-if="paymentForm.priceDiff > 0" class="price-diff-card">
                <div class="price-diff-content">
                    <div class="price-diff-main">
                        <span class="price-diff-title">补差价</span>
                        <div class="price-diff-input">
                            <span class="price-diff-symbol-left">¥</span>
                            <el-input-number v-model="paymentForm.priceDiff" controls-position="right" :min="0"
                                :max="paymentForm.paymentAmount" placeholder="请输入补差价" class="price-diff-number" />
                        </div>
                    </div>
                    <span class="price-diff-desc">使用储值卡或次卡时可能需要补差价</span>
                </div>
            </div>
        </el-form>

        <template #footer>
            <div class="payment-footer">
                <el-button type="danger" @click="close">取消</el-button>
                <template v-if="showPickupButton">
                    <el-button type="primary" color="#626aef" @click="submitPaymentForm(false)">确认收款</el-button>
                    <el-button type="success" @click="submitPaymentForm(true)">收款并取衣</el-button>
                </template>
                <template v-else>
                    <el-button type="primary" @click="submitPaymentForm(false)">确认支付</el-button>
                </template>
            </div>
        </template>
    </el-dialog>

    <!-- 卡券购买弹窗 -->
    <el-dialog v-model="showCouponSale" width="800px" lock-scroll modal align-center :close-on-click-modal="false"
        :show-close="false">
        <CouponSale :userId="userId" :key="showCouponSale" :taggle="() => { showCouponSale = !showCouponSale }"
            :visible="showCouponSale" :couponTypeList="couponTypeList" :submit="submitCouponSale" />
    </el-dialog>
</template>

<script setup name="Pay">
import { pay } from "@/api/system/orders";
import CouponSale from './couponSale.vue';
import { listUserCouponWithValidTime } from '@/api/system/user_coupon';
import { getUser } from '@/api/system/user';
import { isCurrentTimeWithinRange } from "@/utils";
import { getPrice } from "@/api/system/price";
import QrCodeScanner from '@/components/QrCodeScanner/index.vue';
import { ElLoading } from 'element-plus';
import { PaymentMethod } from "@/constants";

const props = defineProps({
    visible: {
        type: Boolean,
        required: true,
        default: false,
    },
    order: {
        type: Object,
        required: false,
        default: () => ({}),
    },
    orders: {
        type: Array,
        required: false,
        default: () => [],
    },
    clothsList: {
        type: Array,
        required: false,
        default: () => [],
    },
    user: {
        type: Object,
        required: false,
        default: () => ({}),
    },
    refresh: {
        type: Function,
        required: true,
    },
    toggle: {
        type: Function,
        required: true,
    },
    createOrder: {
        type: Function,
    },
    userCouponList: {
        type: Array,
        required: false,
        default: () => [],
    },
    couponTypeList: {
        type: Set,
        required: false,
        default: () => new Set(),
    },
    showPickupButton: {
        type: Boolean,
        required: false,
        default: false
    }
});

const emit = defineEmits([
    'payment-success',
    'payment-failed',
    'payment-cancel',
    'submit',
    'pickup',
    'success'
]);

const { proxy } = getCurrentInstance();

console.log(props);
const showPaymentDialog = ref(false);
const showCoupons = ref(true);
const showScannerDialog = ref(false);

// 添加一个标志来跟踪是否已经手动选择了储值卡
const hasManualStorageCardSelection = ref(false);
const hasManualDiscountCardSelection = ref(false);

const paymentForm = ref({
    // 订单信息
    orders: [],
    titles: '',

    // 支付方式和金额
    paymentMethod: 'Alipay',
    totalAmount: 0,
    paymentAmount: 0,
    bonusAmount: 0,
    priceDiff: 0,

    // 卡券相关
    ucId: '',  // 用于储值卡、折扣卡等单选卡券
    timeBased: [], // 用于次卡的使用记录

    // 扫码支付相关
    authCode: '',
    subject: '',
    paymentType: '',

    // 其他
    orderType: 'Laundry'
});
const couponStorageCardId = ref([]);
const couponDiscountCardId = ref([]);
const user = ref({});
const currentUser = ref(null);
const showCouponSale = ref(false);
const activeCollapseItem = ref(['storage-card']); // Track expanded collapse items
const userCouponList = ref(props.userCouponList);
const couponTypeList = ref(props.couponTypeList);
// Computed properties to handle different component scenarios
const userId = computed(() => {
    return props.user && props.user.userId ? props.user.userId :
        props.order && props.order.userId ? props.order.userId : null;
});

// Group time cards by couponId and validity
const groupedTimeCards = computed(() => {
    // 获取所有次卡但不再合并，直接返回每张卡的独立信息
    const timeCards = userCouponList.value.filter(item => item.coupon.couponType === 'SessionCard');

    // 为每张卡添加必要的属性
    return timeCards.map(card => ({
        ...card,
        groupCount: 1, // 固定为1表示不再合并
        totalAvailableValue: card.availableValue,
        totalUcCount: card.ucCount,
        items: [card] // 只包含自己
    }));
});

const isExternalOrder = computed(() => {
    return props.order && (props.order.source === 'Meituan' || props.order.source === 'Douyin');
});

const externalSourceLabel = computed(() => {
    if (props.order && props.order.source === 'Meituan') return '美团结转';
    if (props.order && props.order.source === 'Douyin') return '抖音结转';
    return '';
});

const isOrderWithAdjustment = computed(() => {
    return props.order && props.order.adjust &&
        (props.order.adjust.adjustValueAdd ||
            props.order.adjust.adjustValueSub ||
            props.order.adjust.adjustTotal);
});

function close() {
    emit('payment-cancel');
    props.toggle();
    // 重置手动选择标志
    hasManualStorageCardSelection.value = false;
    hasManualDiscountCardSelection.value = false;
}

// Helper function to get order amount based on available data
function getOrderAmount() {
    // If using PaymentDialog-style multi-order mode
    if (props.orders && props.orders.length > 0) {
        return props.orders.reduce((acc, cur) => acc + cur.mount, 0).toFixed(2);
    }

    // If using pay.vue-style single order mode
    if (props.order && props.order.originalPrice) {
        return (Math.floor(props.order.originalPrice * 100) / 100).toFixed(2);
    }

    // Fallback to total amount in payment form
    return (Math.floor(paymentForm.value.totalAmount * 100) / 100).toFixed(2);
}

function submitCouponSale() {
    listUserCouponWithValidTime(userId.value).then(response => {
        userCouponList.value = response;
        userCouponList.value.filter(item => item.coupon.couponType == 'SessionCard').map(item => {
            item.selected = false;
            item.count = 1;
        });
        couponTypeList.value = new Set(userCouponList.value.map(coupon => coupon.coupon.couponType));
        checkCoupon();
    });
    showCouponSale.value = false;
}

/* 启动扫码支付 */
function startScanPayment() {
    showScannerDialog.value = true;
}

/* 处理扫码错误 */
function handleScanError(error) {
    console.error('扫码错误:', error);
    proxy.notify.error('扫码出错: ' + error);
}

/* 处理扫码超时 */
function handleScanTimeout() {
    proxy.notify.warning('扫码超时，请重试');
}

/**
 * 处理扫码结果
 * @param {string} result - 扫码结果
 */
function handleScanResult(result) {
    console.log('扫码结果:', result);
    showScannerDialog.value = false;

    // 显示全局loading
    const loadingInstance = ElLoading.service({
        lock: true,
        text: '正在处理支付...',
        background: 'rgba(0, 0, 0, 0.7)'
    });

    try {
        // 验证并设置扫码支付数据
        if (setupScanPaymentData(result)) {
            // 自动提交支付表单
            submitPaymentForm().finally(() => {
                loadingInstance.close();
            });
        } else {
            proxy.notify.warning('无法识别的付款码格式');
            loadingInstance.close();
        }
    } catch (error) {
        console.error('处理支付出错:', error);
        proxy.notify.error('处理支付出错');
        loadingInstance.close();
    }
}

/**
 * 设置扫码支付数据
 * @param {string} authCode - 授权码
 * @returns {boolean} 是否设置成功
 */
function setupScanPaymentData(authCode) {
    // 支付宝付款码通常以28开头
    if (authCode.startsWith('28')) {
        paymentForm.value.paymentMethod = 'Alipay';
        paymentForm.value.paymentType = 'alipay';
        paymentForm.value.authCode = authCode;
        paymentForm.value.subject = paymentForm.value.titles || '洗衣服务';
        proxy.notify.success('已识别为支付宝付款码');
        return true;
    }
    // 微信付款码通常以10-15开头
    else if (/^1[0-5]/.test(authCode)) {
        paymentForm.value.paymentMethod = 'WeChatPay';
        paymentForm.value.paymentType = 'wechat';
        paymentForm.value.authCode = authCode;
        paymentForm.value.subject = paymentForm.value.titles || '洗衣服务';
        proxy.notify.success('已识别为微信付款码');
        return true;
    }

    return false;
}

// 初始化表单数据
async function initForm() {
    const targetUserId = userId.value;
    if (!targetUserId) return;

    // 获取用户信息
    await getUser(targetUserId).then(res => {
        if (props.user && props.user.userId) {
            currentUser.value = props.user;
        } else {
            user.value = res;
            currentUser.value = null;
        }
    });

    // 初始化用户卡券列表
    if (props.userCouponList && props.userCouponList.length > 0) {
        userCouponList.value = props.userCouponList;
        // 确保次卡的selected属性被正确初始化为false
        userCouponList.value.filter(item => item.coupon.couponType === 'SessionCard').forEach(item => {
            item.selected = false;
            item.count = 1;
        });
    } else {
        // 获取用户卡券列表
        await listUserCouponWithValidTime(targetUserId).then(response => {
            userCouponList.value = response;
            // 确保次卡的selected属性被正确初始化为false
            userCouponList.value.filter(item => item.coupon.couponType === 'SessionCard').forEach(item => {
                item.selected = false;
                item.count = 1;
            });
            couponTypeList.value = new Set(userCouponList.value.map(coupon => coupon.coupon.couponType));
        });
    }

    // 处理订单信息
    if (props.orders && props.orders.length > 0) {
        // 多订单模式 (来自PaymentDialog组件)
        initMultiOrderForm();
    } else if (props.order && Object.keys(props.order).length > 0) {
        // 单订单模式 (来自pay.vue组件)
        initSingleOrderForm();
    } else {
        // 默认初始化
        paymentForm.value = {
            orders: [],
            paymentMethod: 'Alipay',
            orderType: 'Laundry',
            priceDiff: 0,
            totalAmount: 0,
            paymentAmount: 0,
            bonusAmount: 0,
            titles: ''
        };
    }

    checkCoupon();
}

// 初始化单个订单的表单
function initSingleOrderForm() {
    paymentForm.value = {
        // 订单信息
        orders: [props.order],
        ucOrderId: props.order.orderId,
        payNumber: props.order.orderNumber,
        titles: props.order.orderNumber || '',

        // 支付方式和金额
        paymentMethod: 'Alipay',
        totalAmount: 0,
        paymentAmount: 0,
        bonusAmount: 0,
        priceDiff: 0,

        // 卡券相关
        ucId: '',
        timeBased: [],

        // 扫码支付相关
        authCode: '',
        subject: '',
        paymentType: '',

        // 其他
        orderType: 'Laundry'
    };

    if (props.order.source == 'Meituan') {
        paymentForm.value.paymentMethod = 'Meituan';
        showCoupons.value = false;
    } else if (props.order.source == 'Douyin') {
        paymentForm.value.paymentMethod = 'Douyin';
        showCoupons.value = false;
    }

    // 计算原价和总价
    let price = 0;

    // 如果没有传入originalPrice，需要计算原价
    if (!props.order.originalPrice) {
        // 计算衣物原价（不包含调价）
        let originalPrice = 0;

        // 如果选择了价格方案，那么使用所有选中价格方案的总和
        if (props.order.priceIds && props.order.priceIds.length > 0) {
            // 这种情况下，原价就是价格方案的总和，但我们没有价格方案的详细信息
            // 所以如果有totalPrice，我们就用totalPrice作为原价
            originalPrice = props.order.totalPrice || 0;
        } else {
            // 计算衣物的原始价格总和
            originalPrice = props.order.cloths.reduce((acc, cur) => {
                let priceValue = cur.priceValue;
                if (cur.serviceRequirement == 'Emergency') {
                    priceValue *= 2;
                } else if (cur.serviceRequirement == 'SingleWash') {
                    priceValue *= 1.5;
                }
                return acc + priceValue + cur.processMarkup
            }, 0);
        }

        // 设置计算出的原价
        props.order.originalPrice = originalPrice > 0 ? originalPrice : 0;
    }

    // 使用传递过来的总价，这已经包含了调价
    if (props.order.totalPrice !== undefined && props.order.totalPrice > 0) {
        price = props.order.totalPrice;
    }
    // 如果选择了价格方案，以前是根据priceId计算的，但现在应该考虑priceIds数组
    else if (props.order.priceIds && props.order.priceIds.length > 0) {
        if (props.order.isDiscount) {
            price = props.order.originalPrice || 0;
        } else {
            price = props.order.totalPrice || 0;
        }
    }
    // 使用了单一价格方案的遗留代码
    else if (props.order.priceId) {
        getPrice(props.order.priceId).then(item => {
            price = item ? item.priceValue : 0;
            paymentForm.value.totalAmount = price > 0 ? price : 0;
            paymentForm.value.paymentAmount = paymentForm.value.totalAmount;
        });
    } else if (props.order.adjust.adjustTotal && props.order.adjust.adjustTotal > 0) {
        // 如果存在店主调价总价格，那么使用总价格
        price = props.order.adjust.adjustTotal;
    } else {
        // 如果没有传递总价，则计算价格
        price = props.order.cloths.reduce((acc, cur) => {
            // 计算总价
            // 如果服务要求为加急
            let priceValue = cur.priceValue;
            if (cur.serviceRequirement == 'Emergency') {
                priceValue *= 2;
            } else if (cur.serviceRequirement == 'SingleWash') {
                priceValue *= 1.5;
            }
            return acc +
                priceValue + cur.processMarkup
        }, 0);
        price +=
            Number(props.order.adjust.adjustValueAdd ? props.order.adjust.adjustValueAdd : 0) -
            Number(props.order.adjust.adjustValueSub ? props.order.adjust.adjustValueSub : 0);
    }

    paymentForm.value.totalAmount = price > 0 ? Math.floor(price * 100) / 100 : 0;
    paymentForm.value.paymentAmount = paymentForm.value.totalAmount;
    couponStorageCardId.value = [];
    checkCoupon();
}

/**
 * 提交支付表单
 * @param {boolean} isPickup - 是否需要取衣
 * @returns {Promise<void>}
 */
async function submitPaymentForm(isPickup) {
    try {
        // 验证支付方式和选择
        if (!validatePaymentMethod()) {
            return;
        }

        // 处理金额格式化
        normalizeAmounts();

        // 处理支付方式和优惠券
        processPaymentMethodAndCoupons();

        // 处理订单数据
        await prepareOrderData();

        // 处理特殊来源的订单价格
        handleSpecialSourcePricing();

        // 构造支付请求数据
        const paymentRequest = buildPaymentRequest();

        // 执行支付
        await pay(paymentRequest);

        // 支付成功后的处理
        handlePaymentSuccess(isPickup);
    } catch (error) {
        handlePaymentError(error);
    }
}

/**
 * 验证支付方式是否有效
 * @returns {boolean} 验证是否通过
 */
function validatePaymentMethod() {
    const { paymentMethod } = paymentForm.value;

    // 储值卡支付验证
    if (paymentMethod === 'StoredValueCard' && couponStorageCardId.value.length === 0) {
        proxy.notify.error('您选择了储值卡支付方式，但未选择任何储值卡');
        return false;
    }

    // 次卡支付验证
    if (paymentMethod === 'SessionCard' && !groupedTimeCards.value.some(card => card.selected)) {
        proxy.notify.error('您选择了次卡支付方式，但未选择任何次卡');
        return false;
    }

    // 折扣卡支付验证
    if (paymentMethod === 'DiscountCard' && couponDiscountCardId.value.length === 0) {
        proxy.notify.error('您选择了折扣卡支付方式，但未选择任何折扣卡');
        return false;
    }

    return true;
}

/**
 * 标准化所有金额（截断到小数点后两位）
 */
function normalizeAmounts() {
    const amountFields = ['totalAmount', 'paymentAmount', 'bonusAmount', 'priceDiff'];

    amountFields.forEach(field => {
        if (paymentForm.value[field] !== undefined) {
            paymentForm.value[field] = Math.floor(paymentForm.value[field] * 100) / 100;
        }
    });
}

/**
 * 处理支付方式和优惠券
 */
function processPaymentMethodAndCoupons() {
    // 没有使用优惠券的情况
    if (!paymentForm.value.couponId) {
        processWithoutCoupon();
    } else {
        processWithCoupon();
    }
}

/**
 * 处理没有使用优惠券的支付情况
 */
function processWithoutCoupon() {
    // 使用储值卡
    if (couponStorageCardId.value.length > 0) {
        processStoredValueCardPayment();
    }
    // 使用次卡
    else if (hasSelectedSessionCards()) {
        processSessionCardPayment();
    }
    // 没有使用任何卡券
    else {
        paymentForm.value.ucId = null;
        paymentForm.value.paymentAmountMv = paymentForm.value.paymentAmount;
    }
}

/**
 * 检查是否有选中的次卡
 * @returns {boolean}
 */
function hasSelectedSessionCards() {
    return userCouponList.value.filter(item =>
        item.coupon.couponType === 'SessionCard' && item.selected
    ).length > 0;
}

/**
 * 处理储值卡支付
 */
function processStoredValueCardPayment() {
    // 计算使用了多少储值卡
    paymentForm.value.paymentAmountVip = paymentForm.value.paymentAmount - paymentForm.value.priceDiff;
    paymentForm.value.ucId = couponStorageCardId.value.join(',');
    // 使用了储值卡，实际从其他支付方式中扣除的金额为差价
    paymentForm.value.paymentAmountMv = paymentForm.value.priceDiff;

    // 需要补充差价时设置组合支付方式
    if (paymentForm.value.priceDiff > 0) {
        paymentForm.value.paymentMethod = getComboPaymentMethod(paymentForm.value.paymentMethod, 'StoredValueCard');
    }
}

/**
 * 处理次卡支付
 */
function processSessionCardPayment() {
    // 获取选中的次卡列表
    paymentForm.value.timeBased = getSelectedTimeCardsList();

    // 需要补充差价时设置组合支付方式
    if (paymentForm.value.priceDiff > 0) {
        paymentForm.value.paymentMethod = getComboPaymentMethod(paymentForm.value.paymentMethod, 'SessionCard');
    }
}

/**
 * 获取选中的次卡列表
 * @returns {Array} 选中的次卡列表
 */
function getSelectedTimeCardsList() {
    let list = [];

    // 从分组的次卡中获取选中的次卡数据
    groupedTimeCards.value.filter(card => card.selected).forEach(groupCard => {
        // 计算每张卡应分配的次数
        const individualCount = groupCard.count ? Math.ceil(groupCard.count / groupCard.items.length) : 1;

        // 添加组内每张卡
        groupCard.items.forEach(item => {
            list.push({
                ucId: item.ucId,
                count: individualCount
            });
        });
    });

    // 兼容旧方法
    if (list.length === 0) {
        list = userCouponList.value
            .filter(item => item.coupon.couponType === 'SessionCard' && item.selected)
            .map(item => ({
                ucId: item.ucId,
                count: item.count || 1,
            }));
    }

    return list;
}

/**
 * 处理使用优惠券的支付情况
 */
function processWithCoupon() {
    const coupon = userCouponList.value.find(item => item.ucId == paymentForm.value.couponId);

    if (coupon) {
        // 根据优惠券类型设置组合支付方式
        if (coupon.coupon.couponType === 'DiscountCoupon') {
            paymentForm.value.paymentMethod = getComboPaymentMethod(paymentForm.value.paymentMethod, 'DiscountCoupon');
        } else if (coupon.coupon.couponType === 'SpendAndSaveCard') {
            paymentForm.value.paymentMethod = getComboPaymentMethod(paymentForm.value.paymentMethod, 'SpendAndSaveCard');
        }
    }

    paymentForm.value.ucId = String(paymentForm.value.couponId);
    // 用了优惠券，实际从其他支付方式中扣除的金额为优惠后的总金额
    paymentForm.value.paymentAmountMv = paymentForm.value.paymentAmount;
}

/**
 * 获取组合支付方式
 * @param {string} baseMethod - 基础支付方式
 * @param {string} comboType - 组合类型
 * @returns {string} 组合支付方式
 */
function getComboPaymentMethod(baseMethod, comboType) {
    const methodMap = {
        'Alipay': `AlipayAnd${comboType}`,
        'WeChatPay': `WeChatPayAnd${comboType}`,
        'Cash': `CashAnd${comboType}`
    };

    return methodMap[baseMethod] || baseMethod;
}

/**
 * 准备订单数据
 */
async function prepareOrderData() {
    paymentForm.value.totalAmount = Number(paymentForm.value.totalAmount);

    // 多订单模式
    if (props.orders && props.orders.length > 0) {
        paymentForm.value.orders = props.orders;
    }
    // 单订单模式
    else {
        paymentForm.value.orders = [props.order];

        // 如果需要创建订单
        if (props.createOrder) {
            await createOrderIfNeeded();
        }
    }
}

/**
 * 如果需要，创建订单
 */
async function createOrderIfNeeded() {
    const callback = (res) => {
        if (res && res.orderId) {
            paymentForm.value.orders = [res];
        }
    };
    await props.createOrder(callback);
}

/**
  * 处理特殊来源订单的价格
  */
function handleSpecialSourcePricing() {
    // 处理美团或抖音的价格标签
    if (props.order.source === 'Meituan' || props.order.source === 'Douyin') {
        paymentForm.value.totalAmount = props.order.originalPrice;
        paymentForm.value.paymentAmount = props.order.originalPrice;
    }
}

/**
 * 构造支付请求数据，匹配Rust后端PaymentReq结构
 * @returns {Object} 支付请求对象
 */
function buildPaymentRequest() {
    // 构造Payment对象
    const payment = {
        payNumber: null,
        ucOrderId: null,
        orderType: paymentForm.value.orderType,
        totalAmount: paymentForm.value.totalAmount,
        paymentStatus: null,
        paymentMethod: paymentForm.value.paymentMethod,
        createTime: null,
        updateTime: null,
        storeId: null,
        refundReason: null,
        paymentMethodDetails: [],
        couponUsages: []
    };

    // 构造PaymentReq对象
    const paymentRequest = {
        payment: payment,
        ucIds: paymentForm.value.ucId ? paymentForm.value.ucId.split(',').map(id => parseInt(id)) : null,
        orders: paymentForm.value.orders,
        timeBased: paymentForm.value.timeBased && paymentForm.value.timeBased.length > 0 ? paymentForm.value.timeBased : null,
        authCode: paymentForm.value.authCode || null,
        storeId: null, // 将由后端设置
        subject: paymentForm.value.subject || null,
        paymentType: paymentForm.value.paymentType || null
    };

    return paymentRequest;
}

/**
 * 处理支付成功
 * @param {boolean} isPickup - 是否需要取衣
 */
function handlePaymentSuccess(isPickup) {
    // 支付成功后提示
    proxy.notify.success('支付成功');
    showPaymentDialog.value = false;

    // 更新订单状态
    paymentForm.value.orders.forEach(item => item.paymentStatus = 'Paid');

    // 关闭对话框并刷新数据
    props.toggle();
    props.refresh();

    // 发送事件
    emitSuccessEvents(isPickup);
}

/**
 * 发送成功相关事件
 * @param {boolean} isPickup - 是否需要取衣
 */
function emitSuccessEvents(isPickup) {
    // 发送支付成功事件
    emit('payment-success', {
        paymentMethod: paymentForm.value.paymentMethod,
        amount: paymentForm.value.paymentAmount
    });

    // 发送成功事件
    emit('success', { isPickup });

    // 根据是否取衣发送相应事件
    if (isPickup) {
        emit('pickup');
    } else {
        emit('submit');
    }
}

/**
 * 处理支付错误
 * @param {Error} error - 错误对象
 */
function handlePaymentError(error) {
    // 发送支付失败事件
    emit('payment-failed', error.message || '支付失败');
    console.error('支付失败:', error);
    proxy.notify.error('支付失败');
}

function changeCoupon(couponType, card) {
    if (couponType == 1) {
        // 标记用户已经手动选择了储值卡
        couponDiscountCardId.value = [];
        hasManualStorageCardSelection.value = true;
        hasManualDiscountCardSelection.value = false;

        paymentForm.value.couponId = null;
        paymentForm.value.bonusAmount = 0;
        // 清空次卡选择列表
        userCouponList.value.filter(item => item.coupon.couponType === "SessionCard").map(item => item.selected = false);
        // 同时更新分组后的次卡选中状态
        groupedTimeCards.value.forEach(groupCard => {
            groupCard.selected = false;
        });
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
            paymentForm.value.paymentMethod = 'Alipay';
        } else {
            paymentForm.value.priceDiff = 0;
            paymentForm.value.paymentMethod = 'StoredValueCard';
        }
    } else if (couponType == 2) {
        // 次卡
        // 清空储值卡选择列表
        couponDiscountCardId.value = [];
        couponStorageCardId.value = [];
        // 重置手动选择标志
        hasManualStorageCardSelection.value = false;
        hasManualDiscountCardSelection.value = false;
        paymentForm.value.couponId = null;

        // 当选择或取消选择分组的次卡时，同步更新所有属于该组的次卡
        if (card.items) {
            // 这是一个分组的卡，需要更新组内所有卡的选中状态
            card.items.forEach(item => {
                item.selected = card.selected;
            });
        }

        // 计算默认数量
        let count = 0;
        if (card.selected) {
            // 计算选中的次卡数量（处理分组卡）
            count = groupedTimeCards.value.filter(item => item.selected).reduce((acc, item) => {
                if (item.coupon.couponId !== card.coupon.couponId) {
                    acc += item.count || 0;
                }
                return acc;
            }, 0);

            // 计算输入框的数量
            card.count = props.order?.cloths?.length > count ?
                props.order.cloths.length - count > card.totalAvailableValue ?
                    card.totalAvailableValue : props.order.cloths.length - count : props.order.cloths.length || 1;

            // 需要再加上card.count
            count += card.count;
        } else {
            // 当取消选择时，计算其他选中的次卡数量
            count = groupedTimeCards.value.filter(item => item.selected).reduce((acc, item) => {
                if (item.coupon.couponId !== card.coupon.couponId) {
                    acc += item.count || 0;
                }
                return acc;
            }, 0);
        }

        if (count == 0) {
            paymentForm.value.paymentMethod = 'Alipay';
            paymentForm.value.bonusAmount = 0;
            paymentForm.value.priceDiff = 0;
        } else {
            // 需要补充差价
            const clothsList = props.clothsList && props.clothsList.length > 0 ? props.clothsList :
                (props.order && props.order.cloths ? props.order.cloths : []);

            if (clothsList && clothsList.length > 0) {
                const diffCount = clothsList.length - count;
                // 获取diffCount数量的衣物
                const diffCloths = diffCount > 0 ? clothsList.slice(0, diffCount) : [];
                // 计算差价
                let priceDiff = diffCloths.reduce((acc, cloth) => acc + (cloth.priceValue || 0), 0);
                paymentForm.value.priceDiff = Math.floor(priceDiff * 100) / 100;
                let bonusAmount = paymentForm.value.totalAmount - paymentForm.value.priceDiff;
                paymentForm.value.bonusAmount = Math.floor(bonusAmount * 100) / 100;

                // 如果次卡数量足够支付所有衣物，则使用次卡支付方式
                if (diffCount <= 0) {
                    paymentForm.value.paymentMethod = 'SessionCard';
                } else {
                    // 否则使用支付宝+次卡的组合支付
                    paymentForm.value.paymentMethod = 'AlipayAndSessionCard';
                }
            } else {
                // 如果没有衣物列表，使用订单金额
                paymentForm.value.priceDiff = 0;
                paymentForm.value.bonusAmount = paymentForm.value.totalAmount;
                paymentForm.value.paymentMethod = 'SessionCard'; // 使用次卡支付方式
            }

            // 确保支付总金额正确更新
            paymentForm.value.paymentAmount = paymentForm.value.totalAmount - paymentForm.value.bonusAmount;
        }
    } else if (couponType == 3) {
        //计算优惠金额
        couponDiscountCardId.value = [];
        couponStorageCardId.value = [];
        // 重置手动选择标志
        hasManualStorageCardSelection.value = false;
        hasManualDiscountCardSelection.value = false;
        userCouponList.value.filter(item => item.coupon.couponType === "SessionCard").map(item => item.selected = false)
        const coupon = userCouponList.value.find(item => item.ucId == paymentForm.value.couponId);

        // 满减券
        if (coupon.coupon.couponType == 'SpendAndSaveCard') {
            paymentForm.value.bonusAmount = coupon.coupon.usageValue;
            paymentForm.value.paymentMethod = 'Alipay';
        }
        // 折扣券
        if (coupon.coupon.couponType == 'DiscountCoupon') {
            // 使用整数计算避免浮点数精度问题
            let bonusAmount = (paymentForm.value.totalAmount * (100 - coupon.coupon.usageValue)) / 100;

            // 使用截断而非四舍五入
            bonusAmount = Math.floor(bonusAmount * 100) / 100;

            if (coupon.coupon.usageLimit != 0 && bonusAmount > coupon.coupon.usageLimit) {
                bonusAmount = coupon.coupon.usageLimit;
            }
            paymentForm.value.bonusAmount = bonusAmount;
            // 动态修改支付方式
            paymentForm.value.paymentMethod = 'Alipay';
        }

    } else if (couponType == 4) {
        // 标记用户手动操作折扣卡
        hasManualDiscountCardSelection.value = true;

        // 清理其他支付方式
        couponStorageCardId.value = [];
        hasManualStorageCardSelection.value = false;
        paymentForm.value.couponId = null;
        userCouponList.value.filter(item => item.coupon.couponType === "SessionCard").forEach(item => item.selected = false);
        groupedTimeCards.value.forEach(groupCard => groupCard.selected = false);

        // 核心优化逻辑开始
        // 1. 获取当前选中卡列表
        const currentSelectedCards = userCouponList.value.filter(item =>
            couponDiscountCardId.value.includes(item.ucId)
        );

        // 2. 自动过滤不同折扣率卡片（新增逻辑）
        if (currentSelectedCards.length > 0) {
            // 获取最新选中卡的折扣率（最后一张选中的卡）
            const lastSelectedCard = currentSelectedCards[currentSelectedCards.length - 1];
            const currentDiscountRate = lastSelectedCard.coupon.usageValue;

            // 只保留相同折扣率的卡
            couponDiscountCardId.value = couponDiscountCardId.value.filter(id => {
                const card = userCouponList.value.find(c => c.ucId === id);
                return card?.coupon.usageValue === currentDiscountRate;
            });
        }

        // 3. 重新计算选中卡信息
        let discountCardBalance = 0;
        let discountRate = 0;
        let selectedRate = null;

        userCouponList.value.forEach(item => {
            if (couponDiscountCardId.value.includes(item.ucId)) {
                // 校验折扣率一致性
                if (selectedRate === null) {
                    selectedRate = item.coupon.usageValue;
                } else if (selectedRate !== item.coupon.usageValue) {
                    console.error('折扣率不一致，自动清空选择');
                    couponDiscountCardId.value = [];
                    return;
                }

                discountCardBalance += item.availableValue;
                discountRate = selectedRate / 100; // 转换为小数（如80 → 0.8）
            }
        });

        // 4. 计算支付金额
        if (discountCardBalance === 0) {
            paymentForm.value.priceDiff = 0;
            paymentForm.value.bonusAmount = 0;
            paymentForm.value.paymentMethod = 'Alipay';
        } else {
            const discountedAmount = paymentForm.value.totalAmount * discountRate;

            // 分情况处理
            if (discountCardBalance >= discountedAmount) {
                paymentForm.value.priceDiff = 0;
                paymentForm.value.bonusAmount = paymentForm.value.totalAmount - discountedAmount;
                paymentForm.value.paymentMethod = 'DiscountCard'; // 折扣卡全额支付
            } else {
                const remainingDiscountedAmount = discountedAmount - discountCardBalance;
                const originalPriceDiff = remainingDiscountedAmount / discountRate;

                paymentForm.value.priceDiff = Math.floor(originalPriceDiff * 100) / 100;
                paymentForm.value.bonusAmount = paymentForm.value.totalAmount - discountedAmount;
                paymentForm.value.paymentMethod = 'Alipay'; // 需要补差价
            }
        }

        // 5. 同步UI状态
        userCouponList.value.forEach(item => {
            item.selected = couponDiscountCardId.value.includes(item.ucId);
        });
    }
    paymentForm.value.paymentAmount = paymentForm.value.totalAmount - (paymentForm.value.bonusAmount ? paymentForm.value.bonusAmount : 0);

    // 使用截断而非四舍五入
    paymentForm.value.paymentAmount = Math.floor(paymentForm.value.paymentAmount * 100) / 100;
    console.log(paymentForm.value);
}

// 处理选中逻辑
const handleDiscountCardChange = (selectedIds) => {
    const lastSelectedId = selectedIds[selectedIds.length - 1];
    const lastSelectedCard = userCouponList.value.find(c => c.ucId === lastSelectedId);

    if (lastSelectedCard) {
        // 自动过滤不同折扣率的卡片
        const currentRate = lastSelectedCard.coupon.usageValue;
        couponDiscountCardId.value = selectedIds.filter(id => {
            const card = userCouponList.value.find(c => c.ucId === id);
            return card?.coupon.usageValue === currentRate;
        });
    }

    changeCoupon(4);
};
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

            // 判断最低消费金额 (Minimum spend for order type 'SpendAndSaveCard')
            if (item.coupon.couponType == 'SpendAndSaveCard' && item.coupon.minSpend > order.totalPrice) {
                orderValid = false;
                item.unValidReason = "最低消费金额不足";
            }

            // 判断订单类型'003'的金额限制 (Order type '003')
            if (item.coupon.couponType == 'DiscountCoupon') {
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

    // 根据有效卡券数量自动展开对应面板
    setDefaultActivePanel();
}

// 根据有效卡券数量自动展开对应面板
function setDefaultActivePanel() {
    // 检查各类卡券的有效数量
    const validStorageCards = userCouponList.value.filter(
        item => item.coupon.couponType === 'StoredValueCard' && item.isValid
    ).length;

    const validTimeCards = userCouponList.value.filter(
        item => item.coupon.couponType === 'SessionCard' && item.isValid
    ).length;

    const validDiscountCoupons = userCouponList.value.filter(
        item => item.coupon.couponType !== 'StoredValueCard' &&
            item.coupon.couponType !== 'SessionCard' &&
            item.isValid
    ).length;

    const validDiscountCard = userCouponList.value.filter(item => item.coupon.couponType === 'DiscountCard' && item.isValid).length;

    // 按优先级自动展开面板
    if (validStorageCards > 0) {
        activeCollapseItem.value = ['storage-card'];
    } else if (validTimeCards > 0) {
        activeCollapseItem.value = ['time-card'];
    } else if (validDiscountCoupons > 0) {
        activeCollapseItem.value = ['discount-coupon'];
    } else if (validDiscountCard > 0) {
        activeCollapseItem.value = ['discount-card'];
    } else {
        activeCollapseItem.value = []; // 如果没有有效卡券，不展开任何面板
    }
}

// 智能选择储值卡的函数
function autoSelectStorageCards(isManualTrigger = false) {
    // 如果用户已经手动选择了储值卡 且 不是手动触发的, 则不进行自动选择
    if (hasManualStorageCardSelection.value && !isManualTrigger) return;

    // 如果是手动触发的，则保留用户当前的选择，不重新自动选择卡
    if (isManualTrigger) {
        // 仅设置支付方式，计算价格，但不更改卡片选择
        changeCoupon(1);
        return;
    }

    // 获取所有有效的储值卡
    const validStorageCards = userCouponList.value.filter(
        item => item.coupon.couponType === 'StoredValueCard' && item.isValid
    );

    // 如果没有有效储值卡，直接返回
    if (validStorageCards.length === 0) return;

    // 订单总金额
    const totalAmount = paymentForm.value.totalAmount;

    // 根据余额从小到大排序储值卡
    const sortedCards = [...validStorageCards].sort((a, b) => a.availableValue - b.availableValue);

    // 总可用余额
    const totalAvailableBalance = sortedCards.reduce((sum, card) => sum + card.availableValue, 0);

    // 如果总可用余额不足以支付订单金额，选择所有储值卡
    if (totalAvailableBalance < totalAmount) {
        couponStorageCardId.value = sortedCards.map(card => card.ucId);
    } else {
        // 尝试找到能覆盖订单金额的卡（从小到大选择）
        let selectedCards = [];
        let selectedAmount = 0;

        // 贪心算法：先选小额的卡
        for (const card of sortedCards) {
            if (selectedAmount >= totalAmount) break;

            selectedCards.push(card.ucId);
            selectedAmount += card.availableValue;
        }

        couponStorageCardId.value = selectedCards;
    }

    // 仅当自动选择卡片时，不标记为手动选择
    // 先把标志设为false，然后再触发changeCoupon计算
    hasManualStorageCardSelection.value = false;
    // 触发计算逻辑
    changeCoupon(1);
}
function autoSelectDiscountCards(isManualTrigger = false) {
    // 如果用户已手动选择折扣卡且非手动触发，则不自动选择
    if (hasManualDiscountCardSelection.value && !isManualTrigger) {
        console.debug('[折扣卡] 已有手动选择，跳过自动选择');
        return;
    }

    // 手动触发时保留当前选择
    if (isManualTrigger) {
        console.debug('[折扣卡] 手动触发，仅更新计算');
        changeCoupon(4);
        return;
    }

    // 获取有效折扣卡（类型DiscountCard），并过滤出usageValue有效的卡
    const validDiscountCards = userCouponList.value.filter(item => {
        const isValidType = item.coupon.couponType === 'DiscountCard';
        const hasValidValue = !isNaN(item.coupon.usageValue) && item.coupon.usageValue > 0;
        return isValidType && item.isValid && hasValidValue;
    });

    console.debug('[折扣卡] 有效卡列表:',
        validDiscountCards.map(c => ({
            id: c.ucId,
            rate: c.coupon.usageValue + '%',
            balance: c.availableValue
        }))
    );

    // 无有效卡时清空选择
    if (validDiscountCards.length === 0) {
        console.debug('[折扣卡] 无有效卡，清空选择');
        couponDiscountCardId.value = [];
        hasManualDiscountCardSelection.value = false;
        changeCoupon(4);
        return;
    }

    // 订单金额处理
    const totalAmount = Number(paymentForm.value.totalAmount) || 0;
    console.debug(`[折扣卡] 原始订单金额: ${totalAmount}`);
    if (totalAmount <= 0) {
        console.warn('[折扣卡] 无效订单金额:', totalAmount);
        couponDiscountCardId.value = [];
        return;
    }

    // --- 核心选择逻辑 ---
    // 步骤1：按折扣率分组（key为整数折扣值，如80）
    const discountGroups = validDiscountCards.reduce((groups, card) => {
        const discountRate = parseInt(card.coupon.usageValue); // 获取整数折扣值
        if (isNaN(discountRate)) return groups;

        // 转换为小数折扣率（80 => 0.8）
        const decimalRate = discountRate / 100;

        if (!groups[discountRate]) {
            groups[discountRate] = {
                decimalRate,       // 0.8
                displayRate: `${discountRate}%`, // "80%"
                cards: [],
                totalBalance: 0,
                discountedAmount: totalAmount * decimalRate // 折扣后应付金额
            };
        }

        groups[discountRate].cards.push(card);
        groups[discountRate].totalBalance += card.availableValue;
        return groups;
    }, {});

    console.debug('[折扣卡] 折扣率分组结果:',
        Object.values(discountGroups).map(g => ({
            折扣率: g.displayRate,
            卡数量: g.cards.length,
            总余额: g.totalBalance,
            折扣后应付: g.discountedAmount
        }))
    );

    // 步骤2：寻找最优折扣方案
    let bestGroup = null;
    Object.values(discountGroups).forEach(group => {
        // 计算实际需要支付的金额
        const remainingPayment = Math.max(group.discountedAmount - group.totalBalance, 0);

        // 当前方案的评估指标
        const currentBest = {
            discountRate: group.decimalRate,
            canCover: remainingPayment === 0,
            remainingPayment,
            requiredCards: []
        };

        // 在当前折扣组内选择卡片（从小到大）
        const sortedCards = [...group.cards].sort((a, b) => a.availableValue - b.availableValue);
        let accumulated = 0;

        for (const card of sortedCards) {
            if (accumulated >= group.discountedAmount) break;
            currentBest.requiredCards.push(card.ucId);
            accumulated += card.availableValue;
        }
        currentBest.usedAmount = accumulated;

        console.debug(`[折扣卡] 折扣率 ${group.displayRate} 方案评估:`, {
            需支付: currentBest.remainingPayment,
            用卡数: currentBest.requiredCards.length,
            用卡IDs: currentBest.requiredCards
        });

        // 选择策略（优先级排序）：
        // 1. 优先选能全额覆盖的最低折扣方案
        // 2. 次选剩余支付金额最少的最低折扣方案
        if (!bestGroup) {
            bestGroup = currentBest;
        } else if (
            (currentBest.canCover && currentBest.discountRate < bestGroup.discountRate) ||
            (!currentBest.canCover && currentBest.remainingPayment < bestGroup.remainingPayment) ||
            (currentBest.remainingPayment === bestGroup.remainingPayment &&
                currentBest.discountRate < bestGroup.discountRate)
        ) {
            bestGroup = currentBest;
        }
    });

    // 步骤3：应用选择结果
    if (bestGroup) {
        console.debug('[折扣卡] 最终选择方案:', {
            折扣率: `${bestGroup.discountRate * 100}%`,
            用卡IDs: bestGroup.requiredCards,
            使用金额: bestGroup.usedAmount,
            剩余支付: bestGroup.remainingPayment
        });

        couponDiscountCardId.value = bestGroup.requiredCards;
    } else {
        console.debug('[折扣卡] 未找到合适方案');
        couponDiscountCardId.value = [];
    }

    // 更新状态并触发计算
    hasManualDiscountCardSelection.value = false;
    console.debug('[折扣卡] 触发重新计算');
    changeCoupon(4);
}

/* 支付方式变更的公共处理函数 */
function handlePaymentMethodCommon(paymentMethod) {
    // 重置所有手动选择标志
    const resetManualFlags = () => {
        hasManualStorageCardSelection.value = false;
        hasManualDiscountCardSelection.value = false;
    };

    // 处理储值卡逻辑
    const handleStoredValueCard = () => {
        activeCollapseItem.value = ['storage-card'];
        if (!hasManualStorageCardSelection.value) {
            autoSelectStorageCards();
        } else {
            autoSelectStorageCards(true);
        }
    };

    // 处理折扣卡逻辑
    const handleDiscountCard = () => {
        activeCollapseItem.value = ['discount-card'];
        if (!hasManualDiscountCardSelection.value) {
            autoSelectDiscountCards();
        } else {
            autoSelectDiscountCards(true);
        }
    };

    // 处理次卡逻辑
    const handleSessionCard = () => {
        activeCollapseItem.value = ['time-card'];
        const validTimeCards = groupedTimeCards.value.filter(card => card.isValid);
        if (validTimeCards.length > 0) {
            validTimeCards[0].selected = true;
            changeCoupon(2, validTimeCards[0]);
        }
    };

    // 处理其他支付方式
    const handleOtherPayment = () => {
        activeCollapseItem.value = [];
        groupedTimeCards.value.forEach(card => card.selected = false);
        userCouponList.value
            .filter(item => item.coupon.couponType === "SessionCard")
            .forEach(item => item.selected = false);

        if (paymentForm.value.bonusAmount > 0) {
            paymentForm.value.bonusAmount = 0;
            paymentForm.value.paymentAmount = paymentForm.value.totalAmount;
        }
    };

    // 主逻辑分发
    switch (paymentMethod) {
        case 'StoredValueCard':
            handleStoredValueCard();
            break;
        case 'DiscountCard':
            handleDiscountCard();
            break;
        case 'SessionCard':
            handleSessionCard();
            break;
        default:
            handleOtherPayment();
            resetManualFlags();
    }
}

/* 支付方式变更处理 */
function handlePaymentMethodChange(value) {
    handlePaymentMethodCommon(value);
}

/* 监听支付方式变更 */
watch(() => paymentForm.value.paymentMethod, (newMethod) => {
    handlePaymentMethodCommon(newMethod);
});
// 监听储值卡选择变化
watch(couponStorageCardId, (newVal, oldVal) => {
    // 检测是否是用户手动改变了选择
    if (oldVal.length > 0 || newVal.length > 0) {
        // 标记为手动选择
        hasManualStorageCardSelection.value = true;

        // 如果手动选择了储值卡，自动切换支付方式为储值卡
        if (newVal.length > 0 && paymentForm.value.paymentMethod !== 'StoredValueCard') {
            // 使用isManualTrigger=true调用自动选择函数，这样不会覆盖用户的选择
            paymentForm.value.paymentMethod = 'StoredValueCard';
            autoSelectStorageCards(true);
        }
    }
}, { deep: true });

// 监听储值卡选择变化
watch(couponDiscountCardId, (newVal, oldVal) => {
    // 检测是否是用户手动改变了选择
    if (oldVal.length > 0 || newVal.length > 0) {
        // 标记为手动选择
        hasManualStorageCardSelection.value = true;

        // 如果手动选择了储值卡，自动切换支付方式为储值卡
        if (newVal.length > 0 && paymentForm.value.paymentMethod !== 'DiscountCard') {
            // 使用isManualTrigger=true调用自动选择函数，这样不会覆盖用户的选择
            paymentForm.value.paymentMethod = 'DiscountCard';
            autoSelectDiscountCards(true);
        }
    }
}, { deep: true });

// 初始化多订单表单
function initMultiOrderForm() {
    // 计算订单标题栏以及订单总金额
    let titles = '';
    if (props.orders.length <= 2) {
        titles = props.orders.map(item => item.orderNumber + `(` + item.mount + `元)`).join(' | ');
    } else {
        titles = props.orders.slice(0, 2).map(item => item.orderNumber + `(` + item.mount + `元)`).join(' | ') + ` 等${props.orders.length}个订单`;
    }

    const totalAmount = props.orders.reduce((acc, cur) => acc + cur.mount, 0);

    paymentForm.value = {
        // 订单信息
        orders: props.orders,
        titles: titles,

        // 支付方式和金额
        paymentMethod: 'Alipay',
        totalAmount: totalAmount,
        paymentAmount: totalAmount,
        bonusAmount: 0,
        priceDiff: 0,

        // 卡券相关
        ucId: '',
        timeBased: [],

        // 扫码支付相关
        authCode: '',
        subject: '',
        paymentType: '',

        // 其他
        orderType: 'Laundry'
    };

    // 判断储值卡金额是否能够覆盖订单金额
    const storageCardValue = userCouponList.value.filter(item =>
        item.coupon.couponType === "StoredValueCard" && item.isValid
    ).reduce((acc, cur) => acc + cur.availableValue, 0);

    if (paymentForm.value.totalAmount < storageCardValue) {
        paymentForm.value.paymentMethod = 'StoredValueCard';
    } else {
        // 如果储值卡余额不足，检查折扣卡是否能够覆盖订单金额
        const validDiscountCards = userCouponList.value.filter(item =>
            item.coupon.couponType === "DiscountCard" && item.isValid
        );

        if (validDiscountCards.length > 0) {
            // 按折扣率分组，找到最优折扣率
            const discountGroups = {};
            validDiscountCards.forEach(card => {
                const discountRate = card.coupon.couponValue;
                if (!discountGroups[discountRate]) {
                    discountGroups[discountRate] = [];
                }
                discountGroups[discountRate].push(card);
            });

            // 找到最优的折扣率组合
            let bestDiscountRate = 1;
            let bestGroupBalance = 0;

            for (const [discountRate, cards] of Object.entries(discountGroups)) {
                const rate = parseFloat(discountRate);
                const totalBalance = cards.reduce((sum, card) => sum + card.availableValue, 0);
                const discountedAmount = paymentForm.value.totalAmount * rate;

                // 如果这个折扣率更优且余额足够
                if (rate < bestDiscountRate && totalBalance >= discountedAmount) {
                    bestDiscountRate = rate;
                    bestGroupBalance = totalBalance;
                }
            }

            // 如果找到了合适的折扣卡组合
            if (bestDiscountRate < 1) {
                paymentForm.value.paymentMethod = 'DiscountCard';
            }
        }
    }
}

onMounted(async () => {
    if (props.visible) {
        await initForm();
        showPaymentDialog.value = true;
    }
});

</script>

<style scoped>
.payment-dialog {
    border-radius: 12px;
    overflow: hidden;
    background-color: var(--el-bg-color-page);
}

.scanner-dialog {
    border-radius: 12px;
    overflow: hidden;
}

.order-info {
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
    border-radius: 12px;
    padding: 1rem;
    box-shadow: var(--el-box-shadow-lighter);
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
    padding: 1rem 1rem 0 1rem;
}

.section-title {
    font-size: 1rem;
    font-weight: 600;
    margin: 1rem 0 12px 0;
    color: var(--el-text-color-primary);
}

.payment-method-section {
    margin-bottom: 24px;
}

.payment-method-group {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
    margin-bottom: 10px;
}

.scan-code-btn {
    margin-top: 1rem;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
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
    line-height: 14px;
}

.coupon-section {
    margin-bottom: 24px;
}

.coupon-checkbox-group,
.coupon-radio-group {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
    row-gap: .5rem;
}

.coupon-checkbox,
.coupon-radio {
    height: auto !important;
    margin-right: 0 !important;
    margin-bottom: 12px;
}

.coupon-card {
    width: 200px;
    padding: 12px;
    border-radius: 8px;
    /* border: 1px solid #e4e7ed; */
    background: linear-gradient(135deg, var(--el-color-primary-light-9) 0%, var(--el-color-primary-light-8) 100%);
    transition: all 0.3s;
}

.coupon-card.selected {
    border: 1px solid var(--el-color-primary);
    background: linear-gradient(135deg, #ecf5ff 0%, #f5f7fa 100%);
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
    /* color: #303133; */
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

.coupon-group-info {
    font-size: 12px;
    color: var(--el-color-primary);
    margin-top: 4px;
    background-color: rgba(64, 158, 255, 0.08);
    padding: 2px 6px;
    border-radius: 4px;
    display: inline-block;
}

.coupon-times {
    padding-top: .8rem;
    display: flex;
    flex-direction: column;
    gap: .5rem;
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
    padding: 1rem;
    margin: .5rem 0;
    transition: all 0.3s;
}

.price-summary-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
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
    font-size: 1rem;
    font-weight: 600;
    color: var(--el-text-color-primary);
}

.price-value.discount {
    color: #f56c6c;
}

.price-value.adjust {
    color: #409EFF;
    font-weight: 500;
}

.price-value.adjust-add {
    color: #67C23A;
    margin-right: 8px;
}

.price-value.adjust-sub {
    color: #f56c6c;
}

.price-value.adjust-remark {
    color: #909399;
    font-size: 14px;
    font-style: italic;
}

.adjust-details {
    display: flex;
}

.adjust-price-row {
    background-color: rgba(64, 158, 255, 0.05);
    border-radius: 4px;
    padding: 4px 8px;
    margin: 4px 0;
}

.price-value.total-amount {
    font-size: 24px;
    color: #f56c6c;
}

.price-divider {
    height: 1px;
    background-color: #e4e7ed;
    margin: 12px 0;
}

.price-diff-card {
    background-color: var(--el-fill-color-light);
    border-radius: 8px;
    padding: 1rem;
    margin: 1rem 0;
    transition: all 0.3s;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.price-diff-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.price-diff-content {
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.price-diff-main {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.price-diff-title {
    font-size: 1rem;
    font-weight: 600;
    color: var(--el-text-color-primary);
}

.price-diff-desc {
    font-size: 12px;
    color: var(--el-text-color-secondary);
    margin-top: -2px;
}

.price-diff-input {
    display: flex;
    align-items: center;
    position: relative;
    width: 180px;
}

.price-diff-symbol-left {
    position: absolute;
    left: 15px;
    font-weight: bold;
    color: var(--el-color-primary);
    font-size: 1rem;
    z-index: 1;
}

.price-diff-number :deep(.el-input-number__decrease),
.price-diff-number :deep(.el-input-number__increase) {
    background-color: var(--el-fill-color);
    border-color: var(--el-border-color-lighter);
}

.price-diff-number :deep(.el-input__inner) {
    padding-left: 30px;
    text-align: left;
    font-weight: bold;
    color: var(--el-color-primary);
}

.price-diff-number {
    width: 100%;
}

.price-diff-number :deep(.el-input-number) {
    transition: all 0.2s ease;
}

.price-diff-number :deep(.el-input-number:hover) {
    transform: translateY(-1px);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.price-diff-number :deep(.el-input) {
    border-radius: 4px;
    overflow: hidden;
}

.price-diff-number :deep(.el-input__wrapper) {
    box-shadow: 0 0 0 1px var(--el-border-color) inset;
    transition: all 0.2s ease;
}

.price-diff-number :deep(.el-input__wrapper:hover) {
    box-shadow: 0 0 0 1px var(--el-color-primary-light-3) inset;
}

.price-diff-number :deep(.el-input__wrapper.is-focus) {
    box-shadow: 0 0 0 1px var(--el-color-primary) inset;
}

.payment-footer {
    display: flex;
    justify-content: flex-end;
    gap: 12px;

    button {
        transition: all 0.3s;
    }

    button:hover {
        transform: translateY(-2px);
    }
}

/* 添加一些动画效果 */
.el-dialog__body {
    transition: all 0.3s;
}

.el-collapse-item__header {
    font-weight: 600;
    color: #303133;
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

/* 调整折叠面板样式以修复储值卡复选框遮挡问题 */
.storage-card-group {
    padding-top: 10px;
    /* 为复选框组添加顶部内边距 */
}

/* 调整所有折叠面板内容的样式 */
:deep(.el-collapse-item__content) {
    padding-top: 1rem;
    /* 增加顶部内边距 */
    padding-bottom: 1rem;
    /* 增加底部内边距 */
}

:deep(.el-collapse-item__wrap) {
    padding: 4px;
    /* 增加整体内边距 */
}

.coupon-count-badge {
    background-color: var(--el-color-primary);
    color: white;
    border-radius: 12px;
    padding: 2px 8px;
    font-size: 12px;
    margin-left: 8px;
    min-width: 20px;
    text-align: center;
}

.no-coupons-tip {
    background-color: var(--el-fill-color-light);
    border-radius: 8px;
    text-align: center;
    margin-bottom: 1rem;
}
</style>