<template>
    <div class="overlay" v-if="!props.userId">请先选择会员</div>
    <div class="app-container" v-else>
        <!-- 添加或修改订单包含的衣物清单对话框 -->
        <el-steps :active="step" finish-status="success" simple>
            <el-step class="step-item" title="选择品类" :icon="CopyDocument" v-if="step !== maxStepNum"
                @click="jumpToStep(0)" />
            <el-step class="step-item" title="选择衣物" :icon="User" v-if="step !== maxStepNum" @click="jumpToStep(1)" />
            <el-step class="step-item" title="选择颜色" :icon="PictureRounded" v-if="step !== maxStepNum"
                @click="jumpToStep(2)" />
            <el-step class="step-item" title="洗前瑕疵" :icon="WarningFilled" v-if="step !== maxStepNum"
                @click="jumpToStep(3)" />
            <el-step class="step-item" title="洗后预估" :icon="CoffeeCup" v-if="step !== maxStepNum"
                @click="jumpToStep(4)" />
            <el-step class="step-item" title="选择品牌" :icon="CollectionTag" v-if="step !== maxStepNum"
                @click="jumpToStep(5)" />

            <el-step class="step-item" :title="sys_cloth_cate.find(item => item.value == form.clothingCategory).label"
                :icon="CopyDocument" v-if="step == maxStepNum" @click="jumpToStep(0)" />
            <el-step class="step-item" :title="findClothingName()" :icon="User" v-if="step == maxStepNum"
                @click="jumpToStep(1)" />
            <el-step class="step-item" :title="findColorName()" :icon="PictureRounded" v-if="step == maxStepNum"
                @click="jumpToStep(2)" />
            <el-step class="step-item" title="洗前瑕疵" :icon="WarningFilled" v-if="step == maxStepNum"
                @click="jumpToStep(3)" />
            <el-step class="step-item" title="洗后预估" :icon="CoffeeCup" v-if="step == maxStepNum"
                @click="jumpToStep(4)" />
            <el-step class="step-item"
                :title="form.clothingBrand ? brandList.find(item => { return item.tagId == form.clothingBrand }).tagName : '未选择品牌'"
                :icon="CollectionTag" v-if="step == maxStepNum" @click="jumpToStep(5)" />

        </el-steps>
        <el-form ref="clothsRef" :model="form" :rules="rules" class="form-container">

            <div class="wrapper" v-if="step == 0" key="step0">
                <el-col :span="3">
                    <el-scrollbar class="scrollbar-wrapper">
                        <CustomRadioButtonGroup class="radio-group-column" v-model="form.clothingCategory"
                            @change="cateChange">
                            <CustomRadioButton class="radio-button-column" v-for="dict in sys_cloth_cate"
                                :key="dict.value" :value="dict.value">
                                {{ dict.label }}
                            </CustomRadioButton>
                        </CustomRadioButtonGroup>
                    </el-scrollbar>
                </el-col>
                <el-col :span="21">
                    <el-form-item label="">
                        <div class="input-btn-row">
                            <el-input v-model="cateName" placeholder="请输入分类名称" />
                            <el-button type="primary" @click="handleAddCate">新增</el-button>
                        </div>
                    </el-form-item>
                    <el-scrollbar>
                        <CustomRadioButtonGroup class="items-break" v-model="form.clothingStyle" @change="nextStep">
                            <CustomRadioButton v-for="dict in clothStyleList" :key="dict.dictValue"
                                :value="dict.dictValue">
                                {{ dict.dictLabel }}
                            </CustomRadioButton>
                        </CustomRadioButtonGroup>
                    </el-scrollbar>
                </el-col>
                <el-row class="footer-btn">
                    <el-button type="primary" :disabled="!props.userId || !form.clothingCategory || !form.clothingStyle"
                        @click="nextStep">下一步</el-button>
                </el-row>
            </div>
            <div class="step-wrapper" v-if="step == 1" key="step1">
                <el-row>
                    <el-col :span="24">
                        <el-form-item label="衣物名称">
                            <div class="input-btn-row">
                                <el-input v-model="clothNameInput" ref="clothNameRef" @input="searchCloth"
                                    placeholder="请输衣物名称首字母或衣物名称" />
                                <el-button v-if="showAddClothBtn" type="primary" @click="handleAddCloth">新增</el-button>
                            </div>
                        </el-form-item>
                    </el-col>
                </el-row>
                <div v-if="showAddClothBtn && showPriceContent">
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
                                <el-button type="primary" @click="createCloth">确定添加</el-button>
                            </div>
                        </div>
                    </el-form-item>
                    <el-form-item label="衣挂方式">
                        <el-radio-group v-model="form.clothInfo.hangType">
                            <el-radio :value="'1'">输送线</el-radio>
                            <el-radio :value="'2'">其他</el-radio>
                        </el-radio-group>
                    </el-form-item>
                </div>
                <!-- 展示衣物标签 -->
                <el-row class="item-list-area">
                    <el-scrollbar>
                        <CustomRadioButtonGroup class="color-radio-group" v-model="form.clothingId"
                            @change="step2ClothChange">
                            <CustomRadioButton v-for="cloth in clothingListFilterResult" :key="cloth.clothingId"
                                :value="cloth.clothingId">
                                {{ cloth.clothingName }}
                            </CustomRadioButton>
                        </CustomRadioButtonGroup>
                    </el-scrollbar>
                </el-row>
                <el-row class="footer-btn">
                    <el-button type="primary" @click="preStep">上一步</el-button>
                    <el-button type="primary" @click="nextStep" :disabled="!form.clothingId">下一步</el-button>
                    <el-button type="danger" @click="reset">重新录入</el-button>
                </el-row>
            </div>
            <div class="step-wrapper" v-if="step == 2" key="step2">
                <el-row>
                    <el-col :span="24">
                        <el-form-item label="颜色名称">
                            <div class="input-btn-row">
                                <el-input v-model="clothColorInput" @input="searchColor"
                                    placeholder="请输颜色名称首字母或者颜色名称" />
                                <el-button v-if="showAddColorBtn" type="primary"
                                    @click="addTag('003', clothColorInput)">新增</el-button>
                            </div>
                        </el-form-item>
                    </el-col>
                </el-row>
                <!-- 展示颜色 -->
                <el-row class="item-list-area">
                    <el-scrollbar>
                        <CustomRadioButtonGroup class="color-radio-group" v-model="form.clothingColor"
                            @change="step2ClothChange">
                            <CustomRadioButton v-for="color in colorList" :key="color.tagId" :value="color.tagId"
                                :label="color.tagName">
                                <el-tooltip :content="color.tagNumber">
                                    {{ color.tagName }}
                                </el-tooltip>
                            </CustomRadioButton>
                        </CustomRadioButtonGroup>
                    </el-scrollbar>
                </el-row>
                <el-row class="footer-btn">
                    <el-button type="primary" @click="preStep">上一步</el-button>
                    <el-button type="primary" @click="nextStep">下一步</el-button>
                    <el-button type="danger" @click="reset">重新录入</el-button>
                    <el-button type="primary" @click="jump2last">跳过后续步骤</el-button>
                </el-row>
            </div>
            <div style="" v-if="step == 3" key="step3">
                <el-row>
                    <el-col :span="24">
                        <el-form-item label="瑕疵名称">
                            <div class="input-btn-row">
                                <el-input v-model="flawInput" @input="searchColor" placeholder="请输名称首字母或者名称" />
                                <el-button v-if="showAddFlawBtn" type="primary"
                                    @click="addTag('001', flawInput)">新增</el-button>
                            </div>
                        </el-form-item>
                    </el-col>
                </el-row>
                <!-- 展示瑕疵 -->
                <el-row class="item-list-area">
                    <el-scrollbar>
                        <CheckboxGroup class="color-radio-group" v-model="form.clothingFlawArr">
                            <CheckboxButton v-for="item in flawList" :key="item.tagId" :value="item.tagId">
                                <el-tooltip :content="item.tagNumber">
                                    {{ item.tagName }}
                                </el-tooltip>
                            </CheckboxButton>
                        </CheckboxGroup>
                    </el-scrollbar>
                </el-row>
                <el-row class="footer-btn">
                    <el-button type="primary" @click="preStep">上一步</el-button>
                    <el-button type="primary" @click="nextStep">下一步</el-button>
                    <el-button type="danger" @click="reset">重新录入</el-button>
                    <el-button type="primary" @click="jump2last">跳过后续步骤</el-button>
                </el-row>
            </div>
            <div class="step-wrapper" v-if="step == 4" key="step4">
                <el-row>
                    <el-col :span="24">
                        <el-form-item label="洗后预估">
                            <div class="input-btn-row">
                                <el-input v-model="estimateInput" @input="searchColor" placeholder="请输名称首字母或者名称" />
                                <el-button v-if="showAddEstimateBtn" type="primary"
                                    @click="addTag('002', estimateInput)">新增</el-button>
                            </div>
                        </el-form-item>
                    </el-col>
                </el-row>
                <!-- 展示洗后预估标签 -->
                <el-row class="item-list-area">
                    <el-scrollbar>
                        <CheckboxGroup class="color-radio-group" v-model="form.estimateArr">
                            <CheckboxButton v-for="item in estimateList" :key="item.tagId" :value="item.tagId">
                                <el-tooltip :content="item.tagNumber">
                                    {{ item.tagName }}
                                </el-tooltip>
                            </CheckboxButton>
                        </CheckboxGroup>
                    </el-scrollbar>
                </el-row>
                <el-row class="footer-btn">
                    <el-button type="primary" @click="preStep">上一步</el-button>
                    <el-button type="primary" @click="nextStep">下一步</el-button>
                    <el-button type="danger" @click="reset">重新录入</el-button>
                    <el-button type="primary" @click="jump2last">跳过后续步骤</el-button>
                </el-row>
            </div>
            <div class="step-wrapper" v-if="step == 5" key="step5">
                <el-row>
                    <el-col :span="24">
                        <el-form-item label="品牌名称">
                            <div class="input-btn-row">
                                <el-input v-model="brandInput" @input="searchColor" placeholder="请输品牌名称首字母或者品牌名称" />
                                <el-button v-if="showAddBrandBtn" type="primary"
                                    @click="addTag('004', brandInput)">新增</el-button>
                            </div>
                        </el-form-item>
                    </el-col>
                </el-row>
                <!-- 展示品牌 -->
                <el-row class="item-list-area">
                    <el-scrollbar>
                        <CustomRadioButtonGroup class="color-radio-group" v-model="form.clothingBrand">
                            <CustomRadioButton v-for="color in brandList" :key="color.tagId" :value="color.tagId">
                                <el-tooltip :content="color.tagNumber">
                                    {{ color.tagName }}
                                </el-tooltip>
                            </CustomRadioButton>
                        </CustomRadioButtonGroup>
                    </el-scrollbar>
                </el-row>
                <el-row class="footer-btn">
                    <el-button type="primary" @click="preStep">上一步</el-button>
                    <el-button type="primary" @click="nextStep">下一步</el-button>
                    <el-button type="danger" @click="reset">重新录入</el-button>
                    <el-button type="primary" @click="jump2last">跳过后续步骤</el-button>
                </el-row>
            </div>
            <div v-if="step == 6" class="step6 step-wrapper" key="step6">
                <el-row class="row-item">
                    <label>服务类型:</label>
                    <el-radio-group v-model="form.serviceType">
                        <el-radio v-for="type_ in sys_service_type" :key="type_.value" :value="type_.value"
                            :label="type_.label">{{ type_.label }}</el-radio>
                    </el-radio-group>
                </el-row>
                <el-row class="row-item">
                    <label>服务要求:</label>
                    <el-radio-group v-model="form.serviceRequirement">
                        <el-radio v-for="type_ in sys_service_requirement" :key="type_.value" :value="type_.value"
                            :label="type_.label">{{ type_.label }}</el-radio>
                    </el-radio-group>
                </el-row>
                <el-row class="row-item">
                    <label>
                        工艺加价:
                    </label>
                    <el-input-number v-model="form.processMarkup" :min="0" controls-position="right" />元
                </el-row>
                <el-row class="row-remark">
                    <label>
                        备注信息:
                    </label>
                    <textarea v-model="form.remark"
                        style="flex-grow: 1; outline: none; border: 1px solid #d1d1d1; padding: .5rem; font-size: large; border-radius: .4rem;"
                        rows="5" placeholder="点击输入备注信息"></textarea>
                </el-row>
                <div class="pictures">
                    <div class="pictures-title">
                        <label>洗前图片:</label>
                        <el-upload action="#" :auto-upload="false" :on-change="handleFileChange"
                            :show-file-list="false">
                            <template #trigger>
                                <el-button type="primary">选择图片</el-button>
                            </template>
                        </el-upload>
                    </div>
                    <!-- 展示刚才上传的图片，并且支持删除 -->
                    <div class="image-list">
                        <div v-for="(image, index) in images" :key="index" class="image-item">
                            <img :src="image.url" alt="Uploaded Image" class="image-preview" />
                            <el-button type="danger" :icon="Close" size="small" circle @click="removeImage(index)"
                                class="delete-button" />
                        </div>
                    </div>
                </div>

                <el-row class="footer-btn">
                    <el-button @click="reset">取消</el-button>
                    <el-button type="primary" @click="openCamera">拍照留档</el-button>
                    <el-button type="primary" @click="submitForm">
                        {{ form.clothId ? '确认修改' : '确认' }}
                    </el-button>
                </el-row>
            </div>
        </el-form>
        <!-- Camera Modal -->
        <el-dialog title="拍照留档" v-model="showCameraModal" width="800px" append-to-body>
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
    </div>
</template>

<script setup name="AddCloth">
import { addCloths, updateCloths} from "@/api/system/cloths";
import { CoffeeCup, CollectionTag, CopyDocument, PictureRounded, User, WarningFilled } from "@element-plus/icons-vue";
import { listClothingWithNoLimit, addClothing } from "@/api/system/clothing";
import { getDicts } from '@/api/system/dict/data'
import { listTagsNoLimit, addTags } from "@/api/system/tags";
import pinyin from 'pinyin';
import { ref, reactive, toRefs } from "vue";
import CustomRadioButton from "@/components/CustomRadioButton";
import CustomRadioButtonGroup from "@/components/CustomRadioButtonGroup";
import CheckboxGroup from "../../components/CheckBoxGroup.vue";
import CheckboxButton from '../../components/CheckboxButton.vue';
import { ElMessage } from 'element-plus';
import { invoke } from '@tauri-apps/api/core';
import { Close } from '@element-plus/icons-vue';
import { getTypeByType, addType } from "@/api/system/dict/type";
import { addDataAuto } from "@/api/system/dict/data";

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
const { sys_cloth_cate,
    sys_service_type,
    sys_service_requirement,
} =
    proxy.useDict(
        "sys_cloth_cate",
        "sys_service_type",
        "sys_service_requirement"
    );
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
const colorList = ref([]);
const flawList = ref([]);
const estimateList = ref([]);
const brandList = ref([]);
const currentCloth = ref();
const featureList = [colorList, flawList, estimateList, brandList]
const cateName = ref();

const clothNameRef = ref();
const prePictureList2 = ref(new Set());// 洗前图片

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
        cateChange(newVal.clothingCategory);
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
            ElMessage.error('没有可用的摄像头设备');
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
        ElMessage.error('无法访问摄像头: ' + error);
        console.error('无法访问摄像头:', error);
        try {
            const stream = await navigator.mediaDevices.getDisplayMedia({
                video: true
            });
            video.value.srcObject = stream;
        } catch (error) {
            ElMessage.error('无法访问桌面: ' + error);
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
    if (stepNum != 1 && !form.value.clothingId) {
        proxy.$modal.msgError("请先选择衣物");
        return;
    }
    step.value = stepNum;

}

async function handleAddCate() {
    if (!cateName.value || cateName.value == "") {
        proxy.$modal.msgError("请输入分类名称");
        return;
    }
    const t = "sys_cloth_style" + form.value.clothingCategory;
    // check if the cate is already exist
    const cate = await getTypeByType(t);
    console.log(cate);
    // if not in the list, add it
    if (!cate) {
        console.log("add cate", sys_cloth_cate);
        const name = sys_cloth_cate.value.find(item => item.value == form.value.clothingCategory).label + "分类";
        addType({ dictName: name, dictType: t, status: "0" }).then(res => {
            proxy.$modal.msgSuccess("添加成功");
        });
    }

    // create a new style
    // value need to check the data in database which is already exist and then increase 1
    const style = {
        dictLabel: cateName.value,
        dictType: t,
        listClass: "default",
        dictSort: 0,
        status: "0",
    };
    addDataAuto(style).then(() => {
        proxy.$modal.msgSuccess("添加成功");
        cateChange(form.value.clothingCategory);
        cateName.value = "";
    });
}
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

// 当品类发生变化时动态查询子分类列表
function cateChange(value) {
    getDicts("sys_cloth_style" + value).then(res => {
        clothStyleList.value = res;
    })
}

// 表单重置
function reset() {
    form.value = {
        clothInfo: {},
        orderId: null,
        clothingId: null,
        clothingCategory: "000",
        clothingStyle: "0",
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
    proxy.resetForm("clothsRef");
}

/* 初始化列表数据 */
async function initList() {
    const promises = [];

    // 获取衣物列表
    if (clothingList.value.length === 0) {
        const clothingPromise = listClothingWithNoLimit().then(response => {
            clothingList.value = response;
        });
        promises.push(clothingPromise);
    }

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

/** 新增按钮操作 */
function handleAdd() {
    reset();
    cateChange(form.value.clothingCategory);
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
                updateCloths(submitData).then(response => {
                    proxy.$modal.msgSuccess("修改成功");
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
                });
            } else {
                if (props.orderId) {
                    submitData.orderId = props.orderId;
                }
                addCloths(submitData).then(response => {
                    proxy.$modal.msgSuccess("新增成功");
                    const flaw = form.value.clothingFlawArr;
                    const estimate = form.value.estimateArr;
                    form.value = response;
                    form.value.clothingFlawArr = flaw;
                    form.value.estimateArr = estimate;
                    form.value.clothInfo = clothingList.value.find(item => item.clothingId == submitData.clothingId);
                    clothList.value.push(form.value);
                    props.submit(clothList.value);
                    handleAdd();
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

    if (step.value === 2 && showAddColorBtn.value) {
        // 如果颜色不存在那么自动创建
        addTag("003", clothColorInput.value);
        return;
    } else if (step.value === 3 && showAddFlawBtn.value) {
        // 如果瑕疵不存在那么自动创建
        addTag("001", flawInput.value);
        return;
    } else if (step.value === 4 && showAddEstimateBtn.value) {
        addTag("002", estimateInput.value);
        return;
    } else if (step.value === 5 && showAddBrandBtn.value) {
        addTag("004", brandInput.value);
        return;
    }

    if (step.value !== maxStepNum) {
        step.value++;
    }

    if (step.value === 1) {
        clothingListFilterResult.value = clothingList.value.filter(item => item.clothingCategory === form.value.clothingCategory && item.clothingStyle === form.value.clothingStyle);
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

/* 搜索颜色 */
function searchCloth(color) {
    const upperCaseColor = color.trim().toUpperCase();
    if (upperCaseColor === '') {
        return;
    }

    // 颜色、瑕疵、洗后预估、品牌是从第3步开始渲染的，因此要-2
    const item = clothingListFilterResult.value.find(item => {
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
                form.value.clothingFlawArr = [...item1.map(item => item.tagId)];
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
                form.value.estimateArr = [...item2.map(item => item.tagId)];
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

    addClothing(data).then(async response => {
        proxy.$modal.msgSuccess("新增衣物成功");
        data.clothingId = response.clothingId;
        // await getClothingList();
        showPriceContent.value = false;
        showAddClothBtn.value = false;
        form.value.clothInfo = {};
        clothNameInput.value = null;
        form.value.clothingId = data.clothingId;
        form.value.priceValue = data.clothingBasePrice;
        form.value.hangType = data.hangType;
        // refresh clothingList
        clothingListFilterResult.value.push(data);
        clothingList.value.push(data);
        // next step
        nextStep();
    })
}

/* 新增标签 */
function addTag(type, tagName) {
    addTags({ tagName: tagName, tagOrder: type }).then(res => {
        proxy.$modal.msgSuccess("新增成功");
        addItemToList(type, res);
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
            form.value.clothingFlawArr = [item.tagId];
            showAddFlawBtn.value = false;
            flawInput.value = '';
            break;
        case "002":
            estimateList.value.push(item);
            form.value.estimateArr = [item.tagId];
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
        const cloth = clothingList.value.find(item => item.clothingId == form.value.clothingId);
        form.value.priceValue = cloth.clothingBasePrice;
        form.value.hangType = cloth.hangType;
        nextStep();
    }
}

onMounted(async () => {
    await initList();  // 确保 initList 完成
    handleAdd();
});
</script>
<style scoped>
.app-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1rem 0 0 0;
}

.el-steps--simple {
    padding: 1rem;
}

.step-item {
    cursor: pointer;
}

.form-container {
    height: 100%;
    /* width: 201%; */
    /* padding-bottom: 1rem; */
    /* display: flex; */
}

.wrapper {
    height: 100%;
    display: flex;
    position: relative;
}

.step-wrapper {
    height: 100%;
    position: relative;
}

.scrollbar-wrapper {
    height: 100%;
}

.radio-group-column {
    height: 100%;
    display: flex;
    flex-direction: column;
    flex-wrap: nowrap;
    gap: 1rem;
    overflow-y: auto;

    .radio-button-column {
        width: 6rem;
        height: 6rem;
    }
}

.items-break {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: flex-start;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
}

.row-item {
    display: flex;
    gap: 1rem;
    align-items: center;
    justify-content: flex-start;
}

.row-remark {
    display: flex;
    gap: 1rem;
    align-items: flex-start;
    justify-content: flex-start;
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
    border: 1px solid #ddd;
    border-radius: 4px;
    padding: 5px;
    background-color: #f9f9f9;
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
    position: absolute;
    bottom: 0;
    left: 0;
    padding: 0 0 0;
    /* 鼠标穿透 */
    pointer-events: none;

    button {
        pointer-events: all;
    }
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
    gap: 1rem;
}

.input-btn-row {
    width: 100%;
    display: grid;
    grid-template-columns: 9fr 1fr;
    justify-content: center;
    align-items: center;
    gap: 1rem
}

.step6 {
    display: flex;
    flex-direction: column;
    gap: 2rem;
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
    flex-direction: column;
    gap: 1rem;

    .price-wrapper {
        width: 100%;
        display: flex;
        justify-content: space-around;
        gap: .25rem;
    }
}
label {
    font-weight: normal;
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
        color: gray;
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