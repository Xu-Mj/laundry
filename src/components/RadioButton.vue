<template>
    <label :class="[{ 'custom-radio-button': true }, className, { 'is-checked': isSelected }]">
        <input type="radio" :value="value" hidden @change="onChange" :checked="isSelected">
        <span>{{ label }}<slot></slot></span>
    </label>
</template>

<script>
export default {
    props: {
        value: {
            type: [String, Number],
            required: true
        },
        className: {
            type: String,
            default: ''
        },
        label: {
            type: String,
            required: true
        },
        modelValue: {
            type: [String, Number],
            required: true
        }
    },
    computed: {
        isSelected() {
            return this.modelValue === this.value;
        }
    },
    methods: {
        onChange() {
            this.$emit('update:modelValue', this.value);
        }
    }
}
</script>

<style scoped>
.custom-radio-button {
    width: 6rem;
    height: 3rem;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    background-color: var(--el-fill-color-light);
    cursor: pointer;
    transition: all 0.3s ease;
    box-shadow: var(--el-box-shadow-lighter);

    &:hover {
        transform: translateY(-2px);
        box-shadow: var(--el-box-shadow-light);
    }
}

.custom-radio-button.is-checked {
    background-color: #409eff;
    border-color: #409eff;
    color: #fff;
}

.custom-radio-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}
</style>
