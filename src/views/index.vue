<template>
  <div class="app-container">
    <div class="home">
      <el-button class="menu" :color="menu.color" :dark="menu.dark" :type="menu.type" @click="menu.onClick" plain
        v-for="menu in menus" v-show="menu.show">
        {{ menu.name }}
      </el-button>
    </div>

    <OrderContent :visible="showOrderDialog" :taggle="() => { showOrderDialog = !showOrderDialog }"
      :key="showOrderDialog" />
    <el-dialog :show-close="false" v-model="open" width="1440px" append-to-body lock-scroll modal :before-close="cancel"
      :close-on-click-modal="false">
      <CreateOrder ref="createOrderRef" :orderId="0" :userId="0" :toggle="() => { open = !open }" :refresh="() => { }"
        :key="open" />
    </el-dialog>

    <!-- 卡券售卖弹窗 -->
    <el-dialog title="" v-model="showCouponSale" width="1080px" append-to-body lock-scroll modal
      :close-on-click-modal="false">
      <CouponSale :key="showCouponSale" />
    </el-dialog>

    <!-- 新增会员 -->
    <AddUser :visible="showAddUserDialog" :key="showAddUserDialog"
      :taggle="() => { showAddUserDialog = !showAddUserDialog }" />
    <CouponGift :visible="showCouponGift" :key="showCouponGift" :taggle="() => { showCouponGift = !showCouponGift }" />
    <HangUp :visible="showHangUp" :key="showHangUp" :taggle="() => { showHangUp = !showHangUp }" />
  </div>
</template>

<script setup name="Index">
import OrderContent from '@/views/home/oderContent.vue'
import CreateOrder from '@/views/home/createOrder.vue'
import CouponSale from '@/views/home/couponSale.vue'
import AddUser from '@/views/home/addUser.vue'
import CouponGift from '@/views/home/couponGift.vue';
import HangUp from '@/views/home/hangUp.vue';

const version = ref('3.8.8')

const showCouponSale = ref(false);
const showCouponGift = ref(false);

const showOrderDialog = ref(false);
const showAddUserDialog = ref(false);
const open = ref(false);

const showHangUp = ref(false);

/*
 * 菜单
 * color: 自定义颜色
 * dark: 是否为暗黑主题
 */
const menus = ref([
  { 'name': '收衣收鞋', 'type': 'primary', show: true, onClick: () => { open.value = true } },
  { 'name': '取衣取鞋', 'type': 'primary', show: true, onClick: () => { showOrderDialog.value = true } },
  { 'name': '上挂', 'type': 'primary', show: true, onClick: () => { showHangUp.value = true } },
  { 'name': '交期预警', 'type': 'warning', show: false },
  { 'name': '派送提醒', 'type': 'primary', show: false },
  { 'name': '卡券销售', 'type': 'primary', show: true, onClick: () => { showCouponSale.value = true } },
  { 'name': '卡券赠送', 'type': 'primary', show: true, onClick: () => { showCouponGift.value = true } },
  { 'name': '收款', 'type': 'primary', show: false },
  { 'name': '撤单处理', 'type': 'danger', show: false },
  { 'name': '新增会员', 'type': 'primary', show: true, onClick: () => { showAddUserDialog.value = true } },
  { 'name': '事故处理', 'type': 'danger', show: false },
  { 'name': '线上预约', 'type': 'success', show: false },
  { 'name': '取件预约', 'type': 'success', show: false },
  { 'name': '派送预约', 'type': 'success', show: false },
  { 'name': '线上沟通', 'type': 'success', show: false },
  { 'name': '团购管理', 'type': 'success', show: false },
  { 'name': '综合报表', 'type': 'success', show: false },
  { 'name': '收支报表', 'type': 'success', show: false },
  { 'name': '经营对账', 'type': 'success', show: false },
  { 'name': '新增支出', 'type': 'success', show: false },
  { 'name': '营销推送', 'type': 'success', show: false, color: '#626aef', dark: false },
]);
const createOrderRef = ref();

function cancel(done) {
  if (createOrderRef.value) {
    createOrderRef.value.cancel()
      .then((canClose) => {
        if (canClose) {
          done(); // 允许关闭对话框
        }
      })
      .catch(() => {
        // 用户选择取消，不关闭对话框
      });
  } else {
    done(); // 子组件不存在时，直接关闭对话框
  }
}
</script>

<style scoped lang="scss">
.app-container {
  width: 100%;
  height: calc(100vh - 84px);
  background-color: #efeeee;
  overflow-y: auto;
}

.home {
  margin: auto;
  max-width: 100rem;
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  justify-content: center;
  align-items: center;
  gap: 1rem;

  .menu {
    max-width: 15rem;
    aspect-ratio: 1/1;
    padding: 1rem;
    display: flex;
    justify-content: center;
    align-items: center;
    border: none;
    border-radius: 1rem;
    box-shadow: 18px 18px 30px rgba(0, 0, 0, 0.2),
      -18px -18px 30px rgba(255, 255, 255);
    text-align: center;
    transition: all .2s;

    &:hover {
      cursor: pointer;
      box-shadow: 0 0 0 rgba(0, 0, 0, 0.2),
        0 0 0 rgba(255, 255, 255, 0.8),
        inset 18px 18px 30px rgba(0, 0, 0, 0.1),
        inset -18px -18px 30px rgba(255, 255, 255, .2);

      transition: all .2s;
    }
  }

  button {
    width: 100%;
    height: 100%;
    margin: 0;
    // font-size: large;
  }
}

.row-class-name {
  background-color: rgb(5, 252, 169);
}

@media (min-width: 1000px) {
  .home {
    /* 在大屏幕时增加间距 */
    gap: 40px;

    button {
      font-size: larger;
    }
  }
}

@media (max-width: 500px) {
  .home {
    /* 在大屏幕时增加间距 */
    grid-template-columns: repeat(4, 1fr);
  }
}
</style>
