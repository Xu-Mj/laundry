<template>
  <div class="app-container">
    <div class="home">
      <el-button class="menu" :type="menu.type" @click="menu.onClick" plain v-for="menu in menus">
        {{ menu.name }}
      </el-button>
    </div>

    <el-dialog v-model="showOrderDialog" width="1280px" append-to-body>
      <el-table ref="orderTableRef" :data="ordersList" :show-header="false" style="width: 100%" row-key="orderId"
        @row-click="handleRowClick" @expand-change="handleExpandChange" :row-class-name="rowClassName">
        <!-- 可展开行 -->
        <el-table-column type="expand">
          <template #default="props">
            <el-table :data="props.row.clothList" :loading="props.row.loading" border>
              <el-table-column type="selection" width="55" align="center" />
              <el-table-column label="衣物" align="center">
                <template #default="scope">
                  {{ scope.row.clothInfo.clothingName }}
                  {{ scope.row.clothingColor ? '-' + colorList.find(item => item.tagId ==
                    scope.row.clothingColor).tagName : '' }}
                </template>
              </el-table-column>
              <el-table-column label="服务类型" :width="120" align="center">
                <template #default="scope">
                  <span class="service-type">
                    <dict-tag :options="sys_service_type" :value="scope.row.serviceType" />
                    -
                    <dict-tag :options="sys_service_requirement" :value="scope.row.serviceRequirement" />
                  </span>
                </template>
              </el-table-column>
              <el-table-column label="洗护价格" align="center" prop="priceValue" />
              <el-table-column label="工艺加价" align="center" prop="processMarkup" />
              <el-table-column label="衣物瑕疵" align="center" prop="clothingFlaw">
                <template #default="scope">
                  <el-tag v-for="tagId in scope.row.clothingFlaw ? scope.row.clothingFlaw.split(',') : []" :key="item"
                    type="danger">
                    {{ flawList.find(item => item.tagId == tagId).tagName }}
                  </el-tag>
                </template>
              </el-table-column>
              <el-table-column label="洗后预估" align="center" prop="estimate">
                <template #default="scope">
                  <el-tag v-for="tagId in scope.row.estimate ? scope.row.estimate.split(',') : []" :key="item"
                    type="primary">
                    {{ estimateList.find(item => item.tagId == tagId).tagName }}
                  </el-tag>
                </template>
              </el-table-column>
              <el-table-column label="衣物品牌" align="center" prop="clothingBrand">
                <template #default="scope">
                  <el-tag v-if="scope.row.clothingBrand" type="primary">
                    {{ brandList.find(item => item.tagId == scope.row.clothingBrand).tagName }}
                  </el-tag>
                </template>
              </el-table-column>
              <el-table-column label="图片" align="center" :width="95" class-name="small-padding fixed-width">
                <template #default="scope">
                  <el-button link type="primary" @click="handleShowPicture(scope.row, true)"
                    v-hasPermi="['system:cloths:edit']">洗前</el-button>
                  <el-button link type="primary" @click="handleShowPicture(scope.row, false)"
                    v-hasPermi="['system:cloths:edit']">洗后</el-button>
                </template>
              </el-table-column>
              <el-table-column label="状态" align="center" prop="clothingStatus">
                <template #default="scope">
                  <dict-tag :options="sys_clothing_status" :value="scope.row.clothingStatus" />
                </template>
              </el-table-column>
              <el-table-column label="取回方式" align="center" prop="deliveryMode" />
              <el-table-column label="取回时间" align="center" prop="deliveredTime" />
              <el-table-column label="上挂位置" align="center">
                <template #default="scope">
                  {{ scope.row.hangLocationCode ? scope.row.hangerName + '-' + scope.row.hangerNumber : '' }}
                </template>
              </el-table-column>
              <el-table-column label="操作" align="center" :width="280" class-name="small-padding fixed-width">
                <template #default="scope">
                  <el-button link type="primary" icon="Picture" @click="handleShowPicture(scope.row, true)"
                    v-hasPermi="['system:cloths:edit']">洗前</el-button>
                  <el-button link type="primary" icon="Picture" @click="handleShowPicture(scope.row, false)"
                    v-hasPermi="['system:cloths:edit']">洗后</el-button>
                  <el-button link type="primary" icon="Top" @click="handleShowHangUp(scope.row)"
                    v-if="scope.row.clothingStatus == '01'" v-hasPermi="['system:cloths:remove']">
                    上挂
                  </el-button>
                </template>
              </el-table-column>
            </el-table>
          </template>
        </el-table-column>
        <!-- <el-table-column type="expand">
          <template #default="props">
            <el-table :data="props.row.clothsList" border>
              {{ props.row.clothList }}
              <el-table-column type="selection" width="55" align="center" />
              <el-table-column label="衣物" align="center">
                <template #default="scope">
                  {{ scope.row }}
                  {{ scope.row.clothInfo.clothingName }}
                  {{ scope.row.clothingColor ? '-' + colorList.find(item => item.tagId ==
                    scope.row.clothingColor).tagName : '' }}
                </template>
              </el-table-column>
              <el-table-column label="服务类型" :width="120" align="center">
                <template #default="scope">
                  <span class="service-type">
                    <dict-tag :options="sys_service_type" :value="scope.row.serviceType" />
                    -
                    <dict-tag :options="sys_service_requirement" :value="scope.row.serviceRequirement" />
                  </span>
                </template>
              </el-table-column>
              <el-table-column label="洗护价格" align="center" prop="priceValue" />
              <el-table-column label="工艺加价" align="center" prop="processMarkup" />
              <el-table-column label="衣物瑕疵" align="center" prop="clothingFlaw">
                <template #default="scope">
                  <el-tag v-for="tagId in scope.row.clothingFlaw ? scope.row.clothingFlaw.split(',') : []" :key="item"
                    type="danger">
                    {{ flawList.find(item => item.tagId == tagId).tagName }}
                  </el-tag>
                </template>
              </el-table-column>
              <el-table-column label="洗后预估" align="center" prop="estimate">
                <template #default="scope">
                  <el-tag v-for="tagId in scope.row.estimate ? scope.row.estimate.split(',') : []" :key="item"
                    type="primary">
                    {{ estimateList.find(item => item.tagId == tagId).tagName }}
                  </el-tag>
                </template>
              </el-table-column>
              <el-table-column label="衣物品牌" align="center" prop="clothingBrand">
                <template #default="scope">
                  <el-tag v-if="scope.row.clothingBrand" type="primary">
                    {{ brandList.find(item => item.tagId == scope.row.clothingBrand).tagName }}
                  </el-tag>
                </template>
              </el-table-column> -->
        <!-- <el-table-column label="状态" align="center" prop="clothingStatus">
                <template #default="scope">
                  <dict-tag :options="sys_clothing_status" :value="scope.row.clothingStatus" />
                </template>
              </el-table-column>
              <el-table-column label="取回方式" align="center" prop="deliveryMode" />
              <el-table-column label="取回时间" align="center" prop="deliveredTime" />
              <el-table-column label="上挂位置" align="center">
                <template #default="scope">
                  {{ scope.row.hangLocationCode ? scope.row.hangerName + '-' + scope.row.hangerNumber : '' }}
                </template>
              </el-table-column>
              <el-table-column label="操作" align="center" :width="280" class-name="small-padding fixed-width">
                <template #default="scope">
                  <el-button link type="primary" icon="Picture" @click="handleShowPicture(scope.row, true)"
                    v-hasPermi="['system:cloths:edit']">洗前</el-button>
                  <el-button link type="primary" icon="Picture" @click="handleShowPicture(scope.row, false)"
                    v-hasPermi="['system:cloths:edit']">洗后</el-button>
                  <el-button link type="primary" icon="Top" @click="handleShowHangUp(scope.row)"
                    v-if="scope.row.clothingStatus == '01'" v-hasPermi="['system:cloths:remove']">
                    上挂
                  </el-button>
                </template>
              </el-table-column> -->
        <!-- </el-table>
          </template>
        </el-table-column> -->
        <el-table-column label="订单编码" align="center" width="180" prop="orderNumber" />
        <el-table-column label="所属会员" align="center" width="180">
          <template #default="scope">
            {{ scope.row.nickName + ' - ' + scope.row.phonenumber }}
          </template>
        </el-table-column>
        <el-table-column label="取件码" align="center" prop="pickupCode" />
        <el-table-column label="支付状态" align="center" prop="paymentStatus">
          <template #default="scope">
            <dict-tag :options="sys_payment_status" :value="scope.row.paymentStatus" />
          </template>
        </el-table-column>
      </el-table>
      <!--footer包含 四个button -->
      <template #footer>
        <el-button type="primary" @click="">取衣</el-button>
        <el-button @click="">取衣收款</el-button>
        <el-button @click="">上门派送</el-button>
        <el-button @click="">补打小票</el-button>
      </template>

    </el-dialog>
  </div>
</template>

<script setup name="Index">
import { listOrders } from "@/api/system/orders";
import { listCloths } from "@/api/system/cloths";
import { listTags } from "@/api/system/tags";
import { ref } from 'vue';

const version = ref('3.8.8')
const { proxy } = getCurrentInstance();
const {
  sys_cost_time_alarm,
  sys_payment_status,
  sys_price_order_type,
  sys_business_type,
  sys_delivery_mode,
  sys_order_type,
  sys_order_status,
  sys_payment_method,
  sys_notice_method,
  sys_service_requirement,
  sys_service_type,
  sys_clothing_status
} =
  proxy.useDict(
    'sys_cost_time_alarm',
    'sys_payment_status',
    "sys_price_order_type",
    "sys_business_type",
    "sys_delivery_mode",
    "sys_order_type",
    "sys_order_status",
    "sys_payment_method",
    "sys_notice_method",
    "sys_service_requirement",
    "sys_service_type",
    "sys_clothing_status",
  );

const rowClassName = ref("row-class-name")
const menus = ref([
  { 'name': '收衣收鞋', 'type': 'primary', onClick: () => { console.log('hello'); showOrderDialog.value = true } },
  { 'name': '取衣取鞋', 'type': 'primary', onClick: handleShowOrderList },
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
  { 'name': '营销推送', 'type': 'success' },
])
const showOrderDialog = ref(false);

// 订单列表
const ordersList = ref([]);
const loading = ref(true);
const total = ref(0);
const colorList = ref([]);
const flawList = ref([]);
const estimateList = ref([]);
const brandList = ref([]);

const orderTableRef = ref();

function handleExpandChange(row, expanded) {
  if (expanded && !row.clothList) {
    row.loading = true;
    listCloths({ orderClothId: row.orderId }).then(res => {
      row.clothList = res.rows;
      row.loading = false;
    });
  }
}

function handleRowClick(row) {
  orderTableRef.value.toggleRowExpansion(row);
}

async function handleShowOrderList() {
  await initList();
  getList();
  showOrderDialog.value = true;
}

/* 初始化列表数据 */
async function initList() {
  const promises = [];

  // 获取颜色列表
  if (colorList.value.length === 0) {
    const colorPromise = listTags({ tagOrder: '003' }).then(response => {
      colorList.value = response.rows;
    });
    promises.push(colorPromise);
  }

  // 获取瑕疵列表
  if (flawList.value.length === 0) {
    const flawPromise = listTags({ tagOrder: '001' }).then(response => {
      flawList.value = response.rows;
    });
    promises.push(flawPromise);
  }

  // 获取预估列表
  if (estimateList.value.length === 0) {
    const estimatePromise = listTags({ tagOrder: '002' }).then(response => {
      estimateList.value = response.rows;
    });
    promises.push(estimatePromise);
  }

  // 获取品牌列表
  if (brandList.value.length === 0) {
    const brandPromise = listTags({ tagOrder: '004' }).then(response => {
      brandList.value = response.rows;
    });
    promises.push(brandPromise);
  }

  // 等待所有异步操作完成防止衣物列表数据加载完后这里的数据没有准备好而出错
  await Promise.all(promises);
}

/** 查询洗护服务订单列表 */
function getList() {
  loading.value = true;
  listOrders().then(response => {
    ordersList.value = response.rows;
    if (ordersList.value.length < 7) {
      ordersList.value.map(item => {
        item.loading = true;
        listCloths({ orderClothId: item.orderId }).then(res => {
          item.clothList = res.rows;
          item.loading = false;
          // 展开行
          orderTableRef.value.toggleRowExpansion(item);
        })
      })
    } else {
      ordersList.value.map(item => item.loading = false);
    }
    total.value = response.total;
  });
}
</script>

<style scoped lang="scss">
.app-container {
  width: 100%;
  height: calc(100vh - 84px);
  background-color: #efeeee;
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
<style>
.row-class-name {
  background-color: rgb(5, 252, 169) !important;
}
</style>