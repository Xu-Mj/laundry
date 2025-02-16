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
      <div v-for="item in tableData" :key="item.id" class="row">
        <!-- 上半部分 -->
        <div class="cell">{{ item.clothInfo.clothingName ? item.clothInfo.clothingName : '-' }}</div>
        <div class="cell">{{ item.priceValue }} 元</div>
        <div class="cell">{{ item.processMarkup }} 元</div>
        <div class="cell">{{ item.serviceRequirement }}</div> <!-- 洗护要求留空 -->
        <div class="cell">{{ calculateTotalPrice(item.priceValue, item.processMarkup) }} 元</div>
        <div class="cell action-cell">
          <el-button type="danger" icon="Delete" @click="handleDelete(item.id)" />
        </div>

        <!-- 下半部分 -->
        <div class="cell remark-cell"
          :class="{ 'hidden': !item.remark && (!item.clothingFlawArr || item.clothingFlawArr.length == 0) && (!item.estimateArr || item.estimateArr.length == 0) }">
          <el-tag v-for="tagId in item.clothingFlawArr" :key="tagId" type="danger">
            {{ flawList.find(item => item.tagId == tagId).tagName }}
          </el-tag>
          <span
            v-if="item.clothingFlawArr && item.clothingFlawArr.length > 0 && (item.estimateArr && item.estimateArr.length > 0 || item.remark)">||</span>
          <el-tag v-for="tagId in item.estimateArr" :key="tagId" type="primary">
            {{ estimateList.find(item => item.tagId == tagId).tagName }}
          </el-tag>
          <span v-if="item.estimateArr && item.estimateArr.length > 0 && item.remark">||</span>
          {{ item.remark }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { onMounted } from 'vue';
import { listTagsNoLimit } from "@/api/system/tags";


// 定义 Props
const props = defineProps({
  tableData: {
    type: Array,
    required: true,
  },
});

console.log(props);
// 定义 Emits
const emit = defineEmits(['delete']);

const colorList = ref([]);
const flawList = ref([]);
const estimateList = ref([]);
const brandList = ref([]);

// 计算小计
const calculateTotalPrice = (priceValue, processMarkup) => {
  return (parseFloat(priceValue) || 0) + (parseFloat(processMarkup) || 0);
};

// 删除操作
const handleDelete = (id) => {
  emit('delete', id);
};

// 获取颜色名称
function findColorName() {
  if (form.value.clothingColor) {
    const color = colorList.value.find(item => item.tagId == form.value.clothingColor);
    return color ? color.tagName : '未选择颜色';
  } else {
    return '未选择颜色';
  }
}

// 获取衣物名称
function findClothingName() {
  if (form.value.clothingId) {
    const color = clothingList.value.find(item => item.clothingId == form.value.clothingId);
    return color ? color.clothingName : '未选择衣物';
  } else {
    return '未选择衣物';
  }
}

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
</script>

<style scoped>
.custom-table {
  width: 100%;
  display: flex;
  flex-direction: column;
  border: 1px solid #ebeef5;
  border-radius: 8px;
  overflow: hidden;
  /* box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1); */
  text-align: center;
}

.header {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 1fr 1fr 80px;
  /* 6 列，操作列固定宽度 */
  gap: 1px;
  /* background: linear-gradient(135deg, #f5f7fa, #e9ecef); 渐变色背景 */
  border-bottom: 2px solid #ddd;
  /* 底部边框 */
  padding: .4rem;
}

.header-cell {
  padding: 16px;
  font-size: 14px;
  font-weight: bold;
  color: #333;
  text-align: center;
  border-right: 1px solid #ddd;
  /* 单元格右边框 */
}

.header-cell:last-child {
  border-right: none;
  /* 去掉最后一列的右边框 */
}

.body {
  background-color: #fff;
}

.row {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 1fr 1fr 80px;
  /* 6 列，操作列固定宽度 */
  gap: 1px;
  background-color: #f5f7fa;
}

.cell {
  padding: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-bottom: 1px solid #ebeef5;
  border-right: 1px solid #ebeef5;
}

.cell:last-child {
  border-right: none;
}

.action-cell {
  grid-row: span 2;
  /* 操作列占两行 */
  display: flex;
  align-items: center;
  justify-content: center;
}

.remark-cell {
  grid-column: span 5;
  /* 备注列跨 5 列 */
  display: flex;
  align-items: center;
  justify-content: flex-start;
  padding-left: 12px;
}

.hidden {
  display: none;
  /* 隐藏备注行 */
}

.row:last-child .cell {
  border-bottom: none;
}
</style>