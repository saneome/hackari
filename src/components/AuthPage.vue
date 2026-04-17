<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { gsap } from 'gsap'
import AuthForm from './AuthForm.vue'

const pageRef = ref<HTMLElement | null>(null)
const contentRef = ref<HTMLElement | null>(null)
const decorationRef = ref<HTMLElement | null>(null)

onMounted(() => {
  // Entrance animation
  const tl = gsap.timeline({ delay: 0.2 })

  if (decorationRef.value) {
    tl.fromTo(
      decorationRef.value,
      { opacity: 0, scale: 0.9 },
      { opacity: 1, scale: 1, duration: 1.2, ease: 'power3.out' },
      0
    )
  }

  if (contentRef.value) {
    tl.fromTo(
      contentRef.value.children,
      { y: 40, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.8, stagger: 0.1, ease: 'power3.out' },
      0.3
    )
  }
})
</script>

<template>
  <section ref="pageRef" class="auth-page">
    <!-- Background Decoration -->
    <div ref="decorationRef" class="page-decoration">
      <div class="grid-pattern" />
      <div class="glow-orb orb-1" />
      <div class="glow-orb orb-2" />
      <div class="geometric-lines">
        <div v-for="i in 8" :key="i" class="geo-line" />
      </div>
    </div>

    <!-- Corner Decorations -->
    <div class="corner top-left">
      <span class="mono text-muted">v2.024</span>
    </div>
    <div class="corner top-right">
      <span class="mono text-muted">SECURE</span>
    </div>
    <div class="corner bottom-left">
      <div class="status-dot" />
      <span class="mono text-muted">AUTH SYSTEM</span>
    </div>
    <div class="corner bottom-right">
      <span class="coordinates mono text-muted">
        POS: {{ Math.random().toString(36).substring(2, 8).toUpperCase() }}
      </span>
    </div>

    <!-- Main Content -->
    <div ref="contentRef" class="page-content">
      <!-- Logo -->
      <div class="logo-section">
        <router-link to="/" class="logo-link">
          <span class="logo-text">
            hack<span class="logo-accent">ari</span>
          </span>
          <span class="cursor cursor-blink">_</span>
        </router-link>
        <p class="logo-tagline text-dim">хакатоны. команды. результат.</p>
      </div>

      <!-- Auth Form -->
      <div class="form-container">
        <AuthForm />
      </div>

      <!-- Security Badge -->
      <div class="security-badge">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <span class="badge-text">256-bit encryption</span>
      </div>
    </div>
  </section>
</template>

<style scoped lang="scss">
.auth-page {
  position: relative;
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(180deg, #0a0a0a 0%, #0c0c0c 100%);
  overflow: hidden;
}

// Background Decoration
.page-decoration {
  position: absolute;
  inset: 0;
  pointer-events: none;
}

.grid-pattern {
  position: absolute;
  inset: 0;
  background-image:
    linear-gradient(rgba($color-accent, 0.03) 1px, transparent 1px),
    linear-gradient(90deg, rgba($color-accent, 0.03) 1px, transparent 1px);
  background-size: 60px 60px;
  mask-image: radial-gradient(ellipse at center, black 40%, transparent 70%);
}

.glow-orb {
  position: absolute;
  border-radius: 50%;
  filter: blur(80px);
  opacity: 0.15;

  &.orb-1 {
    width: 400px;
    height: 400px;
    background: $color-accent;
    top: 10%;
    right: 10%;
    animation: float 8s ease-in-out infinite;
  }

  &.orb-2 {
    width: 300px;
    height: 300px;
    background: $color-secondary;
    bottom: 10%;
    left: 10%;
    animation: float 10s ease-in-out infinite reverse;
  }
}

.geometric-lines {
  position: absolute;
  inset: 0;
  overflow: hidden;
}

.geo-line {
  position: absolute;
  height: 1px;
  background: linear-gradient(90deg, transparent, rgba($color-accent, 0.1), transparent);

  &:nth-child(1) { top: 20%; left: -10%; width: 40%; transform: rotate(15deg); }
  &:nth-child(2) { top: 40%; right: -10%; width: 35%; transform: rotate(-10deg); }
  &:nth-child(3) { top: 60%; left: -5%; width: 30%; transform: rotate(5deg); }
  &:nth-child(4) { bottom: 30%; right: -5%; width: 45%; transform: rotate(-15deg); }
  &:nth-child(5) { top: 80%; left: 20%; width: 25%; transform: rotate(25deg); }
  &:nth-child(6) { top: 10%; right: 30%; width: 20%; transform: rotate(-20deg); }
  &:nth-child(7) { bottom: 20%; left: 40%; width: 35%; transform: rotate(8deg); }
  &:nth-child(8) { top: 50%; right: 20%; width: 15%; transform: rotate(-5deg); }
}

@keyframes float {
  0%, 100% { transform: translateY(0) scale(1); }
  50% { transform: translateY(-30px) scale(1.05); }
}

// Corners
.corner {
  position: absolute;
  font-size: 11px;
  letter-spacing: 0.1em;

  &.top-left { top: 40px; left: 40px; }
  &.top-right { top: 40px; right: 40px; }
  &.bottom-left {
    bottom: 40px;
    left: 40px;
    display: flex;
    align-items: center;
    gap: 8px;
  }
  &.bottom-right {
    bottom: 40px;
    right: 40px;
  }

  @media (max-width: 768px) {
    &.top-left, &.top-right {
      top: 20px;
    }
    &.bottom-left, &.bottom-right {
      display: none;
    }
  }
}

.status-dot {
  width: 6px;
  height: 6px;
  background: $color-accent;
  border-radius: 50%;
  animation: pulse 2s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; transform: scale(1); }
  50% { opacity: 0.5; transform: scale(0.8); }
}

// Content
.page-content {
  position: relative;
  z-index: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 32px;
  padding: 24px 40px;
  width: 100%;
  max-width: 560px;

  @media (max-width: 480px) {
    padding: 24px 16px;
    max-width: 100%;
  }
}

// Logo Section
.logo-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}

.logo-link {
  text-decoration: none;
  display: flex;
  align-items: center;
  gap: 4px;
}

.logo-text {
  font-family: $font-display;
  font-size: 32px;
  font-weight: 600;
  letter-spacing: -0.02em;
  color: $color-text;
}

.logo-accent {
  color: $color-accent;
}

.cursor {
  font-family: $font-display;
  font-weight: 300;
  color: $color-accent;
  animation: blink 1.2s step-end infinite;
}

@keyframes blink {
  50% { opacity: 0; }
}

.logo-tagline {
  font-size: 13px;
  letter-spacing: 0.05em;
}

// Form Container
.form-container {
  background: rgba($color-surface, 0.4);
  border: 1px solid $color-border;
  border-radius: 16px;
  padding: 32px 40px;
  backdrop-filter: blur(10px);
  min-width: 400px;
  max-height: 85vh;
  overflow-y: auto;

  &::-webkit-scrollbar {
    width: 4px;
  }

  &::-webkit-scrollbar-track {
    background: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background: rgba($color-accent, 0.2);
    border-radius: 2px;
  }

  @media (max-width: 480px) {
    min-width: auto;
    width: 100%;
    padding: 24px 16px;
    max-height: 90vh;
  }
}

// Security Badge
.security-badge {
  display: flex;
  align-items: center;
  gap: 8px;
  color: $color-text-muted;
  font-size: 11px;
  letter-spacing: 0.05em;

  svg {
    opacity: 0.7;
  }
}
</style>
