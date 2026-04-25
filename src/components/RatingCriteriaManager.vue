<template>
  <div class="criteria-manager">
    <div class="header">
      <h3>Критерии оценивания</h3>
      <div class="weight-info" :class="{ 'weight-valid': isWeightValid, 'weight-invalid': !isWeightValid }">
        Сумма весов: {{ totalWeight.toFixed(2) }} / 1.00
      </div>
    </div>

    <div v-if="!canManage" class="access-note">
      Только организатор хакатона может редактировать критерии. Список доступен только для просмотра.
    </div>

    <div v-if="error" class="error-message">
      {{ error }}
      <button @click="error = null" class="close-btn">&times;</button>
    </div>

    <div v-if="criteria.length === 0 && !loading" class="empty-state">
      <p>Нет настроенных критериев</p>
      <p class="hint">
        {{ canManage ? 'Добавьте критерии, по которым будете оценивать решения команд' : 'Настроить критерии может только организатор хакатона' }}
      </p>
    </div>

    <div v-else class="criteria-list">
      <div
        v-for="(criterion, index) in sortedCriteria"
        :key="criterion.id"
        class="criteria-item"
        :class="{ 'dragging': draggingId === criterion.id }"
        :draggable="canManage"
        @dragstart="canManage && dragStart(criterion.id)"
        @dragover.prevent="canManage && dragOver(index)"
        @drop="canManage && drop(index)"
        @dragend="dragEnd"
      >
        <div v-if="canManage" class="drag-handle">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="9" cy="6" r="1" fill="currentColor"/>
            <circle cx="9" cy="12" r="1" fill="currentColor"/>
            <circle cx="9" cy="18" r="1" fill="currentColor"/>
            <circle cx="15" cy="6" r="1" fill="currentColor"/>
            <circle cx="15" cy="12" r="1" fill="currentColor"/>
            <circle cx="15" cy="18" r="1" fill="currentColor"/>
          </svg>
        </div>

        <div v-if="canManage && editingId === criterion.id" class="edit-form">
          <BaseInput
            v-model="editForm.name"
            label="Название критерия"
            placeholder="Например: Качество кода"
            required
          />
          <BaseInput
            v-model="editForm.description"
            type="text"
            label="Описание"
            placeholder="Опциональное описание критерия"
          />
          <div class="form-row">
            <BaseNumberInput
              v-model="editForm.weight"
              label="Вес"
              :min="0"
              :max="1"
              :step="0.05"
            />
            <BaseNumberInput
              v-model="editForm.max_score"
              label="Макс. балл"
              :min="1"
              :max="100"
              :step="1"
            />
          </div>
          <div class="form-actions">
            <button @click="saveEdit(criterion.id)" class="btn-save" :disabled="saving">
              {{ saving ? 'Сохранение...' : 'Сохранить' }}
            </button>
            <button @click="cancelEdit" class="btn-cancel">Отмена</button>
          </div>
        </div>

        <div v-else class="criteria-content">
          <div class="criteria-main">
            <span class="criteria-name">{{ criterion.name }}</span>
            <span v-if="criterion.description" class="criteria-desc">{{ criterion.description }}</span>
          </div>
          <div class="criteria-meta">
            <span class="badge weight">вес: {{ criterion.weight }}</span>
            <span class="badge max-score">макс: {{ criterion.max_score }}</span>
          </div>
          <div v-if="canManage" class="criteria-actions">
            <button @click="startEdit(criterion)" class="btn-icon" title="Редактировать">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
                <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
              </svg>
            </button>
            <button @click="deleteCriteria(criterion.id)" class="btn-icon delete" title="Удалить">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="3 6 5 6 21 6"/>
                <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>

    <div v-if="canManage && showAddForm" class="add-form">
      <h4>Добавить критерий</h4>
      <BaseInput
        v-model="newForm.name"
        label="Название критерия"
        placeholder="Например: Качество кода"
        required
      />
      <BaseInput
        v-model="newForm.description"
        type="text"
        label="Описание"
        placeholder="Опциональное описание критерия"
      />
      <div class="form-row">
        <BaseNumberInput
          v-model="newForm.weight"
          label="Вес (0-1)"
          :min="0"
          :max="1"
          :step="0.05"
        />
        <BaseNumberInput
          v-model="newForm.max_score"
          label="Максимальный балл"
          :min="1"
          :max="100"
          :step="1"
        />
      </div>
      <div class="form-actions">
        <button @click="addCriteria" class="btn-save" :disabled="saving || !newForm.name">
          {{ saving ? 'Добавление...' : 'Добавить' }}
        </button>
        <button @click="showAddForm = false" class="btn-cancel">Отмена</button>
      </div>
    </div>

    <div v-else-if="canManage" class="actions">
      <button @click="showAddForm = true" class="btn-add">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="12" y1="5" x2="12" y2="19"/>
          <line x1="5" y1="12" x2="19" y2="12"/>
        </svg>
        Добавить критерий
      </button>
    </div>

    <div v-if="loading" class="loading">
      <span class="spinner"></span>
      Загрузка...
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { ratingApi, type RatingCriteria } from '@/services/api'
import BaseInput from '@/components/ui/BaseInput.vue'
import BaseNumberInput from '@/components/ui/BaseNumberInput.vue'
import { useModal } from '@/composables/useModal'

const { confirm } = useModal()

const props = withDefaults(defineProps<{
  hackathonId: string
  canManage?: boolean
}>(), {
  canManage: false,
})

const canManage = computed(() => props.canManage)

const emit = defineEmits<{
  'criteria-updated': []
}>()

const criteria = ref<RatingCriteria[]>([])
const loading = ref(false)
const saving = ref(false)
const error = ref<string | null>(null)
const showAddForm = ref(false)

const editingId = ref<string | null>(null)
const editForm = ref({
  name: '',
  description: '',
  weight: 0.2,
  max_score: 10
})

const newForm = ref({
  name: '',
  description: '',
  weight: 0.2,
  max_score: 10
})

// Drag and drop
const draggingId = ref<string | null>(null)
const dragOverIndex = ref<number | null>(null)

const sortedCriteria = computed(() => {
  return [...criteria.value].sort((a, b) => a.sort_order - b.sort_order)
})

const totalWeight = computed(() => {
  return criteria.value.reduce((sum, c) => sum + c.weight, 0)
})

const isWeightValid = computed(() => {
  return Math.abs(totalWeight.value - 1.0) < 0.001
})

watch(() => props.hackathonId, loadCriteria, { immediate: true })

watch(canManage, (value) => {
  if (!value) {
    showAddForm.value = false
    editingId.value = null
  }
})

async function loadCriteria() {
  loading.value = true
  error.value = null
  try {
    const res = await ratingApi.getCriteria(props.hackathonId)
    if (res.data) {
      criteria.value = res.data.criteria || res.data || []
    } else {
      criteria.value = []
    }
  } catch (e: any) {
    error.value = 'Ошибка загрузки критериев'
  } finally {
    loading.value = false
  }
}

async function addCriteria() {
  if (!canManage.value) return
  if (!newForm.value.name) return

  saving.value = true
  error.value = null
  try {
    const res = await ratingApi.createCriteria(props.hackathonId, newForm.value)
    if (res.data) {
      criteria.value.push(res.data)
      showAddForm.value = false
      newForm.value = { name: '', description: '', weight: 0.2, max_score: 10 }
      emit('criteria-updated')
    }
  } catch (e: any) {
    error.value = 'Ошибка добавления критерия'
  } finally {
    saving.value = false
  }
}

function startEdit(criterion: RatingCriteria) {
  if (!canManage.value) return
  editingId.value = criterion.id
  editForm.value = {
    name: criterion.name,
    description: criterion.description || '',
    weight: criterion.weight,
    max_score: criterion.max_score
  }
}

function cancelEdit() {
  editingId.value = null
}

async function saveEdit(criteriaId: string) {
  if (!canManage.value) return
  saving.value = true
  error.value = null
  try {
    const res = await ratingApi.updateCriteria(props.hackathonId, criteriaId, editForm.value)
    if (res.data) {
      const index = criteria.value.findIndex(c => c.id === criteriaId)
      if (index !== -1) {
        criteria.value[index] = res.data
      }
      editingId.value = null
      emit('criteria-updated')
    }
  } catch (e: any) {
    error.value = 'Ошибка обновления критерия'
  } finally {
    saving.value = false
  }
}

async function deleteCriteria(criteriaId: string) {
  if (!canManage.value) return
  const ok = await confirm({
    title: 'Удалить критерий',
    message: 'Удалить этот критерий?',
    type: 'error',
    confirmText: 'Удалить',
    cancelText: 'Отмена',
  })
  if (!ok) return

  error.value = null
  try {
    await ratingApi.deleteCriteria(props.hackathonId, criteriaId)
    criteria.value = criteria.value.filter(c => c.id !== criteriaId)
    emit('criteria-updated')
  } catch (e: any) {
    error.value = 'Ошибка удаления критерия'
  }
}

// Drag and drop handlers
function dragStart(id: string) {
  if (!canManage.value) return
  draggingId.value = id
}

function dragOver(index: number) {
  if (!canManage.value) return
  dragOverIndex.value = index
}

async function drop(targetIndex: number) {
  if (!canManage.value) return
  if (!draggingId.value || dragOverIndex.value === null) return

  const draggedIndex = sortedCriteria.value.findIndex(c => c.id === draggingId.value)
  if (draggedIndex === targetIndex) return

  const reordered = [...sortedCriteria.value]
  const [moved] = reordered.splice(draggedIndex, 1)
  reordered.splice(targetIndex, 0, moved)

  // Update sort orders
  const newOrder = reordered.map((c, i) => ({ ...c, sort_order: i }))
  criteria.value = newOrder

  try {
    await ratingApi.reorderCriteria(props.hackathonId, {
      criteria_ids: newOrder.map(c => c.id)
    })
    emit('criteria-updated')
  } catch (e: any) {
    error.value = 'Ошибка изменения порядка'
    loadCriteria() // Reload to get original order
  }
}

function dragEnd() {
  draggingId.value = null
  dragOverIndex.value = null
}
</script>

<style scoped lang="scss">
@use '../styles/variables' as *;

.criteria-manager {
  background: var(--surface-color);
  border-radius: 12px;
  padding: 1.5rem;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.header h3 {
  margin: 0;
  color: var(--text-primary);
}

.weight-info {
  font-size: 0.9rem;
  padding: 0.25rem 0.75rem;
  border-radius: 20px;
  background: var(--surface-dark);
}

.weight-valid {
  color: var(--success-color);
}

.weight-invalid {
  color: var(--error-color);
}

.empty-state {
  text-align: center;
  padding: 2rem;
  color: var(--text-secondary);
}

.empty-state .hint {
  font-size: 0.9rem;
  opacity: 0.7;
}

.access-note {
  margin-bottom: 1rem;
  padding: 0.75rem 1rem;
  border-radius: 8px;
  background: rgba(212, 255, 0, 0.08);
  border: 1px solid rgba(212, 255, 0, 0.18);
  color: var(--text-secondary);
}

.criteria-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.criteria-item {
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
  padding: 1rem;
  background: var(--bg-color);
  border-radius: 8px;
  border: 1px solid var(--border-color);
  transition: all 0.2s ease;
}

.criteria-item:hover {
  border-color: var(--primary-color);
}

.criteria-item.dragging {
  opacity: 0.5;
}

.drag-handle {
  cursor: grab;
  color: var(--text-secondary);
  padding: 0.25rem;
}

.drag-handle:active {
  cursor: grabbing;
}

.drag-handle svg {
  width: 20px;
  height: 20px;
}

.criteria-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.criteria-main {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.criteria-name {
  font-weight: 600;
  color: var(--text-primary);
}

.criteria-desc {
  font-size: 0.9rem;
  color: var(--text-secondary);
}

.criteria-meta {
  display: flex;
  gap: 0.5rem;
}

.badge {
  font-size: 0.75rem;
  padding: 0.15rem 0.5rem;
  border-radius: 4px;
  background: var(--surface-dark);
}

.criteria-actions {
  display: flex;
  gap: 0.5rem;
}

.btn-icon {
  padding: 0.5rem;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s;
}

.btn-icon:hover {
  background: var(--surface-dark);
  color: var(--text-primary);
}

.btn-icon.delete:hover {
  background: var(--error-color);
  color: white;
}

.btn-icon svg {
  width: 16px;
  height: 16px;
}

.edit-form, .add-form {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.edit-form :deep(.input-label),
.add-form :deep(.input-label) {
  font-family: $font-display;
  font-size: 11px;
}

.form-row {
  display: flex;
  gap: 1.5rem;
  align-items: flex-start;
}

.form-row :deep(.base-number-input) {
  flex: 1;
}

.btn-save, .btn-cancel {
  padding: 0.5rem 1rem;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
  font-family: $font-body;
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

.actions {
  margin-top: 1rem;
}

.btn-add {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  border: 2px dashed var(--border-color);
  border-radius: 8px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s;
  font-family: $font-body;
}

.btn-add:hover {
  border-color: var(--primary-color);
  color: var(--primary-color);
}

.btn-add svg {
  width: 18px;
  height: 18px;
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
  gap: 0.5rem;
  padding: 1rem;
  color: var(--text-secondary);
}

.spinner {
  width: 16px;
  height: 16px;
  border: 2px solid var(--border-color);
  border-top-color: var(--primary-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
