<template>
   <div class="app-container">
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
      <el-dialog v-model="open" width="500px" :show-close="false" append-to-body>
         <el-form ref="postRef" :model="form" :rules="rules" label-width="80px">
            <el-form-item label="等级名称" prop="levelName">
               <el-input v-model="form.levelName" placeholder="请输入等级名称" />
            </el-form-item>
            <el-form-item label="等级编码" prop="levelCode">
               <el-input v-model="form.levelCode" placeholder="请输入编码名称" />
            </el-form-item>
            <el-form-item label="等级顺序" prop="levelSort">
               <el-input-number v-model="form.levelSort" controls-position="right" :min="0" />
            </el-form-item>
            <el-form-item label="等级状态" prop="status">
               <el-radio-group v-model="form.status">
                  <el-radio v-for="dict in sys_normal_disable" :key="dict.value" :value="dict.value">{{ dict.label
                     }}</el-radio>
               </el-radio-group>
            </el-form-item>
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

<script setup name="Post">
import { listPost, addPost, delPost, getPost, updatePost } from "@/api/system/post";

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
   proxy.$modal.confirm('是否确认删除等级编号为"' + levelIds + '"的数据项？').then(function () {
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
