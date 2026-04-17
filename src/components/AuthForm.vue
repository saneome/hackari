<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue'
import { gsap } from 'gsap'
import BaseButton from './ui/BaseButton.vue'
import BaseInput from './ui/BaseInput.vue'
import { useAuth } from '@/composables/useAuth'

const { login, register } = useAuth()

type AuthMode = 'login' | 'register' | 'reset'
type FieldName = 'name' | 'email' | 'password' | 'confirmPassword'

const mode = ref<AuthMode>('login')
const isLoading = ref(false)
const isSuccess = ref(false)
const currentFieldIndex = ref(0)
const showSubmitButton = ref(false)

const formData = ref({
  email: '',
  password: '',
  confirmPassword: '',
  name: '',
})

const errors = ref<Record<string, string>>({})
const fieldRefs = ref<Record<string, any>>({})

interface ModeConfig {
  title: string
  subtitle: string
  submitText: string
  alternateText: string
  alternateAction: string
  fields: FieldName[]
  fieldLabels: Record<FieldName, string>
  fieldPlaceholders: Record<FieldName, string>
}

const modeConfig = computed<ModeConfig>(() => {
  const configs: Record<AuthMode, ModeConfig> = {
    login: {
      title: 'вход',
      subtitle: 'войдите в систему',
      submitText: 'войти',
      alternateText: 'нет аккаунта?',
      alternateAction: 'создать',
      fields: ['email', 'password'],
      fieldLabels: {
        email: 'email',
        password: 'пароль',
        name: 'имя',
        confirmPassword: 'подтвердите пароль',
      },
      fieldPlaceholders: {
        email: 'name@example.com',
        password: '••••••••',
        name: 'ваше имя',
        confirmPassword: '••••••••',
      },
    },
    register: {
      title: 'регистрация',
      subtitle: 'создайте аккаунт',
      submitText: 'создать',
      alternateText: 'уже есть аккаунт?',
      alternateAction: 'войти',
      fields: ['name', 'email', 'password', 'confirmPassword'],
      fieldLabels: {
        email: 'email',
        password: 'пароль',
        name: 'имя',
        confirmPassword: 'подтвердите пароль',
      },
      fieldPlaceholders: {
        email: 'name@example.com',
        password: '••••••••',
        name: 'как вас зовут?',
        confirmPassword: 'повторите пароль',
      },
    },
    reset: {
      title: 'сброс пароля',
      subtitle: 'восстановление доступа',
      submitText: 'отправить',
      alternateText: 'вспомнили пароль?',
      alternateAction: 'войти',
      fields: ['email'],
      fieldLabels: {
        email: 'email',
        password: 'пароль',
        name: 'имя',
        confirmPassword: 'подтвердите пароль',
      },
      fieldPlaceholders: {
        email: 'name@example.com',
        password: '••••••••',
        name: 'ваше имя',
        confirmPassword: '••••••••',
      },
    },
  }
  return configs[mode.value]
})

const currentField = computed(() => modeConfig.value.fields[currentFieldIndex.value])

const progressPercent = computed(() => {
  const total = modeConfig.value.fields.length
  const current = currentFieldIndex.value
  if (showSubmitButton.value) return 100
  return Math.round((current / total) * 100)
})

const currentValue = computed({
  get: () => {
    const field = currentField.value
    return formData.value[field] || ''
  },
  set: (val: string) => {
    const field = currentField.value
    formData.value[field] = val
  },
})

const currentError = computed(() => {
  const field = currentField.value
  return errors.value[field] || ''
})

const isCurrentFieldValid = computed(() => {
  const field = currentField.value
  const value = formData.value[field]

  if (!value) return false

  switch (field) {
    case 'email':
      return value.includes('@') && value.includes('.')
    case 'password':
      return value.length >= 8
    case 'confirmPassword':
      return value === formData.value.password
    case 'name':
      return value.length >= 2
    default:
      return true
  }
})

const canContinue = computed(() => isCurrentFieldValid.value)

const isLastField = computed(() => {
  return currentFieldIndex.value >= modeConfig.value.fields.length - 1
})

const allFieldsFilled = computed(() => {
  return modeConfig.value.fields.every(field => {
    const value = formData.value[field]
    if (!value) return false
    if (field === 'email') return value.includes('@') && value.includes('.')
    if (field === 'password') return value.length >= 8
    if (field === 'confirmPassword') return value === formData.value.password
    if (field === 'name') return value.length >= 2
    return true
  })
})

const validateCurrentField = () => {
  const field = currentField.value
  const value = formData.value[field]
  errors.value = {}

  if (!value) {
    errors.value[field] = 'обязательное поле'
    return false
  }

  switch (field) {
    case 'email':
      if (!value.includes('@') || !value.includes('.')) {
        errors.value[field] = 'некорректный email'
        return false
      }
      break
    case 'password':
      if (value.length < 8) {
        errors.value[field] = 'минимум 8 символов'
        return false
      }
      break
    case 'confirmPassword':
      if (value !== formData.value.password) {
        errors.value[field] = 'пароли не совпадают'
        return false
      }
      break
    case 'name':
      if (value.length < 2) {
        errors.value[field] = 'минимум 2 символа'
        return false
      }
      break
  }

  return true
}

const goToNextField = () => {
  if (!validateCurrentField()) return

  if (isLastField.value) {
    showSubmitButton.value = true
    nextTick(() => {
      animateSubmitButton()
    })
  } else {
    currentFieldIndex.value++
    nextTick(() => {
      animateFieldTransition()
    })
  }
}

const goToPrevField = () => {
  if (showSubmitButton.value) {
    showSubmitButton.value = false
  } else if (currentFieldIndex.value > 0) {
    currentFieldIndex.value--
  }
  nextTick(() => {
    animateFieldTransition()
  })
}

const animateFieldTransition = () => {
  const fieldEl = document.querySelector('.current-field-wrapper')
  if (fieldEl) {
    gsap.fromTo(fieldEl,
      { opacity: 0, x: 30 },
      { opacity: 1, x: 0, duration: 0.4, ease: 'power2.out' }
    )
  }
}

const animateSubmitButton = () => {
  const btn = document.querySelector('.submit-section')
  if (btn) {
    gsap.fromTo(btn,
      { opacity: 0, y: 20, scale: 0.95 },
      { opacity: 1, y: 0, scale: 1, duration: 0.4, ease: 'back.out(1.7)' }
    )
  }
}

const handleKeydown = (e: KeyboardEvent) => {
  if (e.key === 'Enter' && canContinue.value) {
    e.preventDefault()
    goToNextField()
  }
}

const handleSubmit = async () => {
  if (!allFieldsFilled.value) return

  isLoading.value = true
  errors.value = {}

  try {
    let result

    if (mode.value === 'login') {
      result = await login(
        formData.value.email,
        formData.value.password
      )
    } else if (mode.value === 'register') {
      result = await register(
        formData.value.email,
        formData.value.password,
        formData.value.name
      )
    } else {
      // reset password - mock for now
      await new Promise(resolve => setTimeout(resolve, 1000))
      isLoading.value = false
      isSuccess.value = true
      setTimeout(() => {
        isSuccess.value = false
        resetForm()
        mode.value = 'login'
      }, 2000)
      return
    }

    isLoading.value = false

    if (!result.success) {
      errors.value.submit = result.error || 'Произошла ошибка'
      return
    }

    isSuccess.value = true

    setTimeout(() => {
      isSuccess.value = false
      resetForm()
      window.location.href = '/'
    }, 1500)
  } catch (error) {
    isLoading.value = false
    errors.value.submit = 'Произошла ошибка. Попробуйте снова.'
  }
}

const resetForm = () => {
  formData.value = { email: '', password: '', confirmPassword: '', name: '' }
  currentFieldIndex.value = 0
  showSubmitButton.value = false
  errors.value = {}
}

const switchMode = (newMode: AuthMode) => {
  mode.value = newMode
  resetForm()
  isSuccess.value = false
}

watch(() => mode.value, () => {
  resetForm()
})
</script>

<template>
  <div class="auth-form">
    <!-- Progress Bar -->
    <div v-show="!isSuccess" class="progress-container">
      <div class="progress-bar">
        <div class="progress-fill" :style="{ width: `${progressPercent}%` }" />
      </div>
      <span class="progress-text">{{ progressPercent }}%</span>
    </div>

    <!-- Header -->
    <div v-show="!isSuccess" class="form-header">
      <h2 class="form-title">{{ modeConfig.title }}</h2>
      <p class="form-subtitle">{{ modeConfig.subtitle }}</p>
    </div>

    <!-- Error Message -->
    <div v-if="errors.submit" class="error-message">{{ errors.submit }}</div>

    <!-- Form / Success State wrapper with transition -->
    <transition name="fade" mode="out-in">
      <div v-if="isSuccess" key="success" class="success-state">
        <div class="success-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M20 6L9 17l-5-5" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
        <p class="success-text">
          <span v-if="mode === 'reset'">ссылка отправлена</span>
          <span v-else-if="mode === 'register'">аккаунт создан</span>
          <span v-else>вход выполнен</span>
        </p>
      </div>

      <div v-else key="form" class="form-body">
      <!-- Current Field -->
      <div v-if="!showSubmitButton" class="current-field-wrapper">
        <div class="field-step">
          <span class="step-number">{{ currentFieldIndex + 1 }} / {{ modeConfig.fields.length }}</span>
        </div>

        <BaseInput
          ref="fieldRefs"
          :key="currentField"
          v-model="currentValue"
          :type="currentField === 'password' || currentField === 'confirmPassword' ? 'password' : currentField === 'email' ? 'email' : 'text'"
          :label="modeConfig.fieldLabels[currentField]"
          :placeholder="modeConfig.fieldPlaceholders[currentField]"
          :error="currentError"
          required
          @keydown="handleKeydown"
        />

        <div class="field-actions">
          <BaseButton
            v-if="currentFieldIndex > 0"
            type="button"
            variant="secondary"
            size="md"
            @click="goToPrevField"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M19 12H5M12 19l-7-7 7-7" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            назад
          </BaseButton>

          <BaseButton
            type="button"
            variant="primary"
            size="md"
            :disabled="!canContinue"
            class="continue-btn"
            @click="goToNextField"
          >
            {{ isLastField ? 'завершить' : 'продолжить' }}
            <svg v-if="!isLastField" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M5 12h14M12 5l7 7-7 7" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </BaseButton>
        </div>
      </div>

      <!-- Submit Section -->
      <div v-else class="submit-section">
        <div class="summary-title">проверьте данные</div>

        <div class="summary-list">
          <div
            v-for="field in modeConfig.fields"
            :key="field"
            class="summary-item"
          >
            <span class="summary-label">{{ modeConfig.fieldLabels[field] }}</span>
            <span class="summary-value">
              {{ field === 'password' || field === 'confirmPassword' ? '••••••••' : formData[field] }}
            </span>
          </div>
        </div>

        <div class="submit-actions">
          <BaseButton
            type="button"
            variant="secondary"
            size="lg"
            @click="goToPrevField"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M19 12H5M12 19l-7-7 7-7" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            назад
          </BaseButton>

          <BaseButton
            type="button"
            variant="primary"
            size="lg"
            :loading="isLoading"
            @click="handleSubmit"
          >
            {{ modeConfig.submitText }}
          </BaseButton>
        </div>
      </div>

      <!-- Mode Switch -->
      <div class="form-footer">
        <span class="footer-text">{{ modeConfig.alternateText }}</span>
        <button
          type="button"
          class="link-button"
          @click="switchMode(mode === 'login' ? 'register' : 'login')"
        >
          {{ modeConfig.alternateAction }}
        </button>
      </div>

      <!-- Forgot Password -->
      <div v-if="mode === 'login'" class="form-links">
        <button
          type="button"
          class="link-button small"
          @click="switchMode('reset')"
        >
          забыли пароль?
        </button>
      </div>
    </div>
  </transition>
</div>
</template>

<style scoped lang="scss">
.auth-form {
  width: 100%;
  max-width: 400px;
}

// Progress Bar
.progress-container {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 20px;
}

.progress-bar {
  flex: 1;
  height: 2px;
  background: rgba($color-text, 0.1);
  border-radius: 1px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: $color-accent;
  transition: width 0.4s $transition-smooth;
}

.progress-text {
  font-size: 11px;
  color: $color-text-muted;
  font-variant-numeric: tabular-nums;
  min-width: 32px;
  text-align: right;
}

// Header
.form-header {
  margin-bottom: 24px;
}

.form-title {
  font-family: $font-display;
  font-size: 28px;
  font-weight: 600;
  letter-spacing: -0.02em;
  text-transform: lowercase;
  margin-bottom: 4px;
}

.form-subtitle {
  font-size: 14px;
  color: $color-text-dim;
}

// Current Field
.current-field-wrapper {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.field-step {
  display: flex;
  align-items: center;
  gap: 12px;
}

.step-number {
  font-size: 12px;
  color: $color-accent;
  font-weight: 500;
  font-variant-numeric: tabular-nums;
}

.field-actions {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  margin-top: 8px;
}

.continue-btn {
  flex: 1;

  svg {
    margin-left: 4px;
  }
}

// Submit Section
.submit-section {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.summary-title {
  font-size: 14px;
  color: $color-text-dim;
  text-align: center;
}

.summary-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 16px;
  background: rgba($color-surface, 0.5);
  border: 1px solid $color-border;
  border-radius: 12px;
}

.summary-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.summary-label {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  color: $color-text-muted;
}

.summary-value {
  font-size: 13px;
  color: $color-text;
  max-width: 180px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.submit-actions {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
  margin-top: 8px;

  > * {
    flex: 1;
    min-width: 120px;
  }
}

// Footer
.form-footer {
  display: flex;
  align-items: baseline;
  justify-content: center;
  gap: 8px;
  margin-top: 24px;
  padding-top: 16px;
  border-top: 1px solid $color-border;
}

.footer-text {
  font-family: $font-body;
  font-size: 13px;
  color: $color-text-dim;
}

.form-links {
  display: flex;
  justify-content: center;
  margin-top: 12px;
}

.link-button {
  background: transparent;
  border: none;
  color: $color-accent;
  font-family: $font-body;
  font-size: 13px;
  cursor: pointer;
  padding: 0;
  font-weight: 500;
  position: relative;
  transition: all 0.3s ease;

  // Animated underline
  &::after {
    content: '';
    position: absolute;
    bottom: -2px;
    left: 0;
    width: 0;
    height: 1px;
    background: $color-accent;
    transition: width 0.4s $transition-smooth;
  }

  &:hover {
    color: lighten($color-accent, 10%);
    text-shadow: 0 0 10px rgba($color-accent, 0.5);

    &::after {
      width: 100%;
    }

    // Glitch effect simulation
    animation: link-glitch 0.4s ease;
  }

  // Small links variant
  &.small {
    font-size: 12px;
    color: $color-text-muted;

    &::after {
      background: $color-text-dim;
    }

    &:hover {
      color: $color-text;
      text-shadow: 0 0 8px rgba($color-text, 0.3);

      &::after {
        background: $color-text;
      }
    }

    &:active {
      color: $color-accent;
      text-shadow: 0 0 15px rgba($color-accent, 0.7);
    }
  }

  &:active {
    transform: scale(0.98);
  }
}

@keyframes link-glitch {
  0%, 100% {
    transform: translateX(0);
  }
  25% {
    transform: translateX(-1px);
  }
  50% {
    transform: translateX(1px);
  }
  75% {
    transform: translateX(-1px);
  }
}

// Error Message
.error-message {
  padding: 12px 16px;
  background: rgba($color-error, 0.1);
  border: 1px solid rgba($color-error, 0.3);
  border-radius: 8px;
  color: $color-error;
  font-size: 13px;
  text-align: center;
  margin-bottom: 16px;
}

// Success State
.success-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  padding: 40px 0;
}

.success-icon {
  width: 72px;
  height: 72px;
  border-radius: 50%;
  background: rgba($color-accent, 0.1);
  border: 1px solid rgba($color-accent, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;
  color: $color-accent;
  animation: scaleIn 0.4s ease;

  svg {
    width: 36px;
    height: 36px;
  }
}

.success-text {
  font-size: 18px;
  font-weight: 500;
}

@keyframes scaleIn {
  from {
    transform: scale(0);
    opacity: 0;
  }
  to {
    transform: scale(1);
    opacity: 1;
  }
}

// Transitions
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

// Mobile Adaptations
@media (max-width: 480px) {
  .auth-form {
    padding: 0 8px;
  }

  .form-title {
    font-size: 24px;
  }

  .form-subtitle {
    font-size: 13px;
  }

  .field-actions {
    flex-direction: column;
    gap: 10px;

    > * {
      width: 100%;
    }
  }

  .continue-btn {
    flex: none;
    width: 100%;
  }

  .completed-value {
    max-width: 140px;
  }

  .summary-value {
    max-width: 120px;
  }

  .submit-actions {
    flex-direction: column;
    gap: 10px;

    > * {
      width: 100%;
      min-width: auto;
    }
  }

  .form-footer {
    flex-wrap: wrap;
    text-align: center;
  }
}
</style>
