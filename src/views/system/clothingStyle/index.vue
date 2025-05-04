<template>
    <div class="app-container">
        <transition name="height-fade">
            <el-card class="search-card" v-show="showSearch">
                <el-form :model="queryParams" ref="queryRef" :inline="true" label-width="68px">
                    <el-form-item label="所属品类" prop="categoryId">
                        <el-select v-model="queryParams.categoryId" placeholder="请选择所属品类" clearable style="width: 240px"
                            @change="handleCategoryChange" :disabled="!!categoryName">
                            <el-option v-for="item in categoryOptions" :key="item.categoryId" :label="item.categoryName"
                                :value="item.categoryId" />
                        </el-select>
                    </el-form-item>
                    <el-form-item label="分类名称" prop="styleName">
                        <el-input v-model="queryParams.styleName" placeholder="请输入分类名称" clearable style="width: 240px"
                            @keyup.enter="handleQuery" />
                    </el-form-item>
                    <el-form-item label="分类编码" prop="styleCode">
                        <el-input v-model="queryParams.styleCode" placeholder="请输入分类编码" clearable style="width: 240px"
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
            <div v-if="categoryName" class="category-title">
                <span>当前所属品类: {{ categoryName }}</span>
            </div>
            <el-row :gutter="10" class="mb8">
                <el-col :span="1.5">
                    <el-button type="primary" plain icon="Plus" @click="handleAdd">新增</el-button>
                </el-col>
                <el-col :span="1.5">
                    <el-button type="success" plain icon="Edit" :disabled="single" @click="handleUpdate">修改</el-button>
                </el-col>
                <el-col :span="1.5">
                    <el-button type="danger" plain icon="Delete" :disabled="multiple"
                        @click="handleDelete">删除</el-button>
                </el-col>
                <el-col :span="1.5">
                    <el-button type="warning" plain icon="Back" @click="handleClose">返回</el-button>
                </el-col>
                <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
            </el-row>



            <el-table v-loading="loading" :data="styleList" @selection-change="handleSelectionChange"
                class="modern-table" border stripe>
                <template #empty>
                    <el-empty description="暂无数据" />
                </template>
                <el-table-column type="selection" width="55" align="center" />
                <el-table-column label="分类名称" align="center" prop="styleName" />
                <el-table-column label="分类编码" align="center" prop="styleCode" />
                <el-table-column label="所属品类" align="center" prop="categoryId">
                    <template #default="scope">
                        <span>{{ getCategoryName(scope.row.categoryId) }}</span>
                    </template>
                </el-table-column>
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
                    </template>
                </el-table-column>
            </el-table>

            <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
                v-model:limit="queryParams.pageSize" @pagination="getList" />
        </el-card>

        <!-- 添加或修改分类对话框 -->
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
            <el-form ref="styleRef" :model="form" :rules="rules" label-width="100px">
                <div class="form-card hover-flow">
                    <div class="card-header">
                        <el-icon>
                            <Document />
                        </el-icon>
                        <span>基本信息</span>
                    </div>
                    <div class="card-body">
                        <el-form-item label="所属品类" prop="categoryId">
                            <el-select v-model="form.categoryId" placeholder="请选择所属品类" style="width: 100%">
                                <el-option v-for="item in categoryOptions" :key="item.categoryId"
                                    :label="item.categoryName" :value="item.categoryId" />
                            </el-select>
                        </el-form-item>
                        <el-form-item label="分类名称" prop="styleName">
                            <el-input v-model="form.styleName" placeholder="请输入分类名称" />
                        </el-form-item>
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
import { listStyle, getStyle, delStyle, addStyle, updateStyle } from "@/api/system/clothingStyle";
import { listCategoryAll } from "@/api/system/clothingCategory";
import { ElMessage, ElMessageBox } from 'element-plus';
import { useRoute, useRouter } from 'vue-router';

const route = useRoute();
const router = useRouter();
const categoryName = ref('');

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
    // 分类表格数据
    styleList: [],
    // 品类选项
    categoryOptions: [],
    // 弹出层标题
    title: "",
    // 是否显示弹出层
    open: false,
    // 查询参数
    queryParams: {
        pageNum: 1,
        pageSize: 10,
        categoryId: undefined,
        styleName: undefined,
        styleCode: undefined,
    },
    // 表单参数
    form: {},
    // 表单校验
    rules: {
        categoryId: [
            { required: true, message: "所属品类不能为空", trigger: "change" }
        ],
        styleName: [
            { required: true, message: "分类名称不能为空", trigger: "blur" }
        ],
        styleCode: [
            { required: true, message: "分类编码不能为空", trigger: "blur" }
        ],
        orderNum: [
            { required: true, message: "显示顺序不能为空", trigger: "blur" }
        ]
    }
});

const { loading, ids, single, multiple, showSearch, total, styleList, categoryOptions, title, open, queryParams, form, rules } = toRefs(data);
const queryRef = ref();
const styleRef = ref();

/** 查询分类列表 */
function getList() {
    data.loading = true;
    listStyle(data.queryParams).then(response => {
        data.styleList = response.rows;
        data.total = response.total;
        data.loading = false;
    });
}

/** 查询品类列表 */
function getCategoryList() {
    listCategoryAll().then(response => {
        data.categoryOptions = response;

        // 如果从路由参数中获取了品类ID，查找对应的品类名称
        if (data.queryParams.categoryId) {
            const category = response.find(item => item.categoryId === parseInt(data.queryParams.categoryId));
            if (category) {
                categoryName.value = category.categoryName;
            }
        }
    });
}

/** 根据品类ID获取品类名称 */
function getCategoryName(categoryId) {
    const category = data.categoryOptions.find(item => item.categoryId === categoryId);
    return category ? category.categoryName : '';
}

/** 取消按钮 */
function cancel() {
    data.open = false;
    reset();
}

/** 表单重置 */
function reset() {
    data.form = {
        styleId: undefined,
        categoryId: data.queryParams.categoryId,
        styleName: undefined,
        styleCode: undefined,
        orderNum: 0,
        remark: undefined,
        storeId: 0
    };
    styleRef.value?.resetFields();
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

/** 品类选择变更 */
function handleCategoryChange() {
    handleQuery();
}

/** 多选框选中数据 */
function handleSelectionChange(selection) {
    data.ids = selection.map(item => item.styleId);
    data.single = selection.length !== 1;
    data.multiple = !selection.length;
}

/** 新增按钮操作 */
function handleAdd() {
    reset();
    data.open = true;
    data.title = "添加分类";
}

/** 修改按钮操作 */
function handleUpdate(row) {
    reset();
    const id = row.styleId || data.ids[0];
    getStyle(id).then(response => {
        data.form = response;
        data.open = true;
        data.title = "修改分类";
    });
}

/** 提交按钮 */
function submitForm() {
    styleRef.value?.validate(valid => {
        if (valid) {
            if (data.form.styleId != undefined) {
                updateStyle(data.form).then(response => {
                    if (response) {
                        ElMessage.success("修改成功");
                        data.open = false;
                        getList();
                    }
                });
            } else {
                addStyle(data.form).then(response => {
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
    const ids = row.styleId || data.ids;
    ElMessageBox.confirm('是否确认删除分类编号为"' + ids + '"的数据项?', "警告", {
        confirmButtonText: "确定",
        cancelButtonText: "取消",
        type: "warning"
    }).then(function () {
        return delStyle(ids);
    }).then(() => {
        getList();
        ElMessage.success("删除成功");
    }).catch(() => { });
}

/** 返回按钮操作 */
function handleClose() {
    router.push({ path: '/system/clothingCategory' });
}

onMounted(() => {
    // 从路由参数中获取品类ID
    if (route.query.categoryId) {
        data.queryParams.categoryId = parseInt(route.query.categoryId);
    }
    getCategoryList();
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

.category-title {
    margin-bottom: 15px;
    padding: 8px 15px;
    background-color: var(--el-color-primary-light-9);
    border-radius: 4px;
    font-size: 16px;
    font-weight: bold;
    color: var(--el-color-primary);
}

.link-type {
    color: #409EFF;
    text-decoration: none;
}
</style>