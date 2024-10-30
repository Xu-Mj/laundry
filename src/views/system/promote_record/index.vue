<template>
  <div class="app-container">
    <el-form :model="queryParams" ref="queryRef" :inline="true" v-show="showSearch" label-width="68px">
      <el-form-item label="模板ID" prop="tempId">
        <el-input v-model="queryParams.tempId" placeholder="请输入模板ID" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="推广时间" prop="promoteTime">
        <el-date-picker clearable v-model="queryParams.promoteTime" type="date" value-format="YYYY-MM-DD"
          placeholder="请选择推广时间">
        </el-date-picker>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" icon="Search" @click="handleQuery">搜索</el-button>
        <el-button icon="Refresh" @click="resetQuery">重置</el-button>
      </el-form-item>
    </el-form>

    <el-row :gutter="10" class="mb8">
      <!-- <el-col :span="1.5">
        <el-button type="primary" plain icon="Plus" @click="handleAdd" v-hasPermi="['system:record:add']">新增</el-button>
      </el-col> -->
      <el-col :span="1.5">
        <el-button type="success" plain icon="Edit" :disabled="single" @click="handleUpdate"
          v-hasPermi="['system:record:edit']">修改</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete"
          v-hasPermi="['system:record:remove']">删除</el-button>
      </el-col>
      <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
    </el-row>

    <el-table v-loading="loading" :data="recordList" @selection-change="handleSelectionChange">
      <el-table-column type="selection" width="55" align="center" />
      <el-table-column label="推广类型" align="center" prop="template.promoteType">
        <template #default="scope">
          <dict-tag :options="sys_promote_type" :value="scope.row.template.promoteType" />
        </template>
      </el-table-column>
      <el-table-column label="推广内容" align="center" prop="template.content" />
      <el-table-column label="推广统计" align="center">
        <template #default="scope">
          <dict-tag :options="sys_promote_result" :value="scope.row.status" />
        </template>
      </el-table-column>
      <el-table-column label="推广对象" align="center" prop="promoteObjects">
        <template #default="scope">
          <el-button link type="primary" @click="handleShowPromoteObjects(scope.row)">推广对象</el-button>
        </template>
      </el-table-column>
      <el-table-column label="推广时间" align="center" prop="promoteTime" width="180">
        <template #default="scope">
          <span>{{ parseTime(scope.row.promoteTime) }}</span>
        </template>
      </el-table-column>
      <el-table-column label="状态" align="center" prop="status">
        <template #default="scope">
          <dict-tag :options="sys_promote_result" :value="scope.row.status" />
        </template>
      </el-table-column>

      <el-table-column label="推广图片" align="center" prop="template.promotePicture">
        <template #default="scope">
          <span v-if="!scope.row.template.promotePicture || scope.row.template.promotePicture === ''">未设置</span>
          <el-image v-else :src="pictureUrl + scope.row.template.promotePicture"
            :preview-src-list="[pictureUrl + scope.row.template.promotePicture]" style="width: 100px; height: auto" />
        </template>
      </el-table-column>
      <el-table-column label="备注" align="center" prop="template.remark" />
      <el-table-column label="操作" align="center" class-name="small-padding fixed-width">
        <template #default="scope">
          <el-button link type="primary" icon="Edit" @click="handleUpdate(scope.row)"
            v-hasPermi="['system:record:edit']">修改</el-button>
          <el-button link type="primary" icon="Delete" @click="handleDelete(scope.row)"
            v-hasPermi="['system:record:remove']">删除</el-button>
        </template>
      </el-table-column>
    </el-table>

    <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
      v-model:limit="queryParams.pageSize" @pagination="getList" />
  </div>
</template>

<script setup name="Record">
import { listRecord, getRecord, delRecord, addRecord, updateRecord } from "@/api/system/promote_record";
import { getToken } from "@/utils/auth";

const { proxy } = getCurrentInstance();
const { sys_promote_type, sys_promote_result } = proxy.useDict("sys_promote_type", "sys_promote_result");

const recordList = ref([]);
const open = ref(false);
const loading = ref(true);
const showSearch = ref(true);
const ids = ref([]);
const single = ref(true);
const multiple = ref(true);
const showPromoteObjects = ref(true);
const total = ref(0);
const title = ref("");
const pictureList = ref([]);

const baseUrl = import.meta.env.VITE_APP_BASE_API;
const pictureUrl = ref(baseUrl + "/system/cloths/download/");

const data = reactive({
  form: {},
  queryParams: {
    pageNum: 1,
    pageSize: 10,
    tempId: null,
    promoteObjects: null,
    promoteTime: null,
    status: null,
  },
  rules: {
    promoteObjects: [
      { required: true, message: "推广对象不能为空", trigger: "blur" }
    ],
  }
});

const { queryParams, form, rules } = toRefs(data);

function handleShowPromoteObjects(row) {
  // get user list
  showPromoteObjects.value = true;
}

/** 查询推广记录列表 */
function getList() {
  loading.value = true;
  listRecord(queryParams.value).then(response => {
    recordList.value = response.rows;
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
    id: null,
    tempId: null,
    promoteObjects: null,
    promoteTime: null,
    status: null,
    remark: null
  };
  proxy.resetForm("recordRef");
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
  ids.value = selection.map(item => item.id);
  single.value = selection.length != 1;
  multiple.value = !selection.length;
}

/** 新增按钮操作 */
function handleAdd() {
  reset();
  open.value = true;
  title.value = "添加推广记录";
}

/** 修改按钮操作 */
function handleUpdate(row) {
  reset();
  const _id = row.id || ids.value
  getRecord(_id).then(response => {
    form.value = response.data;
    open.value = true;
    title.value = "修改推广记录";
  });
}

/** 提交按钮 */
function submitForm() {
  proxy.$refs["recordRef"].validate(valid => {
    if (valid) {
      if (form.value.id != null) {
        updateRecord(form.value).then(response => {
          proxy.$modal.msgSuccess("修改成功");
          open.value = false;
          getList();
        });
      } else {
        addRecord(form.value).then(response => {
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
  const _ids = row.id || ids.value;
  proxy.$modal.confirm('是否确认删除推广记录编号为"' + _ids + '"的数据项？').then(function () {
    return delRecord(_ids);
  }).then(() => {
    getList();
    proxy.$modal.msgSuccess("删除成功");
  }).catch(() => { });
}

/** 导出按钮操作 */
function handleExport() {
  proxy.download('system/record/export', {
    ...queryParams.value
  }, `record_${new Date().getTime()}.xlsx`)
}

getList();
</script>
