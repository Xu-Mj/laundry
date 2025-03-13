<template>
    <!-- 售后复洗 -->
    <el-dialog :show-close="false" v-model="showRewashDialog" width="1440px" append-to-body lock-scroll modal
        :close-on-click-modal="false">
        <AddCloth :isRewash="true" :clothes="clothList" :userId="order.userId" :orderId="order.orderId"
            :submit="submitClothes" :key="order.orderId" />

        <el-form :model="order" :rules="rules" label-width="80px">
            <el-form-item label="店主调价">
                <el-col :span="12" class="adjust-price-group">
                    <el-input type="number" :min="0" :max="1000" @input="adjustInput"
                        v-model="order.adjust.adjustValueSub" placeholder="请输入调减金额" />
                    <el-input type="number" :min="0" :max="1000" @input="adjustInput"
                        v-model="order.adjust.adjustValueAdd" placeholder="请输入调增金额" />
                    <el-input type="number" :min="0" :max="Infinity" @input="adjustInput"
                        v-model="order.adjust.adjustTotal" placeholder="请输入总金额" />
                    <el-input v-model="order.adjust.remark" placeholder="备注信息" />
                </el-col>
                <el-col :span="6">
                    <b>总件数: {{ clothList.length }}</b>
                    <b>金额: {{ totalPrice }}</b>
                </el-col>
                <el-col :span="6">
                    <el-button type="primary" @click="genRewashOrder">生成复洗订单</el-button>
                    <el-button type="primary" @click="cancel">取 消</el-button>
                </el-col>

            </el-form-item>
        </el-form>
    </el-dialog>
</template>

<script setup name="ReWash">
import { onMounted } from 'vue';
import { delCloths, rewash } from '../../api/system/cloths';
import { pickUp } from "@/api/system/cloths";
import { addRewashOrder } from '../../api/system/orders';
import AddCloth from './addCloth.vue';

const props = defineProps({
    visible: {
        type: Boolean,
        required: true,
    },
    order: {
        type: Object,
        required: true,
    },
    clothes: {
        type: Array,
        required: true,
    },
    toggle: {
        type: Function,
        required: true,
    },
    refresh: {
        type: Function,
        required: true,
    },
});

const { proxy } = getCurrentInstance();

const showRewashDialog = ref(false);

// 衣物列表
const clothList = ref([]);
const order = ref(null);
const totalPrice = ref(null);

async function genRewashOrder() {
    order.value.clothsIds = clothList.value.map(item => item.clothId);
    await addRewashOrder(order.value).then(res => {
        proxy.notify.success("生成复洗订单成功！");
    });

    // 修改原订单衣物为已取走
    await pickUp(props.clothes).catch(err => {
        proxy.notify.error(err.message);
    });

    showRewashDialog.value = false;
    reset();
    props.refresh();
    props.toggle();
}

function reset() {
    clothList.value = [];
    totalPrice.value = null;
    order.value = null;
}

/* 调价输入框输入事件 */
function adjustInput() {
    if (order.value.adjust.adjustTotal) {
        totalPrice.value = order.value.adjust.adjustTotal;
    } else {
        let price = clothList.value.reduce((acc, cur) => {
            // 计算总价
            // 如果服务要求为加急
            let priceValue = cur.priceValue;
            if (cur.serviceRequirement == '001') {
                priceValue *= 2;
            } else if (cur.serviceRequirement == '002') {
                priceValue *= 1.5;
            }
            return acc +
                priceValue + cur.processMarkup
        }, 0);
        price +=
            Number(order.value.adjust.adjustValueAdd ? order.value.adjust.adjustValueAdd : 0) -
            Number(order.value.adjust.adjustValueSub ? order.value.adjust.adjustValueSub : 0);
        totalPrice.value = price > 0 ? price : 0;
    }
}

function submitClothes(value) {
    clothList.value = value;
}

function cancel() {
    if (props.clothes.length == 0) {
        reset();
        showRewashDialog.value = false;
        props.toggle();
        return;
    }
    // 删除服务端已生成的衣物
    delCloths(clothList.value.map(item => item.clothId)).then(() => {
        reset();
        showRewashDialog.value = false;
        props.toggle();
    });
}

onMounted(async () => {
    if (props.visible) {
        if (props.clothes.length == 0) {
            proxy.notify.error('请选择需要复洗的衣物');
            return
        }
        reset();
        order.value = props.order;
        await rewash(props.clothes).then(res => {
            clothList.value = res.rows;
            adjustInput();
        });
        showRewashDialog.value = true;
    }
});
</script>

<style scoped>
.adjust-price-group {
    width: 100%;
    display: flex;
    justify-content: space-around;
    align-items: center;
    gap: 1rem;
}
</style>