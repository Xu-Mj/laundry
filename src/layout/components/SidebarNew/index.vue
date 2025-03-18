<template>
  <Sidebar :switch="switchAdmin" v-if="isAdmin" />
  <div class="sidebar" v-else>
    <logo :collapse="false" />
    <el-scrollbar wrap-class="scrollbar-wrapper" :view-class="['menu-list', { 'single-column': isSingleColumn }]">
      <button class="btn menu" :class="{ active: route.path == menu.path }" :color="menu.color" :dark="menu.dark"
        :type="menu.type" @click="handleMenuClick(menu)" plain v-for="menu in menus" v-show="menu.show"
        :ref="el => { if (el) menuRefs[menu.name] = el }">
        {{ menu.name }}
      </button>
    </el-scrollbar>
    <navbar :switch="switchAdmin" :isAdmin="true" />
    <CouponSale v-if="showCouponSale" :key="showCouponSale" :visible="showCouponSale"
      :taggle="() => { showCouponSale = !showCouponSale }" />
    <AddUser :visible="showAddUserDialog" :key="showAddUserDialog"
      :taggle="() => { showAddUserDialog = !showAddUserDialog }" />
    <CouponGift :visible="showCouponGift" :key="showCouponGift" :taggle="() => { showCouponGift = !showCouponGift }" />
    <HangUp :visible="showHangUp" :key="showHangUp" :taggle="() => { showHangUp = !showHangUp }" />
    <Expenditure :visible="showExp" :key="showExp" title="支出录入" :taggle="() => { showExp = !showExp }" />
    
    <MenuTourGuide :menuRefs="menuRefs" @tour-finished="handleTourFinished" />
  </div>
</template>

<script setup name="SidebarNew">
import { ref, reactive } from 'vue';
import { useRouter } from 'vue-router';
import useAppStore from '@/store/modules/app'
import useSubscriptionStore from '@/store/modules/subscription'
import Logo from '../Sidebar/Logo.vue';
import Navbar from '../Navbar.vue';
import CouponSale from '@/views/components/couponSale.vue'
import Expenditure from '@/views/components/expenditureAdd.vue'
import AddUser from '@/views/components/addUser.vue'
import CouponGift from '@/views/components/couponGift.vue';
import HangUp from '@/views/components/hangUp.vue';
import Sidebar from '../Sidebar/index.vue';
import MenuTourGuide from '@/components/MenuTourGuide/index.vue';

const router = useRouter();
const route = useRoute();

const appStore = useAppStore()

const {proxy} = getCurrentInstance();
const showCouponSale = ref(false);
const showCouponGift = ref(false);
const showAddUserDialog = ref(false);
const showHangUp = ref(false);
const showExp = ref(false);
const isSingleColumn = ref(false);
const isAdmin = ref(localStorage.getItem('isAdmin') === 'true');
const menuRefs = reactive({});

/*
 * 菜单
 * color: 自定义颜色
 * dark: 是否为暗黑主题
 */
const menus = ref([
  { 'name': '首页', 'type': 'primary', show: true, path: '/index', onClick: () => router.push('/') },
  // { 'name': '图片识别', 'type': 'primary', show: true, path: '/image-test', onClick: () => router.push('/image-test') },
  { 'name': '收衣收鞋', 'type': 'primary', show: true, path: '/create-order', onClick: () => router.push('/create-order') },
  { 'name': '取衣取鞋', 'type': 'primary', show: true, path: '/pick-up', onClick: () => router.push('/pick-up') },
  { 'name': '订单管理', 'type': 'primary', show: true, path: '/order-list', onClick: () => router.push('/order-list') },
  { 'name': '衣物上挂', 'type': 'primary', show: true, path: '/hang-up', onClick: hangupClick },
  { 'name': '交期预警', 'type': 'warning', show: false },
  { 'name': '派送提醒', 'type': 'primary', show: false },
  { 'name': '卡券销售', 'type': 'primary', show: false, path: '/coupon-sale', onClick: () => router.push('/coupon-sale') },
  { 'name': '卡券管理', 'type': 'primary', show: true, path: '/coupon', onClick: () => router.push('/coupon') },
  { 'name': '卡券赠送', 'type': 'primary', show: false, onClick: () => { showCouponGift.value = true } },
  { 'name': '收款', 'type': 'primary', show: false },
  { 'name': '撤单处理', 'type': 'danger', show: false },
  { 'name': '新增会员', 'type': 'primary', show: true, onClick: () => { showAddUserDialog.value = true } },
  { 'name': '会员管理', 'type': 'primary', show: true, onClick: () => { router.push('/users') } },
  { 'name': '事故处理', 'type': 'danger', show: false },
  { 'name': '线上预约', 'type': 'success', show: false },
  { 'name': '取件预约', 'type': 'success', show: false },
  { 'name': '派送预约', 'type': 'success', show: false },
  { 'name': '线上沟通', 'type': 'success', show: false },
  { 'name': '团购管理', 'type': 'success', show: false },
  { 'name': '综合报表', 'type': 'success', show: false },
  { 'name': '支出录入', 'type': 'success', show: true, onClick: () => { showExp.value = true } },
  { 'name': '收支报表', 'type': 'success', show: true, path: '/expenditures', onClick: () => router.push('/expenditures') },
  { 'name': '经营对账', 'type': 'success', show: false },
  { 'name': '营销推送', 'type': 'success', show: false, color: '#626aef', dark: false },
  { 'name': '知识天地', 'type': 'primary', path: '/blogs', show: true, onClick: () => { router.push('/blogs') } },
]);

watch(isAdmin, (newValue) => {
  localStorage.setItem('isAdmin', newValue);
  appStore.setSidebarWidth(newValue)
})

const handleMenuClick = (menu) => {
  if (menu.onClick) {
    menu.onClick();
  }
};

const switchAdmin = () => {
  isAdmin.value = !isAdmin.value;
  router.push('/');
};

function hangupClick() {
  // 判断是否是试用期
  if (useSubscriptionStore().isGuest) {
    proxy.notify.warning('当前处于游客模式，请先注册！');
    return;
  }
  if (useSubscriptionStore().isInTrial) {
    // 弹窗提醒
    proxy.notify.warning('您当前为试用期用户，请升级为正式用户后使用！');
    return;
  }
  showHangUp.value = true;
}
// 引导完成后的处理函数
const handleTourFinished = () => {
  console.log('菜单引导已完成');
};

onMounted(() => {
  appStore.setSidebarWidth(isAdmin.value);
})
</script>

<style scoped>
.btn {
  height: 4rem;
  border: none;
  border-radius: 0.4rem;
  cursor: pointer;
  color: rgb(0, 166, 255);
  position: relative;
  background-color: transparent;
  overflow: hidden;
  transition: all 0.3s ease;
  outline: none;
}

.btn:hover {
  color: #fff;
  background-color: rgba(0, 166, 255, 0.7);
}

.btn:hover::before {
  background-color: rgba(0, 166, 255, 0.7);
}

.menu-list .menu.active {
  background-color: var(--el-fill-color);
  border-color: var(--el-color-primary);
  color: var(--el-color-primary);
}


.taggle-icon {
  position: absolute;
  bottom: 1rem;
  right: -1rem;
  width: 2rem;
  height: 2rem;
  background-color: black;
  cursor: pointer;
} 
</style>