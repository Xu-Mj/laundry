<template>
    <el-card shadow="never">
        <div class="user-details">
            <!-- 会员编号 -->
            <div class="detail-item">
                <span class="detail-label">会员编号:</span>
                <span>{{ user.userId }}</span>
            </div>
            <!-- 会员姓名 -->
            <div class="detail-item">
                <span class="detail-label">会员姓名:</span>
                <span>{{ user.nickName }}</span>
            </div>
            <!-- 会员账号 -->
            <div class="detail-item">
                <span class="detail-label">会员账号:</span>
                <span>{{ user.userName }}</span>
            </div>
            <!-- 手机号码 -->
            <div class="detail-item">
                <span class="detail-label">手机号码:</span>
                <span>{{ user.phonenumber }}</span>
            </div>
            <!-- 会员类型 -->
            <div class="detail-item">
                <span class="detail-label">会员类型:</span>
                <dict-tag :options="sys_user_type" :value="user.userType" />
            </div>
            <!-- 微信标识 -->
            <div class="detail-item">
                <span class="detail-label">微信标识:</span>
                <span>{{ user.openId }}</span>
            </div>
            <!-- 性别 -->
            <div class="detail-item">
                <span class="detail-label">性别:</span>
                <dict-tag :options="sys_user_sex" :value="user.sex" />
            </div>
            <!-- 会员积分 -->
            <div class="detail-item">
                <span class="detail-label">会员积分:</span>
                <span>{{ user.integral }}</span>
            </div>
            <!-- 会员等级 -->
            <div class="detail-item">
                <span class="detail-label">会员等级:</span>
                <span>{{ user.postName }}</span>
            </div>
            <!-- 会员画像 -->
            <div class="detail-item">
                <span class="detail-label">会员画像:</span>
                <template v-if="user.userTags && user.userTags.trim() !== ''">
                    <dict-tag v-for="(item, index) in user.userTags.split(',')" :key="index" :options="sys_user_tags"
                        :value="item.trim()" />
                </template>
                <span v-else>-</span>
            </div>
            <!-- 画像备注 -->
            <div class="detail-item">
                <span class="detail-label">画像备注:</span>
                <span>{{ user.tagsRemark }}</span>
            </div>
            <!-- 黑灰名单 -->
            <div class="detail-item">
                <span class="detail-label">黑灰名单:</span>
                <dict-tag :options="sys_user_identify" :value="user.identify" />
            </div>
            <!-- 账号状态 -->
            <div class="detail-item">
                <span class="detail-label">账号状态:</span>
                <el-switch v-model="user.status" active-value="0" inactive-value="1"
                    @change="handleStatusChange(user)"></el-switch>
            </div>
            <!-- 创建时间 -->
            <div class="detail-item">
                <span class="detail-label">创建时间:</span>
                <span>{{ parseTime(user.createTime) }}</span>
            </div>
            <!-- 卡券列表信息 -->
            <div class="detail-item">
                <span class="detail-label">拥有卡券:</span>
                <template v-if="coupons && coupons.length > 0">
                    <div class="user-tags-container">
                        <span v-for="(card, index) in coupons" :key="index">
                            {{ card.coupon.couponTitle }}
                            -余额
                            {{ card.availableValue }}
                            {{ card.coupon.couponType == '000' ? '元' : '次' }}
                            {{ index === coupons.length - 1 ? '' : '|' }}
                        </span>
                    </div>
                </template>
            </div>
            <!-- 备注信息 -->
            <div class="detail-item">
                <span class="detail-label">备注信息:</span>
                <span>{{ user.remark }}</span>
            </div>
        </div>
    </el-card>
</template>

<script setup>
import { changeUserStatus } from "@/api/system/user";
import { listUserCoupon } from '@/api/system/user_coupon';
import { ref } from "vue";

const { proxy } = getCurrentInstance();
const { sys_user_tags, sys_user_sex, sys_user_type, sys_user_identify } = proxy.useDict("sys_user_tags", "sys_user_sex", "sys_user_type", "sys_user_identify");

// 定义props
const props = defineProps({
    user: {
        type: Object,
        required: true,
    }
});

const coupons = ref();

/* 会员状态修改 */
const handleStatusChange = (row) => {
    let text = row.status === "0" ? "启用" : "停用";
    proxy.$modal.confirm('确认要"' + text + '""' + row.userName + '"会员吗?').then(function () {
        return changeUserStatus(row.userId, row.status);
    }).then(() => {
        proxy.$modal.msgSuccess(text + "成功");
    }).catch(function () {
        row.status = row.status === "0" ? "1" : "0";
    });
};
// 获取会员优惠券列表
if (props.user && props.user.userId) {
    listUserCoupon({ userId: props.user.userId }).then(response => {
        coupons.value = response.rows;
    });
}
</script>

<style scoped>
.el-card {
    border: none;
}

.user-details {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.detail-item {
    display: flex;
    align-items: center;
    gap: .5rem;
}

.detail-label {
    font-weight: bold;
    flex-shrink: 0;
}

.user-tags-container {
    display: flex;
    flex-wrap: wrap;
    gap: .3rem
}
</style>