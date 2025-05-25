<template>
  <!-- 添加或修改价格管理对话框 -->
  <el-dialog v-model="visible" :show-close="false" width="560px" @opened="refNumberGetFocus"
    @closed="refNumberFocus = false" align-center class="price-dialog">
    <template #header>
      <div class="dialog-header hover-flow">
        <h2 class="dialog-title">{{ form.priceId ? '修改价格' : '新增价格' }}</h2>
        <el-button circle @click="cancel">
          <el-icon>
            <Close />
          </el-icon>
        </el-button>
      </div>
    </template>

    <el-form ref="priceRef" :model="form" :rules="rules" label-width="80px" class="price-form">
      <!-- 基本信息卡片 -->
      <div class="form-card hover-flow">
        <div class="card-header">
          <el-icon>
            <InfoFilled />
          </el-icon>
          <span>基本信息</span>
        </div>
        <div class="card-body">
          <el-row :gutter="20">
            <el-col :span="12">
              <el-form-item label="价格名称" prop="priceName">
                <el-input v-model="form.priceName" placeholder="请输入价格名称" class="custom-input" />
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="订单类别" prop="orderType">
                <el-select v-model="form.orderType" placeholder="请选择订单类别" class="custom-select">
                  <el-option v-for="dict in sys_price_order_type" :key="dict.value" :label="dict.label"
                    :value="dict.value" />
                </el-select>
              </el-form-item>
            </el-col>
          </el-row>
        </div>
      </div>

      <!-- 价格设置卡片 -->
      <div class="form-card hover-flow">
        <div class="card-header">
          <el-icon>
            <Money />
          </el-icon>
          <span>价格设置</span>
        </div>
        <div class="card-body">
          <el-row :gutter="20">
            <el-col :span="12">
              <el-form-item label="价格" prop="priceValue">
                <el-input-number v-model="form.priceValue" controls-position="right" placeholder="请输入价格"
                  :disabled="isPriceValueDisabled" class="custom-input-number">
                  <template #prefix>
                    <el-icon>
                      <Coin />
                    </el-icon>
                  </template>
                </el-input-number>
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item v-if="form.orderType == '03'" label="折扣系数" prop="priceDiscount">
                <el-input-number v-model="form.priceDiscount" controls-position="right" :min="0" :max="100"
                  placeholder="请输入折扣系数" :disabled="isPriceDiscountDisabled" class="custom-input-number">
                  <template #prefix>
                    <el-icon>
                      <Discount />
                    </el-icon>
                  </template>
                </el-input-number>
              </el-form-item>
            </el-col>
          </el-row>

          <div class="price-info" v-if="form.priceValue || form.priceDiscount">
            <el-alert type="info" :closable="false" show-icon>
              <template #title>
                <span>{{ form.priceValue ? '已设置固定价格' : '已设置折扣系数：' + form.priceDiscount + '%' }}</span>
              </template>
              <template #default>
                <p>{{ form.priceValue && form.priceDiscount ? '价格和折扣系数只能设置一个' : '请设置价格或折扣系数其中一项' }}</p>
              </template>
            </el-alert>
          </div>
        </div>
      </div>

      <!-- 其他设置卡片 -->
      <div class="form-card hover-flow">
        <div class="card-header">
          <el-icon>
            <Setting />
          </el-icon>
          <span>其他设置</span>
        </div>
        <div class="card-body">
          <el-form-item label="状态" class="status-item">
            <el-radio-group v-model="form.status" class="custom-radio-group">
              <el-radio v-for="dict in sys_normal_disable" :key="dict.value" :value="dict.value" class="custom-radio">
                {{ dict.label }}
              </el-radio>
            </el-radio-group>
          </el-form-item>

          <el-row :gutter="20">
            <el-col :span="12">
              <el-form-item label="显示顺序" prop="orderNum">
                <el-input-number v-model="form.orderNum" :min="0" controls-position="right" placeholder="请输入显示顺序"
                  class="custom-input-number">
                  <template #prefix>
                    <el-icon>
                      <Sort />
                    </el-icon>
                  </template>
                </el-input-number>
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="使用计数" prop="refNum">
                <el-input-number v-model="form.refNum" ref="refNum" :min="0" controls-position="right"
                  placeholder="请输入使用计数" class="custom-input-number">
                  <template #prefix>
                    <el-icon>
                      <Odometer />
                    </el-icon>
                  </template>
                </el-input-number>
              </el-form-item>
            </el-col>
          </el-row>

          <el-form-item label="备注" prop="remark">
            <el-input v-model="form.remark" type="textarea" placeholder="请输入内容" class="custom-textarea" :rows="3" />
          </el-form-item>
        </div>
      </div>
    </el-form>

    <template #footer>
      <div class="dialog-footer">
        <el-button class="hover-flow" type="primary" @click="submitForm" icon="Check"> 确 定</el-button>
        <el-button class="hover-flow" type="danger" @click="cancel" icon="Close">取 消</el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup name="PriceCreateDialog">
import { addPrice, updatePrice, getPrice } from "@/api/system/price";

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false
  },
  priceId: {
    type: Number,
    default: null
  },
  defaultOrderType: {
    type: String,
    default: null
  }
});

const emit = defineEmits(['update:modelValue', 'success', 'cancel']);

const { proxy } = getCurrentInstance();
const { sys_price_order_type, sys_normal_disable } = proxy.useDict("sys_price_order_type", "sys_normal_disable");

const visible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
});

const refNumberFocus = ref(false);
const refNum = ref();

const data = reactive({
  form: {
    priceId: null,
    orderType: null,
    priceName: null,
    priceValue: null,
    priceDiscount: null,
    applicableCloths: null,
    status: "0",
    orderNum: 0,
    refNum: 0,
    remark: null,
  },
  rules: {
    orderType: [
      { required: true, message: "订单类别不能为空", trigger: "change" }
    ],
    priceName: [
      { required: true, message: "价格名称不能为空", trigger: "blur" }
    ],
    orderNum: [
      { required: true, message: "显示顺序不能为空", trigger: "blur" }
    ],
  }
});

const { form, rules } = toRefs(data);

// 是否禁用 priceValue 和 priceDiscount
const isPriceValueDisabled = ref(false);
const isPriceDiscountDisabled = ref(false);

// 监听 priceValue 和 priceDiscount 的变化
watch(
  () => form.value.priceValue,
  (newValue) => {
    if (newValue) {
      isPriceDiscountDisabled.value = true;
    } else {
      isPriceDiscountDisabled.value = false;
    }
  }
);

watch(
  () => form.value.priceDiscount,
  (newValue) => {
    if (newValue) {
      isPriceValueDisabled.value = true;
    } else {
      isPriceValueDisabled.value = false;
    }
  }
);

// 监听弹窗打开，初始化表单
watch(
  () => props.modelValue,
  (newValue) => {
    if (newValue) {
      reset();
      if (props.priceId) {
        // 编辑模式
        getPrice(props.priceId).then(response => {
          form.value = response;
          if (form.value.applicableCloths) {
            form.value.applicableClothsArr = form.value.applicableCloths.split(",");
          }
        });
      } else {
        // 新增模式，设置默认订单类型
        if (props.defaultOrderType) {
          form.value.orderType = props.defaultOrderType;
        }
      }
    }
  }
);

// 表单重置
function reset() {
  form.value = {
    priceId: null,
    orderType: null,
    priceName: null,
    priceValue: null,
    priceDiscount: null,
    applicableCloths: null,
    status: "0",
    orderNum: 0,
    refNum: 0,
    remark: null,
  };
  proxy.resetForm("priceRef");
}

// 取消按钮
function cancel() {
  visible.value = false;
  emit('cancel');
  reset();
}

/** 提交按钮 */
function submitForm() {
  proxy.$refs["priceRef"].validate(valid => {
    if (valid) {
      if (!form.value.priceValue && !form.value.priceDiscount) {
        proxy.notify.error('价格和折扣至少填写一个');
        return;
      }
      if (form.value.applicableClothsArr && form.value.applicableClothsArr.length > 0) {
        form.value.applicableCloths = form.value.applicableClothsArr.join(",");
        delete form.value.applicableClothsArr;
      }
      if (form.value.priceId != null) {
        updatePrice(form.value).then(response => {
          proxy.notify.success("修改成功");
          visible.value = false;
          emit('success');
        });
      } else {
        addPrice(form.value).then(response => {
          proxy.notify.success("新增价格方案成功");
          visible.value = false;
          emit('success');
        });
      }
    }
  });
}

/* 点击修改使用计数时，输入框获取焦点 */
function refNumberGetFocus() {
  if (refNumberFocus.value) {
    refNum.value.focus();
  }
}
</script>

<style scoped>
.price-dialog :deep(.el-dialog__header) {
  margin: 0;
  padding: 15px 20px;
  border-bottom: 1px solid var(--el-border-color-light);
}

.price-form {
  padding: 10px 0;
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

.custom-input :deep(.el-input__wrapper),
.custom-select :deep(.el-input__wrapper),
.custom-input-number :deep(.el-input__wrapper) {
  box-shadow: 0 0 0 1px var(--el-border-color) inset;
  border-radius: 4px;
  transition: all 0.3s;
  width: 100%;
}

.custom-input :deep(.el-input__wrapper:hover),
.custom-select :deep(.el-input__wrapper:hover),
.custom-input-number :deep(.el-input__wrapper:hover) {
  box-shadow: 0 0 0 1px var(--el-color-primary-light-5) inset;
}

.custom-input :deep(.el-input__wrapper.is-focus),
.custom-select :deep(.el-input__wrapper.is-focus),
.custom-input-number :deep(.el-input__wrapper.is-focus) {
  box-shadow: 0 0 0 1px var(--el-color-primary) inset;
}

.custom-textarea :deep(.el-textarea__inner) {
  box-shadow: 0 0 0 1px var(--el-border-color) inset;
  border-radius: 4px;
  transition: all 0.3s;
}

.custom-textarea :deep(.el-textarea__inner:hover) {
  box-shadow: 0 0 0 1px var(--el-color-primary-light-5) inset;
}

.custom-textarea :deep(.el-textarea__inner:focus) {
  box-shadow: 0 0 0 1px var(--el-color-primary) inset;
}

.price-info {
  margin-top: 10px;
}
</style>