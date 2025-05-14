<template>
  <div class="custom-table">
    <!-- 表头 -->
    <div class="header">
      <div class="header-cell">衣物</div>
      <div class="header-cell">单价</div>
      <div class="header-cell">工艺</div>
      <div class="header-cell">洗护要求</div>
      <div class="header-cell">小计</div>
      <div class="header-cell">操作</div>
    </div>

    <!-- 表格内容 -->
    <div class="body">
      <div v-if="tableData.length > 0" v-for="item in tableData" :key="item.id" class="row"
        @click="handleRowClick(item)">
        <!-- 上半部分 -->
        <div class="cell title-cell">
          <el-tooltip
            :content="item.clothInfo.title"
            placement="top"
            :show-after="200"
            :enterable="false"
            effect="dark"
          >
            <div class="ellipsis-text">{{ item.clothInfo.title ? item.clothInfo.title : '-' }}</div>
          </el-tooltip>
        </div>
        <div class="cell price-cell">
          <span class="ellipsis-text">{{ item.priceValue }} 元</span>
        </div>
        <div class="cell markup-cell">
          <span class="ellipsis-text">{{ item.processMarkup }} 元</span>
        </div>
        <div class="cell requirement-cell">
          <dict-tag :options="sys_service_requirement" :value="item.serviceRequirement" />
        </div>
        <div class="cell total-cell">
          <span class="ellipsis-text">{{ calculateTotalPrice(item) }} 元</span>
        </div>
        <div class="cell action-cell">
          <el-button 
            type="danger" 
            icon="Delete" 
            @click="handleDelete(item.clothId, item.clothInfo.title)" 
            :disabled="disabled"
          />
        </div>

        <!-- 下半部分 -->
        <div class="cell remark-cell"
          :class="{ 'hidden': !item.remark && (!item.clothingFlawArr || item.clothingFlawArr.length == 0) && (!item.estimateArr || item.estimateArr.length == 0) }">
          <!-- 瑕疵标签 -->
          <div class="tag-container">
            <el-tag v-for="tagId in item.clothingFlawArr" :key="tagId" type="danger" size="small" class="tag-item">
              <el-tooltip :content="getFlawById(tagId)" placement="top" :show-after="200" :enterable="false" effect="dark">
                <span class="tag-text">{{getFlawById(tagId)}}</span>
              </el-tooltip>
            </el-tag>
          </div>
          
          <span v-if="item.clothingFlawArr && item.clothingFlawArr.length > 0 && (item.estimateArr && item.estimateArr.length > 0 || item.remark)" class="separator">||</span>
          
          <!-- 预估标签 -->
          <div class="tag-container">
            <el-tag v-for="tagId in item.estimateArr" :key="tagId" type="primary" size="small" class="tag-item">
              <el-tooltip :content="getEstimateById(tagId)" placement="top" :show-after="200" :enterable="false" effect="dark">
                <span class="tag-text">{{getEstimateById(tagId)}}</span>
              </el-tooltip>
            </el-tag>
          </div>
          
          <span v-if="item.estimateArr && item.estimateArr.length > 0 && item.remark" class="separator">||</span>
          
          <!-- 备注 -->
          <div class="remark-container" v-if="item.remark">
            <el-tooltip :content="item.remark" placement="top" :show-after="200" :enterable="false" effect="dark">
              <span class="remark-text">{{ item.remark }}</span>
            </el-tooltip>
          </div>
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
import useTagsStore from '@/store/modules/tags';

// 定义 Props
const props = defineProps({
  tableData: {
    type: Array,
    required: true,
  },
  disabled: {
    type: Boolean,
    default: false
  }
});

const { proxy } = getCurrentInstance();
const { sys_service_requirement } = proxy.useDict("sys_service_requirement");

const setSelectedCloth = inject('setSelectedCloth');

// 定义 Emits
const emit = defineEmits(['delete']);

// 使用标签store
const tagsStore = useTagsStore();

// 计算小计
const calculateTotalPrice = (cloth) => {
  let priceValue = cloth.priceValue || 0;
  if (cloth.serviceRequirement == '001') {
    priceValue *= 2;
  } else if (cloth.serviceRequirement == '002') {
    priceValue *= 1.5;
  }
  if (cloth.processMarkup) {
    priceValue += cloth.processMarkup;
  }
  return parseFloat(Math.floor(priceValue * 100) / 100);
};

// 删除操作
const handleDelete = (id, name) => {
  emit('delete', id, name);
};

// 根据ID获取瑕疵标签名称
const getFlawById = (tagId) => {
  const tag = tagsStore.flawList.find(item => item.tagId == tagId);
  return tag ? tag.tagName : '';
};

// 根据ID获取预估标签名称
const getEstimateById = (tagId) => {
  const tag = tagsStore.estimateList.find(item => item.tagId == tagId);
  return tag ? tag.tagName : '';
};

// 处理行点击事件
const handleRowClick = (row) => {
  setSelectedCloth(row);
};

onMounted(async () => {
  // 确保标签数据已加载
  await tagsStore.initTags();
});
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
  grid-template-columns: repeat(5, 1fr) 80px;
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
  grid-template-columns: repeat(5, 1fr) 80px;
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
  min-width: 0;
  overflow: hidden;
}

.action-cell {
  grid-row: span 2;
  display: flex;
  align-items: center;
  justify-content: center;
  border-bottom: none !important;
  padding: 0;
}

.action-cell .el-button {
  transition: all 0.3s;
  width: 32px;
  height: 32px;
  padding: 6px;
}

.action-cell .el-button:hover {
  transform: scale(1.1);
  background-color: var(--el-color-danger-light-5);
}

.remark-cell {
  grid-column: span 5;
  display: flex;
  align-items: center;
  justify-content: flex-start;
  padding: 8px 16px;
  gap: 4px;
  flex-wrap: wrap;
  font-size: small;
  color: var(--el-text-color-secondary);
  min-width: 0;
  min-height: 40px;
  overflow: hidden;
  text-overflow: ellipsis;
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

.title-cell {
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  min-width: 0;
  max-width: 100%;
}

.ellipsis-text {
  max-width: 100%;
  width: 100%;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-size: 14px;
  line-height: 1.4;
  transition: color 0.2s;
}

.ellipsis-text:hover {
  color: var(--el-color-primary);
}

.price-cell,
.markup-cell,
.total-cell {
  font-family: var(--el-font-family);
}

.price-cell .ellipsis-text {
  color: var(--el-text-color-secondary);
}

.markup-cell .ellipsis-text {
  color: var(--el-text-color-regular);
}

.total-cell .ellipsis-text {
  color: var(--el-color-danger);
  font-weight: 600;
}

.requirement-cell {
  /* 标签需要正确对齐 */
  padding: 12px 4px;
}

.tag-item {
  margin: 2px 4px;
  max-width: 100px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-size: 11px;
  padding: 0 6px;
  border-radius: 4px;
}

.tag-item:hover {
  transform: translateY(-1px);
  transition: all 0.2s;
}

.separator {
  color: var(--el-text-color-secondary);
  margin: 0 4px;
}

.remark-text {
  color: var(--el-text-color-regular);
  font-style: italic;
}

/* 表头单元格样式 */
.header-cell {
  padding: 12px 8px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-weight: 600;
  color: var(--el-text-color-primary);
  display: flex;
  align-items: center;
  justify-content: center;
}

/* 添加附加样式 */
.tag-container {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.tag-text {
  max-width: 80px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  display: inline-block;
}

.remark-container {
  flex: 1;
  min-width: 0;
  max-width: 100%;
}
</style>