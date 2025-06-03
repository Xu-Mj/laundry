<template>
    <div class="result-container">
        <el-form :model="queryParams" class="top-bar" ref="queryRef" :inline="true" label-width="68px">
            <el-form-item label="取件码" prop="pickupCode">
                <el-input style="width: 150px;" v-model="queryParams.pickupCode" placeholder="请输入取件码" clearable
                    @keyup.enter="handleQuery" type="number" class="no-spinner" @mousewheel.native.prevent
                    @DOMMouseScroll.native.prevent>
                    <template #prefix>
                        <el-icon>
                            <Ticket />
                        </el-icon>
                    </template>
                </el-input>
            </el-form-item>
            <el-form-item label="手机号" prop="phonenumber">
                <el-input ref="phonenumber" style="width: 200px;" v-model="queryParams.phonenumber"
                    placeholder="请输入会员手机号" clearable @keyup.enter="handleQuery" type="number" class="no-spinner"
                    @mousewheel.native.prevent @DOMMouseScroll.native.prevent>
                    <template #prefix>
                        <el-icon>
                            <Phone />
                        </el-icon>
                    </template>
                </el-input>
            </el-form-item>
            <!-- <el-form-item label="订单编码" prop="orderNumber">
                <el-input style="width: 230px;" v-model="queryParams.orderNumber" placeholder="请输入订单编码" clearable
                    @keyup.enter="handleQuery">
                    <template #prefix>
                        <el-icon>
                            <Document />
                        </el-icon>
                    </template>
                </el-input>
            </el-form-item> -->
            <el-form-item label="姓名" prop="nickName">
                <el-input style="width: 230px;" v-model="queryParams.nickName" placeholder="请输入姓名" clearable
                    @keyup.enter="handleQuery">
                    <template #prefix>
                        <el-icon>
                            <User />
                        </el-icon>
                    </template>
                </el-input>
            </el-form-item>
            <el-form-item label="支付状态" prop="paymentStatus">
                <el-select v-model="queryParams.paymentStatus" @change="handleQuery" clearable style="width: 120px;"
                    placeholder="请选择">
                    <template #prefix>
                        <el-icon>
                            <Warning />
                        </el-icon>
                    </template>
                    <el-option v-for="dict in PaymentStatus" :key="dict.value" :label="dict.label"
                        :value="dict.value" />
                </el-select>
            </el-form-item>
            <el-form-item>
                <el-button class="hover-flow" type="primary" icon="Search" @click="handleQuery" round>搜索</el-button>
                <el-button class="hover-flow" icon="Refresh" @click="resetQuery" round>重置</el-button>
            </el-form-item>
        </el-form>
        <!-- 渲染订单抖索结果列表 -->
        <div class="search-result-list">
            <div v-if="ordersList.length === 0" class="no-result">
                <el-empty description="暂无数据" />
            </div>
            <div v-slide-in v-else class="result-item" v-for="order in ordersList" :key="order.orderId">
                <div class="result-item-order-num">
                    <span>
                        订单编码: {{ order.orderNumber }}
                    </span>
                    <el-button type="primary" size="small" @click="reprintReceipt(order)">补打小票</el-button>
                </div>
                <div class="result-item-info">
                    <div class="info-item">
                        <el-icon>
                            <User />
                        </el-icon>
                        <span style="display: flex; align-items: center;">会员身份: <strong>{{ order.nickName }}</strong>
                            ({{ order.phonenumber }})
                        </span>
                    </div>
                    <div class="info-item">
                        <el-icon>
                            <Wallet />
                        </el-icon>
                        <span>支付状态:</span>
                        <el-tag v-if="order.paymentStatus === 'Unpaid'" style="cursor: pointer;" @click="go2pay(order)">
                            {{ PaymentStatusMap[order.paymentStatus]?.label }}
                        </el-tag>
                        <el-tag v-else :type="PaymentStatusMap[order.paymentStatus]?.type">
                            {{ PaymentStatusMap[order.paymentStatus]?.label }}
                        </el-tag>
                    </div>
                    <div class="info-item">
                        <el-icon>
                            <Money />
                        </el-icon>
                        <span>{{ order.paymentStatus === 'Paid' ? '实际支付金额:' : '应支付金额:' }}</span>
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
                        <el-tag :type="OrderStatusMap[order.status]?.type">
                            {{ OrderStatusMap[order.status]?.label }}
                        </el-tag>
                    </div>
                </div>
                <!-- 订单包含的衣物列表 -->
                <el-table v-if="order.clothList && order.clothList.length > 0" :data="order.clothList"
                    :loading="order.loading" row-key="clothId"
                    @selection-change="selectedItems => handleClothSelectionChange(selectedItems, order)"
                    ref="clothsTableRef" class="modern-table" stripe>
                    <el-table-column type="selection" width="50" align="center" />
                    <el-table-column label="衣物" align="center" min-width="100" show-overflow-tooltip>
                        <template #default="scope">
                            <div class="cloth-name">
                                {{ scope.row.clothInfo.title }}
                                <span v-if="scope.row.clothingColor" class="cloth-color">
                                    {{colorList.find(item => item.tagId == scope.row.clothingColor)?.tagName}}
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
                                <el-tag :type="ServiceTypeMap[scope.row.serviceType]?.type">
                                    {{ ServiceTypeMap[scope.row.serviceType]?.label }}
                                </el-tag>
                                <el-divider direction="vertical" class="vertical-divider" />
                                <el-tag :type="ServiceRequirmentMap[scope.row.serviceRequirement]?.type">
                                    {{ ServiceRequirmentMap[scope.row.serviceRequirement]?.label }}
                                </el-tag>
                            </span>
                        </template>
                    </el-table-column>
                    <el-table-column label="洗护价格" align="center" prop="priceValue">
                        <template #default="scope">
                            <span class="price-value">¥{{ scope.row.priceValue }}</span>
                        </template>
                    </el-table-column>
                    <el-table-column label="工艺加价" align="center" prop="processMarkup">
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
                                    {{flawList.find(item => item.tagId == tagId)?.tagName}}
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
                                    {{estimateList.find(item => item.tagId == tagId)?.tagName}}
                                </el-tag>
                                <span v-if="!scope.row.estimate">-</span>
                            </div>
                        </template>
                    </el-table-column>
                    <el-table-column label="衣物品牌" align="center" prop="clothingBrand" width="100">
                        <template #default="scope">
                            <el-tag v-if="scope.row.clothingBrand" type="success" size="small" effect="light">
                                {{brandList.find(item => item.tagId == scope.row.clothingBrand)?.tagName}}
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
                            </div>
                        </template>
                    </el-table-column>
                    <el-table-column label="洗护状态" align="center" prop="clothingStatus" width="100">
                        <template #default="scope">
                            <el-tag :type="ClothStatusMap[scope.row.clothingStatus]?.type">
                                {{ ClothStatusMap[scope.row.clothingStatus]?.label }}
                            </el-tag>
                        </template>
                    </el-table-column>
                    <el-table-column label="上挂位置" align="center" width="120">
                        <template #default="scope">
                            <el-tag v-if="scope.row.hangLocationCode" type="success">
                                {{ scope.row.hangerName + '-' + scope.row.hangerNumber }}
                            </el-tag>
                            <span v-else>-</span>
                        </template>
                    </el-table-column>
                    <el-table-column label="操作" align="center" width="120" fixed="right">
                        <template #default="scope">
                            <div v-if="scope.row.clothingStatus == 'ReadyForPickup'" class="action-buttons"> <el-button
                                    type="primary" size="small" plain round @click="pickup(scope.row)"> <el-icon>
                                        <TakeawayBox />
                                    </el-icon> 取衣 </el-button> <el-button type="danger" size="small" plain round
                                    @click="handleCompensate(scope.row)"> <el-icon>
                                        <Money />
                                    </el-icon> 赔偿 </el-button> </div>
                            <span v-else>-</span>
                        </template>
                    </el-table-column>
                </el-table>

            </div>
        </div>

        <div class="footer"> <el-button type="danger" @click="props.taggle()" icon="Close" round>关闭</el-button>
            <el-button type="primary" @click="pickup()" icon="TakeawayBox" round>取衣</el-button>
            <el-button type="success" @click="handlePay" icon="Wallet" round>取衣收款</el-button>
            <el-button type="warning" @click="handleDelivery" icon="Van" round>上门派送</el-button>
            <el-button type="info" @click="() => { }" icon="Printer" round>补打小票</el-button>
        </div>
        <el-backtop :right="40" :bottom="180" />
    </div>

    <!-- 展示照片对话框 -->
    <el-dialog title="照片预览" v-model="showPicture" width="600px" :align-center="true" class="picture-dialog">
        <div class="picture-container">
            <el-empty v-if="pictureList.length === 0" description="暂无照片" />
            <el-carousel v-else :interval="4000" type="card" height="300px">
                <el-carousel-item v-for="(item, index) in pictureList" :key="index">
                    <el-image :src="item" fit="contain" :preview-src-list="pictureList" preview-teleported
                        class="carousel-image" />
                </el-carousel-item>
            </el-carousel>
            <div class="picture-grid" v-if="pictureList.length > 0">
                <el-image v-for="(item, index) in pictureList" :key="index" :src="item" fit="cover"
                    :preview-src-list="pictureList" class="grid-image" />
            </div>
        </div>
    </el-dialog> <!-- 派送对话框 -->
    <DeliveryDialog :visible="showDeliveryDialog" :user="currentUser" :selected-cloths="selectedCloths"
        @success="handleDeliverySuccess" @cancel="handleDeliveryCancel" /> <!-- 赔偿对话框 -->
    <CompensationDialog v-model:visible="showCompensationDialog" :selection-list="selectedCloths"
        :order-id="selectedCloths.length > 0 ? selectedCloths[0].orderId : ''"
        :user-id="currentUser ? currentUser.userId : ''" @success="handleCompensationSuccess" />
    <Pay :visible="showPaymentDialog" :user="currentUser" :orders="orders" :clothsList="clothsList"
        :userCouponList="userCouponList" :couponTypeList="couponTypeList" :showPickupButton="true" :refresh="getList"
        :key="showPaymentDialog" :toggle="() => { showPaymentDialog = !showPaymentDialog }"
        @payment-success="handlePaymentSuccess" @success="handlePaymentSuccess" @pickup="handlePaymentPickup"
        @payment-failed="handlePaymentFailed" />

</template>

<script setup name="OderContent">
import { pickUp } from "@/api/system/cloths";
import { getUser } from '@/api/system/user';
import { getPrice } from "@/api/system/price";
import { listCloths } from "@/api/system/cloths";
import { printReceipt } from '@/api/system/printer';
import { selectListExceptCompleted } from "@/api/system/orders";
import { listUserCouponWithValidTime } from '@/api/system/user_coupon';
import { ElMessageBox } from 'element-plus';
import { invoke } from '@tauri-apps/api/core';
import Pay from "./pay.vue";
import DeliveryDialog from "./DeliveryDialog.vue";
import CompensationDialog from "@/views/components/CompensationDialog.vue";
import useTagsStore from "@/store/modules/tags";
import { PaymentMethodMap, ServiceRequirmentMap, ServiceTypeMap, ClothStatusMap, PaymentStatus, PaymentStatusMap, OrderStatusMap } from "@/constants";


const props = defineProps({
    taggle: {
        type: Function,
        required: true,
    }
});

const { proxy } = getCurrentInstance();

// 订单列表
const ordersList = ref([]);
const showPaymentDialog = ref(false);

const loading = ref(true);
const colorList = ref([]);
const flawList = ref([]);
const estimateList = ref([]);
const brandList = ref([]);
const pictureList = ref([]);
const selectedCloths = ref([]);

const clothsTableRef = ref();
// 用户卡券列表
const userCouponList = ref([]);
// 用户卡券种类列表
const couponTypeList = ref();

const showPicture = ref(false);
const showDeliveryDialog = ref(false);
const showCompensationDialog = ref(false);
// 当前需要处理的衣物列表
const clothsList = ref([]);

// 当前用户信息
const currentUser = ref(null);

const phonenumber = ref();
const orders = ref([]);

const data = reactive({
    queryParams: {
        orderNumber: null,
        phonenumber: null,
        pickupCode: null,
        paymentStatus: null,
    },
});

const { queryParams } = toRefs(data);
// 用户校验函数
function validateSameUser() {
    if (selectedCloths.value.length === 0) {
        proxy.notify.warning("请先选择衣物");
        return false;
    }

    // 获取选中衣物所属的用户ID
    const userIds = [...new Set(selectedCloths.value.map(cloth => {
        const order = ordersList.value.find(o => o.orderId === cloth.orderId);
        return order ? order.userId : null;
    }))];

    // 检查是否存在无效的用户ID
    if (userIds.includes(null)) {
        proxy.notify.error("选中的衣物中存在无效订单");
        return false;
    }

    // 检查是否跨用户操作
    if (userIds.length > 1) {
        proxy.notify.warning("不能同时操作不同用户的衣物，请重新选择");
        return false;
    }

    return true;
}
async function go2pay(row) {
    orders.value = [row];
    showPaymentDialog.value = true;
}

async function handlePay() {
    // if (ordersList.value.length == 0) {
    //     proxy.notify.warning("没有可支付的订单或可取走的衣物");
    //     return;
    // }
    // 如果没有选中衣物，检查是否有订单
    if (selectedCloths.value.length === 0) {
        if (ordersList.value.length == 0) {
            proxy.notify.warning("没有可支付的订单或可取走的衣物");
            return;
        }
        
        // 检查所有订单是否属于同一用户
        const userIds = [...new Set(ordersList.value.map(order => order.userId))];
        if (userIds.length > 1) {
            proxy.notify.warning("当前查询结果包含多个用户的订单，请选择具体衣物进行操作");
            return;
        }
    } else {
        // 如果选中了衣物，进行用户校验
        if (!validateSameUser()) {
            return;
        }
    }
    // 遍历订单列表
    // 1. 没有选中衣物，只是支付
    if (selectedCloths.value.length == 0) {
        // 遍历所有的查询结果
        orders.value = ordersList.value.filter(item => item.paymentStatus === 'Unpaid');
        clothsList.value = orders.value.flatMap(order => order.clothList) // 展开每个订单的衣物列表
            .sort((a, b) => b.priceValue - a.priceValue);
    } else {
        // 查询选中衣物所属的订单
        const orderIds = new Set(selectedCloths.value.map(item => item.orderId));
        orders.value = ordersList.value.filter(item => orderIds.has(item.orderId) && item.paymentStatus === 'Unpaid');
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
     // 用户校验
     if (!validateSameUser()) {
        return;
    }

    console.log(selectedCloths.value)
    const cloths = selectedCloths.value.filter(item => item.clothingStatus !== 'PickedUp');
    if (cloths.length == 0) {
        proxy.notify.warning("没有选中符合条件的衣物");
        return;
    }

    // 筛选正在洗护中的衣物进行提示
    const washCloths = cloths.filter(item => item.clothingStatus === 'Processing');
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
    const unpaidOrders = ordersList.value.filter(item => orderIds.includes(item.orderId) && item.paymentStatus !== 'Paid');
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

// 处理支付成功事件
async function handlePaymentSuccess(data) {
    // 支付成功后刷新订单列表
    await getList();
    // proxy.notify.success("支付成功");
    selectedCloths.value = [];
}

// 处理支付失败事件
function handlePaymentFailed(error) {
    console.error('支付失败:', error);
    proxy.notify.error(error || '支付失败');
}

// 处理支付成功并取衣事件
async function handlePaymentPickup() {
    // 获取当前选中的衣物ID列表
    const cloths = selectedCloths.value.filter(item => item.clothingStatus !== 'PickedUp');
    if (cloths.length > 0) {
        const ids = cloths.map(item => item.clothId);
        try {
            await pickUp(ids);
            proxy.notify.success("支付成功并已取走衣物");
            selectedCloths.value = [];
            // 刷新订单列表
            await getList();
        } catch (err) {
            proxy.notify.error("取衣失败: " + err.message);
        }
    } else {
        proxy.notify.success("支付成功，没有需要取走的衣物");
        // 刷新订单列表
        await getList();
    }
}

// 显示赔偿对话框
function handleCompensate(cloth) {
    if (cloth) {
        selectedCloths.value = [cloth];
    }

    if (selectedCloths.value.length === 0) {
        proxy.notify.warning("请先选择需要赔偿的衣物");
        return;
    }
 // 用户校验
    if (!validateSameUser()) {
        return;
    }

    // 检查是否有衣物状态为已完成洗护的衣物
    const validCloths = selectedCloths.value.filter(item => item.clothingStatus === 'ReadyForPickup');
    if (validCloths.length === 0) {
        proxy.notify.warning("只能对已完成洗护的衣物进行赔偿");
        return;
    }

    // 检查衣物是否来自同一订单
    const orderIds = new Set(validCloths.map(item => item.orderId));
    if (orderIds.size > 1) {
        proxy.notify.warning("不能对不同订单的衣物同时进行赔偿");
        return;
    }

    showCompensationDialog.value = true;
}

// 赔偿成功回调
function handleCompensationSuccess() {
    proxy.notify.success("赔偿操作成功");
    selectedCloths.value = [];
    getList();
}

// 处理派送成功事件
// async function handleDeliverySuccess() {
//     showDeliveryDialog.value = false;
//     selectedCloths.value = [];
//     await getList();
//     proxy.notify.success("派送安排成功");
// }

// 处理派送按钮点击事件
function handleDelivery() {
    if (selectedCloths.value.length === 0) {
        proxy.notify.warning("请先选择需要派送的衣物");
        return;
    }

 // 用户校验
    if (!validateSameUser()) {
        return;
    }
    // 检查选中的衣物是否都已完成洗护或正在洗护中
    const invalidCloths = selectedCloths.value.filter(item =>
        item.clothingStatus !== 'Processing' && item.clothingStatus !== 'ReadyForPickup'
    );

    if (invalidCloths.length > 0) {
        proxy.notify.warning("选中的衣物中有不符合派送条件的衣物，只能派送正在洗护或已完成洗护的衣物");
        return;
    }

    // 检查选中的衣物是否都已支付
    const orderIds = [...new Set(selectedCloths.value.map(item => item.orderId))];
    const unpaidOrders = ordersList.value.filter(item => orderIds.includes(item.orderId) && item.paymentStatus !== 'PickedUp');

    if (unpaidOrders.length > 0) {
        proxy.notify.warning("选中的衣物中有未支付的订单，请先完成支付");
        return;
    }

    showDeliveryDialog.value = true;
}

// 派送成功回调
function handleDeliverySuccess() {
    proxy.notify.success("派送操作成功");
    getList();
}

// 派送取消回调
function handleDeliveryCancel() {
    // 对话框关闭后的处理
}

// 衣物列表多选框选中数据
function handleClothSelectionChange(selectedItems, row) {
    // 获取当前订单的所有衣物，用于对比哪些被选中哪些未被选中
    const orderId = row.orderId;

    // 先移除当前订单下所有选中的衣物
    selectedCloths.value = selectedCloths.value.filter(cloth => cloth.orderId !== orderId);

    // 只添加新选中的衣物，这样就不会选择整个订单的衣物
    if (selectedItems && selectedItems.length > 0) {
        selectedCloths.value.push(...selectedItems);
    }
}

/* 初始化列表数据 */
async function initList() {
    // 使用store中的tags缓存
    const tagsStore = useTagsStore();

    // 获取颜色列表
    if (colorList.value.length === 0) {
        colorList.value = tagsStore.getTagsByOrder('Color');
    }

    // 获取瑕疵列表
    if (flawList.value.length === 0) {
        flawList.value = tagsStore.getTagsByOrder('PreCleaningFlaws');
    }

    // 获取预估列表
    if (estimateList.value.length === 0) {
        estimateList.value = tagsStore.getTagsByOrder('PostCleaningProjection');
    }

    // 获取品牌列表
    if (brandList.value.length === 0) {
        brandList.value = tagsStore.getTagsByOrder('Brand');
    }
}

/** 查询洗护服务订单列表 */
async function getList() {
    if (queryParams.value.pickupCode === ''
        && queryParams.value.phonenumber === ''
        && queryParams.value.orderNumber === ''
        && queryParams.value.paymentStatus === '') {
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
        userCouponList.value.filter(item => item.coupon.couponType == 'SessionCard').map(item => {
            item.selected = false;
            item.count = 1;
        })
    });

    // 遍历计算订单价格
    for (const item of ordersList.value) {
        item.loading = true;

        item.clothList = await listCloths({ orderId: item.orderId });
        item.loading = false;

        // 优先处理 `adjust` 的情况

        if (item.payment && item.payment.payId) {
            if (item.payment.paymentMethod == 'Meituan' || item.payment.paymentMethod == 'Douyin') {
                item.mount = await calculatePrice(item);
            } else {
                item.mount = item.payment.paymentAmount;
            }
        } else if (item.adjust) {
            if (item.adjust.adjustTotal) {
                item.mount = item.adjust.adjustTotal;
            } else {
                const price = await calculatePrice(item);
                const adjustedPrice = price +
                    Number(item.adjust.adjustValueAdd || 0) -
                    Number(item.adjust.adjustValueSub || 0);
                item.mount = adjustedPrice > 0 ? adjustedPrice : 0;
            }
        } else {
            // 没有 `adjust` 的情况下计算价格
            const price = await calculatePrice(item);
            item.mount = price > 0 ? price : 0;
        }

        // 过滤已取走的衣物
        if (item.paymentStatus == 'Paid') {
            item.clothList = item.clothList.filter(cloth => cloth.clothingStatus !== 'PickedUp');
        }
    }
}

// 提取出价格计算逻辑
async function calculatePrice(item) {
    // 处理价格方案数组情况
    if (item.priceIds && item.priceIds.length > 0) {
        let totalPrice = 0;
        for (const priceId of item.priceIds) {
            try {
                const priceItem = await getPrice(priceId);
                if (priceItem && priceItem.priceValue) {
                    totalPrice += priceItem.priceValue;
                }
            } catch (error) {
                console.error(`获取价格方案${priceId}失败:`, error);
            }
        }
        return totalPrice;
    }
    // 处理单一价格方案（遗留代码兼容）
    else if (item.priceId) {
        try {
            const priceItem = await getPrice(item.priceId);
            return priceItem ? priceItem.priceValue : 0;
        } catch (error) {
            console.error(`获取价格方案${item.priceId}失败:`, error);
            return 0;
        }
    }
    // 没有价格方案时按衣物计算
    else {
        return item.clothList.reduce((acc, cur) => {
            let priceValue = cur.priceValue;
            if (cur.serviceRequirement === 'Emergency') {
                priceValue *= 2;
            } else if (cur.serviceRequirement === 'SingleWash') {
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
        nickName: queryParams.value.nickName ? queryParams.value.nickName.trim() : null,
        phonenumber: queryParams.value.phonenumber ? queryParams.value.phonenumber.trim() : null,
        paymentStatus: queryParams.value.paymentStatus ? queryParams.value.paymentStatus.trim() : null,
    };
    if (isEmpty(queryParams.value.pickupCode) &&
        isEmpty(queryParams.value.nickName) &&
        isEmpty(queryParams.value.phonenumber) &&
        isEmpty(queryParams.value.paymentStatus)) {
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
        isEmpty(queryParams.value.nickName) &&
        isEmpty(queryParams.value.phonenumber)) {
        return;
    }
    getList();
});

// 假设有orderId可用
async function reprintReceipt(order) {
    try {
        let paymentMethod;
        if (order.payment.payId) {
            paymentMethod = PaymentMethodMap[order.paymentMethod]?.label;
        } else {
            paymentMethod = '未付款';
        }

        await printReceipt({ ...order, paymentMethod });
        proxy.notify.success('小票补打成功');
    } catch (e) {
        proxy.notify.error('小票补打失败');
    }
}
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

/* 照片对话框 */
.picture-container {
    padding: 10px 0;
}

.carousel-image {
    width: 100%;
    height: 100%;
    border-radius: 4px;
}

.picture-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
    gap: 10px;
    margin-top: 20px;
}

.grid-image {
    width: 100%;
    height: 100px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.3s ease;
}

.grid-image:hover {
    transform: scale(1.05);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
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
