<template>
    <el-dialog :align-center="true" v-model="props.visible" width="650px" append-to-body :show-close="false"
        :close-on-click-modal="false" :close-on-press-escape="false">
        <template #header>
            <div class="dialog-header  hover-flow">
                <div>
                    <h2 class="dialog-title">新增会员</h2>
                    <p class="dialog-subtitle">填写会员基本信息</p>
                </div>
                <el-button circle @click="cancel">
                    <el-icon>
                        <Close />
                    </el-icon>
                </el-button>
            </div>
        </template>
        <el-form :model="form" :rules="rules" ref="userRef" label-width="90px" class="modern-form">
            <!-- 基本信息卡片 -->
            <div class="form-card hover-flow">
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
                            <el-form-item label="手机号码" prop="phonenumber" class="highlight-item">
                                <el-input v-model="form.phonenumber" placeholder="请输入手机号码" maxlength="11">
                                    <template #prefix>
                                        <i class="el-icon-mobile"></i>
                                    </template>
                                </el-input>
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
            <div class="form-card hover-flow">
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
                                    <el-radio v-for="dict in sys_normal_disable" :key="dict.value" :value="dict.value">
                                        {{ dict.label }}
                                    </el-radio>
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
            <div class="form-card hover-flow">
                <div class="card-header">
                    <i class="el-icon-location"></i>
                    <span>附加信息</span>
                </div>
                <div class="card-content">
                    <el-row :gutter="20">
                        <el-col :span="24">
                            <el-form-item label="会员住址">
                                <el-input v-model="form.address" type="textarea" placeholder="请输入会员住址"
                                    :rows="2"></el-input>
                            </el-form-item>
                        </el-col>
                    </el-row>
                    <el-row :gutter="20">
                        <el-col :span="24">
                            <el-form-item label="备注">
                                <el-input v-model="form.remark" type="textarea" placeholder="请输入备注信息"
                                    :rows="2"></el-input>
                            </el-form-item>
                        </el-col>
                    </el-row>
                </div>
            </div>
        </el-form>
        <template #footer>
            <div class="dialog-footer">
                <el-button type="primary" @click="submitForm" icon="Check">确 定</el-button>
                <el-button type="danger" @click="cancel" icon="Close">取 消</el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup name="AddUser">
import { addUser, getUser, updateUser, } from "@/api/system/user";
import eventBus from "@/utils/eventBus";

const { proxy } = getCurrentInstance();
const emit = defineEmits(['refresh']);

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
        userType: "01", // 默认设置为'01'会员类型
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
                    // 触发userUpdated事件
                    eventBus.emit('userUpdated');
                    emit('refresh');
                    props.taggle();
                });
            } else {
                if (form.value.userTagsArr && form.value.userTagsArr.length > 0) {
                    form.value.userTags = form.value.userTagsArr.join(",");
                    delete form.value.userTagsArr;
                }

                // 默认使用手机号作为会员账号
                form.value.userName = form.value.phonenumber;

                addUser(form.value).then(response => {
                    proxy.notify.success("新增成功");
                    // 触发userAdded事件
                    eventBus.emit('userAdded');
                    emit('refresh');
                    props.taggle();
                });
            }
        }
    });
};

onMounted(async () => {
    if (props.visible) {
        reset();
        if (props.userId && props.userId != 0) {
            handleUpdate();
        }
    }
});
</script>


<style scoped>
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

.card-content {
    padding: 1rem;
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
    gap: 1rem;
}

.dialog-footer {
    display: flex;
    justify-content: flex-end;
    gap: 12px;

    button {
        transition: all 0.3s;
    }

    button:hover {
        transform: translateY(-2px);
    }
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
