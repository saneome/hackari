<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { gsap } from 'gsap'
import { ScrollTrigger } from 'gsap/ScrollTrigger'
import { Menu, X, LogOut, User as UserIcon, LayoutDashboard, PlusCircle, Settings, ChevronLeft } from 'lucide-vue-next'
import { useAuth } from '@/composables/useAuth'
import { useModal } from '@/composables/useModal'
import { organizerApi } from '@/services/api'

gsap.registerPlugin(ScrollTrigger)

const { alert } = useModal()

const router = useRouter()
const route = useRoute()
const { user, isAuthenticated, logout } = useAuth()

const isScrolled = ref(false)
const isMobileMenuOpen = ref(false)

const navItems = computed(() => [
  { label: 'дашборд', href: '/organizers/dashboard', icon: LayoutDashboard },
  { label: 'создать хакатон', href: '/hackathons/create', icon: PlusCircle },
  { label: 'профиль', href: '/organizers/profile', icon: Settings },
])

const isOrganizerRoute = computed(() => {
  return route.path.startsWith('/organizers') || route.path.startsWith('/hackathons/create')
})

let scrollTriggerInstance: ScrollTrigger | null = null

onMounted(() => {
  scrollTriggerInstance = ScrollTrigger.create({
    trigger: document.body,
    start: 'top -100',
    end: 'max',
    onUpdate: (self) => {
      isScrolled.value = self.progress > 0
    },
  })

  gsap.fromTo('.organizer-header-content',
    { y: -30, opacity: 0 },
    { y: 0, opacity: 1, duration: 0.8, ease: 'power3.out', delay: 0.3 }
  )
})

onUnmounted(() => {
  scrollTriggerInstance?.kill()
})

const goBack = () => {
  router.push('/')
}

const isActiveRoute = (href: string) => {
  return route.path === href
}

const handleCreateHackathon = async () => {
  const res = await organizerApi.getMyOrganizer()
  if (res.data && res.data.is_verified === true) {
    router.push('/hackathons/create')
    return
  }
  await alert({
    title: 'Профиль не верифицирован',
    message: 'Ваш профиль организатора не верифицирован. Создание хакатонов доступно только после прохождения верификации.',
    type: 'warning',
  })
}
</script>

<template>
  <header class="organizer-header" :class="{ 'organizer-header--scrolled': isScrolled }">
    <div class="organizer-header-content">
      <!-- Back to site + Logo -->
      <div class="header-left">
        <button class="back-btn" @click="goBack" title="На главную">
          <ChevronLeft :size="18" />
          <span class="back-text">на сайт</span>
        </button>
        <div class="divider" />
        <router-link to="/organizers/dashboard" class="logo">
          <span class="logo-text logo-text--org">орг</span>
          <span class="logo-text logo-text--accent">панель</span>
          <span class="logo-cursor">_</span>
        </router-link>
      </div>

      <!-- Desktop Navigation -->
      <nav class="desktop-nav">
        <template v-for="item in navItems" :key="item.href">
          <button
            v-if="item.href === '/hackathons/create'"
            type="button"
            class="nav-link"
            :class="{ 'nav-link--active': isActiveRoute(item.href) }"
            @click="handleCreateHackathon"
          >
            <component :is="item.icon" :size="16" class="nav-icon" />
            <span>{{ item.label }}</span>
          </button>
          <router-link
            v-else
            :to="item.href"
            class="nav-link"
            :class="{ 'nav-link--active': isActiveRoute(item.href) }"
          >
            <component :is="item.icon" :size="16" class="nav-icon" />
            <span>{{ item.label }}</span>
          </router-link>
        </template>
      </nav>

      <!-- Auth Section -->
      <div class="header-actions">
        <template v-if="isAuthenticated && user">
          <div class="user-info">
            <UserIcon :size="16" class="user-icon" />
            <span class="user-name">{{ user.name || user.email }}</span>
          </div>
          <button class="logout-btn" @click="logout">
            <LogOut :size="16" />
          </button>
        </template>

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
        <template v-for="item in navItems" :key="item.href">
          <button
            v-if="item.href === '/hackathons/create'"
            type="button"
            class="mobile-nav-link"
            :class="{ 'mobile-nav-link--active': isActiveRoute(item.href) }"
            @click="handleCreateHackathon(); isMobileMenuOpen = false"
          >
            <component :is="item.icon" :size="18" />
            <span>{{ item.label }}</span>
          </button>
          <router-link
            v-else
            :to="item.href"
            class="mobile-nav-link"
            :class="{ 'mobile-nav-link--active': isActiveRoute(item.href) }"
            @click="isMobileMenuOpen = false"
          >
            <component :is="item.icon" :size="18" />
            <span>{{ item.label }}</span>
          </router-link>
        </template>

        <div class="mobile-divider" />

        <button class="mobile-back-btn" @click="goBack; isMobileMenuOpen = false">
          <ChevronLeft :size="18" />
          <span>вернуться на сайт</span>
        </button>

        <template v-if="isAuthenticated && user">
          <div class="mobile-user-info">
            <UserIcon :size="16" />
            <span>{{ user.name || user.email }}</span>
          </div>
          <button class="mobile-logout-btn" @click="logout; isMobileMenuOpen = false">
            <LogOut :size="16" />
            выйти
          </button>
        </template>
      </div>
    </transition>
  </header>
</template>

<style scoped lang="scss">
@use '../styles/variables' as *;

.organizer-header {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 100;
  padding: 16px 40px;
  transition: all 0.4s $transition-smooth;
  background: transparent;

  &--scrolled {
    background: rgba($color-bg, 0.9);
    backdrop-filter: blur(20px);
    border-bottom: 1px solid rgba($color-border, 0.5);
    padding: 12px 40px;

    .logo-cursor {
      animation-duration: 0.8s;
    }
  }

  @media (max-width: 768px) {
    padding: 12px 20px;
  }
}

.organizer-header-content {
  display: grid;
  grid-template-columns: 1fr auto 1fr;
  align-items: center;
  gap: 2rem;
  max-width: 1400px;
  margin: 0 auto;

  @media (max-width: 768px) {
    grid-template-columns: 1fr auto;
    gap: 1rem;
  }
}

// Left section: back button + logo
.header-left {
  display: flex;
  align-items: center;
  gap: 1rem;

  @media (max-width: 768px) {
    .back-btn .back-text,
    .divider {
      display: none;
    }
  }
}

.back-btn {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 8px 12px;
  background: transparent;
  border: 1px solid $color-border;
  border-radius: 6px;
  color: $color-text-dim;
  font-family: $font-body;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.3s ease;
  text-transform: lowercase;

  &:hover {
    border-color: $color-accent;
    color: $color-accent;
  }
}

.divider {
  width: 1px;
  height: 24px;
  background: rgba($color-border, 0.5);
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
}

.logo-text {
  font-family: $font-display;
  font-size: 22px;
  font-weight: 600;
  color: $color-text;
  transition: opacity 0.3s ease;

  &--accent {
    color: $color-accent;
  }

  &--org {
    opacity: 0.7;
  }
}

.logo-cursor {
  font-family: $font-display;
  color: $color-accent;
  animation: blink 1.2s step-end infinite;
  font-weight: 300;
  font-size: 22px;
}

@keyframes blink {
  50% { opacity: 0; }
}

// Desktop Navigation
.desktop-nav {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;

  @media (max-width: 768px) {
    display: none;
  }
}

.nav-link {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 18px;
  background: transparent;
  border: 1px solid transparent;
  border-radius: 8px;
  color: $color-text-dim;
  font-family: $font-body;
  font-size: 14px;
  font-weight: 500;
  text-decoration: none;
  cursor: pointer;
  transition: all 0.3s ease;
  text-transform: lowercase;

  .nav-icon {
    opacity: 0.7;
    transition: opacity 0.3s ease;
  }

  &:hover {
    color: $color-text;
    background: rgba($color-border, 0.3);

    .nav-icon {
      opacity: 1;
    }
  }

  &--active {
    color: $color-accent;
    border-color: $color-accent;
    background: rgba($color-accent, 0.05);

    .nav-icon {
      opacity: 1;
      color: $color-accent;
    }

    &:hover {
      background: rgba($color-accent, 0.08);
    }
  }
}

// Auth Section
.header-actions {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 14px;
  background: rgba($color-border, 0.3);
  border: 1px solid rgba($color-border, 0.5);
  border-radius: 6px;
  color: $color-text;
  font-family: $font-body;
  font-size: 14px;
  font-weight: 500;

  @media (max-width: 768px) {
    display: none;
  }
}

.user-icon {
  color: $color-accent;
  flex-shrink: 0;
}

.user-name {
  max-width: 120px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.logout-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 10px;
  background: transparent;
  border: 1px solid $color-border;
  border-radius: 6px;
  color: $color-text-dim;
  cursor: pointer;
  transition: all 0.3s ease;

  &:hover {
    color: $color-text;
    border-color: $color-accent;
    background: rgba($color-accent, 0.1);
  }

  @media (max-width: 768px) {
    display: none;
  }
}

// Mobile Toggle
.mobile-toggle {
  display: none;
  background: transparent;
  border: 1px solid $color-border;
  border-radius: 6px;
  color: $color-text;
  cursor: pointer;
  padding: 8px;
  transition: border-color 0.3s ease;

  &:hover {
    border-color: $color-accent;
  }

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
  background: rgba($color-bg, 0.98);
  backdrop-filter: blur(20px);
  border-bottom: 1px solid $color-border;
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.mobile-nav-link {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px 16px;
  background: transparent;
  border: 1px solid transparent;
  border-radius: 8px;
  color: $color-text-dim;
  font-family: $font-body;
  font-size: 15px;
  text-decoration: none;
  cursor: pointer;
  transition: all 0.3s ease;
  text-transform: lowercase;

  &:hover {
    color: $color-text;
    background: rgba($color-border, 0.3);
  }

  &--active {
    color: $color-accent;
    border-color: $color-accent;
    background: rgba($color-accent, 0.05);
  }
}

.mobile-divider {
  height: 1px;
  background: $color-border;
  margin: 8px 0;
}

.mobile-back-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 14px 16px;
  background: transparent;
  border: 1px solid $color-border;
  border-radius: 8px;
  color: $color-text-dim;
  font-family: $font-body;
  font-size: 15px;
  cursor: pointer;
  transition: all 0.3s ease;
  text-transform: lowercase;

  &:hover {
    border-color: $color-accent;
    color: $color-accent;
  }
}

.mobile-user-info {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  border: 1px solid $color-border;
  border-radius: 8px;
  font-size: 14px;
  color: $color-text-dim;
  margin-top: 8px;

  span {
    color: $color-text;
  }
}

.mobile-logout-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 14px;
  background: transparent;
  border: 1px solid $color-border;
  border-radius: 8px;
  color: $color-text-dim;
  font-family: $font-body;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;

  &:hover {
    color: $color-error;
    border-color: $color-error;
  }
}

// Transitions
.slide-enter-active, .slide-leave-active {
  transition: all 0.3s $transition-smooth;
}

.slide-enter-from, .slide-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}
</style>
