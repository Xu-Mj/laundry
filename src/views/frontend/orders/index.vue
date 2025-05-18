<template>
  <div class="app-container">
    <!-- 搜索区域 -->
    <transition name="height-fade">
      <el-card class="search-card" v-show="showSearch">
        <el-form :model="queryParams" ref="queryRef" :inline="true" label-width="68px">
          <el-form-item label="订单编码" prop="orderNumber" size="large">
            <el-input size="large" v-model="queryParams.orderNumber" placeholder="请输入订单编码" clearable v-number-only
              style="width: 220px;" @keyup.enter="handleQuery">
              <template #prefix>
                <el-icon>
                  <Document />
                </el-icon>
              </template>
            </el-input>
          </el-form-item>
          <el-form-item label="手机号" prop="phonenumber" size="large">
            <el-input size="large" v-model="queryParams.phonenumber" placeholder="请输入会员手机号" clearable
              style="width: 200px;" @keyup.enter="handleQuery" type="number" class="no-spinner"
              @mousewheel.native.prevent @DOMMouseScroll.native.prevent>
              <template #prefix>
                <el-icon>
                  <Phone />
                </el-icon>
              </template>
            </el-input>
          </el-form-item>
          <el-form-item label="取件码" prop="pickupCode" size="large">
            <el-input size="large" v-model="queryParams.pickupCode" placeholder="请输入取件码" clearable style="width: 140px;"
              @keyup.enter="handleQuery" type="number" class="no-spinner" @mousewheel.native.prevent
              @DOMMouseScroll.native.prevent>
              <template #prefix>
                <el-icon>
                  <Ticket />
                </el-icon>
              </template>
            </el-input>
          </el-form-item>
          <el-form-item label="时效预警" prop="costTimeAlarm" size="large">
            <el-select size="large" v-model="queryParams.costTimeAlarm" @change="handleQuery" clearable
              style="width: 120px;" placeholder="请选择">
              <template #prefix>
                <el-icon>
                  <AlarmClock />
                </el-icon>
              </template>
              <el-option v-for="dict in sys_cost_time_alarm" :key="dict.value" :label="dict.label"
                :value="dict.value" />
            </el-select>
          </el-form-item>
          <el-form-item label="支付状态" prop="paymentStatus" size="large">
            <el-select size="large" v-model="queryParams.paymentStatus" @change="handleQuery" clearable
              style="width: 120px;" placeholder="请选择">
              <template #prefix>
                <el-icon>
                  <Warning />
                </el-icon>
              </template>
              <el-option v-for="dict in sys_payment_status" :key="dict.value" :label="dict.label" :value="dict.value" />
            </el-select>
          </el-form-item>
          <el-form-item label="洗护状态" prop="status" size="large">
            <el-select size="large" v-model="queryParams.status" @change="handleQuery" clearable style="width: 120px;"
              placeholder="请选择">
              <template #prefix>
                <el-icon>
                  <Warning />
                </el-icon>
              </template>
              <el-option v-for="dict in sys_order_status" :key="dict.value" :label="dict.label" :value="dict.value" />
            </el-select>
          </el-form-item>
          <el-button class="hover-flow" type="primary" icon="Search" @click="handleQuery" size="large">搜索</el-button>
          <el-button class="hover-flow" icon="Refresh" @click="resetQuery" size="large">重置</el-button>
        </el-form>
      </el-card>
    </transition>

    <!-- 表格区域 -->
    <el-card class="table-card">
      <div class="table-operations">
        <right-toolbar v-model:showSearch="showSearch" @queryTable="getList" :columns="columns"></right-toolbar>
      </div>
      <el-table v-loading="loading" :data="ordersList" class="modern-table no-wrap-table" border stripe
        :table-layout="fixed" :fit="true">
        <template #empty>
          <el-empty description="暂无数据" />
        </template>
        <el-table-column label="订单编码" align="center" prop="orderNumber" v-if="columns[0].visible" min-width="160"
          show-overflow-tooltip />
        <el-table-column label="所属会员" align="center" v-if="columns[1].visible" min-width="120">
          <template #default="scope">
            <div class="member-info">
              <span class="member-name">{{ scope.row.nickName }}</span>
              <span class="member-phone">{{ scope.row.phonenumber }}</span>
            </div>
          </template>
        </el-table-column>

        <el-table-column label="订单金额" align="center" prop="paymentAmount" v-if="columns[2].visible">
          <template #default="scope">
            <span class="price-tag">
              {{ scope.row.paymentAmount ? scope.row.paymentAmount + '元' : null }}
            </span>
          </template>
        </el-table-column>
        <el-table-column label="支付方式" align="center" prop="paymentBonusType" v-if="columns[3].visible">
          <template #default="scope">
            <dict-tag :options="sys_payment_method_show" :value="scope.row.paymentBonusType" />
          </template>
        </el-table-column>
        <el-table-column label="卡券优惠" align="center" prop="paymentBonusCount" v-if="columns[4].visible">
          <template #default="scope">
            <span v-if="!scope.row.paymentBonusCount || scope.row.paymentBonusCount == 0">-</span>
            <span v-else-if="paymentTimeBasedSet.has(scope.row.paymentBonusType)" class="bonus-count">
              {{ scope.row.paymentBonusCount }}张
            </span>
            <span v-else class="bonus-count">
              {{ scope.row.paymentBonusCount }}元
            </span>
          </template>
        </el-table-column>
        <el-table-column label="实际支付" align="center" v-if="columns[5].visible">
          <template #default="scope">
            <span v-if="scope.row.diffPrice > 0" class="actual-price">
              {{ scope.row.diffPrice }}元
            </span>
          </template>
        </el-table-column>
        <el-table-column label="衣物编码" align="center" v-if="columns[6].visible">
          <template #default="scope">
            <div class="cloth-code-container">
              <el-tag type="primary" v-for="item in scope.row.clothCodes" :key="item" size="small" effect="plain">{{
                item }}</el-tag>
            </div>
          </template>
        </el-table-column>
        <el-table-column label="取件码" align="center" prop="pickupCode" v-if="columns[7].visible">
          <template #default="scope">
            <span class="pickup-code">{{ scope.row.pickupCode }}</span>
          </template>
        </el-table-column>

        <el-table-column label="业务类型" align="center" prop="businessType" v-if="columns[8].visible">
          <template #default="scope">
            <dict-tag :options="sys_business_type" :value="scope.row.businessType" />
          </template>
        </el-table-column>
        <el-table-column label="收衣时间" align="center" prop="createTime" min-width="100" v-if="columns[9].visible">
          <template #default="scope">
            <span class="time-info">{{ formatTime(scope.row.createTime) }}</span>
          </template>
        </el-table-column>

        <el-table-column label="时效预警" align="center" prop="costTimeAlarm" v-if="columns[10].visible">
          <template #default="scope">
            <dict-tag :options="sys_cost_time_alarm" :value="scope.row.costTimeAlarm" />
          </template>
        </el-table-column>
        <el-table-column label="订单完成时间" align="center" prop="completeTime" min-width="100" v-if="columns[11].visible">
          <template #default="scope">
            <span class="time-info">{{ scope.row.completeTime }}</span>
          </template>
        </el-table-column>
        <el-table-column label="订单来源" align="center" prop="source" v-if="columns[12].visible">
          <template #default="scope">
            <dict-tag :options="sys_price_order_type" :value="scope.row.source" />
          </template>
        </el-table-column>
        <el-table-column label="洗护状态" align="center" prop="status" v-if="columns[13].visible">
          <template #default="scope">
            <dict-tag :options="sys_order_status" :value="scope.row.status" />
          </template>
        </el-table-column>
        <el-table-column label="订单类型" align="center" prop="orderType" v-if="columns[14].visible">
          <template #default="scope">
            <dict-tag :options="sys_order_type" :value="scope.row.orderType" />
          </template>
        </el-table-column>
        <el-table-column label="支付状态" align="center" prop="paymentStatus" v-if="columns[15].visible">
          <template #default="scope">
            <dict-tag v-if="scope.row.paymentStatus === '01'" style="cursor: pointer;" @click="go2pay(scope.row)"
              :options="sys_payment_status" :value="scope.row.paymentStatus" />
            <dict-tag v-else :options="sys_payment_status" :value="scope.row.paymentStatus" />
          </template>
        </el-table-column>
        <el-table-column label="取回方式" align="center" prop="deliveryMode" v-if="columns[16].visible">
          <template #default="scope">
            <dict-tag :options="sys_delivery_mode" :value="scope.row.deliveryMode"
              @click="handleDeliveryMode(scope.row)" />
          </template>
        </el-table-column>
        <el-table-column label="预计完成时间" align="center" prop="desireCompleteTime" min-width="100"
          v-if="columns[17].visible">
          <template #default="scope">
            <span>{{ parseTime(scope.row.desireCompleteTime, '{y}-{m}-{d}') }}</span>
          </template>
        </el-table-column>
        <el-table-column label="备注" align="center" prop="remark" show-overflow-tooltip v-if="columns[18].visible" />
        <el-table-column label="操作" fixed="right" align="center" class-name="small-padding fixed-width">
          <template #default="scope">
            <el-button link type="primary" @click="showClothList(scope.row)">衣物</el-button>
            <el-dropdown>
              <el-icon class="el-icon--right">
                <arrow-down />
              </el-icon>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item>
                    <el-button link type="primary" icon="Edit" @click="handleUpdate(scope.row)">{{ scope.row.status ==
                      '06' || scope.row.paymentStatus == '00' ? '查看' : '编辑' }}</el-button>
                  </el-dropdown-item>
                  <el-dropdown-item>
                    <el-button link type="primary" icon="Edit" :disabled="scope.row.status == '06'"
                      @click="handleRefund(scope.row)">退单</el-button>
                  </el-dropdown-item>
                  <!-- <el-dropdown-item>
                    <el-button link type="primary" icon="Edit"
                      :disabled="scope.row.status !== '02' && scope.row.status !== '03'"
                      @click="handleNotify(scope.row)">通知</el-button>
                  </el-dropdown-item>-->
                  <el-dropdown-item>
                    <el-button link type="danger" icon="Delete" @click="handleDelete(scope.row)">删除</el-button>
                  </el-dropdown-item>
                  <el-dropdown-item>
                    <el-button link type="primary" icon="Edit"
                      :disabled="scope.row.paymentStatus !== '01' || scope.row.status == '06'"
                      @click="go2pay(scope.row)">收款</el-button>
                  </el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </template>
        </el-table-column>
      </el-table>

      <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
        v-model:limit="queryParams.pageSize" @pagination="getList" />
    </el-card>
    <!-- 通知弹窗 -->
    <el-dialog v-model="showNoticeDialog" width="400px" :align-center="true" append-to-body>
      <el-form ref="notifyFormRef" :model="notifyForm" :rules="notifyRules" label-width="80px">
        <el-form-item label="通知模板" prop="tempId">
          <el-select v-model="notifyForm.tempId" @change="tempSelectChange">
            <el-option v-for="item in noticeTempList" :key="item.tempId" :label="item.tempName" :value="item.tempId" />
          </el-select>
        </el-form-item>
        <el-form-item label="通知方式" prop="noticeMethod">
          <el-radio-group v-model="notifyForm.noticeMethod">
            <el-radio v-for="dict in sys_notice_method" :key="dict.value" :label="dict.label"
              :value="dict.value"></el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item label="通知内容" prop="content">
          <el-input type="textarea" v-model="notifyForm.content" placeholder="请输入通知标题" />
        </el-form-item>
      </el-form>
      <template #footer>
        <div class="payment-footer">
          <el-button type="primary" @click="submitNotifyForm">发送通知</el-button>
        </div>
      </template>
    </el-dialog>

    <!-- 退单弹窗 -->
    <RefundDialog :visible="showRefundDialog" :order-id="refundForm.orderId" :user-id="refundForm.recvAccount"
      :order-number="refundForm.orderNumber" @refund-success="handleRefundSuccess"
      @refund-cancel="showRefundDialog = false" @update:visible="showRefundDialog = $event" ref="refundDialogRef" />

    <!-- 派送弹窗 -->
    <el-dialog v-model="showExpressInfoDialog" width="600px" :align-center="true" append-to-body>
      <el-form :data="expressInfo">
        <el-form-item label="配送地址" prop="deliveryAddr">
          {{ expressInfo.deliveryAddr }}
        </el-form-item>
        <el-form-item label="快递公司" prop="deliveryComp">
          {{ expressInfo.deliveryComp }}
        </el-form-item>
        <el-form-item label="邮寄时间" prop="dispatchTime">
          {{ expressInfo.dispatchTime }}
        </el-form-item>
        <el-form-item label="快递单号" prop="deliveryNum">
          {{ expressInfo.deliveryNum }}
        </el-form-item>
        <el-form-item label="备注" prop="remark">
          {{ expressInfo.remark }}
        </el-form-item>
      </el-form>
    </el-dialog>
    <el-dialog v-model="showDeliveryInfoDialog" width="600px" :align-center="true" append-to-body>
      <el-form :data="deliveryInfo">
        <el-form-item label="配送地址" prop="deliveryAddr">
          {{ deliveryInfo.deliveryAddr }}
        </el-form-item>
        <el-form-item label="派送时间" prop="dispatchTime">
          {{ deliveryInfo.dispatchTime }}
        </el-form-item>
        <el-form-item label="沟通时间" prop="deliveryNum">
          {{ deliveryInfo.deliveryNum }}
        </el-form-item>
        <el-form-item label="备注" prop="remark">
          {{ deliveryInfo.remark }}
        </el-form-item>
      </el-form>
    </el-dialog>
    <ShowClothsModern :orderId="currentOrderId" :visible="showClothListDialog" :flashList="getList"
      :userId="currentUserId" :key="showClothListDialog"
      :toggle="() => { showClothListDialog = !showClothListDialog }" />
    <el-dialog :show-close="false" v-model="open" fullscreen lock-scroll :before-close="cancel"
      :close-on-click-modal="false">
      <CreateOrder ref="createOrderRef" :visible="true" :orderId="currentOrderId" :userId="currentUserId"
        :toggle="() => { open = !open; getList(); }" :refresh="getList" :key="open" />
    </el-dialog>
    <Pay :visible="showPaymentDialog" :key="showPaymentDialog" :order="currentOrder" :refresh="getList"
      :toggle="() => { showPaymentDialog = !showPaymentDialog }" :userCouponList="userCouponList"
      :couponTypeList="couponTypeList" :showPickupButton="false" @payment-success="handlePaymentSuccess"
      @payment-failed="handlePaymentFailed" />
  </div>
</template>

<script setup name="Orders">
import { listOrders, delOrders } from "@/api/system/orders";
import { getUser } from "@/api/system/user";
import { listUserCouponWithValidTime } from '@/api/system/user_coupon';
import { listDispatch } from '@/api/system/dispatch';
import { addRecord } from '@/api/system/notice_record';
import { listTemplate } from '@/api/system/template';
import ShowClothsModern from './showClothsModern.vue';
import CreateOrder from "@/views/components/createOrder.vue";
import Pay from "@/views/components/pay.vue";
import { listCloths } from "@/api/system/cloths";
import RefundDialog from "@/components/refundDialog.vue";

const { proxy } = getCurrentInstance();
const {
  sys_cost_time_alarm,
  sys_payment_status,
  sys_price_order_type,
  sys_business_type,
  sys_delivery_mode,
  sys_order_type,
  sys_order_status,
  sys_payment_method_show,
  sys_notice_method,
} =
  proxy.useDict(
    'sys_cost_time_alarm',
    'sys_payment_status',
    "sys_price_order_type",
    "sys_business_type",
    "sys_delivery_mode",
    "sys_order_type",
    "sys_order_status",
    "sys_payment_method_show",
    "sys_notice_method",
  );

// 订单列表
const ordersList = ref([]);
// 通知模板列表
const noticeTempList = ref([]);
// 用户卡券列表
const userCouponList = ref([]);
// 用户卡券种类列表
const couponTypeList = ref(new Set());
const open = ref(false);
const showClothListDialog = ref(false);
const showExpressInfoDialog = ref(false);
const showDeliveryInfoDialog = ref(false);
const showNoticeDialog = ref(false);
const showRefundDialog = ref(false);
const loading = ref(true);
const showSearch = ref(true);
const showPaymentDialog = ref(false);
const total = ref(0);
const title = ref("");
const expressInfo = ref({});
const deliveryInfo = ref({});
const currentOrder = ref({});

const currentOrderId = ref(0);
const currentUserId = ref(0);
const createOrderRef = ref();
const paymentTimeBasedSet = new Set(['07', '17', '27', '57']);
const paymentCouponSet = new Set(['18', '17', '27', '57']);

const data = reactive({
  refundForm: {},
  notifyForm: {},
  queryParams: {
    pageNum: 1,
    pageSize: 10,
    orderNumber: null,
    phonenumber: null,
    costTimeAlarm: null,
    pickupCode: null,
    paymentStatus: null,
    status: null,
  },
  rules: {
    businessType: [
      { required: true, message: "业务类型不能为空", trigger: "change" }
    ],
    userId: [
      { required: true, message: "所属会员ID不能为空", trigger: "blur" }
    ],
    source: [
      { required: true, message: "订单来源不能为空", trigger: "blur" }
    ],
    cloths: [
      { required: true, message: "衣物信息不能为空", trigger: "change" }
    ]
  },
  refundRules: {}
});

const { queryParams, refundForm, notifyForm, refundRules } = toRefs(data);

// 列显隐信息
const columns = ref([
  { key: 1, label: `订单编码`, visible: true },
  { key: 2, label: `所属会员`, visible: true },
  { key: 3, label: `订单金额`, visible: true },
  { key: 4, label: `支付方式`, visible: true },
  { key: 5, label: `卡券优惠`, visible: true },
  { key: 6, label: `实际支付`, visible: true },
  { key: 7, label: `衣物编码`, visible: true },
  { key: 8, label: `取件码`, visible: true },
  { key: 9, label: `业务类型`, visible: true },
  { key: 10, label: `收衣时间`, visible: true },
  { key: 11, label: `时效预警`, visible: false },
  { key: 12, label: `订单完成时间`, visible: true },
  { key: 13, label: `订单来源`, visible: true },
  { key: 14, label: `洗护状态`, visible: true },
  { key: 15, label: `订单类型`, visible: false },
  { key: 16, label: `支付状态`, visible: true },
  { key: 17, label: `取回方式`, visible: false },
  { key: 18, label: `预计完成时间`, visible: true },
  { key: 19, label: `备注`, visible: false },
]);

// Save column visibility to local storage
const saveColumnVisibility = () => {
  localStorage.setItem('orderColumns', JSON.stringify(columns.value));
};

// Retrieve column visibility from local storage
const loadColumnVisibility = () => {
  const savedColumns = localStorage.getItem('orderColumns');
  if (savedColumns) {
    columns.value = JSON.parse(savedColumns);
  }
};

// Watch for changes in column visibility and save to local storage
watch(columns, saveColumnVisibility, { deep: true });


// 处理支付成功事件
function handlePaymentSuccess(data) {
  getList();
}

// 处理支付失败事件
function handlePaymentFailed(error) {
  console.error('支付失败:', error);
  proxy.notify.error(error || '支付失败');
}

function go2pay(row) {
  // 根据订单id查询衣物列表
  listCloths({ orderId: row.orderId }).then(async res => {
    currentOrder.value = row;
    currentOrder.value.cloths = res;

    // 计算衣物的原始价格（不包含调价）
    let originalPrice = 0;

    // 如果选择了价格方案，那么使用所有选中价格方案的总和
    if (row.priceIds && row.priceIds.length > 0) {
      // 这种情况暂时无法得知每个价格方案的详情，用总价替代
      originalPrice = row.totalPrice || 0;
    } else {
      // 计算衣物的原始价格总和
      originalPrice = res.reduce((acc, cur) => {
        let priceValue = cur.priceValue;
        if (cur.serviceRequirement == '001') {
          priceValue *= 2;
        } else if (cur.serviceRequirement == '002') {
          priceValue *= 1.5;
        }
        return acc + priceValue + (cur.processMarkup || 0);
      }, 0);
    }

    // 设置原价
    currentOrder.value.originalPrice = originalPrice > 0 ? originalPrice : 0;

    // 获取用户卡券列表
    await listUserCouponWithValidTime(row.userId).then(response => {
      userCouponList.value = response;
      userCouponList.value.filter(item => item.coupon.couponType == '002').map(item => {
        item.selected = false;
        item.count = 1;
      });
      couponTypeList.value = new Set(userCouponList.value.map(coupon => coupon.coupon.couponType));
    });

    showPaymentDialog.value = true;
    console.log(currentOrder.value);
  });
}

/** 查询洗护服务订单列表 */
function getList() {
  loading.value = true;
  listOrders(queryParams.value).then(response => {
    ordersList.value = response.rows;
    total.value = response.total;
    loading.value = false;
    currentOrder.value = null;
  });
}

// 取消按钮
function cancel(done) {
  if (createOrderRef.value) {
    createOrderRef.value.cancel()
      .then((canClose) => {
        if (canClose) {
          done(); // 允许关闭对话框
        }
      })
      .catch(() => {
        // 用户选择取消，不关闭对话框
      });
  } else {
    done(); // 子组件不存在时，直接关闭对话框
  }
}


// 表单重置
function reset() {
  currentOrderId.value = 0;
  currentUserId.value = 0;
  // 重置其他表单数据
  resetNotify();
  resetRefundForm();
}

/* 重置通知表单 */
function resetNotify() {
  notifyForm.value = {
    userId: null,
    orderNumber: null,
    noticeMethod: '1',
    noticeType: '1',
    content: null,
    title: "订单通知"
  };
  proxy.resetForm("notifyFormRef");
}
/* 重置通知表单 */
function resetRefundForm() {
  refundForm.value = {
    orderId: null,
    orderNumber: null,
    expTitle: '订单退款',
    expType: '00',
    unPay: false,
  };
  proxy.resetForm("refundFormRef");
}

/** 搜索按钮操作 */
function handleQuery() {
  queryParams.value.pageNum = 1;
  getList();
}

/** 重置按钮操作 */
function resetQuery() {
  proxy.resetForm("queryRef");
  handleQuery();
}

/** 新增按钮操作 */
function handleAdd() {
  reset();
  title.value = "添加洗护服务订单";
  open.value = true;
}

/** 修改按钮操作 */
function handleUpdate(row) {
  reset();
  currentOrderId.value = row.orderId;
  currentUserId.value = row.userId;
  open.value = true;
}
/** 删除按钮操作 */
function handleDelete(row) {
  const _orderIds = row.orderId ;
    proxy.$modal.confirm('是否确认删除洗护服务订单编号为"' + row.orderNumber + '"的数据项？').then(function () {
      return delOrders(_orderIds);
    }).then(() => {
      getList();
      proxy.notify.success("删除成功");
    }).catch(() => { });
}

/* 提交通知 */
function submitNotifyForm() {
  proxy.$refs["notifyFormRef"].validate(valid => {
    if (valid) {
      addRecord(notifyForm.value).then(res => {
        if (res.code === 200) {
          proxy.notify.success('通知成功');
        }
        showNoticeDialog.value = false;
        resetNotify();
      })
    }
  });
}

/* 展示衣物列表 */
function showClothList(row) {
  currentOrderId.value = row.orderId;
  currentUserId.value = row.userId;
  showClothListDialog.value = true;
}

/* 展示快递信息或者上门派送信息 */
function handleDeliveryMode(row) {
  if (row.deliveryMode === "00") {
    return;
  }
  listDispatch({ orderId: row.orderId }).then(res => {
    if (res.rows && res.rows.length === 1) {
      if (row.deliveryMode === '02') {
        showExpressInfoDialog.value = true;
        expressInfo.value = res.rows[0];
      } else if (row.deliveryMode === '01') {
        deliveryInfo.value = res.rows[0];
        showDeliveryInfoDialog.value = true;
      }
      // 获取用户信息
      getUser(row.userId).then(res => {
        if (row.deliveryMode === '02') {
          expressInfo.value.deliveryAddr = res.address;
        } else if (row.deliveryMode === '01') {
          deliveryInfo.value.deliveryAddr = res.address;
        }
      });
    }
  })
}

/* 退款 */
function handleRefund(row) {
  // 设置必要的数据
  refundForm.value.orderId = row.orderId;
  refundForm.value.orderNumber = row.orderNumber;
  refundForm.value.recvAccount = row.userId;

  // 显示退款对话框
  showRefundDialog.value = true;
}

/* 处理退款成功 */
function handleRefundSuccess() {
  proxy.notify.success("退款成功，正在刷新数据");
  resetRefundForm();
  getList();
}

/* 通知 */
function handleNotify(row) {
  notifyForm.value.userId = row.userId;
  notifyForm.value.orderNumber = row.orderNumber;
  notifyForm.value.noticeMethod = "1";
  notifyForm.value.noticeType = "1";
  notifyForm.value.title = "订单通知";
  listTemplate().then(res => {
    noticeTempList.value = res.rows;
    showNoticeDialog.value = true;
  });
}

/* 测试模板变动 */
function tempSelectChange(tempId) {
  const temp = noticeTempList.value.find(item => item.tempId == notifyForm.value.tempId);
  if (temp) {
    notifyForm.value.content = temp.content;
    notifyForm.value.noticeMethod = temp.noticeMethod;
  }
}

loadColumnVisibility();
getList();
</script>

<style scoped>
/* 表格区域样式 */
.table-operations {
  display: flex;
  justify-content: flex-end;
  margin-bottom: 15px;
}

/* 表格内容样式 */
.no-wrap-table :deep(.el-table__cell) {
  white-space: nowrap !important;
}

.no-wrap-table :deep(.el-table__inner-wrapper) {
  width: 100%;
}

.no-wrap-table :deep(.cell) {
  overflow: hidden;
  text-overflow: ellipsis;
}

.member-info {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.member-name {
  font-weight: bold;
}

.member-phone {
  color: #909399;
  font-size: 12px;
}

.price-tag {
  color: #F56C6C;
  font-weight: bold;
}

.bonus-count {
  color: #67C23A;
  font-weight: bold;
}

.actual-price {
  color: #F56C6C;
  font-weight: bold;
}

.cloth-code-container {
  display: flex;
  flex-wrap: wrap;
  gap: 5px;
  justify-content: center;
}

.pickup-code {
  font-weight: bold;
  color: #67C23A;
  padding: 2px 8px;
  border-radius: 4px;
}

.time-info {
  color: #606266;
  font-size: 13px;
}

.status-tag :deep(.el-tag) {
  font-weight: bold;
}

.payment-tag :deep(.el-tag) {
  font-weight: bold;
}

.operation-buttons {
  display: flex;
  justify-content: center;
  gap: 10px;
}

/* 弹窗样式 */
.payment-footer {
  text-align: center;
}

.status-row {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  justify-content: center;
  align-items: center;
  gap: .2rem;
}

/* 退单弹窗样式 */
.refund-dialog :deep(.el-dialog__header) {
  margin: 0;
  padding: 20px 24px;
  border-bottom: 1px solid var(--el-border-color-lighter);
}

.refund-dialog :deep(.el-dialog__body) {
  padding: 24px;
}

.refund-dialog :deep(.el-dialog__footer) {
  padding: 16px 24px;
  border-top: 1px solid var(--el-border-color-lighter);
}

.refund-dialog-header {
  display: flex;
  align-items: center;
  gap: 10px;
  color: var(--el-color-primary);
  font-size: 18px;
  font-weight: 600;
}

.refund-dialog-header .el-icon {
  font-size: 20px;
}

.refund-dialog-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.refund-info-card {
  background-color: var(--el-fill-color-lighter);
  border-radius: 8px;
  padding: 16px;
}

.refund-order-info {
  display: flex;
  justify-content: space-between;
  margin-bottom: 16px;
}

.refund-info-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.refund-label {
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

.refund-value {
  font-size: 15px;
  font-weight: 500;
  color: var(--el-text-color-primary);
}

.refund-divider {
  height: 1px;
  background-color: var(--el-border-color-lighter);
  margin: 12px 0;
}

.refund-amount-info {
  display: flex;
  justify-content: flex-end;
}

.refund-original-amount {
  font-size: 18px;
  font-weight: 600;
  color: var(--el-color-danger);
}

.refund-form {
  margin-top: 8px;
}

.refund-form :deep(.el-form-item__label) {
  padding-bottom: 8px;
  font-weight: 500;
  color: var(--el-text-color-primary);
}

.refund-input {
  border-radius: 6px;
}

.refund-input-number :deep(.el-input__wrapper) {
  border-radius: 6px;
}

.refund-textarea {
  border-radius: 6px;
}

.refund-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}
</style>

<style>
/* 隐藏所有浏览器中的增减按钮 */
.no-spinner input::-webkit-outer-spin-button,
.no-spinner input::-webkit-inner-spin-button {
  -webkit-appearance: none !important;
}
</style>