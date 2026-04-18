<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { gsap } from 'gsap'
import {
  Plus,
  Users,
  BarChart3,
  Monitor,
  MessageSquare,
  Award,
  type LucideIcon,
} from 'lucide-vue-next'
import type { Feature } from '@/types'

const sectionRef = ref<HTMLElement | null>(null)
const gridRef = ref<HTMLDivElement | null>(null)
const headerRef = ref<HTMLElement | null>(null)

interface FeatureItem extends Omit<Feature, 'icon'> {
  icon: LucideIcon
}

const features: FeatureItem[] = [
  {
    id: 1,
    icon: Plus,
    title: 'создание хакатона',
    description: 'Настройте событие за 5 минут: даты, треки, критерии оценки, призовой фонд.',
    size: 'large',
  },
  {
    id: 2,
    icon: Users,
    title: 'формирование команд',
    description: 'Автоматический подбор по навыкам или ручной режим.',
    size: 'small',
  },
  {
    id: 3,
    icon: BarChart3,
    title: 'трекинг компетенций',
    description: 'Видите, кто чем владеет и кого не хватает.',
    size: 'medium',
  },
  {
    id: 4,
    icon: Monitor,
    title: 'лайв-дашборд',
    description: 'Реальное время, реальные цифры. Кто коммитит, кто залип.',
    size: 'medium',
  },
  {
    id: 5,
    icon: MessageSquare,
    title: 'менторинг',
    description: 'Интегрированные чаты, видеозвонки, shared notes.',
    size: 'small',
  },
  {
    id: 6,
    icon: Award,
    title: 'оценка проектов',
    description: 'Матрица оценки по критериям с весами. Прозрачная таблица результатов.',
    size: 'large',
  },
]

// 3D Tilt effect handler
const handleMouseMove = (e: MouseEvent, card: HTMLElement) => {
  const rect = card.getBoundingClientRect()
  const x = e.clientX - rect.left
  const y = e.clientY - rect.top

  const centerX = rect.width / 2
  const centerY = rect.height / 2

  const rotateX = (y - centerY) / 10
  const rotateY = (centerX - x) / 10

  gsap.to(card, {
    rotateX: rotateX,
    rotateY: rotateY,
    transformPerspective: 1000,
    duration: 0.5,
    ease: 'power2.out',
  })
}

const handleMouseLeave = (card: HTMLElement) => {
  gsap.to(card, {
    rotateX: 0,
    rotateY: 0,
    duration: 0.5,
    ease: 'power2.out',
  })
}

onMounted(() => {
  // Header animation
  if (headerRef.value) {
    const elements = headerRef.value.querySelectorAll('.header-animate')
    gsap.fromTo(elements,
      { y: 40, opacity: 0 },
      {
        y: 0,
        opacity: 1,
        duration: 0.8,
        stagger: 0.1,
        ease: 'power3.out',
        scrollTrigger: {
          trigger: headerRef.value,
          start: 'top 85%',
          toggleActions: 'play none none reverse',
        },
      }
    )
  }

  // Bento cards animation
  const cards = gridRef.value?.querySelectorAll('.bento-card')
  cards?.forEach((card, index) => {
    gsap.fromTo(card,
      {
        scale: 0.9,
        opacity: 0,
        y: 40,
      },
      {
        scale: 1,
        opacity: 1,
        y: 0,
        duration: 0.8,
        ease: 'power3.out',
        scrollTrigger: {
          trigger: card,
          start: 'top 85%',
          toggleActions: 'play none none reverse',
        },
        delay: (index % 3) * 0.1,
      }
    )
  })
})
</script>

<template>
  <section ref="sectionRef" class="features-page">
    <div class="container">
      <header ref="headerRef" class="section-header">
        <span class="section-label header-animate mono">ФУНКЦИОНАЛ</span>
        <h2 class="section-title header-animate">
          всё, что нужно
        </h2>
        <p class="section-subtitle header-animate text-dim">
          Платформа, которая покрывает полный цикл хакатона
        </p>
      </header>

      <div ref="gridRef" class="bento-grid">
        <article
          v-for="feature in features"
          :key="feature.id"
          ref="cardElements"
          class="bento-card"
          :class="`size-${feature.size}`"
          @mousemove="(e) => handleMouseMove(e as MouseEvent, e.currentTarget as HTMLElement)"
          @mouseleave="(e) => handleMouseLeave(e.currentTarget as HTMLElement)"
        >
          <div class="card-content">
            <div class="card-icon">
              <component :is="feature.icon" :size="28" stroke-width="1.5" />
            </div>

            <div class="card-text">
              <h3 class="card-title">{{ feature.title }}</h3>
              <p class="card-description text-dim">{{ feature.description }}</p>
            </div>
      </div>

          <div class="card-highlight" />
          <div class="card-border" />
        </article>
      </div>
    </div>
  </section>
</template>

<style scoped lang="scss">
.features-page {
  position: relative;
  min-height: 100vh;
  padding: 120px 0;
  background: linear-gradient(180deg, #0a0a0a 0%, #0c0c0c 100%);
}

.container {
  max-width: 1200px;
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
  margin-bottom: 64px;
  text-align: center;
}

.section-label {
  display: inline-block;
  font-size: 12px;
  letter-spacing: 0.2em;
  color: $color-text-dim;
  margin-bottom: 16px;
}

.section-title {
  font-size: clamp(36px, 5vw, 56px);
  font-weight: 600;
  line-height: 1.1;
  margin-bottom: 16px;
}

.section-subtitle {
  font-size: 18px;
  max-width: 400px;
  margin: 0 auto;
}

// Bento Grid
.bento-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  grid-auto-rows: minmax(200px, auto);
  gap: 16px;

  @media (max-width: 1024px) {
    grid-template-columns: repeat(2, 1fr);
  }

  @media (max-width: 640px) {
    grid-template-columns: 1fr;
  }
}

// Card sizes
.bento-card {
  transform-style: preserve-3d;
  perspective: 800px;
  cursor: pointer;

  &.size-small {
    grid-column: span 1;

    @media (max-width: 1024px) {
      grid-column: span 1;
    }

    @media (max-width: 640px) {
      grid-column: span 1;
    }
  }

  &.size-medium {
    grid-column: span 2;

    @media (max-width: 640px) {
      grid-column: span 1;
    }
  }

  &.size-large {
    grid-column: span 2;
    grid-row: span 2;

    @media (max-width: 1024px) {
      grid-column: span 2;
      grid-row: span 1;
    }

    @media (max-width: 640px) {
      grid-column: span 1;
    }
  }

  &:hover {
    .card-highlight {
      opacity: 1;
    }

    .card-border {
      border-color: rgba($color-accent, 0.3);
    }
  }
}

.card-content {
  position: relative;
  height: 100%;
  padding: 28px;
  background: rgba($color-surface, 0.4);
  border: 1px solid $color-border;
  border-radius: 12px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  z-index: 2;
  transition: transform 0.3s ease;

  @media (max-width: 768px) {
    padding: 20px;
  }
}

.card-highlight {
  position: absolute;
  inset: 0;
  background: radial-gradient(
    circle at 50% 50%,
    rgba($color-accent, 0.08) 0%,
    transparent 70%
  );
  border-radius: 12px;
  opacity: 0;
  transition: opacity 0.4s ease;
  pointer-events: none;
  z-index: 1;
}

.card-border {
  position: absolute;
  inset: 0;
  border: 1px solid transparent;
  border-radius: 12px;
  transition: border-color 0.3s ease;
  pointer-events: none;
  z-index: 3;
}

.card-icon {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: $color-text-dim;
  transition: all 0.3s ease;

  :deep(svg) {
    width: 28px;
    height: 28px;
  }
}

.card-text {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.card-title {
  font-size: 20px;
  font-weight: 600;
  line-height: 1.2;
}

.card-description {
  font-size: 14px;
  line-height: 1.5;
  max-width: 320px;
}

// Specific card styles for varied heights
.bento-card.size-large {
  .card-content {
    justify-content: flex-end;
  }

  .card-text {
    margin-top: auto;
  }
}
</style>
