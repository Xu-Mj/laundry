<template>
  <div class="app-container">
    <el-form :model="queryParams" ref="queryRef" :inline="true" v-show="showSearch" label-width="68px">
      <el-form-item label="衣物唯一标识ID" prop="clothingId">
        <el-input v-model="queryParams.clothingId" placeholder="请输入衣物唯一标识ID" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="衣物品类" prop="clothingCategory">
        <el-input v-model="queryParams.clothingCategory" placeholder="请输入衣物品类" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="衣物类别" prop="clothingStyle">
        <el-input v-model="queryParams.clothingStyle" placeholder="请输入衣物类别" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="衣物颜色" prop="clothingColor">
        <el-input v-model="queryParams.clothingColor" placeholder="请输入衣物颜色" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="衣物瑕疵" prop="clothingFlaw">
        <el-input v-model="queryParams.clothingFlaw" placeholder="请输入衣物瑕疵" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="洗后预估" prop="estimate">
        <el-input v-model="queryParams.estimate" placeholder="请输入洗后预估" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="衣物品牌" prop="clothingBrand">
        <el-input v-model="queryParams.clothingBrand" placeholder="请输入衣物品牌" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="服务要求" prop="serviceRequirement">
        <el-input v-model="queryParams.serviceRequirement" placeholder="请输入服务要求" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="工艺加价" prop="processMarkup">
        <el-input v-model="queryParams.processMarkup" placeholder="请输入工艺加价" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="洗护价格" prop="priceValue">
        <el-input v-model="queryParams.priceValue" placeholder="请输入洗护价格" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="上挂位置编码" prop="hangLocationCode">
        <el-input v-model="queryParams.hangLocationCode" placeholder="请输入上挂位置编码" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="上挂衣物编码" prop="hangClothCode">
        <el-input v-model="queryParams.hangClothCode" placeholder="请输入上挂衣物编码" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="上挂描述信息" prop="hangRemark">
        <el-input v-model="queryParams.hangRemark" placeholder="请输入上挂描述信息" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" icon="Search" @click="handleQuery">搜索</el-button>
        <el-button icon="Refresh" @click="resetQuery">重置</el-button>
      </el-form-item>
    </el-form>

    <el-row :gutter="10" class="mb8">
      <el-col :span="1.5">
        <el-button type="primary" plain icon="Plus" @click="handleAdd" v-hasPermi="['system:cloths:add']">新增</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button type="success" plain icon="Edit" :disabled="single" @click="handleUpdate"
          v-hasPermi="['system:cloths:edit']">修改</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete"
          v-hasPermi="['system:cloths:remove']">删除</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button type="warning" plain icon="Download" @click="handleExport"
          v-hasPermi="['system:cloths:export']">导出</el-button>
      </el-col>
      <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
    </el-row>

    <el-table v-loading="loading" :data="clothsList" @selection-change="handleSelectionChange">
      <el-table-column type="selection" width="55" align="center" />
      <el-table-column label="订单衣物唯一标识ID" align="center" prop="orderClothId" />
      <el-table-column label="衣物唯一标识ID" align="center" prop="clothingId" />
      <el-table-column label="衣物品类" align="center" prop="clothingCategory" />
      <el-table-column label="衣物类别" align="center" prop="clothingStyle" />
      <el-table-column label="衣物颜色" align="center" prop="clothingColor" />
      <el-table-column label="衣物瑕疵" align="center" prop="clothingFlaw" />
      <el-table-column label="洗后预估" align="center" prop="estimate" />
      <el-table-column label="衣物品牌" align="center" prop="clothingBrand" />
      <el-table-column label="服务类型" align="center" prop="serviceType" />
      <el-table-column label="服务要求" align="center" prop="serviceRequirement" />
      <el-table-column label="洗前图片" align="center" prop="beforePics" />
      <el-table-column label="洗后图片" align="center" prop="afterPics" />
      <el-table-column label="洗护说明" align="center" prop="notes" />
      <el-table-column label="工艺加价" align="center" prop="processMarkup" />
      <el-table-column label="洗护价格" align="center" prop="priceValue" />
      <el-table-column label="上挂位置编码" align="center" prop="hangLocationCode" />
      <el-table-column label="上挂衣物编码" align="center" prop="hangClothCode" />
      <el-table-column label="上挂描述信息" align="center" prop="hangRemark" />
      <el-table-column label="操作" align="center" class-name="small-padding fixed-width">
        <template #default="scope">
          <el-button link type="primary" icon="Edit" @click="handleUpdate(scope.row)"
            v-hasPermi="['system:cloths:edit']">修改</el-button>
          <el-button link type="primary" icon="Delete" @click="handleDelete(scope.row)"
            v-hasPermi="['system:cloths:remove']">删除</el-button>
        </template>
      </el-table-column>
    </el-table>

    <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum" v-model:limit="queryParams.pageSize"
      @pagination="getList" />

    <!-- 添加或修改订单包含的衣物清单对话框 -->
    <el-dialog :title="title" v-model="open" width="500px" append-to-body>
      <el-form ref="clothsRef" :model="form" :rules="rules" label-width="80px">
        <el-form-item label="衣物唯一标识ID" prop="clothingId">
          <el-input v-model="form.clothingId" placeholder="请输入衣物唯一标识ID" />
        </el-form-item>
        <el-form-item label="衣物品类" prop="clothingCategory">
          <el-input v-model="form.clothingCategory" placeholder="请输入衣物品类" />
        </el-form-item>
        <el-form-item label="衣物类别" prop="clothingStyle">
          <el-input v-model="form.clothingStyle" placeholder="请输入衣物类别" />
        </el-form-item>
        <el-form-item label="衣物颜色" prop="clothingColor">
          <el-input v-model="form.clothingColor" placeholder="请输入衣物颜色" />
        </el-form-item>
        <el-form-item label="衣物瑕疵" prop="clothingFlaw">
          <el-input v-model="form.clothingFlaw" placeholder="请输入衣物瑕疵" />
        </el-form-item>
        <el-form-item label="洗后预估" prop="estimate">
          <el-input v-model="form.estimate" placeholder="请输入洗后预估" />
        </el-form-item>
        <el-form-item label="衣物品牌" prop="clothingBrand">
          <el-input v-model="form.clothingBrand" placeholder="请输入衣物品牌" />
        </el-form-item>
        <el-form-item label="服务要求" prop="serviceRequirement">
          <el-input v-model="form.serviceRequirement" placeholder="请输入服务要求" />
        </el-form-item>
        <el-form-item label="洗前图片" prop="beforePics">
          <el-input v-model="form.beforePics" type="textarea" placeholder="请输入内容" />
        </el-form-item>
        <el-form-item label="洗后图片" prop="afterPics">
          <el-input v-model="form.afterPics" type="textarea" placeholder="请输入内容" />
        </el-form-item>
        <el-form-item label="洗护说明" prop="notes">
          <el-input v-model="form.notes" type="textarea" placeholder="请输入内容" />
        </el-form-item>
        <el-form-item label="工艺加价" prop="processMarkup">
          <el-input v-model="form.processMarkup" placeholder="请输入工艺加价" />
        </el-form-item>
        <el-form-item label="洗护价格" prop="priceValue">
          <el-input v-model="form.priceValue" placeholder="请输入洗护价格" />
        </el-form-item>
        <el-form-item label="上挂位置编码" prop="hangLocationCode">
          <el-input v-model="form.hangLocationCode" placeholder="请输入上挂位置编码" />
        </el-form-item>
        <el-form-item label="上挂衣物编码" prop="hangClothCode">
          <el-input v-model="form.hangClothCode" placeholder="请输入上挂衣物编码" />
        </el-form-item>
        <el-form-item label="上挂描述信息" prop="hangRemark">
          <el-input v-model="form.hangRemark" placeholder="请输入上挂描述信息" />
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

<script setup name="Cloths">
import { listCloths, getCloths, delCloths, addCloths, updateCloths } from "@/api/system/cloths";

const { proxy } = getCurrentInstance();

const clothsList = ref([]);
const open = ref(false);
const loading = ref(true);
const showSearch = ref(true);
const ids = ref([]);
const single = ref(true);
const multiple = ref(true);
const total = ref(0);
const title = ref("");

const data = reactive({
  form: {},
  queryParams: {
    pageNum: 1,
    pageSize: 10,
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
    afterPics: null,
    notes: null,
    processMarkup: null,
    priceValue: null,
    hangLocationCode: null,
    hangClothCode: null,
    hangRemark: null,
  },
  rules: {
    clothingId: [
      { required: true, message: "衣物唯一标识ID不能为空", trigger: "blur" }
    ],
    clothingCategory: [
      { required: true, message: "衣物品类不能为空", trigger: "blur" }
    ],
    clothingStyle: [
      { required: true, message: "衣物类别不能为空", trigger: "blur" }
    ],
    serviceType: [
      { required: true, message: "服务类型不能为空", trigger: "change" }
    ],
    priceValue: [
      { required: true, message: "洗护价格不能为空", trigger: "blur" }
    ],
    createTime: [
      { required: true, message: "收取时间不能为空", trigger: "blur" }
    ]
  }
});

const { queryParams, form, rules } = toRefs(data);

/** 查询订单包含的衣物清单列表 */
function getList() {
  loading.value = true;
  listCloths({orderId:queryParams.value.orderClothId}).then(response => {
    clothsList.value = response.rows;
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
    orderClothId: null,
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
    afterPics: null,
    notes: null,
    processMarkup: null,
    priceValue: null,
    hangLocationCode: null,
    hangClothCode: null,
    hangRemark: null,
    createTime: null
  };
  proxy.resetForm("clothsRef");
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
  ids.value = selection.map(item => item.orderClothId);
  single.value = selection.length != 1;
  multiple.value = !selection.length;
}

/** 新增按钮操作 */
function handleAdd() {
  reset();
  open.value = true;
  title.value = "添加订单包含的衣物清单";
}

/** 修改按钮操作 */
function handleUpdate(row) {
  reset();
  const _orderClothId = row.orderClothId || ids.value
  getCloths(_orderClothId).then(response => {
    form.value = response.data;
    open.value = true;
    title.value = "修改订单包含的衣物清单";
  });
}

/** 提交按钮 */
function submitForm() {
  proxy.$refs["clothsRef"].validate(valid => {
    if (valid) {
      if (form.value.orderClothId != null) {
        updateCloths(form.value).then(response => {
          proxy.notify.success("修改成功");
          open.value = false;
          getList();
        });
      } else {
        addCloths(form.value).then(response => {
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
  const _orderClothIds = row.orderClothId || ids.value;
  proxy.$modal.confirm('是否确认删除订单包含的衣物清单编号为"' + _orderClothIds + '"的数据项？').then(function () {
    return delCloths(_orderClothIds);
  }).then(() => {
    getList();
    proxy.notify.success("删除成功");
  }).catch(() => { });
}

/** 导出按钮操作 */
function handleExport() {
  proxy.download('system/cloths/export', {
    ...queryParams.value
  }, `cloths_${new Date().getTime()}.xlsx`)
}

getList();
</script>
