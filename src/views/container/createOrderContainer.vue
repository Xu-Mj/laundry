<template>
    <CreateOrder ref="createOrderRef" :visible="true" :orderId="0" :userId="0" :toggle="() => { router.push('/index') }"
        :refresh="() => { }" />
</template>

<script setup>
import CreateOrder from '@/views/components/createOrder.vue';
const router = useRouter();
</script>