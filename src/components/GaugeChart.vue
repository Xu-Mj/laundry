<template>
    <div
        style="padding: 1rem; position: relative; display: flex; flex-direction: column; justify-content: center; align-items: center; gap: 1rem; ">

        <!-- 标题 -->
        <div v-if="title" style="position: absolute; top:1rem; text-align: center; font-weight: bold;">
            {{ title }}
        </div>
        <!-- 图表容器 -->
        <div ref="chart" style="width: 200px; height: 200px;"></div>
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
            default: '' // 标题参数，可选
        }
    },
    data() {
        return {
            chart: null,
            option: {
                animationEasing: 'quarticInOut',
                animationDuration: 1000,
                animationDurationUpdate: 1000,
                animationEasingUpdate: 'quarticInOut',
                dataset: {
                    source: [[1, this.value]]
                },
                tooltip: {},
                angleAxis: {
                    type: 'value',
                    startAngle: 0,
                    show: false,
                    min: 0,
                    max: this.total
                },
                radiusAxis: {
                    type: 'value',
                    show: false
                },
                polar: {},
                series: [
                    {
                        type: 'custom',
                        coordinateSystem: 'polar',
                        renderItem: this.renderItem
                    }
                ]
            }
        };
    },
    watch: {
        value(newVal) {
            this.option.dataset.source = [[1, newVal]];
            this.chart.setOption(this.option);
        },
        total(newTotal) {
            this.option.angleAxis.max = newTotal;
            this.chart.setOption(this.option);
        }
    },
    mounted() {
        this.chart = echarts.init(this.$refs.chart);
        this.chart.setOption(this.option);
    },
    methods: {
        renderItem(params, api) {
            const valOnRadian = api.value(1);
            const coords = api.coord([api.value(0), valOnRadian]);
            const polarEndRadian = coords[3];

            return {
                type: 'group',
                children: [
                    {
                        type: 'sector',
                        shape: {
                            cx: params.coordSys.cx,
                            cy: params.coordSys.cy,
                            r: this.outerRadius,
                            r0: this.innerRadius,
                            startAngle: 0,
                            endAngle: -polarEndRadian
                        },
                        style: {
                            fill: 'rgba(0, 100, 200, 0.5)' // 自定义扇形颜色
                        }
                    },
                    {
                        type: 'circle',
                        shape: {
                            cx: params.coordSys.cx,
                            cy: params.coordSys.cy,
                            r: this.insidePanelRadius
                        },
                        style: {
                            fill: '#fff',
                            shadowBlur: 12, // 缩小阴影效果
                            shadowOffsetX: 0,
                            shadowOffsetY: 0,
                            shadowColor: 'rgba(76,107,167,0.4)'
                        }
                    },
                    {
                        type: 'text',
                        extra: {
                            valOnRadian: valOnRadian,
                            transition: 'valOnRadian',
                            enterFrom: { valOnRadian: 0 }
                        },
                        style: {
                            text: this.makeText(valOnRadian), // 显示具体数值
                            fontSize: 25, // 缩小字体大小
                            fontWeight: 700,
                            x: params.coordSys.cx,
                            y: params.coordSys.cy,
                            fill: 'rgb(0,50,190)',
                            align: 'center',
                            verticalAlign: 'middle',
                            enterFrom: { opacity: 0 }
                        },
                        during: (apiDuring) => {
                            apiDuring.setStyle(
                                'text',
                                this.makeText(apiDuring.getExtra('valOnRadian'))
                            );
                        }
                    }
                ]
            };
        },
        makeText(valOnRadian) {
            if (valOnRadian < -10) {
                alert('illegal during val: ' + valOnRadian);
            }
            return valOnRadian.toFixed(0); // 直接返回具体数值
        }
    },
    computed: {
        outerRadius() {
            return 100; // 缩小外圆半径
        },
        innerRadius() {
            return 70; // 缩小内圆半径
        },
        insidePanelRadius() {
            return 70; // 缩小中间圆半径
        }
    }
};
</script>

<style scoped>
/* Add any styles you need here */
</style>