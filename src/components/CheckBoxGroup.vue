<template>
    <div :class="['custom-checkbox-group', $attrs.class]">
      <slot></slot>
    </div>
  </template>
  
  <script setup>
  import { provide, ref, watch } from 'vue';
  import { getCurrentInstance } from 'vue';
  
  const props = defineProps({
    modelValue: {
      type: Array,
      default: () => []
    }
  });
  
  const emit = defineEmits(['update:modelValue', 'change']);
  
  const localValue = ref([...props.modelValue]);
  
  // 获取当前组件实例并使用其 uid 生成唯一 name
  const instance = getCurrentInstance();
  const groupName = computed(() => `checkbox-group-${instance.uid}`);
  
  watch(
    () => props.modelValue,
    (newValue) => {
      localValue.value = [...newValue];
    }
  );
  
  // Provide the current value, name, and a method to update it
  provide('checkboxGroupContext', {
    value: localValue,
    name: groupName,
    toggleValue: (value) => {
      const index = localValue.value.indexOf(value);
      if (index > -1) {
        // 如果已选中，则取消选中
        localValue.value.splice(index, 1);
      } else {
        // 如果未选中，则添加到选中列表
        localValue.value.push(value);
      }
      emit('update:modelValue', [...localValue.value]);
      emit('change', [...localValue.value]); // Emit change event when value changes
    }
  });
  </script>
  
  <style scoped>
  .custom-checkbox-group {
    display: flex;
    flex-wrap: wrap;
  }
  </style>