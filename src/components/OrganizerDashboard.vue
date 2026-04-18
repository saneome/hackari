<template>
  <div class="dashboard-page">
    <div class="container">
      <div class="dashboard-header" ref="header">
        <div class="header-content">
          <div class="header-title">
            <h1>Дашборд организатора</h1>
            <p class="header-subtitle">Управляйте вашими хакатонами</p>
          </div>
          <div class="header-actions">
            <router-link to="/organizers/profile" class="btn btn-outline">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
                <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
              </svg>
              Редактировать профиль
            </router-link>
            <router-link to="/hackathons/create" class="btn btn-primary">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="12" y1="5" x2="12" y2="19"/>
                <line x1="5" y1="12" x2="19" y2="12"/>
              </svg>
              Создать хакатон
            </router-link>
          </div>
        </div>
      </div>

      <div class="dashboard-grid">
        <div class="stats-cards" ref="stats">
          <div class="stat-card">
            <div class="stat-icon published">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                <polyline points="14 2 14 8 20 8"/>
                <line x1="16" y1="13" x2="8" y2="13"/>
                <line x1="16" y1="17" x2="8" y2="17"/>
                <polyline points="10 9 9 9 8 9"/>
              </svg>
            </div>
            <div class="stat-info">
              <span class="stat-value">{{ hackathons.length }}</span>
              <span class="stat-label">Всего хакатонов</span>
            </div>
          </div>
          <div class="stat-card">
            <div class="stat-icon draft">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10"/>
                <line x1="12" y1="8" x2="12" y2="12"/>
                <line x1="12" y1="16" x2="12.01" y2="16"/>
              </svg>
            </div>
            <div class="stat-info">
              <span class="stat-value">{{ upcomingCount }}</span>
              <span class="stat-label">Предстоящих</span>
            </div>
          </div>
          <div class="stat-card">
            <div class="stat-icon participants">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
                <circle cx="9" cy="7" r="4"/>
                <path d="M23 21v-2a4 4 0 0 0-3-3.87"/>
                <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
              </svg>
            </div>
            <div class="stat-info">
              <span class="stat-value">{{ totalParticipants }}</span>
              <span class="stat-label">Всего участников</span>
            </div>
          </div>
        </div>

        <div class="content-section" ref="content">
          <div class="section-header">
            <h2>Мои хакатоны</h2>
          </div>

          <div v-if="isLoading" class="loading-state">
            <div class="spinner"></div>
            <span>Загрузка...</span>
          </div>

          <div v-else-if="hackathons.length === 0" class="empty-state">
            <div class="empty-icon">
              <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                <polyline points="14 2 14 8 20 8"/>
              </svg>
            </div>
            <h3>У вас пока нет хакатонов</h3>
            <p>Создайте свой первый хакатон и начните принимать заявки от участников</p>
            <router-link to="/hackathons/create" class="btn btn-primary">
              Создать хакатон
            </router-link>
          </div>

          <div v-else class="hackathons-list">
            <div
              v-for="hackathon in hackathons"
              :key="hackathon.id"
              class="hackathon-card"
            >
              <div class="hackathon-image">
                <img
                  v-if="hackathon.banner_url"
                  :src="hackathon.banner_url"
                  :alt="hackathon.title"
                />
                <div v-else class="hackathon-placeholder">H</div>
                <span
                  class="hackathon-status"
                  :class="hackathon.is_published ? 'published' : 'draft'"
                >
                  {{ hackathon.is_published ? 'Опубликован' : 'Черновик' }}
                </span>
              </div>
              <div class="hackathon-content">
                <h3 class="hackathon-title">{{ hackathon.title }}</h3>
                <div class="hackathon-meta">
                  <span class="meta-item">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <rect x="3" y="4" width="18" height="18" rx="2" ry="2"/>
                      <line x1="16" y1="2" x2="16" y2="6"/>
                      <line x1="8" y1="2" x2="8" y2="6"/>
                      <line x1="3" y1="10" x2="21" y2="10"/>
                    </svg>
                    {{ formatDate(hackathon.event_start) }} - {{ formatDate(hackathon.event_end) }}
                  </span>
                  <span class="meta-item">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
                      <circle cx="9" cy="7" r="4"/>
                    </svg>
                    {{ hackathon.participant_count }} участников
                  </span>
                  <span class="meta-item">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
                      <circle cx="9" cy="7" r="4"/>
                      <path d="M23 21v-2a4 4 0 0 0-3-3.87"/>
                      <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
                    </svg>
                    {{ hackathon.team_count }} команд
                  </span>
                </div>
              </div>
              <div class="hackathon-actions">
                <router-link
                  :to="`/hackathons/${hackathon.id}`"
                  class="btn btn-icon"
                  title="Посмотреть"
                >
                  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
                    <circle cx="12" cy="12" r="3"/>
                  </svg>
                </router-link>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import gsap from 'gsap'
import { organizerApi, hackathonApi } from '@/services/api'
import { useAuth } from '@/composables/useAuth'

const { user } = useAuth()

const header = ref<HTMLElement | null>(null)
const stats = ref<HTMLElement | null>(null)
const content = ref<HTMLElement | null>(null)

const hackathons = ref<{
  id: string
  title: string
  banner_url?: string
  is_published: boolean
  event_start: string
  event_end: string
  participant_count: number
  team_count: number
}[]>([])
const isLoading = ref(true)

const upcomingCount = computed(() => {
  const now = new Date()
  return hackathons.value.filter(h => new Date(h.event_start) > now).length
})

const totalParticipants = computed(() => {
  return hackathons.value.reduce((sum, h) => sum + (h.participant_count || 0), 0)
})

const formatDate = (dateString: string) => {
  const date = new Date(dateString)
  return date.toLocaleDateString('ru-RU', { day: 'numeric', month: 'short' })
}

onMounted(async () => {
  gsap.fromTo(header.value,
    { y: -20, opacity: 0 },
    { y: 0, opacity: 1, duration: 0.5, ease: 'power2.out' }
  )

  gsap.fromTo(stats.value,
    { y: 20, opacity: 0 },
    { y: 0, opacity: 1, duration: 0.5, delay: 0.1, ease: 'power2.out' }
  )

  gsap.fromTo(content.value,
    { y: 20, opacity: 0 },
    { y: 0, opacity: 1, duration: 0.5, delay: 0.2, ease: 'power2.out' }
  )

  // Load organizer's hackathons
  isLoading.value = true
  const response = await hackathonApi.list()
  if (response.data) {
    // Filter only hackathons from this organizer
    // For now, show all hackathons
    hackathons.value = response.data.hackathons
  }
  isLoading.value = false
})
</script>

<style scoped lang="scss">
@use '../styles/variables' as *;

.dashboard-page {
  min-height: 100vh;
  background: var(--bg-color);
  padding: 6rem 1rem 2rem;
}

.container {
  max-width: 1200px;
  margin: 0 auto;
}

.dashboard-header {
  margin-bottom: 2rem;

  .header-content {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
  }

  .header-title {
    h1 {
      font-size: clamp(1.75rem, 4vw, 2.5rem);
      font-weight: 700;
      margin-bottom: 0.5rem;
      background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      background-clip: text;
    }

    .header-subtitle {
      color: var(--text-secondary);
      font-size: 1rem;
    }
  }

  .header-actions {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;

    @media (max-width: 640px) {
      width: 100%;
        justify-content: space-between;
    }
  }
}

.btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.25rem;
  border-radius: 0.75rem;
  font-weight: 600;
  font-size: 0.9rem;
  transition: all 0.3s ease;
  cursor: pointer;
  border: none;
  text-decoration: none;

  &.btn-primary {
    background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
    color: white;

    &:hover {
      transform: translateY(-2px);
      box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
    }
  }

  &.btn-outline {
    background: transparent;
    color: var(--text-color);
    border: 1px solid var(--border-color);

    &:hover {
      border-color: var(--accent-primary);
      color: var(--accent-primary);
    }
  }

  &.btn-icon {
    padding: 0.5rem;
    background: rgba(var(--accent-primary-rgb), 0.1);
    color: var(--accent-primary);

    &:hover {
      background: rgba(var(--accent-primary-rgb), 0.2);
    }
  }
}

.stats-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.stat-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem;
  background: var(--card-bg);
  border: 1px solid var(--border-color);
  border-radius: 1rem;

  .stat-icon {
    width: 48px;
    height: 48px;
    border-radius: 0.75rem;
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;

    &.published {
      background: linear-gradient(135deg, #22c55e, #16a34a);
    }

    &.draft {
      background: linear-gradient(135deg, #f59e0b, #d97706);
    }

    &.participants {
      background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
    }
  }

  .stat-info {
    display: flex;
    flex-direction: column;
  }

  .stat-value {
    font-size: 1.75rem;
    font-weight: 700;
    color: var(--text-color);
  }

  .stat-label {
    font-size: 0.875rem;
    color: var(--text-secondary);
  }
}

.content-section {
  background: var(--card-bg);
  border: 1px solid var(--border-color);
  border-radius: 1.5rem;
  padding: 1.5rem;

  .section-header {
    margin-bottom: 1.5rem;

    h2 {
      font-size: 1.25rem;
      font-weight: 600;
      color: var(--text-color);
    }
  }
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem;
  color: var(--text-secondary);
  gap: 1rem;

  .spinner {
    width: 40px;
    height: 40px;
    border: 3px solid var(--border-color);
    border-top-color: var(--accent-primary);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem;
  text-align: center;

  .empty-icon {
    width: 80px;
    height: 80px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-tertiary);
    margin-bottom: 1.5rem;
  }

  h3 {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-color);
    margin-bottom: 0.5rem;
  }

  p {
    color: var(--text-secondary);
    margin-bottom: 1.5rem;
    max-width: 400px;
  }
}

.hackathons-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.hackathon-card {
  display: flex;
  gap: 1rem;
  padding: 1rem;
  background: rgba(var(--accent-primary-rgb), 0.05);
  border: 1px solid var(--border-color);
  border-radius: 1rem;
  transition: all 0.3s ease;

  &:hover {
    border-color: var(--accent-primary);
    transform: translateX(4px);
  }
}

.hackathon-image {
  position: relative;
  width: 80px;
  height: 80px;
  flex-shrink: 0;
  border-radius: 0.75rem;
  overflow: hidden;

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .hackathon-placeholder {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
    color: white;
    font-size: 2rem;
    font-weight: 700;
  }
}

.hackathon-status {
  position: absolute;
  top: 0.25rem;
  right: 0.25rem;
  padding: 0.25rem 0.5rem;
  border-radius: 0.25rem;
  font-size: 0.7rem;
  font-weight: 600;
  text-transform: uppercase;

  &.published {
    background: #22c55e;
    color: white;
  }

  &.draft {
    background: #f59e0b;
    color: white;
  }
}

.hackathon-content {
  flex: 1;
  min-width: 0;
}

.hackathon-title {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-color);
  margin-bottom: 0.5rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.hackathon-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 1rem;

  .meta-item {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    font-size: 0.85rem;
    color: var(--text-secondary);

    svg {
      color: var(--accent-primary);
    }
  }
}

.hackathon-actions {
  display: flex;
  align-items: center;
}
</style>
