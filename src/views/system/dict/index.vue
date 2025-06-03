<template>
   <div class="app-container">
      <transition name="height-fade">
         <el-card class="search-card" v-show="showSearch">
            <el-form :model="queryParams" ref="queryRef" :inline="true" label-width="68px">
               <el-form-item label="字典名称" prop="dictName">
                  <el-input v-model="queryParams.dictName" placeholder="请输入字典名称" clearable style="width: 240px"
                     @keyup.enter="handleQuery" />
               </el-form-item>
               <el-form-item label="字典类型" prop="dictType">
                  <el-input v-model="queryParams.dictType" placeholder="请输入字典类型" clearable style="width: 240px"
                     @keyup.enter="handleQuery" />
               </el-form-item>
               <el-form-item label="状态" prop="status">
                  <el-select v-model="queryParams.status" placeholder="字典状态" clearable style="width: 240px">
                     <el-option v-for="dict in sys_normal_disable" :key="dict.value" :label="dict.label"
                        :value="dict.value" />
                  </el-select>
               </el-form-item>
               <el-form-item label="创建时间" style="width: 308px">
                  <el-date-picker v-model="dateRange" value-format="YYYY-MM-DD" type="daterange" range-separator="-"
                     start-placeholder="开始日期" end-placeholder="结束日期"></el-date-picker>
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

         <el-table v-loading="loading" :data="typeList" @selection-change="handleSelectionChange" class="modern-table"
            border stripe>
            <el-table-column type="selection" width="55" align="center" />
            <el-table-column label="字典名称" align="center" prop="dictName" :show-overflow-tooltip="true" />
            <el-table-column label="字典类型" align="center" :show-overflow-tooltip="true">
               <template #default="scope">
                  <router-link :to="'/system/dict-data/index/' + scope.row.dictId" class="link-type">
                     <span>{{ scope.row.dictType }}</span>
                  </router-link>
               </template>
            </el-table-column>
            <el-table-column label="状态" align="center" prop="status">
               <template #default="scope">
                  <dict-tag :options="sys_normal_disable" :value="scope.row.status" />
               </template>
            </el-table-column>
            <el-table-column label="备注" align="center" prop="remark" :show-overflow-tooltip="true" />
            <el-table-column label="创建时间" align="center" prop="createTime" width="180">
               <template #default="scope">
                  <span>{{ formatTime(scope.row.createTime) }}</span>
               </template>
            </el-table-column>
            <el-table-column label="操作" align="center" width="160" class-name="small-padding fixed-width">
               <template #default="scope">
                  <el-button link type="primary" icon="Edit" @click="handleUpdate(scope.row)">修改</el-button>
                  <el-button link type="primary" icon="Delete" @click="handleDelete(scope.row)">删除</el-button>
               </template>
            </el-table-column>
         </el-table>

         <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
            v-model:limit="queryParams.pageSize" @pagination="getList" />
      </el-card>
      <!-- 添加或修改参数配置对话框 -->
      <el-dialog v-model="open" :show-close="false" width="550px" align-center destroy-on-close>
         <template #header>
            <div class="dialog-header hover-flow">
               <span class="dialog-title">{{ title }}</span>
               <el-button circle @click="cancel">
                  <el-icon>
                     <Close />
                  </el-icon>
               </el-button>
            </div>
         </template>
         <el-form ref="dictRef" :model="form" :rules="rules" :align-center="true" label-width="80px" class="dict-form">
            <div class="form-card hover-flow">
               <div class="card-header">
                  <el-icon>
                     <Money />
                  </el-icon>
                  <span>基本信息</span>
               </div>
               <div class="card-body">
                  <el-row :gutter="20">
                     <el-col :span="24">
                        <el-form-item label="字典名称" prop="dictName">
                           <el-input v-model="form.dictName" placeholder="请输入字典名称" clearable />
                        </el-form-item>
                     </el-col>
                     <el-col :span="24">
                        <el-form-item label="字典类型" prop="dictType">
                           <el-input v-model="form.dictType" placeholder="请输入字典类型" clearable />
                        </el-form-item>
                     </el-col>
                  </el-row>
               </div>
            </div>

            <div class="form-card hover-flow">
               <div class="card-header">
                  <el-icon>
                     <Money />
                  </el-icon>
                  <span>状态设置</span>
               </div>
               <div class="card-body">
                  <el-row :gutter="20">
                     <el-col :span="24">
                        <el-form-item label="状态" prop="status">
                           <el-radio-group v-model="form.status">
                              <el-radio-button v-for="dict in sys_normal_disable" :key="dict.value" :label="dict.value">
                                 {{ dict.label }}
                              </el-radio-button>
                           </el-radio-group>
                        </el-form-item>
                     </el-col>
                     <el-col :span="24">
                        <el-form-item label="备注" prop="remark">
                           <el-input v-model="form.remark" type="textarea" :rows="3" placeholder="请输入内容" maxlength="200"
                              show-word-limit></el-input>
                        </el-form-item>
                     </el-col>
                  </el-row>
               </div>
            </div>
         </el-form>
         <template #footer>
            <div class="dialog-footer">
               <el-button class="hover-flow" type="primary" @click="submitForm" icon="Check">确 定</el-button>
               <el-button class="hover-flow" type="danger" @click="cancel" icon="Close">取 消</el-button>
            </div>
         </template>
      </el-dialog>
   </div>
</template>

<script setup name="Dict">
import { listType, getType, delType, addType, updateType } from "@/api/system/dict/type";

const { proxy } = getCurrentInstance();
const { sys_normal_disable } = proxy.useDict("sys_normal_disable");

const typeList = ref([]);
const open = ref(false);
const loading = ref(true);
const showSearch = ref(true);
const ids = ref([]);
const single = ref(true);
const multiple = ref(true);
const total = ref(0);
const title = ref("");
const dateRange = ref([]);

const data = reactive({
   form: {},
   queryParams: {
      pageNum: 1,
      pageSize: 18,
      dictName: undefined,
      dictType: undefined,
      status: undefined
   },
   rules: {
      dictName: [{ required: true, message: "字典名称不能为空", trigger: "blur" }],
      dictType: [{ required: true, message: "字典类型不能为空", trigger: "blur" }]
   },
});

const { queryParams, form, rules } = toRefs(data);

/** 查询字典类型列表 */
function getList() {
   loading.value = true;
   listType(proxy.addDateRange(queryParams.value, dateRange.value)).then(response => {
      typeList.value = response.rows;
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
      dictId: undefined,
      dictName: undefined,
      dictType: undefined,
      status: "0",
      remark: undefined
   };
   proxy.resetForm("dictRef");
}

/** 搜索按钮操作 */
function handleQuery() {
   queryParams.value.pageNum = 1;
   getList();
}

/** 重置按钮操作 */
function resetQuery() {
   dateRange.value = [];
   proxy.resetForm("queryRef");
   handleQuery();
}

/** 新增按钮操作 */
function handleAdd() {
   reset();
   title.value = "新建字典类型";
   open.value = true;
}

/** 多选框选中数据 */
function handleSelectionChange(selection) {
   ids.value = selection.map(item => item.dictId);
   single.value = selection.length != 1;
   multiple.value = !selection.length;
}

/** 修改按钮操作 */
function handleUpdate(row) {
   reset();
   const dictId = row.dictId || ids.value;
   getType(dictId).then(response => {
      form.value = response;
      open.value = true;
      title.value = "修改字典类型";
   });
}

/** 提交按钮 */
function submitForm() {
   proxy.$refs["dictRef"].validate(valid => {
      if (valid) {
         if (form.value.dictId != undefined) {
            updateType(form.value).then(response => {
               proxy.notify.success("修改成功");
               open.value = false;
               getList();
            });
         } else {
            addType(form.value).then(response => {
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
   const dictIds = row.dictId || ids.value;
   proxy.$modal.confirm('是否确认删除字典编号为"' + dictIds + '"的数据项？').then(function () {
      return delType(dictIds);
   }).then(() => {
      getList();
      proxy.notify.success("删除成功");
   }).catch(() => { });
}

getList();
</script>

<style scoped>
.dict-form-input {
   width: 100%;
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

.dict-form .el-form-item {
   margin-bottom: 20px;
}

.dict-form .el-input,
.dict-form .el-select,
.dict-form .el-radio-group {
   width: 100%;
}

.dict-form .el-textarea__inner {
   min-height: 80px;
   font-family: var(--el-font-family);
}

.dict-form .el-radio-button {
   margin-right: 10px;
}
</style>
