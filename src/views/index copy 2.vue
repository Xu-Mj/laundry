<template>
  <div class="home">
<!-- 
    <div class="top">
      <el-table :data="tableData">
        <el-table-column prop="name" label="会员标识"></el-table-column>
        <el-table-column prop="message" label="消息"></el-table-column>
        <el-table-column prop="consumeCount" label="消费次数"></el-table-column>
        <el-table-column prop="time" label="时间"></el-table-column>
        <el-table-column label="操作">
          <template #default="scope">
            <el-button type="text" size="small">回复</el-button>
            <el-button type="text" size="small">忽略</el-button>
          </template>
        </el-table-column>
      </el-table>
      <pagination v-show="total > 0" :total="total" v-model:page="queryParams.pageNum"
        v-model:limit="queryParams.pageSize" @pagination="getList" />
    </div> -->
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
import GaugeChart from '../components/GaugeChart.vue';
import { getCountList, getChartList, getOrderTotalCount, fetchPaymentSummary, fetchMonthlyPaymentSummary } from '../api/home';
import { use } from 'echarts/core';
import { CanvasRenderer } from 'echarts/renderers';
import { PieChart } from 'echarts/charts';
import { TitleComponent, TooltipComponent, LegendComponent } from 'echarts/components';
import VChart from 'vue-echarts';

// const tableData = ref();
const tableData = [
  {
    name: '用户001',
    message: '你好，我想咨询一下会员优惠。',
    consumeCount: 5,
    time: '2023-10-01 14:30',
  },
  {
    name: '用户002',
    message: '我的订单为什么还没有发货？',
    consumeCount: 12,
    time: '2023-10-02 09:15',
  },
  {
    name: '用户003',
    message: '请问有积分兑换活动吗？',
    consumeCount: 8,
    time: '2023-10-03 16:45',
  },
  {
    name: '用户004',
    message: '我的会员卡丢失了，怎么补办？',
    consumeCount: 3,
    time: '2023-10-04 11:20',
  },
  {
    name: '用户005',
    message: '最近有什么新品推荐吗？',
    consumeCount: 20,
    time: '2023-10-05 18:00',
  },
];
const selectedDate = ref(new Date());
console.log(selectedDate.value);
const total = ref(tableData.length);
const queryParams = reactive({
  pageNum: 1,
  pageSize: 10,
});

const countList = ref([]);
const orderTotalCount = ref(0);
const chart = ref();
const lineChart = ref();
const barChartOptions = ref({});
// 获取数据
async function getCountData() {
  await getCountList().then(res => {
    countList.value = res;
  });
}

async function getChartData() {
  await getChartList().then(res => {
    chart.value = res;
  });
}


use([CanvasRenderer, PieChart, TitleComponent, TooltipComponent, LegendComponent]);
const codeToLabel = {
  '01': '到店',
  '02': '线上',
  '03': '电话',
  // 其他编码和中文的映射
};
// 将后端数据转换为 ECharts 格式，并替换编码为中文
const parseData = (data) => {
  const result = [];
  for (const key in data) {
    if (Array.isArray(data[key])) {
      const chartData = {
        name: key,
        data: data[key].map(item => {
          // 如果是 source 字段，替换编码为中文
          const label = key === 'source' ? codeToLabel[item[key]] : item[key];
          return {
            value: item.count,
            name: label, // 使用中文替换编码
          };
        }),
      };
      result.push(chartData);
    }
  }
  return result;
};

// 动态生成图表配置
const generateChartOptions = (data, type) => {
  return data.map(item => ({
    title: {
      text: `${item.name} 数据分布`,
      left: 'center',
    },
    tooltip: {
      trigger: 'item',
    },
    legend: {
      orient: 'vertical', // 图例垂直排列
      bottom: 0,
      left: 'left',
    },
    series: [
      {
        name: item.name,
        type: type, // 可以根据需求改为 bar 或其他图表类型
        radius: '50%',
        data: item.data,
        label: {
          show: true, // 显示标签
          formatter: '{b}: {c}', // 格式化显示内容
        },
      },
    ],
  }));
};
const chartData = ref([]);

// 饼图配置
const pieChartOptions = ref({});
// 图表配置列表
const chartList = ref([]);

const initLineChart = async (year, month) => {
  const response = await fetchMonthlyPaymentSummary(year, month); // Adjust the year and month as needed
  const data = response;

  const dates = data.map(item => item.date);
  const amounts = data.map(item => item.totalAmount);

  const option = {
    title: {
      text: '本月收支情况',
      left: 'center',
    },
    tooltip: {
      trigger: 'axis',
    },
    xAxis: {
      type: 'category',
      data: dates, axisLabel: {
        interval: 'auto',
        formatter: (value) => {
          const date = new Date(value);
          return `${date.getDate()}`;
        },
      },
    },
    yAxis: {
      type: 'value',
    },
    series: [
      {
        name: '收支金额',
        type: 'line',
        data: amounts,
      },
    ],
  };

  lineChart.value = option;
};

const handleDateChange = (date) => {
  const year = date.getFullYear();
  const month = date.getMonth() + 1; // getMonth() returns 0-11
  initLineChart(year, month);
};


const initBarChart = async () => {
  const response = await fetchPaymentSummary();
  const data = response;

  const labels = data.map(item => item.label);
  const incomes = data.map(item => item.income);
  const expenses = data.map(item => item.expense);

  barChartOptions.value = {
    title: {
      text: '收入与支出',
      left: 'center',
    },
    tooltip: {
      trigger: 'axis',
      axisPointer: {
        type: 'shadow',
      },
    },
    legend: {
      data: ['收入', '支出'],
      bottom: 0,
    },
    xAxis: {
      type: 'category',
      data: labels,
    },
    yAxis: {
      type: 'value',
    },
    series: [
      {
        name: '收入',
        type: 'bar',
        data: incomes,
        itemStyle: {
          color: '#4caf50',
        },
      },
      {
        name: '支出',
        type: 'bar',
        data: expenses,
        itemStyle: {
          color: '#f44336',
        },
      },
    ],
  };
};

// 初始化数据
onMounted(async () => {
  await getCountData();
  await getChartData();
  orderTotalCount.value = await getOrderTotalCount();
  chartData.value = countList.value.map(item => ({
    value: item.count,
    name: item.title,
  }));
  // countList.value = countList.value.map(item => ({
  //   value: item.count,
  //   name: item.title,
  // }));
  const parsedData = parseData(chart.value);
  chartList.value = generateChartOptions(parsedData, 'pie');
  const year = selectedDate.value.getFullYear();
  const month = selectedDate.value.getMonth() + 1;
  await initLineChart(year, month);
  await initBarChart();
  pieChartOptions.value = {
    title: {
      text: '订单状态占比图',
      left: 'center',
    },
    tooltip: {
      trigger: 'item',
      formatter: '{a} <br/>{b}: {c} ({d}%)', // 显示名称、值和百分比
    },
    legend: {
      orient: 'vertical', // 图例垂直排列
      bottom: 0,
      left: 'left',
    },
    series: [
      {
        name: '订单状态',
        type: 'pie',
        radius: ['40%', '60%'], // 饼图半径
        data: chartData.value,
        label: {
          show: true, // 显示标签
          formatter: '{b}: {c} ({d}%)', // 标签内容格式
        },
        emphasis: {
          itemStyle: {
            shadowBlur: 10,
            shadowOffsetX: 0,
            shadowColor: 'rgba(0, 0, 0, 0.5)',
          },
        },
      },
    ],
  };

  console.log(await fetchMonthlyPaymentSummary(2024, 12))
});
</script>

<style scoped lang="scss">
.home {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 2rem;
  // background-color: #efeeee;
}

.top {
  max-height: 50%;
}

// .wrapper {
//   height: 0;
//   padding-bottom: 100%;
//   position: relative;
// }

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
  // padding-top: 1rem;
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