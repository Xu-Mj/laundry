<template>
    <el-dialog v-model="dialogVisible" title="选择预定义分类" width="60%" align-center :close-on-click-modal="false"
        class="category-dialog" @close="handleClose">
        <div class="predefined-categories">
            <el-checkbox-group v-model="selectedCategories" class="category-group">
                <el-checkbox v-for="category in predefinedCategories" :key="category.categoryName" :label="category"
                    class="category-checkbox">
                    <div class="category-item">
                        <div class="category-info">
                            <span class="category-name">{{ category.categoryName }}</span>
                            <div class="style-list">
                                <el-tag v-for="style in category.styles" :key="style" size="small" class="style-tag"
                                    effect="plain">
                                    {{ style }}
                                </el-tag>
                            </div>
                        </div>
                    </div>
                </el-checkbox>
            </el-checkbox-group>
        </div>
        <template #footer>
            <div class="dialog-footer">
                <div class="selection-info">
                    已选择 {{ selectedCategories.length }} 个分类
                </div>
                <div class="footer-buttons">
                    <el-button @click="handleCancel">取消</el-button>
                    <el-button type="primary" @click="handleConfirm">
                        确认添加
                    </el-button>
                </div>
            </div>
        </template>
    </el-dialog>
</template>

<script setup>
import { addCategory } from "@/api/system/clothingCategory";
import { addStyle } from "@/api/system/clothingStyle";

const props = defineProps({
    modelValue: {
        type: Boolean,
        required: true
    }
});

const { proxy } = getCurrentInstance();

// 预定义的分类数据
const predefinedCategories = ref([
    {
        categoryName: '上衣',
        styles: ['T恤', '衬衫', '卫衣', '毛衣', '外套', '羽绒服', '西装']
    },
    {
        categoryName: '下装',
        styles: ['长裤', '短裤', '牛仔裤', '休闲裤', '运动裤', '裙子']
    },
    {
        categoryName: '内衣',
        styles: ['文胸', '内裤', '袜子', '睡衣', '家居服']
    },
    {
        categoryName: '配饰',
        styles: ['帽子', '围巾', '手套', '腰带', '领带']
    },
    {
        categoryName: '鞋类',
        styles: ['运动鞋', '皮鞋', '休闲鞋', '靴子', '凉鞋']
    }
]);

const emit = defineEmits(['update:modelValue', 'success', 'cancel']);

const dialogVisible = ref(props.modelValue);
const selectedCategories = ref([]);

// 监听 modelValue 的变化
watch(() => props.modelValue, (newVal) => {
    dialogVisible.value = newVal;
});

// 监听 dialogVisible 的变化
watch(() => dialogVisible.value, (newVal) => {
    emit('update:modelValue', newVal);
});

// 处理确认按钮点击
async function handleConfirm() {
    if (selectedCategories.value.length === 0) {
        proxy.notify.warning('请至少选择一个分类');
        return;
    }

    try {
        // 创建选中的分类和样式
        const promises = selectedCategories.value.map(category => {
            // 创建分类
            return addCategory({ categoryName: category.categoryName }).then(categoryRes => {
                // 创建该分类下的所有样式
                const stylePromises = category.styles.map(styleName => {
                    return addStyle({
                        categoryId: categoryRes.categoryId,
                        styleName: styleName,
                        styleCode: "",
                        orderNum: 0
                    });
                });
                return Promise.all(stylePromises);
            });
        });

        await Promise.all(promises);
        proxy.notify.success("分类添加成功");
        emit('success');
        handleClose();
    } catch (error) {
        proxy.notify.error("添加分类失败：" + error);
    }
}

// 处理取消按钮点击
function handleCancel() {
    emit('cancel');
    handleClose();
}

// 处理关闭对话框
function handleClose() {
    selectedCategories.value = [];
    dialogVisible.value = false;
}
</script>

<style scoped>
.category-dialog {
    :deep(.el-dialog__body) {
        padding: 0;
    }

    :deep(.el-dialog__header) {
        margin: 0;
        padding: 24px 32px;
        border-bottom: 1px solid var(--el-border-color-lighter);
    }
}

.predefined-categories {
    padding: 32px;
    background-color: var(--el-bg-color);
}

.category-header {
    display: flex;
    padding: 0 0 16px 0;
    margin-bottom: 24px;
    border-bottom: 1px solid var(--el-border-color-lighter);
    font-weight: 500;
    color: var(--el-text-color-regular);
}

.header-title {
    width: 160px;
    font-size: 15px;
}

.header-styles {
    flex: 1;
    font-size: 15px;
}

.category-group {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 5rem;
    width: 100%;
}

.category-checkbox {
    transition: all 0.2s ease;
    width: 100%;
}

.category-checkbox:hover {
    transform: translateY(-1px);
}

.category-checkbox:deep(.el-checkbox__label) {
    width: 100%;
    padding: 0;
    display: block;
}

.category-checkbox:deep(.el-checkbox__input) {
    display: none;
}

.category-item {
    width: 100%;
    padding: 24px;
    border: 1px solid var(--el-border-color-lighter);
    border-radius: 12px;
    background-color: var(--el-bg-color);
    transition: all 0.2s ease;
    cursor: pointer;
}

.category-info {
    display: flex;
    flex-direction: column;
    gap: 16px;
}

.category-name {
    font-weight: 500;
    font-size: 16px;
    color: var(--el-text-color-primary);
}

.style-list {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
}

.style-tag {
    margin: 0;
    padding: 6px 12px;
    background-color: var(--el-fill-color-blank);
    border: 1px solid var(--el-border-color-lighter);
    border-radius: 6px;
    font-size: 13px;
    color: var(--el-text-color-regular);
    transition: all 0.2s ease;
}

.style-tag:hover {
    background-color: var(--el-fill-color-light);
    border-color: var(--el-border-color);
}

.dialog-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 24px 32px;
    border-top: 1px solid var(--el-border-color-lighter);
    background-color: var(--el-bg-color);
}

.selection-info {
    color: var(--el-text-color-regular);
    font-size: 14px;
}

.footer-buttons {
    display: flex;
    gap: 12px;
}

/* 选中状态的样式 */
.category-checkbox:deep(.el-checkbox__input.is-checked + .el-checkbox__label .category-item) {
    border-color: var(--el-color-primary);
    background-color: var(--el-color-primary-light-9);
    box-shadow: 0 4px 16px 0 rgba(0, 0, 0, 0.08);
}

/* 悬停状态的样式 */
.category-checkbox:hover:deep(.el-checkbox__input:not(.is-checked) + .el-checkbox__label .category-item) {
    border-color: var(--el-border-color);
    box-shadow: 0 4px 16px 0 rgba(0, 0, 0, 0.08);
}
</style>