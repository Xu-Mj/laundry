<template>
    <el-dialog title="付款" v-model="showClothesDialog" width="1366px" append-to-body lock-scroll modal
        :close-on-click-modal="false" @closed="close">
        <el-table v-loading="loading" :data="clothsList" @selection-change="handleSelectionChange">
            <el-table-column type="selection" width="55" align="center" />
            <el-table-column label="衣物" align="center">
                <template #default="scope">
                    {{ scope.row.clothInfo.clothingName }}
                    {{ scope.row.clothingColor ? '-' + colorList.find(item => item.tagId ==
                        scope.row.clothingColor).tagName : '' }}
                </template>
            </el-table-column>
            <el-table-column label="衣物编码" align="center" prop="clothingColor" width="150">
                <template #default="scope">
                    {{ scope.row.hangClothCode }}
                </template>
            </el-table-column>
            <el-table-column label="服务类型" :width="120" align="center">
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
            <el-table-column label="上挂位置" align="center">
                <template #default="scope">
                    {{ scope.row.hangLocationCode ? scope.row.hangerName + '-' + scope.row.hangerNumber : '' }}
                </template>
            </el-table-column>
            <el-table-column label="取回方式" align="center" prop="pickupMethod">
                <template #default="scope">
                    <dict-tag :options="sys_delivery_mode" :value="scope.row.pickupMethod" />
                </template>
            </el-table-column>
            <el-table-column label="取回时间" align="center" prop="pickupTime" width="180">
                <template #default="scope">
                    <span>{{ formatTime(scope.row.pickupTime, '{y}-{m}-{d}') }}</span>
                </template>
            </el-table-column>
            <el-table-column label="上挂备注" align="center" prop="hangRemark" />
            <el-table-column label="操作" align="center" fixed="right" :width="280"
                class-name="small-padding fixed-width">
                <template #default="scope">
                    <el-button link type="primary" icon="Picture"
                        :disabled="scope.row.beforePics == null || scope.row.beforePics.length == 0"
                        @click="handleShowPicture(scope.row, true)">洗前</el-button>
                    <el-button link type="primary" icon="Picture"
                        :disabled="scope.row.afterPics == null || scope.row.afterPics.length == 0"
                        @click="handleShowPicture(scope.row, false)">洗后</el-button>
                    <el-button link type="primary" icon="Top" @click="handleShowHangUp(scope.row)"
                        v-if="scope.row.clothingStatus == '01'">
                        上挂
                    </el-button>
                </template>
            </el-table-column>
        </el-table>

        <div class="footer">
            <!-- <el-button type="success" plain :disabled="pickupDisabled" @click="handlePickup">取走</el-button> -->
            <el-button type="warning" plain :disabled="afterSaleDisabled" @click="afterSale">售后</el-button>
            <el-button type="danger" plain :disabled="compensationDisabled" @click="handleCompensate">赔偿</el-button>
        </div>

        <!-- 展示照片 -->
        <el-dialog title="照片" v-model="showPicture" width="400px" append-to-body>
            <div class="img-container">
                <el-image class="img-item" :preview-src-list="pictureList" :src="item"
                    v-for="(item, index) in pictureList" :key="index" fit="contain" />
            </div>
        </el-dialog>

        <!-- 上挂对话框 -->
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

                <el-form-item label="衣挂位置" prop="hangLocationId">
                    <el-input v-model="hangForm.hangLocationId" placeholder="请输入上挂位置编码" />
                </el-form-item>
                <el-form-item label="衣挂编号" prop="hangerNumber">
                    <el-input v-model="hangForm.hangerNumber" placeholder="请输入上挂衣物编码" />
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

        <!-- 赔偿对话框 -->
        <el-dialog title="赔偿" v-model="showCompensationDialog" width="500px" append-to-body>
            <el-form ref="compensationRef" :model="compensationForm" :rules="compensationRules" label-width="80px">
                <el-form-item label="支出账目">
                    <el-input v-model="compensationForm.expTitle" />
                </el-form-item>
                <el-form-item label="对方账户">
                    <el-input v-model="compensationForm.recvAccountTitle" disabled />
                </el-form-item>
                <el-form-item label="赔偿金额">
                    <el-input-number controls-position="right" v-model="compensationForm.expAmount"
                        placeholder="请输入赔偿金额" />
                </el-form-item>
                <el-form-item label="备注信息">
                    <el-input type="textarea" v-model="compensationForm.remark" placeholder="请输入备注信息" />
                </el-form-item>
            </el-form>
            <template #footer>
                <div class="compensation-footer">
                    <el-button type="primary" @click="compensate">确认赔偿</el-button>
                </div>
            </template>
        </el-dialog>
    </el-dialog>
</template>

<script setup name="Cloths">
import { listCloths } from "@/api/system/cloths";
import { listTagsNoLimit } from "@/api/system/tags";
import { ref } from "vue";
import { getCloths, hangup } from "@/api/system/cloths";
import { getUser } from "@/api/system/user";
import { addExpenditure } from "@/api/system/expenditure";
import {invoke} from '@tauri-apps/api/core';

const props = defineProps({
    visible: {
        type: Boolean,
        required: true,
        default: false,
    },
    orderId: {
        type: Number,
        required: true,
        default: 0
    },
    flashList: {
        type: Function,
        required: true,
    },
    userId: {
        type: String,
        required: true,
    },
    toggle: {
        type: Function,
        required: true,
    },
});

const { proxy } = getCurrentInstance();

const { sys_delivery_mode, sys_clothing_status, sys_service_type, sys_service_requirement, } =
    proxy.useDict("sys_delivery_mode", "sys_clothing_status", "sys_service_type", "sys_service_requirement");

const selectionList = ref([]);
const clothsList = ref([]);
const pictureList = ref([]);
const currentCloth = ref({});
const showPicture = ref(false);
const showPickUpDialog = ref(false);
const showCompensationDialog = ref(false);
const showClothesDialog = ref(false);
const loading = ref(true);
const showHangUp = ref(false);
const colorList = ref([]);
const flawList = ref([]);
const estimateList = ref([]);
const brandList = ref([]);

const afterSaleDisabled = ref(true);
const compensationDisabled = ref(true);
const pickupDisabled = ref(true);
const baseUrl = import.meta.env.VITE_APP_BASE_API;
const pictureUrl = ref(baseUrl + "/system/cloths/download/");
const data = reactive({
    form: {},
    hangForm: {},
    pickupForm: {},
    compensationForm: {},
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
        hangLocationId: [
            { required: true, message: "衣挂位置不能为空", trigger: "blur" }
        ],
        hangClothCode: [
            { required: true, message: "衣挂编号不能为空", trigger: "blur" }
        ]
    },
    pickupRules: {}
});

const { pickupForm, compensationForm, hangForm, hangRules } = toRefs(data);

function close() {
    showClothesDialog.value = false;
    props.toggle();
}

/** 查询订单包含的衣物清单列表 */
async function getList() {
    // 判断是否有订单id
    if (props.orderId == 0) {
        return;
    }
    loading.value = true;

    await listCloths({ orderId: props.orderId }).then(response => {
        clothsList.value = response;
        loading.value = false;
    });
}

/* 初始化列表数据 */
async function initList() {
    const promises = [];

    // 获取颜色列表
    if (colorList.value.length === 0) {
        const colorPromise = listTagsNoLimit({ tagOrder: '003' }).then(response => {
            colorList.value = response;
        });
        promises.push(colorPromise);
    }

    // 获取瑕疵列表
    if (flawList.value.length === 0) {
        const flawPromise = listTagsNoLimit({ tagOrder: '001' }).then(response => {
            flawList.value = response;
        });
        promises.push(flawPromise);
    }

    // 获取预估列表
    if (estimateList.value.length === 0) {
        const estimatePromise = listTagsNoLimit({ tagOrder: '002' }).then(response => {
            estimateList.value = response;
        });
        promises.push(estimatePromise);
    }

    // 获取品牌列表
    if (brandList.value.length === 0) {
        const brandPromise = listTagsNoLimit({ tagOrder: '004' }).then(response => {
            brandList.value = response;
        });
        promises.push(brandPromise);
    }

    // 等待所有异步操作完成防止衣物列表数据加载完后这里的数据没有准备好而出错
    await Promise.all(promises);
}

// 重置取走form
function resetPickupForm() {
    pickupForm.value = {
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
    }
    proxy.resetForm("pickupRef");
}

// 多选框选中数据
function handleSelectionChange(selection) {
    selectionList.value = selection;
    if (selectionList.value.find(item => item.clothingStatus == '00')) {
        afterSaleDisabled.value = false;
    } else {
        afterSaleDisabled.value = true;
    }

    // 当勾选的订单中有已上挂的衣物，则显示取走/赔偿按钮
    if (selectionList.value.find(item => item.clothingStatus == '02')) {
        pickupDisabled.value = false;
        compensationDisabled.value = false;
    } else {
        pickupDisabled.value = true;
        compensationDisabled.value = true;
    }
}


/* 显示上挂 */
function handleShowHangUp(row) {
    showHangUp.value = true;
    currentCloth.value = row;
    hangForm.value = {
        clothId: row.clothId,
        clothingNumber: row.hangClothCode,
        hangLocationId: row.hangLocationCode,
        hangerNumber: row.hangerNumber,
        hangRemark: null
    };
}

/* 赔偿 */
function compensate() {
    addExpenditure(compensationForm.value).then(res => {
        proxy.$modal.msgSuccess("赔偿成功");
        showCompensationDialog.value = false;
    })
}

function handleCompensate() {
    getUser(props.userId).then(res => {
        showCompensationDialog.value = true;
        let title = selectionList.value[0].clothInfo.clothingName;
        if (selectionList.value.length > 1) {
            title += '...';
        }
        compensationForm.value = {
            expTitle: title,
            recvAccountTitle: res.data.userName,
            recvAccount: res.data.userId,
            expAmount: null,
            expType: "01",
            orderId: props.orderId,
            clothIds: selectionList.value.map(item => item.clothId).join(','),
            remark: null,
        }
    });
}

/* 上挂 */
function hangUp() {
    if (currentCloth.value) {
        //校验上挂表单内容
        proxy.$refs["hangUpRef"].validate(async valid => {
            if (valid) {
                console.log(currentCloth.value)
                console.log(hangForm.value)
                proxy.$modal.loading("上挂中...");
                await hangup(hangForm.value).then(res => {
                    proxy.$modal.msgSuccess("上挂成功");
                    showHangUp.value = false;
                    getList();
                    props.flashList();
                }).catch(res => {
                    proxy.$modal.msgError(res.msg);
                });
                proxy.$modal.closeLoading();
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

const loadImage = async (id) => {
    try {
        // 调用 Tauri 后端命令获取图片二进制数据
        const imageData = await invoke('get_image', { id });

        // 将二进制数据转换为 Blob
        const blob = new Blob([new Uint8Array(imageData)], { type: 'image/png' });

        // 生成图片 URL
        return URL.createObjectURL(blob);

        // 提示加载成功
    } catch (error) {
        // 提示加载失败
    }
};

/* 获取图片列表id */
async function handleShowPicture(row, flag) {
    showPicture.value = true;

    try {
        // 获取图片 ID 列表
        const picIds = flag ? row.beforePics?.split(',') : row.afterPics?.split(',');

        if (picIds && picIds.length > 0) {
            // 使用 Promise.all 等待所有图片加载完成
            const imageUrls = await Promise.all(picIds.map(id => loadImage(Number(id))));

            // 过滤掉加载失败的图片（null）
            pictureList.value = imageUrls.filter(url => url !== null);
        } else {
            pictureList.value = []; // 如果没有图片 ID，清空列表
        }
    } catch (error) {
        console.error(`处理图片列表失败: ${error}`);
        pictureList.value = []; // 出错时清空列表
    }
}
// 显示取走
function handlePickup() {
    showPickUpDialog.value = true;
}

function pickup() {

}

function cancelPickup() {
    showPickUpDialog.value = false;
    resetPickupForm();
}


onMounted(async () => {
    if (props.visible) {

        await initList();  // 确保 initList 完成
        await getList();         // 在 initList 完成后调用
        showClothesDialog.value = true;
    }
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