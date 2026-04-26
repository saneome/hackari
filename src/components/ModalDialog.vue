<script setup lang="ts">
import { watch, nextTick } from 'vue'
import { gsap } from 'gsap'
import { X, AlertCircle, CheckCircle, Info, AlertTriangle } from 'lucide-vue-next'
import type { ModalDialogState } from '@/composables/useModalDialog'

const props = defineProps<{
  state: ModalDialogState
}>()

const emit = defineEmits<{
  confirm: []
  cancel: []
}>()

const getIcon = () => {
  switch (props.state.type) {
    case 'success':
      return CheckCircle
    case 'warning':
      return AlertTriangle
    case 'error':
      return AlertCircle
    default:
      return Info
  }
}

const getIconColor = () => {
  switch (props.state.type) {
    case 'success':
      return '#4ade80'
    case 'warning':
      return '#fbbf24'
    case 'error':
      return '#ff4444'
    default:
      return '#D4FF00'
  }
}

// Modal animations
const animateModalOpen = () => {
  const overlay = document.querySelector('.modal-dialog-overlay')
  if (!overlay) return

  const modal = overlay.querySelector('.modal-dialog')
  if (!modal) return

  // Animate overlay
  gsap.fromTo(overlay,
    { opacity: 0 },
    { opacity: 1, duration: 0.2, ease: 'power2.out' }
  )

  // Animate modal
  gsap.fromTo(modal,
    { y: 30, opacity: 0, scale: 0.95 },
    { y: 0, opacity: 1, scale: 1, duration: 0.35, ease: 'back.out(1.2)' }
  )
}

const animateModalClose = (callback?: () => void) => {
  const overlay = document.querySelector('.modal-dialog-overlay')
  if (!overlay) {
    callback?.()
    return
  }

  const modal = overlay.querySelector('.modal-dialog')

  const tl = gsap.timeline({
    onComplete: () => {
      callback?.()
    }
  })

  if (modal) {
    tl.to(modal, { y: 20, opacity: 0, scale: 0.98, duration: 0.2, ease: 'power2.in' })
  }
  tl.to(overlay, { opacity: 0, duration: 0.15, ease: 'power2.in' }, '<')
}

watch(() => props.state.isOpen, (newVal) => {
  if (newVal) {
    nextTick(() => animateModalOpen())
  }
})

const handleConfirm = () => {
  animateModalClose(() => {
    emit('confirm')
  })
}

const handleCancel = () => {
  animateModalClose(() => {
    emit('cancel')
  })
}

const handleOverlayClick = (e: MouseEvent) => {
  if (e.target === e.currentTarget && props.state.cancelText) {
    handleCancel()
  }
}
</script>

<template>
  <Teleport to="body">
    <div
      v-if="state.isOpen"
      class="modal-dialog-overlay"
      @click="handleOverlayClick"
    >
      <div class="modal-dialog">
        <div class="modal-dialog-header">
          <div class="modal-dialog-icon" :style="{ color: getIconColor() }">
            <component :is="getIcon()" :size="32" />
          </div>
          <button
            v-if="state.cancelText"
            class="modal-dialog-close"
            @click="handleCancel"
          >
            <X :size="20" />
          </button>
        </div>

        <div class="modal-dialog-content">
          <h3 class="modal-dialog-title">{{ state.title }}</h3>
          <p class="modal-dialog-message">{{ state.message }}</p>
        </div>

        <div class="modal-dialog-footer">
          <button
            v-if="state.cancelText"
            class="btn btn-secondary"
            @click="handleCancel"
          >
            {{ state.cancelText }}
          </button>
          <button
            class="btn"
            :class="state.cancelText ? 'btn-primary' : 'btn-primary'"
            @click="handleConfirm"
          >
            {{ state.confirmText }}
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<style scoped lang="scss">
@use '../styles/variables' as *;

.modal-dialog-overlay {
  position: fixed;
  inset: 0;
  z-index: 10000;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
  background: rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(4px);
}

.modal-dialog {
  width: 100%;
  max-width: 420px;
  background: $color-surface;
  border: 1px solid $color-border;
  border-radius: 16px;
  overflow: hidden;
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
}

.modal-dialog-header {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 1.5rem 1.5rem 0.5rem;
  position: relative;
}

.modal-dialog-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 64px;
  height: 64px;
  background: rgba($color-accent, 0.1);
  border-radius: 50%;
}

.modal-dialog-close {
  position: absolute;
  top: 1rem;
  right: 1rem;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  background: transparent;
  border: 1px solid $color-border;
  border-radius: 8px;
  color: $color-text-dim;
  cursor: pointer;
  transition: all 0.2s ease;

  &:hover {
    border-color: $color-accent;
    color: $color-accent;
  }
}

.modal-dialog-content {
  padding: 1rem 1.5rem 1.5rem;
  text-align: center;
}

.modal-dialog-title {
  font-family: $font-display;
  font-size: 1.25rem;
  font-weight: 600;
  margin: 0 0 0.75rem;
  color: $color-text;
}

.modal-dialog-message {
  font-size: 0.95rem;
  color: $color-text-dim;
  line-height: 1.6;
  margin: 0;
}

.modal-dialog-footer {
  display: flex;
  gap: 0.75rem;
  padding: 0 1.5rem 1.5rem;
  justify-content: center;
}

// Buttons
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 10px 24px;
  font-family: $font-body;
  font-size: 0.95rem;
  font-weight: 500;
  text-transform: lowercase;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  border: 1px solid transparent;
  min-width: 100px;

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
}

.btn-primary {
  background: $color-accent;
  color: $color-bg;
  border-color: $color-accent;

  &:hover:not(:disabled) {
    background: transparent;
    color: $color-accent;
  }
}

.btn-secondary {
  background: transparent;
  color: $color-text;
  border-color: $color-border;

  &:hover:not(:disabled) {
    border-color: $color-text;
  }
}

// Responsive
@media (max-width: 480px) {
  .modal-dialog-overlay {
    padding: 16px;
  }

  .modal-dialog-footer {
    flex-direction: column-reverse;

    .btn {
      width: 100%;
    }
  }
}
</style>
