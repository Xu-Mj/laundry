<template>
    <!-- 付款弹窗 -->
    <el-dialog title="付款" v-model="showPaymentDialog" width="600px" append-to-body lock-scroll modal
        :close-on-click-modal="false" @closed="close">
        <el-form ref="paymentRef" :model="paymentForm" :rules="paymentRules" label-width="80px">
            <el-form-item label="订单编号">
                {{ paymentForm.payNumber }}
            </el-form-item>
            <el-form-item label="支付方式">
                <template v-if="props.order.source === '01'">
                    美团结转
                </template>
                <template v-else-if="props.order.source === '02'">
                    抖音结转
                </template>
                <template v-else>
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
                </template>
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
                <el-form-item label="优惠券">
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
                <el-button type="primary" @click="submitPaymentForm">确认收款</el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup name="Pay">
import { pay } from "@/api/system/orders";
import { listUserCouponWithValidTime } from '@/api/system/user_coupon';
import { isCurrentTimeWithinRange } from "@/utils";
import { onMounted } from "vue";

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

const { proxy } = getCurrentInstance();
const { sys_payment_method } = proxy.useDict("sys_payment_method");

const showPaymentDialog = ref(false);
const showCoupons = ref(true);

const paymentForm = ref({
    orders: [],
});
// 用户卡券列表
const userCouponList = ref([]);
// 用户卡券种类列表
const couponTypeList = ref(new Set());
const totalPrice = ref(0);
const couponStorageCardId = ref([]);

// 支付方式：01 支付宝，02 微信支付，03 美团结转，04 抖音结转，05 现金支付，06 储值卡支付，07 次卡支付 ，09 其他
// 组合支付：16 支付宝+储值卡，26 微信支付+储值卡， 27 微信支付+次卡，17 支付宝+次卡，18 支付宝+优惠券， 28 微信支付+优惠券
// 56 现金支付+储值卡，57 现金支付+次卡，58 现金支付+优惠券
function close() {
    initPaymentForm();
    props.toggle();
}

/* 初始化支付表单数据 */
function initPaymentForm() {
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

    let price;
    if (props.order.priceId) {
        const item = priceList.value.find(item => item.priceId === props.order.priceId);
        price = item ? item.priceValue : 0;
    } else {
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
    }
    price +=
        Number(props.order.adjust.adjustValueAdd ? props.order.adjust.adjustValueAdd : 0) -
        Number(props.order.adjust.adjustValueSub ? props.order.adjust.adjustValueSub : 0);
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
    // 
    paymentForm.value.orders = [props.order]
    console.log(paymentForm.value)
    pay(paymentForm.value).then(res => {
        proxy.$modal.msgSuccess('支付成功');
        showPaymentDialog.value = false;
        // reset();
        props.refresh();
        props.toggle();
    })
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
        initPaymentForm();
        await listUserCouponWithValidTime({ userId: props.order.userId }).then(response => {
            userCouponList.value = response.rows;
            userCouponList.value.filter(item => item.coupon.couponType == '002').map(item => {
                item.selected = false;
                item.count = 1;
            });
            couponTypeList.value = new Set(userCouponList.value.map(coupon => coupon.coupon.couponType));
            console.log(props.order)
            checkCoupon();
            showPaymentDialog.value = true;
        });
    }
});

</script>

<style scoped>
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