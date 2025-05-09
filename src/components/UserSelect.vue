<template>
  <div class="custom-select-container" :class="{ 'is-disabled': disabled }">
    <div class="select-input-wrapper" @click="handleWrapperClick">
      <input
        ref="inputRef"
        type="text"
        class="select-input"
        :placeholder="placeholder"
        :value="inputValue"
        :disabled="disabled"
        @input="handleInput"
        @focus="handleFocus"
        @blur="handleBlur"
        @keydown.down.prevent="handleKeyDown"
        @keydown.up.prevent="handleKeyUp"
        @keydown.enter.prevent="handleEnter"
        @keydown.esc.prevent="closeDropdown"
      />
      <div class="select-prefix">
        <el-icon>
          <Phone />
        </el-icon>
      </div>
      <div class="select-suffix" v-if="inputValue && !disabled">
        <el-icon @click.stop="(e) => clearInput(e)">
          <CircleClose />
        </el-icon>
      </div>
    </div>
    
    <div class="select-dropdown" v-show="isDropdownVisible" ref="dropdownRef">
      <div class="select-dropdown-list" ref="listRef">
        <div 
          v-for="(item, index) in filteredUserList" 
          :key="item.userId"
          class="select-option"
          :class="{ 'is-active': activeIndex === index }"
          @mousedown.prevent="selectOption(item)"
          @mouseover="activeIndex = index"
        >
          <div class="option-content">
            <span class="option-name">{{ item.nickName }}</span>
            <span class="option-phone">{{ item.phonenumber }}</span>
          </div>
        </div>
        
        <!-- 无数据提示 -->
        <div v-if="filteredUserList.length === 0" class="no-data">
          无匹配数据
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch, nextTick, onMounted, onBeforeUnmount } from 'vue';
import { Phone, CircleClose } from '@element-plus/icons-vue';

const props = defineProps({
  modelValue: {
    type: [Object, String, Number],
    default: null
  },
  placeholder: {
    type: String,
    default: '请输入手机号码搜索'
  },
  disabled: {
    type: Boolean,
    default: false
  },
  searchMethod: {
    type: Function,
    required: true
  },
  allUsers: {
    type: Array,
    default: () => []
  }
});

const emit = defineEmits(['update:modelValue', 'change', 'blur', 'need-create-user', 'validate', 'clear-validation', 'update-phone']);

// refs
const inputRef = ref(null);
const dropdownRef = ref(null);
const listRef = ref(null);

// 状态
const isDropdownVisible = ref(false);
const inputValue = ref('');
const filteredUserList = ref([]);
const activeIndex = ref(-1);
const isValidInput = ref(true);
const isClearing = ref(false);
const hasTriggeredCreateUser = ref(false);
const isCreatingUser = ref(false);

// 计算属性
const shouldCreateUser = computed(() => {
  return inputValue.value && 
         isValidPhone(inputValue.value) && 
         filteredUserList.value.length === 0;
});

// 监听modelValue变化
watch(() => props.modelValue, (newVal) => {
  if (newVal && typeof newVal === 'object') {
    // 如果是对象，显示手机号
    inputValue.value = newVal.phonenumber || '';
    isValidInput.value = true;
  } else if (newVal === null || newVal === undefined) {
    inputValue.value = '';
  }
}, { immediate: true, deep: true });

// 验证是否为有效的手机号
const isValidPhone = (value) => {
  const phoneRegex = /^1[3-9]\d{9}$/;
  return phoneRegex.test(value);
};

// 处理输入框点击
const handleWrapperClick = () => {
  if (props.disabled) return;
  inputRef.value.focus();
};

// 处理输入
const handleInput = (event) => {
  const value = event.target.value;
  
  // 确保输入是数字
  const numericValue = value.replace(/\D/g, '');
  
  if (numericValue !== value) {
    // 如果有非数字字符被移除，更新输入框的值
    event.target.value = numericValue;
  }
  
  // 保存旧值，用于判断是否变化
  const oldValue = inputValue.value;
  
  // 更新输入值
  inputValue.value = numericValue;
  
  // 验证手机号格式
  if (numericValue.length > 0) {
    isValidInput.value = numericValue.length <= 11;
    
    // 如果输入了11位数字，验证是否是有效的手机号
    if (numericValue.length === 11) {
      isValidInput.value = isValidPhone(numericValue);
      
      // 如果不是有效手机号，发出验证事件
      if (!isValidInput.value) {
        emit('validate', false, '请输入有效的手机号');
      } else {
        emit('validate', true);
        
        // 如果是有效手机号，搜索并自动选中结果
        search(numericValue).then(() => {
          // 如果只有一个结果，自动选中
          if (filteredUserList.value.length === 1) {
            selectOption(filteredUserList.value[0]);
          }
          // 如果有多个结果，但有一个手机号完全匹配的，选中它
          else if (filteredUserList.value.length > 1) {
            const exactMatch = filteredUserList.value.find(user => 
              user.phonenumber === numericValue
            );
            if (exactMatch) {
              selectOption(exactMatch);
            }
          }
        });
      }
    } else {
      // 对于非11位的输入，也触发验证
      emit('validate', false, '请输入有效的手机号');
    }
  } else {
    isValidInput.value = true;
    // 当输入为空时，不触发验证错误
    emit('validate', true);
    // 显示所有用户
    loadAllUsers();
  }
  
  // 每次输入都进行搜索（除了已经在上面处理过的11位手机号情况）
  if (numericValue.length !== 11 || !isValidPhone(numericValue)) {
    search(numericValue);
  }
};

// 处理聚焦
const handleFocus = () => {
  if (props.disabled) return;
  
  // 聚焦时显示下拉列表
  isDropdownVisible.value = true;
  
  // 如果有输入值，立即搜索
  if (inputValue.value) {
    search(inputValue.value);
  } else {
    // 没有输入值时，显示所有用户
    loadAllUsers();
  }
  
  // 不再触发need-create-user事件，避免重置用户信息
  // 只有在handleBlur或输入有效手机号时才触发
};

// 加载所有用户
const loadAllUsers = () => {
  props.searchMethod('').then(result => {
    filteredUserList.value = result;
    isDropdownVisible.value = true;
  });
};

// 处理失去焦点
const handleBlur = () => {
  // 如果正在执行清空操作，不处理失焦事件
  if (isClearing.value) {
    return;
  }
  
  // 延迟关闭下拉列表，以便可以点击选项
  setTimeout(() => {
    // 再次检查是否在清空操作中
    if (isClearing.value) {
      return;
    }
    
    isDropdownVisible.value = false;
    activeIndex.value = -1;
    
    // 如果有有效的手机号但没有匹配到用户，通知父组件需要创建用户
    if (shouldCreateUser.value) {
      // 只有在未处于创建用户模式时，才触发创建用户事件
      if (!isCreatingUser.value) {
        isCreatingUser.value = true;
        emit('need-create-user', inputValue.value);
      }
      // 如果已经处于创建用户模式，只需要通知更新手机号
      else {
        emit('update-phone', inputValue.value);
      }
      // 发出验证成功事件
      emit('validate', true);
    } else if (inputValue.value && !isValidPhone(inputValue.value) && inputValue.value.length > 0) {
      // 如果输入了内容但不是有效手机号，发出验证失败事件
      emit('validate', false, '请输入有效的手机号');
    } else if (!inputValue.value && !props.modelValue) {
      // 如果没有输入内容且没有选择用户，只有在表单提交时才需要验证
      // 这里不主动触发验证，让表单验证规则来处理
    } else {
      emit('validate', true);
    }
    
    emit('blur');
  }, 200);
};

// 搜索方法
const search = (query) => {
  if (!query) {
    loadAllUsers();
    return Promise.resolve([]);
  }
  
  // 重置活动索引
  activeIndex.value = -1;
  
  // 调用父组件提供的搜索方法
  return props.searchMethod(query).then(result => {
    filteredUserList.value = result;
    
    // 确保下拉列表可见
    isDropdownVisible.value = true;
    
    // 如果有有效的手机号但没有匹配到用户
    if (shouldCreateUser.value) {
      // 只有在未处于创建用户模式时，才触发创建用户事件
      if (!isCreatingUser.value) {
        isCreatingUser.value = true;
        emit('need-create-user', inputValue.value);
      }
      // 如果已经处于创建用户模式，只需要通知更新手机号
      else {
        emit('update-phone', inputValue.value);
      }
    }
    // 如果匹配到了用户，重置创建用户模式
    else if (result.length > 0) {
      isCreatingUser.value = false;
    }
    
    return result;
  });
};

// 选择选项
const selectOption = (item) => {
  if (item) {
    inputValue.value = item.phonenumber;
    isValidInput.value = true;
    isCreatingUser.value = false; // 重置创建用户状态
    
    emit('update:modelValue', item);
    emit('change', item);
    
    // 发送验证成功事件，清除验证提示
    emit('validate', true);
    
    // 清除表单验证错误
    nextTick(() => {
      // 触发一个自定义事件，通知父组件清除验证错误
      emit('clear-validation');
    });
  }
  
  closeDropdown();
};

// 清空输入
const clearInput = (e) => {
  // 先阻止事件冒泡，避免触发其他事件
  e?.stopPropagation();
  
  // 设置标记，表示这是清空操作
  isClearing.value = true;
  
  // 更新状态
  inputValue.value = '';
  isValidInput.value = true;
  isCreatingUser.value = false; // 重置创建用户状态
  
  // 发出事件
  emit('update:modelValue', null);
  emit('change', null);
  emit('validate', true);
  
  // 清除验证提示
  emit('clear-validation');
  
  // 使用nextTick确保DOM更新后再执行后续操作
  nextTick(() => {
    // 加载所有用户数据
    loadAllUsers();
    
    // 确保下拉列表可见
    isDropdownVisible.value = true;
    
    // 保持焦点在输入框
    inputRef.value?.focus();
    
    // 重置清空标记
    setTimeout(() => {
      isClearing.value = false;
    }, 300);
  });
};

// 关闭下拉列表
const closeDropdown = () => {
  isDropdownVisible.value = false;
  activeIndex.value = -1;
};

// 键盘导航 - 向下
const handleKeyDown = () => {
  if (!isDropdownVisible.value) {
    isDropdownVisible.value = true;
    if (filteredUserList.value.length === 0) {
      loadAllUsers();
    }
    return;
  }
  
  const maxIndex = filteredUserList.value.length - 1;
  
  if (activeIndex.value < maxIndex) {
    activeIndex.value++;
    scrollToActive();
  }
};

// 键盘导航 - 向上
const handleKeyUp = () => {
  if (activeIndex.value > -1) {
    activeIndex.value--;
    scrollToActive();
  }
};

// 键盘导航 - 回车选择
const handleEnter = () => {
  if (!isDropdownVisible.value) return;
  
  if (activeIndex.value >= 0 && activeIndex.value < filteredUserList.value.length) {
    // 选择用户
    selectOption(filteredUserList.value[activeIndex.value]);
  }
};

// 滚动到活动选项
const scrollToActive = () => {
  nextTick(() => {
    const activeEl = listRef.value?.querySelector('.is-active');
    if (activeEl && listRef.value) {
      const containerRect = listRef.value.getBoundingClientRect();
      const activeRect = activeEl.getBoundingClientRect();
      
      if (activeRect.bottom > containerRect.bottom) {
        listRef.value.scrollTop += activeRect.bottom - containerRect.bottom;
      } else if (activeRect.top < containerRect.top) {
        listRef.value.scrollTop -= containerRect.top - activeRect.top;
      }
    }
  });
};

// 点击外部关闭下拉列表
const handleClickOutside = (event) => {
  // 如果正在执行清空操作，不关闭下拉列表
  if (isClearing.value) {
    return;
  }
  
  // 获取清空按钮元素
  const clearButton = document.querySelector('.select-suffix');
  
  // 如果点击的是清空按钮，不关闭下拉列表
  if (clearButton && clearButton.contains(event.target)) {
    return;
  }
  
  const container = document.querySelector('.custom-select-container');
  if (container && !container.contains(event.target)) {
    isDropdownVisible.value = false;
  }
};

// 生命周期钩子
onMounted(() => {
  document.addEventListener('click', handleClickOutside);
});

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside);
});

// 暴露方法给父组件
defineExpose({
  focus: () => {
    inputRef.value?.focus();
  },
  blur: () => {
    inputRef.value?.blur();
  },
  getInputValue: () => inputValue.value,
  validate: () => {
    if (shouldCreateUser.value) {
      return { valid: true };
    } else if (props.modelValue) {
      return { valid: true };
    } else if (inputValue.value && !isValidPhone(inputValue.value)) {
      return { valid: false, message: '请输入有效的手机号码' };
    } else if (!inputValue.value) {
      return { valid: false, message: '请选择或输入有效的手机号码' };
    }
    return { valid: true };
  }
});
</script>

<style scoped>
.custom-select-container {
  position: relative;
  width: 100%;
  font-size: 14px;
}

.custom-select-container.is-disabled {
  opacity: 0.7;
  cursor: not-allowed;
}

.select-input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  width: 100%;
  height: 40px;
  border: 1px solid var(--el-border-color);
  border-radius: 4px;
  background-color: var(--el-fill-color-blank);
  transition: all 0.3s;
  cursor: pointer;
}

.select-input-wrapper:hover {
  border-color: var(--el-color-primary);
}

.select-input-wrapper:focus-within {
  border-color: var(--el-color-primary);
  box-shadow: 0 0 0 2px rgba(var(--el-color-primary-rgb), 0.2);
}

.select-input {
  flex: 1;
  height: 100%;
  padding: 0 30px;
  border: none;
  outline: none;
  background: transparent;
  color: var(--el-text-color-primary);
}

.select-input::placeholder {
  color: var(--el-text-color-placeholder);
}

.select-prefix {
  position: absolute;
  left: 10px;
  color: var(--el-text-color-secondary);
}

.select-suffix {
  position: absolute;
  right: 10px;
  cursor: pointer;
  color: var(--el-text-color-secondary);
}

.select-suffix:hover {
  color: var(--el-text-color-primary);
}

.select-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  width: 100%;
  margin-top: 5px;
  background-color: var(--el-bg-color);
  border: 1px solid var(--el-border-color-light);
  border-radius: 4px;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
  z-index: 2;
}

.select-dropdown-list {
  max-height: 140px; /* 约5条记录的高度 */
  overflow-y: auto;
}

.select-option {
  padding: 10px 12px;
  cursor: pointer;
  transition: background-color 0.3s;
  height: 40px; /* 固定每个选项的高度 */
  box-sizing: border-box;
  display: flex;
  align-items: center;
}

.select-option:hover,
.select-option.is-active {
  background-color: var(--el-fill-color-light);
}

.option-content {
  display: flex;
  justify-content: space-between;
  width: 100%;
}

.option-name {
  font-weight: 500;
}

.option-phone {
  color: var(--el-text-color-secondary);
}

.no-data {
  padding: 10px 12px;
  color: var(--el-text-color-secondary);
  text-align: center;
}
</style> 