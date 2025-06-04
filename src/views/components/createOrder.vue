<template>
    <div style="width: 100%; height: 100%;">
        <!-- 添加或修改洗护服务订单对话框 -->
        <div class="container" v-if="props.visible">
            <div class="left">
                <el-form ref="ordersRef" :model="form" :rules="rules" label-width="90px" class="modern-form">
                    <div class="member-card" ref="memberCardRef">
                        <el-row :gutter="20" class="member-info">
                            <el-col :span="6">
                                <h3 class="section-title1">会员信息</h3>
                            </el-col>
                            <el-col :span="6" v-if="showUserInfoRow">
                                <div class="info-item">
                                    <div class="info-label">余额</div>
                                    <div class="info-value">{{ currentUser.balance ? currentUser.balance : 0 }}元</div>
                                </div>
                            </el-col>
                            <el-col :span="6" v-if="showUserInfoRow">
                                <div class="info-item">
                                    <div class="info-label">积分</div>
                                    <div class="info-value">{{ currentUser.integral ? currentUser.integral : 0 }}分</div>
                                </div>
                            </el-col>
                            <el-col :span="6" class="info-action" v-if="showUserInfoRow">
                                <el-button type="primary" plain icon="Money" link style="outline: none;"
                                    @click="showCouponSale = true">充值</el-button>
                                <el-button type="primary" plain icon="DArrowRight" link style="outline: none;"
                                    @click="showInfoDialog = true">详情</el-button>
                            </el-col>
                        </el-row>
                        <el-row :gutter="20">
                            <el-col :span="12">
                                <el-form-item label="手机号：" prop="userId">
                                    <UserSelect v-model="form.userInfo" :disabled="notEditable"
                                        :user-name="form.nickName" @change="handleUserSelect" @blur="handleBlur"
                                        @need-create-user="handleNeedCreateUserLocal" @update-phone="handleUpdatePhone"
                                        @validate="handleUserValidate" @clear-validation="clearUserValidation"
                                        ref="userSelectRef" />
                                </el-form-item>
                            </el-col>
                            <el-col :span="12">
                                <el-form-item label="姓名：" prop="nickName">
                                    <el-input :disabled="notEditable" size="large" v-model="form.nickName"
                                        placeholder="请输入会员姓名">
                                        <template #prefix>
                                            <el-icon>
                                                <User />
                                            </el-icon>
                                        </template>
                                    </el-input>
                                </el-form-item>
                            </el-col>
                        </el-row>
                        <el-row :gutter="20" v-if="showUserInfoRow && mergedCoupons.length > 0">
                            <el-col :span="24">
                                <el-form-item class="coupon-tags-wrapper">
                                    <div class="coupon-tags">
                                        <el-tag
                                            v-for="(card, index) in mergedCoupons.filter(item => item.coupon.couponType == 'SessionCard' && item.availableValue > 0)"
                                            :key="index" type="success" effect="light" class="coupon-tag">
                                            {{ card.coupon.couponTitle }} - {{ card.ucCount }}张
                                        </el-tag>
                                        <el-tag
                                            v-for="(card, index) in mergedCoupons.filter(item => item.coupon.couponType !== 'StoredValueCard' && item.coupon.couponType !== 'SessionCard' && item.availableValue > 0)"
                                            :key="index" type="warning" effect="light" class="coupon-tag">
                                            {{ card.coupon.couponTitle }} - {{ card.ucCount }}张
                                        </el-tag>
                                    </div>
                                </el-form-item>
                            </el-col>
                        </el-row>
                    </div>
                    <div class="order-source-card" ref="orderSourceRef">
                        <h3 class="section-title">订单来源</h3>
                        <el-form-item prop="source">
                            <el-radio-group v-model="form.source" @change="sourceChanged" :disabled="notEditable"
                                class="modern-radio-group">
                                <el-radio v-for="dict in OrderSource" :key="dict.value" :label="dict.label"
                                    :value="dict.value" class="payment-method-radio">
                                    <div class="payment-method-card"
                                        :class="{ 'selected': form.source === dict.value }">
                                        <el-icon>
                                            <component :is="dict.icon" />
                                        </el-icon>
                                        <span>{{ dict.label }}</span>
                                    </div>
                                </el-radio>
                            </el-radio-group>
                        </el-form-item>
                        <div class="price-section"
                            v-if="(form.priceIds && form.priceIds.length > 0) || priceList.length > 0">
                            <!-- <div style="font-size: small;">价格方案</div> -->
                            <el-form-item props="priceIds">
                                <el-checkbox-group v-model="form.priceIds" :disabled="notEditable"
                                    class="modern-radio-group price-list">
                                    <el-checkbox v-for="item in priceList" class="payment-method-radio"
                                        @change="(event) => priceChange(event, item.priceId)" :key="item.priceId"
                                        :label="item.priceId">
                                        <div class="payment-method-card" :class="{
                                            'selected': form.priceIds && form.priceIds.includes(item.priceId),
                                            'discount-type': isPriceDiscount(item),
                                            'fixed-price-type': !isPriceDiscount(item)
                                        }">
                                            <el-icon v-if="isPriceDiscount(item)">
                                                <Discount />
                                            </el-icon>
                                            <el-icon v-else>
                                                <Money />
                                            </el-icon>
                                            <el-tooltip :content="getPriceTooltip(item)" placement="top"
                                                :show-after="200">
                                                <span ref="priceNameSpan">{{ item.priceName }}</span>
                                            </el-tooltip>
                                        </div>
                                    </el-checkbox>
                                </el-checkbox-group>
                            </el-form-item>
                        </div>
                    </div>
                    <div class="order-list-card" ref="clothListRef">
                        <h3 class="section-title">衣物列表</h3>
                        <CustomTable :table-data="form.cloths" @delete="handleDelete" :disabled="notEditable"
                            @selected="handleClothSelected" />
                    </div>
                    <div class="order-list-card" ref="adjustPriceRef">
                        <h3 class="section-title">店主调价</h3>

                        <div class="adjust-price-group">
                            <div class="adjust-price-group-mask" v-if="form.priceIds && form.priceIds.length > 0">
                                使用了价格方案后不能调价</div>
                            <el-input type="number" :min="0" :max="1000" @input="adjustInput" clearable
                                controls-position="right" @change="adjustInputChange"
                                v-model="form.adjust.adjustValueSub" placeholder="请输入调减金额"
                                :disabled="(form.priceIds && form.priceIds.length > 0) || notEditable" />
                            <el-input type="number" :min="0" :max="1000" @input="adjustInput" clearable
                                controls-position="right" @change="adjustInputChange"
                                v-model="form.adjust.adjustValueAdd" placeholder="请输入调增金额"
                                :disabled="(form.priceIds && form.priceIds.length > 0) || notEditable" />
                            <el-input type="number" :min="0" :max="Infinity" @input="adjustInput" clearable
                                controls-position="right" @change="adjustInputChange" v-model="form.adjust.adjustTotal"
                                placeholder="请输入总金额"
                                :disabled="(form.priceIds && form.priceIds.length > 0) || notEditable" />
                            <el-input v-model="form.adjust.remark" placeholder="备注信息" @change="adjustInputChange"
                                :disabled="(form.priceIds && form.priceIds.length > 0) || notEditable" />
                        </div>
                    </div>
                    <div class="order-summary-card" ref="orderSummaryRef">
                        <el-row :gutter="20" class="footer">
                            <el-col :xs="24" :sm="8">
                                <div class="summary-item">
                                    <div class="summary-label">总件数</div>
                                    <div class="summary-value">{{ form.cloths.length }}</div>
                                </div>
                            </el-col>
                            <el-col :xs="24" :sm="8">
                                <div class="summary-item">
                                    <div class="summary-label">预计取衣</div>
                                    <div class="summary-value">{{ form.desireCompleteTime }}</div>
                                </div>
                            </el-col>
                            <el-col :xs="24" :sm="8">
                                <div class="summary-item">
                                    <div class="summary-label">单据打印</div>
                                    <div class="summary-value">
                                        <el-input-number style="width: 100%;" :min="1" v-model="printCount"
                                            controls-position="right" />
                                    </div>
                                </div>
                            </el-col>
                        </el-row>
                    </div>
                </el-form>
                <div class="left-footer">
                    <div class="total-price">
                        <span class="price-label">总价</span>
                        <span class="price-value">{{ totalPrice }}元</span>
                    </div>
                    <div class="btn-container">
                        <el-button icon="Close" type="danger" @click="cancelSelf">{{ form.orderId ? '关 闭' :
                            '取 消'
                        }}</el-button>
                        <el-button icon="Check" type="primary" color="#626aef" @click="submitForm"
                            :disabled="notEditable || (!(form.source === 'Store') && (!form.priceIds || form.priceIds.length === 0))"
                            v-if="form.source !== 'Meituan' && form.source !== 'Douyin'"
                            ref="submitButtonRef">取衣收款</el-button>
                        <el-button type="success" @click="createAndGo2Pay" icon="Money" :disabled="notEditable"
                            ref="payButtonRef">收衣收款</el-button>
                    </div>
                </div>
            </div>
            <div class="right" :span="14" ref="addClothRef">
                <div v-if="notEditable" class="non-editable-container">
                    <OrderNonEditableMessage :order="form" :totalClothes="form.cloths.length"
                        :totalPrice="totalPrice" />
                </div>
                <AddCloth v-else :userId="form.userId" :orderId="form.orderId" :submit="submitClothes"
                    :disabled="notEditable" :key="form.userId || 'no-user'" :clothes="form.cloths" />
            </div>
        </div>
        <!-- 卡券购买弹窗 -->
        <el-dialog v-model="showCouponSale" width="800px" lock-scroll modal align-center :close-on-click-modal="false"
            :show-close="false">
            <CouponSale :userId="currentUser.userId" :key="showCouponSale"
                :taggle="() => { showCouponSale = !showCouponSale }" :visible="showCouponSale"
                :couponTypeList="couponTypeList" :submit="() => showCouponSale = false" />
        </el-dialog>
        <!-- 支付弹窗 -->
        <Pay :visible="showPaymentDialog" :key="showPaymentDialog" :order="form" :refresh="reset"
            :toggle="() => { showPaymentDialog = !showPaymentDialog }" :createOrder="createAndPay"
            :userCouponList="userCouponList" :couponTypeList="couponTypeList" :showPickupButton="false"
            @payment-success="handlePaymentSuccess" @payment-failed="handlePaymentFailed" />
        <Information :user="currentUser" :visible="showInfoDialog" :key="showInfoDialog"
            :toggle="() => { showInfoDialog = !showInfoDialog }" />

        <!-- <OrderTourGuide 
            :memberCardRef="memberCardRef" 
            :orderSourceRef="orderSourceRef"
            :clothListRef="clothListRef"
            :adjustPriceRef="adjustPriceRef"
            :orderSummaryRef="orderSummaryRef"
            :addClothRef="addClothRef"
            :submitButtonRef="submitButtonRef"
            :payButtonRef="payButtonRef"
            @tour-finished="handleTourFinished"
        /> -->
    </div>
</template>

<script setup name="CreateOrders">
import { ref, reactive, computed, onMounted, onUnmounted, nextTick, toRefs, getCurrentInstance } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import CouponSale from './couponSale.vue';
import { ElMessageBox } from 'element-plus'
import { getOrders, addOrders, updateOrders, updateAdjust } from "@/api/system/orders";
import { listPrice } from "@/api/system/price";
import { delCloths } from "@/api/system/cloths";
import { listCloths } from "@/api/system/cloths";
import { getFutureDate } from "@/utils";
import { getConfigKey } from '@/api/system/config';
import { getUser } from '@/api/system/user';
import AddCloth from "./addCloth.vue";
import Information from "@/views/frontend/user/information.vue";
import CustomTable from '@/components/CustomTable';
import Pay from '@/views/components/pay.vue';
import eventBus from "@/utils/eventBus";
import UserSelect from '@/components/UserSelect.vue';
import OrderNonEditableMessage from '@/components/OrderNonEditableMessage.vue';
// import OrderTourGuide from '@/components/OrderTourGuide/index.vue';
import { OrderSource, PaymentMethodMap } from "@/constants";

// 导入抽离的composable函数
import { useOrderCalculation } from '@/composables/useOrderCalculation';
import { useUserManagement } from '@/composables/useUserManagement';
import { usePrinting } from '@/composables/usePrinting';
import { useOrderValidation } from '@/composables/useOrderValidation';

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

const router = useRouter();
const route = useRoute();

// 使用composables
const {
    currentUser,
    showCreateUser,
    userCouponList,
    mergedCoupons,
    couponTypeList,
    phoneRegex,
    selectUser,
    handleNeedCreateUser,
    handleUpdatePhone,
    createUser,
    handleCouponPurchase,
    getUserCoupons
} = useUserManagement();

const {
    totalPrice,
    isPriceDiscount,
    getPriceTooltip,
    calculateTotalPrice,
    handlePriceChange
} = useOrderCalculation();

const {
    printCloth,
    printReceipt
} = usePrinting();

// 显示的弹窗
const showDialog = ref(false);
const showPaymentDialog = ref(false);
const showCouponSale = ref(false);
const showInfoDialog = ref(false);

// 价格列表
const priceList = ref([]);

const currentOrderId = ref(props.orderId);
const currentUserId = ref(props.userId);
const notEditable = ref(false);

/* 单据打印数量 */
const printCount = ref(1);

// 引导组件需要的ref
const memberCardRef = ref(null);
const orderSourceRef = ref(null);
const clothListRef = ref(null);
const adjustPriceRef = ref(null);
const orderSummaryRef = ref(null);
const addClothRef = ref(null);
const submitButtonRef = ref(null);
const payButtonRef = ref(null);
const userSelectRef = ref(null);
const ordersRef = ref();

// 使用验证hook
const {
    rules,
    handleUserValidate,
    handleBlur,
    clearUserValidation,
    validateOrderForm
} = useOrderValidation({
    userSelectRef,
    showCreateUser,
    currentUser
});

const data = reactive({
    form: {
        cloths: [],
        adjust: {},
        priceIds: [],
        orderId: null,
        orderNumber: null,
        businessType: null,
        userId: null,
        userInfo: null,
        desireCompleteTime: null,
        costTimeAlarm: null,
        pickupCode: null,
        completeTime: null,
        deliveryMode: "00",
        source: "Store",
        status: null,
        paymentStatus: null,
        remark: null,
        orderType: null,
        originalPrice: null,
        createTime: null,
        updateTime: null
    },
    refundForm: {},
    notifyForm: {}
});

const { form } = toRefs(data);

const showUserInfoRow = computed(() => {
    return form.value.userId &&
        Object.keys(currentUser.value).length > 0 &&
        currentUser.value.userId &&
        !showCreateUser.value;
});

// 处理子组件传过来的数据
function submitClothes(cloth) {
    const index = form.value.cloths.findIndex(item => item.clothId === cloth.clothId);
    if (index === -1) {
        form.value.cloths.push(cloth);
    } else {
        form.value.cloths[index] = cloth;
    }

    adjustInput();
}

// 处理价格radio 选中事件
function priceChange(event, priceId) {
    handlePriceChange(event, priceId, form.value, priceList.value);
    adjustInput();
}

// 调整金额输入
function adjustInput() {
    totalPrice.value = calculateTotalPrice(form.value, priceList.value);
}

function adjustInputChange() {
    // 如果是修改操作，那么触发更新请求
    if (form.value.orderId && form.value.orderId !== 0) {
        updateAdjust(form.value).catch(res => {
            proxy.notify.error(res.msg);
        });
    }
}

// 取消按钮
function cancel(returnPromise = true) {
    const performCancel = (resolve, reject) => {
        // 检查是否有未保存的数据
        if (!form.value.userId) {
            reset();
            showDialog.value = true;
            
            if (returnPromise) {
                resolve && resolve(true); // 确认取消
            } else {
                props.toggle();
            }
            return;
        }

        // 修改操作不允许反悔
        if (form.value.orderId) {
            reset();
            showDialog.value = true;
            props.toggle();
            if (returnPromise) {
                resolve && resolve(true);
            }
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
                            showDialog.value = true;
                            props.toggle();
                            if (returnPromise) {
                                resolve && resolve(true); // 允许关闭
                            }
                        })
                        .catch(res => {
                            console.error(res);
                            if (returnPromise) {
                                reject && reject(false); // 出现错误，不允许关闭
                            }
                        });
                } else {
                    reset();
                    showDialog.value = true;
                    props.toggle();
                    if (returnPromise) {
                        resolve && resolve(true); // 允许关闭
                    }
                }
            })
            .catch(() => {
                // 用户取消操作，不关闭对话框
                if (returnPromise) {
                    reject && reject(false); // 拒绝关闭
                }
            });
    };

    if (returnPromise) {
        return new Promise((resolve, reject) => {
            performCancel(resolve, reject);
        });
    } else {
        performCancel();
    }
}

// 表单重置
function reset() {
    console.log("reset");
    form.value = {
        adjust: {},
        cloths: [],
        orderId: null,
        priceIds: [],
        orderNumber: null,
        businessType: null,
        userId: null,
        userInfo: null,
        desireCompleteTime: null,
        costTimeAlarm: null,
        pickupCode: null,
        completeTime: null,
        deliveryMode: "00",
        source: "Store",
        status: null,
        paymentStatus: null,
        remark: null,
        orderType: null,
        originalPrice: null,
        createTime: null,
        updateTime: null
    };
    totalPrice.value = 0;
    showDialog.value = false;
    notEditable.value = false;
    showCreateUser.value = false;
    currentOrderId.value = props.orderId;
    currentUserId.value = props.userId;
    currentUser.value = {};
    proxy.resetForm("ordersRef");
    sourceChanged();
}

// 监听订单来源变化
function sourceChanged() {
    // 获取价格列表
    listPrice({ orderType: form.value.source, status: 0 }).then(res => {
        priceList.value = res;
        form.value.priceIds = [];
        adjustInput();
    });
}

/** 新增按钮操作 */
function handleAdd() {
    reset();
    // 获取预计完成时间
    getConfigKey('desire_complete_time').then(res => {
        const days = res ? Number(res.configValue) : 7;
        form.value.desireCompleteTime = getFutureDate(days);
    });

    // 获取价格列表
    listPrice({ orderType: form.value.source, status: 0 }).then(res => {
        priceList.value = res;
    });
}

/** 修改按钮操作 */
async function handleUpdate() {
    reset();
    // 获取订单内容
    await getOrders(currentOrderId.value).then(response => {
        form.value = response;
        form.value.cloths = [];
        // 如果订单已支付或已退单，设置为不可编辑状态
        if (form.value.paymentStatus === 'Paid' || form.value.status === 'Cancelled' || form.value.status === 'Refunded') {
            notEditable.value = true;
        }
        if (!form.value.adjust) {
            form.value.adjust = {};
        }
        // 确保priceIds字段是数组
        if (!form.value.priceIds) {
            form.value.priceIds = [];
        }
    });

    // 获取衣物列表
    await listCloths({ orderId: props.orderId }).then(res => {
        res.map(item => {
            if (item.estimate) {
                item.estimateArr = item.estimate.split(',').map(Number);
            }
            if (item.clothingFlaw) {
                item.clothingFlawArr = item.clothingFlaw.split(',').map(Number);
            }
        })
        form.value.cloths = res;
    })

    // 获取价格列表
    await listPrice({ orderType: form.value.source }).then(res => {
        priceList.value = res;
    });

    // 获取用户信息
    await getUser(form.value.userId).then(res => {
        currentUser.value = res;
        // 设置userInfo
        form.value.userInfo = res;
    });

    // 获取用户卡券列表
    await getUserCoupons(currentUserId.value);

    // 计算总价
    adjustInput();
}

/** 提交按钮 */
async function submitForm() {
    // 手动设置验证触发类型为submit
    const validateOptions = { trigger: 'submit' };

    // 检查订单是否已支付或退单
    if (form.value.orderId && (form.value.paymentStatus === 'Paid' || form.value.status === 'Cancelled' || form.value.status === 'Refunded')) {
        proxy.notify.error("订单已支付或已退单，不能修改信息");
        return;
    }

    try {
        // 验证表单
        await validateOrderForm(form.value, ordersRef.value, proxy, validateOptions);

        form.value.clothIds = form.value.cloths.map(item => item.clothId);
        if (form.value.adjust.adjustValueAdd || form.value.adjust.adjustValueSub || form.value.adjust.adjustTotal) {
            form.value.adjust.orderId = form.value.orderId;
        }

        // set phonenumber
        form.value.phonenumber = currentUser.value.phonenumber;

        if (showCreateUser.value) {
            try {
                await createUser(form.value);
            } catch (err) {
                proxy.notify.error(err);
                return;
            }
        }

        if (form.value.orderId != null) {
            updateOrders(form.value).then(async () => {
                proxy.notify.success("修改成功");
                // 打印衣物标签和小票并发执行
                const printClothPromise = printCloth(form.value.cloths, currentUser.value, form.value, proxy);
                const printReceiptPromise = printReceipt(form.value, '未付款', totalPrice.value, proxy);

                await Promise.all([printClothPromise, printReceiptPromise]);
                props.refresh();
                props.toggle();
            });
        } else {
            addOrders(form.value).then(async (response) => {
                proxy.notify.success("订单创建成功");
                form.value.orderNumber = response.orderNumber;
                // 打印衣物标签和小票并发执行
                try {
                    const printClothPromise = printCloth(form.value.cloths, currentUser.value, form.value, proxy);
                    const printReceiptPromise = printReceipt(form.value, '未付款', totalPrice.value, proxy);

                    await Promise.all([printClothPromise, printReceiptPromise]);
                } catch (error) {
                    console.error('打印失败:', error);
                    proxy.notify.error("打印失败");
                } finally {
                    // 先刷新订单列表
                    props.refresh();

                    // 重置表单数据
                    handleAdd();
                }
            }).catch(error => {
                console.error('创建订单失败:', error);
                proxy.notify.error("创建订单失败: " + (error.message || '未知错误'));
            });
        }
    } catch (error) {
        console.error(error);
    }
}

async function createAndGo2Pay() {
    // 检查订单是否已支付或退单
    if (form.value.orderId && (form.value.paymentStatus === 'Paid' || form.value.status === 'Cancelled' || form.value.status === 'Refunded')) {
        proxy.notify.error("订单已支付或已退单，不能修改信息");
        return;
    }

    // 提交订单
    try {
        // 验证表单
        await validateOrderForm(form.value, ordersRef.value, proxy);

        if (showCreateUser.value) {
            try {
                await createUser(form.value);
                await getUserCoupons(form.value.userId);
            } catch (err) {
                proxy.notify.error(err);
                return;
            }
        }
        showPaymentDialog.value = true;
    } catch (error) {
        console.error(error);
    }
}

/* 收衣收款 */
async function createAndPay(callback) {
    // 手动设置验证触发类型为submit
    const validateOptions = { trigger: 'submit' };

    // 检查订单是否已支付或退单
    if (form.value.orderId && (form.value.paymentStatus === 'Paid' || form.value.status === 'Cancelled' || form.value.status === 'Refunded')) {
        proxy.notify.error("订单已支付或已退单，不能修改信息");
        return;
    }

    try {
        // 验证表单
        await validateOrderForm(form.value, ordersRef.value, proxy, validateOptions);

        form.value.phonenumber = currentUser.value.phonenumber;

        // 如果是创建订单，那么要先创建订单，拿到订单编码
        if (!form.value.orderId) {
            form.value.clothIds = form.value.cloths.map(item => item.clothId);

            proxy.$modal.loading("正在创建订单，请稍候");
            try {
                const response = await addOrders(form.value);
                proxy.$modal.closeLoading();
                if (!response.adjust) {
                    response.adjust = {};
                }
                form.value.orderId = response.orderId;
                callback(response);
                form.value.orderNumber = response.orderNumber;
                // 确保订单的总价与前端计算的一致，特别是当使用价格方案时
                form.value.totalPrice = totalPrice.value;
                // 截断为两位小数（不四舍五入）
                form.value.totalPrice = Math.floor(form.value.totalPrice * 100) / 100;
            } catch (e) {
                console.error(e);
            } finally {
                proxy.$modal.closeLoading();
            }
        } else {
            // 确保订单的总价与前端计算的一致，特别是当使用价格方案时
            form.value.totalPrice = totalPrice.value;
            // 截断为两位小数（不四舍五入）
            form.value.totalPrice = Math.floor(form.value.totalPrice * 100) / 100;
        }
    } catch (error) {
        console.error(error);
    }
}

// 处理支付成功回调
async function handlePaymentSuccess(paymentInfo) {
    try {
        // 打印衣物标签和小票并发执行
        const printClothPromise = printCloth(form.value.cloths, currentUser.value, form.value, proxy);
        const printReceiptPromise = printReceipt(
            form.value,
            PaymentMethodMap[paymentInfo.paymentMethod]?.label || '未知',
            paymentInfo.amount,
            proxy
        );

        await Promise.all([printClothPromise, printReceiptPromise]);

        // 刷新订单列表
        props.refresh();
        // 重置表单
        reset();
    } catch (error) {
        console.error('打印失败:', error);
        proxy.notify.error("打印失败");
    }
}

// 处理支付失败回调
function handlePaymentFailed(error) {
    console.error('支付失败:', error);
    proxy.notify.error("支付失败: " + (error.message || '未知错误'));
}

function handleDelete(clothId, name) {
    // If the order is not editable (paid or refunded), don't allow deletion
    if (notEditable.value) {
        proxy.notify.error("订单已支付或已退单，不能删除衣物");
        return;
    }

    const title = name ? name : clothId;
    proxy.$modal.confirm('是否确认删除订单包含的衣物"' + title + '"？').then(function () {
        return delCloths(clothId);
    }).then(() => {
        const index = form.value.cloths.findIndex(item => item.clothId === clothId);
        form.value.cloths.splice(index, 1);
        // 重新计算总价
        adjustInput();
        eventBus.emit('cloth-deleted', clothId);

        proxy.notify.success("删除成功");
    }).catch(() => { });
}

const handleRouteLeave = (to, from, next) => {
    if (!form.value.userId) {
        reset();
        next();
        return;
    }

    // 修改操作不允许反悔
    if (form.value.orderId) {
        reset();
        next();
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
                        next();
                    })
                    .catch(res => {
                        console.error(res);
                    });
            } else {
                reset();
                next();
            }
        })
        .catch(() => {
            // 用户取消操作，不关闭对话框
            next(false);
        });
};

// 取消按钮（自身）
function cancelSelf() {
    cancel(false);
}

function handleClothSelected(cloth) {
    eventBus.emit('cloth-selected', cloth);
}

// 处理用户选择，添加了用户ID的reactive更新
async function handleUserSelect(val) {
    await selectUser(val, form.value);
    if (val && typeof val === 'object' && val.userId) {
        currentUserId.value = val.userId;
        // 确保form.userId被正确设置，这对AddCloth组件非常关键
        form.value.userId = val.userId;
    }
}

// 处理需要创建用户的情况
function handleNeedCreateUserLocal(phoneNumber) {
    handleNeedCreateUser(phoneNumber, form.value);

    // 确保form.userId设置了一个临时值，让AddCloth组件可以正常显示
    if (form.value.userId === -999) {
        // 强制触发AddCloth组件的重新渲染
        nextTick(() => {
            // 如果需要，可以添加额外的处理
        });
    }
}

onMounted(async () => {
    router.beforeEach((to, from, next) => {
        if (from.path === route.path) {
            handleRouteLeave(to, from, next);
        } else {
            next();
        }
    });
    if (props.visible) {
        if (props.orderId !== 0) {
            handleUpdate();
        } else {
            handleAdd();
        }
        showDialog.value = true;
    }

    // 监听卡券购买成功事件
    eventBus.on('coupon-purchase-success', (data) => handleCouponPurchase(data, form.value));
});

// 组件卸载时移除事件监听
onUnmounted(() => {
    eventBus.off('coupon-purchase-success', handleCouponPurchase);
});

defineExpose({
    cancel,
});
</script>

<style scoped>
.container {
    height: 100%;
    width: 100%;
    margin: 0;
    position: absolute;
    left: 0;
    top: 0;
    background-color: var(--el-bg-color-page);
    padding: 1.5rem;
    display: flex;
    gap: 1.5rem;
}

.left,
.right {
    background-color: var(--el-bg-color);
    border-radius: 12px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
    position: relative;
    overflow: hidden;
    width: 100%;
    height: 100%;
    transition: all 0.3s ease;
}

.left {
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
}

/* 在表单和按钮容器之间添加内容容器 */
.left>.el-form {
    flex: 1;
    overflow-y: auto;
    /* 下面是解决overflow导致的阴影消失问题 */
    padding: 0 20px;
    /* 增加水平内边距 */
    margin: 0 -20px;
    /* 负外边距抵消宽度变化 */
    clip-path: none !important;
    /* 禁用潜在剪切路径 */
    /* 使表单区域可滚动 */
    margin-bottom: 1.25rem;
    contain: paint;

    /* 隐藏滚动条 */
    &::-webkit-scrollbar {
        display: none;
    }
}

.adjust-price-group {
    width: 100%;
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    align-items: center;
    gap: 1.5rem;
    position: relative;
    background-color: var(--el-fill-color-light);
    padding: 1rem;
    border-radius: 8px;
}

.adjust-price-group-mask {
    width: 100%;
    height: 100%;
    position: absolute;
    left: 0;
    top: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    color: var(--el-text-color-placeholder);
    background-color: var(--el-bg-color-overlay);
    opacity: 0.9;
    z-index: 9;
    backdrop-filter: blur(10px);
}

.footer {
    padding: 1.5rem;
    border-radius: 8px;
    background-color: var(--el-fill-color-light);
}

.total-price {
    width: 100%;
    display: flex;
    justify-content: flex-start;
    align-items: center;
    padding: 1rem;
    background-color: var(--el-color-primary-light-9);
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
    transition: all 0.3s ease;
}

.total-price:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
}

:root.dark .total-price {
    --el-color-primary-light-9: #1d2c40;
    /* 自定义暗黑模式下的颜色 */
}

.left-footer {
    padding-top: 1.25rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1.25rem;
    position: sticky;
    bottom: 0;
    right: 0;
    width: 100%;
    border-top: 1px solid var(--el-border-color-light);
    background-color: var(--el-fill-color-blank);
    z-index: 10;
    margin-top: auto;
}

.btn-container {
    width: 100%;
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 1.25rem;

    button {
        transition: all 0.3s;
    }

    button:hover {
        transform: translateY(-2px);
    }
}

.payment-footer {
    text-align: center;
    margin-top: 1.5rem;
}

.status-row {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    justify-content: center;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
}

.coupon-times {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;

    .coupon-times-item {
        display: flex;
        gap: 0.75rem;
        align-items: center;
    }
}

.payment-amount {
    color: #f56c6c;
    font-size: 22px;
    font-weight: bold;
    transition: all 0.3s;
}

.el-form-item__label {
    color: var(--el-text-color-primary);
    font-weight: 500;
    font-size: 15px;
}

/* 添加响应式设计 */
@media screen and (max-width: 1200px) {
    .container {
        flex-direction: column;
    }

    .left-footer {
        position: static;
        margin-top: 2rem;
    }
}

@media screen and (max-width: 768px) {
    .container {
        padding: 1rem;
        gap: 1rem;
    }

    .left {
        padding: 1rem;
    }

    .left-footer button {
        min-width: auto;
        height: 2.75rem;
        font-size: 14px;
    }
}

/* 美化表单元素 */
:deep(.el-input__inner),
:deep(.el-select),
:deep(.el-input-number) {
    border-radius: 8px;
    transition: all 0.3s;
}

:deep(.el-input:hover .el-input__inner),
:deep(.el-select:hover .el-input__inner),
:deep(.el-input-number:hover .el-input__inner) {
    border-color: var(--el-color-primary);
}

:deep(.el-radio) {
    margin-right: 20px;
    transition: all 0.3s;
}

:deep(.el-radio:hover) {
    transform: translateY(-2px);
}

:deep(.el-form-item) {
    margin-bottom: 1.5rem;
}

:deep(.price-group .el-radio) {
    margin-bottom: 0.75rem;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    background-color: var(--el-fill-color-light);
}

:deep(.price-group .el-radio.is-checked) {
    background-color: var(--el-color-primary-light-9);
}

/* 新增卡片样式 */
.member-card,
.order-source-card,
.order-list-card,
.order-summary-card,
.total-price-card {
    background-color: var(--el-bg-color);
    border-radius: 12px;
    padding: 1rem 1.5rem;
    margin-bottom: 1.5rem;
    box-shadow: var(--el-box-shadow-lighter);
    transition: all 0.3s ease;
}

.member-card:hover,
.order-source-card:hover,
.order-list-card:hover,
.order-summary-card:hover,
.total-price-card:hover {
    transform: translateY(-2px);
    box-shadow: var(--el-box-shadow);
}

/* 会员信息卡片样式 */
.member-info {
    border-bottom: 1px solid var(--el-border-color-light);
    margin-bottom: 1.25rem;

    .section-title1 {
        font-size: 18px;
        font-weight: 600;
        padding-bottom: 0.75rem;
        margin: 0;
        color: var(--el-color-primary-dark-2);
    }
}

.info-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.info-label {
    font-size: 14px;
    color: var(--el-text-color-secondary);
}

.info-value {
    font-size: 18px;
    font-weight: 600;
    color: var(--el-color-primary);
}

.info-action {
    display: flex;
    align-items: center;
    justify-content: center;
}

.coupon-tags-wrapper {
    margin-bottom: 0;
}

.coupon-tags {
    display: flex;
    flex-wrap: wrap;
    gap: .5rem;
}

.price-list {
    width: 100%;
    max-height: 12rem;
    overflow-y: auto;
    background-color: var(--el-fill-color-lighter);
    padding: 0.5rem 0;
    border-radius: 8px;
    margin-bottom: 0.75rem;
}

/* 订单摘要样式 */
.summary-item {
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem;
    background-color: var(--el-bg-color);
    border-radius: 8px;
    transition: all 0.3s;
}

.summary-item:hover {
    transform: translateY(-2px);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.summary-label {
    font-size: 14px;
    color: var(--el-text-color-secondary);
}

.summary-value {
    font-size: 18px;
    font-weight: 600;
    color: var(--el-text-color-primary);
    overflow: hidden;
}

/* 总价样式 */
.price-label {
    font-size: 16px;
    margin-right: 12px;
    color: var(--el-text-color-regular);
}

.price-value {
    font-size: 24px;
    font-weight: bold;
    color: var(--el-color-danger);
}

/* 现代化表单样式 */
.modern-form :deep(.el-form-item__label) {
    font-weight: 500;
}

.modern-radio-group {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
}

.section-title {
    font-size: 18px;
    font-weight: 600;
    padding-bottom: 0.75rem;
    margin-bottom: 1.25rem;
    border-bottom: 1px solid var(--el-border-color-light);
    color: var(--el-color-primary-dark-2);
}


.payment-method-radio {
    margin-right: 0 !important;
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

/* 新增样式：折扣类型和固定价格类型的视觉区分 */
.payment-method-card.discount-type {
    border-left: 4px solid var(--el-color-warning);
}

.payment-method-card.fixed-price-type {
    border-left: 4px solid var(--el-color-success);
}

.payment-method-card.discount-type .el-icon {
    color: var(--el-color-warning);
}

.payment-method-card.fixed-price-type .el-icon {
    color: var(--el-color-success);
}

.payment-method-card .el-icon {
    font-size: 24px;
    margin-bottom: 8px;
    color: var(--el-color-primary);
}

.payment-method-card span {
    font-size: 14px;
    line-height: 14px;
    width: 100%;
    text-align: center;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    padding: 0 4px;
}

/* 新增不可编辑订单的消息样式 */
.non-editable-container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
    background-color: var(--el-bg-color-page);
    overflow: auto;
    padding: 1rem;
}
</style>