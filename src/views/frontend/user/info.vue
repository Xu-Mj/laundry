<template>
    <div class="user-info-container">
        <!-- 基本信息卡片 -->
        <el-card shadow="hover" class="info-card">
            <template #header>
                <div class="card-header">
                    <span>基本信息</span>
                </div>
            </template>
            <div class="info-grid">
                <div class="info-item">
                    <el-icon>
                        <User />
                    </el-icon>
                    <span class="label">会员编号:</span>
                    <span class="value">{{ user.userId }}</span>
                </div>
                <div class="info-item">
                    <el-icon>
                        <Avatar />
                    </el-icon>
                    <span class="label">会员姓名:</span>
                    <span class="value">{{ user.nickName }}</span>
                </div>
                <div class="info-item">
                    <el-icon>
                        <UserFilled />
                    </el-icon>
                    <span class="label">会员账号:</span>
                    <span class="value">{{ user.userName }}</span>
                </div>
                <div class="info-item">
                    <el-icon>
                        <Phone />
                    </el-icon>
                    <span class="label">手机号码:</span>
                    <span class="value">{{ user.phonenumber }}</span>
                </div>
                <div class="info-item">
                    <el-icon>
                        <Medal />
                    </el-icon>
                    <span class="label">会员类型:</span>
                    <dict-tag :options="sys_user_type" :value="user.userType" />
                </div>
                <div class="info-item">
                    <el-icon>
                        <Connection />
                    </el-icon>
                    <span class="label">微信标识:</span>
                    <span class="value">{{ user.openId || '-' }}</span>
                </div>
                <div class="info-item">
                    <el-icon>
                        <Male />
                    </el-icon>
                    <span class="label">性别:</span>
                    <dict-tag :options="sys_user_sex" :value="user.sex" />
                </div>
                <div class="info-item">
                    <el-icon>
                        <Star />
                    </el-icon>
                    <span class="label">会员积分:</span>
                    <span class="value highlight">{{ user.integral || 0 }}</span>
                </div>
                <div class="info-item">
                    <el-icon>
                        <Trophy />
                    </el-icon>
                    <span class="label">会员等级:</span>
                    <span class="value">{{ user.levelName || '-' }}</span>
                </div>
            </div>
        </el-card>

        <!-- 卡券信息卡片 -->
        <el-card shadow="hover" class="info-card">
            <template #header>
                <div class="card-header">
                    <span>卡券信息</span>
                    <el-button type="primary" link size="small" @click="showCouponSale = true">购买卡券</el-button>
                </div>
            </template>
            <div class="coupon-section">
                <!-- 储值卡余额 -->
                <div class="coupon-item balance-item">
                    <el-icon>
                        <Wallet />
                    </el-icon>
                    <span class="label">储值卡余额:</span>
                    <span class="value amount">{{ storageCardBalance }} 元</span>
                </div>

                <!-- 次卡 -->
                <div class="coupon-item">
                    <el-icon>
                        <Ticket />
                    </el-icon>
                    <span class="label">次卡:</span>
                    <div class="coupon-tags"
                        v-if="coupons && coupons.filter(item => item.coupon.couponType == '002' && item.availableValue > 0).length > 0">
                        <el-tag
                            v-for="(card, index) in coupons.filter(item => item.coupon.couponType == '002' && item.availableValue > 0)"
                            :key="index" type="success" effect="light" class="coupon-tag">
                            {{ card.coupon.couponTitle }} - {{ card.ucCount }}张
                        </el-tag>
                    </div>
                    <span class="value" v-else>无可用次卡</span>
                </div>

                <!-- 优惠券 -->
                <div class="coupon-item">
                    <el-icon>
                        <Discount />
                    </el-icon>
                    <span class="label">优惠券:</span>
                    <div class="coupon-tags"
                        v-if="coupons && coupons.filter(item => item.coupon.couponType !== '000' && item.coupon.couponType !== '002' && item.availableValue > 0).length > 0">
                        <el-tag
                            v-for="(card, index) in coupons.filter(item => item.coupon.couponType !== '000' && item.coupon.couponType !== '002' && item.availableValue > 0)"
                            :key="index" type="warning" effect="light" class="coupon-tag">
                            {{ card.coupon.couponTitle }} - {{ card.ucCount }}张
                        </el-tag>
                    </div>
                    <span class="value" v-else>无可用优惠券</span>
                </div>
            </div>
        </el-card>

        <!-- 会员画像卡片 -->
        <el-card shadow="hover" class="info-card">
            <template #header>
                <div class="card-header">
                    <span>会员画像</span>
                </div>
            </template>
            <div class="tags-section">
                <template v-if="user.userTags && user.userTags.trim() !== ''">
                    <div class="tags-container">
                        <dict-tag v-for="(item, index) in user.userTags.split(',')" :key="index"
                            :options="sys_user_tags" :value="item.trim()" class="user-tag" />
                    </div>
                </template>
                <div v-else class="empty-tags">暂无会员画像标签</div>

                <div class="remark-section" v-if="user.tagsRemark">
                    <div class="remark-title">画像备注:</div>
                    <div class="remark-content">{{ user.tagsRemark }}</div>
                </div>
            </div>
        </el-card>

        <!-- 账户状态卡片 -->
        <el-card shadow="hover" class="info-card">
            <template #header>
                <div class="card-header">
                    <span>账户状态</span>
                </div>
            </template>
            <div class="status-section">
                <div class="status-item">
                    <el-icon>
                        <Warning />
                    </el-icon>
                    <span class="label">黑灰名单:</span>
                    <el-select v-model="user.identify" @change="handleIdentifyChange(user)" class="identify-select">
                        <el-option v-for="dict in sys_user_identify" :key="dict.value" :label="dict.label"
                            :value="dict.value" />
                    </el-select>
                </div>
                <div class="status-item">
                    <el-icon>
                        <Switch />
                    </el-icon>
                    <span class="label">账号状态:</span>
                    <el-switch v-model="user.status" active-value="0" inactive-value="1"
                        @change="handleStatusChange(user)" class="status-switch"></el-switch>
                </div>
                <div class="status-item">
                    <el-icon>
                        <Calendar />
                    </el-icon>
                    <span class="label">创建时间:</span>
                    <span class="value">{{ formatTime(user.createTime) }}</span>
                </div>
            </div>
        </el-card>

        <!-- 备注信息卡片 -->
        <el-card shadow="hover" class="info-card" v-if="user.remark">
            <template #header>
                <div class="card-header">
                    <span>备注信息</span>
                </div>
            </template>
            <div class="remark-content">
                {{ user.remark }}
            </div>
        </el-card>

        <!-- 卡券购买弹窗 -->
        <el-dialog v-model="showCouponSale" width="780px" append-to-body lock-scroll modal align-center
            :close-on-click-modal="false" :show-close="false">
            <CouponSale :userId="user.userId" :key="showCouponSale" :taggle="() => { showCouponSale = !showCouponSale }"
                :visible="showCouponSale" :couponTypeList="couponTypeList" :submit="handleCouponBought" />
        </el-dialog>
    </div>
</template>

<script setup>
import { changeUserStatus, changeUserIdentify } from "@/api/system/user";
import { listUserCouponWithValidTime } from '@/api/system/user_coupon';
import { ref, computed } from "vue";
import CouponSale from '@/views/components/couponSale.vue';

const { proxy } = getCurrentInstance();
const { sys_user_tags, sys_user_sex, sys_user_type, sys_user_identify } =
    proxy.useDict("sys_user_tags", "sys_user_sex", "sys_user_type", "sys_user_identify");

// 定义props
const props = defineProps({
    user: {
        type: Object,
        required: true,
    }
});

const coupons = ref();
const showCouponSale = ref(false);
const couponTypeList = ref(new Set());

/* 会员状态修改 */
const handleStatusChange = (row) => {
    let text = row.status === "0" ? "启用" : "停用";
    proxy.$modal.confirm('确认要"' + text + '""' + row.userName + '"会员吗?').then(function () {
        return changeUserStatus(row.userId, row.status);
    }).then(() => {
        proxy.notify.success(text + "成功");
    }).catch(function () {
        row.status = row.status === "0" ? "1" : "0";
    });
};

/* 黑灰名单状态修改 */
const handleIdentifyChange = (row) => {
    const identifyMap = {
        '00': '正常',
        '01': '加入黑名单',
        '02': '加入灰名单'
    };
    const text = identifyMap[row.identify] || '正常';

    proxy.$modal.confirm('确认要将"' + row.userName + '"会员' + (row.identify !== '00' ? '设为' + text : '移出黑灰名单') + '吗?').then(function () {
        return changeUserIdentify(row.userId, row.identify);
    }).then(() => {
        proxy.notify.success('设置成功');
    }).catch(function () {
        // 操作取消，回退选择
        const oldIdentify = proxy.getDictValue(sys_user_identify, row.identify);
        row.identify = oldIdentify ? oldIdentify.value : '00';
    });
};

// 卡券购买成功后刷新卡券列表
function handleCouponBought() {
    // 重新获取会员优惠券列表
    if (props.user && props.user.userId) {
        // 添加延迟，确保数据库同步完成
        listUserCouponWithValidTime(props.user.userId).then(response => {
            const mergedCoupons = response.reduce((acc, cur) => {
                const existing = acc.find(item => item.coupon.couponId === cur.coupon.couponId && item.coupon.couponType !== '000');
                if (existing) {
                    existing.ucCount += cur.ucCount;
                } else {
                    acc.push(cur);
                }
                return acc;
            }, []);

            coupons.value = mergedCoupons;

            // 更新卡券类型列表
            couponTypeList.value = new Set(coupons.value.map(coupon => coupon.coupon.couponType));
        });
    }
    // 关闭弹窗
    showCouponSale.value = false;
};

// 获取会员优惠券列表
if (props.user && props.user.userId) {
    listUserCouponWithValidTime(props.user.userId ).then(response => {
        // 合并相同卡券数量，如果couponId相同，那么计算该couponId的ucCount总和
        const mergedCoupons = response.reduce((acc, cur) => {
            const existing = acc.find(item => item.coupon.couponId === cur.coupon.couponId && item.coupon.couponType !== '000');
            if (existing) {
                existing.ucCount += cur.ucCount;
            } else {
                acc.push(cur);
            }
            return acc;
        }, []);

        coupons.value = mergedCoupons;
        // 初始化卡券类型列表
        couponTypeList.value = new Set(coupons.value.map(coupon => coupon.coupon.couponType));
    });
}

// 使用计算属性计算储值卡余额，确保响应式更新
const storageCardBalance = computed(() => {
    if (!coupons.value || coupons.value.length === 0) return 0;

    return coupons.value
        .filter(item => item.coupon?.couponType === '000')
        .reduce((acc, cur) => acc + (cur.availableValue || 0), 0);
});

</script>

<style scoped>
.user-info-container {
    display: flex;
    flex-direction: column;
    gap: 20px;
}

.info-card {
    border-radius: .5rem;
    transition: all 0.3s;
    border: none;
}

.info-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-weight: 600;
    color: var(--el-text-color-primary);
    border-radius: .5rem;
}

.info-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
}

.info-item {
    display: flex;
    align-items: center;
    gap: .5rem;
}

.info-item .el-icon {
    color: var(--el-color-primary);
    font-size: 18px;
}

.label {
    color: var(--el-text-color-secondary);
    font-weight: 500;
    white-space: nowrap;
}

.value {
    color: var(--el-text-color-primary);
}

.value.highlight {
    color: var(--el-color-danger);
    font-weight: 600;
}

/* 会员画像卡片样式 */
.tags-section {
    padding: .5rem 0;
}

.tags-container {
    display: flex;
    flex-wrap: wrap;
    gap: .5rem;
    margin-bottom: 1rem;
}

.user-tag {
    margin-right: 0;
}

.empty-tags {
    color: var(--el-text-color-secondary);
    font-style: italic;
    padding: .5rem 0;
}

.remark-section {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px dashed var(--el-border-color-lighter);
}

.remark-title {
    font-weight: 600;
    margin-bottom: .5rem;
    color: var(--el-text-color-primary);
}

.remark-content {
    color: var(--el-text-color-regular);
    line-height: 1.5;
    white-space: pre-wrap;
}

/* 账户状态卡片样式 */
.status-section {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.status-item {
    display: flex;
    align-items: center;
    gap: .5rem;
}

.status-item .el-icon {
    color: var(--el-color-primary);
    font-size: 18px;
}

.status-switch {
    margin-left: .5rem;
}

/* 卡券信息卡片样式 */
.coupon-section {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.coupon-item {
    display: flex;
    align-items: flex-start;
    gap: .5rem;
}

.coupon-item .el-icon {
    color: var(--el-color-primary);
    font-size: 18px;
    margin-top: 2px;
}

.balance-item .value.amount {
    color: var(--el-color-danger);
    font-weight: 600;
}

.coupon-tags {
    display: flex;
    flex-wrap: wrap;
    gap: .5rem;
}

.coupon-tag {
    margin: 0;
}

.identify-select {
    margin-left: .5rem;
    width: 120px;
}
</style>