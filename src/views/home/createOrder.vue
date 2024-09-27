<template>
    <div>
        <!-- 添加或修改洗护服务订单对话框 -->
        <el-form ref="ordersRef" :model="form" :rules="rules" label-width="80px">
            <el-row>
                <el-col :span="6">
                    <el-form-item label="会员身份" prop="userId">
                        <el-select v-model="form.userId" filterable :clearable="true" remote reserve-keyword
                            placeholder="请输入手机号码搜索" allow-create @blur="handleBlur" remote-show-suffix
                            :remote-method="searchUserByTel" @change="selectUser" value-key="userId"
                            style="width: 240px">
                            <el-option v-for="item in userListRes" :key="item.userId"
                                :label="item.nickName + '\t' + item.phonenumber" :value="item.userId" />
                        </el-select>
                    </el-form-item>
                </el-col>
                <el-col :span="6">
                    <el-form-item label="会员姓名" prop="nickName">
                        <el-input v-model="form.nickName" placeholder="请输入会员姓名" />
                    </el-form-item>
                </el-col>
            </el-row>
            <el-form-item label="订单来源" prop="source">
                <el-radio-group v-model="form.source">
                    <el-radio v-for="dict in sys_price_order_type" :key="dict.value" :label="dict.label"
                        :value="dict.value">
                        {{ dict.label }}
                    </el-radio>
                </el-radio-group>
            </el-form-item>
            <!-- 价格管理 -->
            <el-form-item class="price-group">
                <el-radio-group v-model="form.priceId">
                    <el-radio v-for="item in priceList" @click="(event) => priceChange(event, item.priceId)"
                        :key="item.priceId" :label="item.priceName" :value="item.priceId">
                        {{ item.priceName }}
                    </el-radio>
                </el-radio-group>
            </el-form-item>
            <el-form-item label="衣物信息">
                <AddCloth v-if="form.userId" :userId="form.userId" :orderId="form.orderId"
                    v-model:value="form.cloths" />
                <span v-else>请选择会员信息后添加衣物</span>
            </el-form-item>
            <el-form-item label="店主调价">
                <el-col :span="12" class="adjust-price-group">
                    <el-input type="number" :min="0" :max="1000" @input="adjustInput"
                        v-model="form.adjust.adjustValueSub" placeholder="请输入调减金额" />
                    <el-input type="number" :min="0" :max="1000" @input="adjustInput"
                        v-model="form.adjust.adjustValueAdd" placeholder="请输入调增金额" />
                    <el-input type="number" :min="0" :max="Infinity" @input="adjustInput"
                        v-model="form.adjust.totalAmount" placeholder="请输入总金额" />
                    <el-input v-model="form.adjust.remark" placeholder="备注信息" />
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
                                {{ totalPrice }}
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
                            <el-button type="primary" plain @click="printOrder">打印</el-button>
                        </el-col>

                    </el-row>
                    <el-form-item label="卡信息：">
                        <div class="coupon-list">
                            <!-- 用户卡相关的信息：coupon类型为000、001、002的 -->
                            <span
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
                                v-for="(item, index) in userCouponList.filter(item => item.coupon.couponType == '003' || item.coupon.couponType == '004')"
                                :key="index">
                                {{ item.coupon.couponTitle }}
                                {{ isCurrentTimeWithinRange(item.coupon.validFrom,
                                    item.coupon.validTo) ? '' : '(不在有效期内)' }}
                            </span>
                        </div>
                    </el-form-item>
                </el-col>
                <el-col class="right" :span="6">
                    <div class="btn-container">
                        <el-button type="success" plain @click="createAndPay">收衣收款</el-button>
                        <el-button type="info" plain :disabled="!form.userId"
                            @click="handleShowCouponSale">卡券购买</el-button>
                        <el-button type="primary" plain @click="submitForm">取衣收款</el-button>
                        <el-button type="warning" plain @click="cancelSelf">取 消</el-button>
                    </div>
                </el-col>
            </el-row>
        </el-form>

        <!-- 付款弹窗 -->
        <el-dialog title="付款" v-model="showPaymentDialog" width="600px" append-to-body lock-scroll modal
            :close-on-click-modal="false">
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
                                <el-radio v-if="dict.value !== '03' && dict.value !== '04'" :value="dict.value">
                                    {{ dict.label }}
                                </el-radio>
                            </template>
                        </el-radio-group>
                    </template>
                </el-form-item>
                <el-form-item label="储值卡">
                    <!-- 列出储值卡列表 -->
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
                <el-form-item label="优惠券">
                    <el-radio-group v-model="paymentForm.couponId" @change="changeCoupon(2)">
                        <el-radio v-for="card in userCouponList.filter(item => item.coupon.couponType !== '000')"
                            :disabled="!card.isValid" :key="card.ucId" :value="card.ucId">
                            {{ card.coupon.couponTitle }}
                            <!-- - -->
                            <!-- {{ card.ucCount }} -->
                            <!-- 张 -->
                            {{ card.isValid ? '' : '(' + card.unValidReason + ')' }}
                        </el-radio>
                    </el-radio-group>
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

        <!-- 卡券售卖弹窗 -->
        <el-dialog title="卡券购买" v-model="showCouponSale" width="1080px" append-to-body lock-scroll modal
            :close-on-click-modal="false">
            <CouponSale :userId="form.userId" :key="showCouponSale" :submit="submitCouponSale" />
        </el-dialog>
    </div>
</template>

<script setup name="CreateOrders">
import { watch } from "vue";
import { ElMessageBox } from 'element-plus'
import { getOrders, addOrders, updateOrders, pay } from "@/api/system/orders";
import { listPrice } from "@/api/system/price";
import { listUserWithNoLimit, addUser } from "@/api/system/user";
import { delCloths } from "@/api/system/cloths";
import { listUserCoupon } from '@/api/system/user_coupon';
import { isCurrentTimeWithinRange, getFutureDate } from "@/utils";
import { getConfigKey } from '@/api/system/config';
import AddCloth from "./addCloth.vue";
import CouponSale from './couponSale.vue';

const props = defineProps({
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
    },
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

// 订单列表
const ordersList = ref([]);
// 用户列表，创建/更新订单时选择框使用
const userList = ref([]);
const userListRes = ref([]);
// 用户卡券列表
const userCouponList = ref([]);
// 通知模板列表
const priceList = ref([]);
const showCreateUser = ref(false);
const showPaymentDialog = ref(false);
const showCouponSale = ref(false);
const title = ref("");
const totalPrice = ref(0);
// 差价
const priceDiff = ref(0);
const couponStorageCardId = ref([]);

const currentOrderId = ref(props.orderId);
const currentUserId = ref(props.userId);

let isInitialWatchFired = false;
const ordersRef = ref();
/* 单据打印数量 */
const printCount = ref(1);
const phoneRegex = /^1[3-9]\d{9}$/;

const data = reactive({
    form: {
        cloths: [],
        adjust: {}
    },
    paymentForm: {},
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

/* 监听form.cloths变动 */
watch(() => form.value.cloths, (newVal) => {
    if(!form.value.userId){
        return;
    }
    console.log('form.cloths changed:', newVal);
    checkCoupon();
    if (form.value.adjust.totalAmount) {
        proxy.$modal.msgWarning('衣物列表发生变动，请重新填写订单金额');
        form.adjust.totalAmount = null;
    }
    adjustInput();
});

// 处理价格radio 选中事件
function priceChange(event, priceId) {
    event.preventDefault();
    form.value.priceId = form.value.priceId === priceId ? null : priceId;
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
    listUserCoupon({ userId: form.value.userId }).then(response => {
        userCouponList.value = response.rows;
        checkCoupon();
    });
    showCouponSale.value = false;
}

function handleShowCouponSale() {
    showCouponSale.value = true;
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
        for (const cloth of form.value.cloths) {
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

// 取消按钮
function cancelSelf() {
    // 检查是否有未保存的数据
    if (!form.value.userId) {
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

/** 新增按钮操作 */
function handleAdd() {
    reset();
    title.value = "添加洗护服务订单";
    // 获取预计完成时间
    getConfigKey('desire_complete_time').then(res => {
        form.value.desireCompleteTime = getFutureDate(res.msg);
    });
    listUserWithNoLimit().then(res => {
        userList.value = res.rows;
    });
    // 获取价格列表
    listPrice({ orderType: form.value.source, status: 0 }).then(res => {
        priceList.value = res.rows;
    });
}

/** 修改按钮操作 */
function handleUpdate() {
    reset();
    // 获取订单内容
    getOrders(currentOrderId.value).then(response => {
        form.value = response.data;
        form.value.cloths = [];
        if (!form.value.adjust) {
            form.value.adjust = {};
        }
        title.value = "修改服务订单";
        // 获取价格列表
        listPrice({ orderType: form.value.source }).then(res => {
            priceList.value = res.rows;
        });
    });

    listUserWithNoLimit().then(res => {
        userList.value = res.rows;
        userListRes.value = userList.value;
    });
    // 获取用户卡券列表
    listUserCoupon({ userId: currentUserId.value }).then(response => {
        userCouponList.value = response.rows;
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
            form.value.clothsIds = form.value.cloths.map(item => item.clothId);
            if (form.value.adjust.adjustValueAdd || form.value.adjust.adjustValueSub || form.value.adjust.totalAmount) {
                form.value.adjust.orderId = form.value.orderId;
            }
            console.log('user info:', showCreateUser.value, form.value.userId, form.value.nickName)
            if (showCreateUser.value) {
                try {
                    const res = await addUser({
                        phonenumber: form.value.userId,
                        nickName: form.value.nickName
                    });

                    form.value.userId = res.data; // 设置返回的用户ID
                } catch (err) {
                    proxy.$modal.msgError(err);
                    return; // 当 addUser 出错时，中断执行
                }
            }
            if (form.value.orderId != null) {
                updateOrders(form.value).then(response => {
                    proxy.$modal.msgSuccess("修改成功");
                    props.refresh();
                    props.toggle();
                });
            } else {
                addOrders(form.value).then(response => {
                    proxy.$modal.msgSuccess("新增成功");
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
    proxy.$refs["ordersRef"].validate(valid => {
        if (valid) {
            if (!form.value.cloths || form.value.cloths.length == 0) {
                proxy.$modal.msgError("衣物信息不能为空");
                return;
            }
            // 如果是创建订单，那么要先创建订单，拿到订单编码
            console.log(!form.value.orderId)
            if (!form.value.orderId) {

                form.value.clothsIds = form.value.cloths.map(item => item.clothId);

                proxy.$modal.loading("正在创建订单，请稍候");
                addOrders(form.value).then(response => {
                    proxy.$modal.closeLoading();
                    form.value.orderId = response.data.orderId;
                    form.value.orderNumber = response.data.orderNumber;
                    // 初始化支付所需数据
                    initPaymentForm();
                    // getList();
                    props.refresh();
                    showPaymentDialog.value = true;
                });
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
        ucOrderId: form.value.orderId,
        payNumber: form.value.orderNumber,
        paymentMethod: '02',
        orderType: '1',
        totalAmount: totalPrice.value,
    }

}

/** 按手机号搜索会员 */
function searchUserByTel(tel) {
    userListRes.value = userList.value.filter(item => item.phonenumber.includes(tel));
    if (userListRes.value.length == 0) {
        showCreateUser.value = true;
        form.value.nickName = null;
        userCouponList.value = [];
    } else {
        if (userListRes.value.length == 1) {
            form.value.nickName = userListRes.value[0].nickName;
            // 查询会员卡券信息
            listUserCoupon({ userId: form.value.userId }).then(response => {
                userCouponList.value = response.rows;
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
    const item = userList.value.find(item => { return item.userId === userId });
    form.value.nickName = item.nickName;
    // 查询会员卡券信息
    listUserCoupon({ userId: userId }).then(response => {
        userCouponList.value = response.rows;
    });
}

/* 获取有效期tooltip 的content */
function getValidTime(validFrom, validTo) {
    return `有效期：${validFrom} ~ ${validTo}`;
}

/* 打印单据 */
function printOrder() {
    proxy.$modal.msgSuccess("正在打印单据...");
}

/* 调价输入框输入事件 */
function adjustInput() {
    if (form.value.adjust.totalAmount) {
        totalPrice.value = form.value.adjust.totalAmount;
    } else {
        // 如果选择了价格item，那么使用价格item中的价格代替衣物价格
        let price;
        if (form.value.priceId) {
            const item = priceList.value.find(item => item.priceId === form.value.priceId);
            price = item ? item.priceValue : 0;
        } else {
            price = form.value.cloths.reduce((acc, cur) => {
                return acc +
                    cur.priceValue + cur.processMarkup
            }, 0);
        }
        price +=
            Number(form.value.adjust.adjustValueAdd ? form.value.adjust.adjustValueAdd : 0) -
            Number(form.value.adjust.adjustValueSub ? form.value.adjust.adjustValueSub : 0);
        totalPrice.value = price > 0 ? price : 0;
    }
}

if (props.orderId !== 0) {
    handleUpdate();
} else {
    handleAdd();
}

defineExpose({
    cancel,
});
</script>

<style scoped>
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
</style>