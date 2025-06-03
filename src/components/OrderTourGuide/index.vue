<template>
  <el-tour v-model="tourVisible" :close-on-click-mask="false">
    <!-- <el-tour-step v-for="(step, index) in tourSteps" :key="index" :show-arrow="true" :target="step.target" placement="right"
        :title="step.title" :description="step.description"/> -->
    <el-tour-step :target="props.memberCardRef" title="会员信息" placement="bottom" description="查看洗衣相关的知识和技巧">
      <div>在这里选择或创建会员信息，可以通过手机号搜索已有会员，或创建新会员。选择会员后可以查看会员的余额和积分信息。</div>
    </el-tour-step>
        <el-tour-step :target="props.orderSourceRef" title="订单来源" placement="bottom" description="查看洗衣相关的知识和技巧">
      <div>选择订单的来源渠道，如到店、美团、抖音等。不同来源可能有不同的价格方案。</div>
    </el-tour-step>
        <el-tour-step :target="props.addClothRef" title="添加衣物" placement="left" description="查看洗衣相关的知识和技巧">
      <div>在右侧区域添加衣物，按照步骤选择品类、衣物、颜色等信息。</div>
    </el-tour-step>
        <el-tour-step :target="props.clothListRef" title="衣物列表" placement="left" description="查看洗衣相关的知识和技巧">
      <div>这里显示已添加的衣物列表，可以查看和删除衣物。</div>
    </el-tour-step>
        <el-tour-step :target="props.adjustPriceRef" title="店主调价" placement="left" description="查看洗衣相关的知识和技巧">
      <div>可以对订单进行调价，包括调增、调减金额或直接设置总金额。使用价格方案后不能调价。</div>
    </el-tour-step>
        <el-tour-step :target="props.orderSummaryRef" title="订单摘要" placement="left" description="查看洗衣相关的知识和技巧">
      <div>显示订单的总件数、预计取衣时间，以及设置打印单据的份数。</div>
    </el-tour-step>
        <el-tour-step :target="props.submitButtonRef" title="取衣收款" placement="top" description="查看洗衣相关的知识和技巧">
      <div>完成订单创建，但并进入收款页面，等待用户取衣时才进行收款</div>
    </el-tour-step>
        <el-tour-step :target="props.payButtonRef" title="收衣收款" placement="top" description="查看洗衣相关的知识和技巧">
      <div>完成订单创建并直接进入收款页面，适用于客户送衣时的场景。</div>
    </el-tour-step>
  </el-tour>
</template>

<script setup>
import { checkTourCompleted, updateTourGuide } from '@/api/system/tour_guide';

const props = defineProps({
  // 引导步骤的目标元素引用
  memberCardRef: {
    type: Object,
    required: true
  },
  orderSourceRef: {
    type: Object,
    required: true
  },
  clothListRef: {
    type: Object,
    required: true
  },
  adjustPriceRef: {
    type: Object,
    required: true
  },
  orderSummaryRef: {
    type: Object,
    required: true
  },
  addClothRef: {
    type: Object,
    required: true
  },
  submitButtonRef: {
    type: Object,
    required: true
  },
  payButtonRef: {
    type: Object,
    required: true
  },
  // 页面标识，用于记录用户已完成的引导
  pageKey: {
    type: String,
    default: 'create_order'
  }
});

const emit = defineEmits(['tour-finished']);

const tourVisible = ref(false);
const currentStep = ref(0);

// 定义引导步骤
const tourSteps = ref([
  {
    target: () => props.memberCardRef,
    title: '会员信息',
    description: '在这里选择或创建会员信息，可以通过手机号搜索已有会员，或创建新会员。选择会员后可以查看会员的余额和积分信息。',
    position: 'bottom'
  },
  {
    target: () => props.orderSourceRef,
    title: '订单来源',
    description: '选择订单的来源渠道，如到店、美团、抖音等。不同来源可能有不同的价格方案。',
    position: 'bottom'
  },
  {
    target: () => props.addClothRef,
    title: '添加衣物',
    description: '在右侧区域添加衣物，按照步骤选择品类、衣物、颜色等信息。',
    position: 'left'
  },
  {
    target: () => props.clothListRef,
    title: '衣物列表',
    description: '这里显示已添加的衣物列表，可以查看和删除衣物。',
    position: 'top'
  },
  {
    target: () => props.adjustPriceRef,
    title: '店主调价',
    description: '可以对订单进行调价，包括调增、调减金额或直接设置总金额。使用价格方案后不能调价。',
    position: 'top'
  },
  {
    target: () => props.orderSummaryRef,
    title: '订单摘要',
    description: '显示订单的总件数、预计取衣时间，以及设置打印单据的份数。',
    position: 'top'
  },
  {
    target: () => props.submitButtonRef,
    title: '取衣收款',
    description: '完成订单创建并进入收款页面，适用于客户取衣时的场景。',
    position: 'top'
  },
  {
    target: () => props.payButtonRef,
    title: '收衣收款',
    description: '完成订单创建并直接进入收款页面，适用于客户送衣时的场景。',
    position: 'top'
  }
]);

// 完成引导
const finishTour = async () => {
  try {
    await updateTourGuide(props.pageKey);
    tourVisible.value = false;
    emit('tour-finished');
  } catch (error) {
    console.error('更新引导记录失败:', error);
  }
};

// 检查用户是否已完成引导
const checkAndStartTour = async () => {
  try {
    const completed = await checkTourCompleted(props.pageKey);
    if (!completed) {
      // 如果未完成，自动开启引导
      tourVisible.value = true;
      currentStep.value = 0;
    }
  } catch (error) {
    console.error('检查引导状态失败:', error);
  }
};

// 监听引导状态变化
watch(tourVisible, (newValue, oldValue) => {
  // 当引导关闭时，更新引导记录
  if (oldValue === true && newValue === false) {
    finishTour();
  }
});

onMounted(() => {
  // 检查并自动开启引导
  checkAndStartTour();
});
</script>
