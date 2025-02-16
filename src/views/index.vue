<template>
  <div class="home">
    <div class="bottom">
      <GaugeChart :total="orderTotalCount" :value="item.count" :title="item.title" v-for="item in countList"
        :key="item.title" class="chart" />
      <GaugeChart :total="300" :value="156" title="系统消息" class="chart" />
      <div class="line-chart-wrapper">
        <el-date-picker class="date-picker" v-model="selectedDate" type="month" placeholder="选择年月"
          @change="handleDateChange" />
        <v-chart class="chart" :option="lineChart" />
      </div>
      <v-chart v-for="item in chartList" class="chart" :option="item" />
      <v-chart class="chart" :option="barChartOptions" />
    </div>
  </div>
</template>

<script setup name="Index">
import { ref, reactive, onMounted } from 'vue';
import { use } from 'echarts/core';
import { CanvasRenderer } from 'echarts/renderers';
import { PieChart } from 'echarts/charts';
import { TitleComponent, TooltipComponent, LegendComponent } from 'echarts/components';
import GaugeChart from '../components/GaugeChart.vue';
import VChart from 'vue-echarts';
import { getCountList, getChartList, getOrderTotalCount, fetchPaymentSummary, fetchMonthlyPaymentSummary } from '../api/home';

use([CanvasRenderer, PieChart, TitleComponent, TooltipComponent, LegendComponent]);

const selectedDate = ref(new Date());
const total = ref(0);
const queryParams = reactive({
  pageNum: 1,
  pageSize: 10,
});
const countList = ref([]);
const orderTotalCount = ref(0);
const chart = ref();
const lineChart = ref();
const barChartOptions = ref({});
const chartData = ref([]);
const pieChartOptions = ref({});
const chartList = ref([]);

const codeToLabel = {
  '01': '到店',
  '02': '线上',
  '03': '电话',
};

const parseData = (data) => {
  return Object.keys(data).map(key => ({
    name: key,
    data: data[key].map(item => ({
      value: item.count,
      name: key === 'source' ? codeToLabel[item[key]] : item[key],
    })),
  }));
};

const generateChartOptions = (data, type) => {
  return data.map(item => ({
    title: { text: `${item.name} 数据分布`, left: 'center' },
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

const initBarChart = async () => {
  const data = await fetchPaymentSummary();
  const labels = data.map(item => item.label);
  const incomes = data.map(item => item.income);
  const expenses = data.map(item => item.expense);

  barChartOptions.value = {
    title: { text: '收入与支出', left: 'center' },
    tooltip: { trigger: 'axis', axisPointer: { type: 'shadow' }, formatter: '{a} <br/>{b}: {c} 元' },
    legend: { data: ['收入', '支出'], bottom: 0 },
    xAxis: { type: 'category', data: labels },
    yAxis: {
      type: 'value',
      name: '金额 (元)',
    },
    series: [
      { name: '收入', type: 'bar', data: incomes, itemStyle: { color: '#4caf50' } },
      { name: '支出', type: 'bar', data: expenses, itemStyle: { color: '#f44336' } },
    ],
  };
};

onMounted(async () => {
  await getCountData();
  await getChartData();
  orderTotalCount.value = await getOrderTotalCount();
  chartData.value = countList.value.map(item => ({ value: item.count, name: item.title }));
  const parsedData = parseData(chart.value);
  chartList.value = generateChartOptions(parsedData, 'pie');
  const year = selectedDate.value.getFullYear();
  const month = selectedDate.value.getMonth() + 1;
  await initLineChart(year, month);
  await initBarChart();
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
});
</script>

<style scoped lang="scss">
.home {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.bottom {
  width: 100%;
  height: 100%;
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: 2rem;
}

.line-chart-wrapper {
  position: relative;
}

.chart {
  width: 100%;
  height: 400px;
  border: 1px solid #ccc;
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