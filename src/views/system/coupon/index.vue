<template>
  <div class="app-container">
    <el-form :model="queryParams" ref="queryRef" :inline="true" v-show="showSearch" label-width="68px">
      <el-form-item label="卡券编码" prop="couponNumber">
        <el-input v-model="queryParams.couponNumber" placeholder="请输入卡券编码" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="卡券名称" prop="couponTitle">
        <el-input v-model="queryParams.couponTitle" placeholder="请输入卡券名称" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="卡券类型" prop="couponType">
        <el-select v-model="queryParams.couponType" @change="selectChange" placeholder="卡券类型" clearable
          style="width: 120px">
          <el-option v-for="dict in sys_coupon_type" :key="dict.value" :label="dict.label" :value="dict.value" />
        </el-select>
      </el-form-item>
      <el-form-item label="卡券状态" prop="status">
        <el-select v-model="queryParams.status" @change="selectChange" placeholder="卡券状态" clearable
          style="width: 120px">
          <el-option v-for="dict in sys_coupon_status" :key="dict.value" :label="dict.label" :value="dict.value" />
        </el-select>
      </el-form-item>
      <el-form-item label="删除状态" prop="status">
        <el-select v-model="queryParams.delFlag" @change="selectChange" placeholder="删除状态" clearable
          style="width: 120px">
          <el-option v-for="dict in sys_del_status" :key="dict.value" :label="dict.label" :value="dict.value" />
        </el-select>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" icon="Search" @click="handleQuery">搜索</el-button>
        <el-button icon="Refresh" @click="resetQuery">重置</el-button>
      </el-form-item>
    </el-form>

    <el-row :gutter="10" class="mb8">
      <el-col :span="1.5">
        <el-button type="primary" plain icon="Plus" @click="handleAdd" v-hasPermi="['system:coupon:add']">新增</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button type="success" :disabled="selectedList.length == 0" plain icon="Sell"
          @click="handleShowSell">卡券销售</el-button>
      </el-col>
      <right-toolbar v-model:showSearch="showSearch" @queryTable="getList" :columns="columns"></right-toolbar>
    </el-row>

    <el-table v-loading="loading" :data="couponList" @selection-change="handleSelectionChange">
      <el-table-column type="selection" width="55" align="center" />
      <!-- <el-table-column label="卡券唯一标识ID" align="center" prop="couponId" /> -->
      <el-table-column label="卡券名称" align="center" prop="couponTitle" v-if="columns[0].visible" />
      <el-table-column label="卡券编码" align="center" prop="couponNumber" v-if="columns[1].visible" width="180" />
      <el-table-column label="卡券类型" align="center" prop="couponType" v-if="columns[2].visible">
        <template #default="scope">
          <dict-tag :options="sys_coupon_type" :value="scope.row.couponType" />
        </template>
      </el-table-column>
      <el-table-column label="卡券面值" align="center" prop="couponValue" v-if="columns[3].visible" />
      <el-table-column label="最低消费金额" align="center" prop="minSpend" width="120" v-if="columns[4].visible" />
      <el-table-column label="客户可见" align="center" prop="customerInvalid" v-if="columns[5].visible">
        <template #default="scope">
          <dict-tag :options="sys_coupon_customer_invalid" :value="scope.row.customerInvalid" />
        </template>
      </el-table-column>
      <el-table-column label="总量限制" align="center" prop="customerSaleTotal" v-if="columns[6].visible" />
      <el-table-column label="单用户数量限制" align="center" prop="customerSaleCount" width="120" v-if="columns[7].visible" />
      <el-table-column label="有效时间-起" align="center" prop="validFrom" width="100" v-if="columns[9].visible">
        <template #default="scope">
          <span>{{ parseTime(scope.row.validFrom, '{y}-{m}-{d} {h}:{i}:{s}') }}</span>
        </template>
      </el-table-column>
      <el-table-column label="有效时间-止" align="center" prop="validTo" width="100" v-if="columns[10].visible">
        <template #default="scope">
          <span>{{ parseTime(scope.row.validTo, '{y}-{m}-{d} {h}:{i}:{s}') }}</span>
        </template>
      </el-table-column>
      <el-table-column label="自动延期" align="center" prop="autoDelay" v-if="columns[11].visible" />
      <el-table-column label="卡券价值" align="center" prop="usageValue" v-if="columns[12].visible" />
      <el-table-column label="限制条件" align="center" prop="usageLimit" v-if="columns[13].visible" />
      <el-table-column label="适用品类" align="center" prop="applicableCategory" v-if="columns[14].visible">
        <template #default="scope">
          <dict-tag :options="sys_cloth_cate" :value="scope.row.applicableCategory" />
        </template>
      </el-table-column>
      <el-table-column label="适用分类" align="center" prop="applicableStyle" v-if="columns[15].visible">
        <template #default="scope">
          <dict-tag :options="sys_cloth_style" :value="scope.row.applicableStyle" />
        </template>
      </el-table-column>
      <el-table-column label="适用衣物" align="center" prop="applicableCloths" v-if="columns[16].visible">
        <template #default="scope">
          <el-tag type="primary"
            v-for="item, index in scope.row.applicableCloths ? scope.row.applicableCloths.split(',') : []" :key="index">{{
            item }}</el-tag>
        </template>
      </el-table-column>
      <el-table-column label="卡券状态" align="center" prop="status">
        <template #default="scope">
          <dict-tag :options="sys_coupon_status" :value="scope.row.status" />
        </template>
      </el-table-column>
      <el-table-column label="卡券描述" align="center" prop="remark" v-if="columns[17].visible" />
      <el-table-column label="操作" align="center" class-name="small-padding" width="140">
        <template #default="scope">
          <el-button link type="primary" icon="Edit" @click="handleUpdate(scope.row)"
            v-hasPermi="['system:coupon:edit']">修改</el-button>
          <el-button link type="primary" icon="Delete" @click="handleDelete(scope.row)"
            v-hasPermi="['system:coupon:remove']">删除</el-button>
        </template>
      </el-table-column>
    </el-table>

    <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
      v-model:limit="queryParams.pageSize" @pagination="getList" />

    <!-- 添加或修改卡券对话框 -->
    <el-dialog :title="title" v-model="open" width="650px" lock-scroll modal :close-on-click-modal="false" append-to-body>
      <el-form ref="couponRef" :model="form" :rules="rules" label-width="110px">
        <el-form-item label="卡券名称" prop="couponTitle">
          <el-input v-model="form.couponTitle" placeholder="请输入卡券名称" />
        </el-form-item>
        <el-row>
          <el-col :span="12">
            <el-tooltip content="售价" placement="top">
              <el-form-item label="卡券面值" prop="couponValue">
                <el-input-number v-model="form.couponValue" @change="form.usageValue=form.couponValue" controls-position="right" placeholder="请输入卡券面值" />
              </el-form-item>
            </el-tooltip>
          </el-col>
          <el-col :span="12">
            <el-form-item label="卡券价值" prop="usageValue">
              <el-input-number v-model="form.usageValue" :min="form.couponValue" controls-position="right" placeholder="请输入卡券价值" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="卡券类别">
              <el-select v-model="form.couponType" placeholder="卡券类别" clearable>
                <el-option v-for="dict in sys_coupon_type" :key="dict.value" :label="dict.label" :value="dict.value" />
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="卡券状态">
              <el-select v-model="form.status" placeholder="卡券状态" clearable>
                <el-option v-for="dict in sys_coupon_status" :key="dict.value" :label="dict.label"
                  :value="dict.value" />
              </el-select>
            </el-form-item>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="最低消费金额" prop="minSpend">
              <el-input v-model="form.minSpend" placeholder="请输入最低消费金额" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-tooltip content="对于折扣券的上限优惠金额" placement="top">
              <el-form-item label="限制条件" prop="usageLimit">
                <el-input-number v-model="form.usageLimit" controls-position="right" placeholder="对于折扣券的上限优惠金额" />
              </el-form-item>
            </el-tooltip>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="客户可见" prop="customerInvalid">
              <el-radio-group v-model="form.customerInvalid">
                <el-radio v-for="dict in sys_coupon_customer_invalid" :key="dict.value" :label="dict.label"
                  :value="dict.value">{{ dict.label }}</el-radio>
              </el-radio-group>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="自动延期" prop="autoDelay">
              <el-radio-group v-model="form.autoDelay">
                <el-radio v-for="dict in sys_coupon_auto_delay" :key="dict.value" :label="dict.label"
                  :value="dict.value">{{
                    dict.label }}</el-radio>
              </el-radio-group>
            </el-form-item>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="总量限制" prop="customerSaleTotal">
              <el-input-number v-model="form.customerSaleTotal" controls-position="right" placeholder="请输入总量限制" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="单用户数量限制" prop="customerSaleCount">
              <el-input-number v-model="form.customerSaleCount" controls-position="right" placeholder="请输入单用户数量限制" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="有效期-起" prop="validFrom">
              <el-date-picker clearable v-model="form.validFrom" type="datetime" value-format="YYYY-MM-DD HH:mm:ss"
                placeholder="请选择有效期-起">
              </el-date-picker>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="有效期-止" prop="validTo">
              <el-date-picker clearable v-model="form.validTo" type="datetime" value-format="YYYY-MM-DD HH:mm:ss"
                placeholder="请选择有效期-止">
              </el-date-picker>
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item label="适用品类" prop="applicableCategory">
          <el-select v-model="form.applicableCategory" placeholder="适用品类" clearable>
            <el-option v-for="dict in sys_cloth_cate" :key="dict.value" :label="dict.label" :value="dict.value" />
          </el-select>
        </el-form-item>
        <el-form-item label="适用分类" prop="applicableStyle">
          <el-select v-model="form.applicableStyle" placeholder="适用分类" clearable>
            <el-option v-for="dict in sys_cloth_style" :key="dict.value" :label="dict.label" :value="dict.value" />
          </el-select>
        </el-form-item>
        <el-form-item label="适用衣物" prop="applicableCloths">
          <el-select v-model="form.applicableCloths" placeholder="适用衣物" clearable multiple filterable remote
            reserve-keyword remote-show-suffix :remote-method="getClothingList" :loading="clothListloading">
            <el-option v-for="item in clothList" :key="item.clothingId"
              :label="item.clothingName + '-' + item.clothingNumber" :value="item.clothingName" />
          </el-select>
        </el-form-item>
        <el-form-item label="卡券描述" prop="remark">
          <el-input v-model="form.remark" type="textarea" placeholder="请输入内容" />
        </el-form-item>
      </el-form>
      <template #footer>
        <div class="dialog-footer">
          <el-button type="primary" @click="submitForm">确 定</el-button>
          <el-button @click="cancel">取 消</el-button>
        </div>
      </template>
    </el-dialog>

    <!-- show sell coupon -->
    <el-dialog v-model="showSell" title="销售卡券" width="800px">
      <el-form ref="sellFormRef" :model="sellForm" label-width="90px" :rules="sellRules">
        <el-form-item label="会员身份" prop="userId">
          <el-select v-model="sellForm.userId" filterable remote reserve-keyword placeholder="请输入手机号码搜索"
            remote-show-suffix :remote-method="searchUserByTel" :loading="searchUserloading" style="width: 240px">
            <el-option v-for="item in userList" :key="item.userId" :label="item.nickName + '\t' + item.phonenumber"
              :value="item.userId" />
          </el-select>
        </el-form-item>
        <el-row>
          <h3 class="title">卡券信息</h3>
        </el-row>
        <el-table :data="selectedList" border>
          <el-table-column label="序号" align="center" type="index" width="60" />
          <el-table-column label="卡券名称" align="center" key="couponTitle" prop="couponTitle" />
          <el-table-column label="有效期" align="center" key="validTo" prop="validTo" />
          <el-table-column label="数量" align="center" key="validTo">
            <template #default="scope">
              <el-input-number v-model="scope.row.count" :min="0"
                :max="scope.row.customerSaleCount < scope.row.customerSaleTotal ? scope.row.customerSaleCount : scope.row.customerSaleTotal"
                controls-position="right" />
            </template>
          </el-table-column>
        </el-table>
        <el-row>
          <h3 class="title">支付方式</h3>
        </el-row>
        <el-row>
          <el-form-item>
            <el-radio-group v-model="sellForm.paymentMethod">
              <el-radio v-for="dict in sys_payment_method" :key="dict.value" :label="dict.label" :value="dict.value" />
            </el-radio-group>
          </el-form-item>
        </el-row>
        <el-row>
          <h3 class="title">备注信息</h3>
        </el-row>
        <el-row>
          <el-form-item style="width: 100%;">
            <el-input type="textarea" v-model="sellForm.remark" placeholder="备注信息" />
          </el-form-item>
        </el-row>
        <el-row>
          <el-col :span="21" class="cash">
            总价：{{ totalPrice }}
          </el-col>
          <el-col :span="3" justify="center" align="right">
            <el-button type="primary" @click="buy">立即购买</el-button>
          </el-col>
        </el-row>
      </el-form>
    </el-dialog>
  </div>
</template>

<script setup name="Coupon">
import { listCoupon, getCoupon, delCoupon, addCoupon, updateCoupon, buyCoupon } from "@/api/system/coupon";
import { listUser } from "@/api/system/user";
import { listClothing } from "@/api/system/clothing";
import { ref, computed } from "vue";

const { proxy } = getCurrentInstance();

const {
  sys_coupon_status,
  sys_del_status,
  sys_coupon_type,
  sys_coupon_customer_invalid,
  sys_coupon_auto_delay,
  sys_cloth_style,
  sys_cloth_cate,
  sys_payment_method
} =
  proxy.useDict(
    "sys_coupon_status",
    "sys_del_status",
    "sys_coupon_type",
    "sys_coupon_customer_invalid",
    "sys_coupon_auto_delay",
    "sys_cloth_style",
    "sys_cloth_cate",
    "sys_payment_method"
  );

const couponList = ref([]);
const userList = ref([]);
const clothList = ref([]);
const open = ref(false);
const showSell = ref(false);
const searchUserloading = ref(false);
const clothListloading = ref(false);
const loading = ref(true);
const showSearch = ref(true);
const ids = ref([]);
const total = ref(0);
const title = ref("");

// 列显隐信息
const columns = ref([
  { key: 0, label: `卡券名称`, visible: true },
  { key: 1, label: `卡券编码`, visible: false },
  { key: 2, label: `卡券类别`, visible: true },
  { key: 3, label: `卡券面值`, visible: true },
  { key: 4, label: `最低消费金额`, visible: true },
  { key: 5, label: `客户可见`, visible: true },
  { key: 6, label: `总量限制`, visible: true },
  { key: 7, label: `单用户数量限制`, visible: true },
  { key: 8, label: `有效期-起`, visible: true },
  { key: 9, label: `有效期-止`, visible: true },
  { key: 10, label: `自动延期`, visible: true },
  { key: 11, label: `卡券价值`, visible: true },
  { key: 12, label: `限制条件`, visible: true },
  { key: 13, label: `适用品类`, visible: true },
  { key: 14, label: `适用分类`, visible: true },
  { key: 15, label: `适用衣物`, visible: true },
  { key: 16, label: `卡券状态`, visible: true },
  { key: 17, label: `描述`, visible: true },
]);

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
      { required: true, message: "卡券面值不能为空", trigger: "blur" }
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

/* 动态计算销售卡券时的总金额 */
const totalPrice = computed(() => {
  return selectedList.value.reduce((accumulator, curItem) => {
    return accumulator + curItem.couponValue * curItem.count;
  }, 0);
});

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
    customerSaleTotal: null,
    customerSaleCount: null,
    validFrom: null,
    validTo: null,
    autoDelay: "2",
    usageValue: null,
    usageLimit: null,
    applicableCategory: null,
    applicableStyle: null,
    applicableCloths: null,
    status: "0",
    remark: null
  };
  proxy.resetForm("couponRef");
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
function handleSelectionChange(selection) {
  selection.forEach(item => item.count = 1);
  selectedList.value = selection;
}

/** 新增按钮操作 */
function handleAdd() {
  reset();
  open.value = true;
  title.value = "添加卡券";
}

/** 修改按钮操作 */
function handleUpdate(row) {
  reset();
  const _couponId = row.couponId || ids.value
  getCoupon(_couponId).then(response => {
    form.value = response.data;
    open.value = true;
    title.value = "修改卡券";
  });
}

/** 提交按钮 */
function submitForm() {
  proxy.$refs["couponRef"].validate(valid => {
    if (valid) {
      if (form.value.couponId != null) {
        updateCoupon(form.value).then(response => {
          proxy.$modal.msgSuccess("修改成功");
          open.value = false;
          getList();
        });
      } else {
        if (form.value.applicableCloths) {
          form.value.applicableCloths = form.value.applicableCloths.join(",");
        }
        addCoupon(form.value).then(response => {
          proxy.$modal.msgSuccess("新增成功");
          open.value = false;
          getList();
        });
      }
    }
  });
}

/** 删除按钮操作 */
function handleDelete(row) {
  const _couponIds = row.couponId || ids.value;
  proxy.$modal.confirm('是否确认删除卡券编号为"' + _couponIds + '"的数据项？').then(function () {
    return delCoupon(_couponIds);
  }).then(() => {
    getList();
    proxy.$modal.msgSuccess("删除成功");
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
  proxy.resetForm("sellFormRef");
}

function handleShowSell() {
  showSell.value = true;
  resetSellForm();
}
/* 根据手机号搜索用户列表 */
function searchUserByTel(tel) {
  // if (!tel || tel.length < 4) { return }
  searchUserloading.value = true;
  listUser({ phonenumber: tel }).then(res => {
    searchUserloading.value = false;
    userList.value = res.rows;
  });
}

/* 购买卡券 */
function buy() {
  proxy.$refs["sellFormRef"].validate(valid => {
    if (valid) {
      const coupons = selectedList.value.filter(item => item.count > 0).map(({ couponId, count }) => ({ couponId, count }));
      sellForm.value.coupons = coupons;
      console.log(sellForm.value);
      buyCoupon(sellForm.value).then(res => {
        proxy.$modal.msgSuccess("购买成功");
        showSell.value = false;
      }).catch();
    }
  });
}

/* 获取衣物列表 */
function getClothingList(name) {
  clothListloading.value = true;
  listClothing({ clothingName: name }).then(res => {
    clothList.value = res.rows;
    clothListloading.value = false;
  });
}
getList();
</script>

<style scoped>
.title {
  border-bottom: 1px solid gray;
}

.cash {
  display: flex;
  justify-content: right;
  align-items: center;
}
</style>