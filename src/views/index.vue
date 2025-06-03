<template>
  <div class="dashboard">
    <!-- 顶部统计卡片区域 -->
    <div class="dashboard-header">
      <h2 class="dashboard-title">数据概览</h2>
      <div class="dashboard-date">{{ formatDate(new Date()) }}</div>
    </div>

    <!-- 数据卡片区域 -->
    <div class="card-section">
      <el-row :gutter="20">
        <el-col :xs="24" :sm="12" :md="6" :lg="6" v-for="(card, index) in statisticCards" :key="index">
          <div class="data-card" :class="`data-card-${index + 1}`">
            <div class="card-icon">
              <el-icon>
                <component :is="card.icon" />
              </el-icon>
            </div>
            <div class="card-content">
              <div class="card-title">{{ card.title }}</div>
              <div class="card-value">{{ card.value }}</div>
              <div class="card-trend" :class="card.trendClass">
                <span>{{ card.trend }}</span>
                <el-icon>
                  <component :is="card.trendIcon" />
                </el-icon>
              </div>
            </div>
          </div>
        </el-col>
      </el-row>
    </div>

    <!-- 图表区域 -->
    <div class="chart-section">
      <el-row :gutter="20">
        <!-- 左侧图表 -->
        <el-col :xs="24" :sm="24" :md="16" :lg="16">
          <div class="chart-card">
            <div class="chart-header">
              <h3>本月收支情况</h3>
              <el-date-picker class="date-picker" v-model="selectedDate" type="month" placeholder="选择年月"
                @change="handleDateChange" size="small" />
            </div>
            <v-chart class="chart" ref="lineChartRef" :option="lineChart" />
          </div>
        </el-col>

        <!-- 右侧图表 -->
        <el-col :xs="24" :sm="24" :md="8" :lg="8">
          <div class="chart-card">
            <div class="chart-header">
              <h3>订单状态占比</h3>
            </div>
            <v-chart class="chart" ref="cycleChartRef" :option="pieChartOptions" />
          </div>
        </el-col>
      </el-row>

      <!-- 底部图表 -->
      <el-row :gutter="20" class="bottom-charts">
        <el-col :xs="24" :sm="12" :md="6" :lg="6" v-for="(item, index) in countList" :key="item.title">
          <div class="chart-card gauge-card">
            <div class="chart-header">
              <h3>{{ item.title }}</h3>
            </div>
            <GaugeChart :total="orderTotalCount" :value="item.count"
              :color="['primary', 'success', 'warning', 'danger'][index % 4]" :showPercentage="true"
              class="gauge-chart" />
          </div>
        </el-col>
        <el-col :xs="24" :sm="12" :md="8" :lg="8" v-for="(item, index) in chartList" :key="index">
          <div class="chart-card">
            <div class="chart-header">
              <h3>订单来源分布</h3>
              <el-date-picker class="date-picker" v-model="selectedSourceDate" type="month" placeholder="选择年月"
                @change="handleSourceDateChange" size="small" />
            </div>
            <v-chart class="chart" :option="item" :ref="(el) => setChartRef(el, index)" />
          </div>
        </el-col>
      </el-row>

      <!-- 第二行底部图表 -->
      <!-- <el-row :gutter="20" class="bottom-charts second-row">
        <el-col :xs="24" :sm="12" :md="8" :lg="8" v-for="(item, index) in chartList" :key="index">
          <div class="chart-card">
            <div class="chart-header">
              <h3>订单来源分布</h3>
            </div>
            <v-chart class="chart" :option="item" :ref="(el) => setChartRef(el, index)" />
          </div>
        </el-col>
      </el-row> -->
    </div>

    <!-- 广告展示位 -->
    <div class="ad-container" v-show="showAD">
      <div class="ad-content">
        <div class="ad-header">
          <h4>推广活动</h4>
          <el-icon class="close-icon" @click="showAD = false">
            <Close />
          </el-icon>
        </div>
        <div class="ad-body">
          <img src="@/assets/images/ad-placeholder.svg" alt="广告" />
          <div class="ad-text">最新优惠活动，立即查看</div>
          <el-button type="primary" size="small" class="ad-button">了解详情</el-button>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup name="Index">
import VChart from 'vue-echarts';
import { use } from 'echarts/core';
import { PieChart, LineChart } from 'echarts/charts';
import { Close } from '@element-plus/icons-vue';
import { useWindowSize } from '@vueuse/core';
import { CanvasRenderer } from 'echarts/renderers';
import GaugeChart from '@/components/GaugeChart.vue';
import { ref, onMounted, nextTick, watch, computed } from 'vue';
import { TitleComponent, TooltipComponent, LegendComponent, GridComponent } from 'echarts/components';
import { getCountList, getChartList, getOrderTotalCount, fetchPaymentSummary, fetchMonthlyPaymentSummary } from '../api/home';
import { OrderSource } from '@/constants';

use([CanvasRenderer, PieChart, LineChart, TitleComponent, TooltipComponent, LegendComponent, GridComponent]);

const selectedDate = ref(new Date());
const selectedSourceDate = ref(new Date());
const countList = ref([]);
const orderTotalCount = ref(0);
const chart = ref();
const lineChart = ref({});
const lineChartRef = ref();
const cycleChartRef = ref();
const paymentData = ref({});
const chartData = ref([]);
const pieChartOptions = ref({});
const chartList = ref([]);
const chartListRefs = ref([]);
const codeToLabel = ref({});
const { width, height } = useWindowSize();

const showAD = ref(true);

// 格式化日期显示
const formatDate = (date) => {
  const year = date.getFullYear();
  const month = date.getMonth() + 1;
  const day = date.getDate();
  return `${year}年${month}月${day}日`;
};

// 格式化货币显示
const formatCurrency = (value) => {
  return new Intl.NumberFormat('zh-CN', {
    style: 'currency',
    currency: 'CNY',
    minimumFractionDigits: 2
  }).format(value);
};

// 统计卡片数据
const statisticCards = computed(() => [
  {
    title: '本日收入',
    value: formatCurrency((paymentData.value.today?.income || 0)),
    trend: `较昨日 ${Math.abs(paymentData.value.today?.incomeRate || 0)}%`,
    trendClass: paymentData.value.today?.incomeRate >= 0 ? 'trend-up' : 'trend-down',
    trendIcon: paymentData.value.today?.incomeRate >= 0 ? 'CaretTop' : 'CaretBottom',
    icon: 'Money'
  },
  {
    title: '本周收入',
    value: formatCurrency((paymentData.value.week?.income || 0)),
    trend: `较上周 ${Math.abs(paymentData.value.week?.incomeRate || 0)}%`,
    trendClass: paymentData.value.week?.incomeRate >= 0 ? 'trend-up' : 'trend-down',
    trendIcon: paymentData.value.week?.incomeRate >= 0 ? 'CaretTop' : 'CaretBottom',
    icon: 'TrendCharts'
  },
  {
    title: '本日支出',
    value: formatCurrency((paymentData.value.today?.expense || 0) / 100),
    trend: `较昨日 ${Math.abs(paymentData.value.today?.expenseRate || 0)}%`,
    trendClass: paymentData.value.today?.expenseRate >= 0 ? 'trend-down' : 'trend-up',
    trendIcon: paymentData.value.today?.expenseRate >= 0 ? 'CaretTop' : 'CaretBottom',
    icon: 'Wallet'
  },
  {
    title: '本周支出',
    value: formatCurrency((paymentData.value.week?.expense || 0) / 100),
    trend: `较上周 ${Math.abs(paymentData.value.week?.expenseRate || 0)}%`,
    trendClass: paymentData.value.week?.expenseRate >= 0 ? 'trend-down' : 'trend-up',
    trendIcon: paymentData.value.week?.expenseRate >= 0 ? 'CaretTop' : 'CaretBottom',
    icon: 'Postcard'
  }
]);

function setChartRef(el, index) {
  if (el) {
    chartListRefs.value[index] = el;
  }
};

const fetchCodeToLabelData = async () => {
  codeToLabel.value = OrderSource.reduce((acc, item) => {
    acc[item.value] = item.label;
    return acc;
  }, {});
};

const parseData = (data) => {
  return Object.keys(data).map(key => ({
    name: key,
    data: data[key].map(item => ({
      value: item.count,
      name: key === 'source' ? codeToLabel.value[item[key]] : item[key],
    })),
  }));
};

const generateChartOptions = (data, type) => {
  return data.map(item => {
    // 检查数据是否为空
    const isEmpty = !item.data || item.data.length === 0;
    
    return {
      tooltip: { trigger: 'item', formatter: '{a} <br/>{b}: {c} ({d}%)' },
      legend: { orient: 'vertical', bottom: 0, left: 'left' },
      series: [{
        name: item.name,
        type: type,
        radius: ['40%', '60%'],
        data: isEmpty ? [{ value: 1, name: '暂无数据', itemStyle: { color: '#f5f7fa' } }] : item.data,
        label: { 
          show: true, 
          formatter: isEmpty ? '暂无数据' : '{b}: {c} ({d}%)',
          position: isEmpty ? 'center' : 'outside',
          fontSize: isEmpty ? 14 : 12,
          color: isEmpty ? '#909399' : 'inherit'
        },
        emphasis: {
          disabled: isEmpty,
          itemStyle: {
            shadowBlur: 10,
            shadowOffsetX: 0,
            shadowColor: 'rgba(0, 0, 0, 0.5)'
          }
        }
      }],
    };
  });
};

const getCountData = async () => {
  const res = await getCountList();
  countList.value = res;
};

const getChartData = async () => {
  const year = selectedSourceDate.value.getFullYear();
  const month = selectedSourceDate.value.getMonth() + 1;
  const res = await getChartList({ year, month });
  chart.value = res;
};

const initLineChart = async (year, month) => {
  const data = await fetchMonthlyPaymentSummary(year, month);

  // 找到第一个收入或支出不为零的数据点索引
  let firstNonZeroIndex = 0;
  for (let i = 0; i < data.length; i++) {
    if (data[i].income > 0 || data[i].expense > 0) {
      firstNonZeroIndex = i;
      break;
    }
  }

  // 检查是否为当前月份，如果是则只显示到当前日期的数据
  let filteredData = data.slice(firstNonZeroIndex);
  const currentDate = new Date();
  const currentYear = currentDate.getFullYear();
  const currentMonth = currentDate.getMonth() + 1;

  // 如果是当前月份，只显示到当前日期
  if (year === currentYear && month === currentMonth) {
    const currentDay = currentDate.getDate();
    filteredData = filteredData.filter(item => {
      const itemDate = new Date(item.date);
      return itemDate.getDate() <= currentDay;
    });
  }

  const dates = filteredData.map(item => item.date);
  const incomes = filteredData.map(item => item.income);
  // 将分转换为元
  const expenses = filteredData.map(item => item.expense / 100);

  lineChart.value = {
    title: { text: '本月收支情况', left: 'center' },
    tooltip: {
      trigger: 'axis',
      formatter: function (params) {
        let result = params[0].name + '<br/>';
        params.forEach(param => {
          result += param.seriesName + ': ' + formatCurrency(param.value) + '<br/>';
        });
        return result;
      }
    },
    legend: {
      data: ['收入', '支出'],
      bottom: 0
    },
    xAxis: {
      type: 'category',
      data: dates,
      axisLabel: {
        interval: 'auto',
        formatter: value => new Date(value).getDate(),
      },
    },
    yAxis: {
      type: 'value',
      name: '金额 (元)',
    },
    series: [
      { name: '收入', type: 'line', data: incomes, itemStyle: { color: '#67C23A' } },
      { name: '支出', type: 'line', data: expenses, itemStyle: { color: '#F56C6C' } }
    ],
  };
};

const handleDateChange = (date) => {
  const year = date.getFullYear();
  const month = date.getMonth() + 1;
  initLineChart(year, month);
};

const handleSourceDateChange = (date) => {
  initSourceCharts();
};

const resizeCharts = () => {
  if (lineChartRef.value && lineChartRef.value.resize) {
    lineChartRef.value.resize();
  } if (cycleChartRef.value && cycleChartRef.value.resize) {
    cycleChartRef.value.resize();
  }
  chartListRefs.value.forEach(chart => {
    if (chart.resize) {
      chart.resize();
    }
  });
};

const initCharts = async () => {
  await fetchCodeToLabelData();
  await getCountData();
  const data = await fetchPaymentSummary();
  paymentData.value = data.reduce((acc, item) => {
    acc[item.label.toLowerCase()] = {
      income: item.income,
      expense: item.expense,
      incomeRate: item.incomeRate,
      expenseRate: item.expenseRate
    };
    return acc;
  }, {});

  orderTotalCount.value = await getOrderTotalCount();
  const year = selectedDate.value.getFullYear();
  const month = selectedDate.value.getMonth() + 1;
  await initLineChart(year, month);
  await initSourceCharts();
  nextTick(() => {
    resizeCharts();
  });
};

async function initSourceCharts() {
  await getChartData();

  chartData.value = countList.value.map(item => ({ value: item.count, name: item.title }));
  const parsedData = parseData(chart.value);
  chartList.value = generateChartOptions(parsedData, 'pie');

  pieChartOptions.value = {
    tooltip: { trigger: 'item', formatter: '{a} <br/>{b}: {c} ({d}%)' },
    legend: { orient: 'vertical', bottom: 0, left: 'left' },
    series: [{
      name: '订单状态',
      type: 'pie',
      radius: ['40%', '60%'],
      data: chartData.value,
      label: { show: true, formatter: '{b}: {c} ({d}%)' },
      emphasis: { itemStyle: { shadowBlur: 10, shadowOffsetX: 0, shadowColor: 'rgba(0, 0, 0, 0.5)' } },
    }],
  };
}

onMounted(async () => {
  await initCharts();
});

watch([width, height], () => {
  resizeCharts();
});
</script>

<style scoped lang="scss">
.dashboard {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1.5rem;
  background-color: var(--el-bg-color-page, #f5f7fa);
  overflow-y: auto;

  // 设置隐藏滚动条
  &::-webkit-scrollbar {
    display: none;
  }
}

/* 顶部标题区域 */
.dashboard-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
}

.dashboard-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--el-text-color-primary);
  margin: 0;
}

.dashboard-date {
  font-size: 0.9rem;
  color: var(--el-text-color-primary);
}

/* 数据卡片区域 */
.card-section {
  margin-bottom: 1.5rem;
}

.data-card {
  height: 100%;
  padding: 1.25rem;
  border-radius: 8px;
  background-color: var(--el-bg-color);
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.05);
  display: flex;
  transition: all 0.3s ease;
  overflow: hidden;
  position: relative;

  &:hover {
    transform: translateY(-5px);
    box-shadow: var(--el-box-shadow-light);
  }

  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 4px;
    height: 100%;
    background: linear-gradient(to bottom, var(--el-color-primary), var(--el-color-primary-light-5));
  }
}

.data-card-1::before {
  background: linear-gradient(to bottom, #67C23A, #95D475);
}

.data-card-2::before {
  background: linear-gradient(to bottom, #409EFF, #79BBFF);
}

.data-card-3::before {
  background: linear-gradient(to bottom, #F56C6C, #F89898);
}

.data-card-4::before {
  background: linear-gradient(to bottom, #E6A23C, #EEBE77);
}

.card-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  border-radius: 8px;
  margin-right: 1rem;
  background-color: var(--el-color-primary-light-9);
  color: var(--el-color-primary);
  font-size: 1.5rem;
}

.data-card-1 .card-icon {
  background-color: rgba(103, 194, 58, 0.1);
  color: #67C23A;
}

.data-card-2 .card-icon {
  background-color: rgba(64, 158, 255, 0.1);
  color: #409EFF;
}

.data-card-3 .card-icon {
  background-color: rgba(245, 108, 108, 0.1);
  color: #F56C6C;
}

.data-card-4 .card-icon {
  background-color: rgba(230, 162, 60, 0.1);
  color: #E6A23C;
}

.card-content {
  flex: 1;
}

.card-title {
  font-size: 0.9rem;
  color: var(--el-text-color-secondary);
  margin-bottom: 0.5rem;
}

.card-value {
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--el-text-color-primary);
  margin-bottom: 0.5rem;
}

.card-trend {
  font-size: 0.8rem;
  display: flex;
  align-items: center;
}

.trend-up {
  color: var(--el-color-success);
}

.trend-down {
  color: var(--el-color-danger);
}

/* 图表区域 */
.chart-section {
  flex: 1;
}

.chart-card {
  height: 100%;
  background-color: var(--el-bg-color);
  border-radius: 8px;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.05);
  padding: 1rem;
  margin-bottom: 1.5rem;
  position: relative;
  transition: all 0.3s;

  &:hover {
    box-shadow: var(--el-box-shadow-light);
    transform: translateY(-5px);
  }
}

.chart-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;

  h3 {
    font-size: 1rem;
    font-weight: 600;
    margin: 0;
    color: var(--el-text-color-primary);
  }
}

.chart {
  width: 100%;
  height: 300px;
}

.gauge-chart {
  height: 250px;
}

.bottom-charts {
  margin-top: 1rem;
}

.bottom-charts.second-row {
  margin-top: 1.5rem;
}

/* 广告展示位 */
.ad-container {
  position: fixed;
  bottom: 20px;
  right: 20px;
  z-index: 1000;
  width: 250px;
  max-width: 100%;
}

.ad-content {
  background-color: var(--el-bg-color);
  border-radius: 8px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  transition: all 0.3s ease;
  border: 1px solid var(--el-border-color-light);
}

.ad-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background-color: var(--el-color-primary-light-9);
  border-bottom: 1px solid var(--el-border-color-light);

  h4 {
    margin: 0;
    font-size: 0.9rem;
    color: var(--el-color-primary);
  }

  .close-icon {
    cursor: pointer;
    color: var(--el-text-color-secondary);
    font-size: 0.9rem;

    &:hover {
      color: var(--el-color-danger);
    }
  }
}

.ad-body {
  padding: 12px;
  display: flex;
  flex-direction: column;
  align-items: center;

  img {
    width: 100%;
    border-radius: 4px;
    margin-bottom: 10px;
  }

  .ad-text {
    font-size: 0.9rem;
    color: var(--el-text-color-primary);
    margin-bottom: 10px;
    text-align: center;
  }

  .ad-button {
    width: 100%;
  }
}

/* 响应式调整 */
@media (max-width: 768px) {
  .dashboard {
    padding: 1rem;
    gap: 1rem;
  }

  .el-row {
    gap: 1rem !important;
  }

  .chart {
    height: 250px;
  }

  .gauge-chart {
    height: 200px;
  }

  .ad-container {
    width: 200px;
    bottom: 10px;
    right: 10px;
  }
}
</style>

<style>
.date-picker {
  width: 120px !important;
}

/* 全局图表样式优化 */
.echarts-tooltip-dilliver {
  margin: 5px 0;
}

.echarts-tooltip-item-marker {
  border-radius: 50%;
}
</style>