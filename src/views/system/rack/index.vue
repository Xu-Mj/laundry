<template>
  <div class="app-container">

    <el-card class="table-card">
      <el-row :gutter="10" class="mb8">
        <el-col :span="1.5">
          <el-button type="primary" plain icon="Plus" @click="handleAdd">新增</el-button>
        </el-col>
        <el-col :span="1.5">
          <el-button type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete">删除</el-button>
        </el-col>
        <right-toolbar @queryTable="getList"></right-toolbar>
      </el-row>

      <el-table v-loading="loading" :data="rackList" @selection-change="handleSelectionChange" class="modern-table"
        border stripe>
        <template #empty>
          <el-empty description="暂无数据" />
        </template>
        <el-table-column type="selection" width="55" align="center" />
        <el-table-column label="架子名称" align="center" prop="name" />
        <el-table-column label="架子类型" align="center" prop="rackType">
          <template #default="scope">
            <el-tag v-if="scope.row.rackType == 1">输送线</el-tag>
            <el-tag v-if="scope.row.rackType == 2">其他</el-tag>
          </template>
        </el-table-column>
        <el-table-column label="容量" align="center" prop="capacity" />
        <el-table-column label="剩余容量" align="center" prop="remainingCapacity" />
        <el-table-column label="当前挂钩位置" align="center" prop="position" />
        <el-table-column label="操作" align="center" class-name="small-padding fixed-width">
          <template #default="scope">
            <el-button link type="primary" icon="Edit" @click="handleUpdate(scope.row)">修改</el-button>
            <el-button link type="primary" icon="Delete" @click="handleDelete(scope.row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>
    <!-- 添加或修改晾衣架对话框 -->
    <el-dialog v-model="open" width="500px" :align-center="true" :show-close="false">
      <template #header>
        <div class="dialog-header hover-flow">
          <span class="dialog-title">{{ title }}</span>
          <el-button icon="Close" circle @click="cancel" />
        </div>
      </template>
      <el-form ref="rackRef" :model="form" :rules="rules" label-width="80px">
        <div class="form-card hover-flow">
          <div class="card-header">
            <el-icon>
              <Money />
            </el-icon>
            <span>基本信息</span>
          </div>
          <div class="card-body">
            <el-form-item label="架子名称" prop="name">
              <el-input v-model="form.name" placeholder="请输入架子名称" />
            </el-form-item>
            <el-form-item label="类型" prop="rackType">
              <el-radio-group v-model="form.rackType">
                <el-radio :value="'1'">输送线</el-radio>
                <el-radio :value="'2'">其他</el-radio>
              </el-radio-group>
            </el-form-item>
            <el-form-item label="容量" prop="capacity">
              <el-input-number controls-position="right" v-model="form.capacity" placeholder="请输入容量" />
            </el-form-item>
          </div>
        </div>
      </el-form>
      <template #footer>
        <div class="dialog-footer">
          <el-button class="hover-flow" type="primary" @click="submitForm" icon="Check">确 定</el-button>
          <el-button class="hover-flow" type="danger" @click="cancel" icon="Close">取 消</el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup name="Rack">
import { listRack, getRack, delRack, addRack, updateRack } from "@/api/system/rack";

const { proxy } = getCurrentInstance();

const rackList = ref([]);
const open = ref(false);
const loading = ref(true);
const ids = ref([]);
const single = ref(true);
const multiple = ref(true);
const title = ref("");

const data = reactive({
  form: {},
  rules: {
    capacity: [
      { required: true, message: "容量不能为空", trigger: "blur" }
    ],
    remainingCapacity: [
      { required: true, message: "剩余容量不能为空", trigger: "blur" }
    ],
    rackType: [
      { required: true, message: "上挂类型", trigger: "blur" }
    ]
  }
});

const { form, rules } = toRefs(data);

/** 查询晾衣架列表 */
function getList() {
  loading.value = true;
  listRack({ pageSize: 100 }).then(response => {
    rackList.value = response;
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
    name: null,
    rackType: '1',
    capacity: 100,
    remainingCapacity: null,
    position: null
  };
  proxy.resetForm("rackRef");
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
  title.value = "添加晾衣架";
}

/** 修改按钮操作 */
function handleUpdate(row) {
  reset();
  const _id = row.id || ids.value
  getRack(_id).then(response => {
    form.value = response;
    open.value = true;
    title.value = "修改晾衣架";
  });
}

/** 提交按钮 */
function submitForm() {
  proxy.$refs["rackRef"].validate(valid => {
    if (valid) {
      if (form.value.id != null) {
        updateRack(form.value).then(response => {
          proxy.notify.success("修改成功");
          open.value = false;
          getList();
        });
      } else {
        addRack(form.value).then(response => {
          proxy.notify.success("新增衣架成功");
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
  proxy.$modal.confirm('是否确认删除晾衣架编号为"' + _ids + '"的数据项？').then(function () {
    return delRack(_ids);
  }).then(() => {
    getList();
    proxy.notify.success("删除成功");
  }).catch(() => { });
}
getList();
</script>
