<template>
  <div class="app-container">
    <el-card class="search-card" v-show="showSearch">
      <el-form :model="queryParams" ref="queryRef" :inline="true" label-width="68px">
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
          <el-input v-model="queryParams.clothingName" placeholder="请输入衣物名称" clearable @keyup.enter="handleQuery" />
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
          <el-button type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete">批量删除</el-button>
        </el-col>
        <el-col :span="1.5">
          <el-button type="success" plain icon="Edit" :disabled="ids.length == 0"
            @click="showUpdateRefNum = true">设置使用计数</el-button>
        </el-col>
        <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
      </el-row>

      <el-table v-loading="loading" :data="clothingList" @selection-change="handleSelectionChange" class="modern-table"
        border stripe>
        <el-table-column type="selection" width="55" align="center" />
        <el-table-column label="衣物名称" align="center" prop="clothingName" />
        <el-table-column label="衣物编码" align="center" prop="clothingNumber" />
        <el-table-column label="所属品类" align="center" prop="clothingCategory"/>
        <el-table-column label="所属分类" align="center" prop="clothingStyle"/>
        <el-table-column label="基准价格" align="center" prop="clothingBasePrice" />
        <el-table-column label="最低价格" align="center" prop="clothingMinPrice" />
        <el-table-column label="显示顺序" align="center" prop="orderNum" />
        <el-table-column label="使用计数" align="center" prop="clothingDegree" />
        <el-table-column label="备注" align="center" prop="remark" show-overflow-tooltip />
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

    <!-- 添加或修改衣物管理对话框 -->
    <el-dialog :title="title" v-model="open" width="650px" append-to-body :align-center="true" :close-on-click-modal="false" :show-close="false" class="clothing-dialog">
      <el-steps :active="activeStep" finish-status="success" simple class="dialog-steps">
        <el-step title="基本信息" icon="Document" />
        <el-step title="价格设置" icon="Money" />
        <el-step title="其他设置" icon="Setting" />
      </el-steps>
      
      <el-form ref="clothingRef" :model="form" :rules="rules" label-width="100px" class="clothing-form">
        <!-- 步骤1：基本信息 -->
        <div v-show="activeStep === 0" class="step-content hover-flow">
          <div class="form-section">
            <div class="section-title">基本信息</div>
            <el-form-item label="衣物名称" prop="clothingName">
              <el-input v-model="form.clothingName" placeholder="请输入衣物名称，如：羽绒服、运动鞋、貂等" clearable>
                <template #prefix>
                  <el-icon><Goods /></el-icon>
                </template>
              </el-input>
            </el-form-item>
            <el-row :gutter="20">
              <el-col :span="12">
                <el-form-item label="衣物品类" prop="clothingCategory">
                  <el-select v-model="form.clothingCategory" placeholder="衣物品类" @change="cateChange" clearable
                    style="width: 100%">
                    <el-option v-for="dict in sys_cloth_cate" :key="dict.value" :label="dict.label" :value="dict.value" />
                  </el-select>
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="所属分类" prop="clothingStyle">
                  <el-select v-model="form.clothingStyle" placeholder="所属分类" no-data-text="请先选择所属品类" clearable
                    style="width: 100%">
                    <el-option v-for="dict in clothStyleList" :key="dict.dictValue" :label="dict.dictLabel"
                      :value="dict.dictValue" />
                  </el-select>
                </el-form-item>
              </el-col>
            </el-row>
          </div>
        </div>
        
        <!-- 步骤2：价格设置 -->
        <div v-show="activeStep === 1" class="step-content hover-flow">
          <div class="form-section">
            <div class="section-title">价格设置</div>
            <el-row :gutter="20">
              <el-col :span="12">
                <el-form-item label="基准价格" prop="clothingBasePrice">
                  <el-input-number v-model="form.clothingBasePrice" controls-position="right" :precision="2" :step="0.1" style="width: 100%">
                    <template #prefix>
                      <el-icon><PriceTag /></el-icon>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="最低价格" prop="clothingMinPrice">
                  <el-input-number v-model="form.clothingMinPrice" controls-position="right" :precision="2" :step="0.1" style="width: 100%">
                    <template #prefix>
                      <el-icon><Discount /></el-icon>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
            </el-row>
            <div class="price-tip" v-if="form.clothingBasePrice && form.clothingMinPrice">
              <el-alert
                :title="`价格区间: ${form.clothingMinPrice} ~ ${form.clothingBasePrice} 元`"
                type="info"
                :closable="false"
                show-icon
              />
            </div>
          </div>
        </div>
        
        <!-- 步骤3：其他设置 -->
        <div v-show="activeStep === 2" class="step-content hover-flow">
          <div class="form-section">
            <div class="section-title">其他设置</div>
            <el-row :gutter="20">
              <el-col :span="12">
                <el-form-item label="显示顺序" prop="orderNum">
                  <el-input-number v-model="form.orderNum" controls-position="right" :min="0" style="width: 100%">
                    <template #prefix>
                      <el-icon><Sort /></el-icon>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="使用计数" prop="clothingDegree">
                  <el-input-number v-model="form.clothingDegree" controls-position="right" :min="0" style="width: 100%">
                    <template #prefix>
                      <el-icon><Odometer /></el-icon>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
            </el-row>
            <el-form-item label="备注" prop="remark">
              <el-input v-model="form.remark" type="textarea" placeholder="请输入内容" :rows="3" />
            </el-form-item>
          </div>
        </div>
      </el-form>
      
      <template #footer>
        <div class="dialog-footer">
          <el-button class="hover-flow" v-if="activeStep > 0" icon="ArrowLeft" @click="prevStep">上一步</el-button>
          <el-button class="hover-flow" v-if="activeStep < 2" type="primary" icon="ArrowRight" @click="nextStep">下一步</el-button>
          <el-button class="hover-flow" v-if="activeStep === 2" type="primary" icon="Check" @click="submitForm">提交</el-button>
          <el-button class="hover-flow" icon="Close" type="danger" @click="cancel">取 消</el-button>
        </div>
      </template>
    </el-dialog>

    <!-- 修改使用次数对话框 -->
    <ref-count-editor
      v-model="showUpdateRefNum"
      :initial-value="tagNumForm.refNumber"
      title="修改使用次数"
      description="设置衣物的使用计数值,用于在选择衣物界面的排序"
      @confirm="handleRefNumConfirm"
      @cancel="cancelUpdateRefNum"
    />
  </div>
</template>

<script setup name="Clothing">
import { listClothing, getClothing, delClothing, addClothing, updateClothing, updateClothingRefNum } from "@/api/system/clothing";
import { getDicts } from '@/api/system/dict/data'
import useDictStore from '@/store/modules/dict'
import { onMounted } from "vue";
import RefCountEditor from "@/components/RefCountEditor/index.vue";

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
const activeStep = ref(0); // 当前步骤

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

// 处理引用计数确认事件
function handleRefNumConfirm(refNumber) {
  updateClothingRefNum({ clothingIds: ids.value, refNum: refNumber }).then(res => {
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
  activeStep.value = 0; // 重置步骤
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

// 下一步按钮操作
function nextStep() {
  // 根据当前步骤验证表单
  if (activeStep.value === 0) {
    // 验证基本信息
    proxy.$refs["clothingRef"].validateField(["clothingName", "clothingCategory", "clothingStyle"], valid => {
      if (!valid) return;
      activeStep.value++;
    });
  } else if (activeStep.value === 1) {
    // 验证价格信息
    proxy.$refs["clothingRef"].validateField(["clothingBasePrice", "clothingMinPrice"], valid => {
      if (!valid) return;
      activeStep.value++;
    });
  }
}

// 上一步按钮操作
function prevStep() {
  if (activeStep.value > 0) {
    activeStep.value--;
  }
}

/** 提交按钮 */
function submitForm() {
  proxy.$refs["clothingRef"].validate(valid => {
    if (valid) {
      if (form.value.clothingId != null) {
        updateClothing(form.value).then(response => {
          proxy.notify.success("修改成功");
          open.value = false;
          activeStep.value = 0; // 重置步骤
          getList();
        });
      } else {
        addClothing(form.value).then(response => {
          proxy.notify.success("新增成功");
          open.value = false;
          activeStep.value = 0; // 重置步骤
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
    proxy.notify.success("删除成功");
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

<style scoped>
.clothing-dialog :deep(.el-dialog__body) {
  padding: 20px 30px;
}

.dialog-steps {
  margin-bottom: 1rem;
  box-shadow: var(--el-box-shadow-lighter);
}

.form-section {
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 20px;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.05);
}

.step-content{
  border-radius: .4rem;
  box-shadow: var(--el-box-shadow-lighter);
}

.section-title {
  font-size: 16px;
  font-weight: bold;
  margin-bottom: 20px;
  color: #409EFF;
  border-bottom: 1px solid #ebeef5;
  padding-bottom: 10px;
}

.price-tip {
  margin-top: 15px;
}

.dialog-footer {
  display: flex;
  justify-content: center;
  gap: 10px;
}
</style>
