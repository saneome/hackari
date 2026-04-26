<script setup lang="ts">
import { ref, onMounted } from 'vue'

const visible = ref(false)

onMounted(() => {
  const accepted = localStorage.getItem('cookie_consent')
  if (!accepted) {
    visible.value = true
  }
})

const accept = () => {
  localStorage.setItem('cookie_consent', 'true')
  visible.value = false
}
</script>

<template>
  <transition name="slide-up">
    <div v-if="visible" class="cookie-banner">
      <div class="cookie-content">
        <p class="cookie-text">
          Мы используем cookie для улучшения работы сайта, персонализации контента и анализа трафика.
        </p>
        <button class="cookie-btn" @click="accept">
          Понятно
        </button>
      </div>
    </div>
  </transition>
</template>

<style scoped lang="scss">
.cookie-banner {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: 1000;
  background: rgba($color-surface, 0.95);
  backdrop-filter: blur(12px);
  border-top: 1px solid $color-border;
  padding: 16px 24px;
}

.cookie-content {
  max-width: 1200px;
  margin: 0 auto;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.cookie-text {
  font-size: 13px;
  line-height: 1.5;
  color: $color-text-dim;
  margin: 0;
}

.cookie-btn {
  flex-shrink: 0;
  padding: 8px 20px;
  background: $color-accent;
  color: $color-bg;
  border: none;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s ease;

  &:hover {
    background: lighten($color-accent, 8%);
  }
}

@media (max-width: 640px) {
  .cookie-content {
    flex-direction: column;
    text-align: center;
  }

  .cookie-btn {
    width: 100%;
  }
}

.slide-up-enter-active,
.slide-up-leave-active {
  transition: transform 0.4s $transition-smooth, opacity 0.4s ease;
}

.slide-up-enter-from,
.slide-up-leave-to {
  transform: translateY(100%);
  opacity: 0;
}
</style>
