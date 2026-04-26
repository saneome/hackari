<template>
  <div class="rating-panel">
    <div class="panel-header">
      <h3>Оценка решений команд</h3>
      <div v-if="!canRate" class="not-ended-warning">
        Хакатон ещё не завершён
      </div>
    </div>

    <div v-if="error" class="error-message">
      {{ error }}
      <button @click="error = null" class="close-btn">&times;</button>
    </div>

    <div v-if="loading" class="loading">
      <span class="spinner"></span>
      Загрузка...
    </div>

    <div v-else-if="submissions.length === 0" class="empty-state">
      <p>Нет решений для оценки</p>
      <p class="hint">Команды ещё не загрузили свои решения</p>
    </div>

    <div v-else class="submissions-list">
      <div
        v-for="submission in submissions"
        :key="submission.id"
        class="submission-card"
        :class="{ 'has-rating': !!submission.rating, 'is-final': submission.rating?.is_final }"
      >
        <div class="submission-header">
          <div class="team-info">
            <h4 class="team-name">{{ submission.team.name }}</h4>
            <span class="submission-title">{{ submission.title }}</span>
          </div>
          <div class="submission-status">
            <span v-if="submission.rating?.is_final" class="status-badge final">{{ submission.rating.total_score.toFixed(2) }} баллов</span>
            <span v-else-if="submission.rating" class="status-badge draft">Черновик</span>
            <span v-else class="status-badge pending">Не оценено</span>
          </div>
        </div>

        <div v-if="submission.description" class="submission-description">
          {{ submission.description }}
        </div>

        <div class="submission-links">
          <a v-if="submission.repo_url" :href="submission.repo_url" target="_blank" class="link">
            <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
              <path d="M12 2C6.477 2 2 6.477 2 12c0 4.42 2.865 8.166 6.839 9.489.5.092.682-.217.682-.482 0-.237-.008-.866-.013-1.7-2.782.603-3.369-1.342-3.369-1.342-.454-1.155-1.11-1.462-1.11-1.462-.908-.62.069-.608.069-.608 1.003.07 1.531 1.03 1.531 1.03.892 1.529 2.341 1.087 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.029-2.683-.103-.253-.446-1.272.098-2.65 0 0 .84-.269 2.75 1.025A9.578 9.578 0 0112 6.836c.85.004 1.705.114 2.504.336 1.909-1.294 2.747-1.025 2.747-1.025.546 1.378.203 2.398.1 2.651.64.699 1.028 1.592 1.028 2.683 0 3.842-2.339 4.687-4.566 4.935.359.309.678.919.678 1.852 0 1.336-.012 2.415-.012 2.743 0 .267.18.578.688.48C19.138 20.163 22 16.418 22 12c0-5.523-4.477-10-10-10z"/>
            </svg>
            Репозиторий
          </a>
          <a v-if="submission.demo_url" :href="submisison.demo_url" target="_blank" class="link">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" width="16" height="16" stroke-width="2">
              <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/>
              <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>
            </svg>
            Демо
          </a>
        </div>

        <div v-if="editingSubmissionId === submission.id" class="rating-form">
          <div class="criteria-scores">
            <div
              v-for="criterion in criteria"
              :key="criterion.id"
              class="score-row"
            >
              <div class="criterion-info">
                <span class="criterion-name">{{ criterion.name }}</span>
                <span class="criterion-weight">вес: {{ criterion.weight }}</span>
              </div>
              <div class="score-input-wrapper">
                <input
                  type="range"
                  :min="0"
                  :max="criterion.max_score"
                  v-model.number="ratingForm.scores[criterion.id]"
                  class="score-slider"
                />
                <input
                  type="number"
                  :min="0"
                  :max="criterion.max_score"
                  v-model.number="ratingForm.scores[criterion.id]"
                  class="score-number"
                />
                <span class="max-score">/ {{ criterion.max_score }}</span>
              </div>
            </div>
          </div>

          <div class="total-preview">
            <span class="total-label">Итоговый балл:</span>
            <span class="total-value">{{ calculatedTotal.toFixed(2) }}</span>
          </div>

          <div class="feedback-section">
            <label>Отзыв организатора:</label>
            <textarea
              v-model="ratingForm.feedback"
              placeholder="Напишите отзыв о решении команды (виден всем)"
              rows="3"
            />
          </div>

          <div class="final-checkbox">
            <label>
              <input type="checkbox" v-model="ratingForm.is_final" />
              <span>Финальная оценка (не подлежит апелляции)</span>
            </label>
          </div>

          <div class="form-actions">
            <button
              @click="saveRating(submission.id)"
              class="btn-save"
              :disabled="saving || !isRatingComplete"
            >
              {{ saving ? 'Сохранение...' : (submission.rating ? 'Обновить' : 'Сохранить') }}
            </button>
            <button @click="cancelEdit" class="btn-cancel">Отмена</button>
          </div>
        </div>

        <div v-else-if="submission.rating" class="rating-display">
          <div class="scores-breakdown">
            <div
              v-for="score in submission.rating.scores"
              :key="score.criteria_id"
              class="score-item"
            >
              <span class="score-name">{{ score.criteria_name }}</span>
              <div class="score-bar">
                <div
                  class="score-fill"
                  :style="{ width: `${(score.score / score.max_score) * 100}%` }"
                />
                <span class="score-text">{{ score.score }} ({{ score.weighted_score.toFixed(2) }})</span>
              </div>
            </div>
          </div>

          <div v-if="submission.rating.feedback" class="rating-feedback">
            <label>Отзыв:</label>
            <p>{{ submission.rating.feedback }}</p>
          </div>

          <div class="rating-meta">
            <span v-if="submission.rating.is_final" class="final-badge">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" width="14" height="14" stroke-width="2">
                <polyline points="20 6 9 17 4 12"/>
              </svg>
              Финальная оценка
            </span>
            <span v-else class="appeal-badge">Под апелляцией</span>
          </div>

          <button @click="startEdit(submission)" class="btn-edit">
            {{ canRate ? 'Редактировать' : 'Просмотреть' }}
          </button>
        </div>

        <div v-else class="no-rating">
          <p>Пока нет оценки</p>
          <button
            v-if="canRate"
            @click="startEdit(submission)"
            class="btn-rate"
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" width="16" height="16" stroke-width="2">
              <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
              <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
            </svg>
            Оценить
          </button>
          <p v-else class="hint">Оценивание станет доступно после окончания хакатона</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { ratingApi, type SubmissionWithRating, type RatingCriteria, type ScoreInput } from '@/services/api'

const props = defineProps<{
  hackathonId: string
  canRate: boolean
}>()

const emit = defineEmits<{
  'rating-updated': []
}>()

const submissions = ref<SubmissionWithRating[]>([])
const criteria = ref<RatingCriteria[]>([])
const loading = ref(false)
const saving = ref(false)
const error = ref<string | null>(null)

const editingSubmissionId = ref<string | null>(null)
const ratingForm = ref<{
  scores: Record<string, number>
  feedback: string
  is_final: boolean
}>({
  scores: {},
  feedback: '',
  is_final: true
})

watch(() => props.hackathonId, loadData, { immediate: true })

async function loadData() {
  loading.value = true
  error.value = null
  try {
    // Load criteria and submissions in parallel
    const [criteriaRes, submissionsRes] = await Promise.all([
      ratingApi.getCriteria(props.hackathonId),
      ratingApi.getSubmissionsWithRatings(props.hackathonId)
    ])

    if (criteriaRes.data) {
      criteria.value = criteriaRes.data
    }

    if (submissionsRes.data) {
      submissions.value = submissionsRes.data.submissions
    }
  } catch (e: any) {
    error.value = 'Ошибка загрузки данных'
  } finally {
    loading.value = false
  }
}

const calculatedTotal = computed(() => {
  let total = 0
  for (const criterion of criteria.value) {
    const score = ratingForm.value.scores[criterion.id] || 0
    total += score * criterion.weight
  }
  return total
})

const isRatingComplete = computed(() => {
  for (const criterion of criteria.value) {
    const score = ratingForm.value.scores[criterion.id]
    if (score === undefined || score === null) {
      return false
    }
  }
  return true
})

function startEdit(submission: SubmissionWithRating) {
  editingSubmissionId.value = submission.id

  // Initialize scores
  const scores: Record<string, number> = {}
  for (const criterion of criteria.value) {
    if (submission.rating) {
      const existingScore = submission.rating.scores.find(
        s => s.criteria_id === criterion.id
      )
      scores[criterion.id] = existingScore?.score || 0
    } else {
      scores[criterion.id] = 0
    }
  }

  ratingForm.value = {
    scores,
    feedback: submission.rating?.feedback || '',
    is_final: submission.rating?.is_final ?? true
  }
}

function cancelEdit() {
  editingSubmissionId.value = null
  ratingForm.value = {
    scores: {},
    feedback: '',
    is_final: true
  }
}

async function saveRating(submissionId: string) {
  if (!isRatingComplete.value) return

  saving.value = true
  error.value = null

  const scoreInputs: ScoreInput[] = Object.entries(ratingForm.value.scores).map(
    ([criteriaId, score]) => ({ criteria_id: criteriaId, score })
  )

  const existingSubmission = submissions.value.find(s => s.id === submissionId)

  if (existingSubmission?.rating) {
    // Update existing rating
    const res = await ratingApi.updateRating(
      props.hackathonId,
      existingSubmission.rating.id,
      {
        scores: scoreInputs,
        feedback: ratingForm.value.feedback || undefined,
        is_final: ratingForm.value.is_final
      }
    )

    if (res.error) {
      error.value = res.error
      saving.value = false
      return
    }
  } else {
    // Create new rating
    const res = await ratingApi.createRating(
      props.hackathonId,
      {
        submission_id: submissionId,
        scores: scoreInputs,
        feedback: ratingForm.value.feedback || undefined,
        is_final: ratingForm.value.is_final
      }
    )

    if (res.error) {
      error.value = res.error
      saving.value = false
      return
    }
  }

  editingSubmissionId.value = null
  await loadData()
  emit('rating-updated')
  saving.value = false
}
</script>

<style scoped>
.rating-panel {
  background: var(--surface-color);
  border-radius: 12px;
  padding: 1.5rem;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.panel-header h3 {
  margin: 0;
  color: var(--text-primary);
}

.not-ended-warning {
  padding: 0.5rem 1rem;
  background: #fff3cd;
  color: #856404;
  border-radius: 6px;
  font-size: 0.9rem;
}

.empty-state {
  text-align: center;
  padding: 3rem 1rem;
  color: var(--text-secondary);
}

.empty-state .hint {
  font-size: 0.9rem;
  opacity: 0.7;
}

.submissions-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.submission-card {
  background: var(--bg-color);
  border-radius: 12px;
  padding: 1.5rem;
  border: 2px solid var(--border-color);
  transition: all 0.2s;
}

.submission-card.has-rating {
  border-color: var(--success-color);
}

.submission-card.is-final {
  border-color: var(--primary-color);
  background: linear-gradient(135deg, var(--bg-color) 0%, rgba(var(--primary-rgb), 0.05) 100%);
}

.submission-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 1rem;
}

.team-info {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.team-name {
  margin: 0;
  font-size: 1.1rem;
  color: var(--text-primary);
}

.submission-title {
  font-size: 0.9rem;
  color: var(--text-secondary);
}

.status-badge {
  padding: 0.4rem 0.8rem;
  border-radius: 20px;
  font-size: 0.85rem;
  font-weight: 500;
}

.status-badge.pending {
  background: var(--surface-dark);
  color: var(--text-secondary);
}

.status-badge.draft {
  background: #fff3cd;
  color: #856404;
}

.status-badge.final {
  background: var(--success-color);
  color: white;
}

.submission-description {
  color: var(--text-secondary);
  font-size: 0.95rem;
  margin-bottom: 1rem;
  line-height: 1.5;
}

.submission-links {
  display: flex;
  gap: 1rem;
  margin-bottom: 1rem;
}

.link {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: var(--primary-color);
  text-decoration: none;
  font-size: 0.9rem;
  transition: opacity 0.2s;
}

.link:hover {
  opacity: 0.8;
}

.rating-form {
  background: var(--surface-color);
  border-radius: 8px;
  padding: 1.5rem;
  margin-top: 1rem;
}

.criteria-scores {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.score-row {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.criterion-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.criterion-name {
  font-weight: 500;
  color: var(--text-primary);
}

.criterion-weight {
  font-size: 0.8rem;
  color: var(--text-secondary);
}

.score-input-wrapper {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.score-slider {
  flex: 1;
  height: 6px;
  -webkit-appearance: none;
  appearance: none;
  background: var(--surface-dark);
  border-radius: 3px;
  outline: none;
}

.score-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--primary-color);
  cursor: pointer;
}

.score-slider::-moz-range-thumb {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--primary-color);
  cursor: pointer;
  border: none;
}

.score-number {
  width: 60px;
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  text-align: center;
  font-size: 1rem;
}

.max-score {
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.total-preview {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  background: var(--surface-dark);
  border-radius: 8px;
  margin-bottom: 1rem;
}

.total-label {
  font-weight: 500;
  color: var(--text-primary);
}

.total-value {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--primary-color);
}

.feedback-section {
  margin-bottom: 1rem;
}

.feedback-section label {
  display: block;
  margin-bottom: 0.5rem;
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.feedback-section textarea {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--bg-color);
  color: var(--text-primary);
  resize: vertical;
}

.final-checkbox {
  margin-bottom: 1rem;
}

.final-checkbox label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  color: var(--text-primary);
}

.form-actions {
  display: flex;
  gap: 0.75rem;
}

.btn-save, .btn-cancel {
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  font-weight: 500;
}

.btn-save {
  background: var(--primary-color);
  color: white;
  border: none;
}

.btn-save:hover:not(:disabled) {
  background: var(--primary-dark);
}

.btn-save:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-cancel {
  background: transparent;
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
}

.btn-cancel:hover {
  border-color: var(--text-primary);
  color: var(--text-primary);
}

.rating-display {
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 1px solid var(--border-color);
}

.scores-breakdown {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  margin-bottom: 1rem;
}

.score-item {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.score-name {
  width: 120px;
  font-size: 0.9rem;
  color: var(--text-secondary);
}

.score-bar {
  flex: 1;
  height: 24px;
  background: var(--surface-dark);
  border-radius: 4px;
  position: relative;
  overflow: hidden;
}

.score-fill {
  height: 100%;
  background: var(--primary-color);
  border-radius: 4px;
  transition: width 0.3s ease;
}

.score-text {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 0.75rem;
  color: var(--text-primary);
  font-weight: 500;
}

.rating-feedback {
  background: var(--surface-dark);
  padding: 1rem;
  border-radius: 8px;
  margin-bottom: 1rem;
}

.rating-feedback label {
  font-size: 0.85rem;
  color: var(--text-secondary);
}

.rating-feedback p {
  margin: 0.5rem 0 0 0;
  color: var(--text-primary);
  line-height: 1.5;
}

.rating-meta {
  display: flex;
  gap: 0.75rem;
  margin-bottom: 1rem;
}

.final-badge, .appeal-badge {
  display: flex;
  align-items: center;
  gap: 0.35rem;
  padding: 0.4rem 0.75rem;
  border-radius: 20px;
  font-size: 0.8rem;
  font-weight: 500;
}

.final-badge {
  background: var(--success-color);
  color: white;
}

.appeal-badge {
  background: #fff3cd;
  color: #856404;
}

.btn-edit {
  padding: 0.5rem 1rem;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s;
}

.btn-edit:hover {
  border-color: var(--primary-color);
  color: var(--primary-color);
}

.no-rating {
  margin-top: 1rem;
  padding: 1.5rem;
  text-align: center;
  background: var(--surface-dark);
  border-radius: 8px;
}

.no-rating p {
  margin: 0 0 1rem 0;
  color: var(--text-secondary);
}

.no-rating .hint {
  font-size: 0.85rem;
  margin-top: 0.5rem;
}

.btn-rate {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  background: var(--primary-color);
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  font-weight: 500;
}

.btn-rate:hover {
  background: var(--primary-dark);
}

.error-message {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1rem;
  background: var(--error-bg);
  color: var(--error-color);
  border-radius: 6px;
  margin-bottom: 1rem;
}

.close-btn {
  background: none;
  border: none;
  color: var(--error-color);
  font-size: 1.5rem;
  cursor: pointer;
  padding: 0;
  line-height: 1;
}

.loading {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 3rem;
  color: var(--text-secondary);
}

.spinner {
  width: 20px;
  height: 20px;
  border: 2px solid var(--border-color);
  border-top-color: var(--primary-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
