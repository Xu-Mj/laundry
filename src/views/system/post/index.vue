<template>
   <div class="app-container">
      <transition name="height-fade">
         <el-card class="search-card" v-show="showSearch">
            <el-form :model="queryParams" ref="queryRef" :inline="true">
               <el-form-item label="等级编码" prop="levelCode">
                  <el-input v-model="queryParams.levelCode" placeholder="请输入等级编码" clearable style="width: 200px"
                     @keyup.enter="handleQuery" />
               </el-form-item>
               <el-form-item label="等级名称" prop="levelName">
                  <el-input v-model="queryParams.levelName" placeholder="请输入等级名称" clearable style="width: 200px"
                     @keyup.enter="handleQuery" />
               </el-form-item>
               <el-form-item label="状态" prop="status">
                  <el-select v-model="queryParams.status" placeholder="等级状态" clearable style="width: 200px">
                     <el-option v-for="dict in sys_normal_disable" :key="dict.value" :label="dict.label"
                        :value="dict.value" />
                  </el-select>
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
               <el-button type="success" plain icon="Edit" :disabled="single" @click="handleUpdate">修改</el-button>
            </el-col>
            <el-col :span="1.5">
               <el-button type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete">删除</el-button>
            </el-col>
            <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
         </el-row>

         <el-table v-loading="loading" :data="postList" @selection-change="handleSelectionChange" class="modern-table"
            border stripe>
            <el-table-column type="selection" width="55" align="center" />
            <el-table-column label="等级编码" align="center" prop="levelCode" />
            <el-table-column label="等级名称" align="center" prop="levelName" />
            <el-table-column label="等级排序" align="center" prop="levelSort" />
            <el-table-column label="状态" align="center" prop="status">
               <template #default="scope">
                  <dict-tag :options="sys_normal_disable" :value="scope.row.status" />
               </template>
            </el-table-column>
            <el-table-column label="创建时间" align="center" prop="createTime" width="180">
               <template #default="scope">
                  <span>{{ parseTime(scope.row.createTime) }}</span>
               </template>
            </el-table-column>
            <el-table-column label="备注" align="center" prop="remark" />
            <el-table-column label="操作" width="180" align="center" class-name="small-padding fixed-width">
               <template #default="scope">
                  <el-button link type="primary" icon="Edit" @click="handleUpdate(scope.row)">修改</el-button>
                  <el-button link type="primary" icon="Delete" @click="handleDelete(scope.row)">删除</el-button>
               </template>
            </el-table-column>
         </el-table>

         <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
            v-model:limit="queryParams.pageSize" @pagination="getList" />
      </el-card>

      <!-- 添加或修改等级对话框 -->
      <el-dialog v-model="open" width="500px" :title="title || (form.levelId ? '修改等级' : '新增等级')" append-to-body align-center class="level-dialog" destroy-on-close>
         <el-form ref="postRef" :model="form" :rules="rules" label-width="80px">
            <div class="dialog-content">
               <div class="section-header">
                  <el-icon class="mr-2"><InfoFilled /></el-icon>
                  <span>基本信息</span>
               </div>
               
               <div class="form-row">
                  <el-form-item label="等级名称" prop="levelName">
                     <el-input v-model="form.levelName" placeholder="请输入等级名称" />
                  </el-form-item>
                  <el-form-item label="等级编码" prop="levelCode">
                     <el-input v-model="form.levelCode" placeholder="请输入编码名称" />
                  </el-form-item>
               </div>
               
               <div class="section-header mt-4">
                  <el-icon class="mr-2"><Setting /></el-icon>
                  <span>附加设置</span>
               </div>
               
               <div class="settings-container">
                  <el-form-item label="等级顺序" prop="levelSort">
                     <el-input-number v-model="form.levelSort" controls-position="right" :min="0" class="w-full" />
                  </el-form-item>
                  
                  <el-form-item label="等级状态" prop="status">
                     <el-radio-group v-model="form.status">
                        <el-radio v-for="dict in sys_normal_disable" :key="dict.value" :label="dict.value">
                           <el-icon v-if="dict.value === '0'" class="status-icon"><Check /></el-icon>
                           <el-icon v-else class="status-icon"><Close /></el-icon>
                           {{ dict.label }}
                        </el-radio>
                     </el-radio-group>
                  </el-form-item>
               </div>
               
               <el-form-item label="备注" prop="remark">
                  <el-input v-model="form.remark" type="textarea" placeholder="请输入内容" :rows="3" />
               </el-form-item>
            </div>
         </el-form>
         <template #footer>
            <div class="dialog-footer">
               <el-button type="primary" @click="submitForm" round>确 定</el-button>
               <el-button @click="cancel" round>取 消</el-button>
            </div>
         </template>
      </el-dialog>
   </div>
</template>

<script setup name="Post">
import { listPost, addPost, delPost, getPost, updatePost } from "@/api/system/post";
import { InfoFilled, Setting, Check, Close } from '@element-plus/icons-vue';

const { proxy } = getCurrentInstance();
const { sys_normal_disable } = proxy.useDict("sys_normal_disable");

const postList = ref([]);
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
      levelCode: undefined,
      levelName: undefined,
      status: undefined
   },
   rules: {
      levelName: [{ required: true, message: "等级名称不能为空", trigger: "blur" }],
      levelCode: [{ required: true, message: "等级编码不能为空", trigger: "blur" }],
      levelSort: [{ required: true, message: "等级顺序不能为空", trigger: "blur" }],
   }
});

const { queryParams, form, rules } = toRefs(data);

/** 查询等级列表 */
function getList() {
   loading.value = true;
   listPost(queryParams.value).then(response => {
      postList.value = response.rows;
      total.value = response.total;
      loading.value = false;
   });
}

/** 取消按钮 */
function cancel() {
   open.value = false;
   reset();
}

/** 表单重置 */
function reset() {
   form.value = {
      levelId: undefined,
      levelCode: undefined,
      levelName: undefined,
      levelSort: 0,
      status: "0",
      remark: undefined
   };
   proxy.resetForm("postRef");
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

/** 多选框选中数据 */
function handleSelectionChange(selection) {
   ids.value = selection.map(item => item.levelId);
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
   const levelId = row.levelId || ids.value;
   getPost(Number(levelId)).then(response => {
      form.value = response;
      open.value = true;
      title.value = "修改等级";
   });
}

/** 提交按钮 */
function submitForm() {
   proxy.$refs["postRef"].validate(valid => {
      if (valid) {
         if (form.value.levelId != undefined) {
            updatePost(form.value).then(response => {
               proxy.notify.success("修改成功");
               open.value = false;
               getList();
            });
         } else {
            addPost(form.value).then(response => {
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
   const levelIds = row.levelId || ids.value;

   let confirmMessage;
  
  if (row.levelId) {
    // 单个删除
    confirmMessage = `是否确认删除等级"${row.levelName}"?`;
  } else {
    // 批量删除
    const priceNames = postList.value
      .filter(item => ids.value.includes(item.levelId))
      .map(item => item.levelName)
      .join("、");
    
    confirmMessage = `是否确认删除以下等级: ${priceNames}?`;
  }
  
   proxy.$modal.confirm(confirmMessage).then(function () {
      return delPost(levelIds);
   }).then(() => {
      getList();
      proxy.notify.success("删除成功");
   }).catch(() => { });
}

/** 导出按钮操作 */
function handleExport() {
   proxy.download("system/post/export", {
      ...queryParams.value
   }, `post_${new Date().getTime()}.xlsx`);
}

getList();
</script>

<style scoped>
.mb8 {
   margin-bottom: 8px;
}

.w-full {
   width: 100%;
}

.mr-2 {
   margin-right: 8px;
}

.mt-4 {
   margin-top: 16px;
}

.dialog-content {
   padding: 0 10px;
}

.section-header {
   display: flex;
   align-items: center;
   font-size: 16px;
   font-weight: 500;
   color: var(--el-text-color-primary);
   margin-bottom: 16px;
   padding-bottom: 10px;
   border-bottom: 1px solid var(--el-border-color-lighter);
}

.form-row {
   display: grid;
   grid-template-columns: 1fr;
   gap: 16px;
   margin-bottom: 16px;
}

@media screen and (min-width: 768px) {
   .form-row {
      grid-template-columns: 1fr 1fr;
   }
}

.dialog-footer {
   display: flex;
   justify-content: center;
   gap: 16px;
   padding-top: 10px;
}

.status-radio-group {
   display: flex;
   flex-wrap: wrap;
   gap: 8px;
}

.status-radio {
   margin-right: 0 !important;
   border-radius: 4px;
}

.status-icon {
   margin-right: 4px;
}

.setting-row {
   display: flex;
   flex-wrap: wrap;
   align-items: center;
   gap: 20px;
}

.setting-row .el-form-item {
   margin-bottom: 10px;
   flex: 1;
}

@media screen and (max-width: 575px) {
   .setting-row {
      flex-direction: column;
   }
   
   .setting-row .el-form-item {
      width: 100%;
   }
}

:deep(.el-form-item) {
   margin-bottom: 20px;
}

:deep(.el-form-item__label) {
   font-weight: 500;
}

:deep(.el-input__wrapper),
:deep(.el-textarea__inner),
:deep(.el-select),
:deep(.el-input-number) {
   box-shadow: 0 0 0 1px var(--el-border-color-light) inset;
   border-radius: 6px;
   transition: box-shadow 0.2s;
}

:deep(.el-input__wrapper:hover),
:deep(.el-textarea__inner:hover),
:deep(.el-select:hover),
:deep(.el-input-number:hover) {
   box-shadow: 0 0 0 1px var(--el-color-primary-light-5) inset;
}

:deep(.el-input__wrapper:focus-within),
:deep(.el-textarea__inner:focus-within),
:deep(.el-select:focus-within),
:deep(.el-input-number:focus-within) {
   box-shadow: 0 0 0 1px var(--el-color-primary) inset;
}

:deep(.el-radio-button__original-radio:checked + .el-radio-button__inner) {
   background-color: var(--el-color-primary-light-8);
   color: var(--el-color-primary);
   border-color: var(--el-color-primary-light-5);
   box-shadow: none;
}

:deep(.level-dialog .el-dialog__header) {
   padding: 20px 20px 10px;
   margin: 0;
   text-align: center;
   font-weight: 600;
   font-size: 18px;
}

:deep(.level-dialog .el-dialog__body) {
   padding: 15px 20px;
}

:deep(.level-dialog .el-dialog__footer) {
   padding: 10px 20px 20px;
   border-top: 1px solid var(--el-border-color-lighter);
}

:deep(.level-dialog .el-dialog) {
   border-radius: 8px;
   overflow: hidden;
   box-shadow: 0 12px 32px 4px rgba(0, 0, 0, 0.1);
}

.modern-table {
   --el-table-border-color: transparent;
   border-radius: 8px;
   overflow: hidden;
   box-shadow: var(--el-box-shadow-lighter);
}

.modern-table :deep(th) {
   background-color: var(--el-fill-color-light);
   color: var(--el-text-color-primary);
   font-weight: 600;
}

.settings-container {
   margin-bottom: 16px;
}

.settings-container .el-form-item {
   margin-bottom: 16px;
}

.status-icon {
   margin-right: 4px;
   vertical-align: middle;
}

.el-radio {
   margin-right: 16px;
}

.el-radio .el-radio__label {
   display: inline-flex;
   align-items: center;
}
</style>
