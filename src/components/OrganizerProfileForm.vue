<template>
  <div class="profile-page">
    <div class="container">
      <div class="page-card" ref="pageCard">
        <div class="page-header">
          <h1 class="page-title">Профиль организатора</h1>
          <p class="page-subtitle">
            Заполните информацию о себе или вашей организации
          </p>
        </div>

        <form @submit.prevent="handleSubmit" class="profile-form">
          <div class="form-section">
            <h3 class="section-title">Основная информация</h3>
            <div class="form-grid">
              <div class="form-field" :class="{ 'has-error': errors.name }">
                <label class="field-label">
                  Название <span class="required">*</span>
                </label>
                <input
                  v-model="form.name"
                  type="text"
                  class="field-input"
                  placeholder="Название компании или ваше имя"
                  @blur="validateField('name')"
                />
                <span v-if="errors.name" class="error-text">{{ errors.name }}</span>
              </div>

              <div class="form-field" :class="{ 'has-error': errors.type_ }">
                <label class="field-label">
                  Тип организации <span class="required">*</span>
                </label>
                <CustomSelect
                  v-model="form.type_"
                  :options="organizerTypeOptions"
                  placeholder="Выберите тип"
                  @blur="validateField('type_')"
                />
                <span v-if="errors.type_" class="error-text">{{ errors.type_ }}</span>
              </div>

              <div class="form-field" :class="{ 'has-error': errors.email }">
                <label class="field-label">
                  Email <span class="required">*</span>
                </label>
                <input
                  v-model="form.email"
                  type="email"
                  class="field-input"
                  placeholder="contact@example.com"
                  @blur="validateField('email')"
                />
                <span v-if="errors.email" class="error-text">{{ errors.email }}</span>
              </div>

              <div class="form-field full-width">
                <label class="field-label">Описание</label>
                <textarea
                  v-model="form.description"
                  class="field-textarea"
                  rows="4"
                  placeholder="Расскажите о вашей организации или о себе..."
                />
              </div>
            </div>
          </div>

          <div class="form-section">
            <h3 class="section-title">Контакты и соцсети</h3>
            <div class="form-grid">
              <div class="form-field">
                <label class="field-label">Сайт</label>
                <input
                  v-model="form.website_url"
                  type="url"
                  class="field-input"
                  placeholder="https://example.com"
                />
              </div>

              <div class="form-field">
                <label class="field-label">Логотип (URL)</label>
                <input
                  v-model="form.logo_url"
                  type="url"
                  class="field-input"
                  placeholder="https://example.com/logo.png"
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
          </div>

          <div class="form-actions" ref="actions">
            <button type="button" class="btn btn-secondary" @click="goBack">
              Назад
            </button>
            <button
              type="submit"
              class="btn btn-primary"
              :disabled="isSubmitting || !isValid"
            >
              <span v-if="isSubmitting" class="spinner"></span>
              <span v-else>
                {{ hasOrganizer ? 'Сохранить' : 'Создать профиль' }}
              </span>
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import gsap from 'gsap'
import { organizerApi } from '@/services/api'
import { useAuth } from '@/composables/useAuth'
import { useModal } from '@/composables/useModal'
import CustomSelect from '@/components/CustomSelect.vue'

const organizerTypeOptions = [
  { value: 'company', label: 'Компания' },
  { value: 'university', label: 'Университет' },
  { value: 'community', label: 'Сообщество' },
  { value: 'individual', label: 'Частное лицо' },
]

const router = useRouter()
const { user, isAuthenticated } = useAuth()
const { alert } = useModal()

const pageCard = ref<HTMLElement | null>(null)
const actions = ref<HTMLElement | null>(null)

const isSubmitting = ref(false)
const hasOrganizer = ref(false)
const socialLinksString = ref('')

const form = reactive({
  name: '',
  type_: '',
  email: '',
  description: '',
  website_url: '',
  logo_url: '',
  social_links: undefined as unknown | undefined,
})

const errors = reactive<Record<string, string>>({})

const isValid = computed(() => {
  return form.name && form.type_ && form.email && !errors.name && !errors.type_ && !errors.email
})

const validateField = (field: string) => {
  errors[field] = ''

  if (field === 'name') {
    if (!form.name.trim()) {
      errors.name = 'Название обязательно'
    } else if (form.name.length < 2) {
      errors.name = 'Минимум 2 символа'
    }
  }

  if (field === 'type_') {
    if (!form.type_) {
      errors.type_ = 'Выберите тип организации'
    }
  }

  if (field === 'email') {
    if (!form.email.trim()) {
      errors.email = 'Email обязателен'
    } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(form.email)) {
      errors.email = 'Некорректный email'
    }
  }
}

const validateForm = () => {
  validateField('name')
  validateField('type_')
  validateField('email')
  return Object.keys(errors).filter(k => errors[k]).length === 0
}

const handleSubmit = async () => {
  if (!validateForm()) return

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

  try {
    const data = {
      name: form.name,
      type_: form.type_,
      email: form.email,
      description: form.description || undefined,
      website_url: form.website_url || undefined,
      logo_url: form.logo_url || undefined,
      social_links: socialLinks,
    }

    console.log('Sending organizer data:', data)
    let response
    if (hasOrganizer.value) {
      response = await organizerApi.updateOrganizer({
        description: data.description,
        website_url: data.website_url,
        logo_url: data.logo_url,
        email: data.email,
        social_links: data.social_links,
      })
    } else {
      response = await organizerApi.createOrganizer(data)
    }
    console.log('Response:', response)

    if (response.data) {
      hasOrganizer.value = true
      router.push('/organizers/dashboard')
    } else {
      await alert({
        title: 'Ошибка',
        message: response.error || 'Произошла ошибка',
        type: 'error'
      })
    }
  } catch (error: any) {
    await alert({
      title: 'Ошибка',
      message: error?.message || 'Произошла ошибка при сохранении',
      type: 'error'
    })
  } finally {
    isSubmitting.value = false
  }
}

const goBack = () => {
  router.push('/organizers')
}

onMounted(async () => {
  gsap.fromTo(pageCard.value,
    { y: 40, opacity: 0 },
    { y: 0, opacity: 1, duration: 0.6, ease: 'power2.out' }
  )

  gsap.fromTo(actions.value,
    { y: 20, opacity: 0 },
    { y: 0, opacity: 1, duration: 0.5, delay: 0.3, ease: 'power2.out' }
  )

  // Check if user already has organizer profile
  if (isAuthenticated.value) {
    const response = await organizerApi.getMyOrganizer()
    if (response.data) {
      hasOrganizer.value = true
      form.name = response.data.name
      form.type_ = response.data.type_
      form.email = response.data.email
      form.description = response.data.description || ''
      form.website_url = response.data.website_url || ''
      form.logo_url = response.data.logo_url || ''
      if (response.data.social_links) {
        socialLinksString.value = JSON.stringify(response.data.social_links, null, 2)
      }
    } else if (user.value) {
      // Pre-fill from user
      form.name = user.value.name
      form.email = user.value.email
    }
  }
})
</script>

<style scoped lang="scss">
@use '../styles/variables' as *;

.profile-page {
  min-height: 100vh;
  background: $color-bg;
  padding: 2rem 1rem;
  display: flex;
  align-items: center;
}

.container {
  max-width: 800px;
  width: 100%;
  margin: 0 auto;
}

.page-card {
  background: $color-surface;
  border: 1px solid $color-border;
  border-radius: 8px;
  padding: 2.5rem;
}

.page-header {
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
    font-family: $font-body;
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
}

.form-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1.25rem;

  @media (max-width: 640px) {
    grid-template-columns: 1fr;
  }
}

.form-field {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;

  &.full-width {
    grid-column: 1 / -1;
  }

  &.has-error {
    .field-input, .field-select, .field-textarea {
      border-color: $color-secondary;
    }
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

  &.btn-primary {
    color: $color-accent;
    border-color: $color-accent;
    min-width: 180px;

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
}

.spinner {
  width: 20px;
  height: 20px;
  border: 2px solid rgba($color-accent, 0.3);
  border-top-color: $color-accent;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>
