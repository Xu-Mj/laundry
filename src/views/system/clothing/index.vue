<template>
  <div class="app-container">
    <el-form :model="queryParams" ref="queryRef" :inline="true" v-show="showSearch" label-width="68px">
      <el-form-item label="衣物类别" prop="clothingCategory">
        <el-select v-model="queryParams.clothingCategory" @change="selectChange" placeholder="衣物类别" clearable
          style="width: 240px">
          <el-option v-for="dict in sys_cloth_cate" :key="dict.value" :label="dict.label" :value="dict.value" />
        </el-select>
      </el-form-item>
      <el-form-item label="衣物编码" prop="clothingNumber">
        <el-input v-model="queryParams.clothingNumber" placeholder="请输入衣物编码" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="衣物名称" prop="clothingName">
        <el-input v-model="queryParams.clothingName" placeholder="请输入衣物名称，如：羽绒服、运动鞋、貂等" clearable
          @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" icon="Search" @click="handleQuery">搜索</el-button>
        <el-button icon="Refresh" @click="resetQuery">重置</el-button>
      </el-form-item>
    </el-form>

    <el-row :gutter="10" class="mb8">
      <el-col :span="1.5">
        <el-button type="primary" plain icon="Plus" @click="handleAdd"
          v-hasPermi="['system:clothing:add']">新增</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete"
          v-hasPermi="['system:clothing:remove']">批量删除</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button type="success" plain icon="Edit" :disabled="ids.length == 0" @click="showUpdateRefNum = true"
          v-hasPermi="['system:clothing:edit']">设置使用计数</el-button>
      </el-col>
      <!-- <el-col :span="1.5">
        <el-button type="warning" plain icon="Download" @click="handleExport"
          v-hasPermi="['system:clothing:export']">导出</el-button>
      </el-col> -->
      <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
    </el-row>

    <el-table v-loading="loading" :data="clothingList" @selection-change="handleSelectionChange">
      <el-table-column type="selection" width="55" align="center" />
      <!-- <el-table-column label="衣物唯一标识ID" align="center" prop="clothingId" /> -->
      <el-table-column label="衣物名称" align="center" prop="clothingName" />
      <el-table-column label="衣物编码" align="center" prop="clothingNumber" />
      <el-table-column label="所属品类" align="center" prop="clothingCategory">
        <template #default="scope">
          <dict-tag :options="sys_cloth_cate" :value="scope.row.clothingCategory" />
        </template>
      </el-table-column>
      <el-table-column label="所属分类" align="center" prop="clothingStyle">
        <template #default="scope">
          <!-- <dict-tag :options="sys_cloth_style" :value="scope.row.clothingStyle" /> -->
          <el-tag :type="getStyleCss(scope.row)">
            {{ getStyle(scope.row) }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column label="基准价格" align="center" prop="clothingBasePrice" />
      <el-table-column label="最低价格" align="center" prop="clothingMinPrice" />
      <el-table-column label="显示顺序" align="center" prop="orderNum" />
      <el-table-column label="使用计数" align="center" prop="clothingDegree" />
      <el-table-column label="备注" align="center" prop="remark" show-overflow-tooltip />
      <el-table-column label="操作" align="center" class-name="small-padding fixed-width">
        <template #default="scope">
          <el-button link type="primary" icon="Edit" @click="handleUpdate(scope.row)"
            v-hasPermi="['system:clothing:edit']">修改</el-button>
          <el-button link type="primary" icon="Delete" @click="handleDelete(scope.row)"
            v-hasPermi="['system:clothing:remove']">删除</el-button>
        </template>
      </el-table-column>
    </el-table>

    <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
      v-model:limit="queryParams.pageSize" @pagination="getList" />

    <!-- 添加或修改衣物管理对话框 -->
    <el-dialog :show-close="false" v-model="open" width="500px" append-to-body>
      <el-form ref="clothingRef" :model="form" :rules="rules" label-width="80px">
        <el-form-item label="衣物名称" prop="clothingName">
          <el-input v-model="form.clothingName" placeholder="请输入衣物名称，如：羽绒服、运动鞋、貂等" />
        </el-form-item>
        <el-row>
          <el-col :span="12">
            <el-form-item label="衣物品类" prop="clothingCategory">
              <el-select v-model="form.clothingCategory" placeholder="衣物品类" @change="cateChange" clearable
                style="width: 240px">
                <el-option v-for="dict in sys_cloth_cate" :key="dict.value" :label="dict.label" :value="dict.value" />
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="所属分类" prop="clothingStyle">
              <el-select v-model="form.clothingStyle" placeholder="所属分类" no-data-text="请先选择所属品类" clearable
                style="width: 240px">
                <el-option v-for="dict in clothStyleList" :key="dict.dictValue" :label="dict.dictLabel"
                  :value="dict.dictValue" />
              </el-select>
            </el-form-item>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="基准价格" prop="clothingBasePrice">
              <el-input-number v-model="form.clothingBasePrice" controls-position="right" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="最低价格" prop="clothingMinPrice">
              <el-input-number v-model="form.clothingMinPrice" controls-position="right" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="显示顺序" prop="orderNum">
              <el-input-number v-model="form.orderNum" controls-position="right" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="使用计数" prop="clothingDegree">
              <el-input-number v-model="form.clothingDegree" controls-position="right" />
            </el-form-item>
          </el-col>
        </el-row>
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

    <!-- 修改使用次数对话框 -->
    <el-dialog title="修改使用次数" v-model="showUpdateRefNum" width="500px" :show-close="false" append-to-body>
      <el-form ref="tagNumRef" :model="tagNumForm" :rules="refNumFormRules" label-width="80px">
        <el-form-item label="使用次数" prop="refNumber">
          <el-input-number :min="0" required v-model="tagNumForm.refNumber" placeholder="请输入使用次数" />
        </el-form-item>
      </el-form>
      <template #footer>
        <div class="dialog-footer">
          <el-button type="primary" @click="updateRefNum">确 定</el-button>
          <el-button @click="cancelUpdateRefNum">取 消</el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup name="Clothing">
import { listClothing, getClothing, delClothing, addClothing, updateClothing, updateClothingRefNum } from "@/api/system/clothing";
import { getDicts } from '@/api/system/dict/data'
import useDictStore from '@/store/modules/dict'
import { onMounted } from "vue";

const { proxy } = getCurrentInstance();

const { sys_cloth_cate } = proxy.useDict("sys_cloth_cate");

// 动态查询字典列表
const clothStyleList = ref([]);
const dictList = ref([]);
const clothingList = ref([]);
const open = ref(false);
const loading = ref(true);
const showUpdateRefNum = ref(false);
const showSearch = ref(true);
const ids = ref([]);
const single = ref(true);
const multiple = ref(true);
const total = ref(0);
const title = ref("");

const data = reactive({
  form: {},
  tagNumForm: {},
  queryParams: {
    pageNum: 1,
    pageSize: 10,
    clothingCategory: null,
    clothingNumber: null,
    clothingName: null,
  },
  rules: {
    clothingCategory: [
      { required: true, message: "服务品类不能为空", trigger: "blur" }
    ],
    clothingNumber: [
      { required: true, message: "衣物编码不能为空", trigger: "blur" }
    ],
    clothingStyle: [
      { required: true, message: "所属分类", trigger: "blur" }
    ],
    clothingName: [
      { required: true, message: "衣物名称不能为空", trigger: "blur" }
    ],
    clothingBasePrice: [
      { required: true, message: "基准价格不能为空", trigger: "blur" }
    ],
    clothingMinPrice: [
      { required: true, message: "最低价格不能为空", trigger: "blur" },
      { validator: validateMinPrice, trigger: 'blur' }
    ],
  },
  refNumFormRules: {
    refNumber: [{ required: true, message: "使用次数不能为空", trigger: "blur" }],
  }
});

const { queryParams, form, tagNumForm, rules, refNumFormRules } = toRefs(data);

// 自定义校验最低价格函数
function validateMinPrice(rule, value, callback) {
  if (value && Number(value) > Number(form.value.clothingBasePrice)) {
    callback(new Error("最低价格不能超过基准价格"));
  } else {
    callback();
  }
};

function getStyleCss(row) {
  const result = dictList.value.filter(item => item.dictType == 'sys_cloth_style' + row.clothingCategory).find(item => item.dictValue ===
    row.clothingStyle);
  return result ? result.listClass : 'default';
}
// 查找分类
function getStyle(row) {
  const result = dictList.value.filter(item => item.dictType == 'sys_cloth_style' + row.clothingCategory)
    .find(item => item.dictValue === row.clothingStyle);
  return result ? result.dictLabel : row.clothingStyle;
}

// 初始化所需要的字典数据
async function initDictList() {
  if (sys_cloth_cate.value.length === 0) {
    await getDicts("sys_cloth_cate").then(resp => {
      sys_cloth_cate.value = resp.map(p => ({ label: p.dictLabel, value: p.dictValue, elTagType: p.listClass, elTagClass: p.cssClass }))
      useDictStore().setDict("sys_cloth_cate", sys_cloth_cate.value);
    })
  }
  sys_cloth_cate.value.forEach(item => {
    getDicts("sys_cloth_style" + item.value).then(res => {
      if (res && res.length > 0) {
        dictList.value.push(...res);
      }
    })
  })
}

// 当品类发生变化时动态查询子分类列表
function cateChange(value) {
  console.log(value);
  form.value.clothingStyle = null;
  clothStyleList.value = dictList.value.filter(p => p.dictType === 'sys_cloth_style' + value);
}

function updateRefNum() {
  proxy.$refs["tagNumRef"].validate(valid => {
    if (valid) {
      updateClothingRefNum({ clothingIds: ids.value, refNum: tagNumForm.value.refNumber }).then(res => {
        proxy.$modal.msgSuccess("修改成功");
        showUpdateRefNum.value = false;
        tagNumForm.value.refNumber = null;
        getList();
      })
    }
  })
}

// 取消按钮
function cancelUpdateRefNum() {
  showUpdateRefNum.value = false;
  tagNumForm.value = {};
}

/** 查询衣物管理列表 */
function getList() {
  loading.value = true;
  listClothing(queryParams.value).then(response => {
    clothingList.value = response.rows;
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
    clothingId: null,
    clothingCategory: null,
    clothingNumber: null,
    clothingStyle: null,
    clothingName: null,
    clothingBasePrice: null,
    clothingMinPrice: null,
    orderNum: 0,
    clothingDegree: 0,
    remark: null
  };
  proxy.resetForm("clothingRef");
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
  ids.value = selection.map(item => item.clothingId);
  single.value = selection.length != 1;
  multiple.value = !selection.length;
}

/** 新增按钮操作 */
function handleAdd() {
  reset();
  open.value = true;
  title.value = "添加衣物管理";
}

/** 修改按钮操作 */
function handleUpdate(row) {
  reset();
  const _clothingId = row.clothingId || ids.value
  getClothing(_clothingId).then(response => {
    form.value = response;
    clothStyleList.value = dictList.value.filter(p => p.dictType === 'sys_cloth_style' + form.value.clothingCategory);
    open.value = true;
    title.value = "修改衣物管理";
  });
}

/** 提交按钮 */
function submitForm() {
  proxy.$refs["clothingRef"].validate(valid => {
    if (valid) {
      if (form.value.clothingId != null) {
        updateClothing(form.value).then(response => {
          proxy.$modal.msgSuccess("修改成功");
          open.value = false;
          getList();
        });
      } else {
        addClothing(form.value).then(response => {
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
  const _clothingIds = row.clothingId || ids.value;
  proxy.$modal.confirm('是否确认删除衣物管理编号为"' + _clothingIds + '"的数据项？').then(function () {
    return delClothing(_clothingIds);
  }).then(() => {
    getList();
    proxy.$modal.msgSuccess("删除成功");
  }).catch(() => { });
}

/* 衣物类别变化触发查询 */
function selectChange() {
  queryParams.value.pageNum = 1;
  getList();
}

onMounted(async () => {
  await initDictList()
  getList();
});
</script>
