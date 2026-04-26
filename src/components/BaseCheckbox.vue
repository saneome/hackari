<script setup lang="ts">
interface Props {
  modelValue: boolean
  label?: string
  name?: string
  error?: string
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
}>()

const toggle = () => {
  emit('update:modelValue', !props.modelValue)
}
</script>

<template>
  <div class="base-checkbox" :class="{ 'is-error': !!error }" @click="toggle">
    <div class="checkbox-box" :class="{ 'is-checked': modelValue }">
      <svg
        v-if="modelValue"
        class="check-icon"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="3"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <polyline points="20 6 9 17 4 12" />
      </svg>
    </div>
    <div v-if="label || $slots.default" class="checkbox-label">
      <slot>
        <span v-if="label" v-html="label"></span>
      </slot>
    </div>
  </div>
  <div v-if="error" class="checkbox-error">{{ error }}</div>
</template>

<style scoped lang="scss">
.base-checkbox {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  cursor: pointer;
  user-select: none;
  transition: opacity 0.2s ease;

  &:hover {
    .checkbox-box {
      border-color: $color-accent;
      box-shadow: 0 0 0 3px rgba($color-accent, 0.15);
    }
  }

  &.is-error {
    .checkbox-box {
      border-color: $color-error;
    }
  }
}

.checkbox-box {
  width: 18px;
  height: 18px;
  min-width: 18px;
  min-height: 18px;
  margin-top: 1px;
  border: 1.5px solid $color-border;
  border-radius: 5px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s $transition-smooth;
  background: $color-surface;
  position: relative;

  &.is-checked {
    background: $color-accent;
    border-color: $color-accent;
    box-shadow: 0 0 0 3px rgba($color-accent, 0.2);

    .check-icon {
      animation: check-pop 0.3s $transition-smooth forwards;
    }
  }
}

.check-icon {
  width: 12px;
  height: 12px;
  color: $color-bg;
  opacity: 0;
  transform: scale(0.6);
}

@keyframes check-pop {
  0% {
    opacity: 0;
    transform: scale(0.3);
  }
  60% {
    opacity: 1;
    transform: scale(1.15);
  }
  100% {
    opacity: 1;
    transform: scale(1);
  }
}

.checkbox-label {
  font-size: 12px;
  line-height: 1.5;
  color: $color-text-dim;
  word-break: break-word;

  :deep(.doc-link) {
    color: $color-accent;
    text-decoration: none;
    border-bottom: 1px solid rgba($color-accent, 0.4);
    transition: border-color 0.2s ease;

    &:hover {
      border-bottom-color: $color-accent;
    }
  }
}

.checkbox-error {
  font-size: 11px;
  color: $color-error;
  margin-top: 4px;
  margin-left: 28px;
}
</style>
