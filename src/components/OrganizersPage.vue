<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { gsap } from 'gsap'
import { ScrollTrigger } from 'gsap/ScrollTrigger'
import { useRouter } from 'vue-router'
import { Users, Megaphone, Network, Lightbulb, ArrowRight, CheckCircle2, Briefcase } from 'lucide-vue-next'

gsap.registerPlugin(ScrollTrigger)

const router = useRouter()

const heroSection = ref<HTMLElement | null>(null)
const titleWrapper = ref<HTMLElement | null>(null)
const geometricShape = ref<HTMLElement | null>(null)
const ctaButton = ref<HTMLButtonElement | null>(null)
const benefitsSection = ref<HTMLElement | null>(null)
const benefitCards = ref<HTMLElement[]>([])
const stepsSection = ref<HTMLElement | null>(null)
const stepItems = ref<HTMLElement[]>([])

// Kinetic typography letters
const titleLetters = computed(() => 'организаторам'.split(''))

const benefits = [
  {
    id: 1,
    number: '01',
    icon: Users,
    title: 'найдите таланты',
    description: 'получите доступ к базе участников и найдите лучших специалистов для вашей компании',
  },
  {
    id: 2,
    number: '02',
    icon: Megaphone,
    title: 'повысьте узнаваемость',
    description: 'расскажите о вашей компании или сообществе целевой аудитории',
  },
  {
    id: 3,
    number: '03',
    icon: Network,
    title: 'развивайте коммьюнити',
    description: 'способствуйте развитию it-экосистемы и нетворкингу',
  },
  {
    id: 4,
    number: '04',
    icon: Lightbulb,
    title: 'создавайте решения',
    description: 'получайте инновационные решения от участников для ваших задач',
  },
]

const steps = [
  { num: '01', title: 'создайте профиль', desc: 'заполните информацию об организации' },
  { num: '02', title: 'примите правила', desc: 'ознакомьтесь с условиями проведения' },
  { num: '03', title: 'создайте хакатон', desc: 'настройте событие через визард' },
]

const navigateToTerms = () => {
  router.push('/organizers/rules')
}

const scrollToBenefits = () => {
  benefitsSection.value?.scrollIntoView({ behavior: 'smooth' })
}

onMounted(() => {
  // Kinetic typography entrance
  const letters = titleWrapper.value?.querySelectorAll('.letter')
  if (letters) {
    gsap.fromTo(letters,
      { y: 100, opacity: 0, rotateX: -90 },
      {
        y: 0,
        opacity: 1,
        rotateX: 0,
        duration: 1.2,
        stagger: 0.06,
        ease: 'power4.out',
        delay: 0.3,
      }
    )
  }

  // Geometric shape entrance
  if (geometricShape.value) {
    gsap.fromTo(geometricShape.value,
      { scale: 0, opacity: 0 },
      {
        scale: 1,
        opacity: 0.5,
        duration: 1.5,
        ease: 'back.out(1.7)',
        delay: 0.6,
      }
    )
  }

  // Hero icon entrance
  const heroIcon = heroSection.value?.querySelector('.hero-icon')
  if (heroIcon) {
    gsap.fromTo(heroIcon,
      { scale: 0.5, opacity: 0, rotate: -20 },
      {
        scale: 1,
        opacity: 1,
        rotate: 0,
        duration: 0.8,
        ease: 'back.out(1.7)',
        delay: 1.2,
      }
    )
  }

  // Hero subtitle entrance
  const heroSubtitle = heroSection.value?.querySelector('.hero-subtitle')
  if (heroSubtitle) {
    gsap.fromTo(heroSubtitle,
      { y: 20, opacity: 0 },
      {
        y: 0,
        opacity: 1,
        duration: 0.6,
        ease: 'power3.out',
        delay: 1.0,
      }
    )
  }

  // CTA buttons entrance
  const heroButtons = heroSection.value?.querySelector('.hero-buttons')
  if (heroButtons) {
    gsap.fromTo(heroButtons,
      { y: 30, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.8, ease: 'power3.out', delay: 1.4 }
    )
  }

  // Scroll-triggered page turn effect for hero
  ScrollTrigger.create({
    trigger: heroSection.value,
    start: 'top top',
    end: 'bottom top',
    scrub: 0.5,
    onUpdate: (self) => {
      const progress = self.progress
      const shapeInner = heroSection.value?.querySelector('.shape-inner') as HTMLElement
      if (shapeInner) {
        shapeInner.style.transform = `
          translateY(${-20 + progress * -30}px)
          rotateZ(${2 + progress * -4}deg)
          rotateX(${progress * -15}deg)
        `
      }
    },
  })

  // Benefits cards animation with 3D flip
  const benefitsGrid = document.querySelector('.benefits-grid')
  const benefitCardsList = benefitsGrid?.querySelectorAll('.benefit-card')
  benefitCardsList?.forEach((card, index) => {
    gsap.fromTo(card,
      {
        rotateY: -90,
        opacity: 0,
        transformOrigin: 'center center',
      },
      {
        rotateY: 0,
        opacity: 1,
        duration: 1,
        ease: 'power3.out',
        scrollTrigger: {
          trigger: card,
          start: 'top 75%',
          toggleActions: 'play none none reverse',
        },
        delay: index * 0.2,
      }
    )
  })

  // Steps animation
  stepItems.value.forEach((step, index) => {
    gsap.fromTo(step,
      { x: index % 2 === 0 ? -40 : 40, opacity: 0 },
      {
        x: 0,
        opacity: 1,
        duration: 0.6,
        delay: index * 0.15,
        ease: 'power2.out',
        scrollTrigger: {
          trigger: step,
          start: 'top 85%',
        },
      }
    )
  })
})
</script>

<template>
  <div class="organizers-page">
    <!-- Hero Section -->
    <section ref="heroSection" class="hero-section">
      <div class="hero-content">
        <!-- 3D Geometric Shape -->
      <div ref="geometricShape" class="geometric-shape">
          <div class="shape-inner">
            <div class="face face-1" />
            <div class="face face-2" />
            <div class="face face-3" />
            <div class="face face-4" />
            <div class="face face-5" />
            <div class="face face-6" />
            <div class="edges">
              <div v-for="i in 6" :key="i" class="edge" />
            </div>
          </div>
        </div>


      <!-- Briefcase Icon -->
      <div class="hero-icon-wrapper">
        <div class="hero-icon">
          <Briefcase :size="48" />
        </div>
      </div>
        <!-- Title -->
        <div ref="titleWrapper" class="title-wrapper">
          <h1 class="hero-title">
            <span
              v-for="(letter, index) in titleLetters"
              :key="index"
              class="letter"
              :class="{ 'letter-accent': index >= 10 }"
            >
              {{ letter }}
            </span>
            <span class="cursor cursor-blink" />
          </h1>
        </div>

        <p class="hero-subtitle mono text-dim">
          // создавайте хакатоны, находите таланты и развивайте it-сообщество
        </p>

        <div class="hero-buttons">
          <button ref="ctaButton" class="cta-button" @click="navigateToTerms">
            <span class="button-text">стать организатором</span>
            <span class="button-icon">
              <ArrowRight :size="18" />
            </span>
            <span class="button-bg" />
          </button>

          <button class="cta-button outline" @click="scrollToBenefits">
            <span class="button-text">узнать больше</span>
            <span class="button-bg" />
          </button>
        </div>
      </div>
    </section>

    <!-- Benefits Section -->
    <section ref="benefitsSection" class="benefits-section">
      <div class="container">
        <div class="section-header">
          <h2 class="section-title">
            <span class="mono text-dim">// </span>
            <span>почему стоит организовать хакатон</span>
          </h2>
        </div>

        <div ref="benefitsGrid" class="benefits-grid">
          <article
            v-for="(benefit, index) in benefits"
            :key="benefit.id"
            ref="benefitCards"
            class="benefit-card"
            :class="`benefit-${benefit.id}`"
            :style="{ transform: `translateY(${index * 40}px)` }"
          >
            <div class="card-inner">
              <div class="benefit-number mono">{{ benefit.number }}</div>

              <div class="benefit-icon">
                <component :is="benefit.icon" :size="32" stroke-width="1.5" />
              </div>

              <h3 class="benefit-title">{{ benefit.title }}</h3>
              <p class="benefit-description">{{ benefit.description }}</p>

              <div class="benefit-indicator">
                <div class="indicator-line" />
                <div class="indicator-dot" />
              </div>
            </div>

            <div class="card-glow" />
          </article>
        </div>
      </div>
    </section>

    <!-- How It Works Section -->
    <section ref="stepsSection" class="steps-section">
      <div class="container">
        <div class="section-header">
          <h2 class="section-title">
            <span class="mono text-dim">// </span>
            <span>как это работает</span>
          </h2>
        </div>

        <div class="steps-list">
          <div
            v-for="(step, index) in steps"
            :key="step.num"
            ref="stepItems"
            class="step-item"
            :class="{ 'step-reverse': index % 2 === 1 }"
          >
            <div class="step-content">
              <span class="step-number mono">{{ step.num }}</span>
              <h3 class="step-title">{{ step.title }}</h3>
              <p class="step-desc text-dim">{{ step.desc }}</p>
            </div>
            <div class="step-line" />
            <div v-if="index === steps.length - 1" class="step-connector">
              <CheckCircle2 :size="32" class="step-check" />
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- CTA Section -->
    <section class="cta-section">
      <div class="container">
        <div class="cta-content">
          <h2 class="cta-title">
            <span class="mono text-dim">// </span>
            <span>готовы создать свой хакатон?</span>
          </h2>
          <p class="cta-text text-dim">
            присоединяйтесь к сообществу организаторов hackari и создайте
            незабываемое событие для it-сообщества
          </p>
          <button class="cta-button large" @click="navigateToTerms">
            <span class="button-text">начать сейчас</span>
            <span class="button-icon">
              <ArrowRight :size="20" />
            </span>
            <span class="button-bg" />
          </button>
        </div>
      </div>
    </section>
  </div>
</template>

<style scoped lang="scss">
.organizers-page {
  min-height: 100vh;
  background: linear-gradient(180deg, #0a0a0a 0%, #111111 50%, #0a0a0a 100%);
}

// Hero Section
.hero-section {
  position: relative;
  height: 100vh;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}

.hero-content {
  position: relative;
  z-index: 10;
  text-align: center;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 24px;
}

// 3D Geometric Shape
.geometric-shape {
  position: absolute;
  top: 50%;
  right: 10%;
  width: 180px;
  height: 180px;
  transform-style: preserve-3d;
  perspective: 1000px;
  margin-top: -90px;
  z-index: 0;
  pointer-events: none;
  opacity: 0.5;

  @media (max-width: 768px) {
    display: none;
  }
}

.shape-inner {
  position: relative;
  width: 100%;
  height: 100%;
  transform-style: preserve-3d;
  animation: float 6s ease-in-out infinite;
}

.face {
  position: absolute;
  width: 140px;
  height: 140px;
  left: 50%;
  top: 50%;
  margin-left: -70px;
  margin-top: -70px;
  background: rgba($color-accent, 0.03);
  border: 1px solid rgba($color-accent, 0.15);
  backdrop-filter: blur(2px);

  @media (max-width: 768px) {
    width: 100px;
    height: 100px;
    margin-left: -50px;
    margin-top: -50px;
  }

  &.face-1 { transform: translateZ(70px); }
  &.face-2 { transform: rotateX(-90deg) translateZ(70px); }
  &.face-3 { transform: rotateY(90deg) translateZ(70px); }
  &.face-4 { transform: rotateY(-90deg) translateZ(70px); }
  &.face-5 { transform: rotateX(90deg) translateZ(70px); }
  &.face-6 { transform: rotateY(180deg) translateZ(70px); }

  @media (max-width: 768px) {
    &.face-1 { transform: translateZ(50px); }
    &.face-2 { transform: rotateX(-90deg) translateZ(50px); }
    &.face-3 { transform: rotateY(90deg) translateZ(50px); }
    &.face-4 { transform: rotateY(-90deg) translateZ(50px); }
    &.face-5 { transform: rotateX(90deg) translateZ(50px); }
    &.face-6 { transform: rotateY(180deg) translateZ(50px); }
  }
}

.edges {
  position: absolute;
  inset: 0;
  transform-style: preserve-3d;
}

.edge {
  position: absolute;
  width: 2px;
  height: 100%;
  left: 50%;
  background: rgba($color-accent, 0.4);
  box-shadow: 0 0 20px rgba($color-accent, 0.3);
  transform-origin: center;

  &:nth-child(1) { transform: translateZ(70px) rotateY(0deg); }
  &:nth-child(2) { transform: translateZ(70px) rotateY(90deg); }
  &:nth-child(3) { transform: translateZ(-70px) rotateY(0deg); }
  &:nth-child(4) { transform: translateZ(-70px) rotateY(90deg); }
  &:nth-child(5) { transform: rotateX(90deg) translateZ(70px); }
  &:nth-child(6) { transform: rotateX(90deg) translateZ(-70px); }

  @media (max-width: 768px) {
    &:nth-child(1) { transform: translateZ(50px) rotateY(0deg); }
    &:nth-child(2) { transform: translateZ(50px) rotateY(90deg); }
    &:nth-child(3) { transform: translateZ(-50px) rotateY(0deg); }
    &:nth-child(4) { transform: translateZ(-50px) rotateY(90deg); }
    &:nth-child(5) { transform: rotateX(90deg) translateZ(50px); }
    &:nth-child(6) { transform: rotateX(90deg) translateZ(-50px); }
  }
}

@keyframes float {
  0%, 100% { transform: translateY(0) rotateZ(0deg); }
  50% { transform: translateY(-20px) rotateZ(2deg); }
}

// Hero Icon
.hero-icon-wrapper {
  transform-style: preserve-3d;
  perspective: 600px;
}

.hero-icon {
  width: 100px;
  height: 100px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, rgba($color-accent, 0.2), rgba($color-accent, 0.05));
  border: 1px solid rgba($color-accent, 0.3);
  border-radius: 24px;
  color: $color-accent;
  position: relative;
  z-index: 2;
  opacity: 0;

  &::before {
    content: '';
    position: absolute;
    inset: -10px;
    background: radial-gradient(circle, rgba($color-accent, 0.2) 0%, transparent 70%);
    filter: blur(20px);
    z-index: -1;
    animation: pulse-glow 3s ease-in-out infinite;
  }
}

@keyframes pulse-glow {
  0%, 100% { opacity: 0.5; transform: scale(1); }
  50% { opacity: 0.8; transform: scale(1.1); }
}

// Title
.title-wrapper {
  overflow: hidden;
  perspective: 600px;
}

.hero-title {
  font-family: $font-display;
  font-size: clamp(36px, 8vw, 96px);
  font-weight: 700;
  letter-spacing: -0.03em;
  line-height: 1;
  display: flex;
  justify-content: center;
  flex-wrap: wrap;
  position: relative;
}

.letter {
  display: inline-block;
  transform-origin: center bottom;
  transform-style: preserve-3d;

  &-accent {
    color: $color-accent;
  }
}

.cursor {
  animation: blink 1.2s step-end infinite;
  color: $color-accent;
  font-weight: 400;
  margin-left: 4px;
}

@keyframes blink {
  50% { opacity: 0; }
}

.hero-subtitle {
  font-size: clamp(14px, 2vw, 16px);
  line-height: 1.6;
  max-width: 480px;
  opacity: 0;
}

.hero-buttons {
  display: flex;
  gap: 16px;
  margin-top: 16px;
  flex-wrap: wrap;
  justify-content: center;
  opacity: 0;
}

// CTA Button
.cta-button {
  position: relative;
  display: inline-flex;
  align-items: center;
  gap: 12px;
  padding: 16px 32px;
  background: transparent;
  border: 1px solid $color-accent;
  color: $color-accent;
  font-family: $font-body;
  font-size: 14px;
  font-weight: 500;
  letter-spacing: 0.02em;
  text-transform: lowercase;
  cursor: pointer;
  overflow: hidden;
  transition: color 0.4s cubic-bezier(0.23, 1, 0.32, 1);

  &:hover {
    color: $color-bg;

    .button-bg {
      transform: scaleX(1);
    }

    .button-icon {
      transform: translateX(4px);
    }
  }

  &:active {
    transform: scale(0.98);
  }

  &.outline {
    border-color: $color-border;
    color: $color-text;

    &:hover {
      color: $color-bg;
      border-color: $color-text;
    }

    .button-bg {
      background: $color-text;
    }
  }

  &.large {
    padding: 18px 36px;
    font-size: 16px;
  }
}

.button-text {
  position: relative;
  z-index: 2;
}

.button-icon {
  position: relative;
  z-index: 2;
  display: flex;
  transition: transform 0.4s cubic-bezier(0.23, 1, 0.32, 1);
}

.button-bg {
  position: absolute;
  inset: 0;
  background: $color-accent;
  transform: scaleX(0);
  transform-origin: left;
  transition: transform 0.4s cubic-bezier(0.23, 1, 0.32, 1);
  z-index: 1;
}

// Benefits Section
.benefits-section {
  padding: 120px 40px;

  @media (max-width: 768px) {
    padding: 80px 20px;
  }
}

.container {
  max-width: 1200px;
  margin: 0 auto;
}

.section-header {
  margin-bottom: 60px;
}

.section-title {
  font-family: $font-display;
  font-size: clamp(28px, 4vw, 40px);
  font-weight: 600;
  letter-spacing: -0.02em;
}

.benefits-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 32px;
  transform-style: preserve-3d;
  perspective: 1000px;
  padding-bottom: 120px;

  @media (max-width: 768px) {
    grid-template-columns: 1fr;
    gap: 24px;
    padding-bottom: 80px;

    .benefit-card {
      transform: translateY(0) !important;
    }
  }
}

.benefit-card {
  position: relative;
  transform-style: preserve-3d;
  perspective: 800px;
  opacity: 0;

  @media (max-width: 768px) {
    transform: translateY(0) !important;
  }

  &:hover {
    .card-glow {
      opacity: 1;
    }

    .card-inner {
      border-color: rgba($color-accent, 0.3);
      transform: translateZ(20px) rotateX(5deg);
    }

    .benefit-icon {
      color: $color-accent;
      transform: scale(1.1);
    }
  }
}

.card-inner {
  position: relative;
  padding: 40px;
  background: rgba($color-surface, 0.5);
  border: 1px solid $color-border;
  border-radius: 8px;
  transition: all 0.4s cubic-bezier(0.23, 1, 0.32, 1);
  transform-style: preserve-3d;
  backdrop-filter: blur(10px);
}

.card-glow {
  position: absolute;
  inset: -1px;
  background: linear-gradient(135deg,
    rgba($color-accent, 0.15) 0%,
    transparent 50%,
    rgba($color-secondary, 0.1) 100%
  );
  border-radius: 8px;
  opacity: 0;
  transition: opacity 0.4s ease;
  pointer-events: none;
  z-index: -1;
}

.benefit-number {
  position: absolute;
  top: 20px;
  left: 20px;
  font-size: 12px;
  color: $color-text-muted;
  letter-spacing: 0.1em;
}

.benefit-icon {
  width: 64px;
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-top: 20px;
  margin-bottom: 24px;
  color: $color-text-dim;
  transition: all 0.3s ease;
}

.benefit-title {
  font-family: $font-display;
  font-size: 24px;
  font-weight: 600;
  margin-bottom: 12px;
  line-height: 1.2;

  @media (max-width: 768px) {
    font-size: 20px;
  }
}

.benefit-description {
  font-size: 15px;
  line-height: 1.6;
  color: $color-text-dim;
}

.benefit-indicator {
  position: absolute;
  bottom: 20px;
  right: 20px;
  display: flex;
  align-items: center;
  gap: 8px;

  .indicator-line {
    width: 40px;
    height: 1px;
    background: linear-gradient(90deg, $color-accent, transparent);
  }

  .indicator-dot {
    width: 6px;
    height: 6px;
    background: $color-accent;
    border-radius: 50%;
    animation: pulse 2s ease-in-out infinite;
  }
}

@keyframes pulse {
  0%, 100% { opacity: 1; transform: scale(1); }
  50% { opacity: 0.5; transform: scale(0.8); }
}

// Steps Section
.steps-section {
  padding: 120px 40px;
  background: linear-gradient(180deg, transparent 0%, rgba($color-accent, 0.02) 50%, transparent 100%);

  @media (max-width: 768px) {
    padding: 80px 20px;
  }
}

.steps-list {
  display: flex;
  flex-direction: column;
  gap: 0;
  max-width: 800px;
  margin: 0 auto;
}

.step-item {
  display: flex;
  align-items: center;
  gap: 40px;
  padding: 40px 0;
  position: relative;

  @media (max-width: 768px) {
    flex-direction: column !important;
    text-align: center;
    gap: 16px;
  }

  &.step-reverse {
    flex-direction: row-reverse;

    .step-content {
      text-align: right;

      @media (max-width: 768px) {
        text-align: center;
      }
    }
  }
}

.step-content {
  flex: 1;
}

.step-number {
  display: block;
  font-size: 12px;
  color: $color-accent;
  margin-bottom: 8px;
  letter-spacing: 0.1em;
}

.step-title {
  font-family: $font-display;
  font-size: 24px;
  font-weight: 600;
  margin-bottom: 8px;
  letter-spacing: -0.01em;
}

.step-desc {
  font-size: 14px;
  line-height: 1.5;
}

.step-line {
  width: 2px;
  height: 100%;
  min-height: 80px;
  background: linear-gradient(180deg, $color-accent 0%, $color-accent 50%, transparent 100%);
  opacity: 0.3;

  @media (max-width: 768px) {
    width: 60px;
    height: 2px;
    min-height: auto;
    background: linear-gradient(90deg, $color-accent 0%, $color-accent 50%, transparent 100%);
  }
}

.step-connector {
  position: absolute;
  right: 0;
  top: 50%;
  transform: translateY(-50%);

  @media (max-width: 768px) {
    position: relative;
    right: auto;
    top: auto;
    transform: none;
  }
}

.step-check {
  color: $color-accent;
}

// CTA Section
.cta-section {
  padding: 120px 40px;
  text-align: center;

  @media (max-width: 768px) {
    padding: 80px 20px;
  }
}

.cta-content {
  max-width: 600px;
  margin: 0 auto;
}

.cta-title {
  font-family: $font-display;
  font-size: clamp(28px, 4vw, 40px);
  font-weight: 600;
  margin-bottom: 16px;
  letter-spacing: -0.02em;
}

.cta-text {
  font-size: 16px;
  line-height: 1.6;
  margin-bottom: 32px;
}

// Utilities
.mono {
  font-family: $font-mono;
}

.text-dim {
  color: $color-text-dim;
}
</style>
