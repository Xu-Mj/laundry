<template>
    <el-card v-if="showDialog">
        <!-- 添加或修改洗护服务订单对话框 -->
        <el-form ref="ordersRef" :model="form" :rules="rules" label-width="80px">
            <el-row>
                <el-col :span="6">
                    <el-form-item label="会员身份" prop="userId">
                        <el-select v-model="form.userId" :disabled="notEditable" filterable :clearable="true" remote
                            reserve-keyword placeholder="请输入手机号码搜索" allow-create @blur="handleBlur" remote-show-suffix
                            :remote-method="searchUserByTel" @change="selectUser" value-key="userId"
                            style="width: 240px">
                            <el-option v-for="item in userListRes" :key="item.userId"
                                :label="item.nickName + '\t' + item.phonenumber" :value="item.userId" />
                        </el-select>
                    </el-form-item>
                </el-col>
                <el-col :span="6">
                    <el-form-item label="会员姓名" prop="nickName">
                        <el-input :disabled="notEditable" v-model="form.nickName" placeholder="请输入会员姓名" />
                    </el-form-item>
                </el-col>
            </el-row>
            <el-form-item label="订单来源" prop="source">
                <el-radio-group v-model="form.source" @change="sourceChanged" :disabled="notEditable">
                    <el-radio v-for="dict in sys_price_order_type" :key="dict.value" :label="dict.label"
                        :value="dict.value">
                        {{ dict.label }}
                    </el-radio>
                </el-radio-group>
            </el-form-item>
            <!-- 价格管理 -->
            <el-form-item class="price-group">
                <el-radio-group v-model="form.priceId" :disabled="notEditable">
                    <el-radio v-for="item in priceList" @click="(event) => priceChange(event, item.priceId)"
                        :key="item.priceId" :label="item.priceName" :value="item.priceId">
                        {{ item.priceName }}
                    </el-radio>
                </el-radio-group>
            </el-form-item>
            <el-form-item label="衣物信息">
                <AddCloth v-if="form.userId" :userId="form.userId" :orderId="form.orderId" :submit="submitClothes"
                    :disabled="notEditable" :key="form.orderId" />
                <span v-else>请选择会员信息后添加衣物</span>
            </el-form-item>
            <el-form-item label="店主调价">
                <el-col :span="12" class="adjust-price-group">
                    <el-input type="number" :min="0" :max="1000" @input="adjustInput" @change="adjustInputChange"
                        v-model="form.adjust.adjustValueSub" placeholder="请输入调减金额" :disabled="notEditable" />
                    <el-input type="number" :min="0" :max="1000" @input="adjustInput" @change="adjustInputChange"
                        v-model="form.adjust.adjustValueAdd" placeholder="请输入调增金额" :disabled="notEditable" />
                    <el-input type="number" :min="0" :max="Infinity" @input="adjustInput" @change="adjustInputChange"
                        v-model="form.adjust.adjustTotal" placeholder="请输入总金额" :disabled="notEditable" />
                    <el-input v-model="form.adjust.remark" placeholder="备注信息" @change="adjustInputChange"
                        :disabled="notEditable" />
                </el-col>
            </el-form-item>
            <!-- 底部左侧信息区域，以及右侧按钮区域 -->
            <el-divider border-style="dashed" />
            <el-row class="footer">
                <el-col class="left" :span="18">
                    <el-row>
                        <el-col :span="4">
                            <el-form-item label="总件数：">{{ form.cloths.length }}</el-form-item>
                        </el-col>
                        <el-col :span="5">
                            <el-form-item label="总金额：">
                                <span class="payment-amount">
                                    {{ totalPrice }}
                                </span>
                            </el-form-item>
                        </el-col>
                        <el-col :span="6">
                            <el-form-item label-width="auto" label="预计取衣时间：">
                                {{ form.desireCompleteTime }}
                            </el-form-item>
                        </el-col>
                        <el-col :span="6">
                            <el-form-item label-width="auto" label="单据打印：">
                                <el-input-number :min="1" v-model="printCount" controls-position="right" />
                            </el-form-item>
                        </el-col>
                        <el-col :span="3">
                            <el-button type="primary" @click="printCloth" plain>打印</el-button>
                        </el-col>

                    </el-row>
                    <el-form-item class="coupon-list-container" label="卡信息：">
                        <div class="coupon-list">
                            <!-- 用户卡相关的信息：coupon类型为000、001、002的 -->
                            <span
                                v-if="userCouponList.filter(item => item.coupon.couponType == '000' || item.coupon.couponType == '001' || item.coupon.couponType == '002').length == 0">无</span>
                            <span v-else
                                v-for="(item, index) in userCouponList.filter(item => item.coupon.couponType == '000' || item.coupon.couponType == '001' || item.coupon.couponType == '002')"
                                :key="index">
                                <el-tooltip :content="getValidTime(item.coupon.validFrom, item.coupon.validTo)">
                                    {{ item.coupon.couponTitle }}
                                    -余额
                                    {{ item.availableValue }}
                                    {{ item.coupon.couponType == '000' ? '元' : '次' }}
                                    {{ isCurrentTimeWithinRange(item.coupon.validFrom,
                                        item.coupon.validTo) ? '' : '(不在有效期内)' }}
                                </el-tooltip>
                            </span>
                        </div>
                    </el-form-item>
                    <el-form-item label="券信息：">
                        <div class="coupon-list">
                            <!-- 用户券相关的信息：coupon类型为003、004的 -->
                            <span
                                v-if="userCouponList.filter(item => item.coupon.couponType == '003' || item.coupon.couponType == '004').length == 0">无</span>
                            <span v-else
                                v-for="(item, index) in userCouponList.filter(item => item.coupon.couponType == '003' || item.coupon.couponType == '004')"
                                :key="index">
                                {{ item.coupon.couponTitle }}
                                {{ isCurrentTimeWithinRange(item.coupon.validFrom,
                                    item.coupon.validTo) ? '' : '(不在有效期内)' }}
                            </span>
                        </div>
                    </el-form-item>
                    <el-form-item v-if="form.userId" label-width="auto" label="历史消费：">
                        <el-button type="primary" plain @click="() => { showHistoryDialog = true }">查看</el-button>
                    </el-form-item>

                </el-col>
                <el-col class="right" :span="6">
                    <div class="btn-container">
                        <el-button type="success" plain @click="createAndPay" :disabled="notEditable">收衣收款</el-button>
                        <el-button type="danger" plain :disabled="!form.userId || notEditable"
                            @click="handleShowCouponSale">卡券购买</el-button>
                        <el-button type="primary" plain @click="submitForm"
                            :disabled="notEditable || form.priceId || form.source == '02' || form.source === '01'">取衣收款</el-button>
                        <el-button type="warning" plain @click="cancelSelf">{{ form.orderId ? '关 闭' : '取 消'
                            }}</el-button>
                    </div>
                </el-col>
            </el-row>
        </el-form>

        <!-- 付款弹窗 -->
        <el-dialog title="付款" v-model="showPaymentDialog" width="600px" append-to-body lock-scroll modal
            :close-on-click-modal="false" @closed="initPaymentForm">
            <el-form ref="paymentRef" :model="paymentForm" :rules="paymentRules" label-width="80px">
                <el-form-item label="订单编号">
                    {{ paymentForm.payNumber }}
                </el-form-item>
                <el-form-item label="支付方式">
                    <template v-if="form.source === '01'">
                        美团结转
                    </template>
                    <template v-else-if="form.source === '02'">
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

        <!-- 卡券售卖弹窗 -->
        <el-dialog title="卡券购买" v-model="showCouponSale" width="1080px" append-to-body lock-scroll modal
            :close-on-click-modal="false">
            <CouponSale :userId="form.userId" :key="showCouponSale" :submit="submitCouponSale" />
        </el-dialog>

        <History :visible="showHistoryDialog" :userId="currentUserId" :key="showHistoryDialog"
            :toggle="() => { showHistoryDialog = !showHistoryDialog }" />
    </el-card>
</template>

<script setup name="CreateOrders">
import { ElMessageBox } from 'element-plus'
import { getOrders, addOrders, updateOrders, pay, updateAdjust } from "@/api/system/orders";
import { listPrice } from "@/api/system/price";
import { listUserWithNoLimit, addUser } from "@/api/system/user";
import { delCloths } from "@/api/system/cloths";
import { listUserCouponWithValidTime } from '@/api/system/user_coupon';
import { isCurrentTimeWithinRange, getFutureDate } from "@/utils";
import { getConfigKey } from '@/api/system/config';
import AddCloth from "./addCloth.vue";
import CouponSale from './couponSale.vue';
import History from "@/views/home/history.vue";
import { invoke } from "@tauri-apps/api/core";

const props = defineProps({
    visible: {
        type: Boolean,
        required: true,
    },
    orderId: {
        type: Number,
        required: true,
        default: 0
    },
    userId: {
        type: Number,
        required: true,
        default: 0
    },
    refresh: {
        type: Function,
        required: true,
    },
    toggle: {
        type: Function,
        required: true,
    }
});
const { proxy } = getCurrentInstance();
const {
    sys_price_order_type,
    sys_payment_method,
} =
    proxy.useDict(
        "sys_price_order_type",
        "sys_payment_method",
    );

// 用户列表，创建/更新订单时选择框使用
const userList = ref([]);
const userListRes = ref([]);
// 用户卡券列表
const userCouponList = ref([]);
// 用户卡券种类列表
const couponTypeList = ref();
// 价格列表
const priceList = ref([]);
const showDialog = ref(false);
const showCreateUser = ref(false);
const showPaymentDialog = ref(false);
const showCouponSale = ref(false);
const showHistoryDialog = ref(false);
const title = ref("");
const totalPrice = ref(0);
// 储值卡
const couponStorageCardId = ref([]);

const currentOrderId = ref(props.orderId);
const currentUserId = ref(props.userId);

const ordersRef = ref();
/* 单据打印数量 */
const printCount = ref(1);
const phoneRegex = /^1[3-9]\d{9}$/;

const notEditable = ref(false);
const showCoupons = ref(true);

const data = reactive({
    form: {
        cloths: [],
        adjust: {}
    },
    paymentForm: {
        orders: [],
    },
    refundForm: {},
    notifyForm: {},
    rules: {
        businessType: [
            { required: true, message: "业务类型不能为空", trigger: "change" }
        ],
        userId: [
            { required: true, message: "所属会员不能为空", trigger: "blur" },
            {
                validator: (rule, value, callback) => {
                    // 当没有匹配到任何会员时才进行手机号格式校验
                    const isNewUser = !userListRes.value.some(item => item.userId === form.value.userId);
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
        source: [
            { required: true, message: "订单来源不能为空", trigger: "blur" }
        ],
        cloths: [
            { required: true, message: "衣物信息不能为空", trigger: "change" }
        ]
    },
});

const { form, paymentForm, rules } = toRefs(data);



function printAllItems() {
    form.value.cloths.forEach((item, index) => {
        // 创建一个隐藏的iframe
        const iframe = document.createElement('iframe');
        iframe.style.visibility = 'hidden';
        iframe.style.position = 'fixed';
        iframe.style.right = '0';
        iframe.style.bottom = '0';
        document.body.appendChild(iframe);

        // 等待iframe加载完成
        iframe.onload = () => {
            const iframeDocument = iframe.contentDocument || iframe.contentWindow.document;
            // 设置iframe的文档内容
            iframeDocument.open();
            iframeDocument.write(generatePrintContent(item));
            iframeDocument.close();

            // 尝试静默打印
            try {
                iframe.contentWindow.print();
            } catch (e) {
                console.error('打印失败:', e);
            }

            // 打印完成后移除iframe
            iframe.onload = null;
            iframe.remove();
        };
    });
}

function generatePrintContent(item) {
    return `
    <div class="printer-container">
        <div class="printer-left">
            <div class="printer-shop-name">印洗匠心</div>
            <div class="printer-code">
                <img id="barcode" />
            </div>
        </div>
        <div class="printer-right">
            <div class="printer-first-line">${item.clothInfo.clothName}</div>
            <div class="printer-second-line"></div>
            <div class="printer-third-line"></div>
        </div>
    </div>`
}

function print() {
    console.log(form.value.cloths)
    const items = form.value.cloths;
    printAllItems(items);
}
// 处理子组件传过来的数据
function submitClothes(list) {
    form.value.cloths = list;
    checkCoupon();
    adjustInput();
}

// 处理价格radio 选中事件
function priceChange(event, priceId) {
    event.preventDefault();
    form.value.priceId = form.value.priceId === priceId ? null : priceId;
    adjustInput();
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
        if (form.value.cloths.length > count) {
            // 需要补充差价
            const diffCount = form.value.cloths.length - count;
            // 获取diffCount数量的衣物
            const diffCloths = form.value.cloths.slice(0, diffCount);
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

// 处理失去焦点的情况，保留用户输入
const handleBlur = (event) => {
    const inputValue = event.target.value;
    if (!userListRes.value.some(item => item.userId === form.value.userId)) {
        // 没有搜索结果且没有选择项时，保留输入
        form.value.userId = inputValue;
    }
    ordersRef.value.validateField('userId');
};

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

function handleShowCouponSale() {
    showCouponSale.value = true;
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
            card.count = form.value.cloths.length > count ?
                form.value.cloths.length - count > card.availableValue ?
                    card.availableValue : form.value.cloths.length - count : form.value.cloths.length;
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
            if (form.value.cloths.length > count) {
                // 需要补充差价
                const diffCount = form.value.cloths.length - count;
                // 获取diffCount数量的衣物
                const diffCloths = form.value.cloths.slice(0, diffCount);
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
    paymentForm.value.orders = [form.value]
    console.log(paymentForm.value)
    pay(paymentForm.value).then(res => {
        proxy.notify.success('支付成功');
        showPaymentDialog.value = false;
        reset();
        props.toggle();
    })

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

// function checkCoupon() {
//     // 判断每个卡券是否有效
//     for (const item of userCouponList.value) {
//         item.isValid = true;
//         item.unValidReason = '';
//         // 判断有效期
//         if (!isCurrentTimeWithinRange(item.coupon.validFrom, item.coupon.validTo)) {
//             item.isValid = false;
//             item.unValidReason = "不在有效期内";
//             continue;
//         }

//         // 判断最低消费金额
//         if (item.coupon.couponType == '004' && item.coupon.minSpend > totalPrice.value) {
//             item.isValid = false;
//             item.unValidReason = "最低消费金额不足";
//             continue;
//         }

//         if (item.coupon.couponType == '003') {
//             if (item.coupon.minSpend > totalPrice.value) {
//                 item.isValid = false;
//                 item.unValidReason = "最低消费金额不足";
//                 continue;
//             }
//             if (item.coupon.usageLimit < totalPrice.value) {
//                 item.isValid = false;
//                 item.unValidReason = "订单金额超过使用上限";
//                 continue;
//             }
//         }
//         // 适用衣物列表
//         const applicableClothsList = item.coupon.applicableCloths ? item.coupon.applicableCloths.split(',') : [];
//         // 适用分类列表
//         const applicableStyleList = item.coupon.applicableStyle ? item.coupon.applicableStyle.split(',') : [];
//         // 适用品类列表
//         const applicableCategoryList = item.coupon.applicableCategory ? item.coupon.applicableCategory.split(',') : [];
//         // 判断品类
//         for (const cloth of form.value.cloths) {
//             // 先判断适用衣物
//             if (applicableClothsList.length != 0 && !applicableClothsList.includes(cloth.clothInfo.clothingId)) {
//                 item.isValid = false;
//                 item.unValidReason = "适用衣物不匹配";
//                 break;
//             }

//             // 判断适用分类
//             if (applicableStyleList.length != 0 && applicableStyleList.includes(cloth.applicableStyle)) {
//                 item.isValid = false;
//                 item.unValidReason = "适用分类不匹配";
//                 break;
//             }
//             // 判断适用品类
//             if (applicableCategoryList.length != 0 && applicableCategoryList.includes(cloth.applicableCategory)) {
//                 item.isValid = false;
//                 item.unValidReason = "适用品类不匹配";
//                 break;
//             }
//         }
//     }
// }

// 取消按钮
function cancelSelf() {
    // 检查是否有未保存的数据
    if (!form.value.userId) {
        reset();
        props.toggle();
        return;
    }

    // 修改操作不允许反悔
    if (form.value.orderId) {
        reset();
        props.toggle();
        return;
    }

    // 弹出确认对话框
    ElMessageBox.confirm('确认取消创建订单？此操作不可逆！')
        .then(() => {
            // 用户确认取消，处理逻辑
            if (!form.value.orderId && form.value.cloths.length > 0) {
                // 删除添加的衣物列表
                delCloths(form.value.cloths.map(item => item.clothId))
                    .then(() => {
                        reset();
                        props.toggle();
                    })
                    .catch(res => {
                        console.error(res);
                    });
            } else {
                reset();
                props.toggle();
            }
        })
        .catch(() => {
            // 用户取消操作，不关闭对话框
        });
}

// 取消按钮
function cancel() {
    return new Promise((resolve, reject) => {
        // 检查是否有未保存的数据
        if (!form.value.userId) {
            reset();
            resolve(true); // 确认取消
            return;
        }

        // 修改操作不允许反悔
        if (form.value.orderId) {
            reset();
            props.toggle();
            return;
        }

        // 弹出确认对话框
        ElMessageBox.confirm('确认取消操作订单？此操作不可逆！')
            .then(() => {
                // 用户确认取消，处理逻辑
                if (!form.value.orderId && form.value.cloths.length > 0) {
                    // 删除添加的衣物列表
                    delCloths(form.value.cloths.map(item => item.clothId))
                        .then(() => {
                            reset();
                            resolve(true); // 允许关闭
                        })
                        .catch(res => {
                            console.error(res);
                            reject(false); // 出现错误，不允许关闭
                        });
                } else {
                    reset();
                    props.toggle();
                    resolve(true); // 允许关闭
                }
            })
            .catch(() => {
                // 用户取消操作，不关闭对话框
                reject(false); // 拒绝关闭
            });
    });
}

// 表单重置
function reset() {
    form.value = {
        adjust: {},
        cloths: [],
        orderId: null,
        priceId: null,
        orderNumber: null,
        businessType: null,
        userId: null,
        desireCompleteTime: null,
        costTimeAlarm: null,
        pickupCode: null,
        completeTime: null,
        deliveryMode: "00",
        source: "03",
        status: null,
        paymentStatus: null,
        remark: null,
        orderType: null,
        createTime: null,
        updateTime: null
    };
    proxy.resetForm("ordersRef");
}

// 监听订单来源变化
function sourceChanged() {
    // 获取价格列表
    listPrice({ orderType: form.value.source, status: 0 }).then(res => {
        priceList.value = res;
        form.value.priceId = null;
        adjustInput();
    });
}

/** 新增按钮操作 */
function handleAdd() {
    reset();
    title.value = "添加洗护服务订单";
    // 获取预计完成时间
    getConfigKey('desire_complete_time').then(res => {
        form.value.desireCompleteTime = getFutureDate(res);
    });
    listUserWithNoLimit().then(res => {
        userList.value = res;
    });
    // 获取价格列表
    listPrice({ orderType: form.value.source, status: 0 }).then(res => {
        priceList.value = res;
    });
}

/** 修改按钮操作 */
function handleUpdate() {
    reset();
    // 获取订单内容
    getOrders(currentOrderId.value).then(response => {
        form.value = response;
        form.value.cloths = [];
        if (form.value.paymentStatus == '00' || form.value.status == '05') {
            notEditable.value = true;
        }
        if (!form.value.adjust) {
            form.value.adjust = {};
        }
        title.value = "修改服务订单";
        // 获取价格列表
        listPrice({ orderType: form.value.source }).then(res => {
            priceList.value = res;
            console.log('create order price list', res)
        });
    });

    listUserWithNoLimit().then(res => {
        userList.value = res.rows;
        userListRes.value = userList.value;
    });
    // 获取用户卡券列表
    listUserCouponWithValidTime(currentUserId.value).then(response => {
        userCouponList.value = response;
        userCouponList.value.filter(item => item.coupon.couponType == '002').map(item => {
            item.selected = false;
            item.count = 1;
        });
        couponTypeList.value = new Set(userCouponList.value.map(coupon => coupon.coupon.couponType));
    });


}

/** 提交按钮 */
async function submitForm() {
    console.log(form.value)
    proxy.$refs["ordersRef"].validate(async valid => {
        if (valid) {
            if (!form.value.cloths || form.value.cloths.length == 0) {
                proxy.$modal.msgError("衣物信息不能为空");
                return;
            }
            form.value.clothIds = form.value.cloths.map(item => item.clothId);
            if (form.value.adjust.adjustValueAdd || form.value.adjust.adjustValueSub || form.value.adjust.adjustTotal) {
                form.value.adjust.orderId = form.value.orderId;
            }
            if (showCreateUser.value) {
                try {
                    const res = await addUser({
                        phonenumber: form.value.userId,
                        nickName: form.value.nickName
                    });

                    form.value.userId = res.userId; // 设置返回的用户ID
                } catch (err) {
                    proxy.$modal.msgError(err);
                    return; // 当 addUser 出错时，中断执行
                }
            }
            if (form.value.orderId != null) {
                updateOrders(form.value).then(response => {
                    proxy.notify.success("修改成功");
                    props.refresh();
                    props.toggle();
                });
            } else {
                addOrders(form.value).then(async response => {
                    proxy.notify.success("新增成功");
                    await printCloth();
                    props.refresh();
                    props.toggle();
                });
            }
        }
    });
}

/* 收衣收款 */
function createAndPay() {
    // 提交订单
    proxy.$refs["ordersRef"].validate(async valid => {
        if (valid) {
            if (!form.value.cloths || form.value.cloths.length == 0) {
                proxy.$modal.msgError("衣物信息不能为空");
                return;
            }
            // 如果选择了美团或者抖音，那么需要选择价格标签
            if (form.value.source == '01' || form.value.source == '02') {
                if (!form.value.priceId) {
                    proxy.$modal.msgError("请选择价格标签");
                    return;
                }
            }

            if (form.value.priceId && form.value.priceId !== 0) {
                showCoupons.value = false;
            }
            if (showCreateUser.value) {
                try {
                    const res = await addUser({
                        phonenumber: form.value.userId,
                        nickName: form.value.nickName
                    });
                    // 重新拉取用户列表
                    await listUserWithNoLimit().then(res => {
                        userList.value = res;
                    });

                    form.value.userId = res.userId; // 设置返回的用户ID

                    await listUserCouponWithValidTime(form.value.userId).then(response => {
                        userCouponList.value = response;
                        userCouponList.value.filter(item => item.coupon.couponType == '002').map(item => {
                            item.selected = false;
                            item.count = 1;
                        });
                        couponTypeList.value = new Set(userCouponList.value.map(coupon => coupon.coupon.couponType));
                    });

                } catch (err) {
                    proxy.$modal.msgError(err);
                    return; // 当 addUser 出错时，中断执行
                }
            }
            // 如果是创建订单，那么要先创建订单，拿到订单编码
            if (!form.value.orderId) {

                form.value.clothIds = form.value.cloths.map(item => item.clothId);

                proxy.$modal.loading("正在创建订单，请稍候");
                await addOrders(form.value).then(response => {
                    proxy.$modal.closeLoading();
                    form.value.orderId = response.orderId;
                    form.value.orderNumber = response.orderNumber;
                    // getList();
                });
                // 打印衣物信息
                await printCloth();
                // 初始化支付所需数据
                initPaymentForm();
                props.refresh();
                showPaymentDialog.value = true;
            } else {
                initPaymentForm();
                showPaymentDialog.value = true;
            }

        }
    });
}

/* 初始化支付表单数据 */
function initPaymentForm() {
    paymentForm.value = {
        orders: [form.value],
        ucOrderId: form.value.orderId,
        payNumber: form.value.orderNumber,
        paymentMethod: '02',
        orderType: '1',
        priceDiff: 0,
        totalAmount: totalPrice.value,
        paymentAmount: totalPrice.value,
    };
    if (form.value.source == '01') {
        paymentForm.value.paymentMethod = '03';
        showCoupons.value = false;
    } else if (form.value.source == '02') {
        paymentForm.value.paymentMethod = '04';
        showCoupons.value = false;
    }

    couponStorageCardId.value = [];
    checkCoupon();
}

/** 按手机号搜索会员 */
function searchUserByTel(tel) {
    userListRes.value = userList.value.filter(item => item.phonenumber.includes(tel));
    if (userListRes.value.length == 0) {
        showCreateUser.value = true;
        form.value.nickName = null;
        form.value.userId = null;
        userCouponList.value = [];
    } else {
        if (userListRes.value.length == 1) {
            form.value.nickName = userListRes.value[0].nickName;
            form.value.userId = userListRes.value[0].userId;
            // 查询会员卡券信息
            listUserCouponWithValidTime(form.value.userId).then(response => {
                userCouponList.value = response;
                userCouponList.value.filter(item => item.coupon.couponType == '002').map(item => {
                    item.selected = false;
                    item.count = 1;
                });
                couponTypeList.value = new Set(userCouponList.value.map(coupon => coupon.coupon.couponType));
            });
        }
        showCreateUser.value = false;
    }
}

/* 选择会员信息 */
function selectUser(userId) {
    if (!userId || userId.length == 0) {
        form.value.nickName = null;
        return;
    }
    console.log(userId);
    currentUserId.value = userId;
    const item = userList.value.find(item => { return item.userId === userId });
    form.value.nickName = item.nickName;
    // 查询会员卡券信息
    listUserCouponWithValidTime(userId).then(response => {
        userCouponList.value = response;
        userCouponList.value.filter(item => item.coupon.couponType == '002').map(item => {
            item.selected = false;
            item.count = 1;
        })
        couponTypeList.value = new Set(userCouponList.value.map(coupon => coupon.coupon.couponType));
    });
}

/* 获取有效期tooltip 的content */
function getValidTime(validFrom, validTo) {
    return `有效期：${validFrom} ~ ${validTo}`;
}

function adjustInputChange() {
    // 如果是修改操作，那么触发更新请求
    if (form.value.orderId && form.value.orderId !== 0) {
        updateAdjust(form.value).catch(res => {
            proxy.$modal.msgError(res.msg);
        })
    }
}

/* 调价输入框输入事件 */
function adjustInput() {
    // 强制转换调价字符串为数字
    if (form.value.adjust.adjustValueAdd) {
        form.value.adjust.adjustValueAdd = Number(form.value.adjust.adjustValueAdd);
    }

    if (form.value.adjust.adjustValueSub) {
        form.value.adjust.adjustValueSub = Number(form.value.adjust.adjustValueSub);
    }

    if (form.value.adjust.adjustTotal) {
        totalPrice.value = Number(form.value.adjust.adjustTotal);
        form.value.adjust.adjustTotal = Number(form.value.adjust.adjustTotal);
    } else {
        // 如果选择了价格item，那么使用价格item中的价格代替衣物价格
        let price;
        if (form.value.priceId) {
            const item = priceList.value.find(item => item.priceId === form.value.priceId);
            price = item ? item.priceValue : 0;
        } else {
            price = form.value.cloths.reduce((acc, cur) => {
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
            Number(form.value.adjust.adjustValueAdd ? form.value.adjust.adjustValueAdd : 0) -
            Number(form.value.adjust.adjustValueSub ? form.value.adjust.adjustValueSub : 0);
        totalPrice.value = price > 0 ? price : 0;
    }
}

async function printCloth() {
    const length = form.value.cloths.length;
    const user = userList.value.find(user => user.userId == form.value.userId);
    const result = form.value.cloths.map((item, index) => ({
        cloth_name: item.clothInfo.clothingName,
        cloth_color: item.clothingColor ? item.clothingColor : 0,
        cloth_flaw: item.clothingFlawArr,
        sum: length,
        num: index + 1,
        code: item.hangClothCode,
        time: item.createTime,
        client: {
            name: user.nickName,
            phone: user.phonenumber,
        },
        shelf: {
            name: String(item.hangLocationCode),
            position: item.hangerNumber,
        }
    }));
    proxy.$modal.loading('正在打印衣物信息...')
    await invoke('print', { items: result }).catch(err => {
        proxy.$modal.msgError(err.kind)
    })
    proxy.$modal.closeLoading();
}


onMounted(async () => {
    if (props.visible) {
        if (props.orderId !== 0) {
            handleUpdate();
        } else {
            handleAdd();
        }
        showDialog.value = true;
    }
});
defineExpose({
    cancel,
});
</script>

<style scoped>
.el-card {
    height: 100%;
    width: 100%;
    margin: 0;
    position: absolute;
    left: 0;
    top: 0;
}

.adjust-price-group {
    width: 100%;
    display: flex;
    justify-content: space-around;
    align-items: center;
    gap: 1rem;
}

.right {
    display: flex;
    justify-content: center;
    align-items: center;
    border-left: 1px dashed #ccc;
}

.btn-container {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    /* 创建两列，每列等宽 */
    grid-template-rows: repeat(2, 1fr);
    justify-content: center;
    align-items: center;
    /* 创建两行，每行等高 */
    gap: 1rem;

    button {
        width: 100%;
        height: 2.5rem;
        margin: 0;
        font-size: large;
    }
}

.payment-footer {
    text-align: center;
}

.coupon-list {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    gap: 1rem;
}

.status-row {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    justify-content: center;
    align-items: center;
    gap: .2rem;
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

.coupon-list-container {
    overflow: hidden;
}
</style>