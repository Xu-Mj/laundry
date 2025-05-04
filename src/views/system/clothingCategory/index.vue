<template>
  <div class="app-container">
    <transition name="height-fade">
      <el-card class="search-card" v-show="showSearch">
        <el-form :model="queryParams" ref="queryRef" :inline="true" label-width="68px">
          <el-form-item label="品类名称" prop="categoryName">
            <el-input v-model="queryParams.categoryName" placeholder="请输入品类名称" clearable style="width: 240px"
              @keyup.enter="handleQuery" />
          </el-form-item>
          <el-form-item label="品类编码" prop="categoryCode">
            <el-input v-model="queryParams.categoryCode" placeholder="请输入品类编码" clearable style="width: 240px"
              @keyup.enter="handleQuery" />
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

      <el-table v-loading="loading" :data="categoryList" @selection-change="handleSelectionChange" class="modern-table"
        border stripe>
        <template #empty>
          <el-empty description="暂无数据" />
        </template>
        <el-table-column type="selection" width="55" align="center" />
        <el-table-column label="品类名称" align="center" prop="categoryName" />
        <el-table-column label="品类编码" align="center" prop="categoryCode" />
        <el-table-column label="显示顺序" align="center" prop="orderNum" />
        <el-table-column label="备注" align="center" prop="remark" show-overflow-tooltip />
        <el-table-column label="创建时间" align="center" prop="createdAt" width="180">
          <template #default="scope">
            <span>{{ formatTime(scope.row.createdAt) }}</span>
          </template>
        </el-table-column>
        <el-table-column label="操作" align="center" class-name="small-padding fixed-width">
          <template #default="scope">
            <el-button link type="primary" icon="Edit" @click="handleUpdate(scope.row)">修改</el-button>
            <el-button link type="primary" icon="Delete" @click="handleDelete(scope.row)">删除</el-button>
            <router-link :to="'/system/clothingStyle?categoryId=' + scope.row.categoryId" class="link-type">
              <el-button link type="primary" icon="View">查看分类</el-button>
            </router-link>
          </template>
        </el-table-column>
      </el-table>

      <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
        v-model:limit="queryParams.pageSize" @pagination="getList" />
    </el-card>

    <!-- 添加或修改品类对话框 -->
    <el-dialog :title="title" v-model="open" width="500px" append-to-body :align-center="true"
      :close-on-click-modal="false" :show-close="false">
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
      <el-form ref="categoryRef" :model="form" :rules="rules" label-width="100px">
        <div class="form-card hover-flow">
          <div class="card-header">
            <el-icon>
              <Document />
            </el-icon>
            <span>基本信息</span>
          </div>
          <div class="card-body">
            <el-form-item label="品类名称" prop="categoryName">
              <el-input v-model="form.categoryName" placeholder="请输入品类名称" />
            </el-form-item>
            <!-- <el-form-item label="品类编码" prop="categoryCode">
              <el-input v-model="form.categoryCode" placeholder="请输入品类编码" />
            </el-form-item> -->
            <el-form-item label="显示顺序" prop="orderNum">
              <el-input-number v-model="form.orderNum" :min="0" />
            </el-form-item>
            <el-form-item label="备注" prop="remark">
              <el-input v-model="form.remark" type="textarea" placeholder="请输入内容" />
            </el-form-item>
          </div>
        </div>
      </el-form>
      <template #footer>
        <div class="dialog-footer">
          <el-button class="hover-flow" type="primary" @click="submitForm">确 定</el-button>
          <el-button class="hover-flow" type="danger" @click="cancel">取 消</el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { listCategory, getCategory, delCategory, addCategory, updateCategory } from "@/api/system/clothingCategory";
import { ElMessage, ElMessageBox } from 'element-plus';
import { ref, reactive, toRefs, onMounted } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();

const data = reactive({
  // 遮罩层
  loading: false,
  // 选中数组
  ids: [],
  // 非单个禁用
  single: true,
  // 非多个禁用
  multiple: true,
  // 显示搜索条件
  showSearch: true,
  // 总条数
  total: 0,
  // 品类表格数据
  categoryList: [],
  // 弹出层标题
  title: "",
  // 是否显示弹出层
  open: false,
  // 查询参数
  queryParams: {
    pageNum: 1,
    pageSize: 10,
    categoryName: undefined,
    categoryCode: undefined,
  },
  // 表单参数
  form: {},
  // 表单校验
  rules: {
    categoryName: [
      { required: true, message: "品类名称不能为空", trigger: "blur" }
    ],
    categoryCode: [
      { required: true, message: "品类编码不能为空", trigger: "blur" }
    ],
    orderNum: [
      { required: true, message: "显示顺序不能为空", trigger: "blur" }
    ]
  }
});

const { loading, ids, single, multiple, showSearch, total, categoryList, title, open, queryParams, form, rules } = toRefs(data);
const queryRef = ref();
const categoryRef = ref();

/** 查询品类列表 */
function getList() {
  data.loading = true;
  listCategory(data.queryParams).then(response => {
    data.categoryList = response.rows;
    data.total = response.total;
    data.loading = false;
  });
}

/** 取消按钮 */
function cancel() {
  data.open = false;
  reset();
}

/** 表单重置 */
function reset() {
  data.form = {
    categoryId: undefined,
    categoryName: undefined,
    categoryCode: undefined,
    orderNum: 0,
    remark: undefined,
    storeId: 0
  };
  categoryRef.value?.resetFields();
}

/** 搜索按钮操作 */
function handleQuery() {
  data.queryParams.pageNum = 1;
  getList();
}

/** 重置按钮操作 */
function resetQuery() {
  queryRef.value?.resetFields();
  handleQuery();
}

/** 多选框选中数据 */
function handleSelectionChange(selection) {
  data.ids = selection.map(item => item.categoryId);
  data.single = selection.length !== 1;
  data.multiple = !selection.length;
}

/** 新增按钮操作 */
function handleAdd() {
  reset();
  data.open = true;
  data.title = "添加品类";
}

/** 修改按钮操作 */
function handleUpdate(row) {
  reset();
  const id = row.categoryId || data.ids[0];
  getCategory(id).then(response => {
    data.form = response;
    data.open = true;
    data.title = "修改品类";
  });
}

/** 提交按钮 */
function submitForm() {
  categoryRef.value?.validate(valid => {
    if (valid) {
      if (data.form.categoryId != undefined) {
        updateCategory(data.form).then(response => {
          if (response) {
            ElMessage.success("修改成功");
            data.open = false;
            getList();
          }
        });
      } else {
        addCategory(data.form).then(response => {
          if (response) {
            ElMessage.success("新增成功");
            data.open = false;
            getList();
          }
        });
      }
    }
  });
}

/** 删除按钮操作 */
function handleDelete(row) {
  const ids = row.categoryId || data.ids;
  ElMessageBox.confirm('是否确认删除品类编号为"' + ids + '"的数据项?', "警告", {
    confirmButtonText: "确定",
    cancelButtonText: "取消",
    type: "warning"
  }).then(function () {
    return delCategory(ids);
  }).then(() => {
    getList();
    ElMessage.success("删除成功");
  }).catch(() => { });
}

onMounted(() => {
  getList();
});
</script>

<style scoped>
.app-container {
  padding: 20px;
}

.search-card,
.table-card {
  margin-bottom: 20px;
  border-radius: 8px;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
}

.mb8 {
  margin-bottom: 8px;
}
</style>