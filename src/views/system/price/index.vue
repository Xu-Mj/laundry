<template>
  <div class="app-container">
    <transition name="height-fade">
      <el-card class="search-card" v-show="showSearch">
        <el-form :model="queryParams" ref="queryRef" :inline="true" label-width="68px">
          <el-form-item label="订单类别" prop="orderType">
            <el-select v-model="queryParams.orderType" @change="selectChange" placeholder="订单类别" clearable
              style="width: 100px;">
              <el-option v-for="dict in OrderSource" :key="dict.value" :label="dict.label" :value="dict.value" />
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
    </transition>

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
        <template #empty>
          <el-empty description="暂无数据" />
        </template>
        <el-table-column type="selection" width="55" align="center" />
        <el-table-column label="价格编码" align="center" prop="priceNumber" />
        <el-table-column label="订单类别" align="center" prop="orderType">
          <template #default="scope">
            <!-- <dict-tag :options="sys_price_order_type" :value="scope.row.orderType" /> -->
            <el-tag :type="OrderSourceMap[scope.row.orderType]?.type">
              {{ OrderSourceMap[scope.row.orderType]?.label }}
            </el-tag>
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

    <!-- 价格创建弹窗组件 -->
    <PriceCreateDialog v-model="open" :price-id="currentPriceId" @success="handleDialogSuccess"
      @cancel="handleDialogCancel" />

    <!-- 修改使用次数对话框 -->
    <ref-count-editor v-model="showUpdateRefNum" :initial-value="tagNumForm.refNumber" title="修改使用次数"
      description="设置价格的使用计数值" @confirm="handleRefNumConfirm" @cancel="cancelUpdateRefNum" />
  </div>
</template>

<script setup name="Price">
import { listPricePagination, delPrice, updatePriceStatus, updatePriceRefNum } from "@/api/system/price";
import { listClothing } from "@/api/system/clothing";
import RefCountEditor from "@/components/RefCountEditor/index.vue";
import PriceCreateDialog from "@/components/PriceCreateDialog/index.vue";
import { OrderSourceMap, OrderSource } from "@/constants";

const { proxy } = getCurrentInstance();

const priceList = ref([]);
const clothList = ref([]);
const open = ref(false);
const currentPriceId = ref(null);
const showUpdateRefNum = ref(false);
const clothListloading = ref(false);
const loading = ref(true);
const showSearch = ref(true);
const ids = ref([]);
const single = ref(true);
const multiple = ref(true);
const total = ref(0);

const data = reactive({
  tagNumForm: {},
  queryParams: {
    pageNum: 1,
    pageSize: 10,
    orderType: null,
    refNum: null,
  },
  refNumFormRules: {
    refNumber: [{ required: true, message: "使用次数不能为空", trigger: "blur" }],
  }
});

const { queryParams, tagNumForm, refNumFormRules } = toRefs(data);
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
  updatePriceRefNum({ clothPriceIds: ids.value, refNum: refNumber }).then(res => {
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

// 弹窗成功回调
function handleDialogSuccess() {
  getList();
}

// 弹窗取消回调
function handleDialogCancel() {
  currentPriceId.value = null;
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
  currentPriceId.value = null;
  open.value = true;
}

/** 修改按钮操作 */
function handleUpdate(row) {
  const _priceId = row.priceId || ids.value;
  currentPriceId.value = _priceId;
  open.value = true;
}



/** 删除按钮操作 */
function handleDelete(row) {
  const _priceIds = row.priceId || ids.value;

  // 获取要删除的价格名称
  let confirmMessage;

  if (row.priceId) {
    // 单个删除
    confirmMessage = `是否确认删除价格"${row.priceName}"?`;
  } else {
    // 批量删除
    const priceNames = priceList.value
      .filter(item => ids.value.includes(item.priceId))
      .map(item => item.priceName)
      .join("、");

    confirmMessage = `是否确认删除以下价格: ${priceNames}?`;
  }

  proxy.$modal.confirm(confirmMessage, "警告", {
    confirmButtonText: "确定",
    cancelButtonText: "取消",
    type: "warning"
  }).then(function () {
    return delPrice(_priceIds);
  }).then(() => {
    getList();
    proxy.notify.success("删除成功");
  }).catch(() => { });
}



/* 订单类别变化触发查询 */
function selectChange() {
  queryParams.value.pageNum = 1;
  getList();
}

getList();
</script>

<style scoped>
/* 页面样式保持不变 */
</style>
