<template>
  <el-form ref="profileForm" :model="profileData" :rules="rules" label-width="120px" class="profile-form">
    <el-row :gutter="20">
      <el-col :span="12">
        <el-form-item label="昵称" prop="nickname">
          <el-input v-model="profileData.nickname" placeholder="请输入昵称" prefix-icon="User" />
        </el-form-item>
      </el-col>
      <el-col :span="12">
        <el-form-item label="性别">
          <el-radio-group v-model="profileData.ownerGender" class="gender-radio">
            <el-radio label="male">男</el-radio>
            <el-radio label="female">女</el-radio>
          </el-radio-group>
        </el-form-item>
      </el-col>
    </el-row>

    <el-row :gutter="20">
      <el-col :span="12">
        <el-form-item label="店铺名称" prop="storeName">
          <el-input v-model="profileData.storeName" placeholder="请输入店铺名称" prefix-icon="Shop" />
        </el-form-item>
      </el-col>
      <el-col :span="12">
        <el-form-item label="店主姓名" prop="ownerName">
          <el-input v-model="profileData.ownerName" placeholder="请输入店主姓名" prefix-icon="User" />
        </el-form-item>
      </el-col>
    </el-row>

    <el-row :gutter="20">
      <el-col :span="12">
        <el-form-item label="联系电话" prop="ownerPhone">
          <el-input v-model="profileData.ownerPhone" disabled placeholder="请输入联系电话" prefix-icon="Phone" />
        </el-form-item>
      </el-col>
      <el-col :span="12">
        <el-form-item label="电子邮箱" prop="email">
          <el-input v-model="profileData.email" placeholder="请输入电子邮箱" prefix-icon="Message" />
        </el-form-item>
      </el-col>
    </el-row>

    <el-form-item label="店铺地址" prop="storeLocation">
      <el-input v-model="profileData.storeLocation" placeholder="请输入店铺地址" prefix-icon="Location" />
    </el-form-item>

    <el-form-item>
      <el-button :disabled="props.isGuest" type="primary" @click="handleUpdateProfile" :loading="updating">
        <el-icon>
          <Check />
        </el-icon> 保存修改
      </el-button>
      <el-button :disabled="props.isGuest" @click="resetForm">
        <el-icon>
          <RefreshRight />
        </el-icon> 重置
      </el-button>
    </el-form-item>
  </el-form>
</template>

<script setup>
import { ElMessageBox } from 'element-plus';
import { Check, RefreshRight } from '@element-plus/icons-vue';
import { getProfile, updateProfile } from '@/api/system/profile';

const props = defineProps({
  profileData: {
    type: Object,
    required: true
  },
  userStore: {
    type: Object,
    required: true
  },
  isGuest: {
    type: Boolean,
    required: true
  }
});

const emit = defineEmits(['update:profileData', 'profile-updated']);

const { proxy } = getCurrentInstance();
const updating = ref(false);

// 表单校验规则
const rules = ref({
  nickname: [
    { required: false, message: '请输入昵称', trigger: 'blur' },
    { min: 2, max: 20, message: '昵称长度必须介于 2 和 20 之间', trigger: 'blur' }
  ],
  storeName: [
    { required: false, message: '请输入店铺名称', trigger: 'blur' },
    { min: 2, max: 30, message: '店铺名称长度必须介于 2 和 30 之间', trigger: 'blur' }
  ],
  ownerName: [
    { required: false, message: '请输入店主姓名', trigger: 'blur' },
    { min: 2, max: 20, message: '店主姓名长度必须介于 2 和 20 之间', trigger: 'blur' }
  ],
  ownerPhone: [
    { required: false, message: '请输入联系电话', trigger: 'blur' },
    { pattern: /^1[3|4|5|6|7|8|9][0-9]\d{8}$/, message: '请输入正确的手机号码', trigger: 'blur' }
  ],
  email: [
    { required: false, message: '请输入电子邮箱', trigger: 'blur' },
    { type: 'email', message: '请输入正确的邮箱地址', trigger: ['blur', 'change'] }
  ],
  storeLocation: [
    { required: false, message: '请输入店铺地址', trigger: 'blur' },
    { min: 5, max: 100, message: '店铺地址长度必须介于 5 和 100 之间', trigger: 'blur' }
  ]
});

// 更新个人信息
const handleUpdateProfile = async () => {
  // 获取表单引用
  const profileFormRef = proxy.$refs.profileForm;
  if (!profileFormRef) return;

  // 表单验证
  profileFormRef.validate(async valid => {
    if (!valid) {
      proxy.notify.warning('请正确填写表单信息');
      return;
    }

    try {
      updating.value = true;
      const user = await updateProfile(props.profileData);
      props.userStore.setUser(user);
      emit('profile-updated', user);
    } catch (error) {
      proxy.notify.error('更新失败：' + (error.message || '未知错误'));
    } finally {
      updating.value = false;
    }
  });
};

// 重置表单
const resetForm = () => {
  ElMessageBox.confirm('确定要重置表单吗？所有未保存的修改将丢失。', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(async () => {
    try {
      const res = await getProfile();
      emit('update:profileData', res.data);
      proxy.notify.success('表单已重置');
    } catch (error) {
      console.error(error);
    }
  }).catch(() => { });
};
</script>

<style scoped>
.profile-form {
  padding: 20px 0;
}

.gender-radio {
  display: flex;
  align-items: center;
  height: 40px;
}
</style>