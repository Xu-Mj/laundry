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
                <el-button class="hover-flow" type="primary" icon="Search" @click="handleQuery" size="large"
                    round>搜索</el-button>
                <el-button class="hover-flow" icon="Refresh" @click="resetQuery" size="large" round>重置</el-button>
            </el-form-item>
        </el-form>
        <!-- 渲染订单抖索结果列表 -->
        <div class="search-result-list">
            <div v-if="ordersList.length === 0" class="no-result">
                <!-- <h1 style="color: #ccc;">暂无数据</h1> -->
                <el-empty description="暂无数据" />
            </div>
            <div v-slide-in v-else class="result-item" v-for="order in ordersList" :key="order.orderId">
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
                    ref="clothsTableRef" class="modern-table" stripe>
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
                            <span v-if="scope.row.processMarkup && scope.row.processMarkup > 0" class="markup-value">
                                +{{ scope.row.processMarkup }}
                            </span>
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
                            <el-tag v-if="scope.row.hangLocationCode" type="info">
                                {{ scope.row.hangerName + '-' + scope.row.hangerNumber }}
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
            <el-button type="danger" @click="props.taggle()" icon="Close" round>关闭</el-button>
            <el-button type="primary" @click="pickup()" icon="TakeawayBox" round>取衣</el-button>
            <el-button type="success" @click="handlePay" icon="Wallet" round>取衣收款</el-button>
            <el-button type="warning" @click="handleDelivery" icon="Van" round>上门派送</el-button>
            <el-button type="info" @click="() => { }" icon="Printer" round>补打小票</el-button>
        </div>
    </div>

    <!-- 展示照片 -->
    <el-dialog title="照片" v-model="showPicture" width="400px" :align-center="true" append-to-body>
        <div class="img-container">
            <el-image class="img-item" show-progress :zoom-rate="1.2" :max-scale="7" :min-scale="0.2"
                :preview-src-list="pictureList" :src="item" v-for="(item, index) in pictureList" :key="index"
                fit="cover" />
        </div>
    </el-dialog>
    <!-- 派送对话框 -->
    <el-dialog v-model="showDeliveryDialog" width="500px" :show-close="false" :align-center="true" append-to-body>
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

    <!-- 复洗 -->
    <ReWash :visible="showRewashDialog" :order="rewashOrder" :clothes="rewashClothesId"
        :refresh="() => { selectedCloths = []; getList(); }" :key="showRewashDialog"
        :toggle="() => { showRewashDialog = !showRewashDialog }" />

    <PaymentDialog :visible="showPaymentDialog" :user="currentUser" :orders="orders" :cloths-list="clothsList"
        :user-coupon-list="userCouponList" :coupon-type-list="couponTypeList" :refresh="() => { getList(); }"
        :key="showPaymentDialog" :toggle="() => { showPaymentDialog = !showPaymentDialog }" />

</template>

<script setup name="OderContent">
import { listCloths } from "@/api/system/cloths";
import { listTagsNoLimit } from "@/api/system/tags";
import { onMounted } from "vue";
import { delivery, pickUp } from "@/api/system/cloths";
import { listUserCouponWithValidTime } from '@/api/system/user_coupon';
import { getUser } from '@/api/system/user';
import { selectListExceptCompleted } from "@/api/system/orders";
import { getPrice } from "@/api/system/price";
import ReWash from "./rewash.vue";
import { ElMessageBox } from 'element-plus';
import { invoke } from '@tauri-apps/api/core';
import PaymentDialog from "./PaymentDialog.vue";
import vSlideIn from "@/vSlideIn";


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
    sys_service_requirement,
    sys_service_type,
    sys_clothing_status
} =
    proxy.useDict(
        'sys_payment_status',
        "sys_order_status",
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


const rewashOrder = ref(null);
const rewashClothesId = ref([]);

const phonenumber = ref();
const orders = ref([]);

const data = reactive({
    deliveryForm: {},
    pickupRules: {},
    queryParams: {
        orderNumber: null,
        phonenumber: "5638",
        pickupCode: null,
    },
});

const { deliveryForm, pickupRules, queryParams } = toRefs(data);

async function go2pay(row) {
    orders.value = [row];
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

    const orders = new Set(selectedCloths.value.map(item => item.orderId));
    if (orders.length > 1) {
        proxy.$message.error("不支持跨订单复洗");
        return;
    }
    rewashClothesId.value = selectedCloths.value.map(item => item.clothId);
    rewashOrder.value = ordersList.value.find(item => item.orderId == orders.values().next().value);

    showRewashDialog.value = true;
}

async function handlePay() {
    if (ordersList.value.length == 0) {
        proxy.notify.warning("没有可支付的订单或可取走的衣物");
        return;
    }

    // 遍历订单列表
    // 1. 没有选中衣物，只是支付
    if (selectedCloths.value.length == 0) {
        // 遍历所有的查询结果
        orders.value = ordersList.value.filter(item => item.paymentStatus === '01');
        clothsList.value = orders.value.flatMap(order => order.clothList) // 展开每个订单的衣物列表
            .sort((a, b) => b.priceValue - a.priceValue);
    } else {
        // 查询选中衣物所属的订单
        const orderIds = new Set(selectedCloths.value.map(item => item.orderId));
        orders.value = ordersList.value.filter(item => orderIds.has(item.orderId) && item.paymentStatus === '01');
        const ids = orders.value.map(item => item.orderId);
        // 排序
        clothsList.value = selectedCloths.value.filter(item => ids.includes(item.orderId)).sort((a, b) => b.priceValue - a.priceValue);
    }
    if (orders.value.length == 0) {
        proxy.notify.warning("没有选中未支付的订单");
        return;
    }

    showPaymentDialog.value = true;
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

    const orderIds = cloths.map(item => item.orderId);

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
    const orderIds = [...new Set(cloths.map(item => item.orderId))];
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
    selectedCloths.value = selectedCloths.value.filter(cloth => cloth.orderId !== orderId);

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
        orderId: null,
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
        // 初始化次卡信息
        userCouponList.value.filter(item => item.coupon.couponType == '002').map(item => {
            item.selected = false;
            item.count = 1;
        })
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

    button {
        transition: all 0.3s;
    }

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
</style>
