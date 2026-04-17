<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { gsap } from 'gsap'
import { ScrollTrigger } from 'gsap/ScrollTrigger'
import { Menu, X } from 'lucide-vue-next'

gsap.registerPlugin(ScrollTrigger)

const isScrolled = ref(false)
const isMobileMenuOpen = ref(false)

const navItems = [
  { label: 'как это работает', href: '#how-it-works' },
  { label: 'возможности', href: '#features' },
  { label: 'статистика', href: '#stats' },
]

let scrollTriggerInstance: ScrollTrigger | null = null

onMounted(() => {
  // Track scroll position for header styling
  scrollTriggerInstance = ScrollTrigger.create({
    trigger: document.body,
    start: 'top -100',
    end: 'max',
    onUpdate: (self) => {
      isScrolled.value = self.progress > 0
    },
  })

  // Entrance animation
  gsap.fromTo('.header-content',
    { y: -30, opacity: 0 },
    { y: 0, opacity: 1, duration: 0.8, ease: 'power3.out', delay: 0.5 }
  )
})

onUnmounted(() => {
  scrollTriggerInstance?.kill()
})

const scrollToSection = (href: string) => {
  const element = document.querySelector(href)
  if (element) {
    element.scrollIntoView({ behavior: 'smooth' })
    isMobileMenuOpen.value = false
  }
}
</script>

<template>
  <header class="app-header" :class="{ 'app-header--scrolled': isScrolled }">
    <div class="header-content">
      <!-- Logo -->
      <router-link
        to="/"
        class="logo"
        active-class=""
        exact-active-class=""
      >
        <span class="logo-text logo-text--hack">hack</span>
        <span class="logo-text logo-text--accent">ari</span>
        <span class="logo-cursor">_</span>
      </router-link>

      <!-- Desktop Navigation -->
      <nav class="desktop-nav">
        <button
          v-for="item in navItems"
          :key="item.href"
          class="nav-link"
          @click="scrollToSection(item.href)"
        >
          {{ item.label }}
        </button>
      </nav>

      <!-- Auth Button (Desktop) -->
      <div class="header-actions">
        <router-link to="/auth" class="auth-link">
          <span class="auth-text">войти</span>
          <span class="auth-bg" />
        </router-link>

        <!-- Mobile Menu Toggle -->
        <button
          class="mobile-toggle"
          @click="isMobileMenuOpen = !isMobileMenuOpen"
        >
          <component :is="isMobileMenuOpen ? X : Menu" :size="24" />
        </button>
      </div>
    </div>

    <!-- Mobile Menu -->
    <transition name="slide">
      <div v-if="isMobileMenuOpen" class="mobile-menu">
        <nav class="mobile-nav">
          <button
            v-for="item in navItems"
            :key="item.href"
            class="mobile-nav-link"
            @click="scrollToSection(item.href)"
          >
            {{ item.label }}
          </button>
        </nav>
        <router-link to="/auth" class="mobile-auth-link" @click="isMobileMenuOpen = false">
          войти
        </router-link>
      </div>
    </transition>
  </header>
</template>

<style scoped lang="scss">
.app-header {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 100;
  padding: 20px 40px;
  transition: all 0.4s $transition-smooth;

  &--scrolled {
    background: rgba($color-bg, 0.8);
    backdrop-filter: blur(20px);
    border-bottom: 1px solid rgba($color-border, 0.5);
    padding: 16px 40px;

    .logo-text {
      opacity: 0.7;
    }

    .logo-cursor {
      animation-duration: 0.8s;
    }
  }

  @media (max-width: 768px) {
    padding: 16px 20px;
  }
}

.header-content {
  display: grid;
  grid-template-columns: 1fr auto 1fr;
  align-items: center;
  max-width: 1400px;
  margin: 0 auto;
}

// Logo
.logo {
  display: flex;
  align-items: center;
  gap: 2px;
  text-decoration: none;
  transition: transform 0.3s ease;
  flex-shrink: 0;

  &:hover {
    transform: translateX(4px);
  }

  // Prevent router-link-active from affecting layout
  &.router-link-active,
  &.router-link-exact-active {
    font-weight: inherit;
    color: inherit;
  }
}

.logo-text {
  font-family: $font-display;
  font-size: 24px;
  font-weight: 600;
  color: $color-text;
  transition: opacity 0.3s ease, transform 0.3s ease;

  &--accent {
    color: $color-accent;
  }
}

.logo-cursor {
  font-family: $font-display;
  color: $color-accent;
  animation: blink 1.2s step-end infinite;
  font-weight: 300;
  font-size: 24px;
  transition: opacity 0.3s ease;
}

@keyframes blink {
  50% { opacity: 0; }
}

// Desktop Navigation
.desktop-nav {
  display: flex;
  align-items: center;
  gap: 8px;

  @media (max-width: 768px) {
    display: none;
  }
}

.nav-link {
  background: transparent;
  border: none;
  color: $color-text-dim;
  font-family: $font-body;
  font-size: 13px;
  font-weight: 500;
  padding: 8px 16px;
  cursor: pointer;
  position: relative;
  transition: color 0.3s ease;
  text-transform: lowercase;

  &::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 50%;
    width: 0;
    height: 1px;
    background: $color-accent;
    transition: all 0.3s ease;
    transform: translateX(-50%);
  }

  &:hover {
    color: $color-text;

    &::after {
      width: calc(100% - 32px);
    }
  }
}

// Auth Button
.header-actions {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 16px;

  @media (max-width: 768px) {
    gap: 8px;
  }
}

.auth-link {
  position: relative;
  display: inline-flex;
  align-items: center;
  padding: 10px 24px;
  border: 1px solid $color-border;
  color: $color-text;
  font-family: $font-body;
  font-size: 13px;
  font-weight: 500;
  text-decoration: none;
  text-transform: lowercase;
  overflow: hidden;
  transition: color 0.4s $transition-smooth;

  &:hover {
    color: $color-bg;
    border-color: $color-accent;

    .auth-bg {
      transform: scaleX(1);
    }
  }
}

.auth-text {
  position: relative;
  z-index: 2;
}

.auth-bg {
  position: absolute;
  inset: 0;
  background: $color-accent;
  transform: scaleX(0);
  transform-origin: left;
  transition: transform 0.4s $transition-smooth;
  z-index: 1;
}

// Mobile Toggle
.mobile-toggle {
  display: none;
  background: transparent;
  border: none;
  color: $color-text;
  cursor: pointer;
  padding: 8px;
  margin-left: 8px;

  @media (max-width: 768px) {
    display: flex;
  }
}

// Mobile Menu
.mobile-menu {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  background: rgba($color-bg, 0.95);
  backdrop-filter: blur(20px);
  border-bottom: 1px solid $color-border;
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.mobile-nav {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.mobile-nav-link {
  background: transparent;
  border: none;
  color: $color-text-dim;
  font-family: $font-body;
  font-size: 16px;
  padding: 12px;
  text-align: left;
  cursor: pointer;
  transition: color 0.3s ease;
  text-transform: lowercase;

  &:hover {
    color: $color-text;
  }
}

.mobile-auth-link {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 14px;
  background: $color-accent;
  color: $color-bg;
  font-family: $font-body;
  font-size: 14px;
  font-weight: 500;
  text-decoration: none;
  text-transform: lowercase;
  transition: opacity 0.3s ease;

  &:hover {
    opacity: 0.9;
  }
}

// Transitions
.slide-enter-active,
.slide-leave-active {
  transition: all 0.3s $transition-smooth;
}

.slide-enter-from,
.slide-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}
</style>
