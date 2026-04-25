<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { useAuth } from '@/composables/useAuth'
import AdminAlertModal from './AdminAlertModal.vue'

type AdminNavItem = {
  label: string
  viewName: string
  badge?: string
}

const emit = defineEmits<{
  (event: 'navigate', viewName: string): void
}>()

const route = useRoute()
const { user, isAuthenticated, logout } = useAuth()

const primaryNavItems: AdminNavItem[] = [
  { label: 'Сводка', viewName: 'dashboard' },
  { label: 'Хакатоны', viewName: 'admin-hackathons' },
]

const managementNavItems = computed<AdminNavItem[]>(() => [
  { label: 'Организаторы', viewName: 'admin-organizers' },
  { label: 'Жалобы', viewName: 'admin-reports' },
  ...(user.value?.isSuperuser ? [{ label: 'Пользователи', viewName: 'admin-users', badge: 'super' }] : []),
])

const activeRouteName = computed(() => {
  const routeName = typeof route.name === 'string' ? route.name : 'dashboard'
  return routeName === 'admin' ? 'dashboard' : routeName
})

const isActiveRoute = (viewName: string) => activeRouteName.value === viewName

const navigate = (viewName: string) => {
  emit('navigate', viewName)
}

const userInitial = computed(() => {
  const source = user.value?.name?.trim() || user.value?.email?.trim() || 'A'
  return source.charAt(0).toUpperCase()
})

const userRoleLabel = computed(() => {
  if (user.value?.isSuperuser) {
    return 'superuser'
  }

  if (user.value?.isStaff) {
    return 'staff'
  }

  return 'user'
})
</script>

<template>
  <div class="admin-layout">
    <div class="grid-lines" aria-hidden="true"></div>

    <aside class="sidebar">
      <div class="sidebar-header">
        <router-link to="/" class="logo" aria-label="Hackari">
          <span class="logo-text logo-hack">hack</span>
          <span class="logo-text logo-ari">ari</span>
          <span class="logo-cursor">_</span>
        </router-link>
        <span class="logo-badge">admin</span>
      </div>

      <nav class="sidebar-nav" aria-label="Административная навигация">
        <ul class="nav-list">
          <li
            v-for="item in primaryNavItems"
            :key="item.viewName"
            class="nav-item"
          >
            <button
              type="button"
              class="nav-link"
              :class="{ active: isActiveRoute(item.viewName) }"
              @click="navigate(item.viewName)"
            >
              <span class="nav-icon" aria-hidden="true">
                <svg
                  v-if="item.viewName === 'dashboard'"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                >
                  <rect x="3" y="3" width="8" height="8" rx="1" />
                  <rect x="13" y="3" width="8" height="5" rx="1" />
                  <rect x="13" y="10" width="8" height="11" rx="1" />
                  <rect x="3" y="13" width="8" height="8" rx="1" />
                </svg>
                <svg
                  v-else
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                >
                  <path d="M9 11l3 3L22 4" />
                  <path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11" />
                </svg>
              </span>
              <span class="nav-label">{{ item.label }}</span>
            </button>
          </li>

          <li class="nav-divider" aria-hidden="true"></li>

          <li
            v-for="item in managementNavItems"
            :key="item.viewName"
            class="nav-item"
          >
            <button
              type="button"
              class="nav-link"
              :class="{ active: isActiveRoute(item.viewName) }"
              @click="navigate(item.viewName)"
            >
              <span class="nav-icon" aria-hidden="true">
                <svg
                  v-if="item.viewName === 'admin-organizers'"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                >
                  <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />
                  <circle cx="9" cy="7" r="4" />
                  <path d="M23 21v-2a4 4 0 0 0-3-3.87" />
                  <path d="M16 3.13a4 4 0 0 1 0 7.75" />
                </svg>
                <svg
                  v-else-if="item.viewName === 'admin-reports'"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                >
                  <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" />
                  <line x1="12" y1="9" x2="12" y2="13" />
                  <line x1="12" y1="17" x2="12.01" y2="17" />
                </svg>
                <svg
                  v-else
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                >
                  <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />
                  <circle cx="9" cy="7" r="4" />
                  <path d="M23 21v-2a4 4 0 0 0-3-3.87" />
                  <path d="M16 3.13a4 4 0 0 1 0 7.75" />
                </svg>
              </span>
              <span class="nav-label">{{ item.label }}</span>
              <span v-if="item.badge" class="nav-badge">{{ item.badge }}</span>
            </button>
          </li>
        </ul>
      </nav>

      <div class="sidebar-footer">
        <div v-if="isAuthenticated && user" class="user-info">
          <div class="user-avatar">{{ userInitial }}</div>
          <div class="user-details">
            <span class="user-name">{{ user.name || user.email }}</span>
            <span class="user-role" :class="{ superuser: user.isSuperuser }">{{ userRoleLabel }}</span>
          </div>
        </div>

        <button
          type="button"
          class="logout-button"
          @click="logout"
          title="Выйти"
          aria-label="Выйти"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M10 17l5-5-5-5" />
            <path d="M15 12H3" />
            <path d="M21 3v18" />
          </svg>
        </button>
      </div>
    </aside>

    <main class="main-content">
      <slot />
    </main>

    <div class="coordinates">
      <span>hackari // admin shell</span>
      <span class="values">secure access</span>
    </div>

    <!-- Admin Modal -->
    <AdminAlertModal />
  </div>
</template>
<style scoped lang="scss">
.admin-layout {
  display: flex;
  min-height: 100vh;
  background: #0a0a0a;
  position: relative;
}

.grid-lines {
  position: fixed;
  inset: 0;
  pointer-events: none;
  z-index: 0;
  opacity: 0.03;

  &::before {
    content: '';
    position: absolute;
    inset: 0;
    background-image:
      linear-gradient(to right, #ffffff 1px, transparent 1px),
      linear-gradient(to bottom, #ffffff 1px, transparent 1px);
    background-size: 80px 80px;
  }
}

.sidebar {
  width: 260px;
  background: #111111;
  border-right: 1px solid rgba(255, 255, 255, 0.08);
  display: flex;
  flex-direction: column;
  position: fixed;
  height: 100vh;
  z-index: 10;
}

.sidebar-header {
  padding: 24px 20px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
}

.logo {
  display: flex;
  align-items: center;
  gap: 2px;
}

.logo-text {
  font-size: 20px;
  font-weight: 600;
  font-family: 'Unbounded', sans-serif;
  letter-spacing: -0.02em;
}

.logo-hack {
  color: #ffffff;
}

.logo-ari {
  color: #ffffff;
}

.logo-cursor {
  color: rgba(255, 255, 255, 0.5);
  animation: blink 1s step-end infinite;
}

@keyframes blink {
  50% { opacity: 0; }
}

.logo-badge {
  background: rgba(255, 255, 255, 0.08);
  color: rgba(255, 255, 255, 0.6);
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  font-family: 'JetBrains Mono', monospace;
}

.sidebar-nav {
  flex: 1;
  padding: 16px 12px;
  overflow-y: auto;
}

.nav-list {
  list-style: none;
  margin: 0;
  padding: 0;
}

.nav-item {
  margin-bottom: 2px;
}

.nav-divider {
  height: 1px;
  background: rgba(255, 255, 255, 0.08);
  margin: 16px 0;
}

.nav-link {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 14px;
  border-radius: 6px;
  color: rgba(255, 255, 255, 0.6);
  text-decoration: none;
  font-size: 14px;
  font-family: 'Onest', system-ui, sans-serif;
  transition: all 0.2s;
  background: none;
  border: none;
  cursor: pointer;
  width: 100%;
  text-align: left;

  &:hover {
    background: rgba(255, 255, 255, 0.04);
    color: #ffffff;
  }

  &.active {
    background: rgba(255, 255, 255, 0.08);
    color: #ffffff;
  }
}

.nav-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
}

.nav-label {
  flex: 1;
}

.nav-badge {
  background: rgba(255, 255, 255, 0.1);
  color: rgba(255, 255, 255, 0.7);
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 9px;
  font-weight: 600;
  font-family: 'JetBrains Mono', monospace;
}

.sidebar-footer {
  padding: 16px;
  border-top: 1px solid rgba(255, 255, 255, 0.08);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.user-avatar {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.08);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  font-weight: 600;
  color: #ffffff;
}

.user-details {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.user-name {
  font-size: 13px;
  font-weight: 500;
  color: #ffffff;
}

.user-role {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.5);
  font-family: 'JetBrains Mono', monospace;

  &.superuser {
    color: #ffffff;
    font-weight: 500;
  }
}

.logout-button {
  background: none;
  border: none;
  color: rgba(255, 255, 255, 0.4);
  cursor: pointer;
  padding: 8px;
  border-radius: 6px;
  transition: all 0.2s;

  &:hover {
    color: #ffffff;
    background: rgba(255, 255, 255, 0.08);
  }
}

.main-content {
  flex: 1;
  margin-left: 260px;
  padding: 32px;
  min-height: 100vh;
  position: relative;
  z-index: 1;
}

.coordinates {
  position: fixed;
  bottom: 20px;
  right: 24px;
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 2px;
  font-size: 11px;
  color: rgba(255, 255, 255, 0.3);
  font-family: 'JetBrains Mono', monospace;
  z-index: 5;
  pointer-events: none;

  .values {
    color: rgba(255, 255, 255, 0.5);
  }
}

.mono {
  font-family: 'JetBrains Mono', monospace;
}
</style>
