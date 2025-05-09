<template>
    <CreateOrder ref="createOrderRef" :visible="true" :orderId="0" :userId="0" :toggle="() => {  }"
        :refresh="() => { }" />
</template>

<script setup>
import CreateOrder from '@/views/components/createOrder.vue';
</script>