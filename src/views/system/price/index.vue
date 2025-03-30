<template>
  <div class="app-container">
    <el-card class="search-card" v-show="showSearch">
      <el-form :model="queryParams" ref="queryRef" :inline="true" label-width="68px">
        <el-form-item label="订单类别" prop="orderType">
          <el-select v-model="queryParams.orderType" @change="selectChange" placeholder="订单类别" clearable
            style="width: 100px;">
            <el-option v-for="dict in sys_price_order_type" :key="dict.value" :label="dict.label" :value="dict.value" />
          </el-select>
        </el-form-item>
        <el-form-item label="价格名称" prop="priceName">
          <el-input v-model="queryParams.priceName" placeholder="请输入价格名称" clearable @keyup.enter="handleQuery" />
        </el-form-item>
        <el-form-item>
          <el-button class="hover-flow" type="primary" icon="Search" @click="handleQuery">搜索</el-button>
          <el-button class="hover-flow" icon="Refresh" @click="resetQuery">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <el-card class="table-card">
      <el-row :gutter="10" class="mb8">
        <el-col :span="1.5">
          <el-button type="primary" plain icon="Plus" @click="handleAdd">新增</el-button>
        </el-col>
        <el-col :span="1.5">
          <el-button type="success" plain icon="Edit" :disabled="ids.length == 0"
            @click="showUpdateRefNum = true">设置使用计数</el-button>
        </el-col>
        <el-col :span="1.5">
          <el-button type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete">删除</el-button>
        </el-col>
        <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
      </el-row>

      <el-table v-loading="loading" :data="priceList" v-model="selectedList" @selection-change="handleSelectionChange"
        class="modern-table" border stripe>
        <el-table-column type="selection" width="55" align="center" />
        <el-table-column label="价格编码" align="center" prop="priceNumber" />
        <el-table-column label="订单类别" align="center" prop="orderType">
          <template #default="scope">
            <dict-tag :options="sys_price_order_type" :value="scope.row.orderType" />
          </template>
        </el-table-column>
        <el-table-column label="价格名称" align="center" prop="priceName" />
        <el-table-column label="价格" align="center" prop="priceValue" />
        <el-table-column label="折扣系数" align="center" prop="priceDiscount" />
        <el-table-column label="显示顺序" align="center" prop="orderNum" />
        <el-table-column label="使用计数" align="center" prop="refNum" />
        <el-table-column label="状态" align="center" width="100">
          <template #default="scope">
            <el-switch v-model="scope.row.status" active-value="0" inactive-value="1"
              @change="handleStatusChange(scope.row)"></el-switch>
          </template>
        </el-table-column>
        <el-table-column label="备注" align="center" prop="remark" show-overflow-tooltip />
        <el-table-column label="创建时间" align="center" prop="createTime" width="180">
          <template #default="scope">
            <span>{{ formatTime(scope.row.createTime) }}</span>
          </template>
        </el-table-column>
        <el-table-column label="操作" align="center" class-name="small-padding fixed-width">
          <template #default="scope">
            <el-button link type="primary" icon="Edit" @click="handleUpdate(scope.row)">修改</el-button>
            <el-button link type="primary" icon="Delete" @click="handleDelete(scope.row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>

      <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
        v-model:limit="queryParams.pageSize" @pagination="getList" />
    </el-card>

    <!-- 添加或修改价格管理对话框 -->
    <el-dialog v-model="open" :show-close="false" width="550px" @opened="refNumberGetFocus"
      @closed="refNumberFocus = false" align-center class="price-dialog">
      <template #header>
        <div class="dialog-header hover-flow">
          <h2 class="dialog-title">{{ form.priceId ? '修改价格' : '新增价格' }}</h2>
          <el-button circle @click="cancel">
            <el-icon>
              <Close />
            </el-icon>
          </el-button>
        </div>
      </template>

      <el-form ref="priceRef" :model="form" :rules="rules" label-width="80px" class="price-form">
        <!-- 基本信息卡片 -->
        <div class="form-card hover-flow">
          <div class="card-header">
            <el-icon>
              <InfoFilled />
            </el-icon>
            <span>基本信息</span>
          </div>
          <div class="card-body">
            <el-row :gutter="20">
              <el-col :span="12">
                <el-form-item label="价格名称" prop="priceName">
                  <el-input v-model="form.priceName" placeholder="请输入价格名称" class="custom-input" />
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="订单类别" prop="orderType">
                  <el-select v-model="form.orderType" placeholder="请选择订单类别" class="custom-select">
                    <el-option v-for="dict in sys_price_order_type" :key="dict.value" :label="dict.label"
                      :value="dict.value" />
                  </el-select>
                </el-form-item>
              </el-col>
            </el-row>
          </div>
        </div>

        <!-- 价格设置卡片 -->
        <div class="form-card hover-flow">
          <div class="card-header">
            <el-icon>
              <Money />
            </el-icon>
            <span>价格设置</span>
          </div>
          <div class="card-body">
            <el-row :gutter="20">
              <el-col :span="12">
                <el-form-item label="价格" prop="priceValue">
                  <el-input-number v-model="form.priceValue" controls-position="right" placeholder="请输入价格"
                    :disabled="isPriceValueDisabled" class="custom-input-number">
                    <template #prefix>
                      <el-icon>
                        <Coin />
                      </el-icon>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="折扣系数" prop="priceDiscount">
                  <el-input-number v-model="form.priceDiscount" controls-position="right" placeholder="请输入折扣系数"
                    :disabled="isPriceDiscountDisabled" class="custom-input-number">
                    <template #prefix>
                      <el-icon>
                        <Discount />
                      </el-icon>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
            </el-row>

            <div class="price-info" v-if="form.priceValue || form.priceDiscount">
              <el-alert type="info" :closable="false" show-icon>
                <template #title>
                  <span>{{ form.priceValue ? '已设置固定价格' : '已设置折扣系数' }}</span>
                </template>
                <template #default>
                  <p>{{ form.priceValue && form.priceDiscount ? '价格和折扣系数只能设置一个' : '请设置价格或折扣系数其中一项' }}</p>
                </template>
              </el-alert>
            </div>
          </div>
        </div>

        <!-- 其他设置卡片 -->
        <div class="form-card hover-flow">
          <div class="card-header">
            <el-icon>
              <Setting />
            </el-icon>
            <span>其他设置</span>
          </div>
          <div class="card-body">
            <el-form-item label="状态" class="status-item">
              <el-radio-group v-model="form.status" class="custom-radio-group">
                <el-radio v-for="dict in sys_normal_disable" :key="dict.value" :value="dict.value" class="custom-radio">
                  {{ dict.label }}
                </el-radio>
              </el-radio-group>
            </el-form-item>

            <el-row :gutter="20">
              <el-col :span="12">
                <el-form-item label="显示顺序" prop="orderNum">
                  <el-input-number v-model="form.orderNum" :min="0" controls-position="right" placeholder="请输入显示顺序"
                    class="custom-input-number">
                    <template #prefix>
                      <el-icon>
                        <Sort />
                      </el-icon>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="使用计数" prop="refNum">
                  <el-input-number v-model="form.refNum" ref="refNum" :min="0" controls-position="right"
                    placeholder="请输入使用计数" class="custom-input-number">
                    <template #prefix>
                      <el-icon>
                        <Odometer />
                      </el-icon>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
            </el-row>

            <el-form-item label="备注" prop="remark">
              <el-input v-model="form.remark" type="textarea" placeholder="请输入内容" class="custom-textarea" :rows="3" />
            </el-form-item>
          </div>
        </div>
      </el-form>

      <template #footer>
        <div class="dialog-footer">
          <el-button class="hover-flow" type="primary" @click="submitForm" icon="Check"> 确 定</el-button>
          <el-button class="hover-flow"  type="danger"@click="cancel" icon="Close">取 消</el-button>
        </div>
      </template>
    </el-dialog>

    <!-- 修改使用次数对话框 -->
    <ref-count-editor v-model="showUpdateRefNum" :initial-value="tagNumForm.refNumber" title="修改使用次数"
      description="设置价格的使用计数值" @confirm="handleRefNumConfirm" @cancel="cancelUpdateRefNum" />
  </div>
</template>

<script setup name="Price">
import { listPricePagination, getPrice, delPrice, addPrice, updatePrice, updatePriceStatus, updatePriceRefNum } from "@/api/system/price";
import { listClothing } from "@/api/system/clothing";
import RefCountEditor from "@/components/RefCountEditor/index.vue";

const { proxy } = getCurrentInstance();
const { sys_price_order_type, sys_normal_disable } = proxy.useDict("sys_price_order_type", "sys_normal_disable");

const priceList = ref([]);
const clothList = ref([]);
const open = ref(false);
const showUpdateRefNum = ref(false);
const clothListloading = ref(false);
const loading = ref(true);
const showSearch = ref(true);
const ids = ref([]);
const single = ref(true);
const multiple = ref(true);
const total = ref(0);
const title = ref("");
const refNumberFocus = ref(false);
const refNum = ref();

const data = reactive({
  form: {},
  tagNumForm: {},
  queryParams: {
    pageNum: 1,
    pageSize: 10,
    orderType: null,
    refNum: null,
  },
  rules: {
    orderType: [
      { required: true, message: "订单类别不能为空", trigger: "change" }
    ],
    priceName: [
      { required: true, message: "价格名称不能为空", trigger: "blur" }
    ],
    orderNum: [
      { required: true, message: "显示顺序不能为空", trigger: "blur" }
    ],/* 
    applicableCloths: [
      { required: true, message: "适用衣物不能为空", trigger: "blur" }
    ], */
  },
  refNumFormRules: {
    refNumber: [{ required: true, message: "使用次数不能为空", trigger: "blur" }],
  }
});

const { queryParams, form, tagNumForm, rules, refNumFormRules } = toRefs(data);
// 是否禁用 priceValue 和 priceDiscount
const isPriceValueDisabled = ref(false);
const isPriceDiscountDisabled = ref(false);

// 监听 priceValue 和 priceDiscount 的变化
watch(
  () => form.value.priceValue,
  (newValue) => {
    // 如果 priceValue 有值，则禁用 priceDiscount，否则启用
    if (newValue) {
      isPriceDiscountDisabled.value = true;
    } else {
      isPriceDiscountDisabled.value = false;
    }
  }
);

watch(
  () => form.value.priceDiscount,
  (newValue) => {
    // 如果 priceDiscount 有值，则禁用 priceValue，否则启用
    if (newValue) {
      isPriceValueDisabled.value = true;
    } else {
      isPriceValueDisabled.value = false;
    }
  }
);
/** 查询价格管理列表 */
function getList() {
  loading.value = true;
  listPricePagination(queryParams.value).then(response => {
    priceList.value = response.rows;
    total.value = response.total;
    loading.value = false;
  });
}

function handleRefNumConfirm(refNumber) {
  updatePriceRefNum({ priceIds: ids.value, refNum: refNumber }).then(res => {
    proxy.notify.success("修改成功");
    showUpdateRefNum.value = false;
    tagNumForm.value.refNumber = null;
    getList();
  }).catch(() => {
    // 处理错误情况
  }).finally(() => {
    // 无论成功失败都执行
  });
}

// 取消按钮
function cancelUpdateRefNum() {
  showUpdateRefNum.value = false;
  tagNumForm.value = {};
}

// 取消按钮
function cancel() {
  open.value = false;
  reset();
}

// 表单重置
function reset() {
  form.value = {
    priceId: null,
    orderType: null,
    priceName: null,
    priceValue: null,
    priceDiscount: null,
    applicableCloths: null,
    status: "0",
    orderNum: 0,
    refNum: 0,
    remark: null,
  };
  proxy.resetForm("priceRef");
}

/** 标签状态修改 */
function handleStatusChange(row) {
  let text = row.status === "0" ? "启用" : "停用";
  proxy.$modal.confirm('确认要' + text + '"' + row.priceName + '"标签吗?').then(function () {
    return updatePriceStatus({ priceId: row.priceId, status: row.status });
  }).then(() => {
    proxy.notify.success(text + "成功");
  }).catch(function () {
    row.status = row.status === "0" ? "1" : "0";
  });
}

/** 搜索按钮操作 */
function handleQuery() {
  queryParams.value.pageNum = 1;
  getList();
}

/* 获取衣物列表 */
function getClothingList(name) {
  clothListloading.value = true;
  listClothing({ clothingName: name }).then(res => {
    clothList.value = res.rows;
    clothListloading.value = false;
  });
}
/** 重置按钮操作 */
function resetQuery() {
  proxy.resetForm("queryRef");
  handleQuery();
}

// 多选框选中数据
function handleSelectionChange(selection) {
  ids.value = selection.map(item => item.priceId);
  single.value = selection.length != 1;
  multiple.value = !selection.length;
}

/** 新增按钮操作 */
function handleAdd() {
  reset();
  open.value = true;
}

/** 修改按钮操作 */
function handleUpdate(row) {
  reset();
  const _priceId = row.priceId || ids.value
  getPrice(_priceId).then(response => {
    form.value = response;
    if (form.value.applicableCloths) {
      form.value.applicableClothsArr = form.value.applicableCloths.split(",");
    }
    open.value = true;
    title.value = "修改价格管理";
  });
}

/** 提交按钮 */
function submitForm() {
  proxy.$refs["priceRef"].validate(valid => {
    if (valid) {
      if (!form.value.priceValue && !form.value.priceDiscount) {
        proxy.notify.error('价格和折扣至少填写一个');
        return;
      }
      if (form.value.applicableClothsArr && form.value.applicableClothsArr.length > 0) {
        form.value.applicableCloths = form.value.applicableClothsArr.join(",");
        delete form.value.applicableClothsArr;
      }
      if (form.value.priceId != null) {
        updatePrice(form.value).then(response => {
          proxy.notify.success("修改成功");
          open.value = false;
          getList();
        });
      } else {
        addPrice(form.value).then(response => {
          proxy.notify.success("新增成功");
          open.value = false;
          getList();
        });
      }
    }
  });
}

/** 删除按钮操作 */
function handleDelete(row) {
  const _priceIds = row.priceId || ids.value;
  proxy.$modal.confirm('是否确认删除价格管理编号为"' + _priceIds + '"的数据项？').then(function () {
    return delPrice(_priceIds);
  }).then(() => {
    getList();
    proxy.notify.success("删除成功");
  }).catch(() => { });
}

/* 点击修改使用计数时，输入框获取焦点 */
function refNumberGetFocus() {
  if (refNumberFocus.value) {
    refNum.value.focus();
  }
}

/* 订单类别变化触发查询 */
function selectChange() {
  queryParams.value.pageNum = 1;
  getList();
}

getList();
</script>

<style scoped>
.price-dialog :deep(.el-dialog__header) {
  margin: 0;
  padding: 15px 20px;
  border-bottom: 1px solid var(--el-border-color-light);
}

.price-form {
  padding: 10px 0;
}

.form-card {
  margin-bottom: 20px;
  border-radius: 8px;
  box-shadow: var(--el-box-shadow-light);
  overflow: hidden;
}

.card-header {
  display: flex;
  align-items: center;
  padding: 12px 15px;
  background-color: var(--el-color-primary-light-9);
  border-bottom: 1px solid var(--el-border-color-light);
  color: var(--el-color-primary);
  font-weight: 600;
}

.card-header .el-icon {
  margin-right: 8px;
  font-size: 16px;
}

.card-body {
  padding: 15px;
}

.custom-input :deep(.el-input__wrapper),
.custom-select :deep(.el-input__wrapper),
.custom-input-number :deep(.el-input__wrapper) {
  box-shadow: 0 0 0 1px var(--el-border-color) inset;
  border-radius: 4px;
  padding: 0 12px;
  transition: all 0.3s;
}

.custom-input :deep(.el-input__wrapper:hover),
.custom-select :deep(.el-input__wrapper:hover),
.custom-input-number :deep(.el-input__wrapper:hover) {
  box-shadow: 0 0 0 1px var(--el-color-primary-light-5) inset;
}

.custom-input :deep(.el-input__wrapper.is-focus),
.custom-select :deep(.el-input__wrapper.is-focus),
.custom-input-number :deep(.el-input__wrapper.is-focus) {
  box-shadow: 0 0 0 1px var(--el-color-primary) inset;
}

.custom-textarea :deep(.el-textarea__inner) {
  box-shadow: 0 0 0 1px var(--el-border-color) inset;
  border-radius: 4px;
  transition: all 0.3s;
}

.custom-textarea :deep(.el-textarea__inner:hover) {
  box-shadow: 0 0 0 1px var(--el-color-primary-light-5) inset;
}

.custom-textarea :deep(.el-textarea__inner:focus) {
  box-shadow: 0 0 0 1px var(--el-color-primary) inset;
}

.price-info {
  margin-top: 10px;
}

</style>
