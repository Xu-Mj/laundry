<template>
  <div class="gauge-chart-container">
    <!-- 标题 -->
    <div v-if="title" class="gauge-title">{{ title }}</div>
    <!-- 图表容器 -->
    <div ref="chart" class="chart-container"></div>
    <!-- 数值和百分比显示 -->
    <div class="data-display">
      <!-- 实际数值显示（主要信息） -->
      <div class="value-display">
        <span class="actual-value">{{ value }}</span>
        <span class="value-label" v-if="valueLabel">{{ valueLabel }}</span>
      </div>
      <!-- 百分比显示（次要信息） -->
      <div class="percentage-display" v-if="showPercentage">
        <span class="percentage-value">{{ percentageValue === 0 ? '' : percentageValue + '%' }}</span>
        <span class="percentage-label" v-if="percentageLabel">{{ percentageLabel }}</span>
      </div>
    </div>
  </div>
</template>

<script>
import * as echarts from 'echarts';

export default {
  props: {
    total: {
      type: Number,
      required: true
    },
    value: {
      type: Number,
      required: true
    },
    title: {
      type: String,
      default: ''
    },
    showPercentage: {
      type: Boolean,
      default: false
    },
    percentageLabel: {
      type: String,
      default: ''
    },
    valueLabel: {
      type: String,
      default: ''
    },
    color: {
      type: String,
      default: 'primary' // primary, success, warning, danger, info
    }
  },
  data() {
    return {
      chart: null,
      option: {
        series: [{
          type: 'gauge',
          startAngle: 90,
          endAngle: -270,
          radius: '90%',
          pointer: {
            show: false
          },
          progress: {
            show: true,
            overlap: false,
            roundCap: true,
            clip: false,
            itemStyle: {
              borderWidth: 0,
              shadowBlur: 10,
              shadowColor: 'rgba(0, 0, 0, 0.1)'
            }
          },
          axisLine: {
            lineStyle: {
              width: 12,
              color: [
                [0, 'rgba(238, 238, 238, 0.3)'] // 背景色
              ]
            }
          },
          splitLine: {
            show: false
          },
          axisTick: {
            show: false
          },
          axisLabel: {
            show: false
          },
          data: [{
            value: 0,
            name: '',
            title: {
              show: false
            },
            detail: {
              show: false
            }
          }],
          title: {
            show: false
          },
          detail: {
            show: false
          },
          animationDuration: 1500,
          animationEasing: 'cubicInOut'
        }]
      }
    };
  },
  computed: {
    percentageValue() {
      if (this.total === 0) return 0;
      return Math.round((this.value / this.total) * 100);
    },
    colorMap() {
      return {
        primary: {
          main: 'rgba(64, 158, 255, 0.9)',
          light: 'rgba(64, 158, 255, 0.1)'
        },
        success: {
          main: 'rgba(103, 194, 58, 0.9)',
          light: 'rgba(103, 194, 58, 0.1)'
        },
        warning: {
          main: 'rgba(230, 162, 60, 0.9)',
          light: 'rgba(230, 162, 60, 0.1)'
        },
        danger: {
          main: 'rgba(245, 108, 108, 0.9)',
          light: 'rgba(245, 108, 108, 0.1)'
        },
        info: {
          main: 'rgba(144, 147, 153, 0.9)',
          light: 'rgba(144, 147, 153, 0.1)'
        }
      };
    },
    themeColor() {
      return this.colorMap[this.color] || this.colorMap.primary;
    }
  },
  watch: {
    value: {
      handler(newVal) {
        this.updateChart();
      }
    },
    total: {
      handler() {
        this.updateChart();
      }
    },
    color: {
      handler() {
        this.updateChart();
      }
    }
  },
  mounted() {
    this.initChart();
    window.addEventListener('resize', this.resizeChart);
  },
  beforeUnmount() {
    window.removeEventListener('resize', this.resizeChart);
    if (this.chart) {
      this.chart.dispose();
      this.chart = null;
    }
  },
  methods: {
    initChart() {
      this.chart = echarts.init(this.$refs.chart);
      this.updateChart();
    },
    updateChart() {
      if (!this.chart) return;

      // 计算百分比值
      const percentage = (this.value / this.total) * 100;

      // 更新图表配置
      this.option.series[0].data[0].value = percentage;
      this.option.series[0].progress.itemStyle.color = this.themeColor.main;
      this.option.series[0].axisLine.lineStyle.color = [
        [percentage / 100, this.themeColor.main],
        [1, 'rgba(238, 238, 238, 0.3)']
      ];

      // 应用更新
      this.chart.setOption(this.option);
    },
    resizeChart() {
      if (this.chart) {
        this.chart.resize();
      }
    }
  }
};
</script>

<style scoped>
.gauge-chart-container {
  position: relative;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: 100%;
  padding: 1rem;
}

.gauge-title {
  position: absolute;
  top: 0.5rem;
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--el-text-color-primary, #303133);
  text-align: center;
}

.chart-container {
  width: 100%;
  height: 180px;
}

.data-display {
  position: absolute;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
}

.value-display {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-bottom: 0.3rem;
}

.actual-value {
  font-size: 2.2rem;
  font-weight: 700;
  line-height: 1;
  color: var(--el-text-color-primary, #303133);
}

.value-label {
  margin-top: 0.2rem;
  font-size: 0.9rem;
  font-weight: 500;
  color: var(--el-text-color-secondary, #909399);
}

.percentage-display {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.percentage-value {
  font-size: 1.4rem;
  font-weight: 600;
  line-height: 1;
  color: var(--el-text-color-primary, #303133);
  opacity: 0.8;
}

.percentage-label {
  margin-top: 0.2rem;
  font-size: 0.8rem;
  color: var(--el-text-color-secondary, #909399);
}
</style>