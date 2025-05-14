<template>
  <el-form ref="profileForm" :model="profileData" :rules="rules" label-width="120px" class="profile-form">
    <el-row :gutter="20">
      <el-col :span="12">
        <el-form-item label="昵称" prop="nickname">
          <el-input v-model="profileData.nickname" placeholder="请输入昵称" prefix-icon="User" />
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
        <el-form-item label="店铺名称" prop="storeName">
          <el-input v-model="profileData.storeName" disabled placeholder="请输入店铺名称" prefix-icon="Shop" />
        </el-form-item>
      </el-col>
      <el-col :span="12">
        <el-form-item label="性别">
          <el-radio-group v-model="profileData.ownerGender" class="gender-radio">
            <el-radio value="male">男</el-radio>
            <el-radio value="female">女</el-radio>
          </el-radio-group>
        </el-form-item>
      </el-col>
    </el-row>

    <el-row :gutter="20">
      <el-col :span="12">
        <!-- 店铺地址 - 省市区选择 -->
        <el-form-item label="店铺地址" prop="addressRegion">
          <el-cascader v-model="profileData.addressRegion" :options="areaData" placeholder="请选择省/市/区"
            class="custom-cascader" :props="{
              checkStrictly: false,
              value: 'value',
              label: 'label',
              children: 'children',
              expandTrigger: 'hover'
            }" clearable />
        </el-form-item>
      </el-col>
      <el-col :span="12">
        <el-form-item label="联系电话" prop="ownerPhone">
          <el-input v-model="profileData.ownerPhone" disabled placeholder="请输入联系电话" prefix-icon="Phone" />
        </el-form-item>

      </el-col>
    </el-row>
    <el-row :gutter="20">
      <el-col :span="12">
        <!-- 详细地址 -->
        <el-form-item label="详细地址" prop="addressDetail">
          <el-input v-model="profileData.addressDetail" placeholder="请输入详细地址（街道、门牌号等）" prefix-icon="Location" />
        </el-form-item>
      </el-col>
      <el-col :span="12">
        <el-form-item label="电子邮箱" prop="email">
          <el-input v-model="profileData.email" placeholder="请输入电子邮箱" prefix-icon="Message" />
        </el-form-item>
      </el-col>
    </el-row>

    <el-form-item>
      <el-button :disabled="props.isGuest" type="primary" @click="handleUpdateProfile" :loading="updating">
        <el-icon>
          <Check />
        </el-icon> 保存修改
      </el-button>
      <!-- <el-button :disabled="props.isGuest" @click="resetForm">
        <el-icon>
          <RefreshRight />
        </el-icon> 重置
      </el-button> -->
    </el-form-item>
  </el-form>
</template>

<script setup>
import { updateProfile } from '@/api/system/profile';
import { areaData, parseAddress, stringifyAddress } from '@/utils/area-data';
import useUserStore from '@/store/modules/user';

const props = defineProps({
  profileData: {
    type: Object,
    required: true
  },
  isGuest: {
    type: Boolean,
    required: true
  }
});

const emit = defineEmits(['update:profileData', 'profile-updated']);
const userStore = useUserStore();
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
  addressRegion: [
    { required: false, message: '请选择省/市/区', trigger: 'change' },
    { type: 'array', min: 3, message: '请完整选择省/市/区', trigger: 'change' }
  ],
  addressDetail: [
    { required: false, message: '请输入详细地址', trigger: 'blur' },
    { min: 3, max: 100, message: '详细地址长度必须介于 3 和 100 之间', trigger: 'blur' }
  ]
});

// 根据地区名称查找对应的地区代码
const findAreaCodesByNames = (names) => {
  if (!names || !Array.isArray(names) || names.length === 0) {
    return [];
  }

  const codes = [];
  let currentLevel = areaData;

  // 省级查找
  if (names.length >= 1) {
    const province = currentLevel.find(item => item.label === names[0]);
    if (province) {
      codes.push(province.value);
      currentLevel = province.children || [];

      // 市级查找
      if (names.length >= 2 && currentLevel.length > 0) {
        const city = currentLevel.find(item => item.label === names[1]);
        if (city) {
          codes.push(city.value);
          currentLevel = city.children || [];

          // 区县级查找
          if (names.length >= 3 && currentLevel.length > 0) {
            const district = currentLevel.find(item => item.label === names[2]);
            if (district) {
              codes.push(district.value);
            }
          }
        }
      }
    }
  }

  console.log(`Converting names ${JSON.stringify(names)} to codes ${JSON.stringify(codes)}`);
  return codes;
};

// 初始化地址数据
const initAddressData = () => {
  console.log("Initial addressRegion:", props.profileData.addressRegion);
  console.log("Initial province/city/district:", {
    province: props.profileData.province,
    city: props.profileData.city,
    district: props.profileData.district
  });

  // 情况1: 如果addressRegion已经是数组，检查是名称数组还是代码数组
  if (props.profileData.addressRegion && Array.isArray(props.profileData.addressRegion)) {
    // 检查第一个元素是字符串且不是数字
    if (props.profileData.addressRegion.length > 0 &&
      typeof props.profileData.addressRegion[0] === 'string' &&
      isNaN(Number(props.profileData.addressRegion[0]))) {
      console.log("Case 1: Converting address names to codes:", props.profileData.addressRegion);
      props.profileData.addressRegion = findAreaCodesByNames(props.profileData.addressRegion);
      console.log("Converted to codes:", props.profileData.addressRegion);
    } else {
      console.log("Case 1: addressRegion already contains codes:", props.profileData.addressRegion);
    }
  }
  // 情况2: 如果有province/city/district字段，使用它们来设置addressRegion
  else if (props.profileData.province || props.profileData.city || props.profileData.district) {
    console.log("Case 2: Using province/city/district fields");
    const regionNames = [];

    if (props.profileData.province) regionNames.push(props.profileData.province);
    if (props.profileData.city) regionNames.push(props.profileData.city);
    if (props.profileData.district) regionNames.push(props.profileData.district);

    props.profileData.addressRegion = findAreaCodesByNames(regionNames);
    console.log("Set addressRegion from names:", regionNames, "to codes:", props.profileData.addressRegion);
  }
  // 情况3: 如果有storeLocation字段，解析它
  else if (props.profileData.storeLocation) {
    console.log("Case 3: Parsing from storeLocation:", props.profileData.storeLocation);
    const addressData = parseAddress(props.profileData.storeLocation);
    props.profileData.addressRegion = addressData.selectedOptions;
    props.profileData.addressDetail = addressData.detailAddress;
    console.log("Parsed addressRegion:", props.profileData.addressRegion);
  }
  // 情况4: 没有地址数据
  else {
    console.log("Case 4: No address data available");
    props.profileData.addressRegion = [];
    props.profileData.addressDetail = '';
  }

  // 确保addressDetail字段存在
  props.profileData.addressDetail = props.profileData.addressDetail || '';

  console.log("Final addressRegion:", props.profileData.addressRegion);
};

// 监听地址变化，更新storeLocation字段
watch(
  () => [props.profileData.addressRegion, props.profileData.addressDetail],
  ([newRegion, newDetail]) => {
    if (newRegion && newRegion.length > 0) {
      props.profileData.storeLocation = stringifyAddress(newRegion, newDetail);
    } else if (newDetail) {
      props.profileData.storeLocation = newDetail;
    } else {
      props.profileData.storeLocation = '';
    }
  },
  { deep: true }
);

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

      // 处理地址数据，确保storeLocation字段和新的地址字段都已更新
      if (props.profileData.addressRegion && props.profileData.addressRegion.length > 0) {
        // 更新storeLocation字段（保持向后兼容）
        props.profileData.storeLocation = stringifyAddress(
          props.profileData.addressRegion,
          props.profileData.addressDetail
        );

        // 获取区域名称而不是代码，用于后端存储
        let areaNames = [];
        let currentLevel = areaData;

        // 遍历addressRegion数组（区域代码）并获取对应的名称
        for (const code of props.profileData.addressRegion) {
          const found = currentLevel.find(item => item.value === code);
          if (found) {
            areaNames.push(found.label);
            if (found.children) {
              currentLevel = found.children;
            } else {
              break;
            }
          } else {
            break;
          }
        }

        // 更新新的地址字段
        if (areaNames.length >= 1) {
          props.profileData.province = areaNames[0];
        }
        if (areaNames.length >= 2) {
          props.profileData.city = areaNames[1];
        }
        if (areaNames.length >= 3) {
          props.profileData.district = areaNames[2];
        }
        props.profileData.addressDetail = props.profileData.addressDetail || '';
      } else {
        // 如果没有选择地址，清空地址字段
        props.profileData.province = null;
        props.profileData.city = null;
        props.profileData.district = null;
        props.profileData.addressDetail = props.profileData.addressDetail || '';
        props.profileData.storeLocation = props.profileData.addressDetail || '';
      }

      // 创建一个不包含中间字段的数据对象
      const updateData = { ...props.profileData };
      delete updateData.addressRegion;
      updateData.avatar = getOriginalPath(updateData.avatar);
      console.log("Update Data:", updateData);

      const user = await updateProfile(updateData);
      userStore.getInfo();
      emit('profile-updated', user);
    } catch (error) {
      proxy.notify.error('更新失败：' + (error.message || '未知错误'));
    } finally {
      updating.value = false;
    }
  });
};

function getOriginalPath(convertedUrl) {
  if (!convertedUrl) return '';

  // 如果是普通路径（未转换的），直接返回
  if (!convertedUrl.startsWith('http://asset.localhost/')) {
    return convertedUrl;
  }

  try {
    const pathPart = convertedUrl.replace(/^https?:\/\/asset\.localhost\//, '');
    const decodedPath = decodeURIComponent(pathPart);
    return decodedPath.replace(/\//g, '\\');
  } catch (error) {
    console.error('路径转换失败:', error, convertedUrl);
    return convertedUrl;
  }
}

// 组件挂载时初始化地址数据
onMounted(() => {
  initAddressData();
});
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

.custom-cascader {
  width: 100%;
}
</style>