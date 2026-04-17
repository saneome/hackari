<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { gsap } from 'gsap'
import {
  Users,
  Code,
  Trophy,
} from 'lucide-vue-next'

const sectionRef = ref<HTMLElement | null>(null)
const stepsRef = ref<HTMLDivElement | null>(null)
const headerRef = ref<HTMLElement | null>(null)
const connectorRef = ref<SVGSVGElement | null>(null)

const steps = [
  {
    id: 1,
    number: '01',
    icon: Users,
    title: 'пригласить команду',
    description: 'Создайте хакатон, задайте тематику и формат. Платформа автоматически подберёт участников по компетенциям и распределит в команды оптимально.',
  },
  {
    id: 2,
    number: '02',
    icon: Code,
    title: 'код и дедлайны',
    description: 'Отслеживайте прогресс в реальном времени. Дашборд показывает кто что делает, где затыки и кто нуждается в поддержке менторов прямо сейчас.',
  },
  {
    id: 3,
    number: '03',
    icon: Trophy,
    title: 'оценка итогов',
    description: 'Многофакторная система оценки учитывает не только код, но и презентацию, командную работу и распределение задач. Прозрачно и справедливо.',
  },
]

onMounted(() => {
  // Header animation
  if (headerRef.value) {
    const headerElements = headerRef.value.querySelectorAll('.header-animate')
    gsap.fromTo(headerElements,
      { y: 50, opacity: 0 },
      {
        y: 0,
        opacity: 1,
        duration: 0.8,
        stagger: 0.1,
        ease: 'power3.out',
        scrollTrigger: {
          trigger: headerRef.value,
          start: 'top 80%',
          toggleActions: 'play none none reverse',
        },
      }
    )
  }

  // Steps animation with 3D card flip
  const stepCards = stepsRef.value?.querySelectorAll('.step-card')
  stepCards?.forEach((card, index) => {
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

  // Connector line animation
  if (connectorRef.value) {
    const path = connectorRef.value.querySelector('.connector-path')
    if (path) {
      const length = (path as SVGPathElement).getTotalLength()
      gsap.set(path, { strokeDasharray: length, strokeDashoffset: length })

      gsap.to(path, {
        strokeDashoffset: 0,
        duration: 2,
        ease: 'power2.inOut',
        scrollTrigger: {
          trigger: stepsRef.value,
          start: 'top 60%',
          end: 'bottom 60%',
          scrub: 1,
        },
      })
    }
  }
})
</script>

<template>
  <section ref="sectionRef" class="how-it-works-page">
    <!-- SVG Connector -->
    <svg ref="connectorRef" class="connector" viewBox="0 0 100 100" preserveAspectRatio="none">
      <path
        class="connector-path"
        d="M 10 20 Q 50 20 50 50 Q 50 80 90 80"
        fill="none"
        stroke="url(#gradient)"
        stroke-width="0.5"
      />
      <defs>
        <linearGradient id="gradient" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="#D4FF00" />
          <stop offset="100%" stop-color="#FF5722" />
        </linearGradient>
      </defs>
    </svg>

    <div class="container">
      <header ref="headerRef" class="section-header">
        <span class="section-label header-animate mono">ПРОЦЕСС</span>
        <h2 class="section-title header-animate">
          как это <span class="accent">работает</span>
        </h2>
        <p class="section-subtitle header-animate text-dim">
          Три шага от идеи до проведённого хакатона
        </p>
      </header>

      <div ref="stepsRef" class="steps-grid">
        <article
          v-for="step in steps"
          :key="step.id"
          class="step-card"
          :class="`step-${step.id}`"
        >
          <div class="card-inner">
            <div class="step-number mono">{{ step.number }}</div>

            <div class="step-icon">
              <component :is="step.icon" :size="32" stroke-width="1.5" />
            </div>

            <h3 class="step-title">{{ step.title }}</h3>
            <p class="step-description text-dim">{{ step.description }}</p>

            <div class="step-indicator">
              <div class="indicator-line" />
              <div class="indicator-dot" />
            </div>
          </div>

          <!-- Hover effect layers -->
          <div class="card-glow" />
        </article>
      </div>
    </div>
  </section>
</template>

<style scoped lang="scss">
.how-it-works-page {
  position: relative;
  min-height: 100vh;
  padding: 120px 0;
  background: linear-gradient(180deg, #111111 0%, #0a0a0a 100%);
  overflow: hidden;
}

.connector {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
  opacity: 0.4;
  pointer-events: none;
  z-index: 0;
}

.container {
  max-width: 1400px;
  margin: 0 auto;
  padding: 0 40px;
  position: relative;
  z-index: 1;

  @media (max-width: 768px) {
    padding: 0 20px;
  }
}

// Header
.section-header {
  text-align: center;
  margin-bottom: 80px;

  @media (max-width: 768px) {
    margin-bottom: 60px;
  }
}

.section-label {
  display: inline-block;
  font-size: 12px;
  letter-spacing: 0.2em;
  color: $color-text-dim;
  margin-bottom: 16px;
}

.section-title {
  font-size: clamp(36px, 5vw, 64px);
  font-weight: 600;
  line-height: 1.1;
  margin-bottom: 16px;
}

.section-subtitle {
  font-size: 18px;
  max-width: 400px;
  margin: 0 auto;
}

// Steps Grid
.steps-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 32px;
  transform-style: preserve-3d;
  perspective: 1000px;

  @media (max-width: 1024px) {
    grid-template-columns: 1fr;
    max-width: 500px;
    margin: 0 auto;
  }
}

// Step Card
.step-card {
  position: relative;
  transform-style: preserve-3d;
  perspective: 800px;

  &:nth-child(1) { transform: translateY(0); }
  &:nth-child(2) { transform: translateY(40px); }
  &:nth-child(3) { transform: translateY(80px); }

  @media (max-width: 1024px) {
    &:nth-child(1),
    &:nth-child(2),
    &:nth-child(3) {
      transform: translateY(0);
    }
  }

  &:hover {
    .card-glow {
      opacity: 1;
    }

    .card-inner {
      border-color: rgba($color-accent, 0.3);
      transform: translateZ(20px) rotateX(5deg);
    }

    .step-icon {
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

.step-number {
  position: absolute;
  top: 20px;
  left: 20px;
  font-size: 12px;
  color: $color-text-muted;
  letter-spacing: 0.1em;
}

.step-icon {
  width: 64px;
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 24px;
  color: $color-text-dim;
  transition: all 0.3s ease;
}

.step-title {
  font-size: 24px;
  font-weight: 600;
  margin-bottom: 12px;
  line-height: 1.2;

  @media (max-width: 768px) {
    font-size: 20px;
  }
}

.step-description {
  font-size: 15px;
  line-height: 1.6;
  color: $color-text-dim;
}

.step-indicator {
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
</style>
