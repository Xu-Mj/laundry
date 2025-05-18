<template>
  <div class="app-container">
    <!-- 搜索区域 -->
    <transition name="height-fade">
      <el-card class="search-card" v-show="showSearch">
        <el-form :model="queryParams" ref="queryRef" :inline="true" label-width="68px">
          <el-form-item label="卡券名称" prop="couponTitle" size="large">
            <el-input size="large" v-model="queryParams.couponTitle" placeholder="请输入卡券名称" clearable
              @keyup.enter="handleQuery" />
          </el-form-item>
          <el-form-item label="卡券类型" prop="couponType" size="large">
            <el-select size="large" v-model="queryParams.couponType" @change="selectChange" placeholder="卡券类型" clearable
              style="width: 120px">
              <el-option v-for="dict in sys_coupon_type" :key="dict.value" :label="dict.label" :value="dict.value" />
            </el-select>
          </el-form-item>
          <el-form-item label="卡券状态" prop="status" size="large">
            <el-select size="large" v-model="queryParams.status" @change="selectChange" placeholder="卡券状态" clearable
              style="width: 120px">
              <el-option v-for="dict in sys_coupon_status" :key="dict.value" :label="dict.label" :value="dict.value" />
            </el-select>
          </el-form-item>
          <el-form-item label="删除状态" prop="status" size="large">
            <el-select v-model="queryParams.delFlag" @change="selectChange" placeholder="删除状态" clearable
              style="width: 120px">
              <el-option v-for="dict in sys_del_status" :key="dict.value" :label="dict.label" :value="dict.value" />
            </el-select>
          </el-form-item>
          <el-form-item>
            <el-button size="large" type="primary" icon="Search" @click="handleQuery">搜索</el-button>
            <el-button size="large" icon="Refresh" @click="resetQuery">重置</el-button>
          </el-form-item>
        </el-form>
      </el-card>
    </transition>

    <!-- 表格区域 -->
    <el-card class="table-card">
      <el-row :gutter="10" class="mb8">
        <el-col :span="1.5">
          <el-button type="primary" plain icon="Plus" @click="handleAdd">新增</el-button>
        </el-col>
        <el-col :span="1.5">
          <el-button type="success" :disabled="selectedList.length == 0" plain icon="Sell"
            @click="handleShowSell">卡券销售</el-button>
        </el-col>
        <right-toolbar v-model:showSearch="showSearch" @queryTable="getList" :columns="columns" />
      </el-row>

      <el-table v-loading="loading" :data="couponList" ref="table" class="modern-table"
        @selection-change="handleSelectionChange">
        <template #empty>
          <el-empty description="暂无数据" />
        </template>
        <el-table-column type="selection" width="55" align="center" />
        <!-- <el-table-column label="卡券唯一标识ID" align="center" prop="couponId" /> -->
        <el-table-column label="卡券名称" align="center" prop="couponTitle" v-if="columns[0].visible" />
        <el-table-column label="卡券编码" align="center" prop="couponNumber" v-if="columns[1].visible" width="180" />
        <el-table-column label="卡券类型" align="center" prop="couponType" v-if="columns[2].visible">
          <template #default="scope">
            <dict-tag :options="sys_coupon_type" :value="scope.row.couponType" />
          </template>
        </el-table-column>
        <el-table-column label="售卖价格(元)" align="center" prop="couponValue" v-if="columns[3].visible" />
        <el-table-column label="最低消费金额(元)" align="center" prop="minSpend" width="140" v-if="columns[4].visible" />
        <el-table-column label="客户可见" align="center" prop="customerInvalid" v-if="columns[5].visible">
          <template #default="scope">
            <dict-tag :options="sys_coupon_customer_invalid" :value="scope.row.customerInvalid" />
          </template>
        </el-table-column>
        <el-table-column label="总量限制" align="center" prop="customerSaleTotal" v-if="columns[6].visible">
          <template #default="scope">
            {{ scope.row.customerSaleTotal == -1 ? '无限制' : scope.row.customerSaleTotal }}
          </template>
        </el-table-column>
        <el-table-column label="单用户数量限制" align="center" prop="customerSaleCount" width="120" v-if="columns[7].visible">
          <template #default="scope">
            {{ scope.row.customerSaleCount == -1 ? '无限制' : scope.row.customerSaleCount }}
          </template>
        </el-table-column>
        <el-table-column label="有效时间-起" align="center" prop="validFrom" v-if="columns[8].visible">
          <template #default="scope">
            <span>{{ parseTime(scope.row.validFrom, '{y}-{m}-{d}') }}</span>
          </template>
        </el-table-column>
        <el-table-column label="有效时间-止" align="center" prop="validTo" v-if="columns[9].visible">
          <template #default="scope">
            <span>{{ parseTime(scope.row.validTo, '{y}-{m}-{d}') }}</span>
          </template>
        </el-table-column>
        <el-table-column label="自动延期" align="center" prop="autoDelay" v-if="columns[10].visible">
          <template #default="scope">
            <dict-tag :options="sys_coupon_auto_delay" :value="scope.row.autoDelay" />
          </template>
        </el-table-column>
        <el-table-column label="卡券价值" align="center" prop="usageValue" v-if="columns[11].visible">
          <template #default="scope">
            <!-- {{ scope.row.couponType === '003' ? scope.row.usageValue / 10 + '折' : scope.row.usageValue + '元' }} -->
            {{ calValue(scope.row.couponType, scope.row.usageValue) }}
          </template>
        </el-table-column>
        <el-table-column label="限制条件" align="center" prop="usageLimit" v-if="columns[12].visible">
          <template #default="scope">
            {{ scope.row.couponType === '003' ? '最高优惠金额限制' + scope.row.usageLimit + '元' :
            scope.row.usageLimit == 0 ? '无限制' : scope.row.usageLimit }}
          </template>
        </el-table-column>
        <el-table-column label="卡券状态" align="center" prop="status" v-if="columns[13].visible">
          <template #default="scope">
            <dict-tag :options="sys_coupon_status" :value="scope.row.status" />
          </template>
        </el-table-column>
        <el-table-column label="卡券描述" align="center" prop="desc" v-if="columns[14].visible" show-overflow-tooltip />
        <el-table-column label="备注" align="center" prop="remark" v-if="columns[15].visible" show-overflow-tooltip />
        <el-table-column label="删除状态" align="center" prop="delFlag" v-if="columns[16].visible" >
          <template #default="scope">
            <dict-tag :options="sys_del_status" :value="scope.row.delFlag" />
          </template>
        </el-table-column>
        <el-table-column label="操作" align="center" class-name="small-padding" width="140">
          <template #default="scope">
            <el-button link type="primary" icon="Edit" :disabled="scope.row.delFlag == '2'" @click="handleUpdate(scope.row)">修改</el-button>
            <el-button link type="primary" icon="Delete" v-if="scope.row.delFlag == '0'" @click="handleDelete(scope.row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>

      <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
        v-model:limit="queryParams.pageSize" @pagination="getList" />
    </el-card>
    <!-- 添加或修改卡券对话框 -->
    <el-dialog v-model="open" width="700px" :show-close="false" lock-scroll modal :close-on-click-modal="false"
      align-center append-to-body>
      <template #header>
        <div class="dialog-header hover-flow">
          <div class="dialog-title">
            <el-icon>
              <Ticket />
            </el-icon>
            <span>{{ title }}</span>
          </div>
          <el-button circle @click="cancel" icon="Close" />
        </div>
      </template>
      <coupon-form :value="form" :coupon-types="sys_coupon_type" :status-options="sys_coupon_status"
        @submit="submitForm" @cancel="cancel" />
    </el-dialog>

    <!-- show sell coupon -->
    <el-dialog v-model="showSell" width="800px" @closed="closeSell" class="coupon-sale-dialog" align-center
      :show-close="false">
      <template #header>
        <div class="dialog-header hover-flow">
          <div class="dialog-title">
            <el-icon>
              <Ticket />
            </el-icon>
            <span>卡券销售</span>
          </div>
          <el-button circle @click="closeSell">
            <el-icon>
              <Close />
            </el-icon>
          </el-button>
        </div>
      </template>

      <el-form ref="sellFormRef" :model="sellForm" label-width="90px" :rules="sellRules" class="coupon-sale-form">
        <!-- 会员信息卡片 -->
        <div class="form-section hover-flow">
          <div class="section-header">
            <el-icon>
              <User />
            </el-icon>
            <span>会员信息</span>
          </div>
          <div class="section-content">
            <template v-if="sellForm.userId && !needCreateUser && !editingUser">
              <div class="member-card hover-flow">
                <div class="member-avatar">
                  <el-avatar :size="50" icon="UserFilled" />
                </div>
                <div class="member-details">
                  <div class="member-name">{{ sellForm.nickName }}</div>
                  <div class="member-phone">{{ sellForm.phonenumber }}</div>
                </div>
                <div class="member-actions">
                  <el-button type="primary" text @click="editingUser = true">
                    <el-icon>
                      <Edit />
                    </el-icon>
                    修改
                  </el-button>
                </div>
              </div>
            </template>
            <template v-else>
              <el-row :gutter="20">
                <el-col :span="14">
                  <el-form-item label="会员身份" prop="userId">
                    <el-select v-model="sellForm.userId" filterable :clearable="true" remote reserve-keyword
                      placeholder="请输入手机号码搜索" allow-create @blur="handleBlur" remote-show-suffix
                      :remote-method="searchUserByTel" @change="selectUser" value-key="userId" class="w-full">
                      <el-option v-for="item in userListRes" :key="item.userId"
                        :label="item.nickName + '\t' + item.phonenumber" :value="item.userId" />
                    </el-select>
                  </el-form-item>
                </el-col>
                <el-col :span="10" v-show="needCreateUser">
                  <el-form-item label="会员姓名" prop="nickName">
                    <el-input v-model="sellForm.nickName" placeholder="请输入会员姓名" />
                  </el-form-item>
                </el-col>
              </el-row>
              <div v-if="editingUser" class="edit-user-actions">
                <el-button type="primary" @click="confirmUserEdit">确认</el-button>
                <el-button @click="cancelUserEdit">取消</el-button>
              </div>
            </template>
          </div>
        </div>

        <!-- 卡券信息卡片 -->
        <div class="form-section hover-flow">
          <div class="section-header">
            <el-icon>
              <Tickets />
            </el-icon>
            <span>卡券信息</span>
          </div>
          <div class="section-content">
            <el-table :data="selectedList" max-height="240px" border class="coupon-table">
              <el-table-column label="序号" align="center" type="index" width="60" />
              <el-table-column label="卡券名称" align="center" key="couponTitle" prop="couponTitle" />
              <el-table-column label="有效期" align="center">
                <template #default="scope">
                  <div class="validity-period">
                    <span>{{ parseTime(scope.row.validFrom, '{y}-{m}-{d}') }}</span>
                    <el-divider direction="horizontal">至</el-divider>
                    <span>{{ parseTime(scope.row.validTo, '{y}-{m}-{d}') }}</span>
                  </div>
                </template>
              </el-table-column>
              <el-table-column label="数量" align="center" width="180">
                <template #default="scope">
                  <el-input-number v-model="scope.row.count" :min="0" :max="(scope.row.customerSaleCount != -1 && scope.row.customerSaleTotal != -1)
                    ? Math.min(scope.row.customerSaleCount, scope.row.customerSaleTotal)
                    : (scope.row.customerSaleTotal != -1 ? scope.row.customerSaleTotal
                      : (scope.row.customerSaleCount != -1 ? scope.row.customerSaleCount : Infinity))"
                    controls-position="right" class="quantity-input" />
                </template>
              </el-table-column>
            </el-table>
          </div>
        </div>

        <!-- 支付方式卡片 -->
        <div class="form-section hover-flow">
          <div class="section-header">
            <el-icon>
              <Wallet />
            </el-icon>
            <span>支付方式</span>
          </div>
          <div class="section-content">
            <el-form-item class="payment-method-section">
              <el-radio-group v-model="sellForm.paymentMethod" class="payment-method-group">
                <el-radio v-for="dict in sys_coupon_payment_method" :key="dict.value" :value="dict.value"
                  class="payment-method-radio">
                  <div class="payment-method-card" :class="{ 'selected': sellForm.paymentMethod === dict.value }">
                    <el-icon v-if="dict.value === '01'">
                      <Money />
                    </el-icon>
                    <el-icon v-else-if="dict.value === '02'">
                      <ChatDotRound />
                    </el-icon>
                    <el-icon v-else-if="dict.value === '05'">
                      <Wallet />
                    </el-icon>
                    <el-icon v-else-if="dict.value === '06'">
                      <CreditCard />
                    </el-icon>
                    <el-icon v-else-if="dict.value === '07'">
                      <Ticket />
                    </el-icon>
                    <el-icon v-else>
                      <More />
                    </el-icon>
                    <span>{{ dict.label }}</span>
                  </div>
                </el-radio>
              </el-radio-group>
            </el-form-item>
          </div>
        </div>

        <!-- 备注信息卡片 -->
        <div class="form-section hover-flow">
          <div class="section-header">
            <el-icon>
              <Document />
            </el-icon>
            <span>备注信息</span>
          </div>
          <div class="section-content">
            <el-form-item style="width: 100%; margin-bottom: 0;">
              <el-input type="textarea" v-model="sellForm.remark" placeholder="备注信息" rows="3" />
            </el-form-item>
          </div>
        </div>

        <!-- 价格信息区域 -->
        <div class="price-summary-card hover-flow">
          <div class="price-row total">
            <span class="price-label">总价</span>
            <span class="price-value total-amount">¥ {{ totalPrice }}</span>
          </div>
        </div>
      </el-form>

      <template #footer>
        <div class="dialog-footer hover-flow">
          <el-button class="hover-flow" size="large" type="danger" @click="closeSell">取消</el-button>
          <el-button class="hover-flow" size="large" type="primary" @click="buy">立即购买</el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup name="Coupon">
import { listCoupon, getCoupon, delCoupon, buyCoupon } from "@/api/system/coupon";
import { listUserWithNoLimit, addUser } from "@/api/system/user";
import CouponForm from "./components/CouponForm.vue";

const { proxy } = getCurrentInstance();

const {
  sys_coupon_status,
  sys_del_status,
  sys_coupon_type,
  sys_coupon_customer_invalid,
  sys_coupon_auto_delay,
  sys_coupon_payment_method
} =
  proxy.useDict(
    "sys_coupon_status",
    "sys_del_status",
    "sys_coupon_type",
    "sys_coupon_customer_invalid",
    "sys_coupon_auto_delay",
    "sys_coupon_payment_method"
  );

const couponList = ref([]);
const userListRes = ref([]);
const userList = ref([]);
const open = ref(false);
const showSell = ref(false);
const searchUserloading = ref(false);
const loading = ref(true);
const showSearch = ref(true);
const needCreateUser = ref(false);
const editingUser = ref(false);
const ids = ref([]);
const total = ref(0);
const title = ref("");
const table = ref();

// 列显隐信息
const columns = ref([
  { key: 0, label: `卡券名称`, visible: true },
  { key: 1, label: `卡券编码`, visible: false },
  { key: 2, label: `卡券类别`, visible: true },
  { key: 3, label: `卡券面值`, visible: true },
  { key: 4, label: `最低消费金额`, visible: true },
  { key: 5, label: `客户可见`, visible: true },
  { key: 6, label: `总量限制`, visible: false },
  { key: 7, label: `单用户数量限制`, visible: false },
  { key: 8, label: `有效期-起`, visible: true },
  { key: 9, label: `有效期-止`, visible: true },
  { key: 10, label: `自动延期`, visible: true },
  { key: 11, label: `卡券价值`, visible: true },
  { key: 12, label: `限制条件`, visible: true },
  // { key: 13, label: `适用品类`, visible: true },
  // { key: 14, label: `适用分类`, visible: true },
  // { key: 15, label: `适用衣物`, visible: true },
  { key: 16, label: `卡券状态`, visible: true },
  { key: 17, label: `卡券描述`, visible: true },
  { key: 18, label: `备注`, visible: true },
  { key: 19, label: `删除状态`, visible: true },
]);
// Save column visibility to local storage
const saveColumnVisibility = () => {
  localStorage.setItem('couponColumns', JSON.stringify(columns.value));
};

// Retrieve column visibility from local storage
const loadColumnVisibility = () => {
  const savedColumns = localStorage.getItem('couponColumns');
  if (savedColumns) {
    columns.value = JSON.parse(savedColumns);
  }
};

// Watch for changes in column visibility and save to local storage
watch(columns, saveColumnVisibility, { deep: true });

const data = reactive({
  form: {},
  sellForm: {
    paymentMethod: "05"
  },
  selectedList: [],
  queryParams: {
    pageNum: 1,
    pageSize: 10,
    couponNumber: null,
    couponType: null,
    couponTitle: null,
    applicableCategory: null,
    applicableStyle: null,
    applicableCloths: null,
    status: null,
    delFlag: null,
  },
  rules: {
    couponType: [
      { required: true, message: "卡券类型不能为空", trigger: "change" }
    ],
    couponTitle: [
      { required: true, message: "卡券名称不能为空", trigger: "blur" }
    ],
    couponValue: [
      { required: true, message: "售卖不能为空", trigger: "blur" }
    ],
    usageValue: [
      { required: true, message: "卡券价值不能为空", trigger: "blur" }
    ],
    validFrom: [
      { required: true, message: "有效期-起不能为空", trigger: "blur" },
      { validator: validateValidFrom, trigger: "blur" }
    ],
    validTo: [
      { required: true, message: "有效期-止不能为空", trigger: "blur" },
      { validator: validateValidTo, trigger: "blur" }
    ],
  },
  sellRules: {
    userId: [
      { required: true, message: "会员信息不能为空", trigger: "blur" }
    ],

  }
});

const { queryParams, form, sellForm, selectedList, rules, sellRules } = toRefs(data);

/* 校验截至有效日期要大于起始日期 */
function validateValidFrom(rules, value, callback) {
  if (value && value > form.value.validTo) {
    callback(new Error("起始日期不能大于截至日期"));
  } else {
    callback();
  }
};

/* 校验截至有效日期要大于起始日期 */
function validateValidTo(rules, value, callback) {
  if (value && value < form.value.validFrom) {
    callback(new Error("截止日期不能小于起始日期"));
  } else {
    callback();
  }
};

function closeSell() {
  resetSellForm();
  table.value.clearSelection();
  selectedList.value = [];
  showSell.value = false;
}

/* 动态计算销售卡券时的总金额 */
const totalPrice = computed(() => {
  return selectedList.value.reduce((accumulator, curItem) => {
    return accumulator + curItem.couponValue * curItem.count;
  }, 0);
});

// 计算卡券价值
function calValue(type, value) {
  if (type === '000') {
    return value + '元';
  }
  if (type === '003') {
    return value + '折';
  }
  if (type === '002') {
    return value + '次';
  }
}

/** 查询卡券列表 */
function getList() {
  loading.value = true;
  listCoupon(queryParams.value).then(response => {
    couponList.value = response.rows;
    total.value = response.total;
    loading.value = false;
  });
}

// 取消按钮
function cancel() {
  open.value = false;
  reset();
}

// 表单重置
function reset() {
  form.value = {
    couponId: null,
    couponNumber: null,
    couponType: "000",
    couponTitle: null,
    couponValue: null,
    minSpend: null,
    customerInvalid: "0",
    customerSaleTotal: -1,
    customerSaleCount: -1,
    validFrom: null,
    validTo: null,
    autoDelay: "0",
    usageValue: null,
    usageLimit: null,
    applicableCategory: null,
    applicableStyle: null,
    applicableCloths: null,
    status: "0",
    desc: null,
    remark: null
  };
}

/** 搜索按钮操作 */
function handleQuery() {
  queryParams.value.pageNum = 1;
  getList();
}

/** 重置按钮操作 */
function resetQuery() {
  proxy.resetForm("queryRef");
  queryParams.value.delFlag = null;
  handleQuery();
}

// 多选框选中数据
function handleSelectionChange(selection) {
  selectedList.value = selection;
}

/** 新增按钮操作 */
function handleAdd() {
  reset();
  // 获取当前日期
  const today = new Date();
  const validFrom = today.toISOString().split('T')[0]; // 格式化为 YYYY-MM-DD

  // 获取6个月后的日期
  const sixMonthsLater = new Date();
  sixMonthsLater.setMonth(sixMonthsLater.getMonth() + 12);
  const validTo = sixMonthsLater.toISOString().split('T')[0]; // 格式化为 YYYY-MM-DD

  // 设置默认值
  form.value.validFrom = validFrom;
  form.value.validTo = validTo;
  open.value = true;
  title.value = "添加卡券";
}

/** 修改按钮操作 */
function handleUpdate(row) {
  reset();
  const _couponId = row.couponId || ids.value
  getCoupon(_couponId).then(response => {
    form.value = response;
    if (form.value.applicableCloths) {
      form.value.applicableClothsArr = form.value.applicableCloths.split(",");
    }
    if (form.value.couponType === '000') {
      form.value.usageValue = form.value.usageValue - form.value.couponValue;
    }
    open.value = true;
    title.value = "修改卡券";
  });
}

/** 提交按钮 */
function submitForm() {
  open.value = false;
  getList()
}

/** 删除按钮操作 */
function handleDelete(row) {
  const _couponIds = row.couponId || ids.value;

  // 获取要删除的卡券名称
  let confirmMessage;

  if (row.couponId) {
    // 单个删除
    confirmMessage = `是否确认删除卡券"${row.couponTitle}"?`;
  } else {
    // 批量删除
    const couponNames = couponList.value
      .filter(item => ids.value.includes(item.couponId))
      .map(item => item.couponTitle)
      .join("、");

    confirmMessage = `是否确认删除以下卡券: ${couponNames}?`;
  }

  proxy.$modal.confirm(confirmMessage, "警告", {
    confirmButtonText: "确定",
    cancelButtonText: "取消",
    type: "warning"
  }).then(function () {
    return delCoupon(_couponIds);
  }).then(() => {
    getList();
    proxy.notify.success("删除成功");
  }).catch(() => { });
}

/* 下拉选择变化触发查询 */
function selectChange() {
  queryParams.value.pageNum = 1;
  getList();
}

function resetSellForm() {
  sellForm.value = {
    userId: null,
    coupons: null,
    remark: null,
    paymentMethod: "05"
  };
  needCreateUser.value = false;
  editingUser.value = false;
  proxy.resetForm("sellFormRef");
}

function handleShowSell() {
  selectedList.value = selectedList.value.filter(item => item.customerSaleCount != 0 && item.customerSaleTotal != 0 && item.status == '0');
  selectedList.value.forEach(item => item.count = 1);

  resetSellForm();
  searchUserloading.value = true;
  listUserWithNoLimit().then(res => {
    searchUserloading.value = false;
    userList.value = res;
    showSell.value = true;
  });
}

/* 选择会员信息 */
function selectUser(userId) {
  if (!userId || userId.length == 0) {
    sellForm.value.nickName = null;
    sellForm.value.phonenumber = null;
    return;
  }
  const item = userList.value.find(item => { return item.userId === userId });
  if (item) {
    sellForm.value.nickName = item.nickName;
    sellForm.value.phonenumber = item.phonenumber;
  }
}

/* 确认用户编辑 */
function confirmUserEdit() {
  editingUser.value = false;
}

/* 取消用户编辑 */
function cancelUserEdit() {
  editingUser.value = false;
}

// 处理失去焦点的情况，保留用户输入
const handleBlur = (event) => {
  const inputValue = event.target.value;
  if (!userListRes.value.some(item => item.userId === sellForm.value.userId)) {
    // 没有搜索结果且没有选择项时，保留输入
    sellForm.value.userId = inputValue;
  }
};

/* 根据手机号搜索用户列表 */
function searchUserByTel(tel) {
  userListRes.value = userList.value.filter(item => item.phonenumber.includes(tel));
  if (userListRes.value.length == 0) {
    // 没找到，需要创建用户
    needCreateUser.value = true;
    sellForm.value.nickName = null;
  } else {
    needCreateUser.value = false;
  }
}

/* 购买卡券 */
async function buy() {
  proxy.$refs["sellFormRef"].validate(async valid => {
    if (valid) {
      const coupons = selectedList.value.filter(item => item.count > 0).map(({ couponId, count }) => ({ couponId, count }));
      sellForm.value.coupons = coupons;
      console.log(sellForm.value);
      if (needCreateUser.value) {
        try {
          const res = await addUser({
            phonenumber: sellForm.value.userId,
            nickName: sellForm.value.nickName
          });

          sellForm.value.userId = res.data; // 设置返回的用户ID
        } catch (err) {
          proxy.notify.error(err);
          return; // 当 addUser 出错时，中断执行
        }
      }
      buyCoupon(sellForm.value).then(res => {
        proxy.notify.success("购买成功");
        resetSellForm();
        showSell.value = false;
      }).catch();
    }
  });
}

loadColumnVisibility();
getList();
</script>

<style scoped>
/* 搜索区域样式 */
.search-card {
  margin-bottom: 20px;
  border-radius: 8px;

  .el-form-item {
    margin-bottom: 0 !important;
  }
}

/* 表格区域样式 */
.table-operations {
  display: flex;
  justify-content: flex-end;
  margin-bottom: 15px;
}

.table-card {
  border-radius: 8px;
  margin-bottom: 20px;
}

.modern-table {
  width: 100%;
  border-radius: 8px;
  overflow: hidden;
}

.modern-table :deep(th) {
  background-color: var(--el-fill-color-light);
  color: var(--el-text-color-primary);
  font-weight: 600;
}

.title {
  border-bottom: 1px solid gray;
}

.cash {
  display: flex;
  justify-content: right;
  align-items: center;
}

/* 卡券售卖弹窗样式 */
.coupon-sale-dialog :deep(.el-dialog__header) {
  padding: 0;
  margin: 0;
}

.coupon-sale-dialog :deep(.el-dialog__body) {
  padding: 20px;
}

.coupon-sale-dialog :deep(.el-dialog__footer) {
  padding: 16px 20px;
  border-top: 1px solid var(--el-border-color-lighter);
}

.coupon-sale-form {
  padding: 0;
}

.form-section {
  margin-bottom: 24px;
  background-color: var(--el-bg-color-overlay);
  border-radius: 8px;
  box-shadow: var(--el-box-shadow-light);
  overflow: hidden;
}

.section-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  background-color: var(--el-fill-color-light);
  border-bottom: 1px solid var(--el-border-color-lighter);
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.section-content {
  padding: 16px;
}

/* 会员卡片样式 */
.member-card {
  display: flex;
  align-items: center;
  background: linear-gradient(135deg, var(--el-fill-color-light) 0%, var(--el-fill-color-dark) 100%);
  border-radius: 12px;
  padding: 16px;
  box-shadow: var(--el-box-shadow-light);
}

.member-avatar {
  margin-right: 16px;
}

.member-details {
  flex: 1;
}

.member-name {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 4px;
  color: var(--el-color-primary);
}

.member-phone {
  font-size: 14px;
  color: var(--el-text-color-secondary);
}

.member-actions {
  margin-left: auto;
}

.edit-user-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 10px;
}

/* 价格信息样式 */
.price-summary-card {
  background-color: var(--el-fill-color-light);
  border-radius: 8px;
  padding: 20px;
  margin: 24px 0 10px 0;
  box-shadow: var(--el-box-shadow);
}

.price-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.price-label {
  font-size: 18px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.price-value {
  font-size: 18px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.price-value.total-amount {
  font-size: 28px;
  font-weight: 700;
  color: var(--el-color-danger);
}


.payment-method-section {
  margin-bottom: 0;
}

.payment-method-group {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
  margin-bottom: 0;
}

.payment-method-radio {
  margin-right: 0 !important;
  margin-bottom: 0;
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
}
</style>