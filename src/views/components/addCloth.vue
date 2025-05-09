<template>
    <h1 class="overlay" v-if="!props.userId">请先选择会员</h1>
    <div class="app-container" v-else>
        <!-- 添加或修改订单包含的衣物清单对话框 -->
        <el-steps :active="step" finish-status="success" align-center="true" simple>
            <el-step class="step-item" title="选择品类" :icon="CopyDocument" @click="jumpToStep(0)" />
            <el-step class="step-item" title="选择衣物" :icon="User" @click="jumpToStep(1)" />
            <el-step class="step-item" title="选择颜色" :icon="PictureRounded" @click="jumpToStep(2)" />
            <el-step class="step-item" title="洗前瑕疵" :icon="WarningFilled" @click="jumpToStep(3)" />
            <el-step class="step-item" title="洗后预估" :icon="CoffeeCup" @click="jumpToStep(4)" />
            <el-step class="step-item" title="选择品牌" :icon="CollectionTag" @click="jumpToStep(5)" />
        </el-steps>
        <el-form ref="clothsRef" :model="form" :rules="rules" class="form-container">

            <div class="steps-container">
                <div class="step" :class="{ active: step === 0 }" key="step0">
                    <el-row class="step0" key="step0">
                        <el-col :span="3" style="height: 100%;">
                            <el-scrollbar>
                                <div class="radio-group-column">
                                    <RadioButton class="radio-button-column" v-model="form.categoryId"
                                        v-for="category in categoryList" :key="category.categoryId"
                                        :label="category.categoryName" :value="category.categoryId"
                                        @change="cateChange" />
                                </div>
                            </el-scrollbar>
                        </el-col>
                        <el-col :span="21" style="height: 100%; padding-left: .5rem;">
                            <el-form-item label="">
                                <div class="input-btn-row">
                                    <el-input size="large" v-model="cateName" placeholder="请输入分类名称" maxlength="20" show-word-limit />
                                    <el-button size="large" type="primary" icon="Plus"
                                        @click="handleAddCate">新增</el-button>
                                </div>
                            </el-form-item>
                            <el-scrollbar class="scrollbar-height">
                                <div class="items-break">
                                    <RadioButton v-for="dict in clothStyleList" :key="dict.dictValue"
                                        v-model="form.styleId" :value="dict.dictValue" :label="dict.dictLabel"
                                        @change="nextStep()" />
                                </div>
                            </el-scrollbar>
                        </el-col>
                        <el-row class="footer-btn">
                            <el-button type="primary" size="large"
                                :disabled="!props.userId || !form.categoryId || !form.styleId" @click="nextStep">下一步
                                <el-icon>
                                    <ArrowRight />
                                </el-icon>
                            </el-button>
                        </el-row>
                    </el-row>
                </div>
                <div class="step" :class="{ active: step === 1 }" key="step1">
                    <el-row>
                        <el-col :span="24">
                            <el-form-item label="衣物名称" size="large">
                                <div class="input-btn-row">
                                    <el-input size="large" v-model="clothNameInput" ref="clothNameRef"
                                        @input="searchCloth" placeholder="请输衣物名称首字母或衣物名称" maxlength="30" show-word-limit />
                                    <el-button size="large" v-if="showAddClothBtn" type="primary" icon="Plus"
                                        @click="handleAddCloth">新增</el-button>
                                </div>
                            </el-form-item>
                        </el-col>
                    </el-row>
                    <div v-if="showAddClothBtn && showPriceContent">
                        <el-form-item size="large" label="洗护价格" v-if="showAddClothBtn && showPriceContent">
                            <div class="price-content">
                                <div class="price-wrapper">
                                    <el-input-number size="large" v-model="form.clothInfo.clothingBasePrice" :min="0"
                                        :controls="false" placeholder="请输入基准价格" />
                                    <el-input-number size="large" v-model="form.clothInfo.clothingMinPrice" :min="0"
                                        :controls="false" placeholder="请输入最低价格" />
                                    <!-- <el-input-number size="large" v-model="form.clothInfo.clothingMetuanPrice" :min="0"
                                        :controls="false" placeholder="请输入美团价格" />
                                    <el-input-number size="large" v-model="form.clothInfo.clothingDouyinPrice" :min="0"
                                        :controls="false" placeholder="请输入抖音价格" />
                                    <el-input-number size="large" v-model="form.clothInfo.clothingXiaochenxuPrice"
                                        :min="0" :controls="false" placeholder="请输入小程序价格" /> -->
                                </div>
                                <el-button size="large" type="primary" @click="createCloth"
                                    icon="CircleCheck">确定添加</el-button>
                            </div>
                        </el-form-item>
                        <el-form-item size="large" label="衣挂方式">
                            <el-radio-group size="large" v-model="form.clothInfo.hangType">
                                <el-radio :value="'1'">输送线</el-radio>
                                <el-radio :value="'2'">其他</el-radio>
                            </el-radio-group>
                        </el-form-item>
                    </div>
                    <!-- 展示衣物标签 -->
                    <el-scrollbar class="scrollbar-height">
                        <div class="items-break">
                            <RadioButton v-for="cloth in clothingListFilterResult" v-model="form.clothingId"
                                :key="cloth.id" @change="step2ClothChange" :value="cloth.id" :label="cloth.title" />
                        </div>
                    </el-scrollbar>
                    <el-row class="footer-btn">
                        <el-button type="primary" size="large" color="#f5f7fa" icon="ArrowLeft"
                            @click="preStep">上一步</el-button>
                        <el-button type="primary" size="large" @click="nextStep" :disabled="!form.clothingId">下一步
                            <el-icon>
                                <ArrowRight />
                            </el-icon>
                        </el-button>
                        <el-button type="danger" size="large" icon="Refresh" @click="reset">重新录入</el-button>
                    </el-row>
                </div>
                <div class="step" :class="{ active: step === 2 }" key="step2">
                    <el-row>
                        <el-col :span="24">
                            <el-form-item size="large" label="颜色名称">
                                <div class="input-btn-row">
                                    <el-input size="large" v-model="clothColorInput" @input="searchColor"
                                        placeholder="请输颜色名称首字母或者颜色名称" maxlength="10" show-word-limit />
                                    <el-button size="large" v-if="showAddColorBtn" type="primary" icon="Plus"
                                        @click="addTag('003', clothColorInput)">新增</el-button>
                                </div>
                            </el-form-item>
                        </el-col>
                    </el-row>
                    <!-- 展示颜色 -->
                    <el-row class="item-list-area">
                        <el-scrollbar class="scrollbar-height">
                            <div class="items-break">
                                <RadioButton v-for="color in tagsStore.colorList" :key="color.tagId" :value="color.tagId"
                                    v-model="form.clothingColor" @change="nextStep" :label="color.tagName"/>
                            </div>
                        </el-scrollbar>
                    </el-row>
                    <el-row class="footer-btn">
                        <el-button type="primary" size="large" color="#f5f7fa" icon="ArrowLeft"
                            @click="preStep">上一步</el-button>
                        <el-button type="primary" size="large" @click="nextStep">下一步
                            <el-icon>
                                <ArrowRight />
                            </el-icon>
                        </el-button>
                        <el-button type="danger" size="large" icon="Refresh" @click="reset">重新录入</el-button>
                        <el-button type="warning" size="large" icon="Promotion" @click="jump2last">跳过后续步骤</el-button>
                    </el-row>
                </div>
                <div class="step" :class="{ active: step === 3 }" key="step3">
                    <el-row>
                        <el-col :span="24">
                            <el-form-item size="large" label="瑕疵名称">
                                <div class="input-btn-row">
                                    <el-input size="large" v-model="flawInput" @input="searchColor"
                                        placeholder="请输名称首字母或者名称" maxlength="15" show-word-limit />
                                    <el-button size="large" v-if="showAddFlawBtn" type="primary" icon="Plus"
                                        @click="addTag('001', flawInput)">新增</el-button>
                                </div>
                            </el-form-item>
                        </el-col>
                    </el-row>
                    <!-- 展示瑕疵 -->
                    <el-scrollbar class="scrollbar-height">
                        <CheckboxGroup class="items-break" v-model="form.clothingFlawArr">
                            <CheckboxButton v-for="item in tagsStore.flawList" :key="item.tagId" :value="item.tagId">
                                <el-tooltip :content="item.tagNumber">
                                    {{ item.tagName }}
                                </el-tooltip>
                            </CheckboxButton>
                        </CheckboxGroup>
                    </el-scrollbar>
                    <el-row class="footer-btn">
                        <el-button type="primary" size="large" color="#f5f7fa" icon="ArrowLeft"
                            @click="preStep">上一步</el-button>
                        <el-button type="primary" size="large" @click="nextStep">下一步
                            <el-icon>
                                <ArrowRight />
                            </el-icon>
                        </el-button>
                        <el-button type="danger" size="large" icon="Refresh" @click="reset">重新录入</el-button>
                        <el-button type="warning" size="large" icon="Promotion" @click="jump2last">跳过后续步骤</el-button>
                    </el-row>
                </div>
                <div class="step" :class="{ active: step === 4 }" key="step4">
                    <el-row>
                        <el-col :span="24">
                            <el-form-item size="large" label="洗后预估">
                                <div class="input-btn-row">
                                    <el-input size="large" v-model="estimateInput" @input="searchColor"
                                        placeholder="请输名称首字母或者名称" maxlength="15" show-word-limit />
                                    <el-button size="large" v-if="showAddEstimateBtn" type="primary" icon="Plus"
                                        @click="addTag('002', estimateInput)">新增</el-button>
                                </div>
                            </el-form-item>
                        </el-col>
                    </el-row>
                    <!-- 展示洗后预估标签 -->
                    <el-scrollbar class="scrollbar-height">
                        <CheckboxGroup class="items-break" v-model="form.estimateArr">
                            <CheckboxButton v-for="item in tagsStore.estimateList" :key="item.tagId" :value="item.tagId">
                                <el-tooltip :content="item.tagNumber">
                                    {{ item.tagName }}
                                </el-tooltip>
                            </CheckboxButton>
                        </CheckboxGroup>
                    </el-scrollbar>
                    <el-row class="footer-btn">
                        <el-button type="primary" size="large" color="#f5f7fa" icon="ArrowLeft"
                            @click="preStep">上一步</el-button>
                        <el-button type="primary" size="large" @click="nextStep">下一步
                            <el-icon>
                                <ArrowRight />
                            </el-icon>
                        </el-button>
                        <el-button type="danger" size="large" icon="Refresh" @click="reset">重新录入</el-button>
                        <el-button type="warning" size="large" icon="Promotion" @click="jump2last">跳过后续步骤</el-button>
                    </el-row>
                </div>
                <div class="step" :class="{ active: step === 5 }" key="step5">
                    <el-row>
                        <el-col :span="24">
                            <el-form-item size="large" label="品牌名称">
                                <div class="input-btn-row">
                                    <el-input size="large" v-model="brandInput" @input="searchColor"
                                        placeholder="请输品牌名称首字母或者品牌名称" maxlength="20" show-word-limit />
                                    <el-button size="large" v-if="showAddBrandBtn" type="primary" icon="Plus"
                                        @click="addTag('004', brandInput)">新增</el-button>
                                </div>
                            </el-form-item>
                        </el-col>
                    </el-row>
                    <!-- 展示品牌 -->
                    <el-scrollbar>
                        <div class="items-break">
                            <RadioButton v-for="brand in tagsStore.brandList" v-model="form.clothingBrand" :key="brand.tagId"
                                :value="brand.tagId" @change="nextStep" :label="brand.tagName" />
                        </div>
                    </el-scrollbar>
                    <el-row class="footer-btn">
                        <el-button type="primary" size="large" color="#f5f7fa" icon="ArrowLeft"
                            @click="preStep">上一步</el-button>
                        <el-button type="primary" size="large" @click="nextStep">下一步
                            <el-icon>
                                <ArrowRight />
                            </el-icon>
                        </el-button>
                        <el-button type="danger" size="large" icon="Refresh" @click="reset">重新录入</el-button>
                        <el-button type="warning" size="large" icon="Promotion" @click="jump2last">跳过后续步骤</el-button>
                    </el-row>
                </div>
                <div class="step step6" :class="{ active: step === 6 }" key="step6">
                    <div class="content-container">
                        <div class="content-inner">
                            <div class="section-title">服务类型</div>
                            <el-radio-group v-model="form.serviceType" class="step6-card">
                                <template v-for="dict in sys_service_type" :key="dict.value">
                                    <el-radio v-if="dict.value !== '03' && dict.value !== '04'" :value="dict.value"
                                        class="payment-method-radio">
                                        <div class="payment-method-card"
                                            :class="{ 'selected': form.serviceType === dict.value }">
                                            <el-icon v-if="dict.value === '000'">
                                                <TakeawayBox />
                                            </el-icon>
                                            <el-icon v-else-if="dict.value === '001'">
                                                <HotWater />
                                            </el-icon>
                                            <el-icon v-else-if="dict.value === '002'">
                                                <Discount />
                                            </el-icon>
                                            <el-icon v-else-if="dict.value === '003'">
                                                <More />
                                            </el-icon>
                                            <span>{{ dict.label }}</span>
                                        </div>
                                    </el-radio>
                                </template>
                            </el-radio-group>
                            <div class="section-title">服务要求</div>
                            <el-radio-group v-model="form.serviceRequirement" class="step6-card">
                                <el-radio v-for="dict in sys_service_requirement" :key="dict.value" :value="dict.value"
                                    class="payment-method-radio" :label="dict.label">
                                    <div class="payment-method-card"
                                        :class="{ 'selected': form.serviceRequirement === dict.value }">
                                        <el-icon v-if="dict.value === '000'">
                                            <CircleCheck />
                                        </el-icon>
                                        <el-icon v-else-if="dict.value === '001'">
                                            <AlarmClock />
                                        </el-icon>
                                        <el-icon v-else-if="dict.value === '002'">
                                            <Box />
                                        </el-icon>
                                        <el-icon v-else-if="dict.value === '003'">
                                            <More />
                                        </el-icon>
                                        <span>{{ dict.label }}</span>
                                    </div>
                                </el-radio>
                            </el-radio-group>
                            <div class="section-title">工艺加价</div>
                            <div class="process-markup">
                                <el-input-number style="width: 14rem;" size="large" v-model="form.processMarkup"
                                    :min="0" :max="6000" :precision="2" />元
                            </div>
                            <div class="section-title">备注信息</div>
                            <div class="step6-card">
                                <textarea v-model="form.remark"
                                    style="flex-grow: 1; outline: none; border: 1px solid #d1d1d1; padding: .5rem; font-size: large; border-radius: .4rem;"
                                    rows="5" placeholder="点击输入备注信息" maxlength="200"></textarea>
                            </div>
                            <div class="pictures">
                                <div class="pictures-title">
                                    <div class="section-title">洗前图片</div>
                                    <el-upload action="#" :auto-upload="false" :on-change="handleFileChange"
                                        :show-file-list="false">
                                        <template #trigger>
                                            <el-button type="primary" plain icon="Picture">
                                                选择图片</el-button>
                                        </template>
                                    </el-upload>
                                </div>
                                <!-- 展示刚才上传的图片，并且支持删除 -->
                                <div class="image-list info-card" v-if="images.length > 0">
                                    <div v-for="(image, index) in images" :key="index" class="image-item">
                                        <img :src="image.url" alt="Uploaded Image" class="image-preview" />
                                        <el-button type="danger" :icon="Close" size="small" circle
                                            @click="removeImage(index)" class="delete-button" />
                                    </div>
                                </div>
                                <div class="info-card empty-info-card" v-else>
                                    <div class="empty-state">
                                        <el-empty description="您还没有上传任何照片哦~" :image-size="80"></el-empty>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="footer-btn">
                        <el-button type="danger" size="large" icon="Close" @click="reset">取消</el-button>
                        <el-button type="primary" size="large" icon="Camera" @click="openCamera">拍照留档</el-button>
                        <el-button type="success" size="large" icon="CircleCheck" @click="submitForm">
                            {{ form.clothId ? '确认修改' : '确认' }}
                        </el-button>
                    </div>
                </div>

            </div>
        </el-form>
        <!-- Camera Modal -->
        <el-dialog title="拍照留档" :align-center="true" v-model="showCameraModal" width="800px" append-to-body>
            <video ref="video" class="video" autoplay></video>
            <canvas ref="canvas" class="canvas"></canvas>
            <div class="image-list">
                <div v-for="(image, index) in capturedImages" :key="index" class="image-item">
                    <img :src="image.url" alt="Captured Image" class="image-preview" />
                    <el-button type="danger" :icon="Close" size="small" circle @click="removeCapturedImage(index)"
                        class="delete-button" />
                </div>
            </div>
            <div class="camera-controls">
                <el-button type="primary" @click="capturePhoto" :disabled="capturedImages.length >= 16">拍照</el-button>
                <el-button type="primary" @click="savePhotos" :disabled="capturedImages.length === 0">保存</el-button>
                <el-button @click="closeCamera">关闭</el-button>
            </div>

        </el-dialog>
        <PredefinedCategories v-model="showCategoryDialog" @success="handlePredefinedCategoriesSuccess"
            @cancel="showCategoryDialog = false" />
        <el-dialog v-model="showCategoryPrompt" title="提示" width="30%" :close-on-click-modal="false">
            <div class="prompt-content">
                <el-icon class="prompt-icon">
                    <Warning />
                </el-icon>
                <span>当前没有分类数据，是否添加预定义分类？</span>
            </div>
            <template #footer>
                <span class="dialog-footer">
                    <el-button @click="handleCategoryPrompt(false)">暂不添加</el-button>
                    <el-button type="primary" @click="handleCategoryPrompt(true)">
                        添加预定义分类
                    </el-button>
                </span>
            </template>
        </el-dialog>
        <RackInitCheck v-model:visible="showRackInitCheck" @setup-complete="handleRackSetupComplete" />
    </div>
</template>

<script setup name="AddCloth">
import { addCloths, updateCloths } from "@/api/system/cloths";
import { CoffeeCup, CollectionTag, CopyDocument, PictureRounded, User, WarningFilled } from "@element-plus/icons-vue";
import { listClothingWithNoLimit, createClothingCreateOrder } from "@/api/system/clothing";
import { listTagsNoLimit, addTags } from "@/api/system/tags";
import { listCategoryAll } from "@/api/system/clothingCategory";
import { listStyleByCategoryId, addStyle } from "@/api/system/clothingStyle";
import pinyin from 'pinyin';
import { ref, reactive, toRefs, nextTick } from "vue";
import CheckboxGroup from "@/components/CheckBoxGroup.vue";
import CheckboxButton from '@/components/CheckboxButton.vue';
import { ElMessage } from 'element-plus';
import { invoke } from '@tauri-apps/api/core';
import { Close } from '@element-plus/icons-vue';
import RadioButton from '@/components/RadioButton.vue'
import PredefinedCategories from './PredefinedCategories.vue'
import { checkRackInitialized } from '@/api/system/rackCheck';
import RackInitCheck from '@/components/RackInitCheck/index.vue';
import { onMounted, inject } from 'vue';
import eventBus from "@/utils/eventBus";
import useTagsStore from '@/store/modules/tags';


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
    value: {
        type: Array,
    },
    submit: {
        type: Function,
        required: true,
    },
    disabled: {
        type: Boolean,
        default: false
    },
    isRewash: {
        type: Boolean,
        default: false
    },
    clothes: {
        type: Array,
        default: []
    }
});

const selectedCloth = inject('selectedCloth');
const { proxy } = getCurrentInstance();
const { sys_service_type, sys_service_requirement } =
    proxy.useDict(
        "sys_service_type",
        "sys_service_requirement"
    );

const tagsStore = useTagsStore();
const featureList = [
    { get value() { return tagsStore.colorList } },
    { get value() { return tagsStore.flawList } },
    { get value() { return tagsStore.estimateList } },
    { get value() { return tagsStore.brandList } }
];

// 衣物品类列表
const categoryList = ref([]);
// 步数
const maxStepNum = 6;
// 添加衣物的列表
const clothList = ref([]);

// 选择衣物时展示的衣物列表
const clothingList = ref([]);
const clothingListFilterResult = ref([]);
const clothStyleList = ref([]);
// 该用户洗过的衣物历史记录
const showUploadPicture = ref(false);
const showPriceContent = ref(false);
const showAddClothBtn = ref(false);
const showAddColorBtn = ref(false);
const showAddFlawBtn = ref(false);
const showAddEstimateBtn = ref(false);
const showAddBrandBtn = ref(false);
const showHistory = ref(false);
const clothListloading = ref(false);
const step = ref(0);
const clothNameInput = ref(null);
const clothColorInput = ref(null);
const flawInput = ref(null);
const estimateInput = ref(null);
const brandInput = ref(null);

// 弹窗控制
const showCategoryDialog = ref(false);
const showCategoryPrompt = ref(false);

// 衣挂初始化检查相关状态
const showRackInitCheck = ref(false);

// 添加这些缺少的变量声明
const currentCloth = ref();
const cateName = ref();
const clothNameRef = ref();
const prePictureList2 = ref(new Set()); // 洗前图片

// 自定义校验最低价格函数
function validateMinPrice(rule, value, callback) {
    if (value && Number(value) > Number(form.value.clothInfo.clothingBasePrice)) {
        callback(new Error("最低价格不能超过基准价格"));
    } else {
        callback();
    }
};

const data = reactive({
    form: {},
    rules: {
        categoryId: [
            { required: true, message: "衣物品类不能为空", trigger: "blur" }
        ],
        styleId: [
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
        ],
        'clothInfo.clothingMinPrice': [
            { validator: validateMinPrice, trigger: 'blur' }
        ]
    }
});

const { form, rules } = toRefs(data);
const images = ref([]); // 用于存储上传的图片
const showCameraModal = ref(false); // 是否显示拍照对话框
const video = ref(null);
const canvas = ref(null);
const capturedImages = ref([]);

// 监听 selectedCloth 的变化
watch(selectedCloth, (newVal) => {
    if (newVal) {
        Object.assign(form.value, newVal);
        step.value = 0;
        cateChange();
    } else {
        reset(); // 如果没有选中任何衣物，则重置表单
    }
});
const transitionName = ref('slide-right');

watch(step, (newStep, oldStep) => {
    if (newStep > oldStep) {
        transitionName.value = 'slide-left';
    } else {
        transitionName.value = 'slide-right';
    }
});
// 打开摄像头
const openCamera = async () => {
    showCameraModal.value = true;
    try {
        const devices = await navigator.mediaDevices.enumerateDevices();
        console.log('devices:', devices);
        const videoDevices = devices.filter(device => device.kind === 'videoinput');
        if (videoDevices.length === 0) {
            proxy.notify.error('没有可用的摄像头设备');
            return;
        }
        const stream = await navigator.mediaDevices.getUserMedia({
            video: {
                deviceId: videoDevices[0].deviceId,
                width: { ideal: 1920 },
                height: { ideal: 1080 }
            }
        });
        video.value.srcObject = stream;
    } catch (error) {
        proxy.notify.error('无法访问摄像头: ' + error);
        console.error('无法访问摄像头:', error);
        try {
            const stream = await navigator.mediaDevices.getDisplayMedia({
                video: true
            });
            video.value.srcObject = stream;
        } catch (error) {
            proxy.notify.error('无法访问桌面: ' + error);
            console.error('无法访问桌面:', error);
        }
    }
};

// 关闭摄像头
const closeCamera = () => {
    showCameraModal.value = false;
    const stream = video.value.srcObject;
    const tracks = stream.getTracks();
    tracks.forEach(track => track.stop());
    video.value.srcObject = null;
    capturedImages.value = [];
};

// 拍照
const capturePhoto = () => {
    const context = canvas.value.getContext('2d');
    canvas.value.width = 1920;
    canvas.value.height = 1080;
    context.drawImage(video.value, 0, 0, canvas.value.width, canvas.value.height);
    const dataUrl = canvas.value.toDataURL('image/png');
    capturedImages.value.push({ url: dataUrl });
};

// 删除拍照的图片
const removeCapturedImage = (index) => {
    capturedImages.value.splice(index, 1);
};

// 保存照片
const savePhotos = async () => {
    for (const image of capturedImages.value) {
        const blob = await (await fetch(image.url)).blob();
        const file = new File([blob], 'photo.png', { type: 'image/png' });
        await handleFileChange({ raw: file });
    }
    closeCamera();
};

// 处理文件选择
const handleFileChange = async (file) => {
    try {
        // 读取文件为 ArrayBuffer
        const fileName = file.raw.name;
        const arrayBuffer = await file.raw.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);

        // 调用 Tauri 后端命令保存图片
        const result = await invoke('save_image', {
            name: fileName,
            data: Array.from(uint8Array),
        });

        // 将图片添加到展示列表
        const imageUrl = URL.createObjectURL(file.raw); // 生成图片的临时 URL
        images.value.push({ url: imageUrl, path: result.path, id: result.id });

        prePictureList2.value.add(result.id);
        ElMessage.success('图片保存成功');

    } catch (error) {
        // 提示保存失败
        ElMessage.error('图片保存失败' + error);
    }
};

// 删除图片
const removeImage = async (index) => {
    try {
        const image = images.value[index];
        // 调用 Tauri 后端命令删除图片文件
        await invoke('delete_image', { id: image.id });

        // 从展示列表中移除图片
        images.value.splice(index, 1);

        // 提示删除成功
        ElMessage.success('图片已删除');
    } catch (error) {
        // 提示删除失败
        ElMessage.error(`删除失败: ${error}`);
    }
};

function jumpToStep(stepNum) {
    if (stepNum == 0) {
        step.value = stepNum;
        return
    }
    if (stepNum < 0 || stepNum > maxStepNum) {
        return;
    }
    // 检查从其他步骤跳转到步骤1时，是否已选择了样式
    if (stepNum == 1 && !form.value.styleId) {
        proxy.notify.error("请先选择衣物类别");
        return;
    }
    if (stepNum != 1 && !form.value.clothingId) {
        proxy.notify.error("请先选择衣物");
        return;
    }
    step.value = stepNum;
}

// 处理添加分类按钮点击
async function handleAddCate() {
    if (!cateName.value || cateName.value == "") {
        proxy.notify.error("请输入分类名称");
        return;
    }

    if (!form.value.categoryId) {
        proxy.notify.error("请先选择品类");
        return;
    }

    // 检查是否已有分类数据
    if (categoryList.value.length === 0) {
        showCategoryPrompt.value = true;
        return;
    }

    // 创建新的分类
    const style = {
        categoryId: form.value.categoryId,
        styleName: cateName.value,
        styleCode: "", // 后端会自动生成
        orderNum: 0
    };

    addStyle(style).then(() => {
        proxy.notify.success("添加成功");
        cateChange();
        cateName.value = "";
    });
}

// 处理预定义分类添加成功
function handlePredefinedCategoriesSuccess() {
    // 刷新分类列表
    listCategoryAll().then(response => {
        categoryList.value = response;
        cateChange();
    });
}

// 当品类发生变化时动态查询子分类列表
function cateChange() {
    if (form.value.categoryId) {
        listStyleByCategoryId(form.value.categoryId).then(res => {
            clothStyleList.value = res.map(item => ({
                dictValue: item.styleId,
                dictLabel: item.styleName
            }));
        })
    }
}

// 表单重置
function reset() {
    form.value = {
        clothInfo: {},
        orderId: null,
        clothingId: null,
        categoryId: categoryList.value.length > 0 ? categoryList.value[0].categoryId : null,
        styleId: null,
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
    clothNameInput.value = null;
    showAddBrandBtn.value = false;
    showAddColorBtn.value = false;
    showAddFlawBtn.value = false;
    showAddEstimateBtn.value = false;
    showAddClothBtn.value = false;
    showHistory.value = false;
    prePictureList2.value.clear();
    images.value = [];
    proxy.resetForm("clothsRef");
    if (form.value.categoryId) {
        cateChange();
    }
}

/* 初始化列表数据 */
async function initList() {
    const promises = [];

    // 获取品类列表
    if (categoryList.value.length === 0) {
        const categoryPromise = listCategoryAll().then(response => {
            categoryList.value = response;
            // 如果品类列表为空，显示提示对话框
            if (categoryList.value.length === 0) {
                showCategoryPrompt.value = true;
            }
        });
        promises.push(categoryPromise);
    }

    // 获取衣物列表
    if (clothingList.value.length === 0) {
        const clothingPromise = listClothingWithNoLimit().then(response => {
            clothingList.value = response;
        });
        promises.push(clothingPromise);
    }

    // 使用store初始化标签数据
    promises.push(tagsStore.initTags());

    // 等待所有异步操作完成
    await Promise.all(promises);
}

/** 新增按钮操作 */
function handleAdd() {
    reset();
    // 如果有品类数据，初始化分类列表
    if (form.value.categoryId) {
        cateChange();
    }
}

/** 提交按钮 */
function submitForm() {
    proxy.$refs["clothsRef"].validate(valid => {
        if (valid) {
            form.value.clothingId = Number(form.value.clothingId);
            if (form.value.clothingColor) {
                form.value.clothingColor = Number(form.value.clothingColor);
            }
            if (form.value.clothingBrand) {
                form.value.clothingBrand = Number(form.value.clothingBrand);
            }
            if (form.value.clothingFlawArr) {
                form.value.clothingFlawArr = form.value.clothingFlawArr.map(item => Number(item));
            }
            if (form.value.estimateArr) {
                form.value.estimateArr = form.value.estimateArr.map(item => Number(item));
            }
            const submitData = { ...form.value };
            if (submitData.estimateArr) {
                submitData.estimate = submitData.estimateArr.join(',');
                delete submitData.estimateArr;
            }
            if (submitData.clothingFlawArr) {
                submitData.clothingFlaw = submitData.clothingFlawArr.join(',');
                delete submitData.clothingFlawArr;
            }
            if (prePictureList2.value.size > 0) {
                submitData.beforePics = Array.from(prePictureList2.value).join(',');
            }
            if (form.value.clothId != null) {
                console.log(clothList.value, form.value)
                updateCloths(submitData).then(() => {
                    proxy.notify.success("修改成功");
                    // 更新衣物列表
                    const clothIndex = clothList.value.findIndex(item => item.clothId == form.value.clothId);
                    if (clothIndex !== -1) {
                        const cloth = clothList.value[clothIndex];
                        const clothInfo = cloth.clothInfo;
                        clothList.value[clothIndex] = form.value; // 替换整个对象
                        clothList.value[clothIndex].clothInfo = clothInfo;
                    }
                    props.submit(clothList.value);
                    handleAdd();
                }).catch(error => {
                    // 如果是notfound那么检查是否配置了晾衣架
                    if (error.kind == "NotFound" && error.details == '没有可用的衣架') {
                        // 检查衣挂是否已初始化
                        checkRackInitialized().then(isInitialized => {
                            if (!isInitialized) {
                                // 如果未初始化，显示衣挂初始化设置对话框
                                showRackInitCheck.value = true;
                            } else {
                                proxy.notify.error("没有可用的衣架，请检查衣架配置");
                            }
                        }).catch(err => {
                            console.error('检查衣挂初始化状态失败:', err);
                            proxy.notify.error("检查衣挂状态失败");
                        });
                        return;
                    }
                    proxy.notify.error("操作失败：" + error);
                });
            } else {
                if (props.orderId) {
                    submitData.orderId = props.orderId;
                }
                addCloths(submitData).then(response => {
                    proxy.notify.success("新增成功");
                    const flaw = form.value.clothingFlawArr;
                    const estimate = form.value.estimateArr;
                    form.value = response;
                    form.value.clothingFlawArr = flaw;
                    form.value.estimateArr = estimate;
                    form.value.clothInfo = clothingList.value.find(item => item.id == submitData.clothingId);
                    clothList.value.push(form.value);
                    props.submit(clothList.value);
                    handleAdd();
                }).catch(error => {
                    // 如果是notfound那么检查是否配置了晾衣架
                    if (error.kind == "NotFound" && error.details == '没有可用的衣架') {
                        // 检查衣挂是否已初始化
                        checkRackInitialized().then(isInitialized => {
                            if (!isInitialized) {
                                // 如果未初始化，显示衣挂初始化设置对话框
                                showRackInitCheck.value = true;
                            } else {
                                proxy.notify.error("没有可用的衣架，请检查衣架配置");
                            }
                        }).catch(err => {
                            console.error('检查衣挂初始化状态失败:', err);
                            proxy.notify.error("检查衣挂状态失败");
                        });
                        return;
                    }
                    proxy.notify.error("操作失败：" + error);
                });
            }
        }
    });
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

    // 如果需要添加标签但未添加，则提示用户先添加标签
    if (step.value === 2 && showAddColorBtn.value) {
        proxy.notify.warning("请先添加颜色标签");
        return;
    } else if (step.value === 3 && showAddFlawBtn.value) {
        proxy.notify.warning("请先添加瑕疵标签");
        return;
    } else if (step.value === 4 && showAddEstimateBtn.value) {
        proxy.notify.warning("请先添加预估标签");
        return;
    } else if (step.value === 5 && showAddBrandBtn.value) {
        proxy.notify.warning("请先添加品牌标签");
        return;
    }

    if (step.value !== maxStepNum) {
        step.value++;
    }

    if (step.value === 1) {
        clothingListFilterResult.value = clothingList.value.filter(item =>
            item.categoryId === form.value.categoryId
            && item.styleId === form.value.styleId);
        console.log(form.value)
        console.log(clothingListFilterResult.value)
    }
    if (step.value === 1 && clothNameRef.value) {
        clothNameRef.value.focus();
    }

}

/* 跳过后续步骤 */
function jump2last() {
    step.value = maxStepNum;
}

/* 获取衣物列表 */
async function getClothingList() {
    clothListloading.value = true;
    listClothingWithNoLimit().then(res => {
        clothingList.value = res.rows;
        clothListloading.value = false;
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

/* 搜索衣物 */
function searchCloth(color) {
    const upperCaseColor = color.trim().toUpperCase();
    if (upperCaseColor === '') {
        return;
    }

    // 颜色、瑕疵、洗后预估、品牌是从第3步开始渲染的，因此要-2
    const item = clothingListFilterResult.value.find(item => {
        return item.title.includes(upperCaseColor) || getPinyinInitials(item.title).includes(upperCaseColor);
    });

    if (!item) {
        showAddClothBtn.value = true;
        form.value.clothingColor = null;
    } else {
        form.value.clothingId = item.id;
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
    switch (index) {
        case 0:
            const item = featureList[index].value.find(item => {
                return item.tagName.includes(upperCaseColor) || getPinyinInitials(item.tagName).includes(upperCaseColor);
            });
            if (!item) {
                showAddColorBtn.value = true;
                form.value.clothingColor = null;
            } else {
                form.value.clothingColor = item.tagId;
                showAddColorBtn.value = false;
            }
            break;
        case 1:
            const item1 = featureList[index].value.filter(item => {
                return item.tagName.includes(upperCaseColor) || getPinyinInitials(item.tagName).includes(upperCaseColor);
            });
            if (item1.length === 0) {
                showAddFlawBtn.value = true;
                form.value.clothingFlaw = null;
            } else {
                // 确保使用字符串类型的值以匹配CheckboxButton中的实现
                form.value.clothingFlawArr = item1.map(item => String(item.tagId));
                showAddFlawBtn.value = false;
            }
            break;
        case 2:
            const item2 = featureList[index].value.filter(item => {
                return item.tagName.includes(upperCaseColor) || getPinyinInitials(item.tagName).includes(upperCaseColor);
            });
            if (item2.length === 0) {
                showAddEstimateBtn.value = true;
                form.value.estimate = null;
            } else {
                // 确保使用字符串类型的值以匹配CheckboxButton中的实现
                form.value.estimateArr = item2.map(item => String(item.tagId));
                showAddEstimateBtn.value = false;
            }
            break;
        case 3:
            const item3 = featureList[index].value.find(item => {
                return item.tagName.includes(upperCaseColor) || getPinyinInitials(item.tagName).includes(upperCaseColor);
            });
            if (!item3) {
                showAddBrandBtn.value = true;
                form.value.clothingBrand = null;
            } else {
                form.value.clothingBrand = item3.tagId;
                showAddBrandBtn.value = false;
            }
            break;
        default: ;
    }
}

/* 显示添加衣物按钮 */
function handleAddCloth() {
    showPriceContent.value = true;
    form.value.clothInfo = {
        hangType: '1'
    };
}

function createCloth() {
    if (!clothNameInput.value || clothNameInput.value.length === 0) {
        proxy.notify.error("请输入衣物名称");
        return;
    }
    const data = form.value.clothInfo;
    if (!data.clothingBasePrice) {
        proxy.notify.error("请输入衣物价格");
        return;
    }

    // 验证最低价格不能超过基准价格
    if (data.clothingMinPrice && Number(data.clothingMinPrice) > Number(data.clothingBasePrice)) {
        proxy.notify.error("最低价格不能超过基准价格");
        return;
    }

    data.clothingMinPrice = data.clothingMinPrice || data.clothingBasePrice;
    data.categoryId = form.value.categoryId;
    data.styleId = form.value.styleId;
    data.title = clothNameInput.value;

    createClothingCreateOrder(data).then(async response => {
        proxy.notify.success("新增衣物成功");
        
        // 创建一个完整的衣物对象
        const newCloth = {
            id: response.id,
            categoryId: data.categoryId,
            styleId: data.styleId,
            title: data.title,
            clothingBasePrice: data.clothingBasePrice,
            clothingMinPrice: data.clothingMinPrice,
            hangType: data.hangType || '1'
        };
        
        // 清理状态
        showPriceContent.value = false;
        showAddClothBtn.value = false;
        clothNameInput.value = null;
        
        // 更新表单值
        form.value.clothingId = newCloth.id;
        form.value.priceValue = newCloth.clothingBasePrice;
        form.value.hangType = newCloth.hangType;
        form.value.clothInfo = {};
        
        // 更新衣物列表
        clothingListFilterResult.value.push(newCloth);
        clothingList.value.push(newCloth);
        
        // 确保新添加的衣物被选中 - 强制触发一次更新
        nextTick(() => {
            // 这里添加一个小延迟，确保UI能正确反映选中状态
            setTimeout(() => {
                const selectedCloth = clothingListFilterResult.value.find(item => item.id === newCloth.id);
                if (selectedCloth) {
                    console.log("已选中衣物:", selectedCloth.title);
                }
            }, 100);
        });
        
        // 自动进入下一步
        nextStep();
    }).catch(error => {
        // 错误处理（保持不变）
        if (error.kind == "notfound" && error.details == '没有可用的衣架') {
            checkRackInitialized().then(isInitialized => {
                if (!isInitialized) {
                    showRackInitCheck.value = true;
                } else {
                    proxy.notify.error("没有可用的衣架，请检查衣架配置");
                }
            }).catch(err => {
                console.error('检查衣挂初始化状态失败:', err);
                proxy.notify.error("检查衣挂状态失败");
            });
            return;
        }
        proxy.notify.error("操作失败：" + error);
    });
}

/* 新增标签 */
async function addTag(type, tagName) {
    try {
        const tagsStore = useTagsStore();
        const newTag = await tagsStore.addTag(type, tagName);
        proxy.notify.success("新增成功");
        
        // 根据标签类型进行选中
        switch (type) {
            case "001": // 瑕疵
                // 确保使用字符串类型的值以匹配CheckboxButton中的实现
                form.value.clothingFlawArr = [String(newTag.tagId)];
                showAddFlawBtn.value = false;
                flawInput.value = '';
                break;
            case "002": // 预估
                // 确保使用字符串类型的值以匹配CheckboxButton中的实现 
                form.value.estimateArr = [String(newTag.tagId)];
                showAddEstimateBtn.value = false;
                estimateInput.value = '';
                break;
            case "003": // 颜色
                form.value.clothingColor = newTag.tagId;
                showAddColorBtn.value = false;
                clothColorInput.value = '';
                break;
            case "004": // 品牌
                form.value.clothingBrand = newTag.tagId;
                showAddBrandBtn.value = false;
                brandInput.value = '';
                break;
        }
        
        // 打印表单状态以便调试
        console.log("新增标签:", type, "tagId:", newTag.tagId, "选中状态:", type === "001" ? form.value.clothingFlawArr : (type === "002" ? form.value.estimateArr : null));
        
        // 确保UI更新
        nextTick(() => {
            // 这里通过强制重新渲染来确保选中状态显示
            if (type === "001" || type === "002") {
                const temp = [...(type === "001" ? form.value.clothingFlawArr : form.value.estimateArr)];
                if (type === "001") {
                    form.value.clothingFlawArr = [];
                    setTimeout(() => {
                        form.value.clothingFlawArr = temp;
                    }, 0);
                } else {
                    form.value.estimateArr = [];
                    setTimeout(() => {
                        form.value.estimateArr = temp;
                    }, 0);
                }
            }
        });
    } catch (error) {
        proxy.notify.error("添加标签失败: " + error);
    }
}

/* 衣物发生变化时要将最后一步的价格设置为选中衣物中的价格 */
function step2ClothChange() {
    if (form.value.clothingId) {
        const cloth = clothingList.value.find(item => item.id == form.value.clothingId);
        form.value.priceValue = cloth.clothingBasePrice;
        form.value.hangType = cloth.hangType;
        nextStep();
    }
}

// 处理用户选择是否添加预定义分类
function handleCategoryPrompt(confirm) {
    showCategoryPrompt.value = false;
    if (confirm) {
        showCategoryDialog.value = true;
    }
}

// 处理衣挂初始化检查完成
const handleRackSetupComplete = (completed) => {
    showRackInitCheck.value = false;
    if (completed) {
        // 如果完成了初始化，可以继续之前的操作
        proxy.notify.success("衣挂配置完成");
    }
};

// 监听衣物删除事件
onMounted(async () => {
    await initList();  // 确保 initList 完成
    handleAdd();

    // 监听衣物删除事件
    eventBus.on('cloth-deleted', (clothId) => {
        // 如果当前正在编辑被删除的衣物，则重置表单
        if (form.value.clothId === clothId) {
            reset();
        }
    });
});
</script>
<style></style>
<style scoped>
.app-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1.5rem;
}

.el-steps--simple {
    padding: 1rem;
}

.step-item {
    cursor: pointer;
    font-size: xx-small;
}

.form-container {
    height: 100%;
}

.steps-container {
    width: 100%;
    height: 100%;
    position: relative;
    overflow: hidden;
}

.step {
    width: 100%;
    height: 100%;
    position: absolute;
    top: 0;
    left: 0;
    opacity: 0;
    visibility: hidden;
    transform: translateX(-100%);
    transition: all 0.3s ease-in-out;
}

.step.active {
    opacity: 1;
    visibility: visible;
    transform: translateX(0);
}

.step0 {
    height: 100%;
    display: flex;
}

.step6 {
    height: 100%;
    display: flex;
    flex-direction: column;
}

.step6>.content-container {
    flex: 1;
    overflow-y: auto;
    /* 下面是解决overflow导致的阴影消失问题 */
    padding: 0 20px;
    /* 增加水平内边距 */
    margin: 0 -20px;
    /* 负外边距抵消宽度变化 */
    clip-path: none !important;
    /* 禁用潜在剪切路径 */
    /* 使表单区域可滚动 */
    margin-bottom: 1.25rem;

    /* 隐藏滚动条 */
    &::-webkit-scrollbar {
        display: none;
    }
}

.content-inner {
    padding: 0 .5rem;
    /* 创建安全间距 */
    position: relative;
    /* 修复阴影层级 */
    z-index: 1;
    /* 提升渲染层级 */
}

.scrollbar-height {
    height: calc(100% - 5.5rem);
}

.items-break {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: flex-start;
    align-items: flex-start;
    gap: 1rem;
    flex-wrap: wrap;
}

.radio-group-column {
    height: 100%;
    display: flex;
    flex-direction: column;
    flex-wrap: nowrap;
    gap: 1rem;
    overflow-y: auto;

}

.radio-button-column {
    width: 6rem;
    height: 6rem;
}

.section-title {
    position: relative;
    text-align: left;
    font-size: 1rem;
    font-weight: 600;
    margin: 1rem 0 12px 0;
    color: var(--el-color-primary-dark-2);
}

.section-title::after {
    content: '';
    position: absolute;
    left: 0;
    bottom: -5px;
    width: 4rem;
    height: 3px;
    background-color: var(--el-color-primary);
    border-radius: 3px;
}

.payment-method-section {
    background-color: var(--el-fill-color);
    border-radius: 8px;
    padding: 1rem;
    box-shadow: var(--el-box-shadow);
}

.process-markup {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    gap: 1rem;
    border-radius: 8px;
    padding: 1rem;
    box-shadow: var(--el-box-shadow-lighter);
    margin-bottom: 1rem;
}

.step6-card {
    display: flex;
    justify-content: center;
    flex-wrap: wrap;
    gap: 1rem;
    border-radius: 8px;
    padding: 1rem;
    box-shadow: var(--el-box-shadow-lighter);
    margin-bottom: 1rem;
    transition: all 0.3s;
}

.process-markup:hover,
.step6-card:hover,
.info-card:hover {
    transform: translateY(-2px);
    box-shadow: var(--el-box-shadow-light);
}

.payment-method-radio {
    margin-right: 0 !important;
    height: auto;
}

.payment-method-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 100px;
    height: 80px;
    border-radius: 8px;
    border: 1px solid var(--el-border-color);
    transition: all 0.3s;
    cursor: pointer;
    background-color: var(--el-bg-color-overlay);
}

.payment-method-card:hover {
    border-color: var(--el-color-primary);
    transform: translateY(-2px);
    box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
}

.payment-method-card.selected {
    border-color: var(--el-color-primary);
    background-color: var(--el-fill-color-light);
    box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
}

.payment-method-card .el-icon {
    font-size: 24px;
    margin-bottom: 8px;
    color: var(--el-color-primary);
}

.payment-method-card span {
    font-size: 14px;
}

/* 衣物信息卡片样式 */
.info-card {
    border-radius: 8px;
    padding: 1rem;
    box-shadow: var(--el-box-shadow-lighter);
    transition: all 0.3s ease;
}


/* 空状态样式 */
.empty-state {
    display: flex;
    justify-content: center;
}

.empty-info-card {
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 1rem 0;
}

.pictures {
    margin-top: 20px;
}

.pictures-title {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    gap: 1rem;
}

.image-list {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
    margin-top: 20px;
}

.image-item {
    position: relative;
    border-radius: 4px;
}

.image-preview {
    max-width: 100px;
    max-height: 100px;
    display: block;
}

.delete-button {
    position: absolute;
    top: 5px;
    right: 5px;
    padding: 5px 10px;
    font-size: 12px;
}

.footer-btn {
    width: 100%;
    display: flex;
    justify-content: flex-end;
    align-items: center;
    position: sticky;
    bottom: 0;
    left: 0;
    padding: 0 0 0;
    /* 鼠标穿透 */
    pointer-events: none;

    button {
        transition: all 0.3s;
        pointer-events: all;
    }

    button:hover {
        transform: translateY(-2px);
    }
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

    .price-wrapper {
        width: 100%;
        display: flex;
        justify-content: flex-start;
        gap: .25rem;
    }
}

.item-list-area {
    width: 100%;
    height: 100%;
    overflow-y: auto;
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
    position: relative;

    span {
        width: 1rem;
        height: 1rem;
        text-align: center;
        position: absolute;
        right: 0;
        top: 0;
        display: none;
        cursor: pointer;
        background-color: rgb(8, 253, 171);
        color: rgb(255, 82, 39);
        border-radius: .2rem;
    }

    &:hover {
        span {
            display: block;
        }
    }
}

.video {
    width: 100%;
    height: auto;
}

.canvas {
    display: none;
    position: relative;
}


.camera-controls {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 1rem;
}

.delete-button {
    position: absolute;
    top: 0px;
    right: 0px;
}

.overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    /* background-color: rgba(0, 0, 0, 0.5); */
    z-index: 999;
    display: flex;
    justify-content: center;
    align-items: center;
}
</style>