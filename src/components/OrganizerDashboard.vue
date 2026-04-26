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
            <div class="stat-icon pending">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10"/>
                <line x1="12" y1="8" x2="12" y2="12"/>
                <line x1="12" y1="16" x2="12.01" y2="16"/>
              </svg>
            </div>
            <div class="stat-info">
              <span class="stat-value">{{ pendingCount }}</span>
              <span class="stat-label">На модерации</span>
            </div>
          </div>
          <div class="stat-card">
            <div class="stat-icon approved">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
                <circle cx="9" cy="7" r="4"/>
                <path d="M23 21v-2a4 4 0 0 0-3-3.87"/>
                <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
              </svg>
            </div>
            <div class="stat-info">
              <span class="stat-value">{{ approvedCount }}</span>
              <span class="stat-label">Одобрено</span>
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
                  :class="getStatusClass(hackathon.status || 'pending')"
                >
                  {{ getStatusText(hackathon.status || 'pending') }}
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
            v-if="hackathon.status === 'rejected'"
            :to="`/hackathons/${hackathon.id}/edit`"
            class="btn btn-icon edit"
            title="Редактировать"
          >
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
              <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
            </svg>
          </router-link>
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
            <router-link
              :to="`/hackathons/${hackathon.id}/criteria`"
              class="btn btn-icon"
              title="Настроить критерии оценивания"
            >
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="4" y1="21" x2="4" y2="14"/>
                <line x1="4" y1="10" x2="4" y2="3"/>
                <line x1="12" y1="21" x2="12" y2="12"/>
                <line x1="12" y1="8" x2="12" y2="3"/>
                <line x1="20" y1="21" x2="20" y2="16"/>
                <line x1="20" y1="12" x2="20" y2="3"/>
                <line x1="1" y1="14" x2="7" y2="14"/>
                <line x1="9" y1="8" x2="15" y2="8"/>
                <line x1="17" y1="16" x2="23" y2="16"/>
              </svg>
            </router-link>
            <router-link
              v-if="isHackathonEnded(hackathon)"
              :to="`/hackathons/${hackathon.id}/ratings`"
              class="btn btn-icon rate"
              title="Оценить решения"
            >
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/>
              </svg>
            </router-link>
            <button
              v-if="canCancel(hackathon)"
              type="button"
              class="btn btn-icon cancel"
              title="Отменить хакатон"
              @click="onCancel(hackathon)"
            >
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10"/>
                <line x1="4.93" y1="4.93" x2="19.07" y2="19.07"/>
              </svg>
            </button>
            <button
              v-if="canDelete(hackathon)"
              type="button"
              class="btn btn-icon delete"
              title="Удалить хакатон"
              @click="onDelete(hackathon)"
            >
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="3 6 5 6 21 6"/>
                <path d="M19 6l-2 14a2 2 0 0 1-2 2H9a2 2 0 0 1-2-2L5 6"/>
                <path d="M10 11v6"/>
                <path d="M14 11v6"/>
                <path d="M9 6V4a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v2"/>
              </svg>
            </button>
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
import { organizerApi, hackathonApi, type OrganizerHackathonSummary } from '@/services/api'
import { useModal } from '@/composables/useModal'

const { alert, confirm } = useModal()

const header = ref<HTMLElement | null>(null)
const stats = ref<HTMLElement | null>(null)
const content = ref<HTMLElement | null>(null)

const hackathons = ref<OrganizerHackathonSummary[]>([])
const isLoading = ref(true)

const pendingCount = computed(() => {
  return hackathons.value.filter(h => (h.status || 'pending') === 'pending').length
})

const approvedCount = computed(() => {
  return hackathons.value.filter(h => h.status === 'approved').length
})

const totalParticipants = computed(() => {
  return hackathons.value.reduce((sum, h) => sum + (h.participant_count || 0), 0)
})

const formatDate = (dateString: string) => {
  const date = new Date(dateString)
  return date.toLocaleDateString('ru-RU', { day: 'numeric', month: 'short' })
}

const isHackathonEnded = (hackathon: OrganizerHackathonSummary) => {
  return new Date(hackathon.event_end) < new Date()
}

const getStatusClass = (status: string) => {
  switch (status) {
    case 'approved': return 'approved'
    case 'rejected': return 'rejected'
    case 'cancelled': return 'cancelled'
    case 'pending':
    default: return 'pending'
  }
}

const getStatusText = (status: string) => {
  switch (status) {
    case 'approved': return 'Одобрен'
    case 'rejected': return 'Отклонен'
    case 'cancelled': return 'Отменён'
    case 'pending':
    default: return 'На модерации'
  }
}

const canDelete = (h: OrganizerHackathonSummary) => {
  const status = h.status || 'pending'
  if (status === 'pending' || status === 'rejected') return true
  if (status === 'approved' && (h.team_count || 0) === 0) return true
  return false
}

const canCancel = (h: OrganizerHackathonSummary) => {
  return (h.status || 'pending') === 'approved' && (h.team_count || 0) > 0
}

const onDelete = async (h: OrganizerHackathonSummary) => {
  const ok = await confirm({
    title: 'Удалить хакатон',
    message: `Вы уверены, что хотите удалить хакатон «${h.title}»? Это действие необратимо.`,
    type: 'error',
    confirmText: 'Удалить',
    cancelText: 'Отмена',
  })
  if (!ok) return
  const res = await hackathonApi.delete(h.id)
  if (res.error) {
    await alert({
      title: 'Не удалось удалить',
      message: res.error,
      type: 'error',
    })
    return
  }
  hackathons.value = hackathons.value.filter(x => x.id !== h.id)
}

const onCancel = async (h: OrganizerHackathonSummary) => {
  const ok = await confirm({
    title: 'Отменить хакатон',
    message: `Отменить хакатон «${h.title}»? Участники получат уведомление, регистрация закроется.`,
    type: 'warning',
    confirmText: 'Отменить хакатон',
    cancelText: 'Назад',
  })
  if (!ok) return
  const res = await hackathonApi.cancel(h.id)
  if (res.error) {
    await alert({
      title: 'Не удалось отменить',
      message: res.error,
      type: 'error',
    })
    return
  }
  h.status = 'cancelled'
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
  try {
    const organizerResponse = await organizerApi.getMyOrganizer()
    if (!organizerResponse.data) {
      hackathons.value = []
      return
    }

    const response = await organizerApi.getHackathons(organizerResponse.data.id)
    hackathons.value = response.data ?? []
  } finally {
    isLoading.value = false
  }
})
</script>

<style scoped lang="scss">
@use '../styles/variables' as *;

.dashboard-page {
  min-height: 100vh;
  background: $color-bg;
  padding: 7rem 2rem 2rem;
}

.container {
  max-width: 1200px;
  margin: 0 auto;
}

.dashboard-header {
  margin-bottom: 2rem;
  padding-bottom: 1.5rem;
  border-bottom: 1px solid $color-surface;

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
      font-weight: 600;
      margin-bottom: 0.5rem;
      font-family: $font-display;
      color: $color-text;
      letter-spacing: -0.02em;
    }

    .header-subtitle {
      color: $color-text-dim;
      font-size: 1rem;
      font-family: $font-body;
    }
  }

  .header-actions {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;

    @media (max-width: 640px) {
      width: 100%;
      justify-content: flex-start;
    }
  }
}

.btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  font-weight: 600;
  font-size: 0.9rem;
  transition: all 0.3s $transition-smooth;
  cursor: pointer;
  border: 1px solid $color-border;
  text-decoration: none;
  text-transform: lowercase;
  font-family: $font-body;
  position: relative;
  overflow: hidden;

  &.btn-primary {
    background: transparent;
    color: $color-accent;
    border-color: $color-accent;

    &::before {
      content: '';
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      background: $color-accent;
      transform: scaleX(0);
      transform-origin: left;
      transition: transform 0.3s $transition-smooth;
      z-index: -1;
    }

    &:hover {
      color: $color-bg;

      &::before {
        transform: scaleX(1);
      }
    }
  }

  &.btn-outline {
    background: transparent;
    color: $color-text;
    border-color: $color-border;

    &::before {
      content: '';
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      background: $color-surface;
      transform: scaleX(0);
      transform-origin: left;
      transition: transform 0.3s $transition-smooth;
      z-index: -1;
    }

    &:hover {
      border-color: $color-text;

      &::before {
        transform: scaleX(1);
      }
    }
  }

  &.btn-icon {
    padding: 0.5rem;
    background: transparent;
    border-color: $color-border;
    color: $color-text-dim;

    &:hover {
      border-color: $color-accent;
      color: $color-accent;
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
  background: $color-surface;
  border: 1px solid $color-border;
  border-radius: 8px;
  transition: border-color 0.3s $transition-smooth;

  &:hover {
    border-color: $color-accent;
  }

  .stat-icon {
    width: 48px;
    height: 48px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;

    &.published {
      background: rgba($color-accent, 0.1);
      color: $color-accent;
    }

    &.pending {
      background: rgba($color-text-dim, 0.1);
      color: $color-text-dim;
    }

    &.approved {
      background: rgba(255, 255, 255, 0.1);
      color: $color-text;
    }
  }

  .stat-info {
    display: flex;
    flex-direction: column;
  }

  .stat-value {
    font-size: 1.75rem;
    font-weight: 700;
    color: $color-text;
    font-family: $font-display;
  }

  .stat-label {
    font-size: 0.875rem;
    color: $color-text-dim;
    font-family: $font-body;
  }
}

.content-section {
  background: $color-surface;
  border: 1px solid $color-border;
  border-radius: 8px;
  padding: 1.5rem;

  .section-header {
    margin-bottom: 1.5rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid $color-border;

    h2 {
      font-size: 1.25rem;
      font-weight: 600;
      color: $color-text;
      font-family: $font-display;
    }
  }
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem;
  color: $color-text-dim;
  gap: 1rem;

  .spinner {
    width: 40px;
    height: 40px;
    border: 2px solid $color-border;
    border-top-color: $color-accent;
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
    color: $color-text-muted;
    margin-bottom: 1.5rem;
  }

  h3 {
    font-size: 1.25rem;
    font-weight: 600;
    color: $color-text;
    margin-bottom: 0.5rem;
    font-family: $font-display;
  }

  p {
    color: $color-text-dim;
    margin-bottom: 1.5rem;
    max-width: 400px;
    font-family: $font-body;
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
  background: rgba($color-accent, 0.02);
  border: 1px solid $color-border;
  border-radius: 8px;
  transition: all 0.3s $transition-smooth;

  &:hover {
    border-color: $color-accent;
    background: rgba($color-accent, 0.04);
  }
}

.hackathon-image {
  position: relative;
  width: 80px;
  height: 80px;
  flex-shrink: 0;
  border-radius: 8px;
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
    background: $color-bg;
    border: 1px solid $color-border;
    color: $color-text;
    font-size: 1.5rem;
    font-weight: 700;
    font-family: $font-display;
  }
}

.hackathon-status {
  position: absolute;
  top: 0.25rem;
  right: 0.25rem;
  padding: 0.2rem 0.5rem;
  border-radius: 4px;
  font-size: 0.65rem;
  font-weight: 600;
  text-transform: uppercase;
  font-family: $font-mono;

  &.published {
    background: rgba($color-accent, 0.9);
    color: $color-bg;
  }

  &.draft {
    background: rgba($color-secondary, 0.9);
    color: white;
  }

  &.pending, &.approved, &.rejected, &.cancelled {
    background: rgba(255, 255, 255, 0.1);
    color: $color-text;
  }

  &.cancelled {
    background: rgba(220, 80, 80, 0.85);
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
  color: $color-text;
  margin-bottom: 0.5rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-family: $font-display;
}

.hackathon-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 1rem;
  font-family: $font-body;

  .meta-item {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    font-size: 0.85rem;
    color: $color-text-dim;

    svg {
      color: $color-accent;
    }
  }
}

.hackathon-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;

  .btn-icon.rate {
    color: $color-accent;
    border-color: $color-accent;

    &:hover {
      background: $color-accent;
      color: $color-bg;
    }
  }

  .btn-icon.delete,
  .btn-icon.cancel {
    color: #d36a6a;
    border-color: rgba(211, 106, 106, 0.4);
    background: transparent;
    cursor: pointer;

    &:hover {
      background: rgba(211, 106, 106, 0.1);
      border-color: #d36a6a;
      color: #d36a6a;
    }
  }
}
</style>
