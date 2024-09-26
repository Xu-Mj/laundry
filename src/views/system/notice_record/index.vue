<template>
  <div class="app-container">
    <el-form :model="queryParams" ref="queryRef" :inline="true" v-show="showSearch" label-width="110px">
      <el-form-item label="会员手机号" prop="phonenumber">
        <el-input v-model="queryParams.phonenumber" placeholder="请输入会员账号" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="订单编码" prop="orderNumber">
        <el-input v-model="queryParams.orderNumber" placeholder="请输入订单编码" clearable @keyup.enter="handleQuery" />
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
        <el-button type="danger" icon="Delete" @click="handleDelete"
          v-hasPermi="['system:record:remove']">删除3个月前的通知</el-button>

      </el-col>
      <el-col :span="1.5">
        <el-button type="danger" icon="Delete" @click="handleDeleteAll"
          v-hasPermi="['system:record:remove']">清空所有通知记录</el-button>

      </el-col>
      <el-col :span="1.5">
        <el-button type="primary" icon="Management">
          <router-link to="/tool/template" class="link-type">
            管理通知模板
          </router-link>
        </el-button>
      </el-col>
      <right-toolbar v-model:showSearch="showSearch" @queryTable="getList" :columns="columns"></right-toolbar>
    </el-row>

    <el-table v-loading="loading" :data="recordList" @selection-change="handleSelectionChange">
      <el-table-column type="selection" width="55" align="center" />
      <!-- <el-table-column label="通知唯一标识ID" align="center" prop="noticeId" /> -->
      <el-table-column label="会员姓名" align="center" prop="nickName" v-if="columns[0].visible" />
      <el-table-column label="会员账号" align="center" prop="userName" v-if="columns[1].visible" />
      <el-table-column label="手机号" align="center" prop="phonenumber" v-if="columns[2].visible" />
      <el-table-column label="订单编码" align="center" prop="orderNumber" v-if="columns[3].visible" />
      <el-table-column label="通知方式" align="center" prop="noticeMethod" v-if="columns[4].visible">
        <template #default="scope">
          <dict-tag :options="sys_notice_method" :value="scope.row.noticeMethod" />
        </template>
      </el-table-column>
      <!-- <el-table-column label="通知类型" align="center" prop="noticeType" v-if="columns[5].visible">
        <template #default="scope">
          <dict-tag :options="sys_notice_type" :value="scope.row.noticeType" />
        </template>
      </el-table-column> -->
      <el-table-column label="通知时间" align="center" prop="noticeTime" width="180" v-if="columns[6].visible" />
      <el-table-column label="通知标题" align="center" prop="title" v-if="columns[7].visible" />
      <el-table-column label="通知内容" align="center" prop="content" v-if="columns[8].visible" />
      <el-table-column label="通知结果" align="center" prop="result" v-if="columns[9].visible">
        <template #default="scope">
          <dict-tag :options="sys_notice_result" :value="scope.row.result" />
        </template>
      </el-table-column>
      <el-table-column label="备注" align="center" prop="remark" v-if="columns[10].visible" />
    </el-table>

    <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
      v-model:limit="queryParams.pageSize" @pagination="getList" />
  </div>
</template>

<script setup name="Record">
import { listRecord, getRecord, delRecord, addRecord, updateRecord } from "@/api/system/notice_record";
import { delAllRecord, delRecordsByDay } from "../../../api/system/notice_record";

const { proxy } = getCurrentInstance();
const { sys_notice_result, sys_notice_method } = proxy.useDict("sys_notice_result", "sys_notice_method");

const recordList = ref([]);
const open = ref(false);
const loading = ref(true);
const showSearch = ref(true);
const ids = ref([]);
const single = ref(true);
const multiple = ref(true);
const total = ref(0);
const title = ref("");
// 列显隐信息
const columns = ref([
  { key: 0, label: `会员姓名`, visible: true },
  { key: 1, label: `会员账号`, visible: false },
  { key: 2, label: `手机号码`, visible: false },
  { key: 3, label: `订单编码`, visible: true },
  { key: 4, label: `通知方式`, visible: true },
  { key: 5, label: `通知类型`, visible: false },
  { key: 6, label: `通知时间`, visible: true },
  { key: 7, label: `通知标题`, visible: true },
  { key: 8, label: `通知内容`, visible: true },
  { key: 9, label: `通知结果`, visible: true },
  { key: 10, label: `备注信息`, visible: true },
]);

const data = reactive({
  form: {},
  queryParams: {
    pageNum: 1,
    pageSize: 10,
    userName: null,
    orderNumber: null,
  },
  rules: {
    userId: [
      { required: true, message: "用户ID不能为空", trigger: "blur" }
    ],
    orderId: [
      { required: true, message: "订单ID不能为空", trigger: "blur" }
    ],
    noticeMethod: [
      { required: true, message: "通知方式，0短信，1小程序不能为空", trigger: "blur" }
    ],
    noticeType: [
      { required: true, message: "通知类型，0自动， 1手动不能为空", trigger: "change" }
    ],
    title: [
      { required: true, message: "通知标题不能为空", trigger: "blur" }
    ],
    content: [
      { required: true, message: "通知内容不能为空", trigger: "blur" }
    ],
  }
});

const { queryParams, form, rules } = toRefs(data);

/** 查询通知记录管理列表 */
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
    noticeId: null,
    userId: null,
    orderId: null,
    noticeMethod: null,
    noticeType: null,
    noticeTime: null,
    title: null,
    content: null,
    result: null,
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
  ids.value = selection.map(item => item.noticeId);
  single.value = selection.length != 1;
  multiple.value = !selection.length;
}

/** 新增按钮操作 */
function handleAdd() {
  reset();
  open.value = true;
  title.value = "添加通知记录管理";
}

/** 修改按钮操作 */
function handleUpdate(row) {
  reset();
  const _noticeId = row.noticeId || ids.value
  getRecord(_noticeId).then(response => {
    form.value = response.data;
    open.value = true;
    title.value = "修改通知记录管理";
  });
}

/** 提交按钮 */
function submitForm() {
  proxy.$refs["recordRef"].validate(valid => {
    if (valid) {
      if (form.value.noticeId != null) {
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
function handleDelete() {
  delRecordsByDay(30).then(res => {
    proxy.$modal.msgSuccess("删除成功");
    getList();
  })

}

/** 清空所有按钮操作 */
function handleDeleteAll() {
  delAllRecord().then(res => {
    proxy.$modal.msgSuccess("删除成功");
    getList();
  })
}

/** 导出按钮操作 */
function handleExport() {
  proxy.download('system/record/export', {
    ...queryParams.value
  }, `record_${new Date().getTime()}.xlsx`)
}

getList();
</script>

<style scoped>
.link-type {
  color: white;
}
</style>