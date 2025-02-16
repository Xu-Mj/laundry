<template>
    <el-dialog :show-close="false" v-model="open" width="500px" append-to-body @closed="props.taggle()">
        <el-form ref="expenditureRef" :model="form" :rules="rules" label-width="80px">
            <el-form-item label="支出类型" prop="expType">
                <el-select v-model="form.expType" placeholder="请选择支出类型" clearable>
                    <el-option v-for="dict in sys_exp_type" :key="dict.value" :label="dict.label"
                        :value="dict.value"></el-option>
                </el-select>
            </el-form-item>
            <el-form-item label="支出账目" prop="expTitle">
                <el-input v-model="form.expTitle" placeholder="请输入支出账目" />
            </el-form-item>
            <el-form-item label="对方账户" prop="recvAccountTitle">
                <el-select v-model="form.recvAccount" filterable :clearable="true" remote reserve-keyword
                    placeholder="请选择对方账户" allow-create @blur="handleBlur" remote-show-suffix
                    :remote-method="searchUserByTel" value-key="recvAccount" style="width: 240px">
                    <el-option v-for="item in userListRes" :key="item.userId"
                        :label="item.nickName + '\t' + item.phonenumber" :value="item.userId" />
                </el-select>
            </el-form-item>
            <el-form-item label="支出金额" prop="expAmount">
                <el-input-number :min="0" v-model="form.expAmount" controls-position="right" placeholder="请输入支出金额" />
            </el-form-item>
            <el-form-item label="备注信息" prop="remark">
                <el-input type="textarea" v-model="form.remark" placeholder="请输入备注信息" />
            </el-form-item>
        </el-form>
        <template #footer>
            <div class="dialog-footer">
                <el-button type="primary" @click="submitForm">确 定</el-button>
                <el-button @click="cancel">取 消</el-button>
            </div>
        </template>
    </el-dialog>
</template>


<script setup>
import { addExpenditure, updateExpenditure } from "@/api/system/expenditure";
import { getUser, listUserWithNoLimit } from "@/api/system/user";

const { proxy } = getCurrentInstance();
const { sys_exp_type } = proxy.useDict("sys_exp_type");
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

const userList = ref([]);
const userListRes = ref([]);
const notACount = ref(false);
const open = ref(false);
const data = reactive({
    form: {},
    queryParams: {
        pageNum: 1,
        pageSize: 10,
        orderId: null,
        clothIds: null,
        expTitle: null,
        recvAccount: null,
        recvAccountTitle: null,
        expType: null,
        expAmount: null,
    },
    rules: {
        expTitle: [
            { required: true, message: "支出账目不能为空", trigger: "blur" }
        ],
        expType: [
            { required: true, message: "支出类型不能为空", trigger: "change" }
        ],
        expAmount: [
            { required: true, message: "支出金额不能为空", trigger: "blur" }
        ],
    }
});

const { queryParams, form, rules } = toRefs(data);
// 处理失去焦点的情况，保留用户输入
const handleBlur = (event) => {
    const inputValue = event.target.value;
    // 如果用户没有输入的话，不进行搜索
    if (!inputValue) return;
    if (!userListRes.value.some(item => item.userId === form.value.recvAccount)) {
        // 没有搜索结果且没有选择项时，保留输入
        form.value.recvAccount = inputValue;
        notACount.value = true;
    } else {
        notACount.value = false;
    }
};

/* 根据手机号搜索用户列表 */
function searchUserByTel(tel) {
    userListRes.value = userList.value.filter(item => item.phonenumber.includes(tel));
    if (userListRes.value.length == 0) {
        // 没找到，需要创建用户
        notACount.value = true;
    } else {
        notACount.value = false;
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
        expId: null,
        orderId: null,
        clothIds: null,
        expTitle: null,
        recvAccount: null,
        recvAccountTitle: null,
        expType: null,
        expAmount: null,
        createTime: null,
        remark: null
    };
    proxy.resetForm("expenditureRef");
}

/** 提交按钮 */
function submitForm() {
    proxy.$refs["expenditureRef"].validate(valid => {
        if (valid) {
            if (notACount.value) {
                form.value.recvAccountTitle = form.value.recvAccount;
                form.value.recvAccount = null;
            } else if (form.value.recvAccount) {
                form.value.recvAccountTitle = userList.value.find(item => item.userId === form.value.recvAccount).nickName;
            }
            if (form.value.expId != null) {
                updateExpenditure(form.value).then(response => {
                    proxy.$modal.msgSuccess("修改成功");
                    open.value = false;
                    props.taggle();
                });
            } else {
                addExpenditure(form.value).then(response => {
                    proxy.$modal.msgSuccess("新增成功");
                    open.value = false;
                    props.taggle();
                });
            }
        }
    });
}

onMounted(() => {
    if (props.visible) {
        reset();
        listUserWithNoLimit().then(res => {
            userList.value = res;
            open.value = true;
        })
    }
});
</script>