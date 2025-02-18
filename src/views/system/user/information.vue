<template>
    <el-dialog v-model="props.visible" :title="props.user.nickName + '-' + props.user.phonenumber" width="66.66%"
        top="0" append-to-body class="custom-dialog" @closed="props.toggle" :show-close="false">
        <!-- <div class="info-body">
            <div class="nav">
                <el-button link @click="">基本信息</el-button>
                <el-button link @click="">消费历史</el-button>
            </div>
            <div class="panel">

            </div>
        </div> -->
        <el-tabs type="card" v-model="currentTab" @tab-click="handleTabClick">
            <el-tab-pane label="基本信息" name="basicInfo">
                <Info :user="props.user" />
            </el-tab-pane>
            <el-tab-pane label="消费历史" name="consumptionHistory" >
                <History v-if="currentTab === 'consumptionHistory'" :userId="props.user.userId" />
            </el-tab-pane>
        </el-tabs>
    </el-dialog>
</template>

<script setup>
import History from './history.vue';
import Info from './info.vue';

const props = defineProps({
    user: {
        type: Object,
        required: true,
    },
    visible: {
        type: Boolean,
        required: true,
    },
    toggle: {
        type: Function,
        required: true,
    }
});
console.log(props);

const currentTab = ref('basicInfo');
function handleTabClick(tab) {
    currentTab.value = tab.props.name;
    console.log(currentTab.value);
}
</script>
<style>
.custom-dialog {
    height: 100%;
    margin-left: auto;
    /* 靠右 */
    margin-right: 0;
    margin-top: 0 !important;
    /* 覆盖默认的顶部间距 */
    display: flex;
    flex-direction: column;
    overflow: auto;
}

/* 在全局样式文件中 */
.el-dialog.custom-dialog:not(.is-fullscreen) {
    margin-top: 0 !important;
}
</style>