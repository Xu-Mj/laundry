<template>
    <!-- 付款弹窗 -->
    <el-dialog v-model="props.visible" width="600px" :align-center="true" append-to-body lock-scroll modal
        :close-on-press-escape="false" :close-on-click-modal="false" :show-close="false" class="payment-dialog">
        <template #header>
            <div class="dialog-header">
                <div class="order-info">
                    <el-icon>
                        <Ticket />
                    </el-icon>
                    <span>订单信息</span>
                </div>
                <div class="order-number" :title="paymentForm.titles">{{ paymentForm.titles }}</div>
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
                    <!-- 当没有任何卡券时显示提示信息 -->
                    <div v-if="userCouponList.length === 0" class="no-coupons-tip">
                        <el-empty description="该用户名下没有任何卡券" :image-size="100">
                            <el-button type="primary" size="small" plain @click="showCouponSale = true">
                                <el-icon>
                                    <Plus />
                                </el-icon>购买卡券
                            </el-button>
                        </el-empty>
                    </div>

                    <!-- 当有卡券但没有有效卡券时显示提示 -->
                    <div v-else-if="userCouponList.filter(item => item.isValid).length === 0" class="no-coupons-tip">
                        <el-empty description="该用户名下没有可用的卡券" :image-size="100">
                            <el-button type="primary" size="small" plain @click="showCouponSale = true">
                                <el-icon>
                                    <Plus />
                                </el-icon>购买卡券
                            </el-button>
                        </el-empty>
                    </div>

                    <!-- 储值卡 -->
                    <el-collapse v-else-if="userCouponList.filter(item => item.coupon.couponType == '000').length !== 0"
                        v-model="activeCollapseItems">
                        <el-collapse-item name="storage-card">
                            <template #title>
                                <span>储值卡</span>
                                <span class="coupon-count-badge">{{userCouponList.filter(item => item.coupon.couponType
                                    == '000' && item.isValid).length}}</span>
                            </template>
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
                    <el-collapse v-if="userCouponList.filter(item => item.coupon.couponType == '002').length != 0"
                        v-model="activeCollapseItems">
                        <el-collapse-item name="time-card">
                            <template #title>
                                <span>次卡</span>
                                <span class="coupon-count-badge">{{userCouponList.filter(item => item.coupon.couponType
                                    == '002' && item.isValid).length}}</span>
                            </template>
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
                        v-if="userCouponList.filter(item => item.coupon.couponType !== '002' && item.coupon.couponType !== '000').length != 0"
                        v-model="activeCollapseItems">
                        <el-collapse-item name="discount-coupon">
                            <template #title>
                                <span>优惠券</span>
                                <span class="coupon-count-badge">{{userCouponList.filter(item => item.coupon.couponType
                                    !== '000' && item.coupon.couponType !== '002' && item.isValid).length}}</span>
                            </template>
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
            <div v-if="paymentForm.priceDiff > 0" class="price-diff-card">
                <div class="price-diff-content">
                    <div class="price-diff-main">
                        <span class="price-diff-title">补差价</span>
                        <div class="price-diff-input">
                            <span class="price-diff-symbol-left">¥</span>
                            <el-input-number v-model="paymentForm.priceDiff" controls-position="right" :min="0"
                                :max="paymentForm.paymentAmount" placeholder="请输入补差价" size="large"
                                class="price-diff-number" />
                        </div>
                    </div>
                    <span class="price-diff-desc">使用储值卡或次卡时可能需要补差价</span>
                </div>
            </div>
        </el-form>

        <template #footer>
            <div class="payment-footer">
                <el-button size="large" type="danger" @click="props.toggle">取消</el-button>
                <el-button size="large" type="primary" color="#626aef"
                    @click="submitPaymentForm(false)">确认收款</el-button>
                <el-button size="large" type="success" @click="submitPaymentForm(true)">收款并取衣</el-button>
            </div>
        </template>
    </el-dialog>

    <!-- 卡券购买弹窗 -->
    <el-dialog v-model="showCouponSale" width="800px" append-to-body lock-scroll modal :align-center="true"
        :close-on-click-modal="false" :show-close="false">
        <CouponSale :userId="props.user.userId" :key="showCouponSale"
            :taggle="() => { showCouponSale = !showCouponSale }" :visible="showCouponSale"
            :couponTypeList="couponTypeList" :submit="submitCouponSale" />
    </el-dialog>
</template>

<script setup>
import { pay } from "@/api/system/orders";
import { isCurrentTimeWithinRange } from "@/utils";
import CouponSale from './couponSale.vue';

const props = defineProps({
    visible: {
        type: Boolean,
        required: true,
        default: false
    },
    user: {
        type: Object,
        required: false,
        default: null
    },
    orders: {
        type: Array,
        required: false,
        default: () => []
    },
    clothsList: {
        type: Array,
        required: false,
        default: () => []
    },
    userCouponList: {
        type: Array,
        required: false,
        default: () => []
    },
    couponTypeList: {
        type: Set,
        required: false,
        default: () => new Set()
    },
    toggle: {
        type: Function,
        required: true,
        default: () => { }
    },
    refresh: {
        type: Function,
        required: false,
        default: () => { }
    }
});

const emit = defineEmits(['update:visible', 'cancel', 'submit', 'pickup', 'success', 'show-coupon-sale']);

const { proxy } = getCurrentInstance();
const { sys_payment_method } = proxy.useDict("sys_payment_method");

// 是否显示卡券选择区域
const showCoupons = ref(true);

// 支付表单
const paymentForm = ref({
    paymentMethod: '02',
    orderType: '1',
    totalAmount: 0,
    bonusAmount: 0,
    paymentAmount: 0,
    priceDiff: 0,
    titles: '',
    orders: []
});
// 选中的储值卡列表
const couponStorageCardId = ref([]);

// 支付表单校验规则
const paymentRules = ref({});

// 总价格
const totalPrice = ref(0);

// 选中的次卡数量总数
const timeCardCount = ref(0);
// 用户卡券列表
const userCouponList = ref(props.userCouponList || []);
// 用户卡券种类列表
const couponTypeList = ref(props.couponTypeList || new Set());
// 折叠面板展开项跟踪
const activeCollapseItems = ref(['storage-card']);
const showCouponSale = ref(false);

onMounted(() => {
    if (props.visible) {
        initPaymentForm();
    }
})

function submitCouponSale() {
    listUserCouponWithValidTime(form.value.userId).then(response => {
        userCouponList.value = response;
        userCouponList.value.filter(item => item.coupon.couponType == '002').map(item => {
            item.selected = false;
            item.count = 1;
        });
        couponTypeList.value = new Set(userCouponList.value.map(coupon => coupon.coupon.couponType));
        checkCoupon();
    });
    showCouponSale.value = false;
}
// 初始化支付表单
function initPaymentForm() {
    paymentForm.value = {
        paymentMethod: '02',
        orderType: '1',
        totalAmount: 0,
        bonusAmount: 0,
        paymentAmount: 0,
        priceDiff: 0,
        titles: '',
        orders: props.orders || []
    };

    couponStorageCardId.value = [];
    updatePaymentForm();
}

// 更新支付表单
async function updatePaymentForm() {
    if (!props.orders || props.orders.length === 0) return;

    // 计算订单标题栏以及订单总金额
    if (props.orders.length <= 2) {
        paymentForm.value.titles = props.orders.map(item => item.orderNumber + `(` + item.mount + `元)`).join(' | ');
    } else {
        paymentForm.value.titles = props.orders.slice(0, 2).map(item => item.orderNumber + `(` + item.mount + `元)`).join(' | ') + ` 等${props.orders.length}个订单`;
    }
    paymentForm.value.totalAmount = props.orders.reduce((acc, cur) => acc + cur.mount, 0);
    paymentForm.value.orders = props.orders;

    // 校验卡券
    checkCoupon();

    // 判断储值卡金额是否能够覆盖订单金额
    const storageCardValue = userCouponList.value.filter(item => item.coupon.couponType === "000" && item.isValid).reduce((acc, cur) => acc + cur.availableValue, 0);
    if (paymentForm.value.totalAmount < storageCardValue) {
        paymentForm.value.paymentMethod = '06';
    }
    paymentForm.value.bonusAmount = 0;
    paymentForm.value.paymentAmount = paymentForm.value.totalAmount;
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
/* 次卡数量变化 */
function changeCouponCount(card) {
    changeCoupon(2, card);
}

/* 提交支付 */
async function submitPaymentForm(isPickup) {
    // 判断是否使用了优惠券
    if (!paymentForm.value.couponId) {
        if (couponStorageCardId.value.length > 0) {
            paymentForm.value.ucId = couponStorageCardId.value.join(',');
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

            // 判断储值卡金额是否能够覆盖订单金额
            if (storageCardPrice < paymentForm.value.totalAmount) {
                paymentForm.value.priceDiff = paymentForm.value.totalAmount - storageCardPrice;
                paymentForm.value.paymentMethod = '02';
            } else {
                paymentForm.value.priceDiff = 0;
                paymentForm.value.paymentMethod = '06';
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
            paymentForm.value.paymentAmountMv = paymentForm.value.paymentAmount;
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
        paymentForm.value.orders.forEach(item => item.paymentStatus = '00');

        // 关闭支付对话框
        props.toggle();

        // 刷新数据
        props.refresh();

        // 发送成功事件，包含是否需要取衣的信息
        emit('success', { isPickup });

        // 如果选中了衣物，并且需要取走
        if (isPickup) {
            emit('pickup');
        } else {
            emit('submit');
        }
    } catch (err) {
        // 错误处理
        console.error('支付失败:', err);
        proxy.notify.error('支付失败: ' + err.message);
    }
}

// Add a watch for payment method changes
// 添加监听，当支付方式变更时，自动展开相应的优惠选项
watch(() => paymentForm.value.paymentMethod, (newMethod) => {
    if (newMethod === '06') {
        // 如果选择了储值卡支付，自动展开储值卡区域
        if (!activeCollapseItems.value.includes('storage-card')) {
            activeCollapseItems.value = ['storage-card'];
        }
    } else if (newMethod === '07') {
        // 如果选择了次卡支付，自动展开次卡区域
        if (!activeCollapseItems.value.includes('time-card')) {
            activeCollapseItems.value = ['time-card'];
        }
    }
});
</script>

<style scoped>
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
    max-width: 500px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    display: block;
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
    font-size: 16px;
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
    font-size: 16px;
    z-index: 1;
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

.price-diff-number :deep(.el-input__inner) {
    padding-left: 30px;
    text-align: left;
    font-weight: bold;
    color: var(--el-color-primary);
}

.price-diff-number :deep(.el-input-number__decrease),
.price-diff-number :deep(.el-input-number__increase) {
    background-color: var(--el-fill-color);
    border-color: var(--el-border-color-lighter);
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

/* Add the coupon count badge styles */
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

/* Add CSS for empty state */
.no-coupons-tip {
    background-color: var(--el-fill-color-light);
    border-radius: 8px;
    padding: 20px;
    text-align: center;
    margin-bottom: 16px;
}
</style>