<template>
  <div class="app-container">
    <el-card class="search-card" v-show="showSearch">
      <el-form :model="queryParams" ref="queryRef" :inline="true" label-width="68px">
        <el-form-item label="模板名称" prop="tempName">
          <el-input v-model="queryParams.tempName" placeholder="请输入模板名称" clearable @keyup.enter="handleQuery" />
        </el-form-item>
        <el-form-item label="通知方式" prop="noticeMethod">
          <el-select v-model="queryParams.noticeMethod" @change="handleQuery" placeholder="请选择通知方式" clearable
            style="width: 160px">
            <el-option v-for="dict in sys_notice_method" :key="dict.value" :label="dict.label" :value="dict.value" />
          </el-select>
        </el-form-item>
        <el-form-item>
          <el-button class="hover-flow" type="primary" icon="Search" @click="handleQuery">搜索</el-button>
          <el-button class="hover-flow" icon="Refresh" @click="resetQuery">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <el-card class="table-card">
      <el-row :gutter="10" class="mb8">
        <el-col :span="1.5">
          <el-button type="primary" plain icon="Plus" @click="handleAdd">新增</el-button>
        </el-col>
        <el-col :span="1.5">
          <el-button type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete">删除</el-button>
        </el-col>
        <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
      </el-row>

      <el-table v-loading="loading" :data="templateList" @selection-change="handleSelectionChange" class="modern-table"
        stripe>
        <el-table-column type="selection" width="55" align="center" />
        <el-table-column label="模板名称" align="center" prop="tempName" />
        <el-table-column label="通知方式" align="center" prop="noticeMethod">
          <template #default="scope">
            <dict-tag :options="sys_notice_method" :value="scope.row.noticeMethod" />
          </template>
        </el-table-column>
        <el-table-column label="模板内容" align="center" prop="content" />
        <el-table-column label="模板类型" align="center" prop="tempType">
          <template #default="scope">
            <dict-tag :options="sys_temp_type" :value="scope.row.tempType" />
          </template>
        </el-table-column>
        <el-table-column label="备注" align="center" prop="remark" />
        <el-table-column label="操作" align="center" class-name="small-padding fixed-width">
          <template #default="scope">
            <el-button link type="primary" icon="Promotion" v-if="scope.row.tempId !== 1"
              @click="handleSendPanel(scope.row)">发送</el-button>
            <el-button link type="primary" icon="Edit" v-if="scope.row.tempId !== 1"
              @click="handleUpdate(scope.row)">修改</el-button>
            <el-button link type="primary" icon="Delete" v-if="scope.row.tempId !== 1"
              @click="handleDelete(scope.row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>

      <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
        v-model:limit="queryParams.pageSize" @pagination="getList" />
    </el-card>

    <!-- 添加或修改通知模板管理对话框 -->
    <el-dialog :show-close="false" v-model="open" width="500px" append-to-body>
      <el-form ref="templateRef" :model="form" :rules="rules" label-width="80px">
        <el-form-item label="模板名称" prop="tempName">
          <el-input v-model="form.tempName" placeholder="请输入模板名称" />
        </el-form-item>
        <el-form-item label="通知方式" prop="noticeMethod">
          <el-radio-group v-model="form.noticeMethod">
            <el-radio v-for="dict in sys_notice_method" :key="dict.value" :label="dict.label" :value="dict.value" />
          </el-radio-group>
        </el-form-item>
        <el-form-item label="模板类型" prop="tempType">
          <el-radio-group v-model="form.tempType" @change="selectTempType">
            <el-radio v-for="dict in sys_temp_type" :key="dict.value" :label="dict.label" :value="dict.value" />
          </el-radio-group>
        </el-form-item>
        <el-form-item label="模板内容">
          <el-input v-model="form.content" :rows="4" type="textarea" placeholder="请输入内容" />
        </el-form-item>
        <el-form-item label="备注" prop="remark">
          <el-input v-model="form.remark" type="textarea" placeholder="请输入内容" />
        </el-form-item>
      </el-form>
      <template #footer>
        <div class="dialog-footer">
          <el-button type="primary" @click="submitForm">确 定</el-button>
          <el-button type="danger" @click="cancel">取 消</el-button>
        </div>
      </template>
    </el-dialog>

    <el-dialog v-model="sendOpen" width="500px" :height="500" append-to-body>
      <el-row>
        <el-col :span="12">
          <el-form-item label="通知会员">
            <el-radio-group v-model="sendAll" @change="selectUserChange">
              <el-radio label="全部" :value="true" />
              <el-radio label="部分" :value="false" />
            </el-radio-group>
          </el-form-item>
        </el-col>
        <el-col :span="12">
          <el-button v-if="!sendAll" @click="selectUserOpen = !selectUserOpen">选择</el-button>
          <el-button type="primary" @click="send">立即发送</el-button>
        </el-col>
      </el-row>
      <el-row class="user-list-area" v-if="!sendAll">
        <template v-for="user in userList" :key="user.userId">
          <span v-if="user.selected" :class="user.selected ? 'user-item selected' : 'user-item'">
            <span v-if="user.selected" class="del-mask" @click="delItem(user)">点击删除</span>
            <span> {{ user.nickName }}</span>
            <span> {{ user.phonenumber }}</span>
          </span>
        </template>
      </el-row>
    </el-dialog>

    <el-dialog title="选择用户" v-model="selectUserOpen" width="650px" :show-close="false" append-to-body>
      <el-row>
        <el-form :inline="true" label-width="60px">
          <el-form-item label="手机号">
            <el-input @input="filterUserListByTel" v-model="filterUserByTel" placeholder="请输入手机号" />
          </el-form-item>
          <el-form-item label="积分">
            <el-input-number @input="queryUserListByScore" :min="0" v-model="filterUserByScore"
              placeholder="请输入积分进行过滤" />
          </el-form-item>
          <el-form-item>
            <el-button type="primary" @click="selectAllUsers">全选</el-button>
          </el-form-item>
        </el-form>
      </el-row>
      <el-row>
        <template v-for="user in userList" :key="user.userId">
          <el-checkbox v-show="user.show" v-model="user.selected">
            {{ user.nickName + '-' + user.phonenumber }}
          </el-checkbox>
        </template>
      </el-row>
      <template #footer>
        <div class="dialog-footer">
          <el-button type="primary" @click="selectUserOpen = !selectUserOpen">确 定</el-button>
          <el-button @click="() => { userList.forEach(user => user.selected = false); selectUserOpen = false; }">取
            消</el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup name="Template">
import { listTemplate, getTemplate, delTemplate, addTemplate, updateTemplate, sendNotice, sendNotice2All } from "@/api/system/template";
import { listUserWithNoLimit } from "@/api/system/user";

const { proxy } = getCurrentInstance();
const { sys_temp_type, sys_notice_method } = proxy.useDict("sys_temp_type", "sys_notice_method");

const templateList = ref([]);
const userList = ref([]);
const open = ref(false);
const sendOpen = ref(false);
const selectUserOpen = ref(false);
const loading = ref(true);
const showSearch = ref(true);
const ids = ref([]);
const single = ref(true);
const multiple = ref(true);
const total = ref(0);
const title = ref("");
const filterUserByTel = ref(null);
const filterUserByScore = ref(null);

const currentNotice = ref(null);
const sendAll = ref(true);

const data = reactive({
  form: {},
  queryParams: {
    pageNum: 1,
    pageSize: 10,
    tempName: null,
    noticeMethod: null,
    content: null,
    tempType: null,
  },
  rules: {
    tempName: [
      { required: true, message: "模板名称不能为空", trigger: "blur" }
    ],
    noticeMethod: [
      { required: true, message: "通知方式不能为空", trigger: "blur" }
    ],
    content: [
      { required: true, message: "模板内容不能为空", trigger: "blur" }
    ],
    tempType: [
      { required: true, message: "模板类型不能为空", trigger: "change" }
    ],
  }
});

const { queryParams, form, rules } = toRefs(data);

const pickupTemplate = "《会员姓名》您的宝贝护理完成了，请凭取件码《取件码》来取走吧~\n请勿将取件码告知他人，以免造成不必要的麻烦。";

function selectTempType() {
  if (form.value.tempType == "0") {
    form.value.content = pickupTemplate;
  } else {
    form.value.content = null;
  }
}
/** 查询通知模板管理列表 */
function getList() {
  loading.value = true;
  console.log(queryParams.value)
  listTemplate(queryParams.value).then(response => {
    templateList.value = response.rows;
    total.value = response.total;
    loading.value = false;
  });
}

// 取消按钮
function cancel() {
  open.value = false;
  reset();
}

// 表单重置
function reset() {
  form.value = {
    tempId: null,
    tempName: null,
    noticeMethod: "1",
    content: pickupTemplate,
    createTime: null,
    tempType: "0",
    remark: null
  };
  proxy.resetForm("templateRef");
}

/** 选择所有用户 */
function selectAllUsers() {
  userList.value.forEach(item => {
    if (item.show) {
      item.selected = true;
    }
  });
}

/** 搜索按钮操作 */
function handleQuery() {
  queryParams.value.pageNum = 1;
  getList();
}

/** 重置按钮操作 */
function resetQuery() {
  proxy.resetForm("queryRef");
  handleQuery();
}

// 多选框选中数据
function handleSelectionChange(selection) {
  ids.value = selection.map(item => item.tempId);
  single.value = selection.length != 1;
  multiple.value = !selection.length;
}

/** 新增按钮操作 */
function handleAdd() {
  reset();
  open.value = true;
  title.value = "添加通知模板管理";
}

/** 修改按钮操作 */
function handleUpdate(row) {
  reset();
  const _tempId = row.tempId || ids.value
  getTemplate(_tempId).then(response => {
    form.value = response.data;
    open.value = true;
    title.value = "修改通知模板管理";
  });
}

/** 提交按钮 */
function submitForm() {
  proxy.$refs["templateRef"].validate(valid => {
    if (valid) {
      if (form.value.tempId != null) {
        updateTemplate(form.value).then(response => {
          proxy.notify.success("修改成功");
          open.value = false;
          getList();
        });
      } else {
        addTemplate(form.value).then(response => {
          proxy.notify.success("新增成功");
          open.value = false;
          getList();
        });
      }
    }
  });
}

/** 删除按钮操作 */
function handleDelete(row) {
  const _tempIds = row.tempId || ids.value;
  proxy.$modal.confirm('是否确认删除通知模板管理编号为"' + _tempIds + '"的数据项？').then(function () {
    return delTemplate(_tempIds);
  }).then(() => {
    getList();
    proxy.notify.success("删除成功");
  }).catch(() => { });
}

/* 查询用户列表,使用async/await处理，避免第一次获取数据为空 */
async function selectUser() {
  const response = await listUserWithNoLimit({});
  userList.value = response.rows;
  userList.value.forEach(item => { item.selected = false; item.show = true; });
};

async function handleSendPanel(row) {
  currentNotice.value = row;
  sendOpen.value = true;
  await selectUser();
  // if (sendAll.value) {
  //   userList.value.map(item => item.selected = true);
  // }
}

/* 处理选择通知会员按钮变化 */
async function selectUserChange() {
  if (userList.value.length == 0) {
    await selectUser();
  }
  if (sendAll.value) {
    userList.value.map(item => item.selected = true);
  } else {
    userList.value.map(item => item.selected = false);
  }
}

/* 删除已选中的用户 */
function delItem(user) {
  userList.value.map(item => {
    if (item.userId == user.userId) {
      item.selected = !item.selected;
    }
  })
}

// 根据手机号过滤
function filterUserListByTel(tel) {
  userList.value.forEach(user => {
    if (user.phonenumber.includes(tel)) {
      user.show = true;
    } else {
      user.show = false;
      user.selected = false;
    }
  })
}

// 根据手机号过滤
function queryUserListByScore(score) {
  userList.value.forEach(user => {
    if (user.integral >= score) {
      user.show = true;
    } else {
      user.show = false;
      user.selected = false;
    }
  });
  console.log(userList.value);

}

/** 发送按钮操作 */
function send() {
  console.log(currentNotice.value)
  const tempId = currentNotice.value.tempId;
  if (sendAll.value) {
    sendNotice2All(tempId).then(response => {
      proxy.notify.success("发送成功");
      sendOpen.value = false;
    }).catch(() => {

    });
  } else {
    const ids = userList.value.filter(item => item.selected).map(item => item.userId);
    if (ids.length == 0) {
      proxy.notify.error("请选择要发送的用户");
      return;
    }
    // 发送通知
    sendNotice({ userIds: ids, tempId: tempId }).then(response => {
      proxy.notify.success("发送成功");
      sendOpen.value = false;
      userList.value.forEach(user => user.selected = false);
    }).catch(() => {

    });
  }
}

getList();
</script>

<style lang="scss" scoped>
.user-list-area {
  display: flex;
  // line-height: 1.5rem;
  justify-content: space-evenly;
  align-items: center;
  flex-flow: row wrap;
}

.user-item {
  padding: .5rem;
  position: relative;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  border: 1px solid gray;
  border-radius: .3rem;
  cursor: pointer;

  &:hover {

    .del-mask,
    .add-mask {
      width: 100%;
      height: 100%;
      display: flex;
      justify-content: center;
      align-items: center;
      position: absolute;
      top: 0;
      left: 0;
      color: white;
    }

    .del-mask {
      background-color: rgba(#f63131, 0.9);
    }

    .add-mask {
      background-color: rgba($color: #28f815, $alpha: 0.8);
    }
  }
}

.selected {
  border: 1px solid #007bf7;
}

.add-mask {
  display: none;
}

.del-mask {
  display: none;
}
</style>