<template>
  <div class="rules-page">
    <div class="container">
      <div class="page-card" ref="pageCard">
        <div class="page-header">
          <h1 class="page-title">Правила публикации хакатона</h1>
          <p class="page-subtitle">
            Перед созданием хакатона, пожалуйста, ознакомьтесь с требованиями к публикации
          </p>
        </div>

        <div class="rules-content">
          <div class="rule-section" v-for="(section, index) in ruleSections" :key="index">
            <div class="rule-icon">
              <component :is="section.icon" />
            </div>
            <div class="rule-body">
              <h3 class="rule-title">{{ section.title }}</h3>
              <p class="rule-description">{{ section.description }}</p>
              <ul v-if="section.items" class="rule-list">
                <li v-for="(item, i) in section.items" :key="i">{{ item }}</li>
              </ul>
            </div>
          </div>
        </div>

        <div class="acceptance-check" ref="acceptanceCheck">
          <label class="checkbox-wrapper">
            <input
              type="checkbox"
              v-model="accepted"
              class="checkbox-input"
            />
            <span class="checkbox-custom">
              <svg v-if="accepted" class="checkbox-check" viewBox="0 0 24 24">
                <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
              </svg>
            </span>
            <span class="checkbox-label">
              Я ознакомился с правилами и обязуюсь их соблюдать
            </span>
          </label>
        </div>

        <div class="actions" ref="actions">
          <button
            class="btn btn-secondary"
            @click="goBack"
          >
            <span class="btn-icon">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M19 12H5M12 19l-7-7 7-7"/>
              </svg>
            </span>
            <span class="btn-text">Назад</span>
          </button>
          <button
            class="btn btn-primary"
            :disabled="!accepted"
            @click="proceed"
          >
            <span class="btn-text">Продолжить</span>
            <span class="btn-icon">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M5 12h14M12 5l7 7-7 7"/>
              </svg>
            </span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import gsap from 'gsap'

const router = useRouter()
const accepted = ref(false)
const pageCard = ref<HTMLElement | null>(null)
const acceptanceCheck = ref<HTMLElement | null>(null)
const actions = ref<HTMLElement | null>(null)

const ruleSections = [
  {
    icon: 'DocumentIcon',
    title: 'Актуальность информации',
    description: 'Все данные о хакатоне должны быть актуальными и соответствовать действительности.',
    items: [
      'Проверьте правильность дат проведения',
      'Укажите реальные призовые фонды',
      'Добавьте актуальные контактные данные',
    ],
  },
  {
    icon: 'ShieldIcon',
    title: 'Требования к содержанию',
    description: 'Хакатон должен соответствовать нашим стандартам качества.',
    items: [
      'Недопустима дискриминация по любому признаку',
      'Запрещена пропаганда наркотиков и насилия',
      'Требуется чёткое описание задач и треков',
    ],
  },
  {
    icon: 'ClockIcon',
    title: 'Сроки и дедлайны',
    description: 'Указывайте реалистичные сроки и придерживайтесь их.',
    items: [
      'Минимальный срок регистрации — 7 дней',
      'Нельзя переносить даты без уважительной причины',
      'Соблюдайте заявленные дедлайны оценки проектов',
    ],
  },
  {
    icon: 'UsersIcon',
    title: 'Обязательства перед участниками',
    description: 'Организатор обязуется выполнять свои обещания.',
    items: [
      'Призовой фонд должен быть выплачен в срок',
      'Предоставьте рабочую инфраструктуру',
      'Отвечайте на вопросы участников',
    ],
  },
  {
    icon: 'LockIcon',
    title: 'Публикация и редактирование',
    description: 'Важные правила о публикации хакатонов.',
    items: [
      'Хакатон публикуется сразу после создания',
      'Редактирование запрещено — проверьте данные перед сохранением',
      'Удаление возможно только до начала регистрации',
    ],
  },
]

const goBack = () => {
  router.push('/organizers')
}

const proceed = () => {
  if (accepted.value) {
    localStorage.setItem('hackathon_rules_accepted', 'true')
    router.push('/organizers/profile')
  }
}

onMounted(() => {
  gsap.fromTo(pageCard.value,
    { y: 40, opacity: 0 },
    { y: 0, opacity: 1, duration: 0.6, ease: 'power2.out' }
  )

  gsap.fromTo(acceptanceCheck.value,
    { y: 20, opacity: 0 },
    { y: 0, opacity: 1, duration: 0.5, delay: 0.3, ease: 'power2.out' }
  )

  gsap.fromTo(actions.value,
    { y: 20, opacity: 0 },
    { y: 0, opacity: 1, duration: 0.5, delay: 0.4, ease: 'power2.out' }
  )
})
</script>

<style scoped lang="scss">
@use '../styles/variables' as *;

.rules-page {
  min-height: 100vh;
  background: var(--bg-color);
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
  background: var(--card-bg);
  border: 1px solid var(--border-color);
  border-radius: 2rem;
  padding: 2.5rem;
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.05);
}

.page-header {
  text-align: center;
  margin-bottom: 2.5rem;

  .page-title {
    font-size: clamp(1.75rem, 4vw, 2.25rem);
    font-weight: 700;
    margin-bottom: 0.75rem;
    background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .page-subtitle {
    color: var(--text-secondary);
    font-size: 1rem;
  }
}

.rules-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  margin-bottom: 2rem;
}

.rule-section {
  display: flex;
  gap: 1rem;
  padding: 1.5rem;
  background: rgba(var(--accent-primary-rgb), 0.05);
  border-radius: 1rem;
  border: 1px solid var(--border-color);
  transition: all 0.3s ease;

  &:hover {
    border-color: var(--accent-primary);
    transform: translateX(4px);
  }
}

.rule-icon {
  flex-shrink: 0;
  width: 48px;
  height: 48px;
  background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
  border-radius: 0.75rem;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 1.25rem;
}

.rule-body {
  flex: 1;
}

.rule-title {
  font-size: 1.1rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
  color: var(--text-color);
}

.rule-description {
  color: var(--text-secondary);
  font-size: 0.95rem;
  margin-bottom: 0.75rem;
  line-height: 1.5;
}

.rule-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  list-style: none;

  li {
    position: relative;
    padding-left: 1.25rem;
    font-size: 0.9rem;
    color: var(--text-secondary);

    &::before {
      content: '';
      position: absolute;
      left: 0;
      top: 0.5rem;
      width: 6px;
      height: 6px;
      background: var(--accent-primary);
      border-radius: 50%;
    }
  }
}

.acceptance-check {
  padding: 1.5rem;
  background: rgba(var(--accent-primary-rgb), 0.05);
  border-radius: 1rem;
  margin-bottom: 1.5rem;
}

.checkbox-wrapper {
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
  cursor: pointer;
}

.checkbox-input {
  display: none;
}

.checkbox-custom {
  flex-shrink: 0;
  width: 24px;
  height: 24px;
  border: 2px solid var(--border-color);
  border-radius: 0.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  margin-top: 2px;

  .checkbox-input:checked + & {
    background: var(--accent-primary);
    border-color: var(--accent-primary);
  }
}

.checkbox-check {
  width: 16px;
  height: 16px;
  fill: white;
}

.checkbox-label {
  font-size: 0.95rem;
  color: var(--text-color);
  line-height: 1.5;
}

.actions {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;

  @media (max-width: 480px) {
    flex-direction: column;
  }
}

.btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.875rem 1.5rem;
  border-radius: 0.75rem;
  font-weight: 600;
  font-size: 1rem;
  transition: all 0.3s ease;
  cursor: pointer;
  border: none;

  &.btn-primary {
    background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
    color: white;
    min-width: 160px;
    justify-content: center;

    &:hover:not(:disabled) {
      transform: translateY(-2px);
      box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
    }

    &:disabled {
      opacity: 0.5;
      cursor: not-allowed;
    }
  }

  &.btn-secondary {
    background: transparent;
    color: var(--text-color);
    border: 2px solid var(--border-color);

    &:hover {
      border-color: var(--accent-primary);
      color: var(--accent-primary);
    }
  }
}
</style>
