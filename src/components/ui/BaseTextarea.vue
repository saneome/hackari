<script setup lang="ts">
import { ref } from 'vue'

interface Props {
  modelValue: string
  label?: string
  placeholder?: string
  error?: string
  disabled?: boolean
  required?: boolean
  rows?: number
}

const props = withDefaults(defineProps<Props>(), {
  disabled: false,
  required: false,
  rows: 4,
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
  (e: 'blur'): void
  (e: 'focus'): void
}>()

const isFocused = ref(false)

const onInput = (event: Event) => {
  const target = event.target as HTMLTextAreaElement
  emit('update:modelValue', target.value)
}
</script>

<template>
  <div
    class="base-textarea"
    :class="{
      'base-textarea--focused': isFocused,
      'base-textarea--error': error,
      'base-textarea--filled': modelValue.length > 0,
      'base-textarea--disabled': disabled,
    }"
  >
    <label v-if="label" class="textarea-label">
      {{ label }}
      <span v-if="required" class="required-mark">*</span>
    </label>

    <div class="textarea-wrapper">
      <textarea
        :rows="rows"
        :value="modelValue"
        :placeholder="placeholder"
        :disabled="disabled"
        class="textarea-field"
        @input="onInput"
        @focus="isFocused = true; emit('focus')"
        @blur="isFocused = false; emit('blur')"
      />
    </div>

    <transition name="fade">
      <span v-if="error" class="textarea-error">{{ error }}</span>
    </transition>
  </div>
</template>

<style scoped lang="scss">
.base-textarea {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.textarea-label {
  font-size: 12px;
  font-weight: 500;
  color: $color-text-dim;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  transition: color 0.3s ease;

  .base-textarea--focused & {
    color: $color-accent;
  }
}

.required-mark {
  color: $color-secondary;
  margin-left: 2px;
}

.textarea-wrapper {
  position: relative;
}

.textarea-field {
  width: 100%;
  padding: 14px;
  background: $color-bg;
  border: 1px solid $color-border;
  border-radius: 8px;
  color: $color-text;
  font-family: $font-body;
  font-size: 15px;
  outline: none;
  transition: all 0.3s ease;
  resize: vertical;
  line-height: 1.5;

  &::placeholder {
    color: $color-text-muted;
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .base-textarea--focused & {
    border-color: $color-accent;
  }

  .base-textarea--error & {
    border-color: $color-secondary;
  }
}

.textarea-error {
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
