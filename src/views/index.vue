<template>
  <div class="app-container">
    <div class="home">
      <el-button class="menu" :color="menu.color" :dark="menu.dark" :type="menu.type" @click="menu.onClick" plain
        v-for="menu in menus">
        {{ menu.name }}
      </el-button>
    </div>

    <OrderContent :visible="showOrderDialog" :taggle="handleShowOrderDialog" />
    <el-dialog :title="title" v-model="open" width="1440px" append-to-body lock-scroll modal :before-close="cancel"
      :close-on-click-modal="false">
      <CreateOrder :orderId="0" :userId="0" :toggle="() => { open = !open }" :refresh="()=>{}" :key="open"/>
    </el-dialog>
  </div>
</template>

<script setup name="Index">
import { ref } from 'vue';
import OrderContent from '@/views/home/oderContent.vue'
import CreateOrder from '@/views/home/createOrder.vue'

const version = ref('3.8.8')
/*
 * 菜单
 * color: 自定义颜色
 * dark: 是否为暗黑主题
 */
const menus = ref([
  { 'name': '收衣收鞋', 'type': 'primary', onClick: () => { console.log('hello'); open.value = true } },
  { 'name': '取衣取鞋', 'type': 'primary', onClick: () => { showOrderDialog.value = true } },
  { 'name': '上挂', 'type': 'primary' },
  { 'name': '交期预警', 'type': 'warning' },
  { 'name': '派送提醒', 'type': 'primary' },
  { 'name': '卡券销售', 'type': 'primary' },
  { 'name': '收款', 'type': 'primary' },
  { 'name': '撤单处理', 'type': 'danger' },
  { 'name': '新增会员', 'type': 'primary' },
  { 'name': '事故处理', 'type': 'danger' },
  { 'name': '线上预约', 'type': 'success' },
  { 'name': '取件预约', 'type': 'success' },
  { 'name': '派送预约', 'type': 'success' },
  { 'name': '线上沟通', 'type': 'success' },
  { 'name': '团购管理', 'type': 'success' },
  { 'name': '综合报表', 'type': 'success' },
  { 'name': '收支报表', 'type': 'success' },
  { 'name': '经营对账', 'type': 'success' },
  { 'name': '新增支出', 'type': 'success' },
  { 'name': '营销推送', 'type': 'success', color: '#626aef', dark: false },
])

const showOrderDialog = ref(false);
const open = ref(false);
function handleShowOrderDialog() {
  showOrderDialog.value = !showOrderDialog.value;
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
