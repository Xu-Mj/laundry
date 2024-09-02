<template>
   <div class="app-container">
      <!--会员数据-->
      <el-form :model="queryParams" ref="queryRef" :inline="true" v-show="showSearch" label-width="68px">
         <el-form-item label="会员账号" prop="userName">
            <el-input v-model="queryParams.userName" placeholder="请输入会员账号" clearable style="width: 240px"
               @keyup.enter="handleQuery" />
         </el-form-item>
         <el-form-item label="手机号码" prop="phonenumber">
            <el-input v-model="queryParams.phonenumber" placeholder="请输入手机号码" clearable style="width: 240px"
               @keyup.enter="handleQuery" @input="handleTelQuery" />
         </el-form-item>
         <el-form-item label="会员等级" prop="postName">
            <el-select v-model="queryParams.postName" placeholder="会员等级" clearable style="width: 240px">
               <el-option v-for="item in postOptions" :key="item.postId" :label="item.postName" :value="item.postName"
                  :disabled="item.status == 1"></el-option>
            </el-select>
         </el-form-item>
         <el-form-item>
            <el-button type="primary" icon="Search" @click="handleQuery">搜索</el-button>
            <el-button icon="Refresh" @click="resetQuery">重置</el-button>
         </el-form-item>
      </el-form>

      <el-row :gutter="10" class="mb8">
         <el-col :span="1.5">
            <el-button type="primary" plain icon="Plus" @click="handleAdd"
               v-hasPermi="['system:user:add']">新增</el-button>
         </el-col>
         <el-col :span="1.5">
            <el-button type="success" plain icon="Edit" :disabled="single" @click="handleUpdate"
               v-hasPermi="['system:user:edit']">修改</el-button>
         </el-col>
         <el-col :span="1.5">
            <el-button type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete"
               v-hasPermi="['system:user:remove']">删除</el-button>
         </el-col>
         <right-toolbar v-model:showSearch="showSearch" @queryTable="getList" :columns="columns"></right-toolbar>
      </el-row>

      <el-table v-loading="loading" :data="userList" @selection-change="handleSelectionChange">
         <el-table-column type="selection" width="50" align="center" />
         <el-table-column label="会员编号" align="center" key="userId" prop="userId" v-if="columns[0].visible" />
         <el-table-column label="会员姓名" align="center" v-if="columns[2].visible">
            <template #default="scope">
               <el-tooltip content="查看会员详情" placement="top">
                  <el-button link @click="showUserInfo(scope.row)">{{ scope.row.nickName }}</el-button>
               </el-tooltip>
            </template>
         </el-table-column>
         <el-table-column label="会员账号" align="center" key="userName" prop="userName" v-if="columns[1].visible"
            :show-overflow-tooltip="true" />
         <el-table-column label="手机号码" align="center" key="phonenumber" prop="phonenumber" v-if="columns[4].visible"
            width="120" />
         <el-table-column label="会员类型" align="center" key="userType" prop="userType" v-if="columns[7].visible">
            <template #default="scope">
               <dict-tag :options="sys_user_type" :value="scope.row.userType" />
            </template>
         </el-table-column>
         <!-- <el-table-column label="会员组织" align="center" key="deptName" prop="dept.deptName" v-if="columns[3].visible"
         :show-overflow-tooltip="true" /> -->
         <el-table-column label="微信标识" align="center" prop="openId" :show-overflow-tooltip="true"
            v-if="columns[10].visible" />
         <el-table-column label="性别" align="center" key="sex" v-if="columns[11].visible">
            <template #default="scope">
               <dict-tag :options="sys_user_sex" :value="scope.row.sex" />
            </template>
         </el-table-column>
         <el-table-column label="会员住址" align="center" key="address" prop="address" v-if="columns[15].visible" />
         <el-table-column label="会员积分" align="center" v-if="columns[12].visible">
            <template #default="scope">
               <el-tooltip content="查看历史记录" placement="top">
                  <el-button link @click="queryIntegralList(scope.row.userId)" v-hasPermi="['system:record:list']">{{
                     scope.row.integral }}</el-button>
               </el-tooltip>
            </template>
         </el-table-column>
         <el-table-column label="会员等级" align="center" key="postName" prop="postName" v-if="columns[8].visible" />
         <el-table-column label="会员画像" align="center" key="userTags" prop="userTags" v-if="columns[13].visible">
            <template #default="scope">
               <!-- 检查 userTags 是否为空或为 null -->
               <template v-if="!scope.row.userTags || scope.row.userTags.trim() === ''">
                  <span>-</span>
               </template>
               <!-- 如果 userTags 不为空，则显示 dict-tag，并设置 el-tooltip -->
               <template v-else>
                  <!-- el-tooltip 组件包裹 dict-tag -->
                  <el-tooltip class="item" v-if="scope.row.tagsRemark" effect="dark" :content="scope.row.tagsRemark"
                     placement="top">
                     <!-- el-tooltip 内部只能包裹一个元素 -->
                     <div class="user-tags-container">
                        <dict-tag v-for="(item, index) in scope.row.userTags.split(',')" :key="index"
                           :options="sys_user_tags" :value="item.trim()" />
                     </div>
                  </el-tooltip>
                  <!-- 如果没有会员画像备注，那么不要显示tooltip -->
                  <template v-else>
                     <dict-tag v-for="(item, index) in scope.row.userTags.split(',')" :key="index"
                        :options="sys_user_tags" :value="item.trim()" />
                  </template>
               </template>
            </template>
         </el-table-column>
         <el-table-column label="黑灰名单" align="center" key="identify" v-if="columns[14].visible">
            <template #default="scope">
               <dict-tag :options="sys_user_identify" :value="scope.row.identify" />
            </template>
         </el-table-column>
         <el-table-column label="账号状态" align="center" key="status" v-if="columns[5].visible">
            <template #default="scope">
               <el-switch v-model="scope.row.status" active-value="0" inactive-value="1"
                  @change="handleStatusChange(scope.row)"></el-switch>
            </template>
         </el-table-column>
         <el-table-column label="创建时间" align="center" prop="createTime" v-if="columns[6].visible" width="160">
            <template #default="scope">
               <span>{{ parseTime(scope.row.createTime) }}</span>
            </template>
         </el-table-column>
         <el-table-column label="备注信息" align="center" prop="remark" :show-overflow-tooltip="true"
            v-if="columns[9].visible" />
         <el-table-column label="操作" align="center" width="150" class-name="small-padding fixed-width">
            <template #default="scope">
               <el-tooltip content="编辑" placement="top">
                  <el-button link type="primary" icon="Edit" @click="handleUpdate(scope.row)"
                     v-hasPermi="['system:user:edit']"></el-button>
               </el-tooltip>
               <el-tooltip content="删除" placement="top">
                  <el-button link type="primary" icon="Delete" @click="handleDelete(scope.row)"
                     v-hasPermi="['system:user:remove']"></el-button>
               </el-tooltip>
               <el-tooltip content="兑换" placement="top">
                  <el-button link type="primary" icon="Shop" @click="handleResetPwd(scope.row)"
                     v-hasPermi="['system:user:resetPwd']"></el-button>
               </el-tooltip>
            </template>
         </el-table-column>
      </el-table>
      <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
         v-model:limit="queryParams.pageSize" @pagination="getList" />

      <!-- 添加或修改会员配置对话框 -->
      <el-dialog :title="title" v-model="open" width="600px" append-to-body close-on-click-modal="false">
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
                        <el-radio v-for="dict in sys_normal_disable" :key="dict.value" :value="dict.value">{{ dict.label
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

      <!-- 会员积分历史记录 -->
      <el-dialog v-model="integralListOpen" width="600px" append-to-body>
         <el-table v-loading="integralLoading" :data="integralList">
            <el-table-column label="ID" align="center" prop="id" />
            <el-table-column label="本次使用积分数量" align="center" prop="identify" />
            <el-table-column label="使用时间" align="center" prop="createTime" width="160">
               <template #default="scope">
                  <span>{{ parseTime(scope.row.createTime) }}</span>
               </template>
            </el-table-column>
         </el-table>
      </el-dialog>

      <!-- 展示会员信息详情 -->
      <el-dialog v-model="userInfoOpen" :show-close="false" width="500px" append-to-body>
         <UserDetailsCard :user="userInfo" />
      </el-dialog>
   </div>
</template>

<script setup name="User">
import { changeUserStatus, listUser, resetUserPwd, delUser, getUser, updateUser, addUser } from "@/api/system/user";
import { listRecord } from "@/api/system/record";
import UserDetailsCard from './info.vue';

const { proxy } = getCurrentInstance();
const {
   sys_normal_disable,
   sys_user_tags,
   sys_user_sex,
   sys_user_type,
   sys_user_identify
} = proxy.useDict("sys_normal_disable", "sys_user_tags", "sys_user_sex", "sys_user_type", "sys_user_identify");

const userList = ref([]);
const integralList = ref([]);
const integralLoading = ref(true);
const integralListOpen = ref(false);
const userInfoOpen = ref(false);
const userInfo = ref({});
const open = ref(false);
const loading = ref(true);
const showSearch = ref(true);
const ids = ref([]);
const single = ref(true);
const multiple = ref(true);
const total = ref(0);
const title = ref("");
const deptName = ref("");
// const deptOptions = ref(undefined);
const initPassword = ref(undefined);
const postOptions = ref([]);
const roleOptions = ref([]);

// 列显隐信息
const columns = ref([
   { key: 0, label: `会员编号`, visible: false },
   { key: 1, label: `会员账号`, visible: true },
   { key: 2, label: `会员姓名`, visible: true },
   { key: 3, label: `归属组织`, visible: true },
   { key: 4, label: `手机号码`, visible: true },
   { key: 5, label: `账号状态`, visible: false },
   { key: 6, label: `创建时间`, visible: false },
   { key: 7, label: `会员类型`, visible: false },
   { key: 8, label: `会员等级`, visible: true },
   { key: 9, label: `备注信息`, visible: true },
   { key: 10, label: `微信标识`, visible: false },
   { key: 11, label: `会员性别`, visible: false },
   { key: 12, label: `会员积分`, visible: true },
   { key: 13, label: `会员画像`, visible: true },
   { key: 14, label: `黑灰名单`, visible: false },
   { key: 14, label: `会员住址`, visible: false },
]);

const data = reactive({
   form: {},
   queryParams: {
      pageNum: 1,
      pageSize: 10,
      userName: undefined,
      phonenumber: undefined,
      postName: undefined,
      deptId: undefined
   },
   rules: {
      userName: [{ min: 2, max: 20, message: "会员账号长度必须介于 4 和 30 之间", trigger: "blur" }],
      nickName: [{ required: true, message: "会员姓名不能为空", trigger: "blur" }],
      password: [{ required: true, message: "会员密码不能为空", trigger: "blur" }, { min: 5, max: 20, message: "会员密码长度必须介于 5 和 20 之间", trigger: "blur" }, { pattern: /^[^<>"'|\\]+$/, message: "不能包含非法字符：< > \" ' \\\ |", trigger: "blur" }],
      email: [{ type: "email", message: "请输入正确的邮箱地址", trigger: ["blur", "change"] }],
      phonenumber: [{ required: true, message: "手机号不能为空", trigger: "blur" }, { pattern: /^1[3|4|5|6|7|8|9][0-9]\d{8}$/, message: "请输入正确的手机号码", trigger: "blur" }]
   }
});

const { queryParams, form, rules } = toRefs(data);

/** 根据名称筛选组织树 */
watch(deptName, val => {
   proxy.$refs["deptTreeRef"].filter(val);
});

/** 查询会员列表 */
function getList() {
   loading.value = true;
   listUser(queryParams.value).then(res => {
      loading.value = false;
      userList.value = res.rows;
      total.value = res.total;
   });
};

/** 搜索按钮操作 */
function handleQuery() {
   queryParams.value.pageNum = 1;
   getList();
};

/* 手机号触发查询 */
function handleTelQuery() {
   if (queryParams.value.phonenumber && queryParams.value.phonenumber.length >= 4) {
      queryParams.value.pageNum = 1;
      getList();
   }
};

/** 重置按钮操作 */
function resetQuery() {
   proxy.resetForm("queryRef");
   queryParams.value.deptId = undefined;
   proxy.$refs.deptTreeRef.setCurrentKey(null);
   handleQuery();
};

/** 删除按钮操作 */
function handleDelete(row) {
   const userIds = row.userId || ids.value;
   proxy.$modal.confirm('是否确认删除会员编号为"' + userIds + '"的数据项？').then(function () {
      return delUser(userIds);
   }).then(() => {
      getList();
      proxy.$modal.msgSuccess("删除成功");
   }).catch(() => { });
};

/** 会员状态修改  */
function handleStatusChange(row) {
   let text = row.status === "0" ? "启用" : "停用";
   proxy.$modal.confirm('确认要"' + text + '""' + row.userName + '"会员吗?').then(function () {
      return changeUserStatus(row.userId, row.status);
   }).then(() => {
      proxy.$modal.msgSuccess(text + "成功");
   }).catch(function () {
      row.status = row.status === "0" ? "1" : "0";
   });
};

/** 重置密码按钮操作 */
function handleResetPwd(row) {
   proxy.$prompt('请输入"' + row.userName + '"的新密码', "提示", {
      confirmButtonText: "确定",
      cancelButtonText: "取消",
      closeOnClickModal: false,
      inputPattern: /^.{5,20}$/,
      inputErrorMessage: "会员密码长度必须介于 5 和 20 之间",
      inputValidator: (value) => {
         if (/<|>|"|'|\||\\/.test(value)) {
            return "不能包含非法字符：< > \" ' \\\ |"
         }
      },
   }).then(({ value }) => {
      resetUserPwd(row.userId, value).then(response => {
         proxy.$modal.msgSuccess("修改成功，新密码是：" + value);
      });
   }).catch(() => { });
};

/** 选择条数  */
function handleSelectionChange(selection) {
   ids.value = selection.map(item => item.userId);
   single.value = selection.length != 1;
   multiple.value = !selection.length;
};

/** 重置操作表单 */
function reset() {
   form.value = {
      userId: undefined,
      deptId: undefined,
      userName: undefined,
      nickName: undefined,
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
};

/* 获取会员等级下拉列表 */
function getPostList() {
   getUser().then(response => {
      postOptions.value = response.posts;
   });
};

/** 新增按钮操作 */
function handleAdd() {
   reset();
   getUser().then(response => {
      postOptions.value = response.posts;
      roleOptions.value = response.roles;
      open.value = true;
      title.value = "添加会员";
      form.value.password = initPassword.value;
   });
};

/** 修改按钮操作 */
function handleUpdate(row) {
   reset();
   const userId = row.userId || ids.value;
   getUser(userId).then(response => {
      form.value = response.data;
      postOptions.value = response.posts;
      roleOptions.value = response.roles;
      form.value.postIds = response.postIds;
      form.value.roleIds = response.roleIds;
      if (form.value.userTags && form.value.userTags.length > 0) {
         form.value.userTagsArr = form.value.userTags.split(",");
      }
      open.value = true;
      title.value = "修改会员";
      form.password = "";
      console.log(form.value)
   });
};

/* 根据用户id查询积分使用列表 */
function queryIntegralList(userId) {
   integralListOpen.value = true;
   integralLoading.value = true;
   listRecord({ userId }).then(response => {
      integralList.value = response.rows;
      integralLoading.value = false;
   });
};

/* 展示用户信息详情 */
function showUserInfo(info) {
   userInfoOpen.value = true;
   userInfo.value = info;
};

/** 提交按钮 */
function submitForm() {
   proxy.$refs["userRef"].validate(valid => {
      if (valid) {
         if (form.value.userId != undefined) {
            if (form.value.userTagsArr && form.value.userTagsArr.length > 0) {
               form.value.userTags = form.value.userTagsArr.join(",");
               delete form.value.userTagsArr;
            }

            updateUser(form.value).then(response => {
               proxy.$modal.msgSuccess("修改成功");
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
               open.value = false;
               getList();
            });
         }
      }
   });
};

getPostList();
getList();
</script>

<style scoped>
/* 设置会员画像样式；水平排列自动换行 */
.user-tags-container {
   display: flex;
   justify-content: center;
   align-items: center;
   flex-flow: row wrap
}
</style>