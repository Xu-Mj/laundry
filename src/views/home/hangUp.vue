<template>
    <el-dialog :title="title" v-model="open" width="400px" :show-close="false" append-to-body
        :before-close="closeHangUpDialog" @opened="refGetFocus">
        <el-form ref="hangUpRef" :model="hangForm" :rules="hangRules" label-width="80px">
            <el-form-item label="衣物编码" prop="clothingNumber">
                <el-input ref="clothingNumberRef" v-model="hangForm.clothingNumber" @change="getClothInfo"
                    @keydown.enter="getClothInfoByEnter" placeholder="请输入衣物编码" />
            </el-form-item>
            <el-form-item label="衣物信息">
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
            <el-form-item label="备注信息" prop="hangRemark">
                <el-input type="textarea" v-model="hangForm.hangRemark" placeholder="请输入上挂描述信息" />
            </el-form-item>
        </el-form>
        <template #footer>
            <div class="hangup-footer">
                <el-button type="primary" ref="hangUpBtnRef" @click="hangUp">确认上挂</el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup name="HangUp">
import { getClothByCode, hangup } from "@/api/system/cloths";
import { listTagsNoLimit } from "@/api/system/tags";
import { listRack } from "@/api/system/rack";

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

const currentCloth = ref(null);
const clothingNumberRef = ref();
const hangUpBtnRef = ref();
const hangLocationList = ref();

function getClothInfoByEnter(event) {
    event.preventDefault();
    getClothInfo();
}

function getClothInfo() {
    getClothByCode(hangForm.value.clothingNumber.trim()).then(res => {
        currentCloth.value = res.data;
        if (!currentCloth.value) {
            proxy.$modal.msgError("衣物编码关联的衣物不存在");
        } else if (currentCloth.value.clothingStatus === '02') {
            proxy.$modal.msgWarning("衣物编码关联的衣物已上挂");
        } else {
            // 查找最合适的衣挂位置
            hangForm.value = {
                clothingNumber: currentCloth.value.hangClothCode,
                clothId: currentCloth.value.clothId,
                hangLocationId: currentCloth.value.hangLocationCode,
                hangerNumber: currentCloth.value.hangerNumber,
                hangRemark: currentCloth.value.hangRemark,
            };
            // 找到了，确认上挂获取焦点
            hangUpBtnRef.value.$el.focus();
        }
    })
}

/* 初始化列表数据 */
async function initList() {
    const promises = [];

    // 获取颜色列表
    if (colorList.value.length === 0) {
        const colorPromise = listTagsNoLimit({ tagOrder: '003' }).then(response => {
            colorList.value = response.rows;
        });
        promises.push(colorPromise);
    }

    // 获取瑕疵列表
    if (flawList.value.length === 0) {
        const flawPromise = listTagsNoLimit({ tagOrder: '001' }).then(response => {
            flawList.value = response.rows;
        });
        promises.push(flawPromise);
    }

    // 获取预估列表
    if (estimateList.value.length === 0) {
        const estimatePromise = listTagsNoLimit({ tagOrder: '002' }).then(response => {
            estimateList.value = response.rows;
        });
        promises.push(estimatePromise);
    }

    // 获取品牌列表
    if (brandList.value.length === 0) {
        const brandPromise = listTagsNoLimit({ tagOrder: '004' }).then(response => {
            brandList.value = response.rows;
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
function closeHangUpDialog(done) {
    hangForm.value = {
        clothingNumber: null,
        hangLocationCode: null,
        hangClothCode: null,
        hangRemark: null
    };
    done();
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
            hangLocationList.value = res.rows;
        })
    }
});
</script>