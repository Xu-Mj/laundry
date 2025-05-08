<template>
    <label :class="['custom-radio-button', className, { 'is-checked': isChecked }]">
        <input type="radio" :value="value" :name="groupName" :disabled="disabled" class="hidden-radio"
            @change="handleChange" />
        <span class="button-content">
            <slot></slot>
        </span>
    </label>
</template>

<script setup>
import { inject, computed } from 'vue';

const props = defineProps({
    value: {
        type: String,
        required: true
    },
    className: {
        type: String,
        default: ''
    },
    disabled: {
        type: Boolean,
        default: false
    }
});

// Inject the group value and update function from parent component
const { value: groupValue, name: groupName, updateValue } = inject('radioGroupContext');

const isChecked = computed(() => groupValue.value == props.value);

const handleChange = (event) => {
    if (!props.disabled) {
        updateValue(event.target.value);
    }
};
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
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    margin: .5rem;
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

.hidden-radio {
    display: none;
}

.button-content {
    font-size: 14px;
    padding: .5rem;
}
</style>