<template>
    <CouponSale :visible="true" :taggle="() => { }" />

</template>

<script setup>
import CouponSale from '@/views/components/couponSale.vue'

onMounted(() => {
    console.log('createOrderContainer');
});
</script>