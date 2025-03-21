<template>
  <div style="height: 100%;">
    <OrderContent :taggle="() => { router.push('/') }" />
  </div>
</template>

<script setup>
import OrderContent from '@/views/components/oderContent.vue';
import { useRouter } from 'vue-router';

const router = useRouter();

</script>