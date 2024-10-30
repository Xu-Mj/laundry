<template>
    <el-dialog v-model="open" title="新增会员" width="600px" append-to-body :show-close="false"
        close-on-click-modal="false">
        <el-form :model="form" :rules="rules" ref="userRef" label-width="80px">
            <el-row>
                <el-col :span="12">
                    <el-form-item label="会员姓名" prop="nickName">
                        <el-input v-model="form.nickName" placeholder="请输入会员姓名" maxlength="30" />
                    </el-form-item>
                </el-col>
                <el-col :span="12">
                    <el-form-item label="会员类型">
                        <el-select v-model="form.userType" placeholder="请选择">
                            <el-option v-for="item in sys_user_type" :key="item.value" :label="item.label"
                                :value="item.value"></el-option>
                        </el-select>
                    </el-form-item>
                </el-col>
            </el-row>
            <el-row>
                <el-col :span="12">
                    <el-form-item label="手机号码" prop="phonenumber">
                        <el-input v-model="form.phonenumber" placeholder="请输入手机号码" maxlength="11" />
                    </el-form-item>
                </el-col>
            </el-row>
            <el-row>
                <!-- <el-col :span="12">
                  <el-form-item label="邮箱" prop="email">
                     <el-input v-model="form.email" placeholder="请输入邮箱" maxlength="50" />
                  </el-form-item>
               </el-col> -->
                <el-col :span="12">
                    <el-form-item v-if="form.userId == undefined" label="会员账号" prop="userName">
                        <el-input v-model="form.userName" placeholder="请输入会员账号" maxlength="30" />
                    </el-form-item>
                </el-col>
                <el-col :span="12">
                    <el-form-item v-if="form.userId == undefined && form.userType == '00'" label="会员密码" prop="password">
                        <el-input v-model="form.password" placeholder="请输入会员密码" type="password" maxlength="20"
                            show-password />
                    </el-form-item>
                </el-col>
            </el-row>
            <el-row>
                <el-col :span="12">
                    <el-form-item label="会员性别">
                        <el-select v-model="form.sex" placeholder="请选择">
                            <el-option v-for="dict in sys_user_sex" :key="dict.value" :label="dict.label"
                                :value="dict.value"></el-option>
                        </el-select>
                    </el-form-item>
                </el-col>
                <el-col :span="12">
                    <el-form-item label="状态">
                        <el-radio-group v-model="form.status">
                            <el-radio v-for="dict in sys_normal_disable" :key="dict.value" :value="dict.value">{{
                                dict.label
                                }}</el-radio>
                        </el-radio-group>
                    </el-form-item>
                </el-col>
            </el-row>
            <el-row>
                <el-col :span="12">
                    <el-form-item label="会员画像">
                        <el-select v-model="form.userTagsArr" multiple placeholder="请选择">
                            <el-option v-for="dict in sys_user_tags" :key="dict.value" :label="dict.label"
                                :value="dict.value"></el-option>
                        </el-select>
                    </el-form-item>
                </el-col>
                <el-col :span="12">
                    <el-form-item label="画像备注">
                        <el-input v-model="form.tagsRemark" placeholder=""></el-input>
                    </el-form-item>
                </el-col>
            </el-row>
            <el-row>
                <el-col :span="24">
                    <el-form-item label="会员住址">
                        <el-input v-model="form.address" type="textarea" placeholder="请输入内容"></el-input>
                    </el-form-item>
                </el-col>
            </el-row>
            <el-row>
                <el-col :span="24">
                    <el-form-item label="备注">
                        <el-input v-model="form.remark" type="textarea" placeholder="请输入内容"></el-input>
                    </el-form-item>
                </el-col>
            </el-row>
        </el-form>
        <template #footer>
            <div class="dialog-footer">
                <el-button type="primary" @click="submitForm">确 定</el-button>
                <el-button @click="cancel">取 消</el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup name="AddUser">
import { getUser, updateUser, addUser } from "@/api/system/user";

const { proxy } = getCurrentInstance();

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
const open = ref(false);
const postOptions = ref();
const roleOptions = ref();

const data = reactive({
    form: {},
    rules: {
        userName: [{ min: 2, max: 20, message: "会员账号长度必须介于 4 和 30 之间", trigger: "blur" }],
        nickName: [{ required: true, message: "会员姓名不能为空", trigger: "blur" }],
        password: [{ required: true, message: "会员密码不能为空", trigger: "blur" }, { min: 5, max: 20, message: "会员密码长度必须介于 5 和 20 之间", trigger: "blur" }, { pattern: /^[^<>"'|\\]+$/, message: "不能包含非法字符：< > \" ' \\\ |", trigger: "blur" }],
        email: [{ type: "email", message: "请输入正确的邮箱地址", trigger: ["blur", "change"] }],
        phonenumber: [{ required: true, message: "手机号不能为空", trigger: "blur" }, { pattern: /^1[3|4|5|6|7|8|9][0-9]\d{8}$/, message: "请输入正确的手机号码", trigger: "blur" }]
    }
});
const { form, rules } = toRefs(data);
const {
    sys_normal_disable,
    sys_user_tags,
    sys_user_sex,
    sys_user_type,
    sys_user_identify
} = proxy.useDict("sys_normal_disable", "sys_user_tags", "sys_user_sex", "sys_user_type", "sys_user_identify");

/** 重置操作表单 */
function reset() {
    form.value = {
        userId: undefined,
        deptId: undefined,
        userName: undefined,
        nickName: undefined,
        userType: "01",
        password: undefined,
        phonenumber: undefined,
        email: undefined,
        sex: undefined,
        status: "0",
        remark: undefined,
        postIds: [],
        roleIds: []
    };
    proxy.resetForm("userRef");
};
/** 取消按钮 */
function cancel() {
    open.value = false;
    reset();
    props.taggle();
};


/** 提交按钮 */
function submitForm() {
    proxy.$refs["userRef"].validate(valid => {
        if (valid) {
            if (form.value.userTagsArr && form.value.userTagsArr.length > 0) {
                form.value.userTags = form.value.userTagsArr.join(",");
                delete form.value.userTagsArr;
            }

            // 如果没有填写会员账号，那么默认使用手机号
            if (form.value.userName == undefined || form.value.userName.trim().length == 0) {
                form.value.userName = form.value.phonenumber;
            }

            addUser(form.value).then(response => {
                proxy.$modal.msgSuccess("新增成功");
                //    open.value = false;
                reset();
            });
        }
    });
};

onMounted(async () => {
    if (props.visible) {
        reset();
        getUser().then(response => {
            postOptions.value = response.posts;
            roleOptions.value = response.roles;
            open.value = true;
        });
    }
});
</script>