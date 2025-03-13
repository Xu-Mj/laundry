<template>
  <div class="app-container">
    <el-form :model="queryParams" ref="queryRef" :inline="true" v-show="showSearch" label-width="68px">
      <el-form-item label="推广方式" prop="promoteMethod">
        <el-input v-model="queryParams.promoteMethod" placeholder="请输入推广方式" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" icon="Search" @click="handleQuery">搜索</el-button>
        <el-button icon="Refresh" @click="resetQuery">重置</el-button>
      </el-form-item>
    </el-form>

    <el-row :gutter="10" class="mb8">
      <el-col :span="1.5">
        <el-button type="primary" plain icon="Plus" @click="handleAdd"
          v-hasPermi="['system:template:add']">新增</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button type="success" plain icon="Edit" :disabled="single" @click="handleUpdate"
          v-hasPermi="['system:template:edit']">修改</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete"
          v-hasPermi="['system:template:remove']">删除</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button type="primary" icon="Management">
          <router-link to="/system/promote-record" class="link-type">
            推广记录查询
          </router-link>
        </el-button>
      </el-col>
      <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
    </el-row>

    <el-table v-loading="loading" :data="templateList" @selection-change="handleSelectionChange">
      <el-table-column type="selection" width="55" align="center" />
      <el-table-column label="推广内容" align="center" prop="content" />
      <el-table-column label="推广类型" align="center" prop="promoteType">
        <template #default="scope">
          <dict-tag :options="sys_promote_type" :value="scope.row.promoteType" />
        </template>
      </el-table-column>
      <el-table-column label="推广方式" align="center" prop="promoteMethod">
        <template #default="scope">
          <dict-tag :options="sys_promote_method" :value="scope.row.promoteMethod" />
        </template>
      </el-table-column>
      <el-table-column label="推广次数" align="center" prop="promoteCount" />
      <el-table-column label="推广对象" align="center">
        <template #default="scope">
          <span v-if="!scope.row.promoteObjects || scope.row.promoteObjects === ''">未选择</span>
          <span v-else-if="scope.row.promoteObjects === '-1'">全部</span>
          <el-button v-else type="primary" link @click="showPromoteObjects(scope.row)">推广对象</el-button>
        </template>
      </el-table-column>
      <el-table-column label="推广图片" align="center">
        <template #default="scope">
          <span v-if="!scope.row.promotePicture || scope.row.promotePicture === ''">未设置</span>
          <el-image v-else :src="pictureUrl + scope.row.promotePicture" :preview-src-list="pictureList"
            style="width: 100px; height: auto" />
        </template>
      </el-table-column>
      <el-table-column label="是否置顶" align="center" prop="isPin" />
      <el-table-column label="推广描述" align="center" prop="remark" />
      <el-table-column label="操作" align="center" class-name="small-padding fixed-width">
        <template #default="scope">
          <el-button link type="primary" @click="handlePromote(scope.row)"
            v-hasPermi="['system:template:promote']">立即推广</el-button>
          <el-button link type="primary" @click="handleUpdate(scope.row)"
            v-hasPermi="['system:template:edit']">修改</el-button>
        </template>
      </el-table-column>
    </el-table>

    <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
      v-model:limit="queryParams.pageSize" @pagination="getList" />

    <!-- 添加或修改推广模板对话框 -->
    <el-dialog :title="title" v-model="open" width="500px" append-to-body @closed="resetObjects">
      <el-form ref="templateRef" :model="form" :rules="rules" label-width="80px">
        <el-form-item label="推广内容" prop="content">
          <el-input type="textarea" v-model="form.content" placeholder="输入内容" />
        </el-form-item>
        <el-form-item label="推广类型" prop="promoteType">
          <el-radio-group v-model="form.promoteType">
            <el-radio v-for="dict in sys_promote_type" :key="dict.value" :value="dict.value">
              {{ dict.label }}
            </el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item label="推广方式" prop="promoteMethod">
          <el-radio-group v-model="form.promoteMethod">
            <el-radio v-for="dict in sys_promote_method" :key="dict.value" :value="dict.value">
              {{ dict.label }}
            </el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item label="是否置顶" prop="isPin">
          <el-radio-group v-model="form.isPin">
            <el-radio :value="'0'">是</el-radio>
            <el-radio :value="'1'">否</el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item label="推广描述" prop="remark">
          <el-input type="textarea" v-model="form.remark" placeholder="请输入推广描述" />
        </el-form-item>
        <el-form-item label="推广图片" prop="promotePicture" v-show="form.promoteMethod !== '01'">
          <el-upload class="upload-demo" :action="uploadBeforeImgUrl" :headers="headers" :initial-index="4" fit="cover"
            :on-preview="handlePreview" :on-remove="handleRemovePicture" :on-success="handleUploadPreSucess"
            list-type="picture">
            <el-button type="primary">点击上传推广图片</el-button>
          </el-upload>
        </el-form-item>
        <el-form-item label="推广对象" prop="promoteObjects">
          <el-row>
            <el-button link @click="handleSelectObjects">选择推广对象</el-button>
            <el-button link v-if="form.promoteObjects"
              @click="showObjectsInUpdate = !showObjectsInUpdate">点击展开</el-button>
          </el-row>
          <el-row v-if="showObjectsInUpdate">
            <span v-for="user in objectList" :key="user.userId">
              {{ user.nickName + '-' + user.phonenumber }}
            </span>
          </el-row>
        </el-form-item>
      </el-form>
      <template #footer>
        <div class="dialog-footer">
          <el-button type="primary" @click="submitForm">确 定</el-button>
          <el-button @click="cancel">取 消</el-button>
        </div>
      </template>
    </el-dialog>
    <!-- 立即推广确认框 -->
    <el-dialog :show-close="false" v-model="showPromoteConfirm" width="500px" append-to-body>
      确认立即发送推广信息至指定会员？
      <template #footer>
        <div class="dialog-footer">
          <el-button type="primary" @click="modifyObjects">修改推广对象</el-button>
          <el-button type="primary" @click="promoteImmediately">确认</el-button>
          <el-button @click="cancelPromote">取 消</el-button>
        </div>
      </template>
    </el-dialog>


    <el-dialog v-model="showSelectUserPanel" width="660px" :height="500" append-to-body>
      <el-form-item label="会员列表：">
        <el-radio-group v-model="sendAll" @change="selectUserChange">
          <el-radio label="全部" :value="true" />
          <el-radio label="部分" :value="false" />
        </el-radio-group>
      </el-form-item>
      <el-row class="user-list-area" v-if="!sendAll">
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
      </el-row>
      <template #footer>
        <div class="dialog-footer">
          <el-button type="primary" @click="saveObjects">保 存</el-button>
          <el-button @click="() => { userList.forEach(user => user.selected = false); showSelectUserPanel = false; }">
            取消
          </el-button>
        </div>
      </template>
    </el-dialog>

    <el-dialog title="推广对象" v-model="showPromoteObjectsPanel" width="650px" :show-close="false" append-to-body>
      <div class="objects-container">
        <span v-for="user in objectList" :key="user.userId">
          {{ user.nickName + '-' + user.phonenumber }}
        </span>
      </div>
      <template #footer>
        <div class="dialog-footer">
          <el-button type="primary" @click="showPromoteObjectsPanel = !showPromoteObjectsPanel">确 定</el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup name="Template">
import { listTemplate, getTemplate, delTemplate, addTemplate, updateTemplate } from "@/api/system/promote_temp";
import { listUserWithNoLimit } from "@/api/system/user";
import { getToken } from "@/utils/auth";
import { getUserListByIds } from "@/api/system/user";
import { promote } from "@/api/system/promote_temp";

const { proxy } = getCurrentInstance();
const { sys_promote_type, sys_promote_method } = proxy.useDict('sys_promote_type', 'sys_promote_method');

const userList = ref([]);
const objectList = ref([]);
const templateList = ref([]);
const open = ref(false);
const showSelectUserPanel = ref(false);
const showPromoteObjectsPanel = ref(false);
const showObjectsInUpdate = ref(false);
const showPicture = ref(false);
const sendAll = ref(false);
const loading = ref(true);
const showSearch = ref(true);
const ids = ref([]);
const single = ref(true);
const multiple = ref(true);
const showPromoteConfirm = ref(false);
const selectUserOpen = ref(false);
const showObjects = ref(false);
const total = ref(0);
const title = ref("");
const filterUserByTel = ref(null);
const filterUserByScore = ref(null);
const pictureList = ref([]);
const currentTemp = ref(null);

const headers = ref({ Authorization: "Bearer " + getToken() });
const baseUrl = import.meta.env.VITE_APP_BASE_API;
const uploadBeforeImgUrl = baseUrl + `/system/promote-template/upload`;
const pictureUrl = ref(baseUrl + "/system/cloths/download/");

const data = reactive({
  form: {},
  queryParams: {
    pageNum: 1,
    pageSize: 10,
    content: null,
    promoteType: null,
    promoteMethod: null,
    promoteCount: null,
    promoteObjects: null,
    promotePicture: null,
    isPin: null,
  },
  rules: {
    content: [
      { required: true, message: "推广内容不能为空", trigger: "blur" }
    ],
    promoteType: [
      { required: true, message: "推广类型不能为空", trigger: "change" }
    ],
    promoteMethod: [
      { required: true, message: "推广方式不能为空", trigger: "blur" }
    ]
  }
});

const { queryParams, form, rules } = toRefs(data);

function saveObjects() {
  if (sendAll.value) {
    form.value.promoteObjects = '-1';
  } else {
    const selectedUsers = userList.value.filter(user => user.selected);
    const userIds = selectedUsers.map(user => user.userId);
    form.value.promoteObjects = userIds.join(',');
  }
  showSelectUserPanel.value = false;
}

function resetObjects() {
  objectList.value = [];
}

function handlePromote(row) {
  currentTemp.value = row;
  showPromoteConfirm.value = true;
  // promote(row).then(res => {
  //   proxy.notify.success('推广成功');
  // }).catch(res => {
  //   proxy.$modal.msgError(res.message);
  // })
}

async function modifyObjects() {
  const value = currentTemp.value;
  if (!value.promoteObjects) {
    const response = await listUserWithNoLimit({});
    userList.value = response.rows;
    userList.value.forEach(item => { item.selected = false; item.show = true; });
  } else if (value.promoteObjects == '-1') {
    // 全部
    sendAll.value = true;
  } else if (value.promoteObjects) {
    const userIds = row.promoteObjects.split(',').map(item => Number(item));
    const response = await listUserWithNoLimit({});
    userList.value = response.rows;
    userList.value.forEach(item => { item.selected = userIds.includes(item.userId); item.show = true; });
  }
  showSelectUserPanel.value = true;
}

function promoteImmediately(row) {
  // 判断是否有推广对象
  if (!row.promoteObjects) {
    proxy.$modal.msgError('请选择推广对象');
    return;
  }
  promote(row).then(res => {
    proxy.notify.success('推广成功');
  }).catch(res => {
    proxy.$modal.msgError(res.message);
  })
}

function handlePreview(row, flag) {
  showPicture.value = true;
  getCloths(row.clothId).then(response => {
    if (flag) {
      pictureList.value = response.data.beforePics ?
        response.data.beforePics.split(',').map(item => pictureUrl.value + item) : [];
    } else {
      pictureList.value = response.data.afterPics ?
        response.data.afterPics.split(',').map(item => pictureUrl.value + item) : [];
    }
  });
}

function handleRemovePicture(event) {
  form.value.promotePicture = null;
}

function handleUploadPreSucess(event) {
  proxy.notify.success("上传成功");
  form.value.promotePicture = event.id;
}

function handleUploadAfterSucess(event) {
  afterPictureList.value.unshift({ id: event.id, url: pictureUrl.value + event.id });
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

async function selectUser() {
  const response = await listUserWithNoLimit({});
  userList.value = response.rows;
  userList.value.forEach(item => { item.selected = false; item.show = true; });
};

/** 选择所有用户 */
function selectAllUsers() {
  userList.value.forEach(item => {
    if (item.show) {
      item.selected = true;
    }
  });
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
    }
  });
}

async function handleSelectObjects() {
  // const response = await listUserWithNoLimit({});
  // userList.value = response.rows;
  // if (form.value.promoteObjects) {
  //   const ids = form.value.promoteObjects.split(',');
  //   userList.value.forEach(item => { item.selected = ids.includes(item.userId); item.show = true; });
  // } else {
  //   userList.value.forEach(item => { item.selected = false; item.show = true; });
  // }
  if (!form.value.promoteObjects) {
    const response = await listUserWithNoLimit({});
    userList.value = response.rows;
    userList.value.forEach(item => { item.selected = false; item.show = true; });
  } else if (form.value.promoteObjects == '-1') {
    // 全部
    sendAll.value = true;
  } else if (form.value.promoteObjects) {
    const userIds = form.value.promoteObjects.split(',').map(item => Number(item));
    const response = await listUserWithNoLimit({});
    userList.value = response.rows;
    userList.value.forEach(item => { item.selected = userIds.includes(item.userId); item.show = true; });
  }
  showSelectUserPanel.value = true;
}

function showPromoteObjects(row) {
  // query user list form server
  getUserListByIds(row.promoteObjects.split(',').map(item => Number(item))).then(response => {
    objectList.value = response.rows;
    showPromoteObjectsPanel.value = true;
  })
}

/** 查询推广模板列表 */
function getList() {
  loading.value = true;
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
    id: null,
    content: null,
    promoteType: null,
    promoteMethod: '00',
    promoteCount: null,
    promoteObjects: null,
    createTime: null,
    promotePicture: null,
    isPin: null,
    remark: null
  };
  proxy.resetForm("templateRef");
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
  ids.value = selection.map(item => item.id);
  single.value = selection.length != 1;
  multiple.value = !selection.length;
}

/** 新增按钮操作 */
function handleAdd() {
  reset();
  open.value = true;
  title.value = "添加推广模板";
}

/** 修改按钮操作 */
async function handleUpdate(row) {
  reset();
  const _id = row.id || ids.value
  await getTemplate(_id).then(response => {
    form.value = response.data;
    open.value = true;
    title.value = "修改推广模板";
  });
  if (row.promoteObjects) {
    const ids = row.promoteObjects.split(',').map(item => Number(item));
    if (ids.length > 0) {
      getUserListByIds(ids).then(response => {
        objectList.value = response.rows;
      })
    }
  }
}

/** 提交按钮 */
function submitForm() {
  proxy.$refs["templateRef"].validate(valid => {
    if (valid) {

      if (sendAll.value) {
        form.value.promoteObjects = '-1';
      } else {
        form.value.promoteObjects = userList.value.filter(item => item.selected).map(item => item.userId).join(',');
      }
      if (form.value.id != null) {
        updateTemplate(form.value).then(response => {
          proxy.notify.success("修改成功");
          open.value = false;
          getList();
        });
      } else {
        if (sendAll.value) {
          form.value.promoteObjects = '-1';
        } else {
          form.value.promoteObjects = userList.value.filter(item => item.selected).map(item => item.userId).join(',');
        }
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
  const _ids = row.id || ids.value;
  proxy.$modal.confirm('是否确认删除推广模板编号为"' + _ids + '"的数据项？').then(function () {
    return delTemplate(_ids);
  }).then(() => {
    getList();
    proxy.notify.success("删除成功");
  }).catch(() => { });
}

getList();
</script>

<style scoped>
.link-type {
  color: white;
}

.objects-container {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
}
</style>