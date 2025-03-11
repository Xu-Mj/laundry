<template>
    <el-dialog title="选择打印机" v-model="open" width="400px" :show-close="false" append-to-body
        @closed="closeHangUpDialog">
        <el-select v-model="printer" placeholder="请选择打印机" @change="set">
            <el-option v-for="item in printers" :key="item.name" :label="item.name" :value="item.name"> </el-option>
        </el-select>
    </el-dialog>
</template>
<script setup name="Setting">
import { ElMessage, ElMessageBox } from 'element-plus';
import { setPrinter, getSettledPrinter, getPrinters } from '../../api/system/printer';

const props = defineProps({
    visible: {
        type: Boolean,
        required: true,
        default: false,
    },
    taggle: {
        type: Function,
        required: true,
    }
});
const open = ref(false);
const printer = ref();
const settledPrinter = ref();
const printers = ref([]);

/* 关闭上挂弹窗 */
function closeHangUpDialog() {
    open.value = false;
    props.taggle();
}

/* 关闭上挂弹窗 */
async function set() {
    settledPrinter.value = printers.value.find(item => item.name === printer.value);
    try {
        await setPrinter(settledPrinter.value);
        ElMessage.success("设置成功");
    } catch (error) {
        console.error(error);
        ElMessage.error("设置失败");
    }
}

onMounted(async () => {
    if (props.visible) {
        settledPrinter.value = await getSettledPrinter();
        printers.value = await getPrinters();
        if (settledPrinter.value) {
            printer.value = settledPrinter.value.name;
        }
        open.value = true;
    }
})

</script>
<style scoped></style>