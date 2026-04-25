<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import { handleConfirm, handleCancel, handleOverlayClick } from '@/composables/useAdminModal'
import { modalState } from '@/composables/useAdminModal'

const handleKeydown = (e: KeyboardEvent) => {
  if (e.key === 'Escape') {
    handleCancel()
  } else if (e.key === 'Enter') {
    handleConfirm()
  }
}

onMounted(() => {
  document.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
  <Teleport to="body">
    <Transition name="modal-overlay">
      <div
        v-if="modalState.isOpen"
        class="admin-modal-overlay"
        @click="handleOverlayClick"
      >
        <div
          class="admin-modal"
          @click.stop
        >
          <div class="admin-modal-header">
            <div class="admin-modal-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" />
                <line x1="12" y1="9" x2="12" y2="13" />
                <line x1="12" y1="17" x2="12.01" y2="17" />
              </svg>
            </div>
            <h3 class="admin-modal-title">{{ modalState.title }}</h3>
          </div>

          <div class="admin-modal-body">
            <p class="admin-modal-message">{{ modalState.message }}</p>
          </div>

          <div class="admin-modal-footer">
            <button
              class="admin-btn admin-btn-secondary"
              @click="handleCancel"
            >
              {{ modalState.cancelText }}
            </button>
            <button
              class="admin-btn admin-btn-primary"
              @click="handleConfirm"
            >
              {{ modalState.confirmText }}
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped lang="scss">
.admin-modal-overlay {
  position: fixed;
  inset: 0;
  z-index: 10000;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(8px);
  padding: 20px;
}

.admin-modal {
  width: 100%;
  max-width: 400px;
  background: #111111;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  padding: 28px;
  box-shadow: 0 20px 40px -12px rgba(0, 0, 0, 0.5);
}

.admin-modal-header {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
}

.admin-modal-icon {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.08);
  border-radius: 50%;
  color: rgba(255, 255, 255, 0.7);

  svg {
    width: 24px;
    height: 24px;
  }
}

.admin-modal-title {
  font-size: 18px;
  font-weight: 600;
  color: #ffffff;
  text-align: center;
  margin: 0;
  font-family: 'Unbounded', sans-serif;
}

.admin-modal-body {
  margin-bottom: 24px;
}

.admin-modal-message {
  font-size: 14px;
  line-height: 1.6;
  color: rgba(255, 255, 255, 0.6);
  text-align: center;
  margin: 0;
}

.admin-modal-footer {
  display: flex;
  gap: 12px;
  justify-content: center;
}

.admin-btn {
  padding: 10px 20px;
  border-radius: 6px;
  font-family: inherit;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  border: none;
  outline: none;

  &:focus-visible {
    box-shadow: 0 0 0 2px currentColor;
  }
}

.admin-btn-primary {
  background: #ffffff;
  color: #0a0a0a;

  &:hover {
    background: rgba(255, 255, 255, 0.9);
  }
}

.admin-btn-secondary {
  background: transparent;
  color: rgba(255, 255, 255, 0.7);
  border: 1px solid rgba(255, 255, 255, 0.2);

  &:hover {
    background: rgba(255, 255, 255, 0.05);
    border-color: rgba(255, 255, 255, 0.3);
    color: #ffffff;
  }
}

// Vue transitions
.modal-overlay-enter-active,
.modal-overlay-leave-active {
  transition: opacity 0.3s ease;
}

.modal-overlay-enter-from,
.modal-overlay-leave-to {
  opacity: 0;
}
</style>
