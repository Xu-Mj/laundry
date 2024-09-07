<template>
  <div class="app-container">
    <el-form :model="queryParams" ref="queryRef" :inline="true" v-show="showSearch" label-width="68px">
      <el-form-item label="订单类别" prop="orderType">
        <el-select v-model="queryParams.orderType" placeholder="订单类别" clearable style="width: 100px;">
          <el-option v-for="dict in sys_price_order_type" :key="dict.value" :label="dict.label" :value="dict.value" />
        </el-select>
      </el-form-item>
      <el-form-item label="衣物名称" prop="applicableDegree">
        <el-input v-model="queryParams.applicableDegree" placeholder="请输入衣物名称" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" icon="Search" @click="handleQuery">搜索</el-button>
        <el-button icon="Refresh" @click="resetQuery">重置</el-button>
      </el-form-item>
    </el-form>

    <el-row :gutter="10" class="mb8">
      <el-col :span="1.5">
        <el-button type="primary" plain icon="Plus" @click="handleAdd" v-hasPermi="['system:price:add']">新增</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button type="success" plain icon="Edit" :disabled="single" @click="handleUpdate"
          v-hasPermi="['system:price:edit']">设置使用计数</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete"
          v-hasPermi="['system:price:remove']">删除</el-button>
      </el-col>
      <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
    </el-row>

    <el-table v-loading="loading" :data="priceList" @selection-change="handleSelectionChange">
      <el-table-column type="selection" width="55" align="center" />
      <!-- <el-table-column label="唯一标识ID" align="center" prop="priceId" /> -->
      <el-table-column label="价格编码" align="center" prop="priceNumber" />
      <el-table-column label="订单类别" align="center" prop="orderType">
        <template #default="scope">
          <dict-tag :options="sys_price_order_type" :value="scope.row.orderType" />
        </template>
      </el-table-column>
      <el-table-column label="价格名称" align="center" prop="priceName" />
      <el-table-column label="价格" align="center" prop="priceValue" />
      <el-table-column label="折扣系数" align="center" prop="priceDiscount" />
      <el-table-column label="适用衣物" align="center" prop="applicableCloths">
        <template #default="scope">
          <el-tag type="primary"
            v-for="item, index in scope.row.applicableCloths ? scope.row.applicableCloths.split(',') : []"
            :key="index">{{
              item }}</el-tag>
        </template>
      </el-table-column>
      <el-table-column label="显示顺序" align="center" prop="orderNum" />
      <el-table-column label="使用计数" align="center" prop="clothingDegree" />
      <el-table-column label="备注" align="center" prop="remark" />
      <el-table-column label="创建时间" align="center" prop="createdAt" width="180">
        <template #default="scope">
          <span>{{ parseTime(scope.row.createdAt, '{y}-{m}-{d}') }}</span>
        </template>
      </el-table-column>
      <el-table-column label="更新时间" align="center" prop="updatedAt" width="180">
        <template #default="scope">
          <span>{{ parseTime(scope.row.updatedAt, '{y}-{m}-{d}') }}</span>
        </template>
      </el-table-column>
      <el-table-column label="操作" align="center" class-name="small-padding fixed-width">
        <template #default="scope">
          <el-button link type="primary" icon="Edit" @click="handleUpdate(scope.row)"
            v-hasPermi="['system:price:edit']">修改</el-button>
          <el-button link type="primary" icon="Delete" @click="handleDelete(scope.row)"
            v-hasPermi="['system:price:remove']">删除</el-button>
        </template>
      </el-table-column>
    </el-table>

    <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
      v-model:limit="queryParams.pageSize" @pagination="getList" />

    <!-- 添加或修改价格管理对话框 -->
    <el-dialog :title="title" v-model="open" width="500px" @opened="refNumberGetFocus" @closed="refNumberFocus = false"
      append-to-body>
      <el-form ref="priceRef" :model="form" :rules="rules" label-width="80px">
        <el-row>
          <el-col :span="12">
            <el-form-item label="价格名称" prop="priceName">
              <el-input v-model="form.priceName" placeholder="请输入价格名称" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="价格" prop="priceValue">
              <el-input v-model="form.priceValue" placeholder="请输入价格" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="订单类型" prop="orderType">
              <el-select v-model="form.orderType" placeholder="请选择订单类型">
                <el-option v-for="dict in sys_price_order_type" :key="dict.value" :label="dict.label"
                  :value="dict.value" />
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="折扣系数" prop="priceDiscount">
              <el-input v-model="form.priceDiscount" placeholder="请输入折扣系数" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item label="适用衣物" prop="applicableCloths">
          <el-select v-model="form.applicableClothsArr" placeholder="适用衣物" clearable multiple filterable remote
            reserve-keyword remote-show-suffix :remote-method="getClothingList" :loading="clothListloading">
            <el-option v-for="item in clothList" :key="item.clothingId"
              :label="item.clothingName + '-' + item.clothingNumber" :value="item.clothingName" />
          </el-select>
        </el-form-item>
        <el-row>
          <el-col :span="12">
            <el-form-item label="显示顺序" prop="orderNum">
              <el-input-number v-model="form.orderNum" :min="0" controls-position="right" placeholder="请输入显示顺序" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="使用计数" prop="clothingDegree">
              <el-input-number v-model="form.clothingDegree" ref="refNum" :min="0" controls-position="right"
                placeholder="请输入使用计数" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item label="备注" prop="remark">
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
  </div>
</template>

<script setup name="Price">
import { listPrice, getPrice, delPrice, addPrice, updatePrice } from "@/api/system/price";
import { listClothing } from "@/api/system/clothing";

const { proxy } = getCurrentInstance();
const { sys_price_order_type } = proxy.useDict("sys_price_order_type");

const priceList = ref([]);
const clothList = ref([]);
const open = ref(false);
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
  queryParams: {
    pageNum: 1,
    pageSize: 10,
    orderType: null,
    clothingDegree: null,
  },
  rules: {
    orderType: [
      { required: true, message: "订单类别不能为空", trigger: "change" }
    ],
    priceName: [
      { required: true, message: "价格名称不能为空", trigger: "blur" }
    ],
    priceValue: [
      { required: true, message: "价格不能为空", trigger: "blur" }
    ],
    orderNum: [
      { required: true, message: "显示顺序不能为空", trigger: "blur" }
    ],
    applicableCloths: [
      { required: true, message: "适用衣物不能为空", trigger: "blur" }
    ],
  }
});

const { queryParams, form, rules } = toRefs(data);

/** 查询价格管理列表 */
function getList() {
  loading.value = true;
  listPrice(queryParams.value).then(response => {
    priceList.value = response.rows;
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
    priceId: null,
    orderType: null,
    priceName: null,
    priceValue: null,
    priceDiscount: null,
    applicableCloths: null,
    orderNum: 0,
    clothingDegree: 0,
    remark: null,
  };
  proxy.resetForm("priceRef");
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
  title.value = "添加价格管理";
}

/** 修改按钮操作 */
function handleUpdate(row) {
  reset();
  const _priceId = row.priceId || ids.value
  getPrice(_priceId).then(response => {
    form.value = response.data;
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
      if (form.value.applicableClothsArr && form.value.applicableClothsArr.length > 0) {
        form.value.applicableCloths = form.value.applicableClothsArr.join(",");
        delete form.value.applicableClothsArr;
      }
      if (form.value.priceId != null) {
        updatePrice(form.value).then(response => {
          proxy.$modal.msgSuccess("修改成功");
          open.value = false;
          getList();
        });
      } else {
        addPrice(form.value).then(response => {
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
  const _priceIds = row.priceId || ids.value;
  proxy.$modal.confirm('是否确认删除价格管理编号为"' + _priceIds + '"的数据项？').then(function () {
    return delPrice(_priceIds);
  }).then(() => {
    getList();
    proxy.$modal.msgSuccess("删除成功");
  }).catch(() => { });
}

/* 点击修改使用计数时，输入框获取焦点 */
function refNumberGetFocus() {
  if (refNumberFocus.value) {
    refNum.value.focus();
  }
}


getList();
</script>
