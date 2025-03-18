<template>
  <el-tour v-model="tourVisible" :scroll-into-view-options="true" placement="right">
    <el-tour-step :show-arrow="true" :target="menuRefs['首页']" placement="right" title="首页" description="返回系统首页">
      <div>点击此按钮返回系统首页，查看系统概览和统计数据</div>
    </el-tour-step>
    <el-tour-step :target="menuRefs['收衣收鞋']" title="收衣收鞋" placement="right" description="创建新的收衣收鞋订单">
      <div>点击此按钮创建新的收衣收鞋订单，记录客户送来的衣物信息</div>
    </el-tour-step>
    <el-tour-step :target="menuRefs['取衣取鞋']" title="取衣取鞋" placement="right" description="处理客户取衣取鞋业务">
      <div>点击此按钮处理客户取衣取鞋业务，查找和完成已有订单</div>
    </el-tour-step>
    <el-tour-step :target="menuRefs['订单管理']" title="订单管理" placement="right" description="查看和管理所有订单">
      <div>点击此按钮查看和管理所有订单，包括订单状态、详情和历史记录</div>
    </el-tour-step>
    <el-tour-step :target="menuRefs['衣物上挂']" title="衣物上挂" placement="right" description="管理衣物上挂流程">
      <div>点击此按钮管理衣物上挂流程，记录衣物位置和状态</div>
    </el-tour-step>
    <el-tour-step :target="menuRefs['卡券管理']" title="卡券管理" placement="right" description="管理店铺的各类卡券">
      <div>点击此按钮管理店铺的各类卡券，包括创建、查看和修改卡券</div>
    </el-tour-step>
    <el-tour-step :target="menuRefs['新增会员']" title="新增会员" placement="right" description="添加新的会员信息">
      <div>点击此按钮添加新的会员信息，记录会员基本资料和联系方式</div>
    </el-tour-step>
    <el-tour-step :target="menuRefs['会员管理']" title="会员管理" placement="right" description="查看和管理所有会员信息">
      <div>点击此按钮查看和管理所有会员信息，包括会员详情和消费记录</div>
    </el-tour-step>
    <el-tour-step :target="menuRefs['支出录入']" title="支出录入" placement="right" description="记录店铺的各项支出">
      <div>点击此按钮记录店铺的各项支出，包括水电费、租金等日常开销</div>
    </el-tour-step>
    <el-tour-step :target="menuRefs['收支报表']" title="收支报表" placement="right" description="查看店铺的收支统计报表">
      <div>点击此按钮查看店铺的收支统计报表，了解经营状况和财务分析</div>
    </el-tour-step>
    <el-tour-step :target="menuRefs['知识天地']" title="知识天地" placement="right" description="查看洗衣相关的知识和技巧">
      <div>点击此按钮查看洗衣相关的知识和技巧，提升服务质量和专业水平</div>
    </el-tour-step>
  </el-tour>
</template>

<script setup>
import { ref, watch, onMounted, defineProps, defineEmits } from 'vue';
import { checkTourCompleted, updateTourGuide } from '@/api/system/tour_guide';

const props = defineProps({
  // 菜单引用对象
  menuRefs: {
    type: Object,
    required: true
  },
  // 页面标识，用于记录用户已完成的引导
  pageKey: {
    type: String,
    default: 'sidebar_menu'
  }
});

const emit = defineEmits(['tour-finished']);

const tourVisible = ref(false);

// 检查并自动开启引导
const checkAndStartTour = async () => {
  try {
    // 检查用户是否已完成菜单引导
    const completed = await checkTourCompleted(props.pageKey);
    if (!completed) {
      // 如果未完成，自动开启引导
      tourVisible.value = true;
    }
  } catch (error) {
    console.error('检查引导状态失败:', error);
  }
};

// 引导完成后更新用户引导记录
const handleTourFinish = async () => {
  try {
    await updateTourGuide(props.pageKey);
    emit('tour-finished');
  } catch (error) {
    console.error('更新引导记录失败:', error);
  }
};

// 监听引导状态变化
watch(tourVisible, (newValue, oldValue) => {
  // 当引导关闭时，更新引导记录
  if (oldValue === true && newValue === false) {
    handleTourFinish();
  }
});

onMounted(() => {
  // 检查并自动开启引导
  checkAndStartTour();
});
</script>