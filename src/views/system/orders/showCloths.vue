<template>
    <div class="app-container">
        <el-table v-loading="loading" :data="clothsList" @selection-change="handleSelectionChange">
            <el-table-column type="selection" width="55" align="center" />
            <el-table-column label="衣物" align="center">
                <template #default="scope">
                    {{ scope.row.clothInfo.clothingName }}
                    {{ scope.row.clothingColor ? '-' + colorList.find(item => item.tagId ==
                        scope.row.clothingColor).tagName : '' }}
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
            <el-table-column label="工艺加价" align="center" prop="processMarkup" />
            <el-table-column label="衣物瑕疵" align="center" prop="clothingFlaw">
                <template #default="scope">
                    <el-tag v-for="tagId in scope.row.clothingFlaw ? scope.row.clothingFlaw.split(',') : []" :key="item"
                        type="danger">
                        {{ flawList.find(item => item.tagId == tagId).tagName }}
                    </el-tag>
                </template>
            </el-table-column>
            <el-table-column label="洗后预估" align="center" prop="estimate">
                <template #default="scope">
                    <el-tag v-for="tagId in scope.row.estimate ? scope.row.estimate.split(',') : []" :key="item"
                        type="primary">
                        {{ estimateList.find(item => item.tagId == tagId).tagName }}
                    </el-tag>
                </template>
            </el-table-column>
            <el-table-column label="衣物品牌" align="center" prop="clothingBrand">
                <template #default="scope">
                    <el-tag v-if="scope.row.clothingBrand" type="primary">
                        {{ brandList.find(item => item.tagId == scope.row.clothingBrand).tagName }}
                    </el-tag>
                </template>
            </el-table-column>
            <el-table-column label="状态" align="center" prop="clothingStatus">
                <template #default="scope">
                    <dict-tag :options="sys_clothing_status" :value="scope.row.clothingStatus" />
                </template>
            </el-table-column>
            <el-table-column label="取回方式" align="center" prop="deliveryMode" />
            <el-table-column label="取回时间" align="center" prop="deliveredTime" />
            <!-- <el-table-column label="上挂位置编码" align="center" prop="hangLocationCode" />
            <el-table-column label="上挂衣物编码" align="center" prop="hangClothCode" />
            <el-table-column label="上挂描述信息" align="center" prop="hangRemark" /> -->
            <el-table-column label="操作" align="center" :width="280" class-name="small-padding fixed-width">
                <template #default="scope">
                    <el-button link type="primary" icon="Picture" @click="handleShowPicture(scope.row, true)"
                        v-hasPermi="['system:cloths:edit']">洗前图像</el-button>
                    <el-button link type="primary" icon="Picture" @click="handleShowPicture(scope.row, false)"
                        v-hasPermi="['system:cloths:edit']">洗后图像</el-button>
                    <el-button link type="primary" icon="Top" @click="handleShowHangUp(scope.row)"
                        v-if="scope.row.clothingStatus == '01'" v-hasPermi="['system:cloths:remove']">上挂</el-button>
                </template>
            </el-table-column>
        </el-table>

        <!-- <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
            v-model:limit="queryParams.pageSize" @pagination="getList" /> -->
        <div class="footer">
            <el-button type="warning" :disabled="afterSaleDisabled" @click="afterSale">售后</el-button>
            <el-button type="danger" :disabled="compensationDisabled" @click="compensate">赔偿</el-button>
        </div>

        <!-- 展示照片 -->
        <el-dialog title="照片" v-model="showPicture" width="400px" append-to-body>
            <div class="img-container">
                <el-image class="img-item" :preview-src-list="pictureList" :src="item"
                    v-for="(item, index) in pictureList" :key="index" fit="contain" />
            </div>
        </el-dialog>
        <!-- 添加或修改订单包含的衣物清单对话框 -->
        <el-dialog :title="title" v-model="showHangUp" width="400px" :show-close="false" append-to-body
            :before-close="closeHangUpDialog">
            <el-form ref="hangUpRef" :model="hangForm" :rules="hangRules" label-width="80px">
                <el-form-item label="衣物编码" prop="clothingNumber">
                    <el-input v-model="hangForm.clothingNumber" placeholder="请输入衣物编码" />
                </el-form-item>
                <el-form-item label="衣物信息">
                    {{ currentCloth.clothInfo.clothingName }}
                    {{ currentCloth.clothingColor ? '-' +
                        colorList.find(item => item.tagId ==
                            currentCloth.clothingColor).tagName : '' }}
                    {{ currentCloth.clothingBrand ? '-' +
                        brandList.find(item => item.tagId ==
                            currentCloth.clothingBrand).tagName : '' }}
                </el-form-item>

                <el-form-item label="衣挂位置" prop="hangLocationCode">
                    <el-input v-model="hangForm.hangLocationCode" placeholder="请输入上挂位置编码" />
                </el-form-item>
                <el-form-item label="衣挂编号" prop="hangClothCode">
                    <el-input v-model="hangForm.hangClothCode" placeholder="请输入上挂衣物编码" />
                </el-form-item>
                <el-form-item label="备注信息" prop="hangRemark">
                    <el-input type="textarea" v-model="hangForm.hangRemark" placeholder="请输入上挂描述信息" />
                </el-form-item>
            </el-form>
            <template #footer>
                <div class="hangup-footer">
                    <el-button type="primary" @click="hangUp">确认上挂</el-button>
                </div>
            </template>
        </el-dialog>
    </div>
</template>

<script setup name="Cloths">
import { listCloths, addCloths, updateCloths } from "@/api/system/cloths";
import { listTags } from "@/api/system/tags";
import { ref } from "vue";
import { getCloths } from "@/api/system/cloths";

const props = defineProps({
    orderId: {
        type: Number,
        required: true,
        default: 0
    }
});

const { proxy } = getCurrentInstance();

const { sys_cloth_cate, sys_clothing_status, sys_service_type, sys_service_requirement, } =
    proxy.useDict("sys_cloth_cate", "sys_clothing_status", "sys_service_type", "sys_service_requirement");

const selectionList = ref([]);
const clothsList = ref([]);
const pictureList = ref([]);
const currentCloth = ref({});
const showPicture = ref(false);
const open = ref(false);
const loading = ref(true);
const showHangUp = ref(false);
const total = ref(0);
const title = ref("");
const colorList = ref([]);
const flawList = ref([]);
const estimateList = ref([]);
const brandList = ref([]);

const afterSaleDisabled = ref(true);
const compensationDisabled = ref(true);
const baseUrl = import.meta.env.VITE_APP_BASE_API;
const pictureUrl = ref(baseUrl + "/system/cloths/download/");
const data = reactive({
    form: {},
    hangForm: {},
    queryParams: {
        pageNum: 1,
        pageSize: 10,
        clothingId: null,
        clothingCategory: null,
        clothingStyle: null,
        clothingColor: null,
        clothingFlaw: null,
        estimate: null,
        clothingBrand: null,
        serviceType: null,
        serviceRequirement: null,
        beforePics: null,
        afterPics: null,
        notes: null,
        processMarkup: null,
        priceValue: null,
        hangLocationCode: null,
        hangClothCode: null,
        hangRemark: null,
    },
    rules: {
        clothingId: [
            { required: true, message: "衣物唯一标识ID不能为空", trigger: "blur" }
        ],
        clothingCategory: [
            { required: true, message: "衣物品类不能为空", trigger: "blur" }
        ],
        clothingStyle: [
            { required: true, message: "衣物类别不能为空", trigger: "blur" }
        ],
        serviceType: [
            { required: true, message: "服务类型不能为空", trigger: "change" }
        ],
        priceValue: [
            { required: true, message: "洗护价格不能为空", trigger: "blur" }
        ],
        createTime: [
            { required: true, message: "收取时间不能为空", trigger: "blur" }
        ]
    },
    hangRules: {
        clothingNumber: [
            { required: true, message: "衣物编码不能为空", trigger: "change" }
        ],
        hangLocationCode: [
            { required: true, message: "衣挂位置不能为空", trigger: "blur" }
        ],
        hangClothCode: [
            { required: true, message: "衣挂编号不能为空", trigger: "blur" }
        ]
    }
});

const { queryParams, form, hangForm, hangRules } = toRefs(data);

/** 查询订单包含的衣物清单列表 */
function getList() {
    // 判断是否有订单id
    if (props.orderId == 0) {
        return;
    }
    loading.value = true;

    listCloths({ orderClothId: props.orderId }).then(response => {
        clothsList.value = response.rows;
        total.value = response.total;
        loading.value = false;
    });
}

/* 初始化列表数据 */
async function initList() {
    const promises = [];

    // 获取颜色列表
    if (colorList.value.length === 0) {
        const colorPromise = listTags({ tagOrder: '003' }).then(response => {
            colorList.value = response.rows;
        });
        promises.push(colorPromise);
    }

    // 获取瑕疵列表
    if (flawList.value.length === 0) {
        const flawPromise = listTags({ tagOrder: '001' }).then(response => {
            flawList.value = response.rows;
        });
        promises.push(flawPromise);
    }

    // 获取预估列表
    if (estimateList.value.length === 0) {
        const estimatePromise = listTags({ tagOrder: '002' }).then(response => {
            estimateList.value = response.rows;
        });
        promises.push(estimatePromise);
    }

    // 获取品牌列表
    if (brandList.value.length === 0) {
        const brandPromise = listTags({ tagOrder: '004' }).then(response => {
            brandList.value = response.rows;
        });
        promises.push(brandPromise);
    }

    // 等待所有异步操作完成防止衣物列表数据加载完后这里的数据没有准备好而出错
    await Promise.all(promises);
}
// 取消按钮
function cancel() {
    open.value = false;
    reset();
}

// 表单重置
function reset() {
    currentCloth.value = {}
    form.value = {
        orderClothId: null,
        clothingId: null,
        clothingCategory: null,
        clothingStyle: null,
        clothingColor: null,
        clothingFlaw: null,
        estimate: null,
        clothingBrand: null,
        serviceType: null,
        serviceRequirement: null,
        beforePics: null,
        afterPics: null,
        notes: null,
        processMarkup: null,
        priceValue: null,
        hangLocationCode: null,
        hangClothCode: null,
        hangRemark: null,
        createTime: null
    };
    proxy.resetForm("clothsRef");
}

// 多选框选中数据
function handleSelectionChange(selection) {
    selectionList.value = selection;
    if (selectionList.value.find(item => item.clothingStatus == '00')) {
        afterSaleDisabled.value = false;
    } else {
        afterSaleDisabled.value = true;
    }

    if (selectionList.value.find(item => item.clothingStatus == '02')) {
        compensationDisabled.value = false;
    } else {
        compensationDisabled.value = true;
    }
}



/** 提交按钮 */
function submitForm() {
    proxy.$refs["clothsRef"].validate(valid => {
        if (valid) {
            if (form.value.orderClothId != null) {
                updateCloths(form.value).then(response => {
                    proxy.$modal.msgSuccess("修改成功");
                    open.value = false;
                    getList();
                });
            } else {
                addCloths(form.value).then(response => {
                    proxy.$modal.msgSuccess("新增成功");
                    open.value = false;
                    getList();
                });
            }
        }
    });
}

/* 显示上挂 */
function handleShowHangUp(row) {
    showHangUp.value = true;
    currentCloth.value = row;
    console.log(row)
}

/* 赔偿 */
function compensate(row) {
}

/* 上挂 */
function hangUp() {
    if (currentCloth.value) {
        //校验上挂表单内容
        proxy.$refs["hangUpRef"].validate(valid => {
            if (valid) {
                console.log(currentCloth.value)
                console.log(hangForm.value)
            }
        });
    }
}

/* 关闭上挂弹窗 */
function closeHangUpDialog(done) {
    hangForm.value = {
        clothingNumber: null,
        hangLocationCode: null,
        hangClothCode: null,
        hangRemark: null
    };
    done();
}

/* 获取图片列表id */
function handleShowPicture(row, flag) {
    showPicture.value = true;
    getCloths(row.clothId).then(response => {
        if (flag) {
            pictureList.value = response.data.beforePics ?
                response.data.beforePics.split(',').map(item => pictureUrl.value + item) : [];
        } else {
            pictureList.value = response.data.afterPics ?
                response.data.afterPics.split(',').map(item => pictureUrl.value + item) : [];
        }
        console.log(pictureList.value)
    });

}

onMounted(async () => {
    await initList();  // 确保 initList 完成
    getList();         // 在 initList 完成后调用
});
</script>

<style scoped>
.service-type {
    display: flex;
    align-items: center;
    gap: .2rem
}

.footer {
    display: flex;
    justify-content: flex-end;
    margin-top: 1rem;
}

.hangup-footer {
    display: flex;
    justify-content: center;
    align-items: center;
}

.img-container {
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-start;
    align-items: center;
    gap: 1rem;
}

.img-item {
    flex: 1 1 calc(33.333% - 1rem);
    /* 每行 3 个元素 */
    box-sizing: border-box;
}
</style>