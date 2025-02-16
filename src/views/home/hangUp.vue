<template>
    <el-dialog :title="title" v-model="open" width="450px" :show-close="false" append-to-body
        @closed="closeHangUpDialog" @opened="refGetFocus">
        <template #header>
            <span class="title">
                会员信息：{{ currentUser ? currentUser.nickName + '-' + currentUser.phonenumber : '' }}
            </span>
        </template>

        <el-form ref="hangUpRef" class="custom-form" :model="hangForm" :rules="hangRules" label-width="80px">
            <el-form-item label="衣物编码" prop="clothingNumber">
                <el-input ref="clothingNumberRef" v-model="hangForm.clothingNumber" @change="getClothInfo"
                    @keydown.enter="getClothInfoByEnter" placeholder="请输入衣物编码" />
            </el-form-item>
            <el-form-item label="衣物信息" class="custom-form-item">
                <span v-if="currentCloth">
                    {{ currentCloth.clothInfo.clothingName }}
                    {{ currentCloth.clothingColor ? '-' +
                        colorList.find(item => item.tagId ==
                            currentCloth.clothingColor).tagName : '' }}
                    {{ currentCloth.clothingBrand ? '-' +
                        brandList.find(item => item.tagId ==
                            currentCloth.clothingBrand).tagName : '' }}
                </span>
                <span v-else>-</span>
            </el-form-item>
            <el-form-item label="衣物瑕疵">
                <el-tag v-for="tagId in currentCloth ? currentCloth.flawListArr : []" :key="tagId" type="danger">
                    {{ flawList.find(item => item.tagId == tagId).tagName }}
                </el-tag>
            </el-form-item>
            <el-form-item label="附加信息">
                {{ currentCloth ? currentCloth.clothInfo.remark : '-' }}
            </el-form-item>
            <el-form-item label="衣挂位置" prop="hangLocationId">
                <!-- <el-input v-model="hangForm.hangLocationId" placeholder="请输入上挂位置编码" /> -->
                <el-select v-model="hangForm.hangLocationId" placeholder="请选择上挂位置编码">
                    <el-option v-for="item in hangLocationList" :key="item.id" :label="item.name" :value="item.id">
                    </el-option>
                </el-select>
            </el-form-item>
            <el-form-item label="衣挂编号" prop="hangerNumber">
                <el-input v-model="hangForm.hangerNumber" placeholder="请输入上挂衣物编码" />
            </el-form-item>
            <el-form-item class="custom-form-item" label="备注信息" prop="hangRemark">
                <el-input type="textarea" v-model="hangForm.hangRemark" placeholder="请输入上挂描述信息" />
            </el-form-item>
            <!-- <el-form-item label="所属会员">
                {{ currentUser ? currentUser.nickName + '-' + currentUser.phonenumber : '-' }}
            </el-form-item> -->
        </el-form>
        <template #footer>
            <div class="hangup-footer">
                <el-button type="primary" @click="open = false">
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
// const estimateList = ref([]);
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
    getClothByCode(hangForm.value.clothingNumber.trim()).then(res => {
        currentCloth.value = res;
        if (!currentCloth.value) {
            proxy.$modal.msgError("衣物编码关联的衣物不存在");
            hangForm.value.clothId = null;
            hangForm.value.hangLocationId = null;
            hangForm.value.hangerNumber = null;
            hangForm.value.hangRemark = null;
            hangupBtnDisabled.value = true;
        } else if (currentCloth.value.clothingStatus === '02') {
            proxy.$modal.msgWarning("衣物编码关联的衣物已上挂");
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
            currentCloth.value.flawListArr = currentCloth.value.clothingFlaw.split(",");
        }
    });
    getUserByClothCode(hangForm.value.clothingNumber).then(res => {
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
    // if (estimateList.value.length === 0) {
    //     const estimatePromise = listTagsNoLimit({ tagOrder: '002' }).then(response => {
    //         estimateList.value = response;
    //     });
    //     promises.push(estimatePromise);
    // }

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
                    proxy.$modal.msgSuccess("上挂成功");
                    open.value = false;
                    props.taggle();
                }).catch(res => {
                    proxy.$modal.msgError(res.msg);
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
.title {
    color: blue;
}

.custom-form .el-form-item {
    border: 1px solid #ddd;
    padding: 10px;
    margin-bottom: 0px;
    border-bottom: none;
}

.custom-form .el-form-item:last-child {
    border-bottom: 1px solid #ddd;
}
</style>