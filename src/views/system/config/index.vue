<template>
  <div class="config-container">
    <div class="config-header">
      <h2>系统配置</h2>
      <p class="subtitle">修改系统关键参数</p>
    </div>
    
    <div class="config-cards">
      <!-- 验证码开关配置卡片 -->
      <el-card v-for="(config, index) in configList" :key="index" class="config-card" :class="{ 'card-highlight': editingItem === index }">
        <div class="card-content">
          <div class="card-header">
            <div class="card-title">{{ config.configName }}</div>
            <el-tag size="small" :type="config.configType === 'Y' ? 'success' : 'info'">
              {{ config.configType === 'Y' ? '系统内置' : '自定义' }}
            </el-tag>
          </div>
          
          <div class="card-body">
            <!-- 验证码开关使用 Switch 组件 -->
            <div class="card-value">
              <template v-if="config.configKey === 'sys.account.captchaEnabled'">
                <template v-if="editingItem === index">
                  <div class="toggle-wrapper">
                    <span class="toggle-label">{{ tempSwitchValue ? '开启' : '关闭' }}</span>
                    <el-switch
                      v-model="tempSwitchValue"
                      active-color="#13ce66"
                      inactive-color="#ff4949"
                      :active-text="'开启'"
                      :inactive-text="'关闭'"
                    />
                  </div>
                </template>
                <template v-else>
                  <div class="value-display">
                    <span class="value-text status-text">{{ config.configValue === 'true' ? '已开启' : '已关闭' }}</span>
                  </div>
                </template>
              </template>
              
              <!-- 预计取衣时间配置 -->
              <template v-else-if="config.configKey === 'desire_complete_time'">
                <template v-if="editingItem === index">
                  <div class="numeric-input-wrapper">
                    <el-input-number
                      v-model="tempNumericValue"
                      :min="1"
                      :max="30"
                      size="large"
                      controls-position="right"
                    />
                    <span class="input-suffix">天</span>
                  </div>
                  <div class="input-help-text">请输入1-30之间的数值（单位：天）</div>
                </template>
                <template v-else>
                  <div class="value-display">
                    <span class="value-text">{{ config.configValue }}</span>
                    <span class="value-unit">天</span>
                  </div>
                </template>
              </template>
              
              <!-- 页面无操作锁定时间配置 -->
              <template v-else-if="config.configKey === 'logout_timeout'">
                <template v-if="editingItem === index">
                  <div class="numeric-input-wrapper">
                    <el-input-number
                      v-model="tempNumericValue"
                      :min="60"
                      :max="3600"
                      :step="60"
                      size="large"
                      controls-position="right"
                    />
                    <span class="input-suffix">秒</span>
                  </div>
                  <div class="input-help-text">请输入60-3600之间的数值（单位：秒）</div>
                </template>
                <template v-else>
                  <div class="value-display">
                    <span class="value-text">{{ config.configValue }}</span>
                    <span class="value-unit">秒</span>
                    <span class="value-description">{{ formatTimeDescription(config.configValue) }}</span>
                  </div>
                </template>
              </template>
              
              <!-- 默认文本输入 -->
              <template v-else>
                <template v-if="editingItem === index">
                  <el-input 
                    v-model="tempValue" 
                    size="large"
                    :placeholder="config.configValue" 
                    @keyup.enter="saveConfig(config)"
                  />
                </template>
                <template v-else>
                  <div class="value-display">
                    <span class="value-text">{{ config.configValue }}</span>
                  </div>
                </template>
              </template>
            </div>
            
            <div class="card-remark">
              <el-tooltip :content="config.remark" placement="top" :show-after="500">
                <el-icon><InfoFilled /></el-icon>
              </el-tooltip>
              <span>{{ config.remark }}</span>
            </div>
          </div>
          
          <div class="card-footer">
            <el-button 
              v-if="editingItem === index"
              type="primary" 
              size="small" 
              @click="saveConfig(config)"
              :loading="loading"
            >
              保存
            </el-button>
            <el-button 
              v-if="editingItem === index"
              type="default" 
              size="small" 
              @click="cancelEdit"
            >
              取消
            </el-button>
            <el-button 
              v-else
              type="primary" 
              plain
              size="small" 
              @click="startEdit(index, config)"
            >
              修改
            </el-button>
          </div>
        </div>
      </el-card>
    </div>
  </div>
</template>

<script setup name="Config">
import { listConfig, updateConfig } from "@/api/system/config";
import { InfoFilled } from '@element-plus/icons-vue';

const { proxy } = getCurrentInstance();

const configList = ref([]);
const loading = ref(false);
const editingItem = ref(null);
const tempValue = ref('');
const tempSwitchValue = ref(false);
const tempNumericValue = ref(0);

/** 获取配置列表 */
function getList() {
  loading.value = true;
  listConfig().then(response => {
    configList.value = response.rows;
    loading.value = false;
  });
}

/** 格式化时间说明 */
function formatTimeDescription(seconds) {
  const secNum = parseInt(seconds, 10);
  if (secNum < 60) return "";
  
  const minutes = Math.floor(secNum / 60);
  if (minutes < 60) {
    return `（${minutes}分钟）`;
  } else {
    const hours = Math.floor(minutes / 60);
    const remainingMinutes = minutes % 60;
    if (remainingMinutes === 0) {
      return `（${hours}小时）`;
    } else {
      return `（${hours}小时${remainingMinutes}分钟）`;
    }
  }
}

/** 开始编辑配置 */
function startEdit(index, config) {
  editingItem.value = index;
  
  if (config.configKey === 'sys.account.captchaEnabled') {
    tempSwitchValue.value = config.configValue === 'true';
  } else if (config.configKey === 'desire_complete_time' || config.configKey === 'logout_timeout') {
    tempNumericValue.value = parseInt(config.configValue);
  } else {
    tempValue.value = config.configValue;
  }
}

/** 取消编辑 */
function cancelEdit() {
  editingItem.value = null;
  tempValue.value = '';
  tempSwitchValue.value = false;
  tempNumericValue.value = 0;
}

/** 保存配置 */
function saveConfig(config) {
  let newValue;
  
  if (config.configKey === 'sys.account.captchaEnabled') {
    newValue = tempSwitchValue.value.toString();
    if (newValue === config.configValue) {
      cancelEdit();
      return;
    }
  } else if (config.configKey === 'desire_complete_time' || config.configKey === 'logout_timeout') {
    newValue = tempNumericValue.value.toString();
    if (newValue === config.configValue) {
      cancelEdit();
      return;
    }
  } else {
    newValue = tempValue.value;
    if (newValue === config.configValue || !newValue.trim()) {
      cancelEdit();
      return;
    }
  }

  loading.value = true;
  const updatedConfig = {
    configId: config.configId,
    configValue: newValue,
    configName: config.configName,
    configKey: config.configKey,
    configType: config.configType,
    remark: config.remark
  };

  updateConfig(updatedConfig).then(() => {
    proxy.notify.success("配置更新成功");
    config.configValue = newValue;
    cancelEdit();
    getList();
  }).catch(() => {
    loading.value = false;
  });
}

// 初始加载
getList();
</script>

<style lang="scss" scoped>
.config-container {
  display: flex;
  flex-direction: column;
  gap: 32px;
  max-width: 1200px;
  margin: 0 auto;
  padding: 32px;
}

.config-header {
  text-align: center;
  margin-bottom: 24px;

  h2 {
    font-size: 32px;
    font-weight: 600;
    margin-bottom: 8px;
    background: linear-gradient(45deg, #3a8ee6, #53a8ff);
    -webkit-background-clip: text;
    background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .subtitle {
    color: #606266;
    font-size: 16px;
  }
}

.config-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(380px, 1fr));
  gap: 28px;
}

.config-card {
  border-radius: 16px;
  box-shadow: 0 10px 20px rgba(0, 0, 0, 0.06);
  transition: all 0.3s ease;
  overflow: hidden;
  border: none;
  
  &:hover {
    transform: translateY(-4px);
    box-shadow: 0 14px 24px rgba(0, 0, 0, 0.1);
  }
  
  &.card-highlight {
    border: 2px solid #409EFF;
  }
}

.card-content {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 16px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  padding-bottom: 16px;
  border-bottom: 1px solid #f0f0f0;
  border-radius: 8px;
}

.card-title {
  font-size: 20px;
  font-weight: 600;
}

.card-body {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.card-value {
  margin-bottom: 12px;
  
  .value-display {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  
  .value-text {
    font-size: 32px;
    font-weight: 700;
    color: #409EFF;
    display: flex;
    align-items: baseline;
  }
  
  .value-unit {
    font-size: 18px;
    color: #606266;
    margin-left: 6px;
  }
  
  .value-description {
    font-size: 14px;
    color: #909399;
    margin-left: 8px;
  }
  
  .status-text {
    &:before {
      content: '';
      display: inline-block;
      width: 12px;
      height: 12px;
      border-radius: 50%;
      margin-right: 8px;
    }
  }
  
  .status-text:contains('已开启') {
    color: #67C23A;
    &:before {
      background-color: #67C23A;
    }
  }
  
  .status-text:contains('已关闭') {
    color: #F56C6C;
    &:before {
      background-color: #F56C6C;
    }
  }
}

.card-remark {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 16px;
  background-color: var(--el-fill-color-light);
  border-radius: 8px;
  color: var(--el-text-color-regular);
  font-size: 14px;
  
  .el-icon {
    color: #909399;
    font-size: 18px;
  }
}

.card-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 20px;
  
  .el-button {
    padding: 10px 20px;
    border-radius: 8px;
    font-weight: 500;
  }
}

.toggle-wrapper {
  display: flex;
  align-items: center;
  gap: 20px;
  margin: 12px 0;
  
  .toggle-label {
    font-size: 18px;
    font-weight: 600;
    min-width: 40px;
  }
}

.numeric-input-wrapper {
  display: flex;
  align-items: center;
  gap: 10px;
  margin: 8px 0;
  
  .input-suffix {
    font-size: 16px;
    color: #606266;
  }
}

.input-help-text {
  margin-top: 8px;
  color: #909399;
  font-size: 13px;
}

:deep(.el-input-number) {
  width: 180px;
}

:deep(.el-switch) {
  --el-switch-on-color: #13ce66;
  --el-switch-off-color: #ff4949;
}
</style>
