<template>
    <el-dialog title="选择打印机" v-model="open" width="400px" :show-close="false" append-to-body
        @closed="closeHangUpDialog">
        <el-select v-model="printer" placeholder="请选择打印机" @change="setPrinter">
            <el-option v-for="item in printers" :key="item.name" :label="item.name" :value="item.name"> </el-option>
        </el-select>
    </el-dialog>
</template>
<script setup name="Setting">
import { invoke } from '@tauri-apps/api/core';

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
async function setPrinter() {
    settledPrinter.value = printers.value.find(item => item.name === printer.value);
    await invoke('set_printer', { printer: settledPrinter.value });
}

onMounted(async () => {
    if (props.visible) {
        settledPrinter.value = await invoke('get_settled_printer');
        printers.value = await invoke('get_printers');
        if(settledPrinter.value) {
            printer.value = settledPrinter.value.name;
        }
        open.value = true;
    }
})

</script>
<style scoped></style>