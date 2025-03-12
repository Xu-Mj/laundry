<template>
  <div class="home">
    <div class="statistic">
      <el-row :gutter="16">
        <el-col :span="6">
          <div class="statistic-card">
            <el-statistic :value="paymentData.today?.income || 0">
              <template #title>
                <div style="display: inline-flex; align-items: center">
                  收入（本日）
                  <el-tooltip effect="dark" content="今日收入总额" placement="top">
                    <el-icon style="margin-left: 4px" :size="12">
                      <Warning />
                    </el-icon>
                  </el-tooltip>
                </div>
              </template>
            </el-statistic>
            <div class="statistic-footer">
              <div class="footer-item">
                <span>较昨日</span>
                <span :class="paymentData.today?.incomeRate >= 0 ? 'green' : 'red'">
                  {{ Math.abs(paymentData.today?.incomeRate || 0) }}%
                  <el-icon>
                    <component :is="paymentData.today?.incomeRate >= 0 ? 'CaretTop' : 'CaretBottom'" />
                  </el-icon>
                </span>
              </div>
            </div>
          </div>
        </el-col>
        <el-col :span="6">
          <div class="statistic-card">
            <el-statistic :value="paymentData.week?.income || 0">
              <template #title>
                <div style="display: inline-flex; align-items: center">
                  收入（本周）
                  <el-tooltip effect="dark" content="本周收入总额" placement="top">
                    <el-icon style="margin-left: 4px" :size="12">
                      <Warning />
                    </el-icon>
                  </el-tooltip>
                </div>
              </template>
            </el-statistic>
            <div class="statistic-footer">
              <div class="footer-item">
                <span>较上周</span>
                <span :class="paymentData.week?.incomeRate >= 0 ? 'green' : 'red'">
                  {{ Math.abs(paymentData.week?.incomeRate || 0) }}%
                  <el-icon>
                    <component :is="paymentData.week?.incomeRate >= 0 ? 'CaretTop' : 'CaretBottom'" />
                  </el-icon>
                </span>
              </div>
            </div>
          </div>
        </el-col>
        <el-col :span="6">
          <div class="statistic-card">
            <el-statistic :value="paymentData.today?.expense || 0">
              <template #title>
                <div style="display: inline-flex; align-items: center">
                  本日支出
                  <el-tooltip effect="dark" content="今日支出总额" placement="top">
                    <el-icon style="margin-left: 4px" :size="12">
                      <Warning />
                    </el-icon>
                  </el-tooltip>
                </div>
              </template>
            </el-statistic>
            <div class="statistic-footer">
              <div class="footer-item">
                <span>较昨日</span>
                <span :class="paymentData.today?.expenseRate >= 0 ? 'red' : 'green'">
                  {{ Math.abs(paymentData.today?.expenseRate || 0) }}%
                  <el-icon>
                    <component :is="paymentData.today?.expenseRate >= 0 ? 'CaretTop' : 'CaretBottom'" />
                  </el-icon>
                </span>
              </div>
            </div>
          </div>
        </el-col>
        <el-col :span="6">
          <div class="statistic-card">
            <el-statistic :value="paymentData.week?.expense || 0">
              <template #title>
                <div style="display: inline-flex; align-items: center">
                  本周支出
                </div>
              </template>
            </el-statistic>
            <div class="statistic-footer">
              <div class="footer-item">
                <span>较上周</span>
                <span :class="paymentData.week?.expenseRate >= 0 ? 'red' : 'green'">
                  {{ Math.abs(paymentData.week?.expenseRate || 0) }}%
                  <el-icon>
                    <component :is="paymentData.week?.expenseRate >= 0 ? 'CaretTop' : 'CaretBottom'" />
                  </el-icon>
                </span>
              </div>
            </div>
          </div>
        </el-col>
      </el-row>
    </div>
    <div class="bottom">
      <GaugeChart :total="orderTotalCount" :value="item.count" :title="item.title" v-for="item in countList"
        :key="item.title" class="chart" />
      <div class="line-chart-wrapper">
        <el-date-picker class="date-picker" v-model="selectedDate" type="month" placeholder="选择年月"
          @change="handleDateChange" />
        <v-chart class="chart" ref="lineChartRef" :option="lineChart" />
      </div>
      <v-chart v-for="(item, index) in chartList" :key="index" class="chart" :option="item"
        :ref="(el) => setChartRef(el, index)" />
    </div>
  </div>
</template>
<script setup name="Index">
import ThemeSwitch from '../components/ThemeSwitch.vue';
import VChart from 'vue-echarts';
import { use } from 'echarts/core';
import { PieChart } from 'echarts/charts';
import { useWindowSize } from '@vueuse/core';
import { getDicts } from '../api/system/dict/data';
import { CanvasRenderer } from 'echarts/renderers';
import GaugeChart from '../components/GaugeChart.vue';
import { ref, onMounted, nextTick, watch, onUpdated } from 'vue';
import { TitleComponent, TooltipComponent, LegendComponent } from 'echarts/components';
import { getCountList, getChartList, getOrderTotalCount, fetchPaymentSummary, fetchMonthlyPaymentSummary } from '../api/home';

use([CanvasRenderer, PieChart, TitleComponent, TooltipComponent, LegendComponent]);

const selectedDate = ref(new Date());
const countList = ref([]);
const orderTotalCount = ref(0);
const chart = ref();
const lineChart = ref({});
const lineChartRef = ref();
const paymentData = ref({});
const chartData = ref([]);
const pieChartOptions = ref({});
const chartList = ref([]);
const chartListRefs = ref([]);
const codeToLabel = ref({});
const { width, height } = useWindowSize();

function setChartRef(el, index) {
  if (el) {
    chartListRefs.value[index] = el;
  }
};

const fetchCodeToLabelData = async () => {
  const res = await getDicts('sys_price_order_type');
  codeToLabel.value = res.reduce((acc, item) => {
    acc[item.dictValue] = item.dictLabel;
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
  return data.map(item => ({
    title: { text: `订单来源数据分布`, left: 'center' },
    tooltip: { trigger: 'item' },
    legend: { orient: 'vertical', bottom: 0, left: 'left' },
    series: [{
      name: item.name,
      type: type,
      radius: '50%',
      data: item.data,
      label: { show: true, formatter: '{b}: {c}' },
    }],
  }));
};

const getCountData = async () => {
  const res = await getCountList();
  countList.value = res;
};

const getChartData = async () => {
  const res = await getChartList();
  chart.value = res;
};

const initLineChart = async (year, month) => {
  const data = await fetchMonthlyPaymentSummary(year, month);
  const dates = data.map(item => item.date);
  const amounts = data.map(item => item.totalAmount);

  lineChart.value = {
    title: { text: '本月收支情况', left: 'center' },
    tooltip: { trigger: 'axis', formatter: '{a} <br/>{b}: {c} 元' },
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
    series: [{ name: '收支金额', type: 'line', data: amounts }],
  };
};

const handleDateChange = (date) => {
  const year = date.getFullYear();
  const month = date.getMonth() + 1;
  initLineChart(year, month);
};

const resizeCharts = () => {
  if (lineChartRef.value && lineChartRef.value.resize) {
    lineChartRef.value.resize();
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
  await getChartData();
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
  chartData.value = countList.value.map(item => ({ value: item.count, name: item.title }));
  const parsedData = parseData(chart.value);
  chartList.value = generateChartOptions(parsedData, 'pie');
  const year = selectedDate.value.getFullYear();
  const month = selectedDate.value.getMonth() + 1;
  await initLineChart(year, month);
  pieChartOptions.value = {
    title: { text: '订单状态占比图', left: 'center' },
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

  nextTick(() => {
    resizeCharts();
  });
};

onMounted(async () => {
  await initCharts();
});

watch([width, height], () => {
  resizeCharts();
});
</script>

<style scoped lang="scss">
.home {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 2rem;
}

.header {
  position: absolute;
  top: 1rem;
  right: 1rem;
  z-index: 1;
}

.statistic{
  width: 100%;
}

.el-statistic {
  --el-statistic-content-font-size: 28px;
}

.statistic-card {
  height: 100%;
  padding: 20px;
  border-radius: 4px;
  background-color: var(--statistic-card-bg-color);
}

.statistic-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  font-size: 12px;
  color: var(--el-text-color-regular);
  margin-top: 16px;
}

.statistic-footer .footer-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.statistic-footer .footer-item span:last-child {
  display: inline-flex;
  align-items: center;
  margin-left: 4px;
}

.green {
  color: var(--el-color-success);
}
.red {
  color: var(--el-color-error);
}
.bottom {
  width: 100%;
  height: 100%;
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  justify-content: center;
  align-items: center;
  gap: 2rem;
}

.line-chart-wrapper {
  position: relative;
  width: 100%;
}

.chart {
  width: 100%;
  height: 400px;
  //border: 1px solid #ccc;
  border-radius: .3rem;
}
</style>

<style>
.date-picker {
  position: absolute;
  width: 7rem !important;
  top: 0;
  right: 0;
  z-index: 1;
}
</style>