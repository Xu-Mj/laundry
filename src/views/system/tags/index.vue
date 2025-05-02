<template>
  <div class="app-container">
    <el-card class="search-card" v-show="showSearch">
      <el-form :model="queryParams" ref="queryRef" :inline="true" label-width="68px">
        <el-form-item label="标签编码" prop="tagNumber">
          <el-input v-model="queryParams.tagNumber" placeholder="请输入标签编码" clearable @keyup.enter="handleQuery" />
        </el-form-item>
        <el-form-item label="标签名称" prop="tagName">
          <el-input v-model="queryParams.tagName" placeholder="请输入标签名称" clearable @keyup.enter="handleQuery" />
        </el-form-item>
        <el-form-item label="标签类别" prop="tagOrder">
          <el-select v-model="queryParams.tagOrder" @change="selectChange" placeholder="标签类别" clearable
            style="width: 240px">
            <el-option v-for="dict in sys_tag_order" :key="dict.value" :label="dict.label" :value="dict.value" />
          </el-select>
        </el-form-item>
        <el-form-item label="状态" prop="status">
          <el-select v-model="queryParams.status" @change="selectChange" placeholder="状态" clearable
            style="width: 240px">
            <el-option v-for="dict in sys_normal_disable" :key="dict.value" :label="dict.label" :value="dict.value" />
          </el-select>
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
          <el-button type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete">删除</el-button>
        </el-col>
        <el-col :span="1.5">
          <el-button type="success" plain icon="Edit" :disabled="ids.length == 0"
            @click="() => { showUpdateRefNum = true }">修改使用计数</el-button>
        </el-col>
        <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
      </el-row>

      <el-table v-loading="loading" :data="tagsList" @selection-change="handleSelectionChange" class="modern-table"
        border stripe>
        <template #empty>
          <el-empty description="暂无数据" />
        </template>
        <el-table-column type="selection" width="55" align="center" />
        <el-table-column label="标签编码" align="center" prop="tagNumber" />
        <el-table-column label="标签类别" align="center" prop="tagOrder">
          <template #default="scope">
            <dict-tag :options="sys_tag_order" :value="scope.row.tagOrder" />
          </template>
        </el-table-column>
        <el-table-column label="标签名称" align="center" prop="tagName" />
        <el-table-column label="使用次数" align="center" prop="refNum" />
        <el-table-column label="显示顺序" align="center" prop="orderNum" />
        <el-table-column label="标签状态" align="center" width="100">
          <template #default="scope">
            <el-switch v-model="scope.row.status" active-value="0" inactive-value="1"
              @change="handleStatusChange(scope.row)"></el-switch>
          </template>
        </el-table-column>
        <el-table-column label="备注" align="center" prop="remark" show-overflow-tooltip />
        <el-table-column label="操作" align="center" class-name="small-padding fixed-width">
          <template #default="scope">
            <el-button link type="primary" icon="Edit" @click="handleUpdate(scope.row, false)">修改</el-button>
            <el-button link type="primary" icon="Delete" @click="handleDelete(scope.row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>

      <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
        v-model:limit="queryParams.pageSize" @pagination="getList" />
    </el-card>

    <!-- 添加或修改对话框 -->
    <el-dialog :title="title || (form.tagId ? '修改标签' : '新增标签')" v-model="open" width="550px" @opened="refNumberGetFocus"
      align-center @closed="refNumberFocus = false" destroy-on-close>
      <el-form ref="tagsRef" :model="form" :rules="rules" label-width="80px">
        <el-divider content-position="left">
          <el-icon>
            <InfoFilled />
          </el-icon>
          <span class="divider-title">基本信息</span>
        </el-divider>

        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="标签类别" prop="tagOrder">
              <el-select v-model="form.tagOrder" placeholder="请选择标签类别" clearable class="w-full">
                <el-option v-for="dict in sys_tag_order" :key="dict.value" :label="dict.label" :value="dict.value" />
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="标签名称" prop="tagName">
              <el-input v-model="form.tagName" placeholder="请输入标签名称" />
            </el-form-item>
          </el-col>
        </el-row>

        <el-divider content-position="left">
          <el-icon>
            <Setting />
          </el-icon>
          <span class="divider-title">附加设置</span>
        </el-divider>

        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="使用次数" prop="refNum">
              <el-input-number v-model="form.refNum" ref="refNum" :min="0" controls-position="right" class="w-full" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="显示顺序" prop="orderNum">
              <el-input-number v-model="form.orderNum" :min="0" controls-position="right" class="w-full" />
            </el-form-item>
          </el-col>
        </el-row>

        <el-form-item label="状态">
          <el-radio-group v-model="form.status">
            <el-radio-button v-for="dict in sys_normal_disable" :key="dict.value" :label="dict.value">
              <template #default>
                <el-icon v-if="dict.value === '0'">
                  <Check />
                </el-icon>
                <el-icon v-else>
                  <Close />
                </el-icon>
                {{ dict.label }}
              </template>
            </el-radio-button>
          </el-radio-group>
        </el-form-item>

        <el-form-item label="备注" prop="remark">
          <el-input v-model="form.remark" type="textarea" placeholder="请输入内容" :rows="3" />
        </el-form-item>
      </el-form>
      <template #footer>
        <div class="dialog-footer">
          <el-button type="primary" @click="submitForm" :icon="Check">确 定</el-button>
          <el-button type="danger" @click="cancel" :icon="Close">取 消</el-button>
        </div>
      </template>
    </el-dialog>

    <!-- 修改使用次数对话框 -->
    <ref-count-editor v-model="showUpdateRefNum" :initial-value="tagNumForm.refNumber" title="修改使用次数"
      description="设置标签的使用计数值" @confirm="handleRefNumConfirm" @cancel="cancelUpdateRefNum" />
  </div>
</template>

<script setup name="Tags">
import { listTags, getTags, delTags, addTags, updateTags, updateTagsRefNum, changeTagStatus } from "@/api/system/tags";
import RefCountEditor from "@/components/RefCountEditor/index.vue";
import { InfoFilled, Setting, Check, Close } from '@element-plus/icons-vue';

const { proxy } = getCurrentInstance();
const { sys_normal_disable, sys_tag_order } = proxy.useDict("sys_normal_disable", "sys_tag_order");

const tagsList = ref([]);
const open = ref(false);
const showUpdateRefNum = ref(false);
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
    tagNumber: null,
    tagOrder: null,
    tagName: null,
    status: null,
  },
  rules: {
    tagOrder: [
      { required: true, message: "标签编码不能为空", trigger: "blur" }
    ],
    tagName: [
      { required: true, message: "标签名称不能为空", trigger: "blur" }
    ]
  }
});

const { queryParams, form, tagNumForm, rules } = toRefs(data);

/** 查询标签列表 */
function getList() {
  loading.value = true;
  listTags(queryParams.value).then(response => {
    tagsList.value = response.rows;
    total.value = response.total;
    loading.value = false;
  });
}

// 取消按钮
function cancel() {
  open.value = false;
  reset();
}

// 取消按钮
function cancelUpdateRefNum() {
  showUpdateRefNum.value = false;
  tagNumForm.value = {};
}

// 表单重置
function reset() {
  form.value = {
    tagId: null,
    tagNumber: null,
    tagOrder: null,
    tagName: null,
    refNum: 0,
    orderNum: 0,
    status: "0",
    remark: null
  };
  proxy.resetForm("tagsRef");
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

/* 点击修改使用计数时，输入框获取焦点 */
function refNumberGetFocus() {
  if (refNumberFocus.value) {
    refNum.value.focus();
  }
}

// 多选框选中数据
function handleSelectionChange(selection) {
  ids.value = selection.map(item => item.tagId);
  single.value = selection.length != 1;
  multiple.value = !selection.length;
}

/** 新增按钮操作 */
function handleAdd() {
  reset();
  open.value = true;
}

/** 修改按钮操作 */
function handleUpdate(row, focus) {
  refNumberFocus.value = focus;
  reset();
  const _tagId = row.tagId || ids.value
  getTags(_tagId).then(response => {
    form.value = response;
    open.value = true;
    title.value = "修改标签";
  });
}

function handleRefNumConfirm(refNumber) {
  updateTagsRefNum({ tagIds: ids.value, refNum: refNumber }).then(res => {
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

/** 提交按钮 */
function submitForm() {
  proxy.$refs["tagsRef"].validate(valid => {
    if (valid) {
      if (form.value.tagId != null) {
        updateTags(form.value).then(response => {
          proxy.notify.success("修改成功");
          open.value = false;
          getList();
        });
      } else {
        addTags(form.value).then(response => {
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
  const _tagIds = row.tagId || ids.value;
  proxy.$modal.confirm('是否确认删除编号为"' + _tagIds + '"的数据项？').then(function () {
    return delTags(_tagIds);
  }).then(() => {
    getList();
    proxy.notify.success("删除成功");
  }).catch(() => { });
}

/** 标签状态修改 */
function handleStatusChange(row) {
  let text = row.status === "0" ? "启用" : "停用";
  proxy.$modal.confirm('确认要' + text + '"' + row.tagName + '"标签吗?').then(function () {
    return changeTagStatus(row.tagId, row.status);
  }).then(() => {
    proxy.notify.success(text + "成功");
  }).catch(function () {
    row.status = row.status === "0" ? "1" : "0";
  });
}

/* 标签类别变化触发查询 */
function selectChange() {
  queryParams.value.pageNum = 1;
  getList();
}

getList();
</script>

<style scoped>
.w-full {
  width: 100%;
}

.divider-title {
  margin-left: 8px;
  font-size: 15px;
  font-weight: 500;
}

.el-divider {
  margin: 16px 0;
}

.el-form-item {
  margin-bottom: 20px;
}

.dialog-footer {
  display: flex;
  justify-content: center;
  gap: 16px;
}

.el-radio-button {
  margin-right: 8px;
}

.el-textarea {
  width: 100%;
}

.el-dialog {
  border-radius: 8px;
  overflow: hidden;
}
</style>
