<template>
    <div class="big-icon">
        <div class="title">{{ props.title }}</div>
        <div class="num">{{ props.num }}</div>
    </div>
</template>

<script setup>
const props = defineProps({
    title: String,
    num: Number,
})
</script>

<style scoped>
.big-icon {
    width: 100%;
    height: 100%;
    /* aspect-ratio: 1 / 1; */
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    border: 1px solid #ccc;
    border-radius: .3rem;
    position: absolute;

    .title {
        color: #333;
    }

    .num {
        width: 100%;
        height: 100%;
        font-size: 3rem;
        font-weight: bolder;
        color: #333;
        display: flex;
        justify-content: center;
        align-items: center;
    }
}
</style>