<script setup lang="ts">
import { ref, watch, nextTick } from 'vue'
import { gsap } from 'gsap'
import { reportApi } from '@/services/api'
import { useModal } from '@/composables/useModal'
import { useScrollLock } from '@/composables/useScrollLock'
import CustomSelect from './CustomSelect.vue'

const { alert } = useModal()

const props = defineProps<{
  show: boolean
  targetType: 'hackathon' | 'organizer' | 'team' | 'user'
  targetId: string
  targetName: string
}>()

const emit = defineEmits<{
  (e: 'update:show', value: boolean): void
  (e: 'close'): void
  (e: 'submitted'): void
}>()

const isVisible = ref(false)

useScrollLock(isVisible)

const animateOpen = () => {
  nextTick(() => {
    const overlay = document.querySelector('.report-modal-overlay') as HTMLElement | null
    const modal = document.querySelector('.report-modal') as HTMLElement | null
    if (!overlay || !modal) return

    gsap.fromTo(overlay,
      { opacity: 0 },
      { opacity: 1, duration: 0.2, ease: 'power2.out' }
    )

    gsap.fromTo(modal,
      { y: 30, opacity: 0, scale: 0.95 },
      { y: 0, opacity: 1, scale: 1, duration: 0.35, ease: 'back.out(1.2)' }
    )
  })
}

const animateClose = (cb: () => void) => {
  const overlay = document.querySelector('.report-modal-overlay') as HTMLElement | null
  const modal = document.querySelector('.report-modal') as HTMLElement | null

  if (!overlay || !modal) {
    cb()
    return
  }

  const tl = gsap.timeline({ onComplete: cb })
  tl.to(modal, { y: 20, opacity: 0, scale: 0.98, duration: 0.2, ease: 'power2.in' })
  tl.to(overlay, { opacity: 0, duration: 0.15, ease: 'power2.in' }, '<')
}

watch(() => props.show, (val) => {
  if (val) {
    reason.value = ''
    description.value = ''
    error.value = ''
    isVisible.value = true
    animateOpen()
  } else if (isVisible.value) {
    animateClose(() => {
      isVisible.value = false
    })
  }
})

const reason = ref('')
const description = ref('')
const isSubmitting = ref(false)
const error = ref('')

const reasons = [
  { value: 'spam', label: 'Спам' },
  { value: 'inappropriate', label: 'Неприемлемый контент' },
  { value: 'fraud', label: 'Мошенничество' },
  { value: 'fake', label: 'Фейковая информация' },
  { value: 'duplicate', label: 'Дубликат' },
  { value: 'other', label: 'Другое' },
]

const submitReport = async () => {
  if (!reason.value) {
    error.value = 'Выберите причину жалобы'
    return
  }

  isSubmitting.value = true
  error.value = ''

  const response = await reportApi.create({
    target_type: props.targetType,
    target_id: props.targetId,
    reason: reason.value,
    description: description.value || undefined,
  })

  isSubmitting.value = false

  if (response.error) {
    error.value = response.error
  } else {
    reason.value = ''
    description.value = ''
    emit('submitted')
    emit('update:show', false)
    emit('close')
    await alert({
      title: 'Жалоба отправлена',
      message: 'Когда модераторы рассмотрят жалобу, вы получите письмо от didorenkoalexander@yandex.ru. Если не увидите его во «Входящих» — проверьте папку «Спам».',
      type: 'success'
    })
  }
}

const close = () => {
  reason.value = ''
  description.value = ''
  error.value = ''
  emit('update:show', false)
  emit('close')
}

const onOverlayClick = (e: MouseEvent) => {
  if (e.target === e.currentTarget) close()
}
</script>

<template>
  <Teleport to="body">
    <div v-if="isVisible" class="report-modal-overlay" @click="onOverlayClick">
      <div class="report-modal">
        <div class="modal-header">
          <h2 class="modal-title">Пожаловаться</h2>
          <button class="close-button" @click="close">&times;</button>
        </div>

        <div class="target-info">
          <span class="target-label">Жалоба на:</span>
          <span class="target-type">{{
            targetType === 'hackathon' ? 'Хакатон' :
            targetType === 'organizer' ? 'Организатора' :
            targetType === 'team' ? 'Команду' : 'Пользователя'
          }}</span>
          <span class="target-name">{{ targetName }}</span>
        </div>

        <div class="form-group">
          <label class="form-label">Причина жалобы <span class="required">*</span></label>
          <CustomSelect
            v-model="reason"
            :options="reasons"
            placeholder="Выберите причину"
            :z-index="10001"
          />
        </div>

        <div class="form-group">
          <label class="form-label">Описание (необязательно)</label>
          <textarea
            v-model="description"
            class="form-textarea"
            rows="4"
            placeholder="Опишите подробнее, что вызывает нарушение..."
          ></textarea>
        </div>

        <div v-if="error" class="error-message">{{ error }}</div>

        <div class="form-actions">
          <button class="btn btn-secondary" @click="close">Отмена</button>
          <button
            class="btn btn-primary"
            :disabled="isSubmitting"
            @click="submitReport"
          >
            {{ isSubmitting ? 'Отправка...' : 'Отправить жалобу' }}
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<style scoped lang="scss">
.report-modal-overlay {
  position: fixed;
  inset: 0;
  z-index: 10000;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
  background: rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(4px);
}

.report-modal {
  width: 420px;
  max-width: 90vw;
  background: #1a1a1a;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 16px;
  padding: 24px;
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  padding-bottom: 16px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
}

.modal-title {
  font-size: 18px;
  font-weight: 600;
  color: #ffffff;
  margin: 0;
}

.close-button {
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

.target-info {
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 8px;
  padding: 12px 16px;
  margin-bottom: 20px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.target-label {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.5);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.target-type {
  font-size: 13px;
  color: rgba(255, 255, 255, 0.7);
}

.target-name {
  font-size: 14px;
  font-weight: 500;
  color: #ffffff;
}

.form-group {
  margin-bottom: 16px;
}

.form-label {
  display: block;
  font-size: 13px;
  color: rgba(255, 255, 255, 0.7);
  margin-bottom: 6px;

  .required {
    color: #ff4444;
  }
}

.form-select,
.form-textarea {
  width: 100%;
  background: #111111;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  padding: 10px 12px;
  color: #ffffff;
  font-size: 14px;
  font-family: inherit;
  transition: border-color 0.2s;

  &:focus {
    outline: none;
    border-color: rgba(255, 255, 255, 0.3);
  }

  &::placeholder {
    color: rgba(255, 255, 255, 0.3);
  }
}

.form-select {
  cursor: pointer;

  option {
    background: #111111;
    color: #ffffff;
  }
}

.form-textarea {
  resize: vertical;
  min-height: 80px;
}

.form-actions {
  display: flex;
  gap: 10px;
  justify-content: flex-end;
  margin-top: 24px;
}

.btn {
  padding: 10px 20px;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: none;

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
}

.btn-secondary {
  background: rgba(255, 255, 255, 0.08);
  color: #ffffff;

  &:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.12);
  }
}

.btn-primary {
  background: #ffffff;
  color: #0a0a0a;

  &:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.9);
  }
}

.error-message {
  color: #ff4444;
  font-size: 13px;
  margin-top: 12px;
}
</style>
