<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { NotFoundAnimationController } from '@/utils/NotFoundAnimationController'
import { useNotFound } from '@/composables/useNotFound'

const router = useRouter()
const containerRef = ref<HTMLElement | null>(null)
let animationController: NotFoundAnimationController | null = null
const { setNotFound } = useNotFound()

// Navigation handlers
const handleGoHome = (): void => {
  router.push('/')
}

const handleGoBack = (): void => {
  router.back()
}

// Mouse move handler for parallax effect
const handleMouseMove = (e: MouseEvent): void => {
  animationController?.handleMouseMove(e.clientX, e.clientY)
}

onMounted(() => {
  setNotFound(true)
  if (containerRef.value) {
    animationController = new NotFoundAnimationController(containerRef.value, {
      duration: 1.2,
      ease: 'power4.out',
      stagger: 0.08,
      delay: 0.3,
    })
    animationController.init()

    window.addEventListener('mousemove', handleMouseMove)
  }
})

onUnmounted(() => {
  setNotFound(false)
  window.removeEventListener('mousemove', handleMouseMove)
  animationController?.destroy()
  animationController = null
})
</script>

<template>
  <div ref="containerRef" class="not-found">
    <!-- Background grid -->
    <div class="not-found__grid" />

    <!-- Decorative shapes -->
    <div class="not-found__decorations">
      <div class="not-found__shape not-found__shape--cube" />
      <div class="not-found__shape not-found__shape--ring" />
      <div class="not-found__shape not-found__shape--dot" />
    </div>

    <!-- Main content -->
    <div class="not-found__content">
      <!-- 404 Digits -->
      <div class="not-found__code">
        <span class="not-found__digit">4</span>
        <span class="not-found__digit not-found__digit--accent">0</span>
        <span class="not-found__digit">4</span>
      </div>

      <!-- Title -->
      <h1 class="not-found__title">
        страница не найдена
      </h1>

      <!-- Subtitle -->
      <p class="not-found__subtitle">
        похоже, вы забрели в неизведанные уголки интернета.
        <br>запрашиваемая страница не существует или была перемещена.
      </p>

      <!-- Error code info -->
      <div class="not-found__meta mono">
        <span class="not-found__meta-item">ERR_CODE: 404</span>
        <span class="not-found__meta-divider" />
        <span class="not-found__meta-item">STATUS: NOT_FOUND</span>
      </div>

      <!-- Actions -->
      <div class="not-found__actions">
        <button class="not-found__button" @click="handleGoHome">
          <span class="not-found__button-text">на главную</span>
          <span class="not-found__button-icon">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </span>
          <span class="not-found__button-bg" />
        </button>

        <button class="not-found__button not-found__button--secondary" @click="handleGoBack">
          <span class="not-found__button-text">назад</span>
          <span class="not-found__button-icon">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M10 19l-7-7m0 0l7-7m-7 7h18" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </span>
          <span class="not-found__button-bg" />
        </button>
      </div>
    </div>

    <!-- Corner decorations -->
    <div class="not-found__corner not-found__corner--top-left" />
    <div class="not-found__corner not-found__corner--top-right" />
    <div class="not-found__corner not-found__corner--bottom-left" />
    <div class="not-found__corner not-found__corner--bottom-right" />
  </div>
</template>

<style scoped lang="scss">
@use '@/styles/variables' as *;

// Block: not-found
.not-found {
  position: relative;
  min-height: 100vh;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(180deg, $color-bg 0%, darken($color-bg, 3%) 100%);
  overflow: hidden;
  padding: 40px 20px;
}

// Element: grid (background)
.not-found__grid {
  position: absolute;
  inset: 0;
  pointer-events: none;
  background-image:
    linear-gradient(rgba($color-accent, 0.03) 1px, transparent 1px),
    linear-gradient(90deg, rgba($color-accent, 0.03) 1px, transparent 1px);
  background-size: 60px 60px;
  opacity: 0.5;
}

// Element: decorations
.not-found__decorations {
  position: absolute;
  inset: 0;
  pointer-events: none;
  z-index: 1;
}

// Element: shape
.not-found__shape {
  position: absolute;
  opacity: 0.15;

  &--cube {
    top: 15%;
    right: 15%;
    width: 120px;
    height: 120px;
    border: 2px solid $color-accent;
    transform: rotate(15deg);

    &::before {
      content: '';
      position: absolute;
      inset: 20px;
      border: 1px solid $color-accent;
      transform: rotate(-15deg);
    }
  }

  &--ring {
    bottom: 20%;
    left: 10%;
    width: 80px;
    height: 80px;
    border: 3px solid $color-accent;
    border-radius: 50%;
    border-top-color: transparent;
    border-bottom-color: transparent;
  }

  &--dot {
    top: 30%;
    left: 25%;
    width: 8px;
    height: 8px;
    background: $color-accent;
    border-radius: 50%;
    box-shadow: 0 0 20px rgba($color-accent, 0.5);
  }
}

// Element: content
.not-found__content {
  position: relative;
  z-index: 10;
  text-align: center;
  max-width: 600px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 24px;
}

// Element: code (404 digits)
.not-found__code {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  perspective: 600px;
}

// Element: digit
.not-found__digit {
  font-family: $font-display;
  font-size: clamp(120px, 25vw, 200px);
  font-weight: 700;
  line-height: 1;
  color: $color-text;
  display: inline-block;
  transform-style: preserve-3d;
  will-change: transform;

  &--accent {
    color: $color-accent;
    text-shadow:
      0 0 40px rgba($color-accent, 0.3),
      0 0 80px rgba($color-accent, 0.1);
  }
}

// Element: title
.not-found__title {
  font-family: $font-display;
  font-size: clamp(24px, 5vw, 36px);
  font-weight: 600;
  color: $color-text;
  text-transform: lowercase;
  letter-spacing: -0.02em;
}

// Element: subtitle
.not-found__subtitle {
  font-family: $font-body;
  font-size: clamp(14px, 2.5vw, 16px);
  color: $color-text-dim;
  line-height: 1.6;
  max-width: 400px;

  br {
    display: block;
  }
}

// Element: meta
.not-found__meta {
  display: flex;
  align-items: center;
  gap: 16px;
  font-size: 12px;
  color: $color-text-muted;
  letter-spacing: 0.05em;
}

// Element: meta-item
.not-found__meta-item {
  font-family: $font-mono;
}

// Element: meta-divider
.not-found__meta-divider {
  width: 4px;
  height: 4px;
  background: $color-text-muted;
  border-radius: 50%;
}

// Element: actions
.not-found__actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 16px;
  margin-top: 16px;
}

// Element: button (стиль как на лендинге)
.not-found__button {
  position: relative;
  display: inline-flex;
  align-items: center;
  gap: 12px;
  padding: 16px 32px;
  background: transparent;
  border: 1px solid $color-accent;
  color: $color-accent;
  font-family: $font-body;
  font-size: 14px;
  font-weight: 500;
  letter-spacing: 0.02em;
  text-transform: lowercase;
  cursor: pointer;
  overflow: hidden;
  transition: all 0.4s $transition-smooth;

  &:hover {
    color: $color-bg;

    .not-found__button-bg {
      transform: scaleX(1);
    }

    .not-found__button-icon {
      transform: rotate(45deg);
    }
  }

  &:active {
    transform: scale(0.98);
  }

  &--secondary {
    border-color: $color-border;
    color: $color-text;

    .not-found__button-bg {
      background: rgba($color-text, 0.1);
    }

    &:hover {
      border-color: rgba($color-text, 0.3);
      color: $color-text;
    }
  }
}

.not-found__button-text {
  position: relative;
  z-index: 2;
}

.not-found__button-icon {
  position: relative;
  z-index: 2;
  display: flex;
  transition: transform 0.4s $transition-smooth;
}

.not-found__button-bg {
  position: absolute;
  inset: 0;
  background: $color-accent;
  transform: scaleX(0);
  transform-origin: left;
  transition: transform 0.4s $transition-smooth;
  z-index: 1;
}

// Element: corner
.not-found__corner {
  position: absolute;
  width: 40px;
  height: 40px;
  border: 2px solid rgba($color-accent, 0.2);
  pointer-events: none;

  &--top-left {
    top: 40px;
    left: 40px;
    border-right: none;
    border-bottom: none;
  }

  &--top-right {
    top: 40px;
    right: 40px;
    border-left: none;
    border-bottom: none;
  }

  &--bottom-left {
    bottom: 40px;
    left: 40px;
    border-right: none;
    border-top: none;
  }

  &--bottom-right {
    bottom: 40px;
    right: 40px;
    border-left: none;
    border-top: none;
  }
}

// Responsive styles
@media (max-width: $breakpoint-md) {
  .not-found {
    padding: 20px;
  }

  .not-found__corner {
    width: 30px;
    height: 30px;

    &--top-left,
    &--top-right {
      top: 20px;
    }

    &--bottom-left,
    &--bottom-right {
      bottom: 20px;
    }

    &--top-left,
    &--bottom-left {
      left: 20px;
    }

    &--top-right,
    &--bottom-right {
      right: 20px;
    }
  }

  .not-found__shape {
    &--cube {
      width: 60px;
      height: 60px;
      top: 10%;
      right: 10%;
    }

    &--ring {
      width: 50px;
      height: 50px;
      bottom: 15%;
      left: 5%;
    }

    &--dot {
      display: none;
    }
  }

  .not-found__content {
    gap: 20px;
  }

  .not-found__code {
    gap: 4px;
  }

  .not-found__meta {
    flex-direction: column;
    gap: 8px;

    &-divider {
      display: none;
    }
  }
}

// Reduced motion preference
@media (prefers-reduced-motion: reduce) {
  .not-found__digit,
  .not-found__shape {
    transition: none !important;
    animation: none !important;
  }
}
</style>
