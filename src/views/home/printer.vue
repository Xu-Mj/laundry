<template>
    <div class="printer-container">
        <div class="printer-left">
            <div class="printer-shop-name">印洗匠心</div>
            <div class="printer-code">
                <img id="barcode" />
            </div>
        </div>
        <div class="printer-right">
            <div class="printer-first-line">{{ props.cloth }}</div>
            <div class="printer-second-line"></div>
            <div class="printer-third-line"></div>
            <div class="printer-client-info"></div>
        </div>
    </div>
</template>

<script setup>
import { onMounted } from 'vue';


const props = defineProps({
    print: Boolean,
    clothNum: String,
    user: Object,
    cloth: Object,
})

function barcode(receiptOrder) {
    JsBarcode('#barcode', receiptOrder, {
        height: 70,
        format: 'CODE128', // 选择要使用的条形码类型
        text: 'NO:  ' + receiptOrder,
        displayValue: true, // 是否在条形码下方显示文字
        textPosition: 'bottom' // 设置文本的垂直位置
    })
}

onMounted(() => {
    if (props.print) {
        barcode(props.clothNum)
    }
})
</script>

<style scoped>
.container {
    display: flex;
}

.left {
    width: 40%;
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
}

.right {
    width: 60%;
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: flex-start;
}
</style>