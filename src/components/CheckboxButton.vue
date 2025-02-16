<template>
    <label :class="['custom-checkbox-button', className, { 'is-checked': isChecked }]">
        <input type="checkbox" :value="value" :name="groupName" :checked="isChecked" :disabled="disabled"
            class="hidden-checkbox" @change="handleChange" />
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

// Inject the group context from parent component
const { value: groupValue, name: groupName, toggleValue } = inject('checkboxGroupContext');

const isChecked = computed(() => groupValue.value.includes(String(props.value)));

const handleChange = (event) => {
    if (!props.disabled) {
        toggleValue(event.target.value);
    }
};
</script>

<style scoped>
.custom-checkbox-button {
    width: 6rem;
    height: 3rem;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    background-color: #fff;
    cursor: pointer;
    transition: all 0.3s ease;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    margin: .5rem;
}

.custom-checkbox-button.is-checked {
    background-color: #409eff;
    border-color: #409eff;
    color: #fff;
}

.custom-checkbox-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

.hidden-checkbox {
    display: none;
}

.button-content {
    font-size: 14px;
}
</style>