<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useScrollLock } from '@/composables/useScrollLock'
import { adminApi } from '@/services/api'
import type { PendingHackathon } from '@/services/api'

const hackathons = ref<PendingHackathon[]>([])
const total = ref(0)
const isLoading = ref(true)
const isProcessing = ref(false)
const errorMessage = ref('')
const successMessage = ref('')
const selectedHackathon = ref<PendingHackathon | null>(null)
const isRejecting = ref(false)
const rejectReason = ref('')

const displayedHackathons = computed(() =>
  [...hackathons.value].sort(
    (left, right) =>
      new Date(right.created_at).getTime() - new Date(left.created_at).getTime()
  )
)

const pendingCount = computed(() => total.value || displayedHackathons.value.length)

const isHackathonModalOpen = computed(() => !!selectedHackathon.value)
useScrollLock(isHackathonModalOpen)

const formatDate = (value: string) =>
  new Intl.DateTimeFormat('ru-RU', {
    dateStyle: 'medium',
    timeStyle: 'short',
  }).format(new Date(value))

const loadHackathons = async () => {
  isLoading.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const response = await adminApi.getPendingHackathons()

    if (response.error) {
      errorMessage.value = response.error
      hackathons.value = []
      total.value = 0
      return
    }

    hackathons.value = response.data?.hackathons ?? []
    total.value = response.data?.total ?? hackathons.value.length
  } catch {
    errorMessage.value = 'Не удалось загрузить очередь модерации'
  } finally {
    isLoading.value = false
  }
}

const openDetails = (hackathon: PendingHackathon) => {
  selectedHackathon.value = hackathon
  isRejecting.value = false
  rejectReason.value = ''
}

const openRejectDialog = (hackathon: PendingHackathon) => {
  selectedHackathon.value = hackathon
  isRejecting.value = true
  rejectReason.value = ''
}

const closeModal = () => {
  selectedHackathon.value = null
  isRejecting.value = false
  rejectReason.value = ''
}

const removeHackathon = (id: string) => {
  hackathons.value = hackathons.value.filter((item) => item.id !== id)
  total.value = Math.max(0, total.value - 1)
}

const approveHackathon = async (hackathon: PendingHackathon) => {
  if (isProcessing.value) {
    return
  }

  isProcessing.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const response = await adminApi.approveHackathon(hackathon.id)

    if (response.error) {
      errorMessage.value = response.error
      return
    }

    removeHackathon(hackathon.id)
    successMessage.value = `Хакатон "${hackathon.title}" одобрен`

    if (selectedHackathon.value?.id === hackathon.id) {
      closeModal()
    }
  } catch {
    errorMessage.value = 'Не удалось одобрить хакатон'
  } finally {
    isProcessing.value = false
  }
}

const rejectHackathon = async () => {
  if (!selectedHackathon.value || isProcessing.value) {
    return
  }

  const reason = rejectReason.value.trim()
  if (!reason) {
    return
  }

  isProcessing.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const response = await adminApi.rejectHackathon(selectedHackathon.value.id, reason)

    if (response.error) {
      errorMessage.value = response.error
      return
    }

    const rejectedTitle = selectedHackathon.value.title
    removeHackathon(selectedHackathon.value.id)
    successMessage.value = `Хакатон "${rejectedTitle}" отклонён`
    closeModal()
  } catch {
    errorMessage.value = 'Не удалось отклонить хакатон'
  } finally {
    isProcessing.value = false
  }
}

onMounted(() => {
  void loadHackathons().then(() => {
    requestAnimationFrame(() => {
      document.querySelectorAll('.animate-in').forEach((el) => {
        el.classList.add('animate-visible')
      })
    })
  })
})
</script>

<template>
  <section class="moderation-queue">
    <header class="queue-header animate-in" style="animation-delay: 0ms">
      <h1 class="queue-title">Модерация хакатонов</h1>
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
      <div class="loading-text">Загружаем заявки на модерацию...</div>
    </div>

    <div v-else-if="displayedHackathons.length === 0" class="empty-state animate-in" style="animation-delay: 150ms">
      <div class="empty-icon" aria-hidden="true">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="4" y="5" width="16" height="14" rx="2" />
          <path d="M8 3v4" />
          <path d="M16 3v4" />
          <path d="M4 10h16" />
        </svg>
      </div>
      <div class="empty-text">Очередь модерации пуста</div>
      <div class="empty-subtext">Новых заявок пока нет</div>
    </div>

    <div v-else class="hackathons-list">
      <article
        v-for="(hackathon, index) in displayedHackathons"
        :key="hackathon.id"
        class="hackathon-card row-animate"
        :style="{ animationDelay: `${200 + index * 30}ms` }"
      >
        <div class="card-header">
          <h2 class="hackathon-title">{{ hackathon.title }}</h2>
          <span class="hackathon-date mono">{{ formatDate(hackathon.created_at) }}</span>
        </div>

        <div class="organizer-info">
          <span class="organizer-label">Организатор:</span>
          <span class="organizer-name">{{ hackathon.organizer.name }}</span>
        </div>

        <p v-if="hackathon.description" class="hackathon-description">
          {{ hackathon.description }}
        </p>

        <div class="card-actions">
          <button type="button" class="btn btn-outline" @click="openDetails(hackathon)">
            Подробнее
          </button>

          <div class="action-buttons">
            <button
              type="button"
              class="btn btn-danger"
              :disabled="isProcessing"
              @click="openRejectDialog(hackathon)"
            >
              Отклонить
            </button>
            <button
              type="button"
              class="btn btn-success"
              :disabled="isProcessing"
              @click="approveHackathon(hackathon)"
            >
              Одобрить
            </button>
          </div>
        </div>
      </article>
    </div>

    <div v-if="selectedHackathon" class="modal-overlay" @click.self="closeModal">
      <div class="modal">
        <div class="modal-header">
          <h2 class="modal-title">{{ selectedHackathon.title }}</h2>
          <button type="button" class="modal-close" @click="closeModal">&times;</button>
        </div>

        <div class="modal-content">
          <div v-if="!isRejecting">
            <div class="detail-row">
              <span class="detail-label">Организатор</span>
              <span class="detail-value">{{ selectedHackathon.organizer.name }}</span>
            </div>
            <div class="detail-row">
              <span class="detail-label">Дата подачи</span>
              <span class="detail-value mono">{{ formatDate(selectedHackathon.created_at) }}</span>
            </div>
            <div class="detail-row">
              <span class="detail-label">Описание</span>
              <span class="detail-value description">
                {{ selectedHackathon.description || 'Описание не указано' }}
              </span>
            </div>
          </div>

          <div v-else>
            <div class="reject-info">
              <strong>Причина отклонения</strong>
              <div>Укажите понятную причину, чтобы организатор понял, что нужно исправить.</div>
            </div>
            <textarea
              v-model="rejectReason"
              class="reject-textarea"
              placeholder="Опишите, почему заявка не может быть одобрена"
            ></textarea>
          </div>
        </div>

        <div class="modal-footer">
          <div v-if="!isRejecting" class="modal-actions">
            <button type="button" class="btn btn-danger" @click="isRejecting = true">
              Отклонить
            </button>
            <button
              type="button"
              class="btn btn-success"
              :disabled="isProcessing"
              @click="approveHackathon(selectedHackathon)"
            >
              {{ isProcessing ? 'Обработка...' : 'Одобрить' }}
            </button>
          </div>

          <template v-else>
            <div class="modal-actions">
              <button type="button" class="btn btn-secondary" @click="isRejecting = false">
                Назад
              </button>
              <button
                type="button"
                class="btn btn-danger"
                :disabled="!rejectReason.trim() || isProcessing"
                @click="rejectHackathon"
              >
                {{ isProcessing ? 'Обработка...' : 'Подтвердить отклонение' }}
              </button>
            </div>
          </template>
        </div>
      </div>
    </div>
  </section>
</template>


<style scoped lang="scss">
.moderation-queue {
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

.hackathons-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.hackathon-card {
  background: #111111;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 8px;
  padding: 20px;
  transition: all 0.2s;

  &:hover {
    border-color: rgba(255, 255, 255, 0.15);
  }

  &.selected {
    border-color: rgba(255, 255, 255, 0.25);
    background: rgba(255, 255, 255, 0.02);
  }
}

.card-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: 12px;
}

.hackathon-title {
  font-size: 16px;
  font-weight: 600;
  color: #ffffff;
  margin: 0;
  flex: 1;
  margin-right: 16px;
}

.hackathon-date {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.4);
}

.organizer-info {
  display: flex;
  gap: 6px;
  margin-bottom: 8px;
  font-size: 13px;
}

.organizer-label {
  color: rgba(255, 255, 255, 0.5);
}

.organizer-name {
  color: rgba(255, 255, 255, 0.8);
}

.hackathon-description {
  font-size: 13px;
  color: rgba(255, 255, 255, 0.6);
  margin: 0 0 16px;
  line-height: 1.5;
}

.card-actions {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.action-buttons {
  display: flex;
  gap: 8px;
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

.btn-danger {
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.2);
  color: #ef4444;

  &:hover:not(:disabled) {
    background: rgba(239, 68, 68, 0.15);
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
  width: 500px;
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
  min-width: 100px;
}

.detail-value {
  color: #ffffff;
  flex: 1;

  &.description {
    line-height: 1.6;
    color: rgba(255, 255, 255, 0.8);
  }
}

.reject-info {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.8);
  margin-bottom: 16px;
  line-height: 1.5;

  strong {
    color: #ffffff;
  }
}

.reject-textarea {
  width: 100%;
  background: #0a0a0a;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  padding: 12px;
  color: #ffffff;
  font-size: 14px;
  font-family: inherit;
  resize: vertical;
  min-height: 80px;

  &:focus {
    outline: none;
    border-color: rgba(255, 255, 255, 0.3);
  }

  &::placeholder {
    color: rgba(255, 255, 255, 0.3);
  }
}

.modal-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-top: 1px solid rgba(255, 255, 255, 0.08);
}

.modal-actions {
  display: flex;
  gap: 8px;
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
