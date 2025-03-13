<template>
    <el-dialog v-model="open" width="500px" :show-close="false" append-to-body @closed="closeHangUpDialog"
        @opened="refGetFocus" class="modern-dialog">
        <template #header>
            <div class="dialog-header">
                <h3 class="dialog-title">衣物上挂</h3>
                <div class="member-info" v-if="currentUser">
                    <el-avatar v-if="currentUser.avatar" :size="32" :src="currentUser.avatar" />
                    <el-avatar :size="32" icon="UserFilled" />
                    <span>{{ currentUser.nickName }} <small>{{ currentUser.phonenumber }}</small></span>
                </div>
            </div>
        </template>

        <div class="form-container">
            <el-form ref="hangUpRef" class="modern-form" :model="hangForm" :rules="hangRules" label-width="90px">
                <!-- 扫码区域 -->
                <div class="scan-section">
                    <div class="section-divider">
                        <span>衣物编码</span>
                    </div>
                    <div class="info-card cloth-number-container">
                        <el-form-item label="衣物编码" prop="clothingNumber">
                            <el-input ref="clothingNumberRef" v-model="hangForm.clothingNumber" @change="getClothInfo"
                                @keydown.enter="getClothInfoByEnter" placeholder="请输入或扫描衣物编码">
                                <template #prefix>
                                    <el-icon>
                                        <Search />
                                    </el-icon>
                                </template>
                            </el-input>
                        </el-form-item>
                    </div>
                </div>

                <!-- 衣物信息卡片 -->
                <div class="info-section">
                    <div class="section-divider">
                        <span>衣物信息</span>
                    </div>
                    <!-- <transition name="fade"> -->
                    <div class="info-card" v-if="currentCloth">
                        <!-- 衣物基本信息 -->
                        <div class="info-header">
                            <div class="cloth-name">
                                <el-icon>
                                    <Goods />
                                </el-icon>
                                {{ currentCloth.clothInfo.clothingName }}
                                <span v-if="currentCloth.clothingColor" class="cloth-detail">
                                    <el-tag size="small" effect="plain" type="info">
                                        {{colorList.find(item => item.tagId == currentCloth.clothingColor).tagName}}
                                    </el-tag>
                                </span>
                                <span v-if="currentCloth.clothingBrand" class="cloth-detail">
                                    <el-tag size="small" effect="plain" type="success">
                                        {{brandList.find(item => item.tagId == currentCloth.clothingBrand).tagName}}
                                    </el-tag>
                                </span>
                            </div>
                        </div>

                        <el-divider class="info-divider" />

                        <!-- 衣物详细信息 -->
                        <div class="info-content">
                            <!-- 瑕疵信息 -->
                            <div class="info-item"
                                v-if="currentCloth.flawListArr && currentCloth.flawListArr.length > 0">
                                <div class="item-header">
                                    <el-icon>
                                        <Warning />
                                    </el-icon>
                                    <span class="item-title">瑕疵信息</span>
                                </div>
                                <div class="tag-container">
                                    <el-tag v-for="tagId in currentCloth.flawListArr" :key="tagId" type="danger"
                                        effect="light" size="small" class="info-tag">
                                        {{flawList.find(item => item.tagId == tagId).tagName}}
                                    </el-tag>
                                </div>
                            </div>

                            <!-- 洗后预估信息 -->
                            <div class="info-item"
                                v-if="currentCloth.estimateArr && currentCloth.estimateArr.length > 0">
                                <div class="item-header">
                                    <el-icon>
                                        <Timer />
                                    </el-icon>
                                    <span class="item-title">洗后预估</span>
                                </div>
                                <div class="tag-container">
                                    <el-tag v-for="tagId in currentCloth.estimateArr" :key="tagId" type="warning"
                                        effect="light" size="small" class="info-tag">
                                        {{estimateList.find(item => item.tagId == tagId).tagName}}
                                    </el-tag>
                                </div>
                            </div>

                            <!-- 备注信息 -->
                            <div class="info-item" v-if="currentCloth.clothInfo.remark">
                                <div class="item-header">
                                    <el-icon>
                                        <Memo />
                                    </el-icon>
                                    <span class="item-title">备注信息</span>
                                </div>
                                <div class="remark-content">{{ currentCloth.clothInfo.remark }}</div>
                            </div>
                        </div>
                    </div>
                    <div class="info-card empty-info-card" v-else>
                        <div class="empty-state">
                            <el-empty description="请先扫描衣物编码" :image-size="80"></el-empty>
                        </div>
                    </div>
                    <!-- </transition> -->
                </div>

                <!-- 挂衣信息 -->
                <div class="hang-section">
                    <div class="section-divider">
                        <span>衣挂信息</span>
                    </div>
                    <div class="info-card">
                        <el-form-item label="衣挂位置" prop="hangLocationId">
                            <el-select v-model="hangForm.hangLocationId" placeholder="请选择上挂位置编码" class="modern-select">
                                <el-option v-for="item in hangLocationList" :key="item.id" :label="item.name"
                                    :value="item.id">
                                </el-option>
                            </el-select>
                        </el-form-item>

                        <el-form-item label="衣挂编号" prop="hangerNumber">
                            <el-input v-model="hangForm.hangerNumber" placeholder="请输入上挂衣物编码">
                                <template #prefix>
                                    <el-icon>
                                        <Location />
                                    </el-icon>
                                </template>
                            </el-input>
                        </el-form-item>

                        <el-form-item label="备注信息" prop="hangRemark">
                            <el-input type="textarea" v-model="hangForm.hangRemark" placeholder="请输入上挂描述信息" rows="2" />
                        </el-form-item>
                    </div>
                </div>
            </el-form>
        </div>

        <template #footer>
            <div class="dialog-footer">
                <el-button @click="open = false" plain>
                    取消
                </el-button>
                <el-button type="primary" ref="hangUpBtnRef" :disabled="hangupBtnDisabled" @click="hangUp">
                    确认上挂
                </el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup name="HangUp">
import { getClothByCode, hangup } from "@/api/system/cloths";
import { listTagsNoLimit } from "@/api/system/tags";
import { listRack } from "@/api/system/rack";
import { getUserByClothCode } from "@/api/system/user";
import { Search, Location, Goods, Warning, Timer, Memo } from '@element-plus/icons-vue';

const props = defineProps({
    visible: {
        type: Boolean,
        required: true,
        default: false,
    },
    taggle: {
        type: Function,
        required: true,
    }
});

const { proxy } = getCurrentInstance();

const data = reactive({
    hangForm: {},
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
});

const { hangForm, hangRules } = toRefs(data);

const colorList = ref([]);
const flawList = ref([]);
const estimateList = ref([]);
const brandList = ref([]);

const open = ref(false);
const hangupBtnDisabled = ref(false);

const currentCloth = ref(null);
const currentUser = ref(null);
const clothingNumberRef = ref();
const hangUpBtnRef = ref();
const hangLocationList = ref();

function getClothInfoByEnter(event) {
    event.preventDefault();
    getClothInfo();
}

function getClothInfo() {
    if (hangForm.value.clothingNumber === null || hangForm.value.clothingNumber.trim() === '') {
        currentCloth.value = null;
        currentUser.value = null;
        return;
    }

    const clothingNumber = hangForm.value.clothingNumber.trim();
    getClothByCode(clothingNumber).then(res => {
        currentCloth.value = res;
        if (!currentCloth.value) {
            proxy.notify.error("衣物编码关联的衣物不存在");
            hangForm.value.clothId = null;
            hangForm.value.hangLocationId = null;
            hangForm.value.hangerNumber = null;
            hangForm.value.hangRemark = null;
            hangupBtnDisabled.value = true;
        } else if (currentCloth.value.clothingStatus === '02') {
            proxy.notify.warning("衣物编码关联的衣物已上挂");
            hangupBtnDisabled.value = true;
            hangForm.value = {
                clothingNumber: currentCloth.value.hangClothCode,
                clothId: currentCloth.value.clothId,
                hangLocationId: currentCloth.value.hangLocationCode,
                hangerNumber: currentCloth.value.hangerNumber,
                hangRemark: currentCloth.value.hangRemark,
            };
        } else {
            // 查找最合适的衣挂位置
            hangForm.value = {
                clothingNumber: currentCloth.value.hangClothCode,
                clothId: currentCloth.value.clothId,
                hangLocationId: currentCloth.value.hangLocationCode,
                hangerNumber: currentCloth.value.hangerNumber,
                hangRemark: currentCloth.value.hangRemark,
            };
            hangupBtnDisabled.value = false;
            // 找到了，确认上挂获取焦点
            hangUpBtnRef.value.$el.focus();
            currentCloth.value.flawListArr = currentCloth.value.clothingFlaw?.split(",");
            currentCloth.value.estimateArr = currentCloth.value.estimate?.split(",");
        }
    });
    getUserByClothCode(clothingNumber).then(res => {
        currentUser.value = res;
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

/* 上挂 */
function hangUp() {
    if (currentCloth.value) {
        //校验上挂表单内容
        proxy.$refs["hangUpRef"].validate(valid => {
            if (valid) {
                console.log(currentCloth.value)
                console.log(hangForm.value)
                hangup(hangForm.value).then(res => {
                    proxy.notify.success("上挂成功");
                    open.value = false;
                    props.taggle();
                }).catch(res => {
                    proxy.notify.error(res.msg);
                })
            }
        });
    }
}

/* 关闭上挂弹窗 */
function closeHangUpDialog() {
    hangForm.value = {
        clothingNumber: null,
        hangLocationCode: null,
        hangClothCode: null,
        hangRemark: null
    };
    props.taggle();
}

// 弹窗开启的时候获取焦点
function refGetFocus() {
    // 取得焦点
    clothingNumberRef.value.focus();
}

onMounted(async () => {
    if (props.visible) {
        await initList();
        open.value = true;
        // 获取衣挂列表
        listRack().then(res => {
            hangLocationList.value = res;
        })
    }
});
</script>

<style scoped>
/* 对话框整体样式 */
.modern-dialog {
    transition: all 0.5s;
}

.modern-dialog :deep(.el-dialog__header) {
    padding: 0;
    margin: 0;
}

.modern-dialog :deep(.el-dialog__body) {
    padding: 20px;
}

/* 对话框头部样式 */
.dialog-header {
    background-color: var(--el-fill-color-light);
    padding: 16px 20px;
    /* border-bottom: 1px solid var(--el-border-color-light); */
    border-radius: 8px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    box-shadow: var(--el-box-shadow);
}

.dialog-title {
    margin: 0;
    font-size: 18px;
    /* color: #303133; */
    font-weight: 600;
}

.member-info {
    display: flex;
    align-items: center;
    gap: 10px;
}

.member-info span {
    font-size: 14px;
}

.member-info small {
    color: #909399;
    margin-left: 5px;
}

/* 表单容器样式 */
.form-container {
    padding: 0;
}

.modern-form {
    margin-top: 10px;
}

/* 扫码区域样式 */
.scan-section {
    margin-bottom: 20px;
}

/* 衣物信息卡片样式 */
.info-card {
    background-color: var(--el-fill-color);
    border-radius: 8px;
    padding: 15px;
    box-shadow: var(--el-box-shadow);


}

.cloth-number-container {
    .el-form-item {
        margin-bottom: 0 !important;
    }
}

.info-header {
    margin-bottom: 12px;
    /* border-bottom: 1px solid #ebeef5; */
    padding-bottom: 8px;
}

.info-title {
    font-size: 16px;
    font-weight: 600;
    color: #303133;
}

.info-content {
    color: #606266;
}

.cloth-name {
    font-size: 15px;
    margin-bottom: 0;
    color: var(--el-text-color-primary);
    font-weight: 500;
    display: flex;
    align-items: center;
    gap: 8px;
}

.cloth-name .el-icon {
    color: var(--el-color-primary);
}

.cloth-detail {
    font-weight: normal;
    margin-left: 5px;
}

.info-divider {
    margin: 12px 0;
}

.info-item {
    margin-bottom: 16px;
}

.info-item:last-child {
    margin-bottom: 0;
}

.item-header {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-bottom: 8px;
}

.item-header .el-icon {
    font-size: 16px;
}

.item-title {
    font-size: 14px;
    font-weight: 500;
    color: #606266;
}

.tag-container {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    padding-left: 24px;
}

.info-tag {
    margin-right: 0;
}

.remark-content {
    color: #606266;
    padding-left: 24px;
    line-height: 1.5;
}

/* 空状态样式 */
.empty-state {
    padding: 10px 0;
    display: flex;
    justify-content: center;
}

.empty-info-card {
    min-height: 120px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 20px;
}

/* 挂衣信息区域样式 */
.hang-section {
    margin-top: 10px;
}

.section-divider {
    position: relative;
    text-align: left;
    margin: 15px 0;
    color: var(--el-text-color-primary);
    font-weight: 600;
    font-size: 16px;
}

.section-divider::after {
    content: '';
    position: absolute;
    left: 0;
    bottom: -5px;
    width: 4rem;
    height: 3px;
    background-color: var(--el-color-primary);
    border-radius: 3px;
}

.modern-select {
    width: 100%;
}

/* 底部按钮样式 */
.dialog-footer {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
}
</style>