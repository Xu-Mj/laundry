<template>
    <div class="app-container">
        <el-row :gutter="10" class="mb8">
            <el-col :span="1.5">
                <el-button type="primary" plain icon="Plus" @click="handleAdd"
                    v-hasPermi="['system:cloths:add']">新增</el-button>
            </el-col>
        </el-row>

        <el-table :data="clothList">
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
            <el-table-column label="衣物瑕疵" align="center" prop="clothingFlaw">
                <template #default="scope">
                    <el-tag v-for="tagId in scope.row.clothingFlawArr" :key="item" type="danger">
                        {{ flawList.find(item => item.tagId == tagId).tagName }}
                    </el-tag>
                </template>
            </el-table-column>
            <el-table-column label="洗后预估" align="center" prop="estimate">
                <template #default="scope">
                    <el-tag v-for="tagId in scope.row.estimateArr" :key="item" type="primary">
                        {{ estimateList.find(item => item.tagId == tagId).tagName }}
                    </el-tag>
                </template>
            </el-table-column>
            <el-table-column label="工艺加价" align="center" prop="processMarkup" />
            <el-table-column label="服务要求" align="center" prop="serviceRequirement">
                <template #default="scope">
                    <dict-tag :options="sys_service_requirement" :value="scope.row.serviceRequirement" />
                </template>
            </el-table-column>
            <el-table-column label="洗护价格" align="center" prop="priceValue" />
            <el-table-column label="补充信息" align="center" prop="hangRemark" />
            <el-table-column label="操作" align="center" :width="200" class-name="small-padding fixed-width">
                <template #default="scope">
                    <el-button link type="primary" :icon="Camera" @click="handleShowUploadPic(scope.row)"
                        v-hasPermi="['system:cloths:edit']">拍照</el-button>
                    <el-button link type="primary" icon="Edit" @click="handleUpdate(scope.row)"
                        v-hasPermi="['system:cloths:edit']">修改</el-button>
                    <el-button link type="primary" icon="Delete" @click="handleDelete(scope.row)"
                        v-hasPermi="['system:cloths:remove']">删除</el-button>
                </template>
            </el-table-column>
        </el-table>

        <!-- 上传照片对话框 -->
        <el-dialog title="上传照片" v-model="showUploadPicture" width="500px" append-to-body @closed="handleCloseUploadPic">
            <el-upload v-model:file-list="fileList" class="upload-demo" :action="uploadBeforeImgUrl" :headers="headers"
                :on-preview="handlePreview" :on-remove="handleRemove" list-type="picture">
                <el-button type="primary">点击上传洗前图片</el-button>
                <template #tip>
                    <div class="el-upload__tip">
                        jpg/png files with a size less than 500kb
                    </div>
                </template>
            </el-upload>
            <el-upload v-model:file-list="fileList" class="upload-demo" :action="uploadAfterImgUrl" :headers="headers"
                :on-preview="handlePreview" :on-remove="handleRemove" list-type="picture">
                <el-button type="primary">点击上传洗后图片</el-button>
                <template #tip>
                    <div class="el-upload__tip">
                        jpg/png files with a size less than 500kb
                    </div>
                </template>
            </el-upload>

            <el-dialog v-model="dialogVisible" title="预览" width="800px" append-to-body>
                <img :src="dialogImageUrl" style="display: block; max-width: 100%; margin: 0 auto" />
            </el-dialog>
        </el-dialog>
        <!-- 添加或修改订单包含的衣物清单对话框 -->
        <el-dialog :title="title" v-model="open" width="1080px" modal :close-on-click-modal="false" @closed="reset()"
            @keydown.enter.native="handleEnterKeyDown" @keydown.right.native="handleEnterKeyDown"
            @keydown.left.native="handleLeftKeyDown" append-to-body>
            <el-steps :active="step" finish-status="success" simple>
                <el-step title="选择品类" :icon="CopyDocument" v-if="step !== 6" />
                <el-step title="选择衣物" :icon="User" v-if="step !== 6" />
                <el-step title="选择颜色" :icon="PictureRounded" v-if="step !== 6" />
                <el-step title="洗前瑕疵" :icon="WarningFilled" v-if="step !== 6" />
                <el-step title="洗后预估" :icon="CoffeeCup" v-if="step !== 6" />
                <el-step title="选择品牌" :icon="CollectionTag" v-if="step !== 6" />

                <el-step :title="sys_cloth_cate.find(item => item.value == form.clothingCategory).label"
                    :icon="CopyDocument" v-if="step == 6" />
                <el-step :title="sys_cloth_style.find(item => item.value == form.clothingStyle).label" :icon="User"
                    v-if="step == 6" />
                <!-- <el-step :title="clothList.find(item => { return item.clothingId == form.clothingId }).clothingName"
                    :icon="User" v-if="step == 6" /> -->
                <el-step
                    :title="form.clothingColor ? colorList.find(item => item.tagId == form.clothingColor).tagName : '未选择颜色'"
                    :icon="PictureRounded" v-if="step == 6" />
                <el-step title="洗前瑕疵" :icon="WarningFilled" v-if="step == 6" />
                <el-step title="洗后预估" :icon="CoffeeCup" v-if="step == 6" />
                <!-- <el-step
                    :title="form.clothingFlaw ? flawList.find(item => { return item.tagId == form.clothingFlaw }).tagName : '没有瑕疵'"
                    :icon="WarningFilled" v-if="step == 6" />
                <el-step
                    :title="form.estimate ? estimateList.find(item => { return item.tagId == form.estimate }).tagName : '没有洗后预估'"
                    :icon="CoffeeCup" v-if="step == 6" /> -->
                <el-step
                    :title="form.clothingBrand ? brandList.find(item => { return item.tagId == form.clothingBrand }).tagName : '没有选择品牌'"
                    :icon="CollectionTag" v-if="step == 6" />

                <el-button type="primary" v-show="step == 6" class="steps-btn" @click="step = 0">编辑</el-button>
            </el-steps>
            <el-form ref="clothsRef" :model="form" :rules="rules" class="form-container">
                <div v-show="step == 0">
                    <el-form-item label="衣物品类">
                        <el-radio-group v-model="form.clothingCategory">
                            <el-radio v-for="dict in sys_cloth_cate" :key="dict.value" :value="dict.value">{{
                                dict.label
                            }}</el-radio>
                        </el-radio-group>
                    </el-form-item>
                    <el-form-item label="衣物类别">
                        <el-radio-group v-model="form.clothingStyle">
                            <el-radio v-for="dict in sys_cloth_style" :key="dict.value" :value="dict.value">{{
                                dict.label
                            }}</el-radio>
                        </el-radio-group>
                    </el-form-item>
                    <el-row class="footer-btn">
                        <el-button type="primary" :disabled="!form.clothingCategory || !form.clothingStyle"
                            @click="nextStep">下一步</el-button>
                    </el-row>
                </div>
                <div v-show="step == 1">
                    <el-form-item label="衣物名称">
                        <div class="input-btn-row">
                            <el-input v-model="clothNameInput" ref="clothNameRef" @input="searchCloth"
                                placeholder="请输衣物名称首字母或衣物名称" />
                            <el-button v-if="showAddClothBtn" type="primary" @click="handleAddCloth">新增</el-button>
                        </div>
                    </el-form-item>
                    <el-form-item label="洗护价格" v-if="showAddClothBtn && showPriceContent">
                        <div class="price-content">
                            <div class="price-wrapper">
                                <el-input-number v-model="form.clothInfo.clothingBasePrice" :min="0" :controls="false"
                                    placeholder="请输入基准价格" />
                                <el-input-number v-model="form.clothInfo.clothingMinPrice" :min="0" :controls="false"
                                    placeholder="请输入最低价格" />
                                <el-input-number v-model="form.clothInfo.clothingMetuanPrice" :min="0" :controls="false"
                                    placeholder="请输入美团价格" />
                                <el-input-number v-model="form.clothInfo.clothingDouyinPrice" :min="0" :controls="false"
                                    placeholder="请输入抖音价格" />
                                <el-input-number v-model="form.clothInfo.clothingXiaochenxuPrice" :min="0"
                                    :controls="false" placeholder="请输入小程序价格" />
                            </div>
                            <el-button type="primary" @click="createCloth">确定添加</el-button>
                        </div>
                    </el-form-item>
                    <!-- 展示衣物标签 -->
                    <el-row class="item-list-area">
                        <el-radio-group class="color-radio-group" v-model="form.clothingId" @change="step2ClothChange">
                            <el-radio v-for="color in clothingList" :key="color.clothingId" :value="color.clothingId">{{
                                color.clothingName
                            }}</el-radio>
                        </el-radio-group>
                    </el-row>
                    <el-row class="footer-btn">
                        <el-button type="primary" @click="preStep">上一步</el-button>
                        <el-button type="primary" @click="nextStep" :disabled="!form.clothingId">下一步</el-button>
                    </el-row>
                </div>
                <div v-show="step == 2">
                    <el-form-item label="颜色名称">
                        <div class="input-btn-row">
                            <el-input v-model="clothColorInput" @input="searchColor" placeholder="请输颜色名称首字母或者颜色名称" />
                            <el-button v-if="showAddColorBtn" type="primary"
                                @click="addTag('003', clothColorInput)">新增</el-button>
                        </div>
                    </el-form-item>
                    <!-- 展示颜色 -->
                    <el-row class="item-list-area">
                        <el-radio-group class="color-radio-group" v-model="form.clothingColor">
                            <el-radio v-for="color in colorList" :key="color.tagId" :value="color.tagId">
                                <el-tooltip :content="color.tagNumber">
                                    {{ color.tagName }}
                                </el-tooltip>
                            </el-radio>
                        </el-radio-group>
                    </el-row>
                    <el-row class="footer-btn">
                        <el-button type="primary" @click="preStep">上一步</el-button>
                        <el-button type="primary" @click="nextStep">下一步</el-button>
                        <el-button type="primary" @click="jump2last">跳过后续步骤</el-button>
                    </el-row>
                </div>
                <div v-show="step == 3">
                    <el-form-item label="瑕疵名称">
                        <div class="input-btn-row">
                            <el-input v-model="flawInput" @input="searchColor" placeholder="请输名称首字母或者名称" />
                            <el-button v-if="showAddFlawBtn" type="primary"
                                @click="addTag('001', flawInput)">新增</el-button>
                        </div>
                    </el-form-item>
                    <!-- 展示瑕疵 -->
                    <el-row class="item-list-area">
                        <el-checkbox-group class="color-radio-group" v-model="form.clothingFlawArr">
                            <el-checkbox v-for="item in flawList" :key="item.tagId" :value="item.tagId">
                                <el-tooltip :content="item.tagNumber">
                                    {{ item.tagName }}
                                </el-tooltip>
                            </el-checkbox>
                        </el-checkbox-group>
                    </el-row>
                    <el-row class="footer-btn">
                        <el-button type="primary" @click="preStep">上一步</el-button>
                        <el-button type="primary" @click="nextStep">下一步</el-button>
                        <el-button type="primary" @click="jump2last">跳过后续步骤</el-button>
                    </el-row>
                </div>
                <div v-show="step == 4">
                    <el-form-item label="洗后预估">
                        <div class="input-btn-row">
                            <el-input v-model="estimateInput" @input="searchColor" placeholder="请输名称首字母或者名称" />
                            <el-button v-if="showAddEstimateBtn" type="primary"
                                @click="addTag('002', estimateInput)">新增</el-button>
                        </div>
                    </el-form-item>
                    <!-- 展示洗后预估标签 -->
                    <el-row class="item-list-area">
                        <el-checkbox-group class="color-radio-group" v-model="form.estimateArr">
                            <el-checkbox v-for="item in estimateList" :key="item.tagId" :value="item.tagId">
                                <el-tooltip :content="item.tagNumber">
                                    {{ item.tagName }}
                                </el-tooltip>
                            </el-checkbox>
                        </el-checkbox-group>
                    </el-row>
                    <el-row class="footer-btn">
                        <el-button type="primary" @click="preStep">上一步</el-button>
                        <el-button type="primary" @click="nextStep">下一步</el-button>
                        <el-button type="primary" @click="jump2last">跳过后续步骤</el-button>
                    </el-row>
                </div>
                <div v-show="step == 5">
                    <el-form-item label="品牌名称">
                        <div class="input-btn-row">
                            <el-input v-model="brandInput" @input="searchColor" placeholder="请输品牌名称首字母或者品牌名称" />
                            <el-button v-if="showAddBrandBtn" type="primary"
                                @click="addTag('004', brandInput)">新增</el-button>
                        </div>
                    </el-form-item>
                    <!-- 展示品牌 -->
                    <el-row class="item-list-area">
                        <el-radio-group class="color-radio-group" v-model="form.clothingBrand">
                            <el-radio v-for="color in brandList" :key="color.tagId" :value="color.tagId">
                                <el-tooltip :content="color.tagNumber">
                                    {{ color.tagName }}
                                </el-tooltip>
                            </el-radio>
                        </el-radio-group>
                    </el-row>
                    <el-row class="footer-btn">
                        <el-button type="primary" @click="preStep">上一步</el-button>
                        <el-button type="primary" @click="nextStep">下一步</el-button>
                        <el-button type="primary" @click="jump2last">跳过后续步骤</el-button>
                    </el-row>
                </div>
                <div v-show="step == 6">
                    <el-row>
                        <el-col :span="12">
                            <el-form-item label="服务类型">
                                <el-radio-group v-model="form.serviceType">
                                    <el-radio v-for="type_ in sys_service_type" :key="type_.value" :value="type_.value"
                                        :label="type_.label">{{ type_.label }}</el-radio>
                                </el-radio-group>
                            </el-form-item>
                        </el-col>
                        <el-col :span="12">
                            <el-form-item label="服务要求">
                                <el-radio-group v-model="form.serviceRequirement">
                                    <el-radio v-for="type_ in sys_service_requirement" :key="type_.value"
                                        :value="type_.value" :label="type_.label">{{ type_.label }}</el-radio>
                                </el-radio-group>
                            </el-form-item>
                        </el-col>
                    </el-row>
                    <el-row>
                        <el-col :span="12" class="markup">
                            <el-form-item label="收费价格">
                                {{ form.priceValue }}
                                <!-- <el-input-number v-model="form.priceValue" :min="0" controls-position="right" /> -->
                            </el-form-item>
                        </el-col>
                        <el-col :span="12" class="markup">
                            <el-form-item label="工艺加价">
                                <el-input-number v-model="form.processMarkup" :min="0" controls-position="right" />
                            </el-form-item>
                            <el-button type="primary" @click="handleShowHistory">{{ showHistory ? '隐藏历史' : '查看历史'
                                }}</el-button>
                        </el-col>
                    </el-row>
                    <!-- 展示历史记录 -->
                    <el-row v-show="showHistory">
                        <el-table :data="clothHistoryList">
                            <el-table-column label="服务日期" align="center" prop="createTime" />
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
                            <el-table-column label="工艺加价" align="center" prop="processMarkup" />
                            <el-table-column label="服务要求" align="center" prop="serviceRequirement">
                                <template #default="scope">
                                    <dict-tag :options="sys_service_requirement"
                                        :value="scope.row.serviceRequirement" />
                                </template>
                            </el-table-column>
                            <el-table-column label="价格" align="center" prop="priceValue" />
                            <el-table-column label="补充信息" align="center" prop="hangRemark" />
                        </el-table>
                    </el-row>
                    <el-row>
                        <el-col :span="12">
                            <el-form-item>
                                <el-input type="textarea" v-model="form.remark" class="remark" placeholder="点击输入备注信息" />
                            </el-form-item>
                        </el-col>
                        <el-col :span="12" class="final-btn">
                            <el-button type="primary" @click="submitForm">确认添加</el-button>
                            <el-button type="primary" @click="cancel">取消</el-button>
                        </el-col>
                    </el-row>
                </div>
            </el-form>
        </el-dialog>
    </div>
</template>

<script setup name="AddCloth">
import { listHistoryCloths, delCloths, addCloths, updateCloths } from "@/api/system/cloths";
import { Camera, CoffeeCup, CollectionTag, CopyDocument, PictureRounded, User, WarningFilled } from "@element-plus/icons-vue";
import { listClothing, addClothing } from "@/api/system/clothing";
import { listTags, addTags } from "@/api/system/tags";
import pinyin from 'pinyin';
import { ref, reactive, toRefs, watch, } from "vue";
import { listCloths } from "@/api/system/cloths";
import { getToken } from "@/utils/auth";

const props = defineProps({
    userId: {
        type: Number,
        required: true,
        default: 0
    },
    orderId: {
        type: Number,
        default: 0
    },
    value: Array
});


const { proxy } = getCurrentInstance();
const { sys_cloth_cate, sys_cloth_style, sys_service_type, sys_service_requirement, } =
    proxy.useDict("sys_cloth_cate", "sys_cloth_style", "sys_service_type", "sys_service_requirement");

// 添加衣物的列表
const clothList = ref([]);
const emit = defineEmits(['update:value']);

// 选择衣物时展示的衣物列表
const clothingList = ref([]);
// 该用户洗过的衣物历史记录
const clothHistoryList = ref([]);
const open = ref(false);
const showUploadPicture = ref(false);
const showPriceContent = ref(false);
const showAddClothBtn = ref(false);
const showAddColorBtn = ref(false);
const showAddFlawBtn = ref(false);
const showAddEstimateBtn = ref(false);
const showAddBrandBtn = ref(false);
const showHistory = ref(false);
const clothListloading = ref(false);
const title = ref("");
const step = ref(0);
const clothNameInput = ref(null);
const clothColorInput = ref(null);
const flawInput = ref(null);
const estimateInput = ref(null);
const brandInput = ref(null);
const colorList = ref([]);
const flawList = ref([]);
const estimateList = ref([]);
const brandList = ref([]);
const currentCloth = ref();
const featureList = [colorList, flawList, estimateList, brandList]
const isBeforePic = ref(true);

const headers = ref({ Authorization: "Bearer " + getToken() });
const baseUrl = import.meta.env.VITE_APP_BASE_API;
const uploadBeforeImgUrl = ref(baseUrl + `/system/cloths/upload?isPre=true&clothId=`); // 上传的图片服务器地址
const uploadAfterImgUrl = ref(baseUrl + `/system/cloths/upload?isPre=false&clothId=`); // 上传的图片服务器地址
const dialogImageUrl = ref("");
const clothNameRef = ref();
const dialogVisible = ref(false);// 预览

const data = reactive({
    form: {},
    rules: {
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
    }
});

const { form, rules } = toRefs(data);

function handlePreview(file) {
    dialogImageUrl.value = file.url;
    dialogVisible.value = true;
    console.log(dialogImageUrl.value)
}

function handleEnterKeyDown() {
    console.log('enter')
    nextStep();
}

function handleLeftKeyDown() {
    preStep();
}

// 当订单id不为空时那么为修改操作
function getList() {
    console.log(props.orderId)
    if (props.orderId && props.orderId !== 0) {
        listCloths({ orderClothId: props.orderId }).then(res => {
            res.rows.map(item => {
                if (item.estimate) {
                    item.estimateArr = item.estimate.split(',').map(Number);
                }
                if (item.clothingFlaw) {
                    item.clothingFlawArr = item.clothingFlaw.split(',').map(Number);
                }
            })
            clothList.value = res.rows;
            emit('update:value', clothList.value);
        })
    }
}

// 取消按钮
function cancel() {
    open.value = false;
    reset();
}

// 表单重置
function reset() {
    form.value = {
        clothInfo: {},
        orderClothId: null,
        clothingId: null,
        clothingCategory: "000",
        clothingStyle: "000",
        clothingColor: null,
        clothingFlaw: null,
        estimate: null,
        clothingBrand: null,
        serviceType: "000",
        serviceRequirement: "000",
        beforePics: null,
        afterPics: null,
        notes: null,
        processMarkup: 0,
        priceValue: 0,
        hangLocationCode: null,
        hangClothCode: null,
        hangRemark: null,
        createTime: null
    };
    step.value = 0;
    showAddBrandBtn.value = false;
    showAddColorBtn.value = false;
    showAddFlawBtn.value = false;
    showAddEstimateBtn.value = false;
    showAddClothBtn.value = false;
    showHistory.value = false;
    proxy.resetForm("clothsRef");
}

/* 初始化列表数据 */
async function initList() {
    const promises = [];

    // 获取衣物列表
    if (clothingList.value.length === 0) {
        const clothingPromise = listClothing({}).then(response => {
            clothingList.value = response.rows;
        });
        promises.push(clothingPromise);
    }

    // 获取颜色列表
    if (colorList.value.length === 0) {
        const colorPromise = listTags({ tagOrder: '003', status: "0" }).then(response => {
            colorList.value = response.rows;
        });
        promises.push(colorPromise);
    }

    // 获取瑕疵列表
    if (flawList.value.length === 0) {
        const flawPromise = listTags({ tagOrder: '001', status: "0" }).then(response => {
            flawList.value = response.rows;
        });
        promises.push(flawPromise);
    }

    // 获取预估列表
    if (estimateList.value.length === 0) {
        const estimatePromise = listTags({ tagOrder: '002', status: "0" }).then(response => {
            estimateList.value = response.rows;
        });
        promises.push(estimatePromise);
    }

    // 获取品牌列表
    if (brandList.value.length === 0) {
        const brandPromise = listTags({ tagOrder: '004', status: "0" }).then(response => {
            brandList.value = response.rows;
        });
        promises.push(brandPromise);
    }

    // 等待所有异步操作完成防止衣物列表数据加载完后这里的数据没有准备好而出错
    await Promise.all(promises);
    console.log('初始化完成')
}

/** 新增按钮操作 */
function handleAdd() {
    reset();
    open.value = true;
    title.value = "添加衣物";
}

/** 修改按钮操作 */
function handleUpdate(row) {
    reset();
    form.value = row;
    open.value = true;
    title.value = "修改衣物";
    console.log(form.value)
}

/** 提交按钮 */
function submitForm() {
    proxy.$refs["clothsRef"].validate(valid => {
        if (valid) {
            const submitData = { ...form.value };
            if (submitData.estimateArr) {
                submitData.estimate = submitData.estimateArr.join(',');
                delete submitData.estimateArr;
            }
            if (submitData.clothingFlawArr) {
                submitData.clothingFlaw = submitData.clothingFlawArr.join(',');
                delete submitData.clothingFlawArr;
            }
            if (form.value.clothId != null) {
                updateCloths(submitData).then(response => {
                    proxy.$modal.msgSuccess("修改成功");
                    open.value = false;
                    getList();
                });
            } else {
                if (props.orderId) {
                    submitData.orderClothId = props.orderId;
                }
                addCloths(submitData).then(response => {
                    proxy.$modal.msgSuccess("新增成功");
                    open.value = false;
                    form.value.clothId = response.data;
                    clothList.value.push(form.value);
                    emit('update:value', clothList.value);
                    // console.log(clothList.value)
                    // getList();
                });
            }
        }
    });
}


/** 删除按钮操作 */
function handleDelete(row) {
    console.log(row)
    const _orderClothIds = row.clothId;
    proxy.$modal.confirm('是否确认删除订单包含的衣物清单编号为"' + _orderClothIds + '"的数据项？').then(function () {
        return delCloths(_orderClothIds);
    }).then(() => {
        getList();
        proxy.$modal.msgSuccess("删除成功");
    }).catch(() => { });
}

/* 上一步 */
function preStep() {
    if (step.value > 0) {
        step.value--;
    }
}

/* 下一步 */
function nextStep() {
    // 校验衣物是否选择
    if (step.value === 1 && !form.value.clothingId) {
        return;
    }
    if (step.value !== 6) {
        step.value++;
    }

    if (step.value === 1 && clothNameRef.value) {
        clothNameRef.value.focus();
    }
}

/* 跳过后续步骤 */
function jump2last() {
    step.value = 6;
}

/* 获取衣物列表 */
function getClothingList(name) {
    clothListloading.value = true;
    listClothing({ clothingName: name }).then(res => {
        clothingList.value = res.rows;
        clothListloading.value = false;
        if (name.length > 0 && res.rows.length > 0) {
            showAddClothBtn.value = true;
        } else {
            showAddClothBtn.value = false;
        }
    });
}

// 获取颜色label的拼音首字母
const getPinyinInitials = (word) => {
    const pinyinResult = pinyin(word, {
        style: pinyin.STYLE_FIRST_LETTER, // 获取首字母
        heteronym: false // 禁用多音字
    });
    return pinyinResult.flat().join('').toUpperCase(); // 转为大写并拼接
};

/* 搜索颜色 */
function searchCloth(color) {
    const upperCaseColor = color.trim().toUpperCase();
    if (upperCaseColor === '') {
        return;
    }

    // 颜色、瑕疵、洗后预估、品牌是从第3步开始渲染的，因此要-2
    const item = clothingList.value.find(item => {
        return item.clothingName.includes(upperCaseColor) || getPinyinInitials(item.clothingName).includes(upperCaseColor);
    });

    if (!item) {
        showAddClothBtn.value = true;
        form.value.clothingColor = null;
    } else {
        form.value.clothingId = item.clothingId;
        showAddClothBtn.value = false;
    }
}
/* 搜索颜色 */
function searchColor(color) {
    const upperCaseColor = color.trim().toUpperCase();
    if (upperCaseColor === '') {
        return;
    }

    // 颜色、瑕疵、洗后预估、品牌是从第3步开始渲染的，因此要-2
    const index = step.value - 2;
    const item = featureList[index].value.find(item => {
        return item.tagName.includes(upperCaseColor) || getPinyinInitials(item.tagName).includes(upperCaseColor);
    });

    if (!item) {
        switch (index) {
            case 0:
                showAddColorBtn.value = true;
                form.value.clothingColor = null;
                break;
            case 1:
                showAddFlawBtn.value = true;
                form.value.clothingFlaw = null;
                break;
            case 2:
                showAddEstimateBtn.value = true;
                form.value.estimate = null;
                break;
            case 3:
                showAddBrandBtn.value = true;
                form.value.clothingBrand = null;
                break;
            default: ;
        }
    } else {
        switch (index) {
            case 0:
                form.value.clothingColor = item.tagId;
                showAddColorBtn.value = false;
                break;
            case 1:
                form.value.clothingFlaw = item.tagId;
                showAddFlawBtn.value = false;
                break;
            case 2:
                form.value.estimate = item.tagId;
                showAddEstimateBtn.value = false;
                break;
            case 3:
                form.value.clothingBrand = item.tagId;
                showAddBrandBtn.value = false;
                break;
            default: ;
        }

    }
}

/* 显示历史记录 */
function handleShowHistory() {
    showHistory.value = !showHistory.value;
    if (showHistory.value && clothHistoryList.value.length === 0) {
        listHistoryCloths(props.userId).then(res => {
            clothHistoryList.value = res.rows;
            clothHistoryList.value.map(item => {
                item.clothingFlawArr = item.clothingFlaw.split(',');
                item.estimateArr = item.estimate.split(',');
            });
        })
    }
}

/* 显示添加衣物按钮 */
function handleAddCloth() {
    showPriceContent.value = true;
    form.value.clothInfo = {};
}

function createCloth() {
    if (!clothNameInput.value || clothNameInput.value.length === 0) {
        proxy.$modal.msgError("请输入衣物名称");
        return;
    }
    const data = form.value.clothInfo;
    if (!data.clothingBasePrice) {
        proxy.$modal.msgError("请输入衣物价格");
        return;
    }
    data.clothingMinPrice = data.clothingMinPrice || data.clothingBasePrice;
    data.clothingCategory = form.value.clothingCategory;
    data.clothingStyle = form.value.clothingStyle;
    data.clothingName = clothNameInput.value;

    addClothing(data).then(response => {
        proxy.$modal.msgSuccess("新增衣物成功");
        getClothingList();
        showPriceContent.value = false;
        showAddClothBtn.value = false;
        form.value.clothInfo = {};
    })
}

/* 新增标签 */
function addTag(type, tagName) {
    addTags({ tagName: tagName, tagOrder: type }).then(res => {
        proxy.$modal.msgSuccess("新增成功");
        addItemToList(type, { tagId: res.data, tagName: tagName, tagOrder: type });
        nextStep();
    });
}

function addItemToList(type, item) {
    switch (type) {
        case "003":
            colorList.value.push(item);
            form.value.clothingColor = item.tagId;
            showAddColorBtn.value = false;
            clothColorInput.value = '';
            break;
        case "001":
            flawList.value.push(item);
            form.value.clothingFlaw = item.tagId;
            showAddFlawBtn.value = false;
            flawInput.value = '';
            break;
        case "002":
            estimateList.value.push(item);
            form.value.estimate = item.tagId;
            showAddEstimateBtn.value = false;
            estimateInput.value = '';
            break;
        case "004":
            brandList.value.push(item);
            form.value.clothingBrand = item.tagId;
            showAddBrandBtn.value = false;
            brandInput.value = '';
            break;
        default:
            ;
    }
}

/* 衣物发生变化时要将最后一步的价格设置为选中衣物中的价格 */
function step2ClothChange() {
    if (form.value.clothingId) {
        form.value.priceValue = clothingList.value.find(item => item.clothingId == form.value.clothingId).clothingBasePrice;
    }
}

/* 显示上传照片dialog */
function handleShowUploadPic(row) {
    currentCloth.value = row;
    showUploadPicture.value = true;
    uploadBeforeImgUrl.value = uploadBeforeImgUrl.value + row.clothId;
    uploadAfterImgUrl.value = uploadAfterImgUrl.value + row.clothId;
    console.log(currentCloth.value)
}

/* 关闭上传图片时清理对象 */
function handleCloseUploadPic() {
    currentCloth.value = {};
    showUploadPicture.value = false;
}

onMounted(async () => {
    await initList();  // 确保 initList 完成
    console.log('准备获取列表');
    getList();         // 在 initList 完成后调用
});
</script>
<style scoped>
.app-container {
    width: 100%;
}

.el-steps--simple {
    padding: 1rem;
}

.form-container {
    padding: 1rem;
}

.footer-btn {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    /* gap: .1rem; */
}

.remark {
    margin-right: 1rem;
}

.final-btn {
    width: 100%;
    display: flex;
    justify-content: flex-start;
    align-items: center;
    gap: 1rem;
    /* 为了能够和el-input 对齐 */
    margin-bottom: 14px;

    button {
        width: 6rem;
        height: 100%;
    }
}

.color-radio-group {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: flex-start;
    align-items: center;
    flex-flow: row wrap;
}

.input-btn-row {
    width: 100%;
    display: grid;
    grid-template-columns: 9fr 1fr;
    justify-content: center;
    align-items: center;
    gap: 1rem
}

.steps-btn {
    margin-left: 1rem;
}


.markup {
    display: flex;
    gap: 1rem;
}

.price-content {
    width: 100%;
    display: flex;
    justify-content: space-between;
    gap: 1rem;

    .price-wrapper {
        width: 100%;
        display: flex;
        justify-content: flex-start;
        gap: .25rem;
    }
}

.item-list-area {
    width: 100%;
    max-height: 3rem;
}
</style>