<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useAuth } from '@/composables/useAuth'
import { adminApi } from '@/services/api'
import type { DashboardStats as DashboardStatsResponse } from '@/services/api'

type StatCard = {
  key: 'hackathons' | 'organizers' | 'reports'
  label: string
  value: number
  hint: string
}

const { user } = useAuth()

const stats = ref<DashboardStatsResponse | null>(null)
const isLoading = ref(true)
const errorMessage = ref('')

const isSuperuser = computed(() => user.value?.isSuperuser ?? false)

const statCards = computed<StatCard[]>(() => [
  {
    key: 'hackathons',
    label: 'Хакатоны на модерации',
    value: stats.value?.pending_hackathons_count ?? 0,
    hint: 'Ожидают проверки',
  },
  {
    key: 'organizers',
    label: 'Организаторы без проверки',
    value: stats.value?.unverified_organizers_count ?? 0,
    hint: 'Нуждаются в подтверждении',
  },
  {
    key: 'reports',
    label: 'Открытые жалобы',
    value: stats.value?.open_reports_count ?? 0,
    hint: 'Требуют реакции',
  },
])

const loadStats = async () => {
  isLoading.value = true
  errorMessage.value = ''

  try {
    const response = await adminApi.getDashboardStats()

    if (response.error) {
      errorMessage.value = response.error
      return
    }

    stats.value = response.data ?? null
  } catch {
    errorMessage.value = 'Не удалось загрузить статистику'
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  void loadStats().then(() => {
    requestAnimationFrame(() => {
      document.querySelectorAll('.animate-in').forEach((el) => {
        el.classList.add('animate-visible')
      })
    })
  })
})
</script>

<template>
  <section class="dashboard-stats">
    <header class="page-header animate-in" style="animation-delay: 0ms">
      <h1 class="page-title">Панель управления</h1>
      <p class="page-subtitle">Краткая сводка по модерации и очередям</p>
    </header>

    <div v-if="errorMessage" class="alert alert-error animate-in" style="animation-delay: 50ms">
      {{ errorMessage }}
    </div>

    <div class="stats-grid animate-in" style="animation-delay: 150ms">
      <article
        v-for="(card, index) in statCards"
        :key="card.key"
        class="stat-card row-animate"
        :style="{ animationDelay: `${200 + index * 30}ms` }"
      >
        <div class="card-decoration"></div>
        <div class="card-content">
          <div class="card-header">
            <span class="card-label">{{ card.label }}</span>
            <div
              class="card-icon"
              :class="{
                'hackathon-icon': card.key === 'hackathons',
                'organizer-icon': card.key === 'organizers',
                'report-icon': card.key === 'reports',
              }"
            >
              <svg
                v-if="card.key === 'hackathons'"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
              >
                <rect x="4" y="5" width="16" height="14" rx="2" />
                <path d="M8 3v4" />
                <path d="M16 3v4" />
                <path d="M4 10h16" />
                <path d="M8 14h4" />
              </svg>
              <svg
                v-else-if="card.key === 'organizers'"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
              >
                <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />
                <circle cx="9" cy="8" r="4" />
                <path d="M23 21v-2a4 4 0 0 0-3-3.87" />
                <path d="M16 3.13a4 4 0 0 1 0 7.75" />
              </svg>
              <svg
                v-else
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
              >
                <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" />
                <line x1="12" y1="9" x2="12" y2="13" />
                <line x1="12" y1="17" x2="12.01" y2="17" />
              </svg>
            </div>
          </div>

          <div v-if="isLoading" class="card-loading">
            <div class="spinner"></div>
          </div>
          <template v-else>
            <div class="card-value">
              <span class="stat-number">{{ card.value.toLocaleString('ru-RU') }}</span>
              <span class="stat-suffix">шт</span>
            </div>
            <div class="card-footer">
              <span class="stat-hint">{{ card.hint }}</span>
            </div>
          </template>
        </div>
      </article>
    </div>

    <section class="quick-actions animate-in" style="animation-delay: 300ms">
      <h2 class="section-title">Быстрые действия</h2>
      <div class="actions-grid">
        <router-link :to="{ name: 'admin' }" class="action-card">
          <div class="action-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="4" y="5" width="16" height="14" rx="2" />
              <path d="M8 3v4" />
              <path d="M16 3v4" />
              <path d="M4 10h16" />
            </svg>
          </div>
          <span class="action-label">Общая сводка</span>
          <span class="action-hint">Вернуться к главной панели</span>
        </router-link>

        <router-link :to="{ name: 'admin-hackathons' }" class="action-card">
          <div class="action-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="4" y="5" width="16" height="14" rx="2" />
              <path d="M8 3v4" />
              <path d="M16 3v4" />
              <path d="M4 10h16" />
              <path d="M8 14h4" />
            </svg>
          </div>
          <span class="action-label">Модерация хакатонов</span>
          <span class="action-hint">Проверить новые заявки</span>
        </router-link>

        <router-link :to="{ name: 'admin-organizers' }" class="action-card">
          <div class="action-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />
              <circle cx="9" cy="8" r="4" />
              <path d="M23 21v-2a4 4 0 0 0-3-3.87" />
              <path d="M16 3.13a4 4 0 0 1 0 7.75" />
            </svg>
          </div>
          <span class="action-label">Верификация организаторов</span>
          <span class="action-hint">Подтвердить новые профили</span>
        </router-link>

        <router-link :to="{ name: 'admin-reports' }" class="action-card">
          <div class="action-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" />
              <line x1="12" y1="9" x2="12" y2="13" />
              <line x1="12" y1="17" x2="12.01" y2="17" />
            </svg>
          </div>
          <span class="action-label">Жалобы</span>
          <span class="action-hint">Разобрать открытые обращения</span>
        </router-link>

        <router-link v-if="isSuperuser" :to="{ name: 'admin-users' }" class="action-card">
          <div class="action-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />
              <circle cx="9" cy="7" r="4" />
              <path d="M23 21v-2a4 4 0 0 0-3-3.87" />
              <path d="M16 3.13a4 4 0 0 1 0 7.75" />
            </svg>
          </div>
          <span class="action-label">Пользователи</span>
          <span class="action-hint">Только для superuser</span>
        </router-link>
      </div>
    </section>
  </section>
</template>
<style scoped lang="scss">
.dashboard-stats {
  max-width: 900px;
}

.page-header {
  margin-bottom: 32px;
}

.page-title {
  font-size: 32px;
  font-weight: 600;
  color: #ffffff;
  margin: 0 0 8px;
  font-family: 'Unbounded', sans-serif;
}

.page-subtitle {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.5);
  margin: 0;
}

.alert {
  padding: 12px 16px;
  border-radius: 6px;
  margin-bottom: 24px;
  font-size: 14px;

  &.alert-error {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.2);
    color: #ef4444;
  }
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 16px;
  margin-bottom: 48px;
}

.stat-card {
  background: #111111;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 12px;
  padding: 24px;
  position: relative;
  overflow: hidden;

  &:hover {
    border-color: rgba(255, 255, 255, 0.15);
  }
}

.card-decoration {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.1), transparent);
}

.card-content {
  position: relative;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
}

.card-label {
  font-size: 13px;
  color: rgba(255, 255, 255, 0.5);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.card-icon {
  width: 40px;
  height: 40px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;

  svg {
    width: 20px;
    height: 20px;
  }

  &.hackathon-icon {
    background: rgba(255, 255, 255, 0.08);
    color: rgba(255, 255, 255, 0.7);
  }

  &.organizer-icon {
    background: rgba(255, 255, 255, 0.08);
    color: rgba(255, 255, 255, 0.7);
  }

  &.report-icon {
    background: rgba(239, 68, 68, 0.1);
    color: #ef4444;
  }
}

.card-loading {
  height: 60px;
  display: flex;
  align-items: center;
}

.spinner {
  width: 24px;
  height: 24px;
  border: 2px solid rgba(255, 255, 255, 0.1);
  border-top-color: #ffffff;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.card-value {
  display: flex;
  align-items: baseline;
  gap: 8px;
}

.stat-number {
  font-size: 36px;
  font-weight: 600;
  color: #ffffff;
  font-family: 'Unbounded', sans-serif;
  line-height: 1;
}

.stat-suffix {
  font-size: 13px;
  color: rgba(255, 255, 255, 0.4);
  font-family: 'JetBrains Mono', monospace;
  text-transform: lowercase;
}

.card-footer {
  margin-top: 12px;
}

.stat-hint {
  font-size: 13px;
  color: rgba(255, 255, 255, 0.4);
}

.quick-actions {
  margin-top: 48px;
}

.section-title {
  font-size: 18px;
  font-weight: 600;
  color: #ffffff;
  margin: 0 0 20px;
  font-family: 'Unbounded', sans-serif;
}

.actions-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 12px;
}

.action-card {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 20px;
  background: #111111;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 8px;
  text-decoration: none;
  transition: all 0.2s;
  cursor: pointer;

  &:hover {
    border-color: rgba(255, 255, 255, 0.2);
    background: rgba(255, 255, 255, 0.02);
  }
}

.action-icon {
  width: 36px;
  height: 36px;
  border-radius: 6px;
  background: rgba(255, 255, 255, 0.05);
  display: flex;
  align-items: center;
  justify-content: center;
  color: rgba(255, 255, 255, 0.6);

  svg {
    width: 18px;
    height: 18px;
  }
}

.action-label {
  font-size: 14px;
  font-weight: 500;
  color: #ffffff;
}

.action-hint {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.4);
}

// Entrance animations
.animate-in {
  opacity: 0;
  transform: translateY(12px);
  animation: fadeSlideIn 0.5s cubic-bezier(0.23, 1, 0.32, 1) forwards;
}

.row-animate {
  opacity: 0;
  transform: translateX(-8px);
  animation: slideInRow 0.4s cubic-bezier(0.23, 1, 0.32, 1) forwards;
}

@keyframes fadeSlideIn {
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes slideInRow {
  to {
    opacity: 1;
    transform: translateX(0);
  }
}
</style>
