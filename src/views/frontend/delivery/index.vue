<template>
  <div class="app-container">
    <transition name="height-fade">
      <el-card shadow="never" class="search-card" v-show="showSearch">
        <el-form :model="queryParams" ref="queryForm" :inline="true">
          <el-form-item label="订单状态" prop="deliveryStatus">
            <el-select v-model="queryParams.deliveryStatus" placeholder="派送状态" clearable style="width: 240px">
              <el-option v-for="dict in deliveryStatusOptions" :key="dict.dictValue" :label="dict.dictLabel"
                :value="dict.dictValue" />
            </el-select>
          </el-form-item>
          <el-form-item label="时间范围" prop="dateRange">
            <el-date-picker v-model="dateRange" type="daterange" range-separator="至" start-placeholder="开始日期"
              end-placeholder="结束日期" value-format="YYYY-MM-DD" style="width: 280px"></el-date-picker>
          </el-form-item>
          <el-form-item>
            <el-button type="primary" icon="Search" @click="handleQuery">搜索</el-button>
            <el-button icon="Refresh" @click="resetQuery">重置</el-button>
          </el-form-item>
        </el-form>
      </el-card>
    </transition>

    <el-card shadow="never" class="table-card">
      <el-row :gutter="10" class="mb8">
        <el-col :span="1.5">
          <!-- <el-button type="success" plain icon="Plus" @click="handleAdd">创建派送</el-button> -->
        </el-col>
        <el-col :span="1.5">
          <el-button type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete">批量取消</el-button>
        </el-col>
        <right-toolbar v-model:showSearch="showSearch" @queryTable="getList" :columns="columns" />
      </el-row>

      <el-table v-loading="loading" :data="deliveryList" @selection-change="handleSelectionChange">
        <template #empty>
          <el-empty description="暂无数据" />
        </template>
        <el-table-column type="selection" width="55" align="center" />
        <el-table-column label="派送编号" align="center" prop="deliveryId" width="100" />
        <el-table-column label="客户信息" align="center" min-width="180">
          <template #default="scope">
            <div class="user-info">
              <el-avatar :size="32" :src="scope.row.user?.avatar" icon="UserFilled"></el-avatar>
              <div class="user-details">
                <div class="user-name">{{ scope.row.user?.nickName || '未知客户' }}</div>
                <div class="user-phone">{{ scope.row.user?.phonenumber || '-' }}</div>
              </div>
            </div>
          </template>
        </el-table-column>
        <el-table-column label="派送地址" align="center" prop="address" min-width="180" show-overflow-tooltip />
        <el-table-column label="派送时间" align="center" prop="dispatchTime" width="160">
          <template #default="scope">
            <span>{{ parseTime(scope.row.dispatchTime) }}</span>
          </template>
        </el-table-column>
        <el-table-column label="衣物数量" align="center" width="100">
          <template #default="scope">
            <el-badge :value="getClothCount(scope.row)" type="primary" />
          </template>
        </el-table-column>
        <el-table-column label="状态" align="center" prop="deliveryStatus" width="120">
          <template #default="scope">
            <el-tag :type="getStatusType(scope.row.deliveryStatus)">
              {{ getStatusLabel(scope.row.deliveryStatus) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="创建时间" align="center" prop="createTime" width="160">
          <template #default="scope">
            <span>{{ formatTime(scope.row.createTime) }}</span>
          </template>
        </el-table-column>
        <el-table-column label="操作" align="center" width="180" class-name="small-padding fixed-width">
          <template #default="scope">
            <el-button type="text" icon="View" @click="handleView(scope.row)"
              v-hasPermi="['system:delivery:query']">查看</el-button>
            <el-button v-if="scope.row.deliveryStatus === '00' || scope.row.deliveryStatus === '01'" type="text"
              icon="Check" @click="handleComplete(scope.row)" v-hasPermi="['system:delivery:edit']">完成</el-button>
            <el-button v-if="scope.row.deliveryStatus === '00' || scope.row.deliveryStatus === '01'" type="text"
              icon="Close" @click="handleCancel(scope.row)" v-hasPermi="['system:delivery:remove']">取消</el-button>
          </template>
        </el-table-column>
      </el-table>

      <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
        v-model:limit="queryParams.pageSize" @pagination="getList" />
    </el-card>

    <!-- 查看派送详情 -->
    <el-dialog :title="'派送详情 #' + deliveryDetail.deliveryId" v-model="viewVisible" width="600px" append-to-body
      align-center>
      <el-descriptions :column="1" border>
        <el-descriptions-item label="派送编号">{{ deliveryDetail.deliveryId }}</el-descriptions-item>
        <el-descriptions-item label="客户信息">
          <div class="user-info">
            <el-avatar :size="32" :src="deliveryDetail.user?.avatar" icon="UserFilled"></el-avatar>
            <div class="user-details">
              <div class="user-name">{{ deliveryDetail.user?.nickName || '未知客户' }}</div>
              <div class="user-phone">{{ deliveryDetail.user?.phonenumber || '-' }}</div>
            </div>
          </div>
        </el-descriptions-item>
        <el-descriptions-item label="派送地址">{{ deliveryDetail.address }}</el-descriptions-item>
        <el-descriptions-item label="派送时间">{{ parseTime(deliveryDetail.dispatchTime) }}</el-descriptions-item>
        <el-descriptions-item label="完成时间" v-if="deliveryDetail.completeTime">
          {{ parseTime(deliveryDetail.completeTime) }}
        </el-descriptions-item>
        <el-descriptions-item label="派送状态">
          <el-tag :type="getStatusType(deliveryDetail.deliveryStatus)">
            {{ getStatusLabel(deliveryDetail.deliveryStatus) }}
          </el-tag>
        </el-descriptions-item>
        <el-descriptions-item label="备注信息">{{ deliveryDetail.remark || '无' }}</el-descriptions-item>
      </el-descriptions>

      <div class="delivery-items">
        <div class="section-title">派送衣物</div>
        <el-table :data="clothesList" size="small">
          <el-table-column prop="clothId" label="衣物编号" width="100" />
          <el-table-column prop="clothInfo.clothingName" label="衣物名称" min-width="120" />
          <el-table-column prop="hangClothCode" label="衣物编码" width="120" />
          <el-table-column prop="priceValue" label="价格" width="80">
            <template #default="scope">
              <span class="price">￥{{ scope.row.priceValue || 0 }}</span>
            </template>
          </el-table-column>
        </el-table>
      </div>
    </el-dialog>

    <!-- 创建派送对话框 -->
    <el-dialog :title="'创建派送'" v-model="addVisible" width="600px" append-to-body>
      <el-form :model="deliveryForm" ref="deliveryFormRef" :rules="rules" label-width="100px">
        <el-form-item label="选择客户" prop="userId">
          <el-select v-model="deliveryForm.userId" filterable placeholder="请选择客户" style="width: 100%"
            @change="handleUserChange">
            <el-option v-for="user in userOptions" :key="user.userId"
              :label="user.nickName + ' (' + user.phonenumber + ')'" :value="user.userId">
              <div class="user-info">
                <el-avatar :size="24" :src="user.avatar" icon="UserFilled"></el-avatar>
                <div class="user-details">
                  <div class="user-name">{{ user.nickName }}</div>
                  <div class="user-phone">{{ user.phonenumber }}</div>
                </div>
              </div>
            </el-option>
          </el-select>
        </el-form-item>
        <el-form-item label="派送地址" prop="address">
          <el-input v-model="deliveryForm.address" placeholder="请输入派送地址" />
        </el-form-item>
        <el-form-item label="派送时间" prop="dispatchTime">
          <el-date-picker v-model="deliveryForm.dispatchTime" type="datetime" placeholder="选择派送时间"
            value-format="YYYY-MM-DD HH:mm:ss" style="width: 100%">
          </el-date-picker>
        </el-form-item>
        <el-form-item label="选择衣物" prop="clothIds">
          <el-transfer v-model="deliveryForm.clothIds" :data="availableClothes" :props="{
            key: 'clothId',
            label: item => `${item.clothInfo?.clothingName} (${item.hangClothCode})`,
          }" :titles="['可选衣物', '已选衣物']" filterable filter-placeholder="请输入衣物名称" :filter-method="filterMethod"
            class="cloth-transfer">
          </el-transfer>
        </el-form-item>
        <el-form-item label="备注" prop="remark">
          <el-input v-model="deliveryForm.remark" type="textarea" placeholder="请输入备注信息" />
        </el-form-item>
      </el-form>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="cancelAdd">取 消</el-button>
          <el-button type="primary" @click="submitDelivery">确 定</el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { listDeliveries, completeDelivery, cancelDelivery, getDeliveryById, delivery } from "@/api/frontend/delivery";
import { getUserListByIds, listUserWithNoLimit } from "@/api/system/user";
import { getOrderClothByIds, getDeliveryEligibleClothes } from "@/api/system/orderCloth";

const { proxy } = getCurrentInstance();

// 遮罩层
const loading = ref(false);
// 选中数组
const ids = ref([]);
// 非单个禁用
const single = ref(true);
// 非多个禁用
const multiple = ref(true);
// 显示搜索条件
const showSearch = ref(true);
// 总条数
const total = ref(0);
// 派送列表
const deliveryList = ref([]);
// 弹出层标题
const title = ref("");
// 是否显示弹出层
const open = ref(false);
// 查看详情弹窗
const viewVisible = ref(false);
// 创建派送弹窗
const addVisible = ref(false);
// 日期范围
const dateRange = ref([]);
// 派送详情
const deliveryDetail = ref({});
// 衣物列表
const clothesList = ref([]);
// 用户选项
const userOptions = ref([]);
// 可用衣物
const availableClothes = ref([]);
// 派送表单
const deliveryForm = ref({
  userId: undefined,
  address: '',
  dispatchTime: '',
  clothIds: [],
  remark: ''
});
// 表单校验规则
const rules = {
  userId: [{ required: true, message: '请选择客户', trigger: 'change' }],
  address: [{ required: true, message: '请输入派送地址', trigger: 'blur' }],
  dispatchTime: [{ required: true, message: '请选择派送时间', trigger: 'change' }],
  clothIds: [{ required: true, type: 'array', min: 1, message: '请至少选择一件衣物', trigger: 'change' }]
};
// 表单引用
const deliveryFormRef = ref(null);

// 派送状态选项
const deliveryStatusOptions = [
  { dictLabel: '待派送', dictValue: '00' },
  { dictLabel: '派送中', dictValue: '01' },
  { dictLabel: '已完成', dictValue: '02' },
  { dictLabel: '已取消', dictValue: '03' }
];

// 查询参数
const queryParams = ref({
  pageNum: 1,
  pageSize: 10,
  deliveryStatus: undefined,
});

/** 查询派送列表 */
function getList() {
  loading.value = true;

  // 构建查询参数
  const params = {
    ...queryParams.value
  };

  // 添加日期范围参数
  if (dateRange.value && dateRange.value.length > 0) {
    params.params = {
      beginTime: dateRange.value[0],
      endTime: dateRange.value[1]
    };
  }

  listDeliveries(params).then(response => {
    deliveryList.value = response.rows;
    total.value = response.total;

    // 加载用户信息
    loadUsersInfo();
    loading.value = false;
  }).catch(() => {
    loading.value = false;
  });
}

/** 加载用户信息 */
function loadUsersInfo() {
  // 收集所有用户ID
  const userIds = deliveryList.value
    .map(delivery => delivery.userId)
    .filter(id => id);

  if (userIds.length > 0) {
    getUserListByIds(userIds).then(res => {
      const userMap = {};
      res.forEach(user => {
        userMap[user.userId] = user;
      });

      // 将用户信息添加到派送记录中
      deliveryList.value.forEach(delivery => {
        if (delivery.userId) {
          delivery.user = userMap[delivery.userId];
        }
      });
    });
  }
}

/** 获取衣物数量 */
function getClothCount(row) {
  if (!row.clothId) return 0;
  return row.clothId.split(',').length;
}

/** 获取状态标签 */
function getStatusLabel(status) {
  const statusMap = {
    '00': '待派送',
    '01': '派送中',
    '02': '已完成',
    '03': '已取消'
  };
  return statusMap[status] || '未知';
}

/** 获取状态类型 */
function getStatusType(status) {
  const typeMap = {
    '00': 'info',
    '01': 'warning',
    '02': 'success',
    '03': 'danger'
  };
  return typeMap[status] || 'info';
}

/** 搜索按钮操作 */
function handleQuery() {
  queryParams.value.pageNum = 1;
  getList();
}

/** 重置按钮操作 */
function resetQuery() {
  dateRange.value = [];
  proxy.resetForm("queryForm");
  handleQuery();
}

/** 选择条数 */
function handleSelectionChange(selection) {
  ids.value = selection.map(item => item.deliveryId);
  single.value = selection.length !== 1;
  multiple.value = !selection.length;
}

/** 新增按钮操作 */
function handleAdd() {
  addVisible.value = true;
  resetDeliveryForm();
  // 加载用户列表
  loadUserOptions();
}

/** 加载用户选项 */
function loadUserOptions() {
  listUserWithNoLimit().then(res => {
    userOptions.value = res || [];
  });
}

/** 重置派送表单 */
function resetDeliveryForm() {
  deliveryForm.value = {
    userId: undefined,
    address: '',
    dispatchTime: '',
    clothIds: [],
    remark: ''
  };
  availableClothes.value = [];
  if (deliveryFormRef.value) {
    deliveryFormRef.value.resetFields();
  }
}

/** 用户选择变更 */
function handleUserChange(userId) {
  if (!userId) {
    deliveryForm.value.address = '';
    availableClothes.value = [];
    return;
  }

  // 加载用户地址
  const selectedUser = userOptions.value.find(user => user.userId === userId);
  if (selectedUser && selectedUser.address) {
    deliveryForm.value.address = selectedUser.address;
  }

  // 加载用户可派送衣物
  getDeliveryEligibleClothes(userId).then(res => {
    availableClothes.value = res || [];
  });
}

/** 衣物筛选方法 */
function filterMethod(query, item) {
  return item.clothInfo?.clothingName?.toLowerCase().includes(query.toLowerCase()) ||
    item.hangClothCode?.toLowerCase().includes(query.toLowerCase());
}

/** 取消新增 */
function cancelAdd() {
  addVisible.value = false;
  resetDeliveryForm();
}

/** 提交派送 */
function submitDelivery() {
  deliveryFormRef.value.validate(valid => {
    if (valid) {
      const deliveryData = {
        userId: deliveryForm.value.userId,
        address: deliveryForm.value.address,
        dispatchTime: deliveryForm.value.dispatchTime,
        clothId: deliveryForm.value.clothIds.join(','),
        remark: deliveryForm.value.remark,
        deliveryStatus: '00' // 默认为待派送状态
      };

      delivery(deliveryData).then(res => {
        proxy.$modal.msgSuccess("创建派送成功");
        addVisible.value = false;
        getList();
      }).catch(() => { });
    }
  });
}

/** 查看详情操作 */
function handleView(row) {
  deliveryDetail.value = { ...row };
  viewVisible.value = true;

  // 加载衣物详情
  if (row.clothId) {
    const clothIds = row.clothId.split(',').map(id => parseInt(id));
    getOrderClothByIds(clothIds).then(res => {
      clothesList.value = res || [];
    });
  }
}

/** 完成派送操作 */
function handleComplete(row) {
  proxy.$modal.confirm('确认已完成派送 #' + row.deliveryId + ' 吗？').then(function () {
    return completeDelivery(row.deliveryId);
  }).then(() => {
    getList();
    proxy.$modal.msgSuccess("派送完成成功");
  }).catch(() => { });
}

/** 取消派送操作 */
function handleCancel(row) {
  proxy.$modal.confirm('确认取消派送 #' + row.deliveryId + ' 吗？').then(function () {
    return cancelDelivery(row.deliveryId);
  }).then(() => {
    getList();
    proxy.$modal.msgSuccess("派送取消成功");
  }).catch(() => { });
}

/** 批量删除操作 */
function handleDelete() {
  proxy.$modal.confirm('是否确认取消选中的' + ids.value.length + '条派送记录？').then(function () {
    const promises = ids.value.map(id => cancelDelivery(id));
    return Promise.all(promises);
  }).then(() => {
    getList();
    proxy.$modal.msgSuccess("批量取消成功");
  }).catch(() => { });
}

onMounted(() => {
  getList();
});
</script>

<style scoped>
.search-card {
  margin-bottom: 20px;
}

.table-card {
  margin-bottom: 20px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.card-header-text {
  font-size: 16px;
  font-weight: 600;
}

.user-info {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.user-details {
  margin-left: 8px;
}

.user-name {
  font-weight: 500;
}

.user-phone {
  font-size: 12px;
  color: #999;
}

.delivery-items {
  margin-top: 20px;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 10px;
  padding-bottom: 10px;
  border-bottom: 1px solid #eee;
}

.price {
  color: #F56C6C;
  font-weight: 500;
}

.cloth-transfer {
  text-align: left;
  display: block;
}

.cloth-transfer :deep(.el-transfer-panel) {
  width: 100%;
  margin-bottom: 10px;
}

.cloth-transfer :deep(.el-transfer-panel__body) {
  height: 300px;
}

.cloth-transfer :deep(.el-transfer-panel__list.is-filterable) {
  height: 250px;
}
</style>