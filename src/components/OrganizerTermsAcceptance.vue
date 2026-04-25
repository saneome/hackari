<template>
  <div class="terms-page">
    <div class="progress-bar-container">
      <div class="progress-bar" :style="{ width: progressPercent + '%' }"></div>
    </div>
    
    <div class="slides-container">
      <Transition :name="transitionName" mode="out-in">
        <div class="slide" :key="currentSlide">
          
          <div class="slide-left">
            <div class="icon-wrapper">
              <component :is="currentTerm.icon" class="animated-icon" />
              <div class="icon-glow"></div>
            </div>
          </div>
          
          <div class="slide-right">
            <div class="slide-content">
              <span class="slide-counter">Правило {{ currentSlide + 1 }} из {{ terms.length }}</span>
              <h1 class="slide-title">{{ currentTerm.title }}</h1>
              
              <div class="slide-description">
                <p>{{ currentTerm.description }}</p>
              </div>

            </div>
          </div>

        </div>
      </Transition>
    </div>

    <div v-if="isLastSlide" class="slide-action slide-action--page">
      <label class="checkbox-wrapper" :class="{ 'is-checked': currentTerm.accepted }">
        <input 
          type="checkbox" 
          v-model="currentTerm.accepted" 
          class="hidden-checkbox"
        />
        <div class="custom-checkbox">
          <Check v-if="currentTerm.accepted" :size="18" stroke-width="3" />
        </div>
        <span class="checkbox-label">Я ознакомился и принимаю все условия</span>
      </label>
    </div>

    <div class="navigation-controls">
      <button 
        class="btn-nav btn-prev" 
        @click="prevSlide" 
        :disabled="currentSlide === 0"
      >
        <ArrowLeft :size="20" />
        <span>Назад</span>
      </button>
      
      <button 
        v-if="currentSlide < terms.length - 1"
        class="btn-nav btn-next" 
        @click="nextSlide" 
      >
        <span>Далее</span>
        <ArrowRight :size="20" />
      </button>

      <button 
        v-else
        class="btn-nav btn-finish" 
        @click="finishValidation" 
        :disabled="!allAccepted || isSubmittingAcceptance"
      >
        <span>Завершить</span>
        <Check :size="20" />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, markRaw } from 'vue'
import { useRouter } from 'vue-router'
import { userApi } from '@/services/api'
import { 
  Check, 
  ArrowRight, 
  ArrowLeft, 
  ShieldCheck, 
  Users, 
  MessageSquare, 
  BadgeCheck,
  Wallet 
} from 'lucide-vue-next'

const router = useRouter()
const currentSlide = ref(0)
const transitionName = ref('slide-right')
const isSubmittingAcceptance = ref(false)

const terms = ref([
  {
    title: 'Честность и прозрачность',
    description: 'Мы ожидаем от организаторов полной прозрачности во всех аспектах проведения мероприятий на нашей платформе. Публикуйте только достоверную информацию о хакатонах. Призовые фонды должны соответствовать заявленным изначально, а даты проведения и дедлайны должны быть реальными и строго соблюдаться.',
    icon: markRaw(ShieldCheck),
    accepted: false,
  },
  {
    title: 'Уважение к участникам',
    description: 'Основа успешного комьюнити — это взаимное уважение между организаторами и участниками. Взаимодействуйте с участниками как с партнерами. Оперативно отвечайте на вопросы, оказывайте поддержку командам и не допускайте дискриминации по любым признакам.',
    icon: markRaw(Users),
    accepted: false,
  },
  {
    title: 'Обратная связь и коммуникация',
    description: 'Организатор обязуется поддерживать активные каналы связи на протяжении всего мероприятия. Обеспечьте актуальные контактные данные, следите за обновлениями и будьте на связи с администрацией платформы. Регулярно обновляйте статусы и информируйте участников обо всех изменениях.',
    icon: markRaw(MessageSquare),
    accepted: false,
  },
  {
    title: 'Качество материалов',
    description: 'Предоставляемый контент должен быть высокого качества и полностью раскрывать суть мероприятия. Описания задач должны быть четкими и недвусмысленными. Сформулируйте прозрачные критерии оценки для жюри и участников, убедитесь, что требования к командам реалистичны и выполнимы.',
    icon: markRaw(BadgeCheck),
    accepted: false,
  },
  {
    title: 'Финансовые обязательства',
    description: 'Выплата призов и вознаграждений — критически важный фактор доверия к платформе. Гарантируйте выплаты призов в установленные правилами сроки. Выплачивайте полные суммы без скрытых комиссий и вычетов, сохраняя процедуру награждения максимально открытой и понятной.',
    icon: markRaw(Wallet),
    accepted: false,
  },
])

const currentTerm = computed(() => terms.value[currentSlide.value])
const isLastSlide = computed(() => currentSlide.value === terms.value.length - 1)
const allAccepted = computed(() => terms.value.every(t => t.accepted))
const acceptedCount = computed(() => terms.value.filter(t => t.accepted).length)
const progressPercent = computed(() => (acceptedCount.value / terms.value.length) * 100)

const nextSlide = () => {
  if (currentSlide.value < terms.value.length - 1) {
    terms.value[currentSlide.value].accepted = true
    transitionName.value = 'slide-right'
    currentSlide.value++
  }
}

const prevSlide = () => {
  if (currentSlide.value > 0) {
    transitionName.value = 'slide-left'
    currentSlide.value--
  }
}

const finishValidation = () => {
  if (!allAccepted.value || isSubmittingAcceptance.value) return

  isSubmittingAcceptance.value = true

  void userApi.acceptOrganizerTerms().then((response) => {
    if (response.error) {
      isSubmittingAcceptance.value = false
      return
    }

    router.replace('/organizers/profile')
  })
}

onMounted(() => {
  void userApi.getMe().then((response) => {
    if (response.data?.organizerTermsAcceptedAt) {
      router.replace('/organizers/profile')
    }
  })
})
</script>

<style scoped lang="scss">
@use '../styles/variables' as *;

.terms-page {
  min-height: 100vh;
  background-color: #050505;
  color: #fff;
  display: flex;
  flex-direction: column;
  position: relative;
  overflow: hidden;
}

.progress-bar-container {
  position: fixed;
  top: 0;
  left: 0;
  height: 6px;
  width: 100%;
  background: rgba(255, 255, 255, 0.05);
  z-index: 100;
}

.progress-bar {
  height: 100%;
  background: linear-gradient(90deg, #b0f551, #e3ff73);
  transition: width 0.5s cubic-bezier(0.2, 0.8, 0.2, 1);
  box-shadow: 0 0 15px rgba(176, 245, 81, 0.5);
}

.slides-container {
  flex: 1;
  display: flex;
  width: 100%;
  height: 100vh;
  position: relative;
}

.slide {
  position: absolute;
  top: clamp(2.5rem, 5vh, 4rem);
  left: 0;
  width: 100%;
  bottom: 0;
  height: auto;
  box-sizing: border-box;
  display: grid;
  grid-template-columns: minmax(320px, 0.9fr) minmax(0, 1.1fr);
  align-items: center;
  justify-content: center;
  gap: clamp(2rem, 5vw, 5rem);
  padding: 0 clamp(1.5rem, 5vw, 5rem) 5rem;
  background: radial-gradient(circle at left center, rgba($color-accent, 0.05) 0%, transparent 60%);
}

.slide-left {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  padding-top: clamp(1.5rem, 4vh, 3.5rem);
}

.icon-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  animation: icon-shell-enter 0.9s cubic-bezier(0.2, 0.8, 0.2, 1) both;
}

.animated-icon {
  width: 18vw;
  height: 18vw;
  min-width: 250px;
  min-height: 250px;
  max-width: 450px;
  max-height: 450px;
  color: $color-accent;
  filter: drop-shadow(0 0 30px rgba($color-accent, 0.3));
  animation: icon-float 6s ease-in-out 0.9s infinite;
  z-index: 2;
}

.icon-glow {
  position: absolute;
  width: 20vw;
  height: 20vw;
  background: $color-accent;
  filter: blur(120px);
  opacity: 0.15;
  border-radius: 50%;
  animation: glow-pulse 4s ease-in-out infinite alternate;
  z-index: 1;
}

.slide-right {
  height: 100%;
  display: flex;
  align-items: flex-start;
  padding: clamp(6rem, 12vh, 12rem) 10vw 6rem 4vw;
}

.slide-content {
  max-width: 720px;
  display: grid;
  gap: 0;
}

.slide-counter {
  display: inline-block;
  font-size: 0.9rem;
  text-transform: uppercase;
  letter-spacing: 0.15em;
  color: $color-accent;
  font-weight: 600;
  margin-bottom: 2rem;
  animation: content-rise 0.7s cubic-bezier(0.2, 0.8, 0.2, 1) both;
}

.slide-title {
  font-size: clamp(2.2rem, 3.5vw, 3.45rem);
  font-weight: 700;
  margin-bottom: 1.5rem;
  background: linear-gradient(135deg, #fff, #999);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  line-height: 1.1;
  letter-spacing: -0.02em;
  animation: content-rise 0.75s cubic-bezier(0.2, 0.8, 0.2, 1) both;
  animation-delay: 0.08s;
}

.slide-description {
  font-size: clamp(1.1rem, 1.35vw, 1.25rem);
  color: #a0a0a0;
  line-height: 1.6;
  margin-bottom: 3.5rem;
  animation: content-rise 0.78s cubic-bezier(0.2, 0.8, 0.2, 1) both;
  animation-delay: 0.16s;
  
  p {
    margin: 0;
  }
}

.slide-action {
  margin-top: 2rem;
  animation: content-rise 0.8s cubic-bezier(0.2, 0.8, 0.2, 1) both;
  animation-delay: 0.24s;
}

.slide-action--page {
  position: fixed;
  left: 4vw;
  bottom: 4vw;
  z-index: 60;
  margin-top: 0;
  animation: page-action-rise 0.8s cubic-bezier(0.2, 0.8, 0.2, 1) both;
}

.checkbox-wrapper {
  display: inline-flex;
  align-items: center;
  gap: 1.2rem;
  cursor: pointer;
  padding: 1.2rem 1.8rem;
  border-radius: 20px;
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid rgba(255, 255, 255, 0.08);
  transition: all 0.3s ease;
  
  &:hover {
    background: rgba(255, 255, 255, 0.06);
    transform: translateY(-2px);
  }
  
  &.is-checked {
    background: rgba($color-accent, 0.08);
    border-color: rgba($color-accent, 0.4);
    
    .checkbox-label {
      color: #fff;
    }
  }
}

.hidden-checkbox {
  position: absolute;
  opacity: 0;
  cursor: pointer;
  height: 0;
  width: 0;
}

.custom-checkbox {
  width: 32px;
  height: 32px;
  border-radius: 10px;
  border: 2px solid rgba(255, 255, 255, 0.4);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  color: #000;
  
  .is-checked & {
    background: $color-accent;
    border-color: $color-accent;
    transform: scale(1.05);
  }
}

.checkbox-label {
  font-size: 1.2rem;
  color: #888;
  font-weight: 500;
  transition: color 0.3s ease;
  user-select: none;
}

.navigation-controls {
  position: fixed;
  bottom: 4vw;
  right: 4vw;
  display: flex;
  gap: 1rem;
  z-index: 50;
}

.btn-nav {
  display: flex;
  align-items: center;
  gap: 0.8rem;
  padding: 1.2rem 2.2rem;
  border-radius: 16px;
  font-family: $font-body;
  font-size: 1.15rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  border: none;
  backdrop-filter: blur(10px);
  
  &:disabled {
    opacity: 0.3;
    cursor: not-allowed;
    transform: none !important;
  }
}

.btn-prev {
  background: rgba(255, 255, 255, 0.05);
  color: #fff;
  border: 1px solid rgba(255, 255, 255, 0.1);
  
  &:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.1);
  }
}

.btn-next, .btn-finish {
  background: $color-accent;
  color: #000;
  box-shadow: 0 8px 25px rgba($color-accent, 0.25);
  
  &:hover:not(:disabled) {
    transform: translateY(-3px);
    box-shadow: 0 12px 30px rgba($color-accent, 0.4);
  }
}

/* Animations & Transitions */
@keyframes icon-float {
  0% { transform: translateY(0px) rotate(0deg); }
  50% { transform: translateY(-20px) rotate(2deg); }
  100% { transform: translateY(0px) rotate(0deg); }
}

@keyframes icon-shell-enter {
  0% {
    opacity: 0;
    transform: translateX(-30px) scale(0.92);
  }
  100% {
    opacity: 1;
    transform: translateX(0) scale(1);
  }
}

@keyframes content-rise {
  0% {
    opacity: 0;
    transform: translateY(22px);
    filter: blur(6px);
  }
  100% {
    opacity: 1;
    transform: translateY(0);
    filter: blur(0);
  }
}

@keyframes glow-pulse {
  0% { opacity: 0.1; transform: scale(1); }
  50% { opacity: 0.2; transform: scale(1.1); }
  100% { opacity: 0.1; transform: scale(1); }
}

@keyframes page-action-rise {
  0% {
    opacity: 0;
    transform: translateY(18px);
  }
  100% {
    opacity: 1;
    transform: translateY(0);
  }
}

.slide-right-enter-active,
.slide-right-leave-active,
.slide-left-enter-active,
.slide-left-leave-active {
  transition: all 0.7s cubic-bezier(0.2, 0.8, 0.2, 1);
}

.slide-right-enter-from {
  opacity: 0;
  transform: translateY(40px) scale(0.98);
}
.slide-right-leave-to {
  opacity: 0;
  transform: translateY(-40px) scale(0.98);
}

.slide-left-enter-from {
  opacity: 0;
  transform: translateY(-40px) scale(0.98);
}
.slide-left-leave-to {
  opacity: 0;
  transform: translateY(40px) scale(0.98);
}

@media (max-width: 992px) {
  .slide {
    top: 0;
    grid-template-columns: 1fr;
    gap: 1.5rem;
    padding: 7rem 1.5rem 9rem;
  }
  
  .slide-left {
    height: auto;
    padding-top: 0;
    background: transparent;
  }
  
  .animated-icon {
    width: 30vw;
    height: 30vw;
    min-width: 150px;
    min-height: 150px;
  }
  
  .slide-right {
    height: auto;
    padding: 0;
    align-items: flex-start;
  }
  
  .slide-content {
    max-width: 100%;
  }
  
  .slide-counter {
    margin-bottom: 0.5rem;
  }

  .slide-title {
    font-size: clamp(1.8rem, 3.5vw, 3rem);
    margin-bottom: 0.5rem;
  }

  .slide-description {
    font-size: clamp(1rem, 1.35vw, 1.25rem);
  }
  
  .navigation-controls {
    bottom: 2rem;
    right: 2rem;
    left: 2rem;
    justify-content: space-between;
  }
  
  .btn-nav {
    padding: 0.8rem 1rem;
    font-size: 0.95rem;
  }

  .checkbox-wrapper {
    padding: 0.8rem 1.2rem;
    gap: 0.8rem;
  }

  .custom-checkbox {
    width: 24px;
    height: 24px;
    border-radius: 8px;
  }

  .checkbox-label {
    font-size: 1rem;
  }

  .slide-action--page {
    left: 1rem;
    right: 1rem;
    bottom: 7rem;
  }
}
</style>
