<template>
  <div class="app-container">
    <el-form :model="queryParams" ref="queryRef" :inline="true" v-show="showSearch" label-width="68px">
      <el-form-item label="订单编码" prop="orderNumber">
        <el-input v-model="queryParams.orderNumber" placeholder="请输入订单编码" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="手机号" prop="phonenumber">
        <el-input v-model="queryParams.phonenumber" placeholder="请输入会员手机号" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="时效预警" prop="costTimeAlarm">
        <el-select v-model="queryParams.costTimeAlarm" @change="handleQuery" clearable style="width: 150px;"
          placeholder="请选择">
          <el-option v-for="dict in sys_cost_time_alarm" :key="dict.value" :label="dict.label" :value="dict.value" />
        </el-select>
      </el-form-item>
      <el-form-item label="取件码" prop="pickupCode">
        <el-input v-model="queryParams.pickupCode" placeholder="请输入取件码" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="支付状态" prop="paymentStatus">
        <el-select v-model="queryParams.paymentStatus" @change="handleQuery" clearable style="width: 150px;"
          placeholder="请选择支付状态">
          <el-option v-for="dict in sys_payment_status" :key="dict.value" :label="dict.label" :value="dict.value" />
        </el-select>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" icon="Search" @click="handleQuery">搜索</el-button>
        <el-button icon="Refresh" @click="resetQuery">重置</el-button>
      </el-form-item>
    </el-form>

    <el-row :gutter="10" class="mb8">
      <el-col :span="1.5">
        <el-button type="primary" plain icon="Plus" @click="handleAdd" v-hasPermi="['system:orders:add']">新增</el-button>
      </el-col>
      <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
    </el-row>

    <el-table v-loading="loading" :data="ordersList">
      <!-- <el-table-column type="selection" width="55" align="center" /> -->
      <el-table-column label="订单编码" align="center" width="180" prop="orderNumber" />
      <el-table-column label="所属会员" align="center" width="180">
        <template #default="scope">
          {{ scope.row.nickName + ' - ' + scope.row.phonenumber }}
        </template>
      </el-table-column>
      <el-table-column label="业务类型" align="center" prop="businessType">
        <template #default="scope">
          <dict-tag :options="sys_business_type" :value="scope.row.businessType" />
        </template>
      </el-table-column>
      <el-table-column label="订单创建时间" align="center" prop="createTime" width="180">
        <template #default="scope">
          <span>{{ parseTime(scope.row.createTime, '{y}-{m}-{d}') }}</span>
        </template>
      </el-table-column>
      <el-table-column label="预计完成时间" align="center" prop="desireCompleteTime" width="180">
        <template #default="scope">
          <span>{{ parseTime(scope.row.desireCompleteTime, '{y}-{m}-{d}') }}</span>
        </template>
      </el-table-column>
      <el-table-column label="时效预警" align="center" prop="costTimeAlarm">
        <template #default="scope">
          <dict-tag :options="sys_cost_time_alarm" :value="scope.row.costTimeAlarm" />
        </template>
      </el-table-column>
      <el-table-column label="取件码" align="center" prop="pickupCode" />
      <el-table-column label="订单完成时间" align="center" prop="completeTime" width="140">
        <template #default="scope">
          <span>{{ parseTime(scope.row.completeTime, '{y}-{m}-{d}') }}</span>
        </template>
      </el-table-column>
      <el-table-column label="取回方式" align="center" prop="deliveryMode">
        <template #default="scope">
          <dict-tag :options="sys_delivery_mode" :value="scope.row.deliveryMode"
            @click="handleDeliveryMode(scope.row)" />
        </template>
      </el-table-column>
      <el-table-column label="订单来源" align="center" prop="source">
        <template #default="scope">
          <dict-tag :options="sys_price_order_type" :value="scope.row.source" />
        </template>
      </el-table-column>
      <el-table-column label="状态" align="center" prop="status" :width="140">
        <template #default="scope">
          <div class="status-row">
            <dict-tag :options="sys_order_status" :value="scope.row.status" />
            <dict-tag :options="sys_payment_status" :value="scope.row.paymentStatus" />
          </div>
        </template>
      </el-table-column>
      <el-table-column label="订单类型" align="center" prop="orderType">
        <template #default="scope">
          <dict-tag :options="sys_order_type" :value="scope.row.orderType" />
        </template>
      </el-table-column>
      <el-table-column label="支付状态" align="center" prop="paymentStatus">
        <template #default="scope">
          <dict-tag :options="sys_payment_status" :value="scope.row.paymentStatus" />
        </template>
      </el-table-column>
      <el-table-column label="备注" align="center" prop="remark" />
      <el-table-column label="操作" align="center" class-name="small-padding fixed-width">
        <template #default="scope">
          <el-button link type="primary" icon="Edit" @click="showClothList(scope.row)"
            v-hasPermi="['system:orders:edit']">衣物</el-button>
          <el-dropdown>
            <el-icon class="el-icon--right">
              <arrow-down />
            </el-icon>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item>
                  <el-button v-if="scope.row.paymentStatus !== '00'" link type="primary" icon="Edit"
                    @click="handleUpdate(scope.row)" v-hasPermi="['system:orders:edit']">编辑</el-button>
                </el-dropdown-item>
                <el-dropdown-item>
                  <el-button link type="primary" icon="Edit" :disabled="scope.row.status == '05'"
                    @click="handleRefund(scope.row)" v-hasPermi="['system:orders:edit']">退单</el-button>
                </el-dropdown-item>
                <el-dropdown-item>
                  <el-button link type="primary" icon="Edit"
                    :disabled="scope.row.status !== '02' && scope.row.status !== '03'" @click="handleNotify(scope.row)"
                    v-hasPermi="['system:orders:edit']">通知</el-button>
                </el-dropdown-item>
                <el-dropdown-item disabled>
                  <el-button link type="primary" icon="Edit"
                    :disabled="scope.row.paymentStatus !== '01' || scope.row.status == '05'"
                    @click="handleUpdate(scope.row)" v-hasPermi="['system:orders:edit']">收款</el-button>
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </template>
      </el-table-column>
    </el-table>

    <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
      v-model:limit="queryParams.pageSize" @pagination="getList" />

    <!-- 通知弹窗 -->
    <el-dialog v-model="showNoticeDialog" width="400px" append-to-body>
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
    <el-dialog v-model="showRefundDialog" width="400px" append-to-body>
      <el-form ref="refundFormRef" :model="refundForm" :rules="refundRules" label-width="80px">
        <el-form-item label="支出账目" prop="expTitle">
          <el-input v-model="refundForm.expTitle" placeholder="请输入支出账目" />
          <!-- <el-select v-model="refundForm.expType" disabled>
            <el-option v-for="item in sys_exp_type" :key="item.value" :label="item.label" :value="item.value" />
          </el-select> -->
        </el-form-item>
        <el-form-item label="对方账户" prop="recvAccountTitle">
          <el-input v-model="refundForm.recvAccountTitle" disabled />
        </el-form-item>
        <el-form-item label="实退金额" prop="expAmount">
          <el-input type="number" v-model="refundForm.expAmount" placeholder="请输入退款金额" />
        </el-form-item>
        <el-form-item label="备注信息" prop="remark">
          <el-input type="textarea" v-model="refundForm.remark" placeholder="请输入备注信息" />
        </el-form-item>
      </el-form>
      <template #footer>
        <div class="payment-footer">
          <el-button type="primary" @click="submitRefundForm">确认退款</el-button>
        </div>
      </template>
    </el-dialog>

    <!-- 派送弹窗 -->
    <el-dialog v-model="showExpressInfoDialog" width="600px" append-to-body>
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
    <el-dialog v-model="showDeliveryInfoDialog" width="600px" append-to-body>
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
    <!-- 衣物列表弹窗 -->
    <el-dialog title="衣物" v-model="showClothListDialog" width="1440px" append-to-body>
      <ShowCloths :orderId="currentOrderId" :flashList="getList" :userId="currentUserId" :key="currentOrderId" />
    </el-dialog>

    <!-- 添加或修改洗护服务订单对话框 -->
    <el-dialog :title="title" v-model="open" width="1440px" append-to-body lock-scroll modal :before-close="cancel"
      :close-on-click-modal="false">
      <el-form ref="ordersRef" :model="form" :rules="rules" label-width="80px">
        <el-row>
          <el-col :span="6">
            <el-form-item label="会员身份" prop="userId">
              <el-select v-model="form.userId" filterable :clearable="true" remote reserve-keyword
                placeholder="请输入手机号码搜索" allow-create @blur="handleBlur" remote-show-suffix
                :remote-method="searchUserByTel" @change="selectUser" value-key="userId" style="width: 240px">
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
            <el-radio v-for="dict in sys_price_order_type" :key="dict.value" :label="dict.label" :value="dict.value">{{
              dict.label
            }}</el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item label="衣物信息">
          <AddCloth v-if="form.userId" :userId="form.userId" :orderId="form.orderId" v-model:value="form.cloths" />
          <span v-else>请选择会员信息后添加衣物</span>
        </el-form-item>
        <el-form-item label="店主调价">
          <el-col :span="12" class="adjust-price-group">
            <el-input type="number" :min="0" @input="adjustInput" v-model="form.adjust.adjustValueSub"
              placeholder="请输入调减金额" />
            <el-input type="number" :min="0" @input="adjustInput" v-model="form.adjust.adjustValueAdd"
              placeholder="请输入调增金额" />
            <el-input type="number" :min="0" @input="adjustInput" v-model="form.adjust.totalAmount"
              placeholder="请输入总金额" />
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
              <el-col :span="4">
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
              <el-col :span="4">
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
              <el-button type="info" plain @click="handleShowCouponSale">卡券购买</el-button>
              <el-button type="primary" plain @click="submitForm">保 存</el-button>
              <el-button type="warning" plain @click="cancel">取 消</el-button>
            </div>
          </el-col>
        </el-row>
      </el-form>
    </el-dialog>

    <!-- 付款弹窗 -->
    <el-dialog title="付款" v-model="showPaymentDialog" width="600px" append-to-body lock-scroll modal
      :close-on-click-modal="false">
      <el-form ref="paymentRef" :model="paymentForm" :rules="paymentRules" label-width="80px">
        <el-form-item label="订单编号">
          {{ paymentForm.payNumber }}
        </el-form-item>
        <el-form-item label="支付方式">
          <el-radio-group v-model="paymentForm.paymentMethod">
            <el-radio v-for="dict in sys_payment_method" :key="dict.value" :label="dict.value">
              {{ dict.label }}
            </el-radio>
          </el-radio-group>
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
      <CouponSale :userId="form.userId" :submit="submitCouponSale" />"
    </el-dialog>
  </div>
</template>

<script setup name="Orders">
import { watch } from "vue";
import { ElMessageBox } from 'element-plus'
import { listOrders, getOrders, delOrders, addOrders, updateOrders,getRefundInfo, pay  } from "@/api/system/orders";
import { listUser, getUser } from "@/api/system/user";
import { delCloths } from "@/api/system/cloths";
import { listUserCoupon } from '@/api/system/user_coupon';
import { isCurrentTimeWithinRange, getFutureDate } from "@/utils";
import { listDispatch } from '@/api/system/dispatch';
import { refund } from '@/api/system/orders';
import { addRecord } from '@/api/system/notice_record';
import { listTemplate } from '@/api/system/template';
import { getConfigKey } from '@/api/system/config';
import AddCloth from "./addCloth.vue";
import ShowCloths from './showCloths.vue';
import CouponSale from './couponSale.vue';
import { addUser } from "../../../api/system/user";

const { proxy } = getCurrentInstance();
const {
  sys_cost_time_alarm,
  sys_payment_status,
  sys_price_order_type,
  sys_business_type,
  sys_delivery_mode,
  sys_order_type,
  sys_order_status,
  sys_payment_method,
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
    "sys_payment_method",
    "sys_notice_method",
  );

// 订单列表
const ordersList = ref([]);
// 用户列表，创建/更新订单时选择框使用
const userList = ref([]);
const userListRes = ref([]);
// 用户卡券列表
const userCouponList = ref([]);
// 通知模板列表
const noticeTempList = ref([]);
const open = ref(false);
const searchUserloading = ref(false);
const showClothListDialog = ref(false);
const showExpressInfoDialog = ref(false);
const showDeliveryInfoDialog = ref(false);
const showNoticeDialog = ref(false);
const showRefundDialog = ref(false);
const showCreateUser = ref(false);
const showPaymentDialog = ref(false);
const showCouponSale = ref(false);
const loading = ref(true);
const showSearch = ref(true);
const ids = ref([]);
const total = ref(0);
const title = ref("");
const expressInfo = ref({});
const deliveryInfo = ref({});
const totalPrice = ref(0);
// 差价
const priceDiff = ref(0);
const couponStorageCardId = ref([]);

const currentOrderId = ref(0);
const currentUserId = ref(null);

/* 单据打印数量 */
const printCount = ref(1);

const data = reactive({
  form: {
    adjust: {}
  },
  paymentForm: {},
  refundForm: {},
  notifyForm: {},
  queryParams: {
    pageNum: 1,
    pageSize: 10,
    orderNumber: null,
    businessType: null,
    phonenumber: null,
    desireCompleteTime: null,
    costTimeAlarm: null,
    pickupCode: null,
    completeTime: null,
    deliveryMode: null,
    source: null,
    status: null,
    paymentStatus: null,
    orderType: null,
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

const { queryParams, form, paymentForm, refundForm, notifyForm, rules, refundRules } = toRefs(data);

/* 监听form.cloths变动 */

watch(() => form.value.cloths, (newVal) => {
  console.log('form.cloths changed:', newVal);
  checkCoupon();
  if (form.value.adjust.totalAmount) {
    proxy.$modal.msgWarning('衣物列表发生变动，请重新填写订单金额');
    form.adjust.totalAmount = null;
  }
  adjustInput();
});
// 处理失去焦点的情况，保留用户输入
const handleBlur = (event) => {
  const inputValue = event.target.value;
  if (!userListRes.value.some(item => item.userId === form.value.userId)) {
    // 没有搜索结果且没有选择项时，保留输入
    form.value.userId = inputValue;
  }
};
/* 卡券购买完成后的回调，重新获取卡券列表 */
function submitCouponSale() {
  listUserCoupon({ userId: form.userId }).then(response => {
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
    open.value = false;
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
/** 查询洗护服务订单列表 */
function getList() {
  loading.value = true;
  listOrders(queryParams.value).then(response => {
    ordersList.value = response.rows;
    total.value = response.total;
    loading.value = false;
  });
}

// 取消按钮
function cancel() {
  if (!form.value.userId) {
    reset();
    open.value = false;
    return;
  }
  ElMessageBox.confirm('确认取消操作订单？此操作不可逆！')
    .then(() => {
      // 请求服务器删除添加的衣物列表
      if (!form.value.orderId && form.value.cloths.length > 0) {
        delCloths(form.value.cloths.map(item => item.clothId)).then(res => {
          reset();
          open.value = false;
        }).catch(res => {
          console.error(res)
        })
      } else {
        open.value = false;
        reset();
      }
    })
    .catch(() => {
      // catch error
    })
}

// 表单重置
function reset() {
  currentOrderId.value = 0;
  form.value = {
    adjust: {},
    cloths: [],
    orderId: null,
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

// 多选框选中数据
// function handleSelectionChange(selection) {
//   ids.value = selection.map(item => item.orderId);
//   single.value = selection.length != 1;
//   multiple.value = !selection.length;
// }

/** 新增按钮操作 */
function handleAdd() {
  reset();
  title.value = "添加洗护服务订单";
  // 获取预计完成时间
  getConfigKey('desire_complete_time').then(res => {
    form.value.desireCompleteTime = getFutureDate(res.msg);
  });
  listUser().then(res => {
    userList.value = res.rows;
    open.value = true;
  });
}

/** 修改按钮操作 */
function handleUpdate(row) {
  // console.log(row)
  reset();
  const _orderId = row.orderId || ids.value
  // 获取订单内容
  getOrders(_orderId).then(response => {
    form.value = response.data;
    form.value.cloths = [];
    if (!form.value.adjust) {
      form.value.adjust = {};
    }
    open.value = true;
    title.value = "修改服务订单";
  });

  listUser().then(res => {
    userList.value = res.rows;
    userListRes.value = userList.value;
  });
  // 获取用户卡券列表
  listUserCoupon({ userId: row.userId }).then(response => {
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
          open.value = false;
          getList();
        });
      } else {
        addOrders(form.value).then(response => {
          proxy.$modal.msgSuccess("新增成功");
          open.value = false;
          getList();
        });
      }
    }
  });
}

/* 提交退款 */
function submitRefundForm() {
  proxy.$refs["refundFormRef"].validate(valid => {
    if (valid) {
      refund(refundForm.value).then(res => {
        if (res.code === 200) {
          proxy.$modal.msgSuccess('退款成功');
          showRefundDialog.value = false;
          resetRefundForm();
          getList();
        }
      })
    }
  });
}

/* 提交通知 */
function submitNotifyForm() {
  proxy.$refs["notifyFormRef"].validate(valid => {
    if (valid) {
      addRecord(notifyForm.value).then(res => {
        if (res.code === 200) {
          proxy.$modal.msgSuccess('通知成功');
        }
        showNoticeDialog.value = false;
        resetNotify();
      })
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
          getList();
          open.value = true;
          showPaymentDialog.value = true;
        });
      } else {
        initPaymentForm();
        open.value = true;
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

/** 删除按钮操作 */
function handleDelete(row) {
  const _orderIds = row.orderId || ids.value;
  proxy.$modal.confirm('是否确认删除洗护服务订单编号为"' + _orderIds + '"的数据项？').then(function () {
    return delOrders(_orderIds);
  }).then(() => {
    getList();
    proxy.$modal.msgSuccess("删除成功");
  }).catch(() => { });
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


const handleClose = (done) => {
  ElMessageBox.confirm('确认取消操作订单？此操作不可逆！')
    .then(() => {
      done()
    })
    .catch(() => {
      // catch error
    })
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
          expressInfo.value.deliveryAddr = res.data.address;
        } else if (row.deliveryMode === '01') {
          deliveryInfo.value.deliveryAddr = res.data.address;
        }
      });
    }
    console.log(deliveryInfo.value)
  })
}

/* 退款 */
function handleRefund(row) {
  refundForm.value.orderId = row.orderId;
  refundForm.value.expTitle = "订单退款";
  refundForm.value.expType = "00";
  // refundForm.value.expAmount = row.;
  refundForm.value.recvAccount = row.userId;
  getRefundInfo(row.orderId, row.userId).then(res => {
    refundForm.value.recvAccountTitle = res.data.user.userName;
    if (res.data.payment) {
      // 已经支付了
      if (res.data.payment.paymentAmountMv) {
        refundForm.value.expAmount = res.data.payment.paymentAmountMv;
      } else {
        refundForm.value.expAmount = res.data.payment.paymentAmount;
      }
    }
    showRefundDialog.value = true;

  })
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

/* 调价输入框输入事件 */
function adjustInput() {
  if (form.value.adjust.totalAmount) {
    totalPrice.value = form.value.adjust.totalAmount;
  } else {
    const price = form.value.cloths.reduce((acc, cur) => {
      return acc +
        cur.priceValue + cur.processMarkup
    }, 0) +
      Number(form.value.adjust.adjustValueAdd ? form.value.adjust.adjustValueAdd : 0) -
      Number(form.value.adjust.adjustValueSub ? form.value.adjust.adjustValueSub : 0);
    totalPrice.value = price > 0 ? price : 0;
  }
}
getList();
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