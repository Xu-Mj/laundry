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
                    <div class="card-actions">
                        <el-button type="info" link size="small" @click="showCouponHistory = true">查看历史卡券</el-button>
                        <el-button type="primary" link size="small" @click="showCouponSale = true">购买卡券</el-button>
                    </div>
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

        <!-- 卡券历史弹窗 -->
        <el-dialog v-model="showCouponHistory" title="历史卡券" width="780px" align-center
            custom-class="coupon-history-dialog" :show-close="true">
            <div v-if="historyCoupons && historyCoupons.length > 0" class="coupon-history-container">
                <div v-for="(item, index) in historyCoupons" :key="index" class="coupon-history-item"
                    :class="[`coupon-type-${item.coupon.couponType}`, getActiveClass(item)]">
                    <div class="coupon-left">
                        <div class="coupon-value">
                            <template v-if="item.coupon.couponType === '000'">
                                <span class="amount">{{ item.availableValue || 0 }}</span>
                                <span class="unit">元</span>
                            </template>
                            <template v-else-if="item.coupon.couponType === '003'">
                                <span class="amount">{{ item.coupon.usageValue / 10 }}</span>
                                <span class="unit">折</span>
                            </template>
                            <template v-else-if="item.coupon.couponType === '002'">
                                <span class="amount">{{ item.availableValue }}</span>
                                <span class="unit">次</span>
                            </template>
                            <template v-else>
                                <span class="amount">{{ item.ucCount }}</span>
                                <span class="unit">张</span>
                            </template>
                        </div>
                        <div class="coupon-type">
                            {{ getCouponTypeName(item.coupon.couponType) }}
                        </div>
                    </div>
                    <div class="coupon-right">
                        <div class="coupon-title">{{ item.coupon.couponTitle }}</div>
                        <div class="coupon-desc" v-if="item.coupon.couponDesc">{{ item.coupon.couponDesc }}</div>
                        <div class="coupon-meta">
                            <div class="coupon-meta-item">
                                <el-icon>
                                    <Calendar />
                                </el-icon>
                                <span>有效期至: {{ item.coupon.validTo ? formatTime(item.coupon.validTo) : '永久有效' }}</span>
                            </div>
                            <div class="coupon-meta-item" v-if="item.coupon.couponType === '000'">
                                <el-icon>
                                    <Wallet />
                                </el-icon>
                                <span>可用余额: {{ item.availableValue || 0 }}元</span>
                            </div>
                            <div class="coupon-meta-item" v-else>
                                <el-icon>
                                    <Ticket />
                                </el-icon>
                                <span>剩余数量: {{ item.ucCount }}张</span>
                            </div>
                        </div>
                        <div class="coupon-status">
                            <el-tag size="small" :type="getAvailable(item) ? 'success' : 'info'" effect="light">
                                {{ getAvailable(item) ? '可用' : '已用完' }}
                            </el-tag>
                        </div>
                    </div>
                </div>
            </div>
            <div v-else class="empty-history">
                <el-empty description="暂无历史卡券记录" />
            </div>
        </el-dialog>
    </div>
</template>

<script setup>
import { changeUserStatus, changeUserIdentify } from "@/api/system/user";
import { listUserCouponWithValidTime, listUserCouponNoPage } from '@/api/system/user_coupon';
import { ref, computed, watch } from "vue";
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
const showCouponHistory = ref(false);
const historyCoupons = ref([]);
const couponTypeList = ref(new Set());

function getActiveClass(item) {
    if (item.coupon.couponType === '000' || item.coupon.couponType === '002') {
        return item.availableValue > 0 ? 'coupon-active' : 'coupon-inactive';
    }
    return item.ucCount > 0 ? 'coupon-active' : 'coupon-inactive';
}

function getAvailable(item) {
    if (item.coupon.couponType === '000' || item.coupon.couponType === '002') {
        return item.availableValue > 0;
    }
    return item.ucCount > 0;
}
// 获取卡券类型名称
const getCouponTypeName = (type) => {
    const typeMap = {
        '000': '储值卡',
        '002': '次卡',
        '003': '折扣券',
        '004': '满减券'
    };
    return typeMap[type] || '未知类型';
};

// 获取卡券类型标签样式
const getCouponTypeTag = (type) => {
    const typeTagMap = {
        '000': 'danger',
        '001': 'warning',
        '002': 'success',
        '003': 'primary',
        '004': 'info'
    };
    return typeTagMap[type] || '';
};

// 查询用户历史卡券
const fetchHistoryCoupons = () => {
    if (props.user && props.user.userId) {
        listUserCouponNoPage({ userId: props.user.userId }).then(response => {
            console.log(response);
            historyCoupons.value = response || [];
        });
    }
};

// 监听历史卡券弹窗显示状态
watch(showCouponHistory, (newVal) => {
    if (newVal) {
        fetchHistoryCoupons();
    }
});

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
    listUserCouponWithValidTime(props.user.userId).then(response => {
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

.card-actions {
    display: flex;
    gap: 10px;
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

.empty-history {
    padding: 40px 0;
}

/* 卡券历史弹窗样式 */
:deep(.coupon-history-dialog) {
    border-radius: 8px;
    max-height: 80vh;
    /* Limit dialog height */
    display: flex;
    flex-direction: column;
}

:deep(.coupon-history-dialog .el-dialog__header) {
    padding: 20px;
    margin-right: 0;
    border-bottom: 1px solid var(--el-border-color-light);
    flex-shrink: 0;
    /* Prevent header from shrinking */
}

:deep(.coupon-history-dialog .el-dialog__body) {
    padding: 20px;
    overflow: hidden;
    /* Hide overflow from dialog body */
    flex: 1;
    /* Let body take available space */
}

.coupon-history-container {
    display: flex;
    flex-direction: column;
    gap: 15px;
    max-height: 60vh;
    overflow-y: auto;
    padding-right: 5px;
    padding-bottom: 10px;
}

.coupon-history-item {
    display: flex;
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.05);
    transition: all 0.3s;
    position: relative;
    min-height: 120px;
}

.coupon-history-item:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 16px 0 rgba(0, 0, 0, 0.1);
}

.coupon-left {
    width: 120px;
    padding: 20px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    color: #fff;
    flex-shrink: 0;
}

.coupon-right {
    flex: 1;
    padding: 15px 20px;
    position: relative;
    background-color: #fff;
    border-left: 1px dashed #e8e8e8;
    display: flex;
    flex-direction: column;
    justify-content: center;
}

.coupon-right:before {
    content: '';
    position: absolute;
    top: -8px;
    left: -8px;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background-color: var(--el-bg-color);
}

.coupon-right:after {
    content: '';
    position: absolute;
    bottom: -8px;
    left: -8px;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background-color: var(--el-bg-color);
}

.coupon-value {
    text-align: center;
    line-height: 1;
}

.coupon-value .amount {
    font-size: 28px;
    font-weight: bold;
}

.coupon-value .unit {
    font-size: 14px;
    margin-left: 2px;
}

.coupon-type {
    margin-top: 8px;
    font-size: 14px;
}

.coupon-title {
    font-size: 16px;
    font-weight: bold;
    margin-bottom: 5px;
    color: var(--el-text-color-primary);
    line-height: 1.4;
}

.coupon-desc {
    font-size: 12px;
    color: var(--el-text-color-secondary);
    margin-bottom: 10px;
    line-height: 1.4;
    max-height: 50px;
    overflow-y: auto;
}

.coupon-meta {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
    margin-top: 10px;
    margin-bottom: 5px;
}

.coupon-meta-item {
    display: flex;
    align-items: center;
    font-size: 12px;
    color: var(--el-text-color-secondary);
}

.coupon-meta-item .el-icon {
    margin-right: 4px;
    font-size: 14px;
}

.coupon-status {
    position: absolute;
    top: 15px;
    right: 15px;
}

/* 卡券类型样式 - 只应用于可用的卡券 */
.coupon-active.coupon-type-000 .coupon-left {
    background-color: var(--el-color-danger);
}

/* .coupon-active.coupon-type-001 .coupon-left {
    background-color: var(--el-color-warning);
} */

.coupon-active.coupon-type-002 .coupon-left {
    background-color: var(--el-color-success);
}

.coupon-active.coupon-type-003 .coupon-left {
    background-color: var(--el-color-primary);
}

.coupon-active.coupon-type-004 .coupon-left {
    background-color: var(--el-color-warning);
}

/* 已用完的卡券样式 */
.coupon-inactive {
    opacity: 0.7;
}

.coupon-inactive .coupon-left {
    background-color: #909399 !important;
    /* Override any type-specific background color */
}
</style>