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
                    <span>订单 - {{ paymentForm.payNumber }}</span>
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
                <div class="member-name">{{ user.nickName }}</div>
                <div class="member-phone">{{ user.phonenumber }}</div>
            </div>
            <div class="member-points">
                <div class="points-label">积分</div>
                <div class="points-value">{{ user.integral }}</div>
            </div>
        </div>

        <el-form ref="paymentRef" :model="paymentForm" :rules="paymentRules" label-width="80px" class="payment-form">
            <!-- 支付方式选择 -->
            <div class="section-title">支付方式</div>
            <el-form-item class="payment-method-section">
                <template v-if="props.order.source === '01'">
                    <div class="payment-method-card selected">
                        <el-icon>
                            <Promotion />
                        </el-icon>
                        <span>美团结转</span>
                    </div>
                </template>
                <template v-else-if="props.order.source === '02'">
                    <div class="payment-method-card selected">
                        <el-icon>
                            <VideoPlay />
                        </el-icon>
                        <span>抖音结转</span>
                    </div>
                </template>
                <template v-else>
                    <el-radio-group v-model="paymentForm.paymentMethod" class="payment-method-group">
                        <template v-for="dict in sys_payment_method" :key="dict.value">
                            <template v-if="dict.value == '06'">
                                <el-radio v-if="couponTypeList.has('000')" :value="dict.value"
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
                                <el-radio v-if="couponTypeList.has('002')" :value="dict.value"
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

                    <!-- 扫码支付按钮 -->
                    <el-button type="primary" @click="startScanPayment" class="scan-code-btn"
                        v-if="paymentForm.paymentMethod === '01' || paymentForm.paymentMethod === '02'">
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
                    <el-collapse>
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
                <!-- 显示原价 -->
                <!-- <div class="price-row">
                    <span class="price-label">衣物原价</span>
                    <span class="price-value">¥ {{ props.order.originalPrice }}</span>
                </div> -->
                <div class="price-row">
                    <span class="price-label">订单金额</span>
                    <span class="price-value">¥ {{ props.order.originalPrice ? props.order.originalPrice :
                        paymentForm.totalAmount }}</span>
                </div>
                <!-- 店主调价信息 -->
                <template
                    v-if="props.order.adjust && (props.order.adjust.adjustValueAdd || props.order.adjust.adjustValueSub || props.order.adjust.adjustTotal)">
                    <div class="price-row adjust-price-row" v-if="props.order.adjust.adjustTotal">
                        <span class="price-label">店主调价</span>
                        <span class="price-value adjust">总价调整为 ¥ {{ props.order.adjust.adjustTotal }}</span>
                    </div>
                    <div class="price-row adjust-price-row" v-else>
                        <span class="price-label">店主调价</span>
                        <div class="adjust-details">
                            <span v-if="props.order.adjust.adjustValueAdd" class="price-value adjust-add">
                                + ¥{{ props.order.adjust.adjustValueAdd }}
                            </span>
                            <span v-if="props.order.adjust.adjustValueSub" class="price-value adjust-sub">
                                - ¥{{ props.order.adjust.adjustValueSub }}
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
                <el-button size="large" type="danger" @click="close">取消</el-button>
                <el-button size="large" type="primary" @click="submitPaymentForm">确认支付</el-button>
            </div>
        </template>
    </el-dialog>

    <!-- 卡券购买弹窗 -->
    <el-dialog v-model="showCouponSale" width="600px" append-to-body lock-scroll modal :align-center="true"
        :close-on-click-modal="false" :show-close="false">
        <CouponSale :userId="props.order.userId" :key="showCouponSale"
            :taggle="() => { showCouponSale = !showCouponSale }" :visible="showCouponSale"
            :couponTypeList="couponTypeList" :submit="submitCouponSale" />
    </el-dialog>
</template>

<script setup name="Pay">
import { pay } from "@/api/system/orders";
import CouponSale from './couponSale.vue';
import { listUserCouponWithValidTime } from '@/api/system/user_coupon';
import { getUser } from '@/api/system/user';
import { isCurrentTimeWithinRange } from "@/utils";
import { onMounted } from "vue";
import { getPrice } from "@/api/system/price";
import QrCodeScanner from '@/components/QrCodeScanner/index.vue';
import { ElLoading } from 'element-plus';

const props = defineProps({
    visible: {
        type: Boolean,
        required: true,
        default: false,
    },
    order: {
        type: Object,
        required: true,
    },
    refresh: {
        type: Function,
        required: true,
    },
    toggle: {
        type: Function,
        required: true,
    },
});

const emit = defineEmits(['payment-success', 'payment-failed', 'payment-cancel']);

const { proxy } = getCurrentInstance();
const { sys_payment_method } = proxy.useDict("sys_payment_method");

const showPaymentDialog = ref(false);
const showCoupons = ref(true);
const showScannerDialog = ref(false);

const paymentForm = ref({
    orders: [],
});
// 用户卡券列表
const userCouponList = ref([]);
// 用户卡券种类列表
const couponTypeList = ref(new Set());
const totalPrice = ref(0);
const couponStorageCardId = ref([]);
const user = ref({});
const showCouponSale = ref(false);

// 支付方式：01 支付宝，02 微信支付，03 美团结转，04 抖音结转，05 现金支付，06 储值卡支付，07 次卡支付 ，09 其他
// 组合支付：16 支付宝+储值卡，26 微信支付+储值卡， 27 微信支付+次卡，17 支付宝+次卡，18 支付宝+优惠券， 28 微信支付+优惠券
// 56 现金支付+储值卡，57 现金支付+次卡，58 现金支付+优惠券
function close() {
    emit('payment-cancel');
    props.toggle();
}


/* 卡券购买完成后的回调，重新获取卡券列表 */
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

/* 处理扫码结果 */
function handleScanResult(result) {
    console.log('扫码结果:', result);
    // 关闭扫码弹窗
    showScannerDialog.value = false;

    // 显示全局loading
    const loadingInstance = ElLoading.service({
        lock: true,
        text: '正在处理支付...',
        background: 'rgba(0, 0, 0, 0.7)'
    });

    // 根据扫码结果处理支付逻辑
    try {
        // 示例：根据扫码结果前缀判断支付方式
        if (result.startsWith('1') || result.startsWith('2') || result.startsWith('3')) {
            // 支付宝付款码通常以1、2、3开头
            paymentForm.value.paymentMethod = '01';
            proxy.notify.success('已识别为支付宝付款码');
        } else if (result.startsWith('1')) {
            // 微信付款码通常以1开头
            paymentForm.value.paymentMethod = '02';
            proxy.notify.success('已识别为微信付款码');
        } else {
            proxy.notify.warning('无法识别的付款码格式');
        }

        // 自动提交支付表单
        submitPaymentForm().finally(() => {
            // 关闭loading
            loadingInstance.close();
        });
    } catch (error) {
        console.error('处理支付出错:', error);
        proxy.notify.error('处理支付出错');
        loadingInstance.close();
    }
}

// 初始化表单数据
function initForm() {
    // 获取用户信息
    getUser(props.order.userId).then(res => {
        user.value = res;
    });

    // 获取用户卡券列表
    listUserCouponWithValidTime(props.order.userId).then(response => {
        userCouponList.value = response;
        userCouponList.value.filter(item => item.coupon.couponType == '002').map(item => {
            item.selected = false;
            item.count = 1;
        });
        couponTypeList.value = new Set(userCouponList.value.map(coupon => coupon.coupon.couponType));
    });

    paymentForm.value = {
        orders: [props.order],
        ucOrderId: props.order.orderId,
        payNumber: props.order.orderNumber,
        paymentMethod: '02',
        orderType: '1',
        priceDiff: 0,
        totalAmount: 0,
        paymentAmount: 0,
    };

    if (props.order.source == '01') {
        paymentForm.value.paymentMethod = '03';
        showCoupons.value = false;
    } else if (props.order.source == '02') {
        paymentForm.value.paymentMethod = '04';
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
                if (cur.serviceRequirement == '001') {
                    priceValue *= 2;
                } else if (cur.serviceRequirement == '002') {
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
        // 这里的总价已经在createOrder.vue中计算好并通过totalPrice传递，
        // 所以这个条件可能永远不会执行，但我们保留它作为后备
        price = props.order.totalPrice || 0;
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
            if (cur.serviceRequirement == '001') {
                priceValue *= 2;
            } else if (cur.serviceRequirement == '002') {
                priceValue *= 1.5;
            }
            return acc +
                priceValue + cur.processMarkup
        }, 0);
        price +=
            Number(props.order.adjust.adjustValueAdd ? props.order.adjust.adjustValueAdd : 0) -
            Number(props.order.adjust.adjustValueSub ? props.order.adjust.adjustValueSub : 0);
    }

    paymentForm.value.totalAmount = price > 0 ? price : 0;
    paymentForm.value.paymentAmount = paymentForm.value.totalAmount;
    couponStorageCardId.value = [];
    checkCoupon();
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
    // 
    paymentForm.value.orders = [props.order]
    console.log(paymentForm.value)
    pay(paymentForm.value).then(res => {
        proxy.notify.success('支付成功');
        showPaymentDialog.value = false;
        // 发送支付成功回调
        emit('payment-success', {
            paymentMethod: paymentForm.value.paymentMethod,
            amount: paymentForm.value.paymentAmount
        });
        props.refresh();
        props.toggle();
    }).catch(error => {
        // 发送支付失败回调
        emit('payment-failed', error.message || '支付失败');
    });
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
            card.count = props.order.cloths.length > count ?
                props.order.cloths.length - count > card.availableValue ?
                    card.availableValue : props.order.cloths.length - count : props.order.cloths.length;
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

        if (count == 0) {
            paymentForm.value.paymentMethod = '02';
            paymentForm.value.bonusAmount = 0;
            paymentForm.value.priceDiff = 0;
        } else {

            // 计算差价
            if (props.order.cloths.length > count) {
                // 需要补充差价
                const diffCount = props.order.cloths.length - count;
                // 获取diffCount数量的衣物
                const diffCloths = props.order.cloths.slice(0, diffCount);
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
            debugger
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

// 此卡数量改变
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
        if (props.order.cloths.length > count) {
            // 需要补充差价
            const diffCount = props.order.cloths.length - count;
            // 获取diffCount数量的衣物
            const diffCloths = props.order.cloths.slice(0, diffCount);
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

onMounted(async () => {
    if (props.visible) {
        await initForm();
        await listUserCouponWithValidTime(props.order.userId).then(response => {
            userCouponList.value = response;
            userCouponList.value.filter(item => item.coupon.couponType == '002').map(item => {
                item.selected = false;
                item.count = 1;
            });
            couponTypeList.value = new Set(userCouponList.value.map(coupon => coupon.coupon.couponType));
            checkCoupon();
            showPaymentDialog.value = true;
        });
        // get user information
        await getUser(props.order.userId).then(response => {
            user.value = response;
        });
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
    /* background: linear-gradient(135deg, var(--el-fill-color-light) 0%, var(--el-fill-color-dark) 100%); */
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
    border: 1px solid #e4e7ed;
    background: linear-gradient(135deg, #ffffff 0%, #f5f7fa 100%);
    transition: all 0.3s;
}

.coupon-card.selected {
    border-color: var(--el-color-primary);
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
    color: #303133;
}

.coupon-value {
    font-size: 14px;
    color: #606266;
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

.price-diff-section {
    margin-top: 1rem;
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
</style>