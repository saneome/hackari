<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { adminApi } from '@/services/api'
import type { AdminReport, ReportDetail } from '@/services/api'

const reports = ref<AdminReport[]>([])
const isLoading = ref(true)
const isProcessing = ref(false)
const isDetailLoading = ref(false)
const errorMessage = ref('')
const successMessage = ref('')
const currentFilter = ref<'all' | 'open' | 'resolved' | 'closed'>('all')
const selectedReportSummary = ref<AdminReport | null>(null)
const selectedReportDetail = ref<ReportDetail | null>(null)
const resolutionNote = ref('')

const filterOptions = [
  { value: 'all', label: 'Все' },
  { value: 'open', label: 'Открытые' },
  { value: 'resolved', label: 'Обработанные' },
  { value: 'closed', label: 'Закрытые' },
] as const

const filteredReports = computed(() =>
  currentFilter.value === 'all'
    ? reports.value
    : reports.value.filter((report) => report.status === currentFilter.value)
)

const filterCounts = computed(() => ({
  all: reports.value.length,
  open: reports.value.filter((report) => report.status === 'open').length,
  resolved: reports.value.filter((report) => report.status === 'resolved').length,
  closed: reports.value.filter((report) => report.status === 'closed').length,
}))

const activeReport = computed(() => selectedReportDetail.value ?? selectedReportSummary.value)

const formatDate = (value: string) =>
  new Intl.DateTimeFormat('ru-RU', {
    dateStyle: 'medium',
    timeStyle: 'short',
  }).format(new Date(value))

const formatTargetType = (value: string) => {
  const labels: Record<string, string> = {
    hackathon: 'Хакатон',
    organizer: 'Организатор',
    team: 'Команда',
    user: 'Пользователь',
  }

  return labels[value] || value
}

const statusLabel = (value: AdminReport['status']) => {
  const labels: Record<AdminReport['status'], string> = {
    open: 'Открыта',
    resolved: 'Обработана',
    closed: 'Закрыта',
  }

  return labels[value]
}

const loadReports = async () => {
  isLoading.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const response = await adminApi.getReports()

    if (response.error) {
      errorMessage.value = response.error
      reports.value = []
      return
    }

    reports.value = response.data?.reports ?? []
  } catch {
    errorMessage.value = 'Не удалось загрузить жалобы'
  } finally {
    isLoading.value = false
  }
}

const loadReportDetail = async (id: string) => {
  isDetailLoading.value = true

  try {
    const response = await adminApi.getReportDetail(id)

    if (response.error) {
      errorMessage.value = response.error
      return
    }

    selectedReportDetail.value = response.data ?? null
    resolutionNote.value = response.data?.resolution_note ?? ''
  } catch {
    errorMessage.value = 'Не удалось загрузить подробности жалобы'
  } finally {
    isDetailLoading.value = false
  }
}

const openReport = (report: AdminReport) => {
  selectedReportSummary.value = report
  selectedReportDetail.value = null
  resolutionNote.value = ''
  void loadReportDetail(report.id)
}

const closeModal = () => {
  selectedReportSummary.value = null
  selectedReportDetail.value = null
  resolutionNote.value = ''
  isDetailLoading.value = false
}

const refreshAndClose = async () => {
  closeModal()
  await loadReports()
}

const resolveReport = async () => {
  if (!selectedReportSummary.value || isProcessing.value) {
    return
  }

  isProcessing.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const response = await adminApi.resolveReport(
      selectedReportSummary.value.id,
      resolutionNote.value.trim()
    )

    if (response.error) {
      errorMessage.value = response.error
      return
    }

    successMessage.value = `Жалоба на ${selectedReportSummary.value.target_name} обработана`
    await refreshAndClose()
  } catch {
    errorMessage.value = 'Не удалось обработать жалобу'
  } finally {
    isProcessing.value = false
  }
}

const closeReport = async () => {
  if (!selectedReportSummary.value || isProcessing.value) {
    return
  }

  isProcessing.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const response = await adminApi.closeReport(selectedReportSummary.value.id)

    if (response.error) {
      errorMessage.value = response.error
      return
    }

    successMessage.value = `Жалоба на ${selectedReportSummary.value.target_name} закрыта`
    await refreshAndClose()
  } catch {
    errorMessage.value = 'Не удалось закрыть жалобу'
  } finally {
    isProcessing.value = false
  }
}

onMounted(() => {
  void loadReports().then(() => {
    requestAnimationFrame(() => {
      document.querySelectorAll('.animate-in').forEach((el) => {
        el.classList.add('animate-visible')
      })
    })
  })
})
</script>

<template>
  <section class="reports-management">
    <header class="management-header animate-in" style="animation-delay: 0ms">
      <h1 class="management-title">Жалобы</h1>
    </header>

    <div class="filters animate-in" style="animation-delay: 50ms">
      <button
        v-for="filter in filterOptions"
        :key="filter.value"
        type="button"
        class="filter-btn"
        :class="{ active: currentFilter === filter.value }"
        @click="currentFilter = filter.value"
      >
        <span>{{ filter.label }}</span>
        <span class="count">{{ filterCounts[filter.value] }}</span>
      </button>
    </div>

    <div v-if="errorMessage" class="alert alert-error animate-in" style="animation-delay: 100ms">
      {{ errorMessage }}
    </div>

    <div v-if="successMessage" class="alert alert-success animate-in" style="animation-delay: 100ms">
      {{ successMessage }}
    </div>

    <div v-if="isLoading" class="loading-state">
      <div class="spinner"></div>
      <div class="loading-text">Загружаем жалобы...</div>
    </div>

    <div v-else-if="filteredReports.length === 0" class="empty-state animate-in" style="animation-delay: 150ms">
      <div class="empty-icon" aria-hidden="true">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" />
          <line x1="12" y1="9" x2="12" y2="13" />
          <line x1="12" y1="17" x2="12.01" y2="17" />
        </svg>
      </div>
      <div class="empty-text">Жалоб не найдено</div>
      <div class="empty-subtext">Попробуйте выбрать другой фильтр</div>
    </div>

    <div v-else class="reports-list">
      <article
        v-for="(report, index) in filteredReports"
        :key="report.id"
        class="report-card row-animate"
        :class="report.status"
        :style="{ animationDelay: `${200 + index * 30}ms` }"
      >
        <div class="report-header">
          <div class="report-target">
            <span class="target-type">{{ formatTargetType(report.target_type) }}</span>
            <span class="target-name">{{ report.target_name }}</span>
          </div>
          <span class="report-status" :class="`status-${report.status}`">
            {{ statusLabel(report.status) }}
          </span>
        </div>

        <div class="report-reason">
          <span class="reason-label">Причина:</span>
          <span class="reason-value">{{ report.reason }}</span>
        </div>

        <div class="report-meta">
          <span class="reporter">{{ report.reporter_name }}</span>
          <span class="dot">•</span>
          <span class="mono">{{ formatDate(report.created_at) }}</span>
        </div>

        <div class="report-actions">
          <button type="button" class="btn btn-outline" @click="openReport(report)">
            Подробнее
          </button>
        </div>
      </article>
    </div>

    <div v-if="selectedReportSummary" class="modal-overlay" @click.self="closeModal">
      <div class="modal">
        <div class="modal-header">
          <h2 class="modal-title">{{ selectedReportSummary.target_name }}</h2>
          <button type="button" class="modal-close" @click="closeModal">&times;</button>
        </div>

        <div class="modal-content">
          <div v-if="isDetailLoading && !selectedReportDetail" class="loading-state">
            <div class="spinner"></div>
            <div class="loading-text">Загружаем подробности...</div>
          </div>

          <template v-else>
            <div
              class="status-banner"
              :class="`status-${activeReport?.status || selectedReportSummary.status}`"
            >
              {{ statusLabel(activeReport?.status || selectedReportSummary.status) }}
            </div>

            <div class="detail-row">
              <span class="detail-label">Объект</span>
              <span class="detail-value">
                {{ formatTargetType(activeReport?.target_type || selectedReportSummary.target_type) }}
              </span>
            </div>
            <div class="detail-row">
              <span class="detail-label">Название</span>
              <span class="detail-value">{{ activeReport?.target_name || selectedReportSummary.target_name }}</span>
            </div>
            <div class="detail-row">
              <span class="detail-label">Причина</span>
              <span class="detail-value description">{{ activeReport?.reason || selectedReportSummary.reason }}</span>
            </div>
            <div class="detail-row">
              <span class="detail-label">Автор жалобы</span>
              <span class="detail-value">{{ selectedReportDetail?.reporter?.name || selectedReportSummary.reporter_name }}</span>
            </div>
            <div class="detail-row">
              <span class="detail-label">Дата</span>
              <span class="detail-value mono">{{ formatDate(activeReport?.created_at || selectedReportSummary.created_at) }}</span>
            </div>
            <div v-if="selectedReportDetail?.description" class="detail-row">
              <span class="detail-label">Описание</span>
              <span class="detail-value description">{{ selectedReportDetail.description }}</span>
            </div>

            <div v-if="selectedReportSummary.status === 'open'" class="resolution-form">
              <h4 class="section-title">Обработать жалобу</h4>
              <textarea
                v-model="resolutionNote"
                class="resolution-textarea"
                rows="3"
                placeholder="Укажите примечание к решению (опционально)..."
              ></textarea>
            </div>

            <div v-else-if="selectedReportDetail" class="resolution-section">
              <h4 class="section-title">Решение</h4>
              <div class="detail-row">
                <span class="detail-label">Обработал</span>
                <span class="detail-value">{{ selectedReportDetail.resolved_by?.name || '—' }}</span>
              </div>
              <div class="detail-row">
                <span class="detail-label">Время</span>
                <span class="detail-value mono">{{ selectedReportDetail.resolved_at ? formatDate(selectedReportDetail.resolved_at) : '—' }}</span>
              </div>
              <div v-if="selectedReportDetail.resolution_note" class="detail-row">
                <span class="detail-label">Примечание</span>
                <span class="detail-value description">{{ selectedReportDetail.resolution_note }}</span>
              </div>
            </div>
          </template>
        </div>

        <div class="modal-footer">
          <button type="button" class="btn btn-secondary" @click="closeModal">
            Закрыть
          </button>
          <template v-if="(activeReport?.status || selectedReportSummary.status) === 'open'">
            <button type="button" class="btn btn-neutral" :disabled="isProcessing" @click="closeReport">
              {{ isProcessing ? '...' : 'Закрыть без рассмотрения' }}
            </button>
            <button type="button" class="btn btn-success" :disabled="isProcessing" @click="resolveReport">
              {{ isProcessing ? 'Обработка...' : 'Обработать' }}
            </button>
          </template>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped lang="scss">
.reports-management {
  max-width: 900px;
}

.management-header {
  margin-bottom: 20px;
}

.management-title {
  font-size: 24px;
  font-weight: 600;
  color: #ffffff;
  margin: 0;
  font-family: 'Unbounded', sans-serif;
}

.filters {
  display: flex;
  gap: 8px;
  margin-bottom: 24px;
  flex-wrap: wrap;
}

.filter-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  color: rgba(255, 255, 255, 0.7);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;

  &:hover {
    background: rgba(255, 255, 255, 0.08);
    color: #ffffff;
  }

  &.active {
    background: rgba(255, 255, 255, 0.12);
    border-color: rgba(255, 255, 255, 0.25);
    color: #ffffff;
  }

  .count {
    background: rgba(255, 255, 255, 0.1);
    padding: 2px 6px;
    border-radius: 3px;
    font-size: 11px;
    font-family: 'JetBrains Mono', monospace;
  }
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

.reports-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.report-card {
  background: #111111;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 8px;
  padding: 16px 20px;
  transition: all 0.2s;

  &:hover {
    border-color: rgba(255, 255, 255, 0.15);
  }

  &.open {
    border-left: 3px solid #ef4444;
  }
}

.report-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 10px;
}

.report-target {
  display: flex;
  align-items: center;
  gap: 8px;
}

.target-type {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: rgba(255, 255, 255, 0.5);
}

.target-name {
  font-size: 14px;
  font-weight: 500;
  color: #ffffff;
}

.report-status {
  font-size: 11px;
  padding: 4px 8px;
  border-radius: 4px;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.05em;

  &.status-open {
    background: rgba(239, 68, 68, 0.1);
    color: #ef4444;
  }

  &.status-resolved {
    background: rgba(34, 197, 94, 0.1);
    color: #22c55e;
  }

  &.status-closed {
    background: rgba(255, 255, 255, 0.08);
    color: rgba(255, 255, 255, 0.5);
  }
}

.report-reason {
  font-size: 13px;
  margin-bottom: 8px;
}

.reason-label {
  color: rgba(255, 255, 255, 0.5);
}

.reason-value {
  color: rgba(255, 255, 255, 0.8);
}

.report-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: rgba(255, 255, 255, 0.4);
  margin-bottom: 12px;
}

.reporter {
  color: rgba(255, 255, 255, 0.5);
}

.dot {
  color: rgba(255, 255, 255, 0.2);
}

.report-actions {
  display: flex;
  justify-content: flex-end;
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

.btn-neutral {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  color: rgba(255, 255, 255, 0.6);

  &:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.08);
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
  width: 520px;
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

.status-banner {
  padding: 10px 16px;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  text-align: center;
  margin-bottom: 16px;

  &.status-open {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.2);
    color: #ef4444;
  }

  &.status-resolved {
    background: rgba(34, 197, 94, 0.1);
    border: 1px solid rgba(34, 197, 94, 0.2);
    color: #22c55e;
  }

  &.status-closed {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 0.5);
  }
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
  word-break: break-word;

  &.description {
    line-height: 1.6;
    color: rgba(255, 255, 255, 0.8);
  }
}

.resolution-section {
  margin-top: 20px;
  padding-top: 20px;
  border-top: 1px solid rgba(255, 255, 255, 0.08);
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: #ffffff;
  margin: 0 0 12px;
}

.resolution-form {
  margin-top: 20px;
  padding-top: 20px;
  border-top: 1px solid rgba(255, 255, 255, 0.08);
}

.resolution-textarea {
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
  justify-content: flex-end;
  gap: 8px;
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
