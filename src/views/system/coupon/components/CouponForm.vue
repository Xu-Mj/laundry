<template>
  <div class="coupon-form-container">
    <el-form ref="couponFormRef" :model="formData" :rules="rules" label-width="90px">
      <!-- 基本信息卡片 -->
      <div class="form-card">
        <div class="card-header">
          <i class="el-icon-info"></i>
          <span>基本信息</span>
        </div>
        <div class="card-body">
          <el-row :gutter="20">
            <el-col :span="12">
              <el-form-item label="卡券名称" prop="couponTitle">
                <el-input v-model="formData.couponTitle" placeholder="请输入卡券名称" />
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="卡券类别" prop="couponType">
                <el-select v-model="formData.couponType" placeholder="卡券类别" clearable class="w-full">
                  <el-option v-for="dict in couponTypes" :key="dict.value" :label="dict.label" :value="dict.value" />
                </el-select>
              </el-form-item>
            </el-col>
          </el-row>
        </div>
      </div>

      <!-- 价值信息卡片 -->
      <div class="form-card">
        <div class="card-header">
          <i class="el-icon-money"></i>
          <span>价值信息</span>
        </div>
        <div class="card-body">
          <!-- 储值卡 -->
          <el-row v-if="formData.couponType === '000'" :gutter="20">
            <el-col :span="12">
              <el-form-item label="储值金额" prop="couponValue">
                <el-input-number v-model="formData.couponValue" controls-position="right" placeholder="请输入储值金额"
                  class="w-full" />
                <div class="form-tip">卡券售价</div>
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="赠送金额" prop="usageValue">
                <el-input-number v-model="formData.usageValue" controls-position="right" placeholder="请输入赠送金额"
                  class="w-full" />
                <div class="form-tip">额外赠送的金额</div>
              </el-form-item>
            </el-col>
          </el-row>

          <!-- 代金券 -->
          <el-row v-if="formData.couponType === '001'" :gutter="20">
            <el-col :span="12">
              <el-form-item label="售卖价格" prop="couponValue">
                <el-input-number v-model="formData.couponValue" controls-position="right" placeholder="请输入售卖价格"
                  class="w-full" />
                <div class="form-tip">卡券售价</div>
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="卡券价值" prop="usageValue">
                <el-input-number v-model="formData.usageValue" controls-position="right" placeholder="请输入卡券价值"
                  class="w-full" />
                <div class="form-tip">可抵扣的金额</div>
              </el-form-item>
            </el-col>
          </el-row>

          <!-- 次卡 -->
          <el-row v-if="formData.couponType === '002'" :gutter="20">
            <el-col :span="12">
              <el-form-item label="售卖价格" prop="couponValue">
                <el-input-number v-model="formData.couponValue" controls-position="right" placeholder="请输入售卖价格"
                  class="w-full" />
                <div class="form-tip">卡券售价</div>
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="卡券次数" prop="usageValue">
                <el-input-number v-model="formData.usageValue" controls-position="right" placeholder="请输入卡券次数"
                  class="w-full" />
                <div class="form-tip">可使用的次数</div>
              </el-form-item>
            </el-col>
          </el-row>

          <!-- 折扣券 -->
          <el-row v-if="formData.couponType === '003'" :gutter="20">
            <el-col :span="12">
              <el-form-item label="售卖价格" prop="couponValue">
                <el-input-number v-model="formData.couponValue" controls-position="right" placeholder="请输入售卖价格"
                  class="w-full" />
                <div class="form-tip">卡券售价</div>
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="折扣比例" prop="usageValue">
                <el-input-number v-model="formData.usageValue" :min="0" :max="100" controls-position="right"
                  placeholder="请输入折扣比例" class="w-full">
                  <template #suffix>
                    <span class="input-suffix">%</span>
                  </template>
                </el-input-number>
                <div class="form-tip">折扣百分比</div>
              </el-form-item>
            </el-col>
          </el-row>
          <el-row v-if="formData.couponType === '003'" :gutter="20">
            <el-col :span="12">
              <el-form-item label="至多优惠" prop="usageLimit">
                <el-input-number v-model="formData.usageLimit" controls-position="right" placeholder="折扣券的上限优惠金额"
                  class="w-full" />
                <div class="form-tip">最高优惠金额限制</div>
              </el-form-item>
            </el-col>
          </el-row>

          <!-- 满减券 -->
          <el-row v-if="formData.couponType === '004'" :gutter="20">
            <el-col :span="12">
              <el-form-item label="售卖价格" prop="couponValue">
                <el-input-number v-model="formData.couponValue" @change="formData.usageValue = formData.couponValue"
                  controls-position="right" placeholder="售卖价格" class="w-full" />
                <div class="form-tip">卡券售价</div>
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="满减金额" prop="usageValue">
                <el-input-number v-model="formData.usageValue" :min="formData.couponValue" controls-position="right"
                  placeholder="请输入满减金额" class="w-full" />
                <div class="form-tip">满足条件后可减免的金额</div>
              </el-form-item>
            </el-col>
          </el-row>
          <el-row v-if="formData.couponType === '004'" :gutter="20">
            <el-col :span="12">
              <el-form-item label="最低消费金额" prop="minSpend">
                <el-input-number v-model="formData.minSpend" controls-position="right" placeholder="请输入最低消费金额"
                  class="w-full" />
                <div class="form-tip">满足此金额才可使用</div>
              </el-form-item>
            </el-col>
          </el-row>
        </div>
      </div>

      <!-- 使用规则卡片 -->
      <div class="form-card">
        <div class="card-header">
          <i class="el-icon-setting"></i>
          <span>使用规则</span>
        </div>
        <div class="card-body">
          <el-row :gutter="20">
            <el-col :span="12">
              <el-form-item label="客户可见" prop="customerInvalid">
                <el-switch v-model="formData.customerInvalid" active-value="0" inactive-value="1" active-text="可见"
                  inactive-text="不可见" inline-prompt />
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="自动延期" prop="autoDelay">
                <el-switch v-model="formData.autoDelay" active-value="0" inactive-value="1" active-text="是"
                  inactive-text="否" inline-prompt />
              </el-form-item>
            </el-col>
          </el-row>
          <el-row :gutter="20">
            <el-col :span="12">
              <el-form-item label="总量限制" prop="customerSaleTotal">
                <el-input-number min="-1" v-model="formData.customerSaleTotal" controls-position="right"
                  placeholder="-1为不限制" class="w-full" />
                <div class="form-tip">卡券可出售总量限制，-1为不限制</div>
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="单用户限制" prop="customerSaleCount" label-width="90px">
                <el-input-number :min="-1" v-model="formData.customerSaleCount" controls-position="right"
                  placeholder="-1为不限制" class="w-full" />
                <div class="form-tip">单用户可购买数量限制，-1为不限制</div>
              </el-form-item>
            </el-col>
          </el-row>
          <el-row :gutter="20">
            <el-col :span="12">
              <el-form-item label="有效期-起" prop="validFrom">
                <el-date-picker clearable v-model="formData.validFrom" type="date" value-format="YYYY-MM-DD"
                  placeholder="请选择有效期-起" class="w-full">
                </el-date-picker>
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="有效期-止" prop="validTo">
                <el-date-picker clearable v-model="formData.validTo" type="date" value-format="YYYY-MM-DD"
                  placeholder="请选择有效期-止" class="w-full">
                </el-date-picker>
              </el-form-item>
            </el-col>
          </el-row>
        </div>
      </div>

      <!-- 其他信息卡片 -->
      <div class="form-card">
        <div class="card-header">
          <i class="el-icon-more"></i>
          <span>其他信息</span>
        </div>
        <div class="card-body">
          <el-row :gutter="20">
            <el-col :span="12">
              <el-form-item label="卡券状态" prop="status">
                <el-select v-model="formData.status" placeholder="卡券状态" clearable class="w-full">
                  <el-option v-for="dict in statusOptions" :key="dict.value" :label="dict.label" :value="dict.value" />
                </el-select>
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="卡券描述" prop="desc">
                <el-input v-model="formData.desc" type="textarea" placeholder="请输入内容" />
              </el-form-item>
            </el-col>
          </el-row>
          <el-form-item label="备注" prop="remark">
            <el-input v-model="formData.remark" type="textarea" placeholder="请输入内容" />
          </el-form-item>
        </div>
      </div>
    </el-form>

    <div class="form-actions">
      <el-button @click="cancel">取 消</el-button>
      <el-button type="primary" @click="submitForm">确 定</el-button>
    </div>
  </div>
</template>

<script setup>
import { ref, defineProps, defineEmits } from 'vue';

const props = defineProps({
  value: {
    type: Object,
    required: true
  },
  couponTypes: {
    type: Array,
    required: true
  },
  statusOptions: {
    type: Array,
    required: true
  }
});

const emit = defineEmits(['submit', 'cancel']);

const couponFormRef = ref(null);
const formData = ref({ ...props.value });

// 表单验证规则
const rules = {
  couponType: [
    { required: true, message: "卡券类型不能为空", trigger: "change" }
  ],
  couponTitle: [
    { required: true, message: "卡券名称不能为空", trigger: "blur" }
  ],
  couponValue: [
    { required: true, message: "售卖价格不能为空", trigger: "blur" }
  ],
  usageValue: [
    { required: true, message: "卡券价值不能为空", trigger: "blur" }
  ],
  validFrom: [
    { required: true, message: "有效期-起不能为空", trigger: "blur" },
    { validator: validateValidFrom, trigger: "blur" }
  ],
  validTo: [
    { required: true, message: "有效期-止不能为空", trigger: "blur" },
    { validator: validateValidTo, trigger: "blur" }
  ],
};

// 校验起始日期
function validateValidFrom(rule, value, callback) {
  if (value && formData.value.validTo && value > formData.value.validTo) {
    callback(new Error("起始日期不能大于截至日期"));
  } else {
    callback();
  }
};

// 校验截止日期
function validateValidTo(rule, value, callback) {
  if (value && formData.value.validFrom && value < formData.value.validFrom) {
    callback(new Error("截止日期不能小于起始日期"));
  } else {
    callback();
  }
};

// 提交表单
const submitForm = () => {
  couponFormRef.value.validate((valid) => {
    if (valid) {
      // 处理储值卡的特殊逻辑
      if (formData.value.couponType === '000') {
        const originalUsageValue = formData.value.usageValue;
        formData.value.usageValue = formData.value.couponValue + originalUsageValue;
      }

      emit('submit', formData.value);
    }
  });
};

// 取消操作
const cancel = () => {
  emit('cancel');
};
</script>

<style scoped>
.coupon-form-container {
  padding: 10px;
}

.form-card {
  background-color: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.05);
  margin-bottom: 20px;
  overflow: hidden;
  transition: all 0.3s ease;
}

.form-card:hover {
  box-shadow: 0 4px 16px 0 rgba(0, 0, 0, 0.1);
}

.card-header {
  background-color: #f5f7fa;
  padding: 12px 15px;
  font-weight: 600;
  color: #303133;
  border-bottom: 1px solid #ebeef5;
  display: flex;
  align-items: center;
}

.card-header i {
  margin-right: 8px;
  color: #409eff;
}

.card-body {
  padding: 20px 15px;
}

.form-tip {
  font-size: 12px;
  color: #909399;
  margin-top: 4px;
  line-height: 1.2;
}

.form-actions {
  display: flex;
  justify-content: center;
  gap: 15px;
  margin-top: 20px;
}

.w-full {
  width: 100%;
}

.input-suffix {
  margin-left: 5px;
  color: #606266;
}

:deep(.el-form-item__label) {
  font-weight: 500;
}

:deep(.el-input__inner),
:deep(.el-select),
:deep(.el-input-number) {
  border-radius: 4px;
}

:deep(.el-switch) {
  --el-switch-on-color: #13ce66;
}

:deep(.el-date-editor) {
  width: 100%;
}

:deep(.el-textarea__inner) {
  min-height: 80px;
  border-radius: 4px;
}
</style>