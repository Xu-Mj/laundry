<template>
    <label :class="['custom-checkbox-button', className, { 'is-checked': isChecked }]">
        <input type="checkbox" :value="value" :name="groupName" :checked="isChecked" :disabled="disabled"
            class="hidden-checkbox" @change="handleChange" />
        <el-tooltip
            :content="$slots.default?.()"
            placement="top"
            :show-after="200"
            :disabled="!isTextOverflow"
        >
            <span class="button-content" ref="textSpan">
                <slot></slot>
            </span>
        </el-tooltip>
    </label>
</template>

<script setup>
import { inject, computed, ref, onMounted, onUpdated } from 'vue';

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

const isTextOverflow = ref(false);
const textSpan = ref(null);

// Inject the group context from parent component
const { value: groupValue, name: groupName, toggleValue } = inject('checkboxGroupContext');

const isChecked = computed(() => groupValue.value.includes(String(props.value)));

const handleChange = (event) => {
    if (!props.disabled) {
        toggleValue(event.target.value);
    }
};

const checkTextOverflow = () => {
    if (textSpan.value) {
        isTextOverflow.value = textSpan.value.scrollWidth > textSpan.value.clientWidth;
    }
};

onMounted(() => {
    checkTextOverflow();
});

onUpdated(() => {
    checkTextOverflow();
});
</script>

<style scoped>
.custom-checkbox-button {
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
    width: 100%;
    text-align: center;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    padding: 0 4px;
}
</style>