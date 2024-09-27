<template>
  <div class="app-container">
    <el-form :model="queryParams" ref="queryRef" :inline="true" v-show="showSearch" label-width="68px">
      <el-form-item label="支出账目" prop="expTitle">
        <el-input v-model="queryParams.expTitle" placeholder="请输入支出账目" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="账户名称" prop="recvAccountTitle">
        <el-input v-model="queryParams.recvAccountTitle" placeholder="请输入收款账户名称" clearable @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="支出类型" prop="expType">
        <el-select v-model="queryParams.expType" placeholder="请选择支出类型" clearable style="width: 150px"
          @change="handleQuery">
          <el-option v-for="dict in sys_exp_type" :key="dict.value" :label="dict.label" :value="dict.value"></el-option>
        </el-select>
      </el-form-item>
      <el-form-item label="支出时间" style="width: 308px">
        <el-date-picker v-model="dateRange" value-format="YYYY-MM-DD" type="daterange" range-separator="-"
          start-placeholder="开始日期" end-placeholder="结束日期" @change="handleQuery"></el-date-picker>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" icon="Search" @click="handleQuery">搜索</el-button>
        <el-button icon="Refresh" @click="resetQuery">重置</el-button>
      </el-form-item>
    </el-form>

    <el-row :gutter="10" class="mb8">
      <el-col :span="1.5">
        <el-button type="primary" plain icon="Plus" @click="handleAdd"
          v-hasPermi="['system:expenditure:add']">新增</el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete"
          v-hasPermi="['system:expenditure:remove']">删除</el-button>
      </el-col>
      <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
    </el-row>

    <el-table v-loading="loading" :data="expenditureList" @selection-change="handleSelectionChange">
      <el-table-column type="selection" width="55" align="center" />
      <el-table-column label="支出账目" align="center" prop="expTitle">
        <template #default="scope">
          <el-button v-if="scope.row.expType == '00' || scope.row.expType == '03'" link type="primary"
            @click="showOrderInfo(scope.row)">{{ scope.row.expTitle }}</el-button>
          <span v-else>>{{ scope.row.expTitle }}</span>
        </template>
      </el-table-column>
      <el-table-column label="收款账户名称" align="center" prop="recvAccountTitle">
        <template #default="scope">
          <el-button v-if="scope.row.expType == '00' || scope.row.expType == '01' || scope.row.expType == '03'" link
            type="primary" @click="showUserInfo(scope.row)">{{ scope.row.recvAccountTitle }}</el-button>
          <span v-else>>{{ scope.row.recvAccountTitle }}</span>
        </template>
      </el-table-column>
      <el-table-column label="支出类型" align="center" prop="expType">
        <template #default="scope">
          <dict-tag :options="sys_exp_type" :value="scope.row.expType" />
        </template>
      </el-table-column>
      <el-table-column label="支出金额" align="center" prop="expAmount" />
      <el-table-column label="支出金额" align="center" prop="expAmount" />
      <el-table-column label="支出时间" align="center" prop="createTime">
        <template #default="scope">
          <span>{{ parseTime(scope.row.createTime, '{y}-{m}-{d} {hh}:{mm}:{ss}') }}</span>
        </template>
      </el-table-column>
      <el-table-column label="备注信息" align="center" prop="remark" />
      <el-table-column label="操作" align="center" class-name="small-padding fixed-width">
        <template #default="scope">
          <el-button link type="primary" icon="Edit" @click="handleUpdate(scope.row)"
            v-hasPermi="['system:expenditure:edit']">修改</el-button>
          <el-button link type="primary" icon="Delete" @click="handleDelete(scope.row)"
            v-hasPermi="['system:expenditure:remove']">删除</el-button>
        </template>
      </el-table-column>
    </el-table>

    <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
      v-model:limit="queryParams.pageSize" @pagination="getList" />

    <!-- 添加或修改支出对话框 -->
    <el-dialog :show-close="false" v-model="open" width="500px" append-to-body>
      <el-form ref="expenditureRef" :model="form" :rules="rules" label-width="80px">
        <el-form-item label="支出账目" prop="expTitle">
          <el-input v-model="form.expTitle" placeholder="请输入支出账目" />
        </el-form-item>
        <el-form-item label="对方账户" prop="recvAccountTitle">
          <el-select v-model="form.recvAccount" filterable :clearable="true" remote reserve-keyword
            placeholder="请选择对方账户" allow-create @blur="handleBlur" remote-show-suffix :remote-method="searchUserByTel"
            value-key="recvAccount" style="width: 240px">
            <el-option v-for="item in userListRes" :key="item.userId" :label="item.nickName + '\t' + item.phonenumber"
              :value="item.userId" />
          </el-select>
        </el-form-item>
        <el-row>
          <el-col :span="12">

            <el-form-item label="支出类型" prop="expType">
              <el-select v-model="form.expType" placeholder="请选择支出类型" clearable>
                <el-option v-for="dict in sys_exp_type" :key="dict.value" :label="dict.label"
                  :value="dict.value"></el-option>
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="支出金额" prop="expAmount">
              <el-input-number :min="0" v-model="form.expAmount" controls-position="right" placeholder="请输入支出金额" />
            </el-form-item>
          </el-col>
        </el-row>
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

    <!-- 展示会员详细信息 -->
    <el-dialog title="会员详细信息" v-model="showUserInfoDialog" width="400px" append-to-body>
      <UserInfo :user="userInfo" />
    </el-dialog>
    <!-- 展示支出详细信息 -->
    <el-dialog title="支出详细信息" v-model="showDetailDialog" width="400px" append-to-body>
      <el-form ref="expenditureRef" :model="detail" label-width="80px">
        <el-form-item label="支出账目" prop="expTitle">
          {{ detail.expTitle }}
        </el-form-item>
        <el-form-item label="对方账户" prop="recvAccountTitle">
          {{ detail.recvAccountTitle }}
        </el-form-item>
        <el-form-item label="支出类型" prop="expType">
          <dict-tag :options="sys_exp_type" :value="detail.expType" />
        </el-form-item>
        <el-form-item label="支出金额" prop="expAmount">
          {{ detail.expAmount }}
        </el-form-item>
        <el-form-item label="订单编码" prop="expAmount">
          {{ detail.order.orderNumber }}
        </el-form-item>
        <el-form-item label="备注信息" prop="remark">
          {{ detail.remark }}
        </el-form-item>
      </el-form>
    </el-dialog>
  </div>
</template>

<script setup name="Expenditure">
import { listExpenditure, getExpenditure, delExpenditure, addExpenditure, updateExpenditure } from "@/api/system/expenditure";
import { getUser, listUser } from "@/api/system/user";
import { getOrders } from "@/api/system/orders";
import UserInfo from '@/views/system/user/info';

const { proxy } = getCurrentInstance();
const { sys_exp_type } = proxy.useDict("sys_exp_type");

const expenditureList = ref([]);
const userList = ref([]);
const userListRes = ref([]);
const notACount = ref(false);
const open = ref(false);
const loading = ref(true);
const showSearch = ref(true);
const showUserInfoDialog = ref(false);
const showDetailDialog = ref(false);
const ids = ref([]);
const single = ref(true);
const multiple = ref(true);
const total = ref(0);
const title = ref("");
const userInfo = ref(null);
const detail = ref(null);
const dateRange = ref([]);

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

function showOrderInfo(row) {
  if (row.orderId) {
    detail.value = row;
    getOrders(row.orderId).then(res => {
      detail.value.order = res.data;
      showDetailDialog.value = true;
    })
  }
}

function showUserInfo(row) {
  if (row.recvAccount) {
    getUser(row.recvAccount).then(res => {
      userInfo.value = res.data;
      showUserInfoDialog.value = true;
    })
  }
}

/** 查询支出列表 */
function getList() {
  loading.value = true;
  listExpenditure(proxy.addDateRange(queryParams.value, dateRange.value)).then(response => {
    expenditureList.value = response.rows;
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

/** 搜索按钮操作 */
function handleQuery() {
  queryParams.value.pageNum = 1;
  getList();
}

/** 重置按钮操作 */
function resetQuery() {
  proxy.resetForm("queryRef");
  queryParams.value.expType = null;
  dateRange.value = [];
  handleQuery();
}

// 多选框选中数据
function handleSelectionChange(selection) {
  ids.value = selection.map(item => item.expId);
  single.value = selection.length != 1;
  multiple.value = !selection.length;
}

/** 新增按钮操作 */
function handleAdd() {
  reset();
  listUser().then(res => {
    userList.value = res.rows;
    open.value = true;
  })
  // title.value = "添加支出";
}

/** 修改按钮操作 */
async function handleUpdate(row) {
  reset();
  const _expId = row.expId || ids.value
  await listUser().then(res => {
    userList.value = res.rows;
  })
  console.log(userList.value)
  getExpenditure(_expId).then(response => {
    form.value = response.data;
    if(!form.value.recvAccount){
      form.value.recvAccount = form.value.recvAccountTitle;
    }
    open.value = true;
    // title.value = "修改支出";
  });
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
          getList();
        });
      } else {
        addExpenditure(form.value).then(response => {
          proxy.$modal.msgSuccess("新增成功");
          open.value = false;
          getList();
        });
      }
    }
  });
}

/** 删除按钮操作 */
function handleDelete(row) {
  const _expIds = row.expId || ids.value;
  proxy.$modal.confirm('是否确认删除支出编号为"' + _expIds + '"的数据项？').then(function () {
    return delExpenditure(_expIds);
  }).then(() => {
    getList();
    proxy.$modal.msgSuccess("删除成功");
  }).catch(() => { });
}

/** 导出按钮操作 */
function handleExport() {
  proxy.download('system/expenditure/export', {
    ...queryParams.value
  }, `expenditure_${new Date().getTime()}.xlsx`)
}

getList();
</script>
