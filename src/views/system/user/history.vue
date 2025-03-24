<template>
    <!-- 统计信息卡片 -->
    <div class="history-stats">
        <div class="stat-card">
            <div class="stat-icon">
                <el-icon>
                    <Money />
                </el-icon>
            </div>
            <div class="stat-content">
                <div class="stat-value">{{ totalAmount }}元</div>
                <div class="stat-label">总消费金额</div>
            </div>
        </div>
        <div class="stat-card">
            <div class="stat-icon">
                <el-icon>
                    <ShoppingCart />
                </el-icon>
            </div>
            <div class="stat-content">
                <div class="stat-value">{{ avgPrice }}元</div>
                <div class="stat-label">平均客单价</div>
            </div>
        </div>
    </div>

    <!-- 搜索表单 -->
    <el-form :inline="true" class="search-form">
        <el-form-item label="衣物名称">
            <el-input v-model="queryParams.clothingName" placeholder="请输入衣物名称" clearable @keyup.enter="handleQuery"
                prefix-icon="Search" />
        </el-form-item>
        <el-form-item label="消费日期">
            <el-date-picker v-model="dateRange" value-format="YYYY-MM-DD" type="daterange" range-separator="-"
                start-placeholder="开始日期" end-placeholder="结束日期" style="width: 260px" />
        </el-form-item>
        <el-form-item>
            <el-button class="search-btn" type="primary" @click="handleQuery">
                <el-icon>
                    <Search />
                </el-icon>
                <span>搜索</span>
            </el-button>
            <el-button class="search-btn" @click="resetQuery">
                <el-icon>
                    <Refresh />
                </el-icon>
                <span>重置</span>
            </el-button>
        </el-form-item>
    </el-form>

    <!-- 渲染订单抖索结果列表 -->
    <div class="search-result-list">

        <div v-if="orderList.length === 0" class="no-result">
            <el-empty description="暂无数据" />
        </div>
        <div v-slide-in v-else class="result-item" v-for="order in orderList" :key="order.orderId">
            <div class="result-item-info">
                <span>订单编码: {{ order.orderNumber }}</span>
                <span>订单日期: {{ formatTime(order.createTime) }}</span>
                <span style="display: flex; align-items: center; gap: .5rem;">消费金额:
                    <span style="color: red;font-weight: bold; align-items: center;">
                        {{ order.mount }}
                    </span>
                </span>
            </div>
            <!-- 订单包含的衣物列表 -->
            <el-table v-if="order.clothList && order.clothList.length > 0" class="cloths-table" :data="order.clothList"
                :loading="order.loading" row-key="clothingId"
                @selection-change="selectedItems => handleClothSelectionChange(selectedItems, order)"
                ref="clothsTableRef" border="dash" :max-height="500" stripe>
                <el-table-column label="衣物编码" align="center" prop="clothingColor" width="110">
                    <template #default="scope">
                        {{ scope.row.hangClothCode }}
                    </template>
                </el-table-column>
                <el-table-column label="衣物" align="center">
                    <template #default="scope">
                        {{ scope.row.clothInfo.clothingName }}

                    </template>
                </el-table-column>
                <el-table-column label="衣物品牌" align="center" prop="clothingBrand">
                    <template #default="scope">
                        <el-tag v-if="scope.row.clothingBrand" type="primary">
                            {{brandList.find(item => item.tagId == scope.row.clothingBrand).tagName}}
                        </el-tag>
                    </template>
                </el-table-column>
                <el-table-column label="衣物颜色" align="center">
                    <template #default="scope">
                        <el-tag v-if="scope.row.clothingColor" type="primary">
                            {{scope.row.clothingColor ? colorList.find(item => item.tagId ==
                                scope.row.clothingColor).tagName : ''}}
                        </el-tag>
                    </template>
                </el-table-column>
                <el-table-column label="服务类型" align="center">
                    <template #default="scope">
                        <span class="service-type">
                            <dict-tag :options="sys_service_type" :value="scope.row.serviceType" />
                            -
                            <dict-tag :options="sys_service_requirement" :value="scope.row.serviceRequirement" />
                        </span>
                    </template>
                </el-table-column>
                <el-table-column label="洗护价格" align="center" prop="priceValue" />
                <el-table-column label="备注" align="center" prop="remark" />
            </el-table>

        </div>
        <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
            v-model:limit="queryParams.pageSize" @pagination="getList" />
    </div>


</template>

<script setup name="History">
import { selectListHistory, getCountByUserId } from "@/api/system/orders";
import { getTotalAmountAndAvgConsume } from "@/api/system/payment";
import { listCloths } from "@/api/system/cloths";
import { listTagsNoLimit } from "@/api/system/tags";
import { getPrice } from "@/api/system/price";
import { Money, ShoppingCart, Search, Refresh } from '@element-plus/icons-vue';
import { formatTime } from '@/utils/ruoyi';
import vSlideIn from "@/vSlideIn";

const props = defineProps({

    userId: {
        type: Object,
        required: true,
    },
});

const { proxy } = getCurrentInstance();
const {
    sys_service_type,
    sys_service_requirement,
} =
    proxy.useDict(
        "sys_service_type",
        "sys_service_requirement"
    );

const show = ref(false);
const loading = ref(true);

const orderList = ref([]);
const dateRange = ref([]);
const totalAmount = ref(0);
const avgPrice = ref(0);

const colorList = ref([]);
const flawList = ref([]);
const estimateList = ref([]);
const brandList = ref([]);
const total = ref(0);

const queryParams = ref({
    pageNum: 1,
    pageSize: 10,
});


/** 搜索按钮操作 */
function handleQuery() {
    getList();
}
/** 重置按钮操作 */
function resetQuery() {
    dateRange.value = [];
    queryParams.value = {
        pageNum: 1,
        pageSize: 10,
    };
    getList();
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

function close() {
    queryParams.value = { pageNum: 1, pageSize: 10 };
    show.value = false;
    props.toggle();
}

// 提取出价格计算逻辑
async function calculatePrice(item) {
    if (item.priceId) {
        const { data: priceItem } = await getPrice(item.priceId);
        return priceItem ? priceItem.priceValue : 0;
    } else {
        return item.clothList.reduce((acc, cur) => {
            let priceValue = cur.priceValue;
            if (cur.serviceRequirement === '001') {
                priceValue *= 2;
            } else if (cur.serviceRequirement === '002') {
                priceValue *= 1.5;
            }
            return acc + priceValue + cur.processMarkup;
        }, 0);
    }
}

async function getList() {
    queryParams.value.userId = props.userId;
    // if (queryParams.value.pickupCode === '' && queryParams.value.phonenumber === '' && queryParams.value.orderNumber === '') {
    //     return;
    // }
    if (dateRange.value && dateRange.value.length === 2) {
        queryParams.value.startTime = dateRange.value[0];
        queryParams.value.endTime = dateRange.value[1];
    } else {
        queryParams.value.startTime = null;
        queryParams.value.endTime = null;
    }
    loading.value = true;
    await selectListHistory(queryParams.value).then(response => {
        orderList.value = response.rows;
        total.value = response.total;
    }).finally(() => {
        loading.value = false;
    });
    // 遍历计算订单价格
    for (const item of orderList.value) {
        item.loading = true;

        item.clothList = await listCloths({ orderId: item.orderId });
        item.loading = false;

        let price = 0;

        // 优先处理 `adjust` 的情况
        if (item.adjust) {
            if (item.adjust.adjustTotal) {
                item.mount = item.adjust.adjustTotal;
            } else {
                price = await calculatePrice(item);
                price +=
                    Number(item.adjust.adjustValueAdd || 0) -
                    Number(item.adjust.adjustValueSub || 0);
                item.mount = price > 0 ? price : 0;
            }
        } else {
            // 没有 `adjust` 的情况下计算价格
            price = await calculatePrice(item);
            item.mount = price > 0 ? price : 0;
        }
    }

    await getTotalAmountAndAvgConsume(props.userId).then(response => {
        totalAmount.value = response;
    });

    const orderCount = await getCountByUserId(props.userId);
    if (orderCount > 0) {
        avgPrice.value = (totalAmount.value / orderCount).toFixed(2);
    }

}

// 处理衣物选择变化
function handleClothSelectionChange(selectedItems, order) {
    // 可以在这里处理选中的衣物项目
    console.log('选中的衣物:', selectedItems);
    console.log('所属订单:', order.orderNumber);
}

onMounted(async () => {
    await initList();
    getList();
})
</script>

<style scoped>
/* 响应式设计 */
@media screen and (max-width: 768px) {
    .history-stats {
        flex-direction: column;
        gap: 12px;
    }

    .search-form {
        padding: 12px;
    }

    .result-item-info {
        flex-direction: column;
        align-items: flex-start;
        gap: 8px;

    }

    .cloths-table {
        width: 100%;
        overflow-x: auto;
    }
}

/* 统计卡片样式 */
.history-stats {
    display: flex;
    gap: 20px;
    margin-bottom: 24px;
}

.stat-card {
    flex: 1;
    display: flex;
    align-items: center;
    padding: 1rem;
    background-color: var(--el-fill-color-light);
    border-radius: 8px;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.05);
    transition: all 0.3s;
}

.stat-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.stat-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 48px;
    height: 48px;
    border-radius: 12px;
    background-color: var(--el-color-primary-light-9);
    margin-right: 1rem;
}

.stat-icon .el-icon {
    font-size: 24px;
    color: var(--el-color-primary);
}

.stat-content {
    display: flex;
    flex-direction: column;
}

.stat-value {
    font-size: 20px;
    font-weight: 600;
    color: var(--el-text-color-primary);
    line-height: 1.2;
}

.stat-label {
    font-size: 14px;
    color: var(--el-text-color-secondary);
    margin-top: 4px;
}

/* 搜索表单样式 */
.search-form {
    background-color: var(--el-fill-color-blank);
    padding: 1rem;
    border-radius: 8px;
    margin-bottom: 24px;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.05);
}

.search-btn {
    transition: all 0.3s;
}

.search-btn:hover {
    transform: translateY(-2px);
    box-shadow: var(--el-box-shadow-light);
}

/* 结果列表样式 */
.search-result-list {
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.result-item {
    display: flex;
    flex-direction: column;
    background-color: var(--el-fill-color-blank);
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.05);
    transition: all 0.3s;
}

.result-item:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.result-item-info {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background-color: var(--el-color-primary-light-9);
    border-bottom: 1px solid var(--el-border-color-lighter);
}

.result-item-info span {
    display: flex;
    align-items: center;
    color: var(--el-text-color-primary);
    font-weight: 500;
}

/* 自定义暗黑模式下的颜色 */
:root.dark .result-item-info {
    --el-color-primary-light-9: #1d2c40;
}

.cloths-table {
    margin: 0;
    border-radius: 0 0 8px 8px;
}

.cloths-table :deep(th) {
    background-color: var(--el-fill-color-light);
    color: var(--el-text-color-primary);
    font-weight: 600;
}

.cloths-table :deep(.el-table__row) {
    transition: all 0.2s;
}

.cloths-table :deep(.el-table__row:hover) {
    background-color: var(--el-color-primary-light-9) !important;
}

.cloths-table :deep(.el-table__cell) {
    padding: 8px 0;
}

.cloths-table :deep(.el-table--striped .el-table__body tr.el-table__row--striped td) {
    background-color: var(--el-fill-color-light);
}

.service-type {
    display: flex;
    align-items: center;
    gap: 4px;
}

/* 分页样式 */
.el-pagination {
    margin-top: 20px;
    justify-content: flex-end;
}

/* 空状态样式 */
.no-result {
    padding: 40px 0;
    text-align: center;
}
</style>