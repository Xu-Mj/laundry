<template>
  <div class="sidebar">
    <logo />
    <el-scrollbar wrap-class="scrollbar-wrapper" :view-class="['menu-list', { 'single-column': isSingleColumn }]">
      <button v-for="menu in menus" class="btn menu" :class="{ active: route.path == menu.path }" :color="menu.color"
        :dark="menu.dark" :type="menu.type" @click="handleMenuClick(menu)" plain v-show="menu.show"
        :ref="el => { if (el) menuRefs[menu.name] = el }">
        {{ menu.name }}
      </button>
    </el-scrollbar>
    <div class="navbar-container">
      <navbar :switch="switchAdmin" :isAdmin="isAdmin" />
    </div>
    <CouponSale v-if="showCouponSale" :key="showCouponSale" :visible="showCouponSale"
      :taggle="() => { showCouponSale = !showCouponSale }" />
    <AddUser :visible="showAddUserDialog" :key="showAddUserDialog"
      :taggle="() => { showAddUserDialog = !showAddUserDialog }" />
    <CouponGift :visible="showCouponGift" :key="showCouponGift" :taggle="() => { showCouponGift = !showCouponGift }" />
    <HangUp :visible="showHangUp" :key="showHangUp" :taggle="() => { showHangUp = !showHangUp }" />
    <Expenditure :visible="showExp" :key="showExp" title="支出录入" :taggle="() => { showExp = !showExp }" />

    <MenuTourGuide :menuRefs="menuRefs" />
  </div>
</template>

<script setup name="Sidebar">
import { useRouter, useRoute } from 'vue-router';
import useUserStore from '@/store/modules/user'
import Logo from './Logo.vue';
import Navbar from '../Navbar.vue';
import CouponSale from '@/views/components/couponSale.vue'
import Expenditure from '@/views/components/expenditureAdd.vue'
import AddUser from '@/views/components/addUser.vue'
import CouponGift from '@/views/components/couponGift.vue';
import HangUp from '@/views/components/hangUp.vue';
import MenuTourGuide from '@/components/MenuTourGuide/index.vue';

const router = useRouter();
const route = useRoute();

const { proxy } = getCurrentInstance();
const showCouponSale = ref(false);
const showCouponGift = ref(false);
const showAddUserDialog = ref(false);
const showHangUp = ref(false);
const showExp = ref(false);
const isSingleColumn = ref(false);
// 根据当前路由路径判断是否为管理端路由
const isSystemRoute = route.path.startsWith('/system');
const isAdmin = ref(isSystemRoute || localStorage.getItem('isAdmin') === 'true');
const menuRefs = reactive({});

/*
 * 菜单
 * color: 自定义颜色
 * dark: 是否为暗黑主题
 */
const normalMenus = [
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
  { 'name': '新增会员', 'type': 'primary', show: true, onClick: () => { showAddUserDialog.value = true } },
  { 'name': '会员管理', 'type': 'primary', show: true, path: '/users', onClick: () => { router.push('/users') } },
  { 'name': '派送管理', 'type': 'primary', show: true, path: '/delivery', onClick: () => { router.push('/delivery') } },
  { 'name': '事故处理', 'type': 'danger', show: false },
  { 'name': '线上预约', 'type': 'success', show: false },
  { 'name': '取件预约', 'type': 'success', show: false },
  { 'name': '派送预约', 'type': 'success', show: false },
  { 'name': '线上沟通', 'type': 'success', show: false },
  { 'name': '团购管理', 'type': 'success', show: false },
  { 'name': '综合报表', 'type': 'success', show: false },
  { 'name': '支出录入', 'type': 'success', show: true, onClick: () => { showExp.value = true } },
  { 'name': '收支报表', 'type': 'success', show: true, path: '/expenditures', onClick: () => router.push('/expenditures') },
  { 'name': '营销推送', 'type': 'success', show: false, color: '#626aef', dark: false },
  { 'name': '知识天地', 'type': 'primary', path: '/blogs', show: false, onClick: () => { router.push('/blogs') } },
];

const manageMenus = [
  { 'name': '首页', 'type': 'primary', show: true, path: '/index', onClick: () => router.push('/') },
  { 'name': '衣挂管理', 'type': 'primary', show: true, path: '/system/rack', onClick: () => router.push('/system/rack') },
  { 'name': '衣物管理', 'type': 'primary', show: true, path: '/system/clothing', onClick: () => router.push('/system/clothing') },
  { 'name': '价格管理', 'type': 'primary', show: true, path: '/system/price', onClick: () => router.push('/system/price') },
  { 'name': '等级管理', 'type': 'primary', show: true, path: '/system/post', onClick: () => router.push('/system/post') },
  { 'name': '标签管理', 'type': 'primary', show: true, path: '/system/tags', onClick: () => router.push('/system/tags') },
  { 'name': '通知记录', 'type': 'primary', show: true, path: '/system/notice-record', onClick: () => router.push('/system/notice-record') },
  { 'name': '参数设置', 'type': 'primary', show: true, path: '/system/config', onClick: () => router.push('/system/config') },
  { 'name': '字典管理', 'type': 'primary', show: true, path: '/system/dict', onClick: () => router.push('/system/dict') },
  { 'name': '衣物分类', 'type': 'primary', show: true, path: '/system/clothingCategory', onClick: () => router.push('/system/clothingCategory') },
];

const menus = computed(() => {
  return isAdmin.value ? manageMenus : normalMenus;
});

const userStore = useUserStore();

watch(isAdmin, (newValue) => {
  localStorage.setItem('isAdmin', newValue);
})

// 监听路由变化，自动切换菜单类型
watch(() => route.path, (newPath) => {
  if (newPath === '/index') {
    return;
  }
  // 判断是否为管理端路由（以/system开头）
  const isSystemRoute = newPath.startsWith('/system');
  // 如果当前路由类型与菜单类型不匹配，则自动切换
  if (isSystemRoute !== isAdmin.value) {
    isAdmin.value = isSystemRoute;
  }
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
  if (userStore.trial.isGuest) {
    proxy.notify.warning('当前处于游客模式，请先注册！');
    return;
  }
  if (userStore.trial.isInTrial) {
    // 弹窗提醒
    proxy.notify.warning('您当前为试用期用户，请升级为正式用户后使用！');
    return;
  }
  showHangUp.value = true;
}
</script>

<style scoped>
.sidebar {
  height: 100%;
  display: flex;
  flex-direction: column;
  position: relative;
}

.el-scrollbar {
  flex: 1;
  overflow-y: auto;
}

.navbar-container {
  width: 100%;
  margin-top: auto;
}

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
</style>