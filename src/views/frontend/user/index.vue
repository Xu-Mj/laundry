<template>
   <div class="app-container">

      <!-- 搜索区域 -->
      <transition name="height-fade">
         <el-card class="search-card" v-show="showSearch">
            <el-form :model="queryParams" ref="queryRef" :inline="true" label-width="68px">
               <el-form-item label="会员账号" prop="userName" size="large">
                  <el-input size="large" v-model="queryParams.userName" placeholder="请输入会员账号" clearable
                     style="width: 240px" @keyup.enter="handleQuery" />
               </el-form-item>
               <el-form-item label="手机号码" prop="phonenumber" size="large">
                  <el-input size="large" v-model="queryParams.phonenumber" placeholder="请输入手机号码" clearable
                     style="width: 240px" @keyup.enter="handleQuery" @input="handleTelQuery" />
               </el-form-item>
               <el-form-item label="会员等级" prop="levelName" size="large">
                  <el-select size="large" v-model="queryParams.levelId" placeholder="会员等级" clearable
                     style="width: 240px">
                     <el-option v-for="item in levelOptions" :key="item.levelId" :label="item.levelName"
                        :value="item.levelId" :disabled="item.status == 1"></el-option>
                  </el-select>
               </el-form-item>
               <el-form-item>
                  <el-button type="primary" icon="Search" @click="handleQuery" size="large">搜索</el-button>
                  <el-button icon="Refresh" @click="resetQuery" size="large">重置</el-button>
               </el-form-item>
            </el-form>
         </el-card>
      </transition>

      <el-card class="table-card">
         <el-row :gutter="10" class="mb8">
            <el-col :span="1.5">
               <el-button type="primary" plain icon="Plus" @click="handleAdd">新增</el-button>
            </el-col>
            <el-col :span="1.5">
               <el-button type="success" plain icon="Edit" :disabled="single" @click="handleUpdate">修改</el-button>
            </el-col>
            <el-col :span="1.5">
               <el-button type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete">删除</el-button>
            </el-col>
            <right-toolbar v-model:showSearch="showSearch" @queryTable="getList" :columns="columns"></right-toolbar>
         </el-row>

         <el-table v-loading="loading" :show-close="false" :data="userList" @selection-change="handleSelectionChange"
            class="modern-table" stripe>
            <el-table-column type="selection" width="50" align="center" />
            <!-- <el-table-column label="会员编号" align="center" key="userId" prop="userId" v-if="columns[0].visible" /> -->
            <el-table-column label="会员姓名" align="center" v-if="columns[1].visible">
               <template #default="scope">
                  <el-tooltip content="查看会员详情" placement="top">
                     <el-button link type="primary" @click="showUserInfo(scope.row)">{{ scope.row.nickName
                     }}</el-button>
                  </el-tooltip>
               </template>
            </el-table-column>
            <el-table-column label="会员账号" align="center" key="userName" prop="userName" v-if="columns[0].visible"
               :show-overflow-tooltip="true" />
            <el-table-column label="手机号码" align="center" key="phonenumber" prop="phonenumber" v-if="columns[3].visible"
               width="120" />
            <el-table-column label="会员类型" align="center" key="userType" prop="userType" v-if="columns[6].visible">
               <template #default="scope">
                  <dict-tag :options="sys_user_type" :value="scope.row.userType" />
               </template>
            </el-table-column>
            <!-- <el-table-column label="会员组织" align="center" key="deptName" prop="dept.deptName" v-if="columns[3].visible"
         :show-overflow-tooltip="true" /> -->
            <el-table-column label="微信标识" align="center" prop="openId" :show-overflow-tooltip="true"
               v-if="columns[9].visible" />
            <el-table-column label="性别" align="center" key="sex" v-if="columns[10].visible">
               <template #default="scope">
                  <dict-tag :options="sys_user_sex" :value="scope.row.sex" />
               </template>
            </el-table-column>
            <el-table-column label="会员住址" align="center" key="address" prop="address" v-if="columns[14].visible" />
            <el-table-column label="余额" align="center" prop="balance" v-if="columns[15].visible">
               <template #default="scope">
                  <span style="color: red;">
                     {{ scope.row.balance }}
                  </span>
                  元
               </template>
            </el-table-column>
            <el-table-column label="会员积分" align="center" prop="integral" v-if="columns[11].visible" />
            <!-- <el-table-column label="会员积分" align="center" v-if="columns[11].visible">
               <template #default="scope">
                  <el-tooltip content="查看历史记录" placement="top">
                     <el-button type="primary" link @click="queryIntegralList(scope.row.userId)">
                        {{ scope.row.integral }}</el-button>
                  </el-tooltip>
               </template>
            </el-table-column> -->
            <el-table-column label="会员等级" align="center" key="levelName" prop="levelName" v-if="columns[7].visible" />
            <el-table-column label="会员画像" align="center" key="userTags" prop="userTags" v-if="columns[12].visible">
               <template #default="scope">
                  <!-- 如果 userTags 不为空，则显示 dict-tag，并设置 el-tooltip -->
                  <template v-if="scope.row.userTags && scope.row.userTags.trim() !== ''">
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
                        <div class="user-tags-container">
                           <dict-tag v-for="(item, index) in scope.row.userTags.split(',')" :key="index"
                              :options="sys_user_tags" :value="item.trim()" />
                        </div>
                     </template>
                  </template>
               </template>
            </el-table-column>
            <el-table-column label="黑灰名单" align="center" key="identify" v-if="columns[13].visible">
               <template #default="scope">
                  <dict-tag :options="sys_user_identify" :value="scope.row.identify" />
               </template>
            </el-table-column>
            <el-table-column label="账号状态" align="center" key="status" v-if="columns[4].visible">
               <template #default="scope">
                  <el-switch v-model="scope.row.status" active-value="0" inactive-value="1"
                     @change="handleStatusChange(scope.row)"></el-switch>
               </template>
            </el-table-column>
            <el-table-column label="创建时间" align="center" prop="createTime" v-if="columns[5].visible">
               <template #default="scope">
                  <span>{{ parseTime(scope.row.createTime) }}</span>
               </template>
            </el-table-column>
            <el-table-column label="备注信息" align="center" prop="remark" show-overflow-tooltip
               v-if="columns[8].visible" />
            <el-table-column label="操作" align="center" width="150" class-name="small-padding fixed-width">
               <template #default="scope">
                  <el-tooltip content="编辑" placement="top">
                     <el-button link type="primary" icon="Edit" @click="handleUpdate(scope.row)" />
                  </el-tooltip>
                  <el-tooltip content="删除" placement="top">
                     <el-button link type="primary" icon="Delete" @click="handleDelete(scope.row)" />
                  </el-tooltip>
                  <el-tooltip content="重置密码" placement="top">
                     <el-button link type="primary" icon="Key" @click="handleResetPwd(scope.row)" />
                  </el-tooltip>
               </template>
            </el-table-column>
         </el-table>
         <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
            v-model:limit="queryParams.pageSize" @pagination="getList" />
      </el-card>

      <!-- 添加或修改会员配置对话框 -->
      <AddUser :visible="open" :key="open" :userId="needUpdateUserId"
         :taggle="() => { needUpdateUserId = 0; open = !open }" />
      <!-- 会员积分历史记录 -->
      <el-dialog v-model="integralListOpen" width="600px" append-to-body>
         <el-table v-loading="integralLoading" :data="integralList">
            <!-- <el-table-column label="ID" align="center" prop="id" /> -->
            <el-table-column label="兑换数量" align="center" prop="identify" />
            <el-table-column label="兑换卡券" align="center" prop="couponTitle" />
            <el-table-column label="使用时间" align="center" prop="createTime" width="160">
               <template #default="scope">
                  <span>{{ parseTime(scope.row.createTime) }}</span>
               </template>
            </el-table-column>
         </el-table>
      </el-dialog>

      <!-- 展示会员信息详情 -->
      <Information :user="userInfo" :visible="userInfoOpen" :key="userInfoOpen"
         :toggle="() => { userInfoOpen = !userInfoOpen }" />
   </div>
</template>

<script setup name="User">
import { changeUserStatus, listUser, resetUserPwd, delUser } from "@/api/system/user";
import { listRecord } from "@/api/system/record";
import { listPostAll } from "@/api/system/post";
import Information from "@/views/frontend/user/information.vue";
import AddUser from '@/views/components/addUser.vue';

const { proxy } = getCurrentInstance();
const {
   sys_user_tags,
   sys_user_sex,
   sys_user_type,
   sys_user_identify
} = proxy.useDict("sys_user_tags", "sys_user_sex", "sys_user_type", "sys_user_identify");

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
const needUpdateUserId = ref();
const levelOptions = ref([]);
// const roleOptions = ref([]);

// 列显隐信息
const columns = ref([
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
   { key: 15, label: `会员住址`, visible: false },
   { key: 16, label: `余额`, visible: true },
]);

const data = reactive({
   form: {},
   queryParams: {
      pageNum: 1,
      pageSize: 10,
      userName: undefined,
      phonenumber: undefined,
      levelName: undefined,
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

// Save column visibility to local storage
const saveColumnVisibility = () => {
   localStorage.setItem('userColumns', JSON.stringify(columns.value));
};

// Retrieve column visibility from local storage
const loadColumnVisibility = () => {
   const savedColumns = localStorage.getItem('userColumns');
   if (savedColumns) {
      columns.value = JSON.parse(savedColumns);
   }
};

// Watch for changes in column visibility and save to local storage
watch(columns, saveColumnVisibility, { deep: true });

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
   handleQuery();
};

/** 删除按钮操作 */
function handleDelete(row) {
   if (row && row.balance > 0) {
      proxy.notify.warning("会员余额大于0，无法删除！");
      return;
   } else if (!row && ids.value.length > 0) {
      // query user list by ids
      if (userList.value.filter(item => ids.value.contains(item.userId)).filter(item => item.balance > 0).length > 0) {
         proxy.notify.warning("存在会员余额大于0的用户，无法删除！");
         return;
      }
   }


   const userIds = row.userId || ids.value;
   proxy.$modal.confirm('是否确认删除会员编号为"' + userIds + '"的数据项？').then(function () {
      return delUser(userIds);
   }).then(() => {
      getList();
      proxy.notify.success("删除成功");
   }).catch(() => { });
};

/** 会员状态修改  */
function handleStatusChange(row) {
   let text = row.status === "0" ? "启用" : "停用";
   proxy.$modal.confirm('确认要"' + text + '""' + row.userName + '"会员吗?').then(function () {
      return changeUserStatus(row.userId, row.status);
   }).then(() => {
      proxy.notify.success(text + "成功");
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
         proxy.notify.success("修改成功，新密码是：" + value);
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
      userType: "01",
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
   listPostAll().then(response => {
      levelOptions.value = response;
   });
};

/** 新增按钮操作 */
function handleAdd() {
   reset();
   open.value = true;
};

/** 修改按钮操作 */
function handleUpdate(row) {
   needUpdateUserId.value = row.userId || ids.value;
   open.value = true;
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

loadColumnVisibility();
getPostList();
getList();
</script>

<style scoped>
/* 设置会员画像样式；水平排列自动换行 */
.user-tags-container {
   display: flex;
   justify-content: center;
   align-items: center;
   flex-flow: row wrap;
   gap: .5rem;
}
</style>