<template>
  <el-dialog
    title="衣物列表"
    v-model="showClothesDialog"
    width="1200px"
    append-to-body
    lock-scroll
    modal
    :close-on-click-modal="false"
    @closed="close"
    class="modern-dialog"
  >
    <!-- 顶部操作栏 -->
    <div class="top-actions">
      <div class="search-bar">
        <el-input
          placeholder="搜索衣物..."
          prefix-icon="Search"
          v-model="searchQuery"
          clearable
          @clear="getList"
          @keyup.enter="searchCloths"
        >
          <template #append>
            <el-button @click="searchCloths">
              <el-icon><Search /></el-icon>
            </el-button>
          </template>
        </el-input>
      </div>
      <div class="filter-actions">
        <el-select v-model="statusFilter" style="width: 120px;" placeholder="状态筛选" clearable @change="filterByStatus">
          <el-option
            v-for="item in sys_clothing_status"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          >
           
          </el-option>
        </el-select>
      </div>
    </div>

    <!-- 加载状态 -->
    <div v-if="loading" class="loading-container">
      <el-skeleton :rows="3" animated />
      <el-skeleton :rows="3" animated />
    </div>

    <!-- 衣物卡片列表 -->
    <div v-else class="cloth-cards-container">
      <el-empty v-if="clothsList.length === 0" description="暂无衣物数据" />
      
      <div v-else class="cloth-cards">
        <el-card 
          v-for="(item, index) in clothsList" 
          :key="index" 
          class="cloth-card"
          :class="{ 'selected-card': isSelected(item) }"
          shadow="hover"
          @click="toggleSelection(item)"
        >
          <!-- 卡片头部 - 衣物基本信息 -->
          <template #header>
            <div class="card-header">
              <div class="cloth-info">
                <div class="cloth-name">
                  <el-icon><Goods /></el-icon>
                  <span>{{ item.clothInfo.clothingName }}</span>
                  <el-tag 
                    v-if="item.clothingColor" 
                    size="small" 
                    effect="plain" 
                    type="info"
                    class="cloth-tag"
                  >
                    {{ colorList.find(color => color.tagId == item.clothingColor)?.tagName }}
                  </el-tag>
                  <el-tag 
                    v-if="item.clothingBrand" 
                    size="small" 
                    effect="plain" 
                    type="success"
                    class="cloth-tag"
                  >
                    {{ brandList.find(brand => brand.tagId == item.clothingBrand)?.tagName }}
                  </el-tag>
                </div>
                <div class="cloth-code">
                  <el-tooltip content="衣物编码" placement="top">
                    <el-tag type="info" effect="plain">
                      <el-icon><Ticket /></el-icon>
                      {{ item.hangClothCode }}
                    </el-tag>
                  </el-tooltip>
                </div>
              </div>
              <div class="cloth-status">
                <dict-tag :options="sys_clothing_status" :value="item.clothingStatus" />
              </div>
            </div>
          </template>

          <!-- 卡片内容 -->
          <div class="card-content">
            <!-- 服务信息 -->
            <div class="info-section">
              <div class="section-title">
                <el-icon><Service /></el-icon>
                <span>服务信息</span>
              </div>
              <div class="service-info">
                <div class="info-item full-width">
                  <span class="item-label">服务类型:</span>
                  <span class="service-type">
                    <dict-tag :options="sys_service_type" :value="item.serviceType" />
                    -
                    <dict-tag :options="sys_service_requirement" :value="item.serviceRequirement" />
                  </span>
                </div>
                <div class="info-item full-width">
                  <span class="item-label">价格:</span>
                  <span class="price-value">¥{{ item.priceValue }}</span>
                  <span v-if="item.processMarkup && item.processMarkup > 0" class="markup">
                    (工艺加价: ¥{{ item.processMarkup }})
                  </span>
                </div>
              </div>
            </div>

            <!-- 衣物详情 -->
            <div class="info-section cloth-details">
              <div class="section-title">
                <el-icon><InfoFilled /></el-icon>
                <span>瑕疵/预估</span>
              </div>
              <div class="tags-container" v-if="item.clothingFlaw || item.estimate">
                <!-- 瑕疵信息 -->
                <div v-if="item.clothingFlaw" class="tag-group">
                  <span class="tag-label">瑕疵:</span>
                  <div class="tags">
                    <el-tag 
                      v-for="tagId in item.clothingFlaw.split(',')" 
                      :key="tagId"
                      type="danger"
                      size="small"
                      effect="light"
                    >
                      {{ flawList.find(flaw => flaw.tagId == tagId)?.tagName }}
                    </el-tag>
                  </div>
                </div>
                
                <!-- 洗后预估 -->
                <div v-if="item.estimate" class="tag-group">
                  <span class="tag-label">预估:</span>
                  <div class="tags">
                    <el-tag 
                      v-for="tagId in item.estimate.split(',')" 
                      :key="tagId"
                      type="warning"
                      size="small"
                      effect="light"
                    >
                      {{ estimateList.find(est => est.tagId == tagId)?.tagName }}
                    </el-tag>
                  </div>
                </div>
              </div>
              <!-- 无衣物详情数据时显示的占位内容 -->
              <div class="no-details" v-else>
                <span>暂无瑕疵/预估数据</span>
              </div>
            </div>

            <!-- 位置和取回信息 -->
            <div class="info-section">
              <div class="section-title">
                <el-icon><Location /></el-icon>
                <span>位置与取回</span>
              </div>
              <div class="info-row">
                <div class="info-item" v-if="item.hangLocationCode">
                  <span class="item-label">上挂位置:</span>
                  <el-tag type="info" effect="plain" size="small">
                    {{ item.hangerName }}-{{ item.hangerNumber }}
                  </el-tag>
                </div>
                <div class="info-item">
                  <span class="item-label">取回方式:</span>
                  <el-tag v-if="!item.pickupMethod" effect="plain">未取走</el-tag>
                  <dict-tag v-else :options="sys_delivery_mode" :value="item.pickupMethod" />
                </div>
                <div class="info-item" v-if="item.pickupTime">
                  <span class="item-label">取回时间:</span>
                  <span>{{ formatTime(item.pickupTime, '{y}-{m}-{d}') }}</span>
                </div>
              </div>
              <div class="info-item remark" v-if="item.hangRemark">
                <span class="item-label">备注:</span>
                <span>{{ item.hangRemark }}</span>
              </div>
            </div>

            <!-- 操作按钮 -->
            <div class="card-actions">
              <el-button 
                type="primary" 
                :icon="Picture" 
                size="small"
                :disabled="!item.beforePics || item.beforePics.length === 0"
                @click.stop="handleShowPicture(item, true)"
              >
                洗前照片
              </el-button>
              <el-button 
                type="success" 
                :icon="Picture" 
                size="small"
                :disabled="!item.afterPics || item.afterPics.length === 0"
                @click.stop="handleShowPicture(item, false)"
              >
                洗后照片
              </el-button>
              <el-button 
                type="warning" 
                :icon="Top" 
                size="small"
                v-if="item.clothingStatus == '01'"
                @click.stop="handleShowHangUp(item)"
              >
                上挂
              </el-button>
            </div>
          </div>
        </el-card>
      </div>
    </div>

    <!-- 底部操作栏 -->
    <div class="bottom-actions">
      <div class="selection-info" v-if="selectionList.length > 0">
        已选择 <span class="selection-count">{{ selectionList.length }}</span> 件衣物
      </div>
      <div class="action-buttons">
        <el-button type="warning" plain :disabled="afterSaleDisabled" @click="afterSale">
          <el-icon><Warning /></el-icon> 售后
        </el-button>
        <el-button type="danger" plain :disabled="compensationDisabled" @click="handleCompensate">
          <el-icon><Money /></el-icon> 赔偿
        </el-button>
      </div>
    </div>

    <!-- 展示照片对话框 -->
    <el-dialog
      title="照片预览"
      v-model="showPicture"
      width="600px"
      append-to-body
      class="picture-dialog"
    >
      <div class="picture-container">
        <el-empty v-if="pictureList.length === 0" description="暂无照片" />
        <el-carousel v-else :interval="4000" type="card" height="300px">
          <el-carousel-item v-for="(item, index) in pictureList" :key="index">
            <el-image
              :src="item"
              fit="contain"
              :preview-src-list="pictureList"
              class="carousel-image"
            />
          </el-carousel-item>
        </el-carousel>
        <div class="picture-grid" v-if="pictureList.length > 0">
          <el-image
            v-for="(item, index) in pictureList"
            :key="index"
            :src="item"
            fit="cover"
            :preview-src-list="pictureList"
            class="grid-image"
          />
        </div>
      </div>
    </el-dialog>

    <!-- 上挂对话框 -->
    <el-dialog
      title="衣物上挂"
      v-model="showHangUp"
      width="450px"
      :show-close="false"
      append-to-body
      :before-close="closeHangUpDialog"
      class="hangup-dialog"
    >
      <div class="hangup-content">
        <div class="cloth-preview" v-if="currentCloth">
          <div class="preview-header">
            <el-icon><Goods /></el-icon>
            <span class="cloth-title">{{ currentCloth.clothInfo?.clothingName }}</span>
            <el-tag 
              v-if="currentCloth.clothingColor" 
              size="small" 
              effect="plain" 
              type="info"
            >
              {{ colorList.find(color => color.tagId == currentCloth.clothingColor)?.tagName }}
            </el-tag>
            <el-tag 
              v-if="currentCloth.clothingBrand" 
              size="small" 
              effect="plain" 
              type="success"
            >
              {{ brandList.find(brand => brand.tagId == currentCloth.clothingBrand)?.tagName }}
            </el-tag>
          </div>
        </div>

        <el-form ref="hangUpRef" :model="hangForm" :rules="hangRules" label-width="90px" class="hangup-form">
          <el-form-item label="衣物编码" prop="clothingNumber">
            <el-input
              v-model="hangForm.clothingNumber"
              placeholder="请输入衣物编码"
              prefix-icon="Ticket"
            />
          </el-form-item>

          <el-form-item label="衣挂位置" prop="hangLocationId">
            <el-select v-model="hangForm.hangLocationId" placeholder="请选择上挂位置" class="full-width">
              <el-option
                v-for="item in hangLocationList"
                :key="item.id"
                :label="item.name"
                :value="item.id"
              />
            </el-select>
          </el-form-item>

          <el-form-item label="衣挂编号" prop="hangerNumber">
            <el-input
              v-model="hangForm.hangerNumber"
              placeholder="请输入上挂衣物编码"
              prefix-icon="Location"
            />
          </el-form-item>

          <el-form-item label="备注信息" prop="hangRemark">
            <el-input
              type="textarea"
              v-model="hangForm.hangRemark"
              placeholder="请输入上挂描述信息"
              rows="3"
            />
          </el-form-item>
        </el-form>
      </div>

      <template #footer>
        <div class="dialog-footer">
          <el-button @click="showHangUp = false" plain>取消</el-button>
          <el-button type="primary" @click="hangUp">确认上挂</el-button>
        </div>
      </template>
    </el-dialog>

    <!-- 赔偿对话框 -->
    <el-dialog
      title="衣物赔偿"
      v-model="showCompensationDialog"
      width="500px"
      append-to-body
      class="compensation-dialog"
    >
      <el-form ref="compensationRef" :model="compensationForm" :rules="compensationRules" label-width="90px">
        <el-form-item label="支出账目" prop="expTitle">
          <el-input v-model="compensationForm.expTitle" placeholder="请输入支出账目名称" />
        </el-form-item>
        <el-form-item label="对方账户" prop="recvAccountTitle">
          <el-input v-model="compensationForm.recvAccountTitle" disabled />
        </el-form-item>
        <el-form-item label="赔偿金额" prop="expAmount">
          <el-input-number
            v-model="compensationForm.expAmount"
            controls-position="right"
            :precision="2"
            :min="0"
            placeholder="请输入赔偿金额"
            class="full-width"
          />
        </el-form-item>
        <el-form-item label="备注信息" prop="remark">
          <el-input
            type="textarea"
            v-model="compensationForm.remark"
            placeholder="请输入备注信息"
            rows="3"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="showCompensationDialog = false" plain>取消</el-button>
          <el-button type="primary" @click="compensate">确认赔偿</el-button>
        </div>
      </template>
    </el-dialog>
  </el-dialog>
</template>

<script setup name="ClothsModern">
import { listCloths } from "@/api/system/cloths";
import { listTagsNoLimit } from "@/api/system/tags";
import { listRack } from "@/api/system/rack";
import { hangup } from "@/api/system/cloths";
import { getUser } from "@/api/system/user";
import { addExpenditure } from "@/api/system/expenditure";
import { invoke } from '@tauri-apps/api/core';
import { Search, Goods, Location, Picture, Top, InfoFilled, Warning, Money, Ticket, Service } from '@element-plus/icons-vue';

const props = defineProps({
  visible: {
    type: Boolean,
    required: true,
    default: false,
  },
  orderId: {
    type: Number,
    required: true,
    default: 0
  },
  flashList: {
    type: Function,
    required: true,
  },
  userId: {
    type: String,
    required: true,
  },
  toggle: {
    type: Function,
    required: true,
  },
});

const { proxy } = getCurrentInstance();

const { sys_delivery_mode, sys_clothing_status, sys_service_type, sys_service_requirement } =
  proxy.useDict("sys_delivery_mode", "sys_clothing_status", "sys_service_type", "sys_service_requirement");

// 状态数据
const selectionList = ref([]);
const clothsList = ref([]);
const pictureList = ref([]);
const currentCloth = ref({});
const showPicture = ref(false);
const showCompensationDialog = ref(false);
const showClothesDialog = ref(false);
const loading = ref(true);
const showHangUp = ref(false);
const colorList = ref([]);
const flawList = ref([]);
const estimateList = ref([]);
const brandList = ref([]);
const hangLocationList = ref([]);

// 搜索和筛选
const searchQuery = ref('');
const statusFilter = ref('');

// 按钮状态
const afterSaleDisabled = ref(true);
const compensationDisabled = ref(true);
const pickupDisabled = ref(true);


// 表单数据
const data = reactive({
  hangForm: {},
  pickupForm: {},
  compensationForm: {},
  hangRules: {
    clothingNumber: [
      { required: true, message: "衣物编码不能为空", trigger: "change" }
    ],
    hangLocationId: [
      { required: true, message: "衣挂位置不能为空", trigger: "blur" }
    ],
    hangerNumber: [
      { required: true, message: "衣挂编号不能为空", trigger: "blur" }
    ]
  },
  compensationRules: {
    expTitle: [
      { required: true, message: "支出账目不能为空", trigger: "blur" }
    ],
    expAmount: [
      { required: true, message: "赔偿金额不能为空", trigger: "blur" }
    ]
  }
});

const { hangForm, compensationForm, hangRules, compensationRules } = toRefs(data);

// 关闭对话框
function close() {
  showClothesDialog.value = false;
  props.toggle();
}

// 获取衣物列表
async function getList() {
  if (props.orderId == 0) {
    return;
  }
  loading.value = true;

  await listCloths({ orderId: props.orderId }).then(response => {
    clothsList.value = response;
    loading.value = false;
  });
}

// 搜索衣物
function searchCloths() {
  if (!searchQuery.value) {
    getList();
    return;
  }
  
  loading.value = true;
  const query = searchQuery.value.toLowerCase();
  
  listCloths({ orderId: props.orderId }).then(response => {
    clothsList.value = response.filter(item => {
      return item.clothInfo.clothingName.toLowerCase().includes(query) ||
             item.hangClothCode.toLowerCase().includes(query);
    });
    loading.value = false;
  });
}

// 状态筛选
function filterByStatus() {
  if (!statusFilter.value) {
    getList();
    return;
  }
  
  loading.value = true;
  
  listCloths({ orderId: props.orderId }).then(response => {
    clothsList.value = response.filter(item => {
      return item.clothingStatus === statusFilter.value;
    });
    loading.value = false;
  });
}

// 获取状态类型
function getStatusType(status) {
  switch(status) {
    case '00': return 'info';
    case '01': return 'warning';
    case '02': return 'success';
    case '03': return 'danger';
    default: return 'info';
  }
}


// 选择处理
function isSelected(item) {
  return selectionList.value.some(selected => selected.clothId === item.clothId);
}

function toggleSelection(item) {
  const index = selectionList.value.findIndex(selected => selected.clothId === item.clothId);
  
  if (index > -1) {
    selectionList.value.splice(index, 1);
  } else {
    selectionList.value.push(item);
  }
  
  updateButtonStatus();
}

function updateButtonStatus() {
  afterSaleDisabled.value = !selectionList.value.some(item => item.clothingStatus == '00');
  compensationDisabled.value = !selectionList.value.some(item => item.clothingStatus == '02');
  pickupDisabled.value = !selectionList.value.some(item => item.clothingStatus == '02');
}

// 初始化列表数据
async function initList() {
  const promises = [];

  // 获取颜色列表
  if (colorList.value.length === 0) {
    const colorPromise = listTagsNoLimit({ tagOrder: '003' }).then(response => {
      colorList.value = response;
    });
    promises.push(colorPromise);
  }

  // 获取瑕疵列表
  if (flawList.value.length === 0) {
    const flawPromise = listTagsNoLimit({ tagOrder: '001' }).then(response => {
      flawList.value = response;
    });
    promises.push(flawPromise);
  }

  // 获取预估列表
  if (estimateList.value.length === 0) {
    const estimatePromise = listTagsNoLimit({ tagOrder: '002' }).then(response => {
      estimateList.value = response;
    });
    promises.push(estimatePromise);
  }

  // 获取品牌列表
  if (brandList.value.length === 0) {
    const brandPromise = listTagsNoLimit({ tagOrder: '004' }).then(response => {
      brandList.value = response;
    });
    promises.push(brandPromise);
  }
  
  // 获取衣挂位置列表
  const rackPromise = listRack().then(res => {
    hangLocationList.value = res;
  });
  promises.push(rackPromise);

  // 等待所有异步操作完成
  await Promise.all(promises);
}

// 显示上挂对话框
function handleShowHangUp(row) {
  showHangUp.value = true;
  currentCloth.value = row;
  hangForm.value = {
    clothId: row.clothId,
    clothingNumber: row.hangClothCode,
    hangLocationId: row.hangLocationCode,
    hangerNumber: row.hangerNumber,
    hangRemark: null
  };
}

// 上挂操作
function hangUp() {
  if (currentCloth.value) {
    proxy.$refs["hangUpRef"].validate(async valid => {
      if (valid) {
        proxy.$modal.loading("上挂中...");
        await hangup(hangForm.value).then(res => {
          proxy.notify.success("上挂成功");
          showHangUp.value = false;
          getList();
          props.flashList();
        }).catch(res => {
          proxy.$modal.msgError(res.msg);
        });
        proxy.$modal.closeLoading();
      }
    });
  }
}

// 关闭上挂对话框
function closeHangUpDialog(done) {
  hangForm.value = {
    clothingNumber: null,
    hangLocationId: null,
    hangerNumber: null,
    hangRemark: null
  };
  done && done();
}

// 赔偿操作
function handleCompensate() {
  getUser(props.userId).then(res => {
    showCompensationDialog.value = true;
    let title = selectionList.value[0].clothInfo.clothingName;
    if (selectionList.value.length > 1) {
      title += '...';
    }
    compensationForm.value = {
      expTitle: title,
      recvAccountTitle: res.data.userName,
      recvAccount: res.data.userId,
      expAmount: null,
      expType: "01",
      orderId: props.orderId,
      clothIds: selectionList.value.map(item => item.clothId).join(','),
      remark: null,
    }
  });
}

// 确认赔偿
function compensate() {
  proxy.$refs["compensationRef"].validate(valid => {
    if (valid) {
      addExpenditure(compensationForm.value).then(res => {
        proxy.notify.success("赔偿成功");
        showCompensationDialog.value = false;
        getList();
        props.flashList();
      }).catch(err => {
        proxy.$modal.msgError(err.msg || "赔偿失败");
      });
    }
  });
}

// 售后处理
function afterSale() {
  // 实现售后处理逻辑
  proxy.$modal.msgInfo("售后功能开发中");
}

// 加载图片
const loadImage = async (id) => {
  try {
    // 调用 Tauri 后端命令获取图片二进制数据
    const imageData = await invoke('get_image', { id });

    // 将二进制数据转换为 Blob
    const blob = new Blob([new Uint8Array(imageData)], { type: 'image/png' });

    // 生成图片 URL
    return URL.createObjectURL(blob);
  } catch (error) {
    console.error(`加载图片失败: ${error}`);
    return null;
  }
};

// 显示图片
async function handleShowPicture(row, flag) {
  showPicture.value = true;

  try {
    // 获取图片 ID 列表
    const picIds = flag ? row.beforePics?.split(',') : row.afterPics?.split(',');

    if (picIds && picIds.length > 0) {
      // 使用 Promise.all 等待所有图片加载完成
      const imageUrls = await Promise.all(picIds.map(id => loadImage(Number(id))));

      // 过滤掉加载失败的图片（null）
      pictureList.value = imageUrls.filter(url => url !== null);
    } else {
      pictureList.value = []; // 如果没有图片 ID，清空列表
    }
  } catch (error) {
    console.error(`处理图片列表失败: ${error}`);
    pictureList.value = []; // 出错时清空列表
  }
}

onMounted(async () => {
  if (props.visible) {
    await initList();  // 确保 initList 完成
    await getList();   // 在 initList 完成后调用
    showClothesDialog.value = true;
  }
});
</script>

<style scoped>
/* 对话框样式 */
.modern-dialog :deep(.el-dialog__header) {
  padding: 16px 20px;
  border-bottom: 1px solid var(--el-border-color-lighter);
  margin-right: 0;
}

.modern-dialog :deep(.el-dialog__title) {
  font-size: 18px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.modern-dialog :deep(.el-dialog__body) {
  padding: 20px;
}

.modern-dialog :deep(.el-dialog__footer) {
  padding: 16px 20px;
  border-top: 1px solid var(--el-border-color-lighter);
}

/* 顶部操作栏 */
.top-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.search-bar {
  width: 300px;
}

.filter-actions {
  display: flex;
  gap: 10px;
}

/* 加载状态 */
.loading-container {
  padding: 20px;
}

/* 卡片列表样式 */
.cloth-cards-container {
  min-height: 300px;
}

.cloth-cards {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: 16px;
  margin-bottom: 20px;
}

.cloth-card {
  transition: all 0.3s ease;
  border: 1px solid var(--el-border-color-lighter);
}

.cloth-card:hover {
  transform: translateY(-3px);
  box-shadow: 0 6px 12px rgba(0, 0, 0, 0.1);
}

.selected-card {
  border: 2px solid var(--el-color-primary);
  box-shadow: 0 4px 12px rgba(var(--el-color-primary-rgb), 0.2);
}

/* 卡片头部 */
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.cloth-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.cloth-name {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 500;
  font-size: 16px;
}

.cloth-tag {
  margin-left: 4px;
}

.cloth-code {
  margin-top: 4px;
}

/* 卡片内容 */
.card-content {
  padding: 8px 0;
}

.info-section {
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px dashed var(--el-border-color-lighter);
}

.info-section:last-child {
  border-bottom: none;
  margin-bottom: 0;
  padding-bottom: 0;
}

.section-title {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 10px;
  font-weight: 500;
  color: var(--el-text-color-primary);
  font-size: 15px;
}

/* 服务信息样式 */
.service-info {
  display: flex;
  flex-direction: column;
  gap: 8px;

  .service-type{
    display: flex;
    gap: .5rem;
  }
}

.info-row {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  margin-bottom: 8px;
}

.info-item {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 6px;
}

.full-width {
  width: 100%;
}

.item-label {
  color: var(--el-text-color-secondary);
  font-size: 13px;
  min-width: 60px;
}

.price-value {
  font-weight: 500;
  color: var(--el-color-danger);
}

.markup {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

/* 衣物详情样式 */
.cloth-details {
  min-height: 80px;
}

.no-details {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 40px;
  color: var(--el-text-color-secondary);
  font-size: 13px;
  background-color: var(--el-fill-color-light);
  border-radius: 4px;
}

.info-row {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  margin-bottom: 8px;
}

.info-item {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 6px;
}

.item-label {
  color: var(--el-text-color-secondary);
  font-size: 13px;
}

.price-value {
  font-weight: 500;
  color: var(--el-color-danger);
}

.markup {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

/* 标签容器 */
.tags-container {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.tag-group {
  display: flex;
  align-items: flex-start;
  gap: 8px;
}

.tag-label {
  color: var(--el-text-color-secondary);
  font-size: 13px;
  white-space: nowrap;
}

.tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

/* 备注信息 */
.remark {
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px dashed var(--el-border-color-lighter);
}

/* 操作按钮 */
.card-actions {
  display: flex;
  justify-content: space-evenly;
  gap: 8px;
  margin-top: 16px;
}

/* 分页容器 */
.pagination-container {
  display: flex;
  justify-content: center;
  margin-top: 20px;
  margin-bottom: 10px;
}

/* 底部操作栏 */
.bottom-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 20px;
  padding-top: 16px;
  border-top: 1px solid var(--el-border-color-lighter);
}

.selection-info {
  font-size: 14px;
  color: var(--el-text-color-secondary);
}

.selection-count {
  font-weight: 600;
  color: var(--el-color-primary);
}

.action-buttons {
  display: flex;
  gap: 10px;
}

/* 照片对话框 */
.picture-container {
  padding: 10px 0;
}

.carousel-image {
  width: 100%;
  height: 100%;
  border-radius: 4px;
}

.picture-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
  gap: 10px;
  margin-top: 20px;
}

.grid-image {
  width: 100%;
  height: 100px;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.grid-image:hover {
  transform: scale(1.05);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

/* 上挂对话框 */
.hangup-dialog :deep(.el-dialog__header) {
  padding: 16px 20px;
  border-bottom: 1px solid var(--el-border-color-lighter);
}

.hangup-content {
  padding: 10px 0;
}

.cloth-preview {
  background-color: var(--el-fill-color-light);
  padding: 16px;
  border-radius: 4px;
  margin-bottom: 20px;
}

.preview-header {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.cloth-title {
  font-weight: 500;
  font-size: 16px;
}

.hangup-form {
  margin-top: 20px;
}

.full-width {
  width: 100%;
}

/* 对话框底部 */
.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

/* 响应式调整 */
@media (max-width: 768px) {
  .cloth-cards {
    grid-template-columns: 1fr;
  }
  
  .top-actions {
    flex-direction: column;
    align-items: stretch;
    gap: 10px;
  }
  
  .search-bar {
    width: 100%;
  }
}
</style>