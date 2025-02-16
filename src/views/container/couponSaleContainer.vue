<template>
    <CouponSale :visible="true" :taggle="() => { }" />

</template>

<script setup>
import CouponSale from '@/views/home/couponSale.vue'
const router = useRouter();

onMounted(() => {
    console.log('createOrderContainer');
});
</script>