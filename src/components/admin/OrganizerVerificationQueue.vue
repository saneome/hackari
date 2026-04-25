<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useAdminModal } from '@/composables/useAdminModal'
import { adminApi } from '@/services/api'
import type { UnverifiedOrganizer } from '@/services/api'

const { confirm } = useAdminModal()

const organizers = ref<UnverifiedOrganizer[]>([])
const total = ref(0)
const isLoading = ref(true)
const isVerifyingId = ref('')
const errorMessage = ref('')
const successMessage = ref('')
const selectedOrganizer = ref<UnverifiedOrganizer | null>(null)

const displayedOrganizers = computed(() =>
  [...organizers.value].sort(
    (left, right) =>
      new Date(right.created_at).getTime() - new Date(left.created_at).getTime()
  )
)

const pendingCount = computed(() => total.value || displayedOrganizers.value.length)

const formatDate = (value: string) =>
  new Intl.DateTimeFormat('ru-RU', {
    dateStyle: 'medium',
    timeStyle: 'short',
  }).format(new Date(value))

const getInitial = (organizer: UnverifiedOrganizer) => {
  const source = organizer.name.trim() || organizer.user_name.trim() || organizer.email.trim() || 'U'
  return source.charAt(0).toUpperCase()
}

const loadOrganizers = async () => {
  isLoading.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const response = await adminApi.getUnverifiedOrganizers()

    if (response.error) {
      errorMessage.value = response.error
      organizers.value = []
      total.value = 0
      return
    }

    organizers.value = response.data?.organizers ?? []
    total.value = response.data?.total ?? organizers.value.length
  } catch {
    errorMessage.value = 'Не удалось загрузить очередь верификации'
  } finally {
    isLoading.value = false
  }
}

const openDetails = (organizer: UnverifiedOrganizer) => {
  selectedOrganizer.value = organizer
}

const closeModal = () => {
  selectedOrganizer.value = null
}

const removeOrganizer = (id: string) => {
  organizers.value = organizers.value.filter((item) => item.id !== id)
  total.value = Math.max(0, total.value - 1)
}

const verifyOrganizer = async (organizer: UnverifiedOrganizer, needsConfirm = true) => {
  if (isVerifyingId.value) {
    return
  }

  if (needsConfirm) {
    const confirmed = await confirm({
      title: 'Подтверждение',
      message: `Подтвердить организатора ${organizer.name}?`,
      type: 'warning',
      confirmText: 'Подтвердить',
      cancelText: 'Отмена',
    })
    if (!confirmed) {
      return
    }
  }

  isVerifyingId.value = organizer.id
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const response = await adminApi.verifyOrganizer(organizer.id)

    if (response.error) {
      errorMessage.value = response.error
      return
    }

    removeOrganizer(organizer.id)
    successMessage.value = `Организатор "${organizer.name}" подтверждён`

    if (selectedOrganizer.value?.id === organizer.id) {
      closeModal()
    }
  } catch {
    errorMessage.value = 'Не удалось подтвердить организатора'
  } finally {
    isVerifyingId.value = ''
  }
}

onMounted(() => {
  void loadOrganizers().then(() => {
    requestAnimationFrame(() => {
      document.querySelectorAll('.animate-in').forEach((el) => {
        el.classList.add('animate-visible')
      })
    })
  })
})
</script>

<template>
  <section class="verification-queue">
    <header class="queue-header animate-in" style="animation-delay: 0ms">
      <h1 class="queue-title">Верификация организаторов</h1>
      <span class="queue-count">{{ pendingCount }} на проверке</span>
    </header>

    <div v-if="errorMessage" class="alert alert-error animate-in" style="animation-delay: 50ms">
      {{ errorMessage }}
    </div>

    <div v-if="successMessage" class="alert alert-success animate-in" style="animation-delay: 100ms">
      {{ successMessage }}
    </div>

    <div v-if="isLoading" class="loading-state">
      <div class="spinner"></div>
      <div class="loading-text">Загружаем организаторов...</div>
    </div>

    <div v-else-if="displayedOrganizers.length === 0" class="empty-state animate-in" style="animation-delay: 150ms">
      <div class="empty-icon" aria-hidden="true">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />
          <circle cx="9" cy="7" r="4" />
          <path d="M23 21v-2a4 4 0 0 0-3-3.87" />
          <path d="M16 3.13a4 4 0 0 1 0 7.75" />
        </svg>
      </div>
      <div class="empty-text">Неподтверждённых организаторов нет</div>
      <div class="empty-subtext">Все заявки уже обработаны</div>
    </div>

    <div v-else class="organizers-list">
      <article
        v-for="(organizer, index) in displayedOrganizers"
        :key="organizer.id"
        class="organizer-card row-animate"
        :style="{ animationDelay: `${200 + index * 30}ms` }"
      >
        <div class="card-header">
          <div class="organizer-avatar">{{ getInitial(organizer) }}</div>
          <div class="organizer-info">
            <h2 class="organizer-name">{{ organizer.name }}</h2>
            <div class="organizer-meta">
              <span class="organizer-email">{{ organizer.email }}</span>
              <span class="dot">•</span>
              <span>{{ organizer.user_name }}</span>
            </div>
          </div>
          <span class="organizer-date mono">{{ formatDate(organizer.created_at) }}</span>
        </div>

        <p v-if="organizer.description" class="organizer-description">
          {{ organizer.description }}
        </p>

        <div class="organizer-links">
          <a :href="`mailto:${organizer.email}`" class="organizer-link">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M4 4h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2z" />
              <path d="M22 6l-10 7L2 6" />
            </svg>
            {{ organizer.email }}
          </a>
        </div>

        <div class="card-actions">
          <button type="button" class="btn btn-outline" @click="openDetails(organizer)">
            Подробнее
          </button>

          <button
            type="button"
            class="btn btn-success"
            :disabled="isVerifyingId === organizer.id"
            @click="verifyOrganizer(organizer)"
          >
            <span v-if="isVerifyingId === organizer.id" class="btn-spinner"></span>
            <span v-else>Подтвердить</span>
          </button>
        </div>
      </article>
    </div>

    <div v-if="selectedOrganizer" class="modal-overlay" @click.self="closeModal">
      <div class="modal">
        <div class="modal-header">
          <h2 class="modal-title">{{ selectedOrganizer.name }}</h2>
          <button type="button" class="modal-close" @click="closeModal">&times;</button>
        </div>

        <div class="modal-content">
          <div class="detail-row">
            <span class="detail-label">Организация</span>
            <span class="detail-value">{{ selectedOrganizer.name }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">Пользователь</span>
            <span class="detail-value">{{ selectedOrganizer.user_name }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">Email</span>
            <a :href="`mailto:${selectedOrganizer.email}`" class="detail-value link">
              {{ selectedOrganizer.email }}
            </a>
          </div>
          <div class="detail-row">
            <span class="detail-label">User ID</span>
            <span class="detail-value mono">{{ selectedOrganizer.user_id }}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">Создан</span>
            <span class="detail-value mono">{{ formatDate(selectedOrganizer.created_at) }}</span>
          </div>
          <div v-if="selectedOrganizer.description" class="detail-row">
            <span class="detail-label">Описание</span>
            <span class="detail-value description">{{ selectedOrganizer.description }}</span>
          </div>
        </div>

        <div class="modal-footer">
          <button type="button" class="btn btn-secondary" @click="closeModal">
            Закрыть
          </button>
          <button
            type="button"
            class="btn btn-success"
            :disabled="isVerifyingId === selectedOrganizer.id"
            @click="verifyOrganizer(selectedOrganizer, false)"
          >
            <span v-if="isVerifyingId === selectedOrganizer.id" class="btn-spinner"></span>
            <span v-else>Подтвердить</span>
          </button>
        </div>
      </div>
    </div>
  </section>
</template>


<style scoped lang="scss">
.verification-queue {
  max-width: 900px;
}

.queue-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 24px;
}

.queue-title {
  font-size: 24px;
  font-weight: 600;
  color: #ffffff;
  margin: 0;
  font-family: 'Unbounded', sans-serif;
}

.queue-count {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.5);
  padding: 6px 12px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 4px;
}

.alert {
  padding: 12px 16px;
  border-radius: 6px;
  margin-bottom: 16px;
  font-size: 14px;

  &.alert-success {
    background: rgba(34, 197, 94, 0.1);
    border: 1px solid rgba(34, 197, 94, 0.2);
    color: #22c55e;
  }

  &.alert-error {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.2);
    color: #ef4444;
  }
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  gap: 16px;
}

.spinner {
  width: 32px;
  height: 32px;
  border: 2px solid rgba(255, 255, 255, 0.1);
  border-top-color: #ffffff;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.loading-text {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.5);
}

.empty-state {
  text-align: center;
  padding: 80px 20px;
}

.empty-icon {
  width: 64px;
  height: 64px;
  margin: 0 auto 20px;
  color: rgba(255, 255, 255, 0.2);

  svg {
    width: 100%;
    height: 100%;
  }
}

.empty-text {
  font-size: 18px;
  color: rgba(255, 255, 255, 0.7);
  margin-bottom: 8px;
}

.empty-subtext {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.4);
}

.organizers-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.organizer-card {
  background: #111111;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 8px;
  padding: 20px;
  transition: all 0.2s;

  &:hover {
    border-color: rgba(255, 255, 255, 0.15);
  }
}

.card-header {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 16px;
}

.organizer-avatar {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.08);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18px;
  font-weight: 600;
  color: #ffffff;
  flex-shrink: 0;
}

.organizer-info {
  flex: 1;
  min-width: 0;
}

.organizer-name {
  font-size: 16px;
  font-weight: 600;
  color: #ffffff;
  margin: 0 0 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.organizer-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: rgba(255, 255, 255, 0.5);
}

.organizer-email {
  color: rgba(255, 255, 255, 0.6);
}

.dot {
  color: rgba(255, 255, 255, 0.3);
}

.organizer-date {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.4);
  flex-shrink: 0;
}

.organizer-description {
  font-size: 13px;
  color: rgba(255, 255, 255, 0.6);
  margin: 0 0 12px;
  line-height: 1.5;
}

.organizer-links {
  margin-bottom: 16px;
}

.organizer-link {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: rgba(255, 255, 255, 0.6);
  text-decoration: none;
  transition: color 0.2s;

  &:hover {
    color: #ffffff;
  }

  svg {
    width: 14px;
    height: 14px;
  }
}

.card-actions {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: none;

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  svg {
    width: 14px;
    height: 14px;
  }
}

.btn-outline {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  color: rgba(255, 255, 255, 0.7);

  &:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.08);
    color: #ffffff;
  }
}

.btn-success {
  background: rgba(34, 197, 94, 0.1);
  border: 1px solid rgba(34, 197, 94, 0.2);
  color: #22c55e;

  &:hover:not(:disabled) {
    background: rgba(34, 197, 94, 0.15);
  }
}

.btn-secondary {
  background: rgba(255, 255, 255, 0.08);
  color: rgba(255, 255, 255, 0.7);

  &:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.12);
    color: #ffffff;
  }
}

.btn-spinner {
  width: 14px;
  height: 14px;
  border: 2px solid rgba(255, 255, 255, 0.1);
  border-top-color: currentColor;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
  padding: 20px;
}

.modal {
  background: #111111;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  width: 480px;
  max-width: 100%;
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
}

.modal-title {
  font-size: 16px;
  font-weight: 600;
  color: #ffffff;
  margin: 0;
}

.modal-close {
  background: none;
  border: none;
  color: rgba(255, 255, 255, 0.5);
  font-size: 24px;
  cursor: pointer;
  transition: color 0.2s;

  &:hover {
    color: #ffffff;
  }
}

.modal-content {
  padding: 20px;
  overflow-y: auto;
}

.detail-row {
  display: flex;
  gap: 12px;
  margin-bottom: 12px;
  font-size: 14px;

  &:last-child {
    margin-bottom: 0;
  }
}

.detail-label {
  color: rgba(255, 255, 255, 0.5);
  min-width: 120px;
}

.detail-value {
  color: #ffffff;
  flex: 1;
  word-break: break-word;

  &.description {
    line-height: 1.6;
    color: rgba(255, 255, 255, 0.8);
  }

  &.link {
    color: rgba(255, 255, 255, 0.7);
    text-decoration: none;

    &:hover {
      color: #ffffff;
      text-decoration: underline;
    }
  }
}

.modal-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 20px;
  border-top: 1px solid rgba(255, 255, 255, 0.08);
}

.mono {
  font-family: 'JetBrains Mono', monospace;
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
