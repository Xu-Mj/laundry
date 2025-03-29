<template>
    <div style="width: 100%; height: 100%;">
        <!-- 添加或修改洗护服务订单对话框 -->
        <div class="container" v-if="props.visible">
            <div class="left">
                <el-form ref="ordersRef" :model="form" :rules="rules" label-width="90px" class="modern-form">
                    <div class="member-card" ref="memberCardRef">
                        <h3 class="section-title">会员信息</h3>
                        <el-row :gutter="20">
                            <el-col :span="12">
                                <el-form-item size="large" label="会员：" prop="userId">
                                    <el-select v-model="form.userId" :disabled="notEditable" filterable
                                        :clearable="true" remote reserve-keyword placeholder="请输入手机号码搜索" allow-create
                                        @blur="handleBlur" remote-show-suffix :remote-method="searchUserByTel"
                                        @visible-change="handleVisibleChange" value-key="userId" style="width: 100%">
                                        <el-option v-for="item in userListRes" :key="item.userId"
                                            :label="item.nickName + '\t' + item.phonenumber" :value="item.userId" />
                                        <template #prefix>
                                            <el-icon>
                                                <Phone />
                                            </el-icon>
                                        </template>
                                    </el-select>
                                </el-form-item>
                            </el-col>
                            <el-col :span="12">
                                <el-form-item size="large" label="姓名：" prop="nickName">
                                    <el-input :disabled="notEditable" v-model="form.nickName" placeholder="请输入会员姓名">
                                        <template #prefix>
                                            <el-icon>
                                                <User />
                                            </el-icon>
                                        </template>
                                    </el-input>
                                </el-form-item>
                            </el-col>
                        </el-row>
                        <el-row :gutter="20" v-if="form.userId" class="member-info">
                            <el-col :span="8">
                                <div class="info-item">
                                    <div class="info-label">余额</div>
                                    <div class="info-value">{{ currentUser.balance ? currentUser.balance : 0 }}元</div>
                                </div>
                            </el-col>
                            <el-col :span="8">
                                <div class="info-item">
                                    <div class="info-label">积分</div>
                                    <div class="info-value">{{ currentUser.integral ? currentUser.integral : 0 }}分</div>
                                </div>
                            </el-col>
                            <el-col :span="8" class="info-action">
                                <el-button type="primary" plain icon="DArrowRight"
                                    @click="showInfoDialog = true">详情</el-button>
                            </el-col>
                        </el-row>
                    </div>
                    <div class="order-source-card" ref="orderSourceRef">
                        <h3 class="section-title">订单来源</h3>
                        <el-form-item prop="source">
                            <el-radio-group v-model="form.source" @change="sourceChanged" :disabled="notEditable"
                                class="modern-radio-group">
                                <el-radio v-for="dict in sys_price_order_type" :key="dict.value" :label="dict.label"
                                    :value="dict.value" class="payment-method-radio">
                                    <div class="payment-method-card"
                                        :class="{ 'selected': form.source === dict.value }">
                                        <el-icon v-if="dict.label === '其他'">
                                            <More />
                                        </el-icon>
                                        <el-icon v-else-if="dict.label === '到店'">
                                            <School />
                                        </el-icon>
                                        <el-icon v-else-if="dict.label === '美团'">
                                            <Food />
                                        </el-icon>
                                        <el-icon v-else-if="dict.label === '抖音'">
                                            <Goods />
                                        </el-icon>
                                        <el-icon v-else-if="dict.label === '小程序'">
                                            <Iphone />
                                        </el-icon>
                                        <el-icon v-else>
                                            <MostlyCloudy />
                                        </el-icon>
                                        <span>{{ dict.label }}</span>
                                    </div>
                                </el-radio>
                            </el-radio-group>
                        </el-form-item>
                        <div class="price-section" v-if="form.priceId || priceList.length > 0">
                            <div style="font-size: small;">价格方案</div>
                            <el-form-item props="priceId">
                                <el-radio-group v-model="form.priceId" :disabled="notEditable"
                                    class="modern-radio-group">
                                    <el-radio v-for="item in priceList"
                                        @click="(event) => priceChange(event, item.priceId)" :key="item.priceId"
                                        :label="item.priceName" :value="item.priceId">
                                        <div class="payment-method-card"
                                            :class="{ 'selected': form.priceId === item.priceId }">
                                            <el-icon>
                                                <Money />
                                            </el-icon>
                                            <span>{{ item.priceName }}</span>
                                        </div>
                                    </el-radio>
                                </el-radio-group>
                            </el-form-item>
                        </div>
                    </div>
                    <div class="order-list-card" ref="clothListRef">
                        <h3 class="section-title">衣物列表</h3>
                        <CustomTable :table-data="form.cloths" @delete="handleDelete" />
                    </div>
                    <div class="order-list-card" ref="adjustPriceRef">
                        <h3 class="section-title">店主调价</h3>

                        <div class="adjust-price-group">
                            <div class="adjust-price-group-mask" v-if="form.priceId">使用了价格方案后不能调价</div>
                            <el-input size="large" type="number" :min="0" :max="1000" @input="adjustInput"
                                @change="adjustInputChange" v-model="form.adjust.adjustValueSub" placeholder="请输入调减金额"
                                :disabled="form.priceId" />
                            <el-input size="large" type="number" :min="0" :max="1000" @input="adjustInput"
                                @change="adjustInputChange" v-model="form.adjust.adjustValueAdd" placeholder="请输入调增金额"
                                :disabled="form.priceId" />
                            <el-input size="large" type="number" :min="0" :max="Infinity" @input="adjustInput"
                                @change="adjustInputChange" v-model="form.adjust.adjustTotal" placeholder="请输入总金额"
                                :disabled="form.priceId" />
                            <el-input size="large" v-model="form.adjust.remark" placeholder="备注信息"
                                @change="adjustInputChange" :disabled="form.priceId" />
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
                                        <el-input-number :min="1" v-model="printCount" controls-position="right"
                                            size="large" />
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
                        <el-button size="large" icon="Close" type="danger" @click="cancelSelf">{{ form.orderId ? '关 闭' :
                            '取 消'
                            }}</el-button>
                        <el-button size="large" icon="Check" type="primary" color="#626aef" @click="submitForm"
                            :disabled="notEditable && !(form.source === '03') && (form.priceId || form.source === '02' || form.source === '01')" ref="submitButtonRef">取衣收款</el-button>
                        <el-button size="large" type="success" @click="createAndPay" icon="Money"
                            :disabled="notEditable" ref="payButtonRef">收衣收款</el-button>
                    </div>
                </div>
            </div>
            <div class="right" :span="14" ref="addClothRef">
                <AddCloth :userId="form.userId" :orderId="form.orderId" :submit="submitClothes" :disabled="notEditable"
                    :key="form.userId" />
            </div>
        </div>

        <Pay :visible="showPaymentDialog" :key="showPaymentDialog" :order="form" :refresh="cancelSelf"
            :toggle="() => { showPaymentDialog = !showPaymentDialog }" />
        <Information :user="currentUser" :visible="showInfoDialog" :key="showInfoDialog"
            :toggle="() => { showInfoDialog = !showInfoDialog }" />
            
        <OrderTourGuide 
            :memberCardRef="memberCardRef" 
            :orderSourceRef="orderSourceRef"
            :clothListRef="clothListRef"
            :adjustPriceRef="adjustPriceRef"
            :orderSummaryRef="orderSummaryRef"
            :addClothRef="addClothRef"
            :submitButtonRef="submitButtonRef"
            :payButtonRef="payButtonRef"
            @tour-finished="handleTourFinished"
        />
    </div>
</template>

<script setup name="CreateOrders">
import { ElMessageBox } from 'element-plus'
import { getOrders, addOrders, updateOrders, updateAdjust } from "@/api/system/orders";
import { listPrice } from "@/api/system/price";
import { listUserWithNoLimit, addUser, getUser } from "@/api/system/user";
import { delCloths } from "@/api/system/cloths";
import { listUserCouponWithValidTime } from '@/api/system/user_coupon';
import { listCloths } from "@/api/system/cloths";
import { getFutureDate } from "@/utils";
import { getConfigKey } from '@/api/system/config';
import AddCloth from "./addCloth.vue";
import { print } from "@/api/system/printer";
import Information from "@/views/system/user/information.vue";
import CustomTable from '@/components/CustomTable';
import Pay from '@/views/components/pay.vue';
import OrderTourGuide from '@/components/OrderTourGuide/index.vue';

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
const { sys_price_order_type } = proxy.useDict("sys_price_order_type");

const router = useRouter();
const route = useRoute();

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
const totalPrice = ref(0);

const currentOrderId = ref(props.orderId);
const currentUserId = ref(props.userId);
const currentUser = ref({});
const ordersRef = ref();
/* 单据打印数量 */
const printCount = ref(1);
const phoneRegex = /^1[3-9]\d{9}$/;

const showInfoDialog = ref(false);

const notEditable = ref(false);
const showCoupons = ref(true);

// 引导组件需要的ref
const memberCardRef = ref(null);
const orderSourceRef = ref(null);
const clothListRef = ref(null);
const adjustPriceRef = ref(null);
const orderSummaryRef = ref(null);
const addClothRef = ref(null);
const submitButtonRef = ref(null);
const payButtonRef = ref(null);

const data = reactive({
    form: {
        cloths: [],
        adjust: {}
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

const { form, rules } = toRefs(data);

// 共享的状态
const selectedCloth = ref(null);

// 提供共享的状态和方法
provide('selectedCloth', selectedCloth);
provide('setSelectedCloth', (cloth) => {
    console.log(cloth)
    selectedCloth.value = cloth;
});

// 处理子组件传过来的数据
function submitClothes(list) {
    form.value.cloths = list;
    adjustInput();
}

// 处理价格radio 选中事件
function priceChange(event, priceId) {
    event.preventDefault();
    form.value.priceId = form.value.priceId === priceId ? null : priceId;
    adjustInput();
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

// 取消按钮
function cancelSelf() {
    // 检查是否有未保存的数据
    if (!form.value.userId) {
        reset();
        showDialog.value = false;
        props.toggle();
        return;
    }

    // 修改操作不允许反悔
    if (form.value.orderId) {
        reset();
        showDialog.value = false;
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
            showDialog.value = true;

            resolve(true); // 确认取消
            return;
        }

        // 修改操作不允许反悔
        if (form.value.orderId) {
            reset();
            showDialog.value = true;
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
                            showDialog.value = true;
                            resolve(true); // 允许关闭
                        })
                        .catch(res => {
                            console.error(res);
                            reject(false); // 出现错误，不允许关闭
                        });
                } else {
                    reset();
                    showDialog.value = true;
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
    totalPrice.value = 0;
    showDialog.value = false;
    notEditable.value = false;
    showCreateUser.value = false;
    currentOrderId.value = props.orderId;
    currentUserId.value = props.userId;
    currentUser.value = {};
    proxy.resetForm("ordersRef");
}

// 监听订单来源变化
function sourceChanged() {
    // 获取价格列表
    listPrice({ orderType: form.value.source, status: 0 }).then(res => {
        console.log('res', res)
        priceList.value = res;
        form.value.priceId = null;
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
    listUserWithNoLimit().then(res => {
        userList.value = res;
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
        if (form.value.paymentStatus == '00' || form.value.status == '05') {
            notEditable.value = true;
        }
        if (!form.value.adjust) {
            form.value.adjust = {};
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
        console.log('create order price list', res)
    });

    // 获取用户信息
    await getUser(form.value.userId).then(res => {
        currentUser.value = res;
    });

    await listUserWithNoLimit().then(res => {
        userList.value = res;
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

    // 计算总价
    adjustInput();
}

/** 提交按钮 */
async function submitForm() {
    proxy.$refs["ordersRef"].validate(async valid => {
        if (valid) {
            if (!form.value.cloths || form.value.cloths.length == 0) {
                proxy.notify.error("衣物信息不能为空");
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
                    proxy.notify.error(err);
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
                    reset();
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
                proxy.notify.error("衣物信息不能为空");
                return;
            }
            // 如果选择了美团或者抖音，那么需要选择价格标签
            if (form.value.source == '01' || form.value.source == '02') {
                if (!form.value.priceId) {
                    proxy.notify.error("请选择价格标签");
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
                    proxy.notify.error(err);
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
                props.refresh();
                showPaymentDialog.value = true;
            } else {
                showPaymentDialog.value = true;
            }

        }
    });
}

/** 按手机号搜索会员 */
function searchUserByTel(tel) {
    userListRes.value = userList.value.filter(item => item.phonenumber.includes(tel));
    if (userListRes.value.length == 0) {
        showCreateUser.value = true;
        form.value.nickName = null;
        form.value.userId = null;
        userCouponList.value = [];
        currentUser.value = {
            phonenumber: tel,
            status: "0",
        };
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
function handleVisibleChange(visible) {
    if (visible && userListRes.value.length === 1) {
        form.value.userId = userListRes.value[0].userId;
        form.value.nickName = userListRes.value[0].nickName;
        selectUser(form.value.userId);
    }
}
/* 选择会员信息 */
async function selectUser(userId) {
    if (!userId || userId.length == 0) {
        form.value.nickName = null;
        return;
    }
    currentUserId.value = userId;
    const item = userList.value.find(item => { return item.userId === userId });
    currentUser.value = await getUser(userId);

    form.value.nickName = item.nickName;
    // 查询会员卡券信息
    await listUserCouponWithValidTime(userId).then(response => {
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
            proxy.notify.error(res.msg);
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
    try {

        proxy.$modal.loading('正在打印衣物信息...')
        await print(result);
    } finally {
        proxy.$modal.closeLoading();
    }
}

function handleDelete(clothId, name) {
    const title = name ? name : clothId;
    proxy.$modal.confirm('是否确认删除订单包含的衣物清单编号为"' + title + '"的数据项？').then(function () {
        return delCloths(clothId);
    }).then(() => {

        const index = form.value.cloths.findIndex(item => item.clothId === clothId);
        form.value.cloths.splice(index, 1);
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


// 引导完成后的处理函数
const handleTourFinished = () => {
    console.log('订单创建引导已完成');
};

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
    display: flex;
    justify-content: space-around;
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

h3 {
    font-size: 20px;
    padding-bottom: 0.75rem;
    margin-bottom: 1.25rem;
    border-bottom: 1px solid var(--el-border-color-light);
    color: var(--el-color-primary-dark-2);
    font-weight: 600;
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
    margin-top: 1rem;
    background-color: var(--el-fill-color-light);
    border-radius: 8px;
    padding: 1rem;
}

.info-item {
    display: flex;
    flex-direction: column;
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
    margin-bottom: 1.25rem;
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

.payment-method-card .el-icon {
    font-size: 24px;
    margin-bottom: 8px;
    color: var(--el-color-primary);
}

.payment-method-card span {
    font-size: 14px;
    line-height: 14px;
}
</style>