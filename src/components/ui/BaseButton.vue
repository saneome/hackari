<script setup lang="ts">
interface Props {
  variant?: 'primary' | 'secondary' | 'ghost'
  size?: 'sm' | 'md' | 'lg'
  type?: 'button' | 'submit' | 'reset'
  disabled?: boolean
  loading?: boolean
}

withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'md',
  type: 'button',
  disabled: false,
  loading: false,
})

defineEmits<{
  (e: 'click', event: MouseEvent): void
}>()
</script>

<template>
  <button
    :type="type"
    :disabled="disabled || loading"
    :class="[
      'base-button',
      `base-button--${variant}`,
      `base-button--${size}`,
      { 'base-button--loading': loading }
    ]"
    @click="$emit('click', $event)"
  >
    <span class="button-content">
      <slot />
    </span>
    <span class="button-bg" />
    <span v-if="loading" class="button-spinner">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
      </svg>
    </span>
  </button>
</template>

<style scoped lang="scss">
.base-button {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  font-family: $font-body;
  font-weight: 500;
  letter-spacing: 0.02em;
  text-transform: lowercase;
  cursor: pointer;
  overflow: hidden;
  transition: all 0.4s $transition-smooth;
  border: none;
  background: transparent;
  border-radius: 8px;

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  &--loading {
    .button-content {
      opacity: 0;
    }
  }
}

.button-content {
  position: relative;
  z-index: 2;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: opacity 0.3s ease;
}

.button-bg {
  position: absolute;
  inset: 0;
  transform: scaleX(0);
  transform-origin: left;
  transition: transform 0.4s $transition-smooth;
  z-index: 1;
}

.button-spinner {
  position: absolute;
  z-index: 3;
  width: 20px;
  height: 20px;
  animation: spin 1s linear infinite;

  svg {
    width: 100%;
    height: 100%;
  }
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

// Sizes
.base-button--sm {
  padding: 10px 20px;
  font-size: 12px;
}

.base-button--md {
  padding: 14px 28px;
  font-size: 14px;
}

.base-button--lg {
  padding: 18px 36px;
  font-size: 16px;
}

// Variants
.base-button--primary {
  border: 1px solid $color-accent;
  color: $color-accent;

  .button-bg {
    background: $color-accent;
  }

  &:hover:not(:disabled) {
    color: $color-bg;

    .button-bg {
      transform: scaleX(1);
    }
  }
}

.base-button--secondary {
  border: 1px solid $color-border;
  color: $color-text;

  .button-bg {
    background: rgba($color-text, 0.1);
  }

  &:hover:not(:disabled) {
    border-color: rgba($color-text, 0.3);

    .button-bg {
      transform: scaleX(1);
    }
  }
}

.base-button--ghost {
  color: $color-text-dim;
  border: 1px solid transparent;

  .button-bg {
    background: rgba($color-text, 0.05);
  }

  &:hover:not(:disabled) {
    color: $color-text;

    .button-bg {
      transform: scaleX(1);
    }
  }
}
</style>
