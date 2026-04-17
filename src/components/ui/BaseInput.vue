<script setup lang="ts">
import { ref, computed } from 'vue'

interface Props {
  modelValue: string
  type?: 'text' | 'email' | 'password' | 'number'
  label?: string
  placeholder?: string
  error?: string
  disabled?: boolean
  required?: boolean
  autocomplete?: string
}

const props = withDefaults(defineProps<Props>(), {
  type: 'text',
  disabled: false,
  required: false,
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
  (e: 'blur'): void
  (e: 'focus'): void
}>()

const isFocused = ref(false)
const isPasswordVisible = ref(false)

const inputType = computed(() => {
  if (props.type === 'password') {
    return isPasswordVisible.value ? 'text' : 'password'
  }
  return props.type
})

const hasValue = computed(() => props.modelValue.length > 0)

const togglePassword = () => {
  isPasswordVisible.value = !isPasswordVisible.value
}

const onInput = (event: Event) => {
  const target = event.target as HTMLInputElement
  emit('update:modelValue', target.value)
}
</script>

<template>
  <div
    class="base-input"
    :class="{
      'base-input--focused': isFocused,
      'base-input--error': error,
      'base-input--filled': hasValue,
      'base-input--disabled': disabled,
    }"
  >
    <label v-if="label" class="input-label">
      {{ label }}
      <span v-if="required" class="required-mark">*</span>
    </label>

    <div class="input-wrapper">
      <input
        :type="inputType"
        :value="modelValue"
        :placeholder="placeholder"
        :disabled="disabled"
        :autocomplete="autocomplete"
        class="input-field"
        @input="onInput"
        @focus="isFocused = true; emit('focus')"
        @blur="isFocused = false; emit('blur')"
      />

      <!-- Password toggle -->
      <button
        v-if="type === 'password'"
        type="button"
        class="password-toggle"
        @click="togglePassword"
      >
        <svg
          v-if="isPasswordVisible"
          width="18"
          height="18"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="1.5"
        >
          <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" />
          <circle cx="12" cy="12" r="3" />
          <path d="M4 4l16 16" />
        </svg>
        <svg
          v-else
          width="18"
          height="18"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="1.5"
        >
          <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" />
          <circle cx="12" cy="12" r="3" />
        </svg>
      </button>

      <!-- Focus line animation -->
      <span class="input-line" />
    </div>

    <transition name="fade">
      <span v-if="error" class="input-error">{{ error }}</span>
    </transition>
  </div>
</template>

<style scoped lang="scss">
.base-input {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.input-label {
  font-size: 12px;
  font-weight: 500;
  color: $color-text-dim;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  transition: color 0.3s ease;

  .base-input--focused & {
    color: $color-accent;
  }
}

.required-mark {
  color: $color-secondary;
  margin-left: 2px;
}

.input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
}

.input-field {
  width: 100%;
  padding: 14px 0;
  background: transparent;
  border: none;
  border-bottom: 1px solid $color-border;
  color: $color-text;
  font-family: $font-body;
  font-size: 15px;
  outline: none;
  transition: all 0.3s ease;

  &::placeholder {
    color: $color-text-muted;
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .base-input--focused & {
    border-bottom-color: transparent;
  }

  .base-input--error & {
    border-bottom-color: $color-secondary;
  }
}

.input-line {
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  height: 1px;
  background: $color-accent;
  transform: scaleX(0);
  transform-origin: left;
  transition: transform 0.4s $transition-smooth;

  .base-input--focused & {
    transform: scaleX(1);
  }

  .base-input--error & {
    background: $color-secondary;
    transform: scaleX(1);
  }
}

.password-toggle {
  position: absolute;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
  background: transparent;
  border: none;
  color: $color-text-dim;
  cursor: pointer;
  padding: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: color 0.3s ease;

  &:hover {
    color: $color-text;
  }
}

.input-error {
  font-size: 12px;
  color: $color-secondary;
  display: flex;
  align-items: center;
  gap: 4px;

  &::before {
    content: '!';
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: $color-secondary;
    color: $color-bg;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 10px;
    font-weight: 700;
  }
}

// Transitions
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease, transform 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>
