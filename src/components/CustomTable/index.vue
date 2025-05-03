<template>
  <div class="custom-table">
    <!-- 表头 -->
    <div class="header">
      <div>衣物</div>
      <div>单价</div>
      <div>工艺</div>
      <div>洗护要求</div>
      <div>小计</div>
      <div>操作</div>
    </div>

    <!-- 表格内容 -->
    <div class="body">
      <div v-if="tableData.length > 0" v-for="item in tableData" :key="item.id" class="row"
        @click="handleRowClick(item)">
        <!-- 上半部分 -->
        <div class="cell">{{ item.clothInfo.title ? item.clothInfo.title : '-' }}</div>
        <div class="cell">{{ item.priceValue }} 元</div>
        <div class="cell">{{ item.processMarkup }} 元</div>
        <div class="cell">
          <dict-tag :options="sys_service_requirement" :value="item.serviceRequirement" />
        </div> <!-- 洗护要求留空 -->
        <div class="cell">{{ calculateTotalPrice(item) }} 元</div>
        <div class="cell action-cell">
          <el-button type="danger" icon="Delete" @click="handleDelete(item.clothId, item.clothInfo.title)" />
        </div>

        <!-- 下半部分 -->
        <div class="cell remark-cell"
          :class="{ 'hidden': !item.remark && (!item.clothingFlawArr || item.clothingFlawArr.length == 0) && (!item.estimateArr || item.estimateArr.length == 0) }">
          <el-tag v-for="tagId in item.clothingFlawArr" :key="tagId" type="danger">
            {{flawList.find(item => item.tagId == tagId).tagName}}
          </el-tag>
          <span
            v-if="item.clothingFlawArr && item.clothingFlawArr.length > 0 && (item.estimateArr && item.estimateArr.length > 0 || item.remark)">||</span>
          <el-tag v-for="tagId in item.estimateArr" :key="tagId" type="primary">
            {{estimateList.find(item => item.tagId == tagId).tagName}}
          </el-tag>
          <span v-if="item.estimateArr && item.estimateArr.length > 0 && item.remark">||</span>
          {{ item.remark }}
        </div>
      </div>
      <div class="empty-info-card" v-else>
        <div class="empty-state">
          <el-empty description="暂无数据~" :image-size="80"></el-empty>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { onMounted, inject } from 'vue';
import { listTagsNoLimit } from "@/api/system/tags";

// 定义 Props
const props = defineProps({
  tableData: {
    type: Array,
    required: true,
  },
});

const { proxy } = getCurrentInstance();
const { sys_service_requirement } = proxy.useDict("sys_service_requirement");

const setSelectedCloth = inject('setSelectedCloth');

// 定义 Emits
const emit = defineEmits(['delete']);

const colorList = ref([]);
const flawList = ref([]);
const estimateList = ref([]);
const brandList = ref([]);

// 计算小计
const calculateTotalPrice = (cloth) => {
  let priceValue = cloth.priceValue || 0;
  if (cloth.serviceRequirement == '001') {
    priceValue *= 2;
  } else if (cloth.serviceRequirement == '002') {
    priceValue *= 1.5;
  }
  return parseFloat(priceValue);
};

// 删除操作
const handleDelete = (id, name) => {
  emit('delete', id, name);
};

/* 初始化列表数据 */
async function initList() {
  const promises = [];

  // 获取颜色列表
  if (colorList.value.length === 0) {
    const colorPromise = listTagsNoLimit({ tagOrder: '003', status: "0" }).then(response => {
      colorList.value = response;
    });
    promises.push(colorPromise);
  }

  // 获取瑕疵列表
  if (flawList.value.length === 0) {
    const flawPromise = listTagsNoLimit({ tagOrder: '001', status: "0" }).then(response => {
      flawList.value = response;
    });
    promises.push(flawPromise);
  }

  // 获取预估列表
  if (estimateList.value.length === 0) {
    const estimatePromise = listTagsNoLimit({ tagOrder: '002', status: "0" }).then(response => {
      estimateList.value = response;
    });
    promises.push(estimatePromise);
  }

  // 获取品牌列表
  if (brandList.value.length === 0) {
    const brandPromise = listTagsNoLimit({ tagOrder: '004', status: "0" }).then(response => {
      brandList.value = response;
    });
    promises.push(brandPromise);
  }

  // 等待所有异步操作完成防止衣物列表数据加载完后这里的数据没有准备好而出错
  await Promise.all(promises);
}

onMounted(async () => {
  await initList();
});

// 处理行点击事件
const handleRowClick = (row) => {
  setSelectedCloth(row);
};
</script>


<style scoped>
.custom-table {
  width: 100%;
  display: flex;
  flex-direction: column;
  border: 1px solid var(--el-border-color-lighter);
  border-radius: 12px;
  overflow: hidden;
  text-align: center;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.05);
  margin-bottom: 1rem;
  transition: all 0.3s ease;
}

.custom-table:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.header {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 1fr 1fr 80px;
  gap: 1px;
  background-color: var(--el-color-primary-light-9);
  padding: 0.75rem 0.5rem;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

:root.dark .header {
  --el-color-primary-light-9: #1d2c40;
}

.header>div {
  padding: 8px;
}

.row {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 1fr 1fr 80px;
  gap: 1px;
  cursor: pointer;
  transition: all 0.2s;
}

.row:hover {
  background-color: var(--el-fill-color-light);
  transform: translateY(-1px);
}

.cell {
  padding: 12px 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-bottom: 1px solid var(--el-border-color-lighter);
}

.action-cell {
  grid-row: span 2;
  display: flex;
  align-items: center;
  justify-content: center;
}

.action-cell .el-button {
  transition: all 0.3s;
}

.action-cell .el-button:hover {
  transform: scale(1.1);
}

.remark-cell {
  grid-column: span 5;
  display: flex;
  align-items: center;
  justify-content: flex-start;
  padding: 8px 16px;
  gap: 8px;
  flex-wrap: wrap;
}

.remark-cell .el-tag {
  margin-right: 6px;
  transition: all 0.3s;
}

.remark-cell .el-tag:hover {
  transform: translateY(-2px);
}

.hidden {
  display: none;
}

.row:last-child .cell {
  border-bottom: none;
}

.empty-state {
    display: flex;
    justify-content: center;
}

.empty-info-card {
    display: flex;
    align-items: center;
    justify-content: center;
}

</style>