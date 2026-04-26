<template>
  <div class="wizard-page">
    <div class="container">
      <div class="wizard-card" ref="wizardCard">
        <div class="wizard-header">
          <h1 class="page-title">{{ isEditMode ? 'Редактирование хакатона' : 'Создание хакатона' }}</h1>
          <p class="page-subtitle">{{ isEditMode ? 'Внесите изменения в ваш хакатон' : 'Заполните информацию о вашем хакатоне' }}</p>

          <!-- Stepper -->
          <div class="stepper">
            <div
              v-for="(step, index) in steps"
              :key="index"
              class="step"
              :class="{
                'active': currentStep === index,
                'completed': currentStep > index
              }"
            >
              <div class="step-number">
                <span v-if="currentStep > index">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                    <polyline points="20 6 9 17 4 12"/>
                  </svg>
                </span>
                <span v-else>{{ index + 1 }}</span>
              </div>
              <div class="step-label">{{ step.label }}</div>
              <div v-if="index < steps.length - 1" class="step-connector"></div>
            </div>
          </div>
        </div>

        <form @submit.prevent="handleSubmit" class="wizard-form">
          <!-- Step 1: Basic Information -->
          <div v-if="currentStep === 0" class="form-step" ref="stepContent">
            <div class="form-section">
              <h3 class="section-title">Основная информация</h3>

              <div class="form-field" :class="{ 'has-error': errors.title }">
                <label class="field-label">
                  Название хакатона <span class="required">*</span>
                </label>
                <input
                  v-model="form.title"
                  type="text"
                  class="field-input"
                  placeholder="Например: Hackari Spring 2025"
                  @blur="validateField('title')"
                />
                <span v-if="errors.title" class="error-text">{{ errors.title }}</span>
              </div>

              <div class="form-field" :class="{ 'has-error': errors.description }">
                <label class="field-label">
                  Описание <span class="required">*</span>
                </label>
                <textarea
                  v-model="form.description"
                  class="field-textarea"
                  rows="4"
                  placeholder="Расскажите о хакатоне, его целях и особенностях..."
                  @blur="validateField('description')"
                />
                <span v-if="errors.description" class="error-text">{{ errors.description }}</span>
              </div>

              <div class="form-field">
                <label class="field-label">URL баннера</label>
                <input
                  v-model="form.banner_url"
                  type="url"
                  class="field-input"
                  placeholder="https://example.com/banner.jpg"
                />
              </div>
            </div>

            <div class="form-section">
              <h3 class="section-title">Формат проведения</h3>

              <div class="form-row">
                <div class="form-field" :class="{ 'has-error': errors.location_type }">
                  <label class="field-label">
                    Тип проведения <span class="required">*</span>
                  </label>
                  <CustomSelect
                    v-model="form.location_type"
                    :options="locationTypeOptions"
                    placeholder="Выберите тип"
                    @blur="validateField('location_type')"
                  />
                  <span v-if="errors.location_type" class="error-text">{{ errors.location_type }}</span>
                </div>

                <div class="form-field">
                  <label class="field-label">Город</label>
                  <input
                    v-model="form.city"
                    type="text"
                    class="field-input"
                    placeholder="Москва"
                    :disabled="form.location_type === 'online'"
                  />
                </div>

                <div class="form-field">
                  <label class="field-label">Место проведения</label>
                  <input
                    v-model="form.venue"
                    type="text"
                    class="field-input"
                    placeholder="Адрес или название площадки"
                    :disabled="form.location_type === 'online'"
                  />
                </div>
              </div>
            </div>

            <div class="form-section">
              <h3 class="section-title">Даты</h3>

              <div class="form-row">
                <div class="form-field" :class="{ 'has-error': errors.registration_start }">
                  <label class="field-label">
                    Начало регистрации <span class="required">*</span>
                  </label>
                  <input
                    v-model="form.registration_start"
                    type="datetime-local"
                    class="field-input"
                    @blur="validateField('registration_start')"
                  />
                  <span v-if="errors.registration_start" class="error-text">{{ errors.registration_start }}</span>
                </div>

                <div class="form-field" :class="{ 'has-error': errors.registration_end }">
                  <label class="field-label">
                    Конец регистрации <span class="required">*</span>
                  </label>
                  <input
                    v-model="form.registration_end"
                    type="datetime-local"
                    class="field-input"
                    @blur="validateField('registration_end')"
                  />
                  <span v-if="errors.registration_end" class="error-text">{{ errors.registration_end }}</span>
                </div>
              </div>

              <div class="form-row">
                <div class="form-field" :class="{ 'has-error': errors.event_start }">
                  <label class="field-label">
                    Начало хакатона <span class="required">*</span>
                  </label>
                  <input
                    v-model="form.event_start"
                    type="datetime-local"
                    class="field-input"
                    @blur="validateField('event_start')"
                  />
                  <span v-if="errors.event_start" class="error-text">{{ errors.event_start }}</span>
                </div>

                <div class="form-field" :class="{ 'has-error': errors.event_end }">
                  <label class="field-label">
                    Конец хакатона <span class="required">*</span>
                  </label>
                  <input
                    v-model="form.event_end"
                    type="datetime-local"
                    class="field-input"
                    @blur="validateField('event_end')"
                  />
                  <span v-if="errors.event_end" class="error-text">{{ errors.event_end }}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Step 2: Contacts & Prize -->
          <div v-if="currentStep === 1" class="form-step" ref="stepContent">
            <div class="form-section">
              <h3 class="section-title">Контактная информация</h3>

              <div class="form-field" :class="{ 'has-error': errors.contact_email }">
                <label class="field-label">
                  Контактный email <span class="required">*</span>
                </label>
                <input
                  v-model="form.contact_email"
                  type="email"
                  class="field-input"
                  placeholder="hackathon@example.com"
                  @blur="validateField('contact_email')"
                />
                <span v-if="errors.contact_email" class="error-text">{{ errors.contact_email }}</span>
              </div>

              <div class="form-field">
                <label class="field-label">Сайт хакатона</label>
                <input
                  v-model="form.website_url"
                  type="url"
                  class="field-input"
                  placeholder="https://hackathon.example.com"
                />
              </div>

              <div class="form-field full-width">
                <label class="field-label">Социальные сети (JSON)</label>
                <textarea
                  v-model="socialLinksString"
                  class="field-textarea"
                  rows="3"
                  placeholder='{"vk": "https://vk.com/...", "telegram": "https://t.me/..."}'
                />
              </div>
            </div>

            <div class="form-section">
              <h3 class="section-title">Призовой фонд</h3>

              <div class="form-row">
                <div class="form-field">
                  <label class="field-label">Сумма призового фонда</label>
                  <input
                    v-model="form.prize_pool"
                    type="text"
                    class="field-input"
                    placeholder="100000"
                  />
                </div>

                <div class="form-field">
                  <label class="field-label">Валюта</label>
                  <CustomSelect
                    v-model="form.prize_currency"
                    :options="currencyOptions"
                    placeholder="Выберите валюту"
                  />
                </div>
              </div>

              <div class="form-field full-width">
                <label class="field-label">Описание призового фонда</label>
                <textarea
                  v-model="form.prize_description"
                  class="field-textarea"
                  rows="3"
                  placeholder="Опишите распределение призов, особые номинации..."
                />
              </div>
            </div>

            <div class="form-section">
              <h3 class="section-title">Лимит участников</h3>

              <div class="form-field">
                <label class="field-label">Максимальное количество участников</label>
                <input
                  v-model="form.max_participants"
                  type="number"
                  class="field-input"
                  placeholder="Без ограничений"
                  min="1"
                />
              </div>
            </div>
          </div>

          <!-- Step 3: Requirements & Skills -->
          <div v-if="currentStep === 2" class="form-step" ref="stepContent">
            <div class="form-section">
              <h3 class="section-title">Требования к участникам</h3>

              <div class="form-field">
                <label class="field-label">Описание требований</label>
                <textarea
                  v-model="form.requirements"
                  class="field-textarea"
                  rows="4"
                  placeholder="Какие навыки и знания требуются от участников..."
                />
              </div>

              <div class="form-row">
                <div class="form-field">
                  <label class="field-label">Минимальный размер команды</label>
                  <input
                    v-model="form.team_size_min"
                    type="number"
                    class="field-input"
                    placeholder="1"
                    min="1"
                  />
                </div>

                <div class="form-field">
                  <label class="field-label">Максимальный размер команды</label>
                  <input
                    v-model="form.team_size_max"
                    type="number"
                    class="field-input"
                    placeholder="5"
                    min="1"
                  />
                </div>

                <div class="form-field">
                  <label class="field-label">Возрастное ограничение</label>
                  <CustomSelect
                    v-model="form.age_restriction"
                    :options="ageRestrictionOptions"
                    placeholder="Без ограничений"
                  />
                </div>
              </div>
            </div>

            <div class="form-section">
              <h3 class="section-title">Компетенции хакатона</h3>
              <p class="section-description">Выберите навыки и технологии, которые будут актуальны для данного хакатона</p>

              <div v-if="isLoadingSkills" class="loading-skills">
                <div class="spinner"></div>
                <span>Загрузка навыков...</span>
              </div>

              <div v-else class="skills-grid">
                <label
                  v-for="skill in availableSkills"
                  :key="skill.id"
                  class="skill-checkbox"
                  :class="{ 'selected': selectedSkills.has(skill.id) }"
                >
                  <input
                    type="checkbox"
                    :value="skill.id"
                    :checked="selectedSkills.has(skill.id)"
                    @change="toggleSkill(skill.id)"
                    class="skill-input"
                  />
                  <span class="skill-name">{{ skill.name }}</span>
                  <span class="skill-category">{{ skill.category }}</span>
                </label>
              </div>

              <div v-if="selectedSkills.size > 0" class="selected-count">
                Выбрано: {{ selectedSkills.size }} навыков
              </div>
            </div>
          </div>

          <!-- Step 4: Tracks & Deadlines -->
          <div v-if="currentStep === 3" class="form-step" ref="stepContent">
            <div class="form-section">
              <h3 class="section-title">Треки хакатона</h3>

              <div class="tracks-list">
                <div
                  v-for="(track, index) in form.tracks"
                  :key="index"
                  class="track-card"
                >
                  <div class="track-header">
                    <h4 class="track-number">Трек {{ index + 1 }}</h4>
                    <button type="button" class="btn-remove" @click="removeTrack(index)">
                      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M18 6L6 18M6 6l12 12"/>
                      </svg>
                    </button>
                  </div>

                  <div class="form-field">
                    <label class="field-label">Название трека <span class="required">*</span></label>
                    <input
                      v-model="track.name"
                      type="text"
                      class="field-input"
                      placeholder="Например: Web-разработка"
                    />
                  </div>

                  <div class="form-field">
                    <label class="field-label">Описание трека</label>
                    <textarea
                      v-model="track.description"
                      class="field-textarea"
                      rows="2"
                      placeholder="Описание задач и целей трека..."
                    />
                  </div>

                  <div class="form-row">
                    <div class="form-field">
                      <label class="field-label">Описание призов</label>
                      <input
                        v-model="track.prize_description"
                        type="text"
                        class="field-input"
                        placeholder="Например: 1 место — 50 000₽"
                      />
                    </div>

                    <div class="form-field">
                      <label class="field-label">Макс. команд</label>
                      <input
                        v-model="track.max_teams"
                        type="number"
                        class="field-input"
                        placeholder="Без ограничений"
                        min="1"
                      />
                    </div>
                  </div>
                </div>

                <button
                  type="button"
                  class="btn btn-outline btn-add"
                  @click="addTrack"
                >
                  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <line x1="12" y1="5" x2="12" y2="19"/>
                    <line x1="5" y1="12" x2="19" y2="12"/>
                  </svg>
                  Добавить трек
                </button>
              </div>
            </div>

            <div class="form-section">
              <h3 class="section-title">Дедлайны и этапы</h3>

              <div class="deadlines-list">
                <div
                  v-for="(deadline, index) in form.deadlines"
                  :key="index"
                  class="deadline-card"
                >
                  <div class="deadline-header">
                    <h4 class="deadline-number">Этап {{ index + 1 }}</h4>
                    <button type="button" class="btn-remove" @click="removeDeadline(index)">
                      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M18 6L6 18M6 6l12 12"/>
                      </svg>
                    </button>
                  </div>

                  <div class="form-row">
                    <div class="form-field">
                      <label class="field-label">Название <span class="required">*</span></label>
                      <input
                        v-model="deadline.name"
                        type="text"
                        class="field-input"
                        placeholder="Например: Регистрация команд"
                      />
                    </div>

                    <div class="form-field">
                      <label class="field-label">Дедлайн <span class="required">*</span></label>
                      <input
                        v-model="deadline.deadline_at"
                        type="datetime-local"
                        class="field-input"
                      />
                    </div>
                  </div>

                  <div class="form-field">
                    <label class="field-label">Описание</label>
                    <input
                      v-model="deadline.description"
                      type="text"
                      class="field-input"
                      placeholder="Что должно быть выполнено к этому сроку..."
                    />
                  </div>

                  <div class="form-field checkbox-field">
                    <label class="checkbox-wrapper">
                      <input
                        v-model="deadline.is_milestone"
                        type="checkbox"
                        class="checkbox-input"
                      />
                      <span class="checkbox-custom"></span>
                      <span class="checkbox-label">Это ключевой этап (майлстоун)</span>
                    </label>
                  </div>
                </div>

                <button
                  type="button"
                  class="btn btn-outline btn-add"
                  @click="addDeadline"
                >
                  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <line x1="12" y1="5" x2="12" y2="19"/>
                    <line x1="5" y1="12" x2="19" y2="12"/>
                  </svg>
                  Добавить дедлайн
                </button>
              </div>
            </div>
          </div>

          <!-- Form Actions -->
          <div class="form-actions" ref="actions">
            <button
              v-if="currentStep > 0"
              type="button"
              class="btn btn-secondary"
              @click="prevStep"
            >
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M19 12H5M12 19l-7-7 7-7"/>
              </svg>
              Назад
            </button>

            <button
              v-if="currentStep < steps.length - 1"
              type="button"
              class="btn btn-primary"
              @click="nextStep"
              :disabled="!isStepValid"
            >
              Далее
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M5 12h14M12 5l7 7-7 7"/>
              </svg>
            </button>

            <button
              v-else
              type="submit"
              class="btn btn-primary"
              :disabled="isSubmitting || !isFormValid"
            >
              <span v-if="isSubmitting" class="spinner"></span>
              <template v-else>
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="20 6 9 17 4 12"/>
                </svg>
                {{ isEditMode ? 'Сохранить изменения' : 'Создать хакатон' }}
              </template>
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, watch } from 'vue'
import { useRouter } from 'vue-router'
import gsap from 'gsap'
import { organizerApi, userApi, hackathonApi } from '@/services/api'
import type { AvailableSkill, CreateHackathonRequest, CreateTrackRequest, CreateDeadlineRequest } from '@/services/api'
import { useAuth } from '@/composables/useAuth'
import { useModal } from '@/composables/useModal'
import CustomSelect from '@/components/CustomSelect.vue'

const props = defineProps<{
  editId?: string
}>()

const isEditMode = computed(() => !!props.editId)

const router = useRouter()
const { user } = useAuth()
const { alert } = useModal()

// Store organizer ID for hackathon creation
const organizerId = ref<string>('')

const wizardCard = ref<HTMLElement | null>(null)
const stepContent = ref<HTMLElement | null>(null)
const actions = ref<HTMLElement | null>(null)

const currentStep = ref(0)
const isSubmitting = ref(false)
const isLoadingSkills = ref(false)
const availableSkills = ref<AvailableSkill[]>([])
const selectedSkills = ref<Set<string>>(new Set())
const socialLinksString = ref('')

const steps = [
  { label: 'Основное', key: 'basic' },
  { label: 'Призовой фонд', key: 'prize' },
  { label: 'Требования', key: 'requirements' },
  { label: 'Треки', key: 'tracks' },
]

const form = reactive<CreateHackathonRequest>({
  title: '',
  description: '',
  location_type: '',
  city: '',
  venue: '',
  registration_start: '',
  registration_end: '',
  event_start: '',
  event_end: '',
  contact_email: '',
  website_url: '',
  social_links: undefined,
  prize_pool: '',
  prize_currency: '',
  prize_description: '',
  max_participants: undefined,
  requirements: '',
  team_size_min: 1,
  team_size_max: 5,
  age_restriction: '',
  skills: [],
  tracks: [],
  deadlines: [],
})

// Select options
const locationTypeOptions = [
  { value: 'online', label: 'Онлайн' },
  { value: 'offline', label: 'Офлайн' },
  { value: 'hybrid', label: 'Гибрид' },
]

const currencyOptions = [
  { value: 'RUB', label: 'RUB — Российский рубль' },
  { value: 'USD', label: 'USD — Доллар США' },
  { value: 'EUR', label: 'EUR — Евро' },
]

const ageRestrictionOptions = [
  { value: '14+', label: '14+' },
  { value: '16+', label: '16+' },
  { value: '18+', label: '18+' },
  { value: '', label: 'Без ограничений' },
]

const errors = reactive<Record<string, string>>({})

const validateField = (field: string) => {
  errors[field] = ''

  switch (field) {
    case 'title':
      if (!form.title.trim()) {
        errors.title = 'Название обязательно'
      } else if (form.title.length < 3) {
        errors.title = 'Минимум 3 символа'
      }
      break
    case 'description':
      if (!form.description.trim()) {
        errors.description = 'Описание обязательно'
      } else if (form.description.length < 50) {
        errors.description = 'Минимум 50 символов'
      }
      break
    case 'location_type':
      if (!form.location_type) {
        errors.location_type = 'Выберите тип проведения'
      }
      break
    case 'registration_start':
      if (!form.registration_start) {
        errors.registration_start = 'Укажите дату начала регистрации'
      }
      break
    case 'registration_end':
      if (!form.registration_end) {
        errors.registration_end = 'Укажите дату конца регистрации'
      } else if (form.registration_start && new Date(form.registration_end) <= new Date(form.registration_start)) {
        errors.registration_end = 'Дата окончания должна быть позже начала'
      }
      break
    case 'event_start':
      if (!form.event_start) {
        errors.event_start = 'Укажите дату начала хакатона'
      } else if (form.registration_end && new Date(form.event_start) < new Date(form.registration_end)) {
        errors.event_start = 'Хакатон должен начаться после окончания регистрации'
      }
      break
    case 'event_end':
      if (!form.event_end) {
        errors.event_end = 'Укажите дату окончания хакатона'
      } else if (form.event_start && new Date(form.event_end) <= new Date(form.event_start)) {
        errors.event_end = 'Дата окончания должна быть позже начала'
      }
      break
    case 'contact_email':
      if (!form.contact_email.trim()) {
        errors.contact_email = 'Контактный email обязателен'
      } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(form.contact_email)) {
        errors.contact_email = 'Некорректный email'
      }
      break
  }
}

const validateCurrentStep = () => {
  errors[''] = '' // Clear general error

  if (currentStep.value === 0) {
    validateField('title')
    validateField('description')
    validateField('location_type')
    validateField('registration_start')
    validateField('registration_end')
    validateField('event_start')
    validateField('event_end')
  } else if (currentStep.value === 1) {
    validateField('contact_email')
  }

  return !Object.values(errors).some(e => e)
}

const isStepValid = computed(() => {
  if (currentStep.value === 0) {
    return form.title && form.description && form.location_type &&
      form.registration_start && form.registration_end &&
      form.event_start && form.event_end &&
      !errors.title && !errors.description && !errors.location_type &&
      !errors.registration_start && !errors.registration_end &&
      !errors.event_start && !errors.event_end
  }
  if (currentStep.value === 1) {
    return form.contact_email && !errors.contact_email
  }
  if (currentStep.value === 2) {
    return true // Skills are optional
  }
  if (currentStep.value === 3) {
    // At least one track and all tracks have names
    return form.tracks.length > 0 && form.tracks.every(t => t.name.trim())
  }
  return true
})

const isFormValid = computed(() => {
  return form.title && form.description && form.location_type &&
    form.registration_start && form.registration_end &&
    form.event_start && form.event_end && form.contact_email &&
    form.tracks.length > 0 && form.tracks.every(t => t.name.trim())
})

const nextStep = () => {
  if (!validateCurrentStep()) return
  if (currentStep.value < steps.length - 1) {
    currentStep.value++
  }
}

const prevStep = () => {
  if (currentStep.value > 0) {
    currentStep.value--
  }
}

const toggleSkill = (skillId: string) => {
  if (selectedSkills.value.has(skillId)) {
    selectedSkills.value.delete(skillId)
  } else {
    selectedSkills.value.add(skillId)
  }
  form.skills = Array.from(selectedSkills.value)
}

const addTrack = () => {
  form.tracks.push({
    name: '',
    description: '',
    prize_description: '',
    max_teams: undefined,
  })
}

const removeTrack = (index: number) => {
  form.tracks.splice(index, 1)
}

const addDeadline = () => {
  form.deadlines.push({
    name: '',
    description: '',
    deadline_at: '',
    is_milestone: false,
  })
}

const removeDeadline = (index: number) => {
  form.deadlines.splice(index, 1)
}

const handleSubmit = async () => {
  if (!isFormValid.value) return

  // Parse social links
  let socialLinks = undefined
  if (socialLinksString.value.trim()) {
    try {
      socialLinks = JSON.parse(socialLinksString.value)
    } catch {
      // ignore parse error
    }
  }

  isSubmitting.value = true

  // Convert numeric strings to numbers and dates to ISO format
  const toISOString = (dateStr: string) => dateStr ? new Date(dateStr).toISOString() : ''

  // Filter out empty optional fields
  const cleanDeadlines = form.deadlines
    .filter(d => d.name && d.deadline_at)
    .map(d => ({
      name: d.name,
      description: d.description || undefined,
      deadline_at: toISOString(d.deadline_at),
      is_milestone: d.is_milestone,
    }))

  const data: CreateHackathonRequest = {
    ...form,
    new_organizer_id: organizerId.value,
    registration_start: toISOString(form.registration_start),
    // ... rest of the fields
    registration_end: toISOString(form.registration_end),
    event_start: toISOString(form.event_start),
    event_end: toISOString(form.event_end),
    max_participants: form.max_participants ? Number(form.max_participants) : undefined,
    team_size_min: form.team_size_min ? Number(form.team_size_min) : undefined,
    team_size_max: form.team_size_max ? Number(form.team_size_max) : undefined,
    tracks: form.tracks
      .filter(t => t.name.trim())
      .map(t => ({
        name: t.name,
        description: t.description || undefined,
        prize_description: t.prize_description || undefined,
        max_teams: t.max_teams ? Number(t.max_teams) : undefined,
      })),
    deadlines: cleanDeadlines,
    social_links: socialLinks,
    banner_url: form.banner_url || undefined,
    city: form.location_type !== 'online' ? form.city || undefined : undefined,
    venue: form.location_type !== 'online' ? form.venue || undefined : undefined,
    website_url: form.website_url || undefined,
    requirements: form.requirements || undefined,
    age_restriction: form.age_restriction || undefined,
    prize_pool: form.prize_pool || undefined,
    prize_currency: form.prize_currency || undefined,
    prize_description: form.prize_description || undefined,
  }

  if (isEditMode.value && props.editId) {
    const { new_organizer_id: _omit, tracks: _tracksOmit, deadlines: _deadlinesOmit, ...updateData } = data as any
    const response = await organizerApi.updateHackathon(props.editId, updateData)

    if (response.error) {
      await alert({
        title: 'Ошибка',
        message: response.error,
        type: 'error',
      })
    } else {
      router.push(`/hackathons/${props.editId}`)
    }
  } else {
    const response = await organizerApi.createHackathon(data)

    if (response.data) {
      const hackathonId = (response.data as any).id
      if (hackathonId) {
        router.push(`/hackathons/${hackathonId}`)
      } else {
        router.push('/organizers/dashboard')
      }
    } else {
      await alert({
        title: 'Ошибка',
        message: response.error || 'Неизвестная ошибка',
        type: 'error',
      })
    }
  }
  isSubmitting.value = false
}

// Animation for step transitions
watch(currentStep, () => {
  if (stepContent.value) {
    gsap.fromTo(stepContent.value,
      { opacity: 0, x: 20 },
      { opacity: 1, x: 0, duration: 0.3, ease: 'power2.out' }
    )
  }
})

onMounted(async () => {
  gsap.fromTo(wizardCard.value,
    { y: 40, opacity: 0 },
    { y: 0, opacity: 1, duration: 0.6, ease: 'power2.out' }
  )

  gsap.fromTo(actions.value,
    { y: 20, opacity: 0 },
    { y: 0, opacity: 1, duration: 0.5, delay: 0.3, ease: 'power2.out' }
  )

  // Load organizer profile and available skills
  isLoadingSkills.value = true

  const [skillsResponse, organizerResponse] = await Promise.all([
    userApi.getAvailableSkills(),
    organizerApi.getMyOrganizer()
  ])

  if (skillsResponse.data) {
    availableSkills.value = skillsResponse.data
  }

  if (organizerResponse.data) {
    organizerId.value = organizerResponse.data.id
  }

  isLoadingSkills.value = false

  if (isEditMode.value && props.editId) {
    const hackathonResp = await hackathonApi.getById(props.editId)
    if (hackathonResp.data) {
      const h = hackathonResp.data as any
      const toLocalInput = (iso?: string) => {
        if (!iso) return ''
        const d = new Date(iso)
        const tzOffset = d.getTimezoneOffset() * 60000
        return new Date(d.getTime() - tzOffset).toISOString().slice(0, 16)
      }
      Object.assign(form, {
        title: h.title || '',
        description: h.description || '',
        location_type: h.location_type || '',
        city: h.city || '',
        venue: h.venue || '',
        registration_start: toLocalInput(h.registration_start),
        registration_end: toLocalInput(h.registration_end),
        event_start: toLocalInput(h.event_start),
        event_end: toLocalInput(h.event_end),
        contact_email: h.contact_email || '',
        website_url: h.website_url || '',
        social_links: h.social_links,
        prize_pool: h.prize_pool ?? '',
        prize_currency: h.prize_currency || '',
        prize_description: h.prize_description || '',
        max_participants: h.max_participants ?? undefined,
        requirements: h.requirements || '',
        team_size_min: h.team_size_min ?? 1,
        team_size_max: h.team_size_max ?? 5,
        age_restriction: h.age_restriction || '',
      })
      form.banner_url = h.banner_url || ''
      socialLinksString.value = h.social_links ? JSON.stringify(h.social_links) : ''
      selectedSkills.value = new Set((h.skills || []).map((s: any) => s.id))
      form.skills = Array.from(selectedSkills.value)
      form.tracks = (h.tracks || []).map((t: any) => ({
        name: t.name || '',
        description: t.description || '',
        prize_description: t.prize_description || '',
        max_teams: t.max_teams ?? undefined,
      }))
      form.deadlines = (h.deadlines || []).map((d: any) => ({
        name: d.name || '',
        description: d.description || '',
        deadline_at: toLocalInput(d.deadline_at),
        is_milestone: !!d.is_milestone,
      }))
    }
  }

  // Add initial track only in create mode
  if (!isEditMode.value && form.tracks.length === 0) {
    addTrack()
  }
})
</script>

<style scoped lang="scss">
@use '../styles/variables' as *;

.wizard-page {
  min-height: 100vh;
  background: $color-bg;
  padding: 6rem 1rem 2rem;
}

.container {
  max-width: 900px;
  margin: 0 auto;
}

.wizard-card {
  background: $color-surface;
  border: 1px solid $color-border;
  border-radius: 8px;
  padding: 2.5rem;
}

.wizard-header {
  text-align: center;
  margin-bottom: 2.5rem;

  .page-title {
    font-size: clamp(1.75rem, 4vw, 2.25rem);
    font-weight: 600;
    margin-bottom: 0.75rem;
    color: $color-text;
    font-family: $font-display;
    letter-spacing: -0.02em;
  }

  .page-subtitle {
    color: $color-text-dim;
    font-size: 1rem;
    margin-bottom: 2rem;
    font-family: $font-body;
  }
}

// Stepper
.stepper {
  display: flex;
  justify-content: center;
  align-items: flex-start;
  gap: 0;
  max-width: 600px;
  margin: 0 auto;
  overflow-x: auto;
  padding-bottom: 0.5rem;

  @media (max-width: 640px) {
    gap: 0.5rem;
  }
}

.step {
  display: flex;
  flex-direction: column;
  align-items: center;
  position: relative;
  flex: 1;
  min-width: 80px;
  max-width: 140px;

  @media (max-width: 640px) {
    min-width: 60px;
  }

  &:not(:last-child) {
    &::after {
      content: '';
      position: absolute;
      top: 20px;
      left: calc(50% + 24px);
      width: calc(100% - 48px);
      height: 2px;
      background: $color-border;
      transition: background 0.3s $transition-smooth;
    }
  }

  .step-number {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 0.875rem;
    background: $color-bg;
    border: 2px solid $color-border;
    color: $color-text-dim;
    transition: all 0.3s $transition-smooth;
    font-family: $font-display;
    flex-shrink: 0;

    svg {
      stroke: currentColor;
      width: 18px;
      height: 18px;
    }

    @media (max-width: 640px) {
      width: 32px;
      height: 32px;
      font-size: 0.75rem;
    }
  }

  .step-label {
    margin-top: 0.5rem;
    font-size: 0.75rem;
    color: $color-text-dim;
    font-weight: 500;
    text-align: center;
    font-family: $font-body;
    line-height: 1.3;
    padding: 0 0.25rem;

    @media (max-width: 640px) {
      font-size: 0.65rem;
      max-width: 60px;
      white-space: normal;
    }
  }

  &.active {
    .step-number {
      background: $color-accent;
      border-color: $color-accent;
      color: $color-bg;
    }

    .step-label {
      color: $color-accent;
    }
  }

  &.completed {
    .step-number {
      background: $color-accent;
      border-color: $color-accent;
      color: $color-bg;
    }

    .step-label {
      color: $color-text;
    }

    &:not(:last-child)::after {
      background: $color-accent;
    }
  }
}

.step-connector {
  display: none;
}

// Form Sections
.wizard-form {
  margin-top: 2rem;
}

.form-step {
  animation: fadeIn 0.3s $transition-smooth;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateX(20px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

.form-section {
  margin-bottom: 2rem;

  .section-title {
    font-size: 1.1rem;
    font-weight: 600;
    color: $color-text;
    margin-bottom: 1.25rem;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid $color-border;
    font-family: $font-display;
  }

  .section-description {
    color: $color-text-dim;
    font-size: 0.9rem;
    margin-bottom: 1rem;
    margin-top: -0.75rem;
    font-family: $font-body;
  }
}

.form-row {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;

  @media (max-width: 640px) {
    grid-template-columns: 1fr;
  }
}

.form-field {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin-bottom: 1rem;

  &.full-width {
    grid-column: 1 / -1;
  }

  &.has-error {
    .field-input, .field-select, .field-textarea {
      border-color: $color-secondary;
    }
  }

  &.checkbox-field {
    flex-direction: row;
    align-items: center;
    gap: 0.75rem;
  }
}

.field-label {
  font-size: 0.875rem;
  font-weight: 500;
  color: $color-text-dim;
  font-family: $font-body;

  .required {
    color: $color-secondary;
  }
}

.field-input,
.field-select,
.field-textarea {
  padding: 0.875rem 1rem;
  border: 1px solid $color-border;
  border-radius: 8px;
  background: $color-bg;
  color: $color-text;
  font-size: 0.95rem;
  transition: all 0.2s ease;
  font-family: $font-body;

  &:focus {
    outline: none;
    border-color: $color-accent;
    box-shadow: 0 0 0 3px rgba($color-accent, 0.1);
  }

  &::placeholder {
    color: $color-text-muted;
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
}

.field-select {
  cursor: pointer;
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke='%236b7280'%3E%3Cpath stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='M19 9l-7 7-7-7'%3E%3C/path%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 0.75rem center;
  background-size: 1.25rem;
  padding-right: 2.5rem;
}

.field-textarea {
  resize: vertical;
  min-height: 100px;
}

.error-text {
  font-size: 0.8rem;
  color: $color-secondary;
  font-family: $font-body;
}

// Skills
.loading-skills {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 2rem;
  color: $color-text-dim;
  font-family: $font-body;

  .spinner {
    width: 24px;
    height: 24px;
    border: 2px solid $color-border;
    border-top-color: $color-accent;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.skills-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
  gap: 0.75rem;
}

.skill-checkbox {
  display: flex;
  flex-direction: column;
  padding: 0.75rem;
  background: $color-bg;
  border: 1px solid $color-border;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;

  &:hover {
    border-color: $color-accent;
  }

  &.selected {
    border-color: $color-accent;
    background: rgba($color-accent, 0.05);
  }

  .skill-input {
    display: none;
  }

  .skill-name {
    font-size: 0.9rem;
    font-weight: 500;
    color: $color-text;
    font-family: $font-body;
  }

  .skill-category {
    font-size: 0.75rem;
    color: $color-text-dim;
    margin-top: 0.25rem;
    font-family: $font-body;
  }
}

.selected-count {
  margin-top: 1rem;
  padding: 0.75rem;
  background: rgba($color-accent, 0.05);
  border-radius: 4px;
  font-size: 0.9rem;
  color: $color-accent;
  font-weight: 500;
  font-family: $font-body;
}

// Tracks & Deadlines
.tracks-list,
.deadlines-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.track-card,
.deadline-card {
  padding: 1.5rem;
  background: rgba($color-accent, 0.02);
  border: 1px solid $color-border;
  border-radius: 8px;
}

.track-header,
.deadline-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;

  .track-number,
  .deadline-number {
    font-size: 1rem;
    font-weight: 600;
    color: $color-text;
    font-family: $font-display;
  }
}

.btn-remove {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0.5rem;
  background: transparent;
  border: none;
  color: $color-text-dim;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s ease;

  &:hover {
    background: rgba($color-secondary, 0.1);
    color: $color-secondary;
  }
}

.btn-add {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 1rem;

  svg {
    stroke: currentColor;
  }
}

// Checkbox
.checkbox-wrapper {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  cursor: pointer;
}

.checkbox-input {
  display: none;
}

.checkbox-custom {
  width: 20px;
  height: 20px;
  border: 2px solid $color-border;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  flex-shrink: 0;
  position: relative;

  &::after {
    content: '';
    width: 10px;
    height: 10px;
    background: $color-bg;
    border-radius: 2px;
    opacity: 0;
    transform: scale(0);
    transition: all 0.2s ease;
  }

  .checkbox-input:checked + & {
    background: $color-accent;
    border-color: $color-accent;

    &::after {
      opacity: 1;
      transform: scale(1);
    }
  }
}

.checkbox-label {
  font-size: 0.9rem;
  color: $color-text;
  font-family: $font-body;
}

// Actions
.form-actions {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  margin-top: 2rem;
  padding-top: 1.5rem;
  border-top: 1px solid $color-border;

  @media (max-width: 480px) {
    flex-direction: column;
  }
}

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 0.875rem 1.5rem;
  border-radius: 8px;
  font-weight: 600;
  font-size: 1rem;
  transition: all 0.3s $transition-smooth;
  cursor: pointer;
  border: 1px solid $color-border;
  text-transform: lowercase;
  font-family: $font-body;
  position: relative;
  overflow: hidden;
  background: transparent;

  svg {
    stroke: currentColor;
  }

  &.btn-primary {
    color: $color-accent;
    border-color: $color-accent;
    min-width: 140px;

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

    &:hover:not(:disabled) {
      color: $color-bg;

      &::before {
        transform: scaleX(1);
      }
    }

    &:disabled {
      opacity: 0.5;
      cursor: not-allowed;
    }
  }

  &.btn-secondary {
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

  &.btn-outline {
    color: $color-text;
    border: 1px dashed $color-border;

    &::before {
      content: '';
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      background: rgba($color-accent, 0.02);
      transform: scaleX(0);
      transform-origin: left;
      transition: transform 0.3s $transition-smooth;
      z-index: -1;
    }

    &:hover {
      border-color: $color-accent;
      color: $color-accent;
      border-style: solid;

      &::before {
        transform: scaleX(1);
      }
    }
  }
}

.spinner {
  width: 20px;
  height: 20px;
  border: 2px solid rgba($color-accent, 0.3);
  border-top-color: $color-accent;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}
</style>
