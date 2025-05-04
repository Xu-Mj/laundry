<template>
  <div class="app-container">
    <el-card class="search-card" v-show="showSearch">
      <el-form :model="queryParams" ref="queryRef" :inline="true" label-width="68px">
        <el-form-item label="衣物类别" prop="categoryId">
          <el-select v-model="queryParams.categoryId" @change="selectChange" placeholder="衣物类别" clearable
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
        <template #empty>
          <el-empty description="暂无数据" />
        </template>
        <el-table-column type="selection" width="55" align="center" />
        <el-table-column label="衣物名称" align="center" prop="title" />
        <el-table-column label="所属品类" align="center" prop="clothingCategory" />
        <el-table-column label="所属分类" align="center" prop="clothingStyle" />
        <el-table-column label="基准价格" align="center" prop="clothingBasePrice" />
        <el-table-column label="最低价格" align="center" prop="clothingMinPrice" />
        <el-table-column label="库存数量" align="center" prop="stockQuantity" />
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

      <pagination v-show="total > 0" :total="total" v-model:page="queryParams.page" v-model:limit="queryParams.pageSize"
        @pagination="getList" />
    </el-card>

    <!-- 添加或修改衣物管理对话框 -->
    <el-dialog :title="title" v-model="open" width="650px" append-to-body :align-center="true"
      :close-on-click-modal="false" :show-close="false" class="clothing-dialog">
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
            <el-form-item label="衣物名称" prop="title">
              <el-input v-model="form.title" placeholder="请输入衣物名称，如：羽绒服、运动鞋、貂等" clearable>
                <template #prefix>
                  <el-icon>
                    <Goods />
                  </el-icon>
                </template>
              </el-input>
            </el-form-item>
            <el-row :gutter="20">
              <el-col :span="12">
                <el-form-item label="衣物品类" prop="categoryId">
                  <el-select v-model="form.categoryId" placeholder="衣物品类" @change="cateChange" clearable
                    style="width: 100%">
                    <el-option v-for="dict in categoryList" :key="dict.categoryId" :label="dict.categoryName"
                      :value="dict.categoryId" />
                  </el-select>
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="所属分类" prop="styleId">
                  <el-select v-model="form.styleId" placeholder="所属分类" no-data-text="请先选择所属品类" clearable
                    style="width: 100%">
                    <el-option v-for="dict in styleList" :key="dict.styleId" :label="dict.styleName"
                      :value="dict.styleId" />
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
                  <el-input-number v-model="form.clothingBasePrice" controls-position="right" :precision="2" :step="0.1"
                    style="width: 100%">
                    <template #prefix>
                      <el-icon>
                        <PriceTag />
                      </el-icon>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="最低价格" prop="clothingMinPrice">
                  <el-input-number v-model="form.clothingMinPrice" controls-position="right" :precision="2" :step="0.1"
                    style="width: 100%">
                    <template #prefix>
                      <el-icon>
                        <Discount />
                      </el-icon>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
            </el-row>
            <div class="price-tip" v-if="form.clothingBasePrice && form.clothingMinPrice">
              <el-alert :title="`价格区间: ${form.clothingMinPrice} ~ ${form.clothingBasePrice} 元`" type="info"
                :closable="false" show-icon />
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
                      <el-icon>
                        <Sort />
                      </el-icon>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="使用计数" prop="clothingDegree">
                  <el-input-number v-model="form.clothingDegree" controls-position="right" :min="0" style="width: 100%">
                    <template #prefix>
                      <el-icon>
                        <Odometer />
                      </el-icon>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
            </el-row>
            <el-row :gutter="20">
              <el-col :span="12">
                <el-form-item label="库存数量" prop="stockQuantity">
                  <el-input-number v-model="form.stockQuantity" controls-position="right" :min="0" style="width: 100%">
                    <template #prefix>
                      <el-icon>
                        <Goods />
                      </el-icon>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="是否上架" prop="isPutOnSale">
                  <el-switch v-model="form.isPutOnSale" active-text="上架" inactive-text="下架" />
                </el-form-item>
              </el-col>
            </el-row>
            <el-row :gutter="20">
              <el-col :span="12">
                <el-form-item label="挂衣方式" prop="hangType">
                  <el-radio-group size="large" v-model="form.hangType">
                    <el-radio :value="'1'">输送线</el-radio>
                    <el-radio :value="'2'">其他</el-radio>
                  </el-radio-group>
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="标签" prop="tagList">
                  <el-select v-model="form.tagListVec" multiple filterable allow-create default-first-option
                    placeholder="请选择或创建标签" style="width: 100%">
                    <el-option v-for="tag in tagOptions" :key="tag" :label="tag" :value="tag" />
                  </el-select>
                </el-form-item>
              </el-col>
            </el-row>
            <el-form-item label="主图" prop="primaryImage">
              <el-upload :auto-upload="false" :show-file-list="false" :on-change="handlePrimaryImageChange"
                list-type="picture-card" :before-upload="beforeImageUpload">
                <img v-if="form.primaryImage" :src="form.primaryImage" class="avatar" />
                <el-icon v-else>
                  <Plus />
                </el-icon>
              </el-upload>
            </el-form-item>
            <el-form-item label="图片集合" prop="imagesVec">
              <el-upload :auto-upload="false" list-type="picture-card" :limit="5" :on-exceed="handleExceed"
                :on-change="handleImageChange" :on-remove="handleImageRemove" :before-upload="beforeImageUpload"
                :disabled="imageFiles.length >= 5">
                <el-icon v-if="imageFiles.length < 5">
                  <Plus />
                </el-icon>
                <div v-else class="el-upload__tip">
                  已达到最大上传数量(5张)
                </div>
              </el-upload>
            </el-form-item>
            <el-form-item label="备注" prop="remark">
              <el-input v-model="form.remark" type="textarea" placeholder="请输入内容" :rows="3" />
            </el-form-item>
          </div>
        </div>
      </el-form>

      <template #footer>
        <div class="dialog-footer">
          <el-button class="hover-flow" v-if="activeStep > 0" icon="ArrowLeft" @click="prevStep">上一步</el-button>
          <el-button class="hover-flow" v-if="activeStep < 2" type="primary" icon="ArrowRight"
            @click="nextStep">下一步</el-button>
          <el-button class="hover-flow" v-if="activeStep === 2" type="primary" icon="Check"
            @click="submitForm">提交</el-button>
          <el-button class="hover-flow" icon="Close" type="danger" @click="cancel">取 消</el-button>
        </div>
      </template>
    </el-dialog>

    <!-- 修改使用次数对话框 -->
    <ref-count-editor v-model="showUpdateRefNum" :initial-value="tagNumForm.refNumber" title="修改使用次数"
      description="设置衣物的使用计数值,用于在选择衣物界面的排序" @confirm="handleRefNumConfirm" @cancel="cancelUpdateRefNum" />
  </div>
</template>

<script setup name="Clothing">
import { listClothing, getClothing, delClothing, addClothing, updateClothing, updateClothingRefNum } from "@/api/system/clothing";
import { listCategoryAll } from "@/api/system/clothingCategory";
import { listStyleByCategoryId } from "@/api/system/clothingStyle";
import { onMounted } from "vue";
import RefCountEditor from "@/components/RefCountEditor/index.vue";
import { ElMessage } from 'element-plus';
import { invoke } from '@tauri-apps/api/core';
import { nanoid } from 'nanoid';
import { convertFileSrc } from '@tauri-apps/api/core'

const { proxy } = getCurrentInstance();

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
const tagOptions = ref(['热门', '新品', '推荐', '特价', '限时']); // 预设标签选项
const primaryImageFile = ref(null); // 主图文件
const imageFiles = ref([]); // 图片集合文件
const categoryList = ref([]);
const styleList = ref([]);

const data = reactive({
  form: {},
  tagNumForm: {},
  queryParams: {
    page: 1,
    pageSize: 10,
    categoryId: null,
    clothingNumber: null,
    clothingName: null,
  },
  rules: {
    categoryId: [
      { required: true, message: "服务品类不能为空", trigger: "blur" }
    ],
    styleId: [
      { required: true, message: "所属分类", trigger: "blur" }
    ],
    title: [
      { required: true, message: "衣物名称不能为空", trigger: "blur" }
    ],
    clothingBasePrice: [
      { required: true, message: "基准价格不能为空", trigger: "blur" }
    ],
    clothingMinPrice: [
      { required: true, message: "最低价格不能为空", trigger: "blur" },
      { validator: validateMinPrice, trigger: 'blur' }
    ],
    primaryImage: [
      { required: true, message: "主图不能为空", trigger: "blur" }
    ],
    hangType: [
      { required: true, message: "挂衣方式不能为空", trigger: "change" }
    ],
  },
  refNumFormRules: {
    refNumber: [{ required: true, message: "使用次数不能为空", trigger: "blur" }],
  }
});

const { queryParams, form, tagNumForm, rules } = toRefs(data);

// 自定义校验最低价格函数
function validateMinPrice(rule, value, callback) {
  if (value && Number(value) > Number(form.value.clothingBasePrice)) {
    callback(new Error("最低价格不能超过基准价格"));
  } else {
    callback();
  }
};

// 初始化所需要的字典数据
async function initDictList() {
  await listCategoryAll().then(res => {
    categoryList.value = res;
  })
}

// 当品类发生变化时动态查询子分类列表
async function cateChange(id) {
  form.value.styleId = null;
  await listStyleByCategoryId(id).then(res => {
    styleList.value = res;
  })
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
    id: null,
    categoryId: null,
    styleId: null,
    title: null,
    clothingBasePrice: null,
    clothingMinPrice: null,
    stockQuantity: 0,
    orderNum: 0,
    clothingDegree: 0,
    isPutOnSale: true,
    primaryImage: null,
    images: null,
    imagesVec: [],
    tagListVec: [],
    hangType: '1',
    remark: null
  };
  primaryImageFile.value = null;
  imageFiles.value = [];
  proxy.resetForm("clothingRef");
}

/** 搜索按钮操作 */
function handleQuery() {
  queryParams.value.page = 1;
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
  title.value = "添加衣物";
}

/** 修改按钮操作 */
function handleUpdate(row) {
  reset();
  const _clothingId = row.id || ids.value
  getClothing(_clothingId).then(response => {
    form.value = response;
    initDictList();
    // handle image file path
    if (form.value.primaryImage && form.value.primaryImage !== '') {
      form.value.primaryImage = convertFileSrc(form.value.primaryImage);
    }
    if (form.value.images && form.value.images !== '') {
      form.value.imagesVec = form.value.images.split(',');
      form.value.imagesVec.forEach((image, index) => {
        form.value.imagesVec[index] = convertFileSrc(image);
      });
      imageFiles.value = form.value.imagesVec;
    }
    open.value = true;
    title.value = "修改衣物";
  });
}

// 下一步按钮操作
function nextStep() {
  // 根据当前步骤验证表单
  if (activeStep.value === 0) {
    // 验证基本信息
    proxy.$refs["clothingRef"].validateField(["title", "categoryId", "styleId"], valid => {
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
function getOriginalPath(convertedUrl) {
  if (!convertedUrl) return '';

  // 如果是普通路径（未转换的），直接返回
  if (!convertedUrl.startsWith('http://asset.localhost/')) {
    return convertedUrl;
  }

  try {
    const pathPart = convertedUrl.replace(/^https?:\/\/asset\.localhost\//, '');
    const decodedPath = decodeURIComponent(pathPart);
    return decodedPath.replace(/\//g, '\\');
  } catch (error) {
    console.error('路径转换失败:', error, convertedUrl);
    return convertedUrl;
  }
}
/** 提交按钮 */
async function submitForm() {
  proxy.$refs["clothingRef"].validate(async valid => {
    if (valid) {
      try {
        loading.value = true;

        // 处理图片上传
        let primaryImagePath = null;
        let imagesPath = [];

        if (primaryImageFile.value) {
          primaryImagePath = await saveImageToLocal(primaryImageFile.value);
          if (!primaryImagePath) return;
        }

        if (imageFiles.value.length > 0) {
          for (const file of imageFiles.value) {
            const path = await saveImageToLocal(file);
            if (path) imagesPath.push(path);
          }
        }

        // 准备提交数据
        const submitData = { ...form.value };

        // 设置图片路径
        if (primaryImagePath) submitData.primaryImage = primaryImagePath;
        if (imagesPath.length > 0) {
          submitData.imagesVec = imagesPath;
          submitData.images = imagesPath.join(',');
        } else {
          submitData.images = null;
          submitData.imagesVec = [];
        }

        // 处理标签列表
        if (submitData.tagListVec && submitData.tagListVec.length > 0) {
          submitData.tagList = submitData.tagListVec.join(',');
        }

        console.log('提交数据:', submitData);
        if (form.value.id != null) {
          submitData.primaryImage = getOriginalPath(submitData.primaryImage);
          submitData.imagesVec = submitData.imagesVec.map(image => getOriginalPath(image));
          submitData.images = submitData.imagesVec.join(',');
          console.log('修改数据:', submitData);
          await updateClothing(submitData);
          proxy.notify.success("修改成功");
          reset();
          open.value = false;
          activeStep.value = 0; // 重置步骤
          getList();
        } else {
          const res = await addClothing(submitData);
          console.log('新增成功:', res);
          proxy.notify.success("新增成功");
          reset();
          open.value = false;
          activeStep.value = 0; // 重置步骤
          getList();
        }
      } catch (error) {
        console.error('提交失败:', error);
        proxy.notify.error('提交失败');
      } finally {
        loading.value = false;
      }
    }
  });
}

/** 删除按钮操作 */
function handleDelete(row) {
  const _clothingIds = row.id || ids.value;
  proxy.$modal.confirm('是否确认删除衣物管理编号为"' + _clothingIds + '"的数据项？').then(function () {
    return delClothing(_clothingIds);
  }).then(() => {
    getList();
    proxy.notify.success("删除成功");
  }).catch(() => { });
}

/* 衣物类别变化触发查询 */
function selectChange() {
  queryParams.value.page = 1;
  getList();
}

// 图片上传前的验证
function beforeImageUpload(file) {
  const isJPG = file.type === 'image/jpeg' || file.type === 'image/png';
  const isLt2M = file.size / 1024 / 1024 < 2;

  if (!isJPG) {
    ElMessage.error('上传图片只能是 JPG 或 PNG 格式!');
    return false;
  }
  if (!isLt2M) {
    ElMessage.error('上传图片大小不能超过 2MB!');
    return false;
  }
  return true;
}

// 主图选择处理
async function handlePrimaryImageChange(file) {
  if (!beforeImageUpload(file.raw)) return;
  primaryImageFile.value = file.raw;

  // 显示预览
  const reader = new FileReader();
  reader.onload = (e) => {
    form.value.primaryImage = e.target.result;
  };
  reader.readAsDataURL(file.raw);
}

function handleExceed(files, fileList) {
  proxy.notify.warning(
    `当前限制选择 5 个文件，本次选择了 ${files.length} 个文件，共选择了 ${files.length + fileList.length} 个文件`
  )
}

// 图片集合选择处理
async function handleImageChange(file) {
  if (!beforeImageUpload(file.raw)) return;
  imageFiles.value.push(file.raw);

  // 显示预览
  const reader = new FileReader();
  reader.onload = (e) => {
    if (!form.value.imagesVec) {
      form.value.imagesVec = [];
    }
    form.value.imagesVec.push(e.target.result);
  };
  reader.readAsDataURL(file.raw);
}

// 移除图片处理
async function handleImageRemove(file) {
  const index = form.value.imagesVec.indexOf(file.url);
  if (index !== -1) {
    // 获取该索引下的值
    const path = form.value.imagesVec[index];
    await deleteFile(path).catch(() => {
      proxy.notify.error("删除图片失败");
    });
    form.value.imagesVec.splice(index, 1);
    imageFiles.value.splice(index, 1);
  }
}

// 保存图片到本地
async function saveImageToLocal(file) {
  try {
    const fileData = await file.arrayBuffer();
    const fileName = `${nanoid()}_${file.name}`;
    const result = await invoke('save_image', {
      name: fileName,
      data: Array.from(new Uint8Array(fileData))
    });
    return result.path;
  } catch (error) {
    console.error('保存图片失败:', error);
    ElMessage.error('保存图片失败');
    return null;
  }
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

.step-content {
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

.avatar {
  width: 100%;
  height: 100%;
  display: block;
}

.avatar-uploader .el-upload {
  border: 1px dashed var(--el-border-color);
  border-radius: 6px;
  cursor: pointer;
  position: relative;
  overflow: hidden;
  transition: var(--el-transition-duration-fast);
}

.avatar-uploader .el-upload:hover {
  border-color: var(--el-color-primary);
}

.avatar-uploader-icon {
  font-size: 28px;
  color: #8c939d;
  width: 178px;
  height: 178px;
  text-align: center;
  line-height: 178px;
}
</style>
