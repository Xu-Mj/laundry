<template>
  <div class="app-container">
    <!-- <el-form :model="queryParams" ref="queryRef" :inline="true" v-show="showSearch" label-width="68px">
      <el-form-item label="洗衣订单的编号" prop="orderId">
        <el-input
          v-model="queryParams.orderId"
          placeholder="请输入洗衣订单的编号"
          clearable
          @keyup.enter="handleQuery"
        />
      </el-form-item>
      <el-form-item label="订单中需要赔偿的衣物ID列表" prop="clothIds">
        <el-input
          v-model="queryParams.clothIds"
          placeholder="请输入订单中需要赔偿的衣物ID列表"
          clearable
          @keyup.enter="handleQuery"
        />
      </el-form-item>
      <el-form-item label="支出账目" prop="expTitle">
        <el-input
          v-model="queryParams.expTitle"
          placeholder="请输入支出账目"
          clearable
          @keyup.enter="handleQuery"
        />
      </el-form-item>
      <el-form-item label="收款账户ID" prop="recvAccount">
        <el-input
          v-model="queryParams.recvAccount"
          placeholder="请输入收款账户ID"
          clearable
          @keyup.enter="handleQuery"
        />
      </el-form-item>
      <el-form-item label="收款账户名称" prop="recvAccountTitle">
        <el-input
          v-model="queryParams.recvAccountTitle"
          placeholder="请输入收款账户名称"
          clearable
          @keyup.enter="handleQuery"
        />
      </el-form-item>
      <el-form-item label="支出金额" prop="expAmount">
        <el-input
          v-model="queryParams.expAmount"
          placeholder="请输入支出金额"
          clearable
          @keyup.enter="handleQuery"
        />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" icon="Search" @click="handleQuery">搜索</el-button>
        <el-button icon="Refresh" @click="resetQuery">重置</el-button>
      </el-form-item>
    </el-form> -->

    <el-row :gutter="10" class="mb8">
      <el-col :span="1.5">
        <el-button type="primary" plain icon="Plus" @click="handleAdd"
          v-hasPermi="['system:expenditure:add']">新增</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button type="success" plain icon="Edit" :disabled="single" @click="handleUpdate"
          v-hasPermi="['system:expenditure:edit']">修改</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete"
          v-hasPermi="['system:expenditure:remove']">删除</el-button>
      </el-col>
      <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
    </el-row>

    <el-table v-loading="loading" :data="expenditureList" @selection-change="handleSelectionChange">
      <el-table-column type="selection" width="55" align="center" />
      <!-- <el-table-column label="ID" align="center" prop="expId" /> -->
      <!-- <el-table-column label="订单编号" align="center" prop="orderId" /> -->
      <!-- <el-table-column label="订单中需要赔偿的衣物ID列表" align="center" prop="clothIds" /> -->
      <el-table-column label="支出账目" align="center" prop="expTitle" />
      <el-table-column label="收款账户ID" align="center" prop="recvAccount" />
      <el-table-column label="收款账户名称" align="center" prop="recvAccountTitle" />
      <el-table-column label="支出类型" align="center" prop="expType">
        <template #default="scope">
          <dict-tag :options="sys_exp_type" :value="scope.row.expType" />
        </template>
      </el-table-column>
      <el-table-column label="支出金额" align="center" prop="expAmount" />
      <el-table-column label="备注信息" align="center" prop="remark" />
      <el-table-column label="操作" align="center" class-name="small-padding fixed-width">
        <template #default="scope">
          <el-button link type="primary" icon="Edit" @click="handleUpdate(scope.row)"
            v-hasPermi="['system:expenditure:edit']">修改</el-button>
          <el-button link type="primary" icon="Delete" @click="handleDelete(scope.row)"
            v-hasPermi="['system:expenditure:remove']">删除</el-button>
        </template>
      </el-table-column>
    </el-table>

    <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
      v-model:limit="queryParams.pageSize" @pagination="getList" />

    <!-- 添加或修改支出对话框 -->
    <el-dialog :title="title" v-model="open" width="500px" append-to-body>
      <el-form ref="expenditureRef" :model="form" :rules="rules" label-width="80px">
        <el-form-item label="洗衣订单的编号" prop="orderId">
          <el-input v-model="form.orderId" placeholder="请输入洗衣订单的编号" />
        </el-form-item>
        <el-form-item label="订单中需要赔偿的衣物ID列表" prop="clothIds">
          <el-input v-model="form.clothIds" placeholder="请输入订单中需要赔偿的衣物ID列表" />
        </el-form-item>
        <el-form-item label="支出账目" prop="expTitle">
          <el-input v-model="form.expTitle" placeholder="请输入支出账目" />
        </el-form-item>
        <el-form-item label="收款账户ID" prop="recvAccount">
          <el-input v-model="form.recvAccount" placeholder="请输入收款账户ID" />
        </el-form-item>
        <el-form-item label="收款账户名称" prop="recvAccountTitle">
          <el-input v-model="form.recvAccountTitle" placeholder="请输入收款账户名称" />
        </el-form-item>
        <el-form-item label="支出金额" prop="expAmount">
          <el-input v-model="form.expAmount" placeholder="请输入支出金额" />
        </el-form-item>
        <el-form-item label="备注信息" prop="remark">
          <el-input v-model="form.remark" placeholder="请输入备注信息" />
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

<script setup name="Expenditure">
import { listExpenditure, getExpenditure, delExpenditure, addExpenditure, updateExpenditure } from "@/api/system/expenditure";

const { proxy } = getCurrentInstance();
const { sys_exp_type } = proxy.useDict("sys_exp_type");

const expenditureList = ref([]);
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
    orderId: null,
    clothIds: null,
    expTitle: null,
    recvAccount: null,
    recvAccountTitle: null,
    expType: null,
    expAmount: null,
  },
  rules: {
    expTitle: [
      { required: true, message: "支出账目不能为空", trigger: "blur" }
    ],
    expType: [
      { required: true, message: "支出类型不能为空", trigger: "change" }
    ],
    expAmount: [
      { required: true, message: "支出金额不能为空", trigger: "blur" }
    ],
  }
});

const { queryParams, form, rules } = toRefs(data);

/** 查询支出列表 */
function getList() {
  loading.value = true;
  listExpenditure(queryParams.value).then(response => {
    expenditureList.value = response.rows;
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
    expId: null,
    orderId: null,
    clothIds: null,
    expTitle: null,
    recvAccount: null,
    recvAccountTitle: null,
    expType: null,
    expAmount: null,
    createTime: null,
    remark: null
  };
  proxy.resetForm("expenditureRef");
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
  ids.value = selection.map(item => item.expId);
  single.value = selection.length != 1;
  multiple.value = !selection.length;
}

/** 新增按钮操作 */
function handleAdd() {
  reset();
  open.value = true;
  title.value = "添加支出";
}

/** 修改按钮操作 */
function handleUpdate(row) {
  reset();
  const _expId = row.expId || ids.value
  getExpenditure(_expId).then(response => {
    form.value = response.data;
    open.value = true;
    title.value = "修改支出";
  });
}

/** 提交按钮 */
function submitForm() {
  proxy.$refs["expenditureRef"].validate(valid => {
    if (valid) {
      if (form.value.expId != null) {
        updateExpenditure(form.value).then(response => {
          proxy.$modal.msgSuccess("修改成功");
          open.value = false;
          getList();
        });
      } else {
        addExpenditure(form.value).then(response => {
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
  const _expIds = row.expId || ids.value;
  proxy.$modal.confirm('是否确认删除支出编号为"' + _expIds + '"的数据项？').then(function () {
    return delExpenditure(_expIds);
  }).then(() => {
    getList();
    proxy.$modal.msgSuccess("删除成功");
  }).catch(() => { });
}

/** 导出按钮操作 */
function handleExport() {
  proxy.download('system/expenditure/export', {
    ...queryParams.value
  }, `expenditure_${new Date().getTime()}.xlsx`)
}

getList();
</script>
