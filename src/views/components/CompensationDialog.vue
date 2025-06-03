<template>
  <el-dialog v-model="dialogVisible" width="500px" :align-center="true" :show-close="true" destroy-on-close
    class="compensation-dialog" top="10vh">
    <template #header>
      <div class="compensation-dialog-header">
        <el-icon>
          <Money />
        </el-icon>
        <span>衣物赔偿</span>
      </div>
    </template>

    <div class="compensation-dialog-content">
      <div class="compensation-info-card">
        <div class="compensation-order-info">
          <div class="compensation-info-item">
            <span class="compensation-label">赔偿标题</span>
            <span class="compensation-value">{{ compensationForm.expTitle }}</span>
          </div>
          <div class="compensation-info-item">
            <span class="compensation-label">赔偿状态</span>
            <el-tag size="small" type="success">可赔偿</el-tag>
          </div>
        </div>
        <div class="compensation-divider"></div>
        <div class="selected-clothes" v-if="selectionList.length > 0">
          <div class="compensation-label">已选衣物</div>
          <div class="selected-clothes-list">
            <el-tag v-for="(item, index) in selectionList" :key="index" class="selected-cloth-tag" type="info"
              effect="light">
              {{ item.clothInfo?.title || '衣物' }}
            </el-tag>
          </div>
        </div>
      </div>

      <el-form ref="compensationRef" :model="compensationForm" :rules="compensationRules" label-position="top"
        class="compensation-form">
        <el-form-item label="赔偿名称" prop="expTitle">
          <el-input v-model="compensationForm.expTitle" placeholder="请输入赔偿名称" class="compensation-input">
            <template #prefix>
              <el-icon>
                <Document />
              </el-icon>
            </template>
          </el-input>
        </el-form-item>

        <el-form-item label="对方账户" prop="recvAccountTitle">
          <el-input v-model="compensationForm.accountTitle" disabled class="compensation-input">
            <template #prefix>
              <el-icon>
                <User />
              </el-icon>
            </template>
          </el-input>
        </el-form-item>

        <el-form-item label="赔偿金额" prop="expAmount">
          <el-input-number v-model="compensationForm.expAmount" :precision="2" :step="0.01" :min="0"
            controls-position="right" style="width: 100%" class="compensation-input-number" placeholder="请输入赔偿金额">
            <template #prefix>
              <el-icon>
                <Money />
              </el-icon>
            </template>
          </el-input-number>
        </el-form-item>

        <el-form-item label="备注信息" prop="remark">
          <el-input type="textarea" v-model="compensationForm.remark" placeholder="请输入赔偿原因或其他备注信息" :rows="3"
            resize="none" class="compensation-textarea" />
        </el-form-item>
      </el-form>
    </div>

    <template #footer>
      <div class="compensation-footer">
        <el-button @click="closeDialog" plain>取消</el-button>
        <el-button type="primary" @click="compensate">
          <el-icon>
            <Check />
          </el-icon>确认赔偿
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup>
import { getUser } from "@/api/system/user";
import { addExpenditure } from "@/api/system/expenditure";

const props = defineProps({
  visible: {
    type: Boolean,
    default: false
  },
  selectionList: {
    type: Array,
    default: () => []
  },
  orderId: {
    type: Number,
    required: true
  },
  userId: {
    type: Number,
    required: true
  }
});

const emit = defineEmits(['update:visible', 'success']);

const dialogVisible = ref(false);
const { proxy } = getCurrentInstance();

// 表单数据
const data = reactive({
  compensationForm: {
    expTitle: '',
    accountTitle: '',
    recvAccountTitle: '',
    recvAccount: '',
    expAmount: null,
    expType: "01",
    orderId: props.orderId,
    clothIds: '',
    remark: null
  },
  compensationRules: {
    expTitle: [
      { required: true, message: "支出账目不能为空", trigger: "blur" }
    ],
    expAmount: [
      { required: true, message: "赔偿金额不能为空", trigger: "blur" }
    ]
  }
});

const { compensationForm, compensationRules } = toRefs(data);

// 监听visible属性变化
watch(() => props.visible, (newVal) => {
  dialogVisible.value = newVal;
  if (newVal) {
    initCompensation();
  }
});

// 监听dialogVisible变化，同步回父组件
watch(dialogVisible, (newVal) => {
  emit('update:visible', newVal);
});

// 初始化赔偿数据
async function initCompensation() {
  try {
    // 获取用户信息
    const res = await getUser(props.userId);

    // 创建更专业的赔偿标题
    let selectedCount = props.selectionList.length;
    let title;

    if (selectedCount === 1) {
      title = `衣物赔偿-${props.selectionList[0].clothInfo.title}`;
    } else {
      title = `衣物赔偿-${selectedCount}件衣物`;
    }

    // 设置表单数据
    compensationForm.value = {
      expTitle: title,
      accountTitle: `${res.nickName}-${res.phonenumber}`,
      recvAccountTitle: res.nickName,
      recvAccount: res.userId,
      expAmount: null,
      expType: "01",
      orderId: props.orderId,
      clothIds: props.selectionList.map(item => item.clothId).join(','),
      remark: null,
    };
  } catch (error) {
    console.error('获取用户信息失败:', error);
    proxy.notify.error('获取用户信息失败');
  }
}

// 关闭对话框
function closeDialog() {
  dialogVisible.value = false;
}

// 确认赔偿
function compensate() {
  proxy.$refs["compensationRef"].validate(valid => {
    if (valid) {
      // 转换金额为分
      compensationForm.value.expAmount = compensationForm.value.expAmount * 100;

      addExpenditure(compensationForm.value).then(() => {
        proxy.notify.success("赔偿成功");
        dialogVisible.value = false;
        emit('success');
      }).catch(err => {
        console.error(err);
        proxy.notify.error("赔偿失败");
      });
    }
  });
}
</script>

<style scoped>
/* 赔偿对话框样式 */
.compensation-dialog :deep(.el-dialog__header) {
  margin: 0;
  padding: 20px 24px;
  border-bottom: 1px solid var(--el-border-color-lighter);
}

.compensation-dialog :deep(.el-dialog__body) {
  padding: 24px;
}

.compensation-dialog :deep(.el-dialog__footer) {
  padding: 16px 24px;
  border-top: 1px solid var(--el-border-color-lighter);
}

.compensation-dialog-header {
  display: flex;
  align-items: center;
  gap: 10px;
  color: var(--el-color-primary);
  font-size: 18px;
  font-weight: 600;
}

.compensation-dialog-header .el-icon {
  font-size: 20px;
}

.compensation-dialog-content {
  padding: 20px;
}

.compensation-info-card {
  background-color: var(--el-fill-color-lighter);
  border-radius: 8px;
  padding: 16px;
  box-shadow: var(--el-box-shadow-light);
  margin-bottom: 20px;
  transition: all 0.3s ease;
}

.compensation-info-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--el-box-shadow);
}

.compensation-order-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.compensation-info-item {
  display: flex;
  flex-direction: column;
}

.compensation-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--el-text-color-primary);
}

.compensation-value {
  font-size: 16px;
  font-weight: 600;
  color: var(--el-color-primary);
}

.compensation-divider {
  margin: 12px 0;
  border-top: 1px dashed var(--el-border-color-lighter);
}

.selected-clothes {
  margin: 16px 0;
}

.selected-clothes-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-top: 8px;
}

.selected-cloth-tag {
  display: inline-flex;
  align-items: center;
  background-color: var(--el-fill-color-light);
  padding: 4px 10px;
  border-radius: 4px;
  font-size: 12px;
  color: var(--el-text-color-primary);
  border: 1px solid var(--el-border-color-lighter);
  transition: all 0.2s ease;
}

.selected-cloth-tag:hover {
  transform: translateY(-2px);
  background-color: var(--el-color-primary-light-9);
  color: var(--el-color-primary);
  border-color: var(--el-color-primary-light-5);
}

.compensation-form {
  margin-top: 10px;
}

.compensation-input,
.compensation-input-number,
.compensation-textarea {
  transition: all 0.3s ease;
  border-radius: 6px;
  width: 100%;
}

.compensation-input:hover,
.compensation-input-number:hover,
.compensation-textarea:hover {
  box-shadow: var(--el-box-shadow-light);
}

.compensation-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.compensation-footer button {
  transition: all 0.3s ease;
}

.compensation-footer button:hover {
  transform: translateY(-2px);
}
</style>