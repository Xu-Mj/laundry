<template>
    <div :class="['custom-radio-group', $attrs.class]">
      <slot></slot>
    </div>
  </template>
  
  <script setup>
  import { provide, ref, watch, computed } from 'vue';
  
  const props = defineProps({
    modelValue: {
      type: String,
      default: ''
    },
    // 不再需要用户提供的 name 属性
  });
  
  const emit = defineEmits(['update:modelValue', 'change']);
  
  const localValue = ref(props.modelValue);
  
  // 使用 Vue 内置的 uid 或者简单递增计数器生成唯一 name
  const instance = getCurrentInstance();
  const groupName = computed(() => `radio-group-${instance.uid}`);
  
  watch(
    () => props.modelValue,
    (newValue) => {
      localValue.value = newValue;
    }
  );
  
  // Provide the current value, name, and a method to update it
  provide('radioGroupContext', {
    value: localValue,
    name: groupName,
    updateValue: (newValue) => {
      if (localValue.value !== newValue) {
        localValue.value = newValue;
        emit('update:modelValue', newValue);
        emit('change', newValue); // Emit change event when value changes
      }
    }
  });
  </script>
  
  <style scoped>
  .custom-radio-group {
    display: flex;
    flex-wrap: wrap;
  }
  </style>