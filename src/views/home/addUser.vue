<template>
    <el-dialog v-model="props.visible" width="650px" append-to-body :show-close="false" :close-on-click-modal="false">
        <template #header>
            <div class="dialog-header">
                <h2 class="dialog-title">新增会员</h2>
                <p class="dialog-subtitle">填写会员基本信息</p>
            </div>
        </template>
        <el-form :model="form" :rules="rules" ref="userRef" label-width="90px" class="modern-form">
            <!-- 基本信息卡片 -->
            <div class="form-card">
                <div class="card-header">
                    <i class="el-icon-user"></i>
                    <span>基本信息</span>
                </div>
                <div class="card-content">
                    <el-row :gutter="20">
                        <el-col :span="12">
                            <el-form-item label="会员姓名" prop="nickName">
                                <el-input v-model="form.nickName" placeholder="请输入会员姓名" maxlength="30" />
                            </el-form-item>
                        </el-col>
                        <el-col :span="12">
                            <el-form-item label="会员类型" class="highlight-item">
                                <el-select v-model="form.userType" placeholder="请选择" class="full-width">
                                    <el-option v-for="item in sys_user_type" :key="item.value" :label="item.label"
                                        :value="item.value"></el-option>
                                </el-select>
                            </el-form-item>
                        </el-col>
                    </el-row>
                    <el-row :gutter="20">
                        <el-col :span="12">
                            <el-form-item label="手机号码" prop="phonenumber" class="highlight-item">
                                <el-input v-model="form.phonenumber" placeholder="请输入手机号码" maxlength="11">
                                    <template #prefix>
                                        <i class="el-icon-mobile"></i>
                                    </template>
                                </el-input>
                            </el-form-item>
                        </el-col>
                        <el-col :span="12">
                            <el-form-item v-if="form.userId == undefined" label="会员账号" prop="userName">
                                <el-input v-model="form.userName" placeholder="请输入会员账号" maxlength="30" />
                            </el-form-item>
                        </el-col>
                    </el-row>
                    <el-row :gutter="20">
                        <el-col :span="12">
                            <el-form-item v-if="form.userId == undefined && form.userType == '00'" label="会员密码"
                                prop="password">
                                <el-input v-model="form.password" placeholder="请输入会员密码" type="password" maxlength="20"
                                    show-password />
                            </el-form-item>
                        </el-col>
                    </el-row>
                </div>
            </div>

            <!-- 个人特征卡片 -->
            <div class="form-card">
                <div class="card-header">
                    <i class="el-icon-info"></i>
                    <span>个人特征</span>
                </div>
                <div class="card-content">
                    <el-row :gutter="20">
                        <el-col :span="12">
                            <el-form-item label="会员性别">
                                <el-select v-model="form.sex" placeholder="请选择" class="full-width">
                                    <el-option v-for="dict in sys_user_sex" :key="dict.value" :label="dict.label"
                                        :value="dict.value"></el-option>
                                </el-select>
                            </el-form-item>
                        </el-col>
                        <el-col :span="12">
                            <el-form-item label="状态">
                                <el-radio-group v-model="form.status" class="modern-radio-group">
                                    <el-radio v-for="dict in sys_normal_disable" :key="dict.value"
                                        :value="dict.value">{{
                                            dict.label }}</el-radio>
                                </el-radio-group>
                            </el-form-item>
                        </el-col>
                    </el-row>
                    <el-row :gutter="20">
                        <el-col :span="12">
                            <el-form-item label="会员画像">
                                <el-select v-model="form.userTagsArr" multiple placeholder="请选择" class="full-width">
                                    <el-option v-for="dict in sys_user_tags" :key="dict.value" :label="dict.label"
                                        :value="dict.value"></el-option>
                                </el-select>
                            </el-form-item>
                        </el-col>
                        <el-col :span="12">
                            <el-form-item label="画像备注">
                                <el-input v-model="form.tagsRemark" placeholder="补充画像信息"></el-input>
                            </el-form-item>
                        </el-col>
                    </el-row>
                </div>
            </div>

            <!-- 附加信息卡片 -->
            <div class="form-card">
                <div class="card-header">
                    <i class="el-icon-location"></i>
                    <span>附加信息</span>
                </div>
                <div class="card-content">
                    <el-row :gutter="20">
                        <el-col :span="24">
                            <el-form-item label="会员住址">
                                <el-input v-model="form.address" type="textarea" placeholder="请输入会员住址"
                                    rows="2"></el-input>
                            </el-form-item>
                        </el-col>
                    </el-row>
                    <el-row :gutter="20">
                        <el-col :span="24">
                            <el-form-item label="备注">
                                <el-input v-model="form.remark" type="textarea" placeholder="请输入备注信息"
                                    rows="2"></el-input>
                            </el-form-item>
                        </el-col>
                    </el-row>
                </div>
            </div>
        </el-form>
        <template #footer>
            <div class="dialog-footer">
                <el-button @click="cancel">取 消</el-button>
                <el-button type="primary" @click="submitForm">确 定</el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup name="AddUser">
import { addUser,  getUser, updateUser, } from "@/api/system/user";

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
    },
    userId: {
        type: Number,
        required: false,
    }
});

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
    reset();
    props.taggle();
};

/** 修改按钮操作 */
function handleUpdate() {
    getUser(props.userId).then(response => {
        form.value = response;
        form.value.postIds = response.postIds;
        form.value.roleIds = response.roleIds;
        if (form.value.userTags && form.value.userTags.length > 0) {
            form.value.userTagsArr = form.value.userTags.split(",");
        }
        form.password = "";
    });
};

/** 提交按钮 */
function submitForm() {
    proxy.$refs["userRef"].validate(valid => {
        if (valid) {
            if (form.value.userId != undefined) {
                if (form.value.userTagsArr && form.value.userTagsArr.length > 0) {
                    form.value.userTags = form.value.userTagsArr.join(",");
                    delete form.value.userTagsArr;
                } else {
                    form.value.userTags = "";
                }

                updateUser(form.value).then(response => {
                    proxy.notify.success("修改成功")
                    open.value = false;
                    getList();
                });
            } else {
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
                    props.taggle();
                });
            }
        }
    });
};

onMounted(async () => {
    if (props.visible) {
        reset();
        if(props.userId&&props.userId!=0){
            handleUpdate();
        }
    }
});
</script>


<style scoped>
.dialog-header {
    text-align: left;
    padding: 0 0 16px 0;
    border-bottom: 1px solid #f0f0f0;
    margin-bottom: 16px;
}

.dialog-title {
    margin: 0;
    font-size: 18px;
    /* color: #303133; */
    font-weight: 600;
}

.dialog-subtitle {
    margin: 8px 0 0 0;
    font-size: 14px;
    color: var(--el-text-color-regular);
}

.modern-form {
    max-height: 65vh;
    overflow-y: auto;
    padding-right: 10px;
}

.form-card {
    /* background-color: #ffffff; */
    border-radius: 8px;
    /* box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.05); */
    box-shadow: var(--el-box-shadow-light);
    margin-bottom: 20px;
    overflow: hidden;
    transition: all 0.3s ease;
}

.form-card:hover {
    box-shadow: 0 4px 16px 0 rgba(0, 0, 0, 0.6);
}

.form-card:last-child {
    margin-bottom: 10px;
}

.card-header {
    padding: 12px 16px;
    background-color: var(--el-fill-color-light);
    border-bottom: 1px solid #ebeef5;
    display: flex;
    align-items: center;
}

.card-header i {
    margin-right: 8px;
    color: #409EFF;
}

.card-header span {
    font-size: 15px;
    font-weight: 500;
    color: var(--el-text-color-primary);
}

.card-content {
    padding: 16px;
}

.highlight-item :deep(.el-input__wrapper),
.highlight-item :deep(.el-select .el-input__wrapper) {
    box-shadow: 0 0 0 1px #409EFF inset;
    transition: all 0.3s ease;
}

.full-width {
    width: 100%;
}

.modern-radio-group {
    display: flex;
    gap: 16px;
}

.dialog-footer {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
}

/* 美化滚动条 */
.modern-form::-webkit-scrollbar {
    width: 6px;
}

.modern-form::-webkit-scrollbar-thumb {
    background-color: #dcdfe6;
    border-radius: 3px;
}

.modern-form::-webkit-scrollbar-track {
    background-color: #f5f7fa;
}
</style>
