<template>
    <el-dialog v-model="show" width="80%" :align-center="true" @closed="close">
        <el-table :data="data">
            <el-table-column label="衣物" align="center">
                <template #default="scope">
                    {{ scope.row.clothInfo.clothingName }}
                </template>
            </el-table-column>
            <el-table-column label="衣物编码" align="center" prop="clothingColor">
                <template #default="scope">
                    {{ scope.row.hangClothCode }}
                </template>
            </el-table-column>
            <el-table-column label="服务日期" align="center" prop="createTime" >
                <template #default="scope">
                    {{ formatTime(scope.row.createTime) }}
                </template>
            </el-table-column>
            <el-table-column label="衣物颜色" align="center" prop="clothingColor">
                <template #default="scope">
                    <el-tag v-if="scope.row.clothingColor" type="success">
                        {{ scope.row.clothingColor ? colorList.find(item => {
                            return item.tagId ==
                                scope.row.clothingColor
                        }).tagName : '-' }}
                    </el-tag>
                </template>
            </el-table-column>
            <el-table-column label="服务类型" align="center" prop="serviceType">
                <template #default="scope">
                    <dict-tag :options="sys_service_type" :value="scope.row.serviceType" />
                </template>
            </el-table-column>
            <el-table-column label="洗后预估" align="center" prop="estimate">
                <template #default="scope">
                    <el-tag v-for="tagId in scope.row.estimateArr" :key="item" type="primary">
                        {{ estimateList.find(item => item.tagId == tagId).tagName }}
                    </el-tag>
                </template>
            </el-table-column>
            <el-table-column label="衣物瑕疵" align="center" prop="clothingFlaw">
                <template #default="scope">
                    <el-tag v-for="tagId in scope.row.clothingFlawArr" :key="item" type="danger">
                        {{ flawList.find(item => item.tagId == tagId).tagName }}
                    </el-tag>
                </template>
            </el-table-column>
            <el-table-column label="工艺加价" align="center" prop="processMarkup">
                <template #default="scope">
                    <span style="color: red">
                        {{ scope.row.processMarkup }}
                    </span>
                </template>
            </el-table-column>
            <el-table-column label="服务要求" align="center" prop="serviceRequirement">
                <template #default="scope">
                    <dict-tag :options="sys_service_requirement" :value="scope.row.serviceRequirement" />
                </template>
            </el-table-column>
            <el-table-column label="价格" align="center" prop="priceValue">
                <template #default="scope">
                    <span style="color: red;">
                        {{ scope.row.priceValue }}
                    </span>
                </template>
            </el-table-column>
            <el-table-column label="补充信息" align="center" prop="hangRemark" />
        </el-table>
        <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
            v-model:limit="queryParams.pageSize" @pagination="getList" />
    </el-dialog>
</template>

<script setup name="History">
import { listHistoryCloths } from "@/api/system/cloths";
import { listTagsNoLimit } from "@/api/system/tags";
import { onMounted, ref } from 'vue';

const props = defineProps({
    visible: {
        type: Boolean,
        required: true,
        default: false,
    },
    userId: {
        type: Object,
        required: true,
    },
    toggle: {
        type: Function,
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

const data = ref([]);
const show = ref(false);

const colorList = ref([]);
const flawList = ref([]);
const estimateList = ref([]);
const brandList = ref([]);
const total = ref(0);

const queryParams = ref({
    pageNum: 1,
    pageSize: 10,
});
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

function getList() {
    queryParams.value.userId = props.userId;
    listHistoryCloths(queryParams.value).then(res => {
        data.value = res.rows;
        data.value.map(item => {
            item.clothingFlawArr = item.clothingFlaw ? item.clothingFlaw.split(',') : [];
            item.estimateArr = item.estimate ? item.estimate.split(',') : [];
        });
        total.value = res.total;
        show.value = true;
    })
}

onMounted(async () => {
    if (props.visible) {
        await initList();
        getList();
    }
})
</script>

<style scoped lang="scss">
.el-pagination {
    right: 1rem !important;
}
</style>