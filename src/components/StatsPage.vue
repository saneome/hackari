<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { gsap } from 'gsap'
import { ScrollTrigger } from 'gsap/ScrollTrigger'
import { TrendingUp, Users, Trophy, Clock } from 'lucide-vue-next'

const sectionRef = ref<HTMLElement | null>(null)
const statsRef = ref<HTMLDivElement | null>(null)
const chartRef = ref<HTMLDivElement | null>(null)
const headerRef = ref<HTMLElement | null>(null)
const statsAnimated = ref(false)
const hoveredSegment = ref<number | null>(null)

const stats = [
  { icon: TrendingUp, value: 150, suffix: '+', label: 'проведённых хакатонов', prefix: '' },
  { icon: Users, value: 12000, suffix: '', label: 'зарегистрированных участников', prefix: '' },
  { icon: Trophy, value: 4500, suffix: '+', label: 'сформированных команд', prefix: '' },
  { icon: Clock, value: 98, suffix: '%', label: 'успешных завершений', prefix: '' },
]

const competencies = [
  { name: 'Frontend', value: 34 },
  { name: 'Backend', value: 28 },
  { name: 'ML / AI', value: 15 },
  { name: 'Product', value: 12 },
  { name: 'Design / UX', value: 11 },
]

// CountUp animation
const animateCounter = (element: HTMLElement, target: number, duration: number = 2) => {
  const obj = { value: 0 }
  gsap.to(obj, {
    value: target,
    duration,
    ease: 'power2.out',
    onUpdate: () => {
      element.textContent = Math.round(obj.value).toLocaleString()
    },
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

  // Stats counter animation
  if (statsRef.value) {
    const statValues = statsRef.value.querySelectorAll('.stat-value')

    ScrollTrigger.create({
      trigger: statsRef.value,
      start: 'top 70%',
      onEnter: () => {
        if (!statsAnimated.value) {
          statsAnimated.value = true
          statValues.forEach((el, index) => {
            animateCounter(el as HTMLElement, stats[index].value)
          })
        }
      },
    })

    // Cards entrance
    const cards = statsRef.value.querySelectorAll('.stat-card')
    gsap.fromTo(cards,
      { y: 60, opacity: 0 },
      {
        y: 0,
        opacity: 1,
        duration: 0.8,
        stagger: 0.15,
        ease: 'power3.out',
        scrollTrigger: {
          trigger: statsRef.value,
          start: 'top 75%',
          toggleActions: 'play none none reverse',
        },
      }
    )
  }

    // Chart animation
    if (chartRef.value) {
      gsap.fromTo(chartRef.value,
        { scale: 0.9, opacity: 0 },
        {
          scale: 1,
          opacity: 1,
          duration: 1,
          ease: 'power3.out',
          scrollTrigger: {
            trigger: chartRef.value,
            start: 'top 75%',
            toggleActions: 'play none none reverse',
          },
        }
      )

      // Animate chart segments - draw each arc progressively
      const segments = chartRef.value.querySelectorAll('.chart-segment')
      segments.forEach((segment, index) => {
        const path = segment as SVGPathElement
        const length = path.getTotalLength()

        gsap.fromTo(path,
          { strokeDashoffset: length },
          {
            strokeDashoffset: 0,
            duration: 0.8,
            delay: index * 0.15,
            ease: 'power2.out',
            scrollTrigger: {
              trigger: chartRef.value,
              start: 'top 70%',
              toggleActions: 'play none none reverse',
            },
          }
        )
      })
    }
})

// Calculate SVG arc path and animation props
const getArcProps = (value: number, offset: number, total: number) => {
  const radius = 80
  const startAngle = (offset / total) * 360 - 90
  const endAngle = ((offset + value) / total) * 360 - 90

  const toRad = (deg: number) => (deg * Math.PI) / 180

  const x1 = radius * Math.cos(toRad(startAngle))
  const y1 = radius * Math.sin(toRad(startAngle))
  const x2 = radius * Math.cos(toRad(endAngle))
  const y2 = radius * Math.sin(toRad(endAngle))

  const largeArc = value > total / 2 ? 1 : 0

  // Calculate arc length for animation
  const angleDiff = ((value / total) * 360 * Math.PI) / 180
  const arcLength = radius * angleDiff

  const d = `M ${x1} ${y1} A ${radius} ${radius} 0 ${largeArc} 1 ${x2} ${y2}`

  return { d, arcLength }
}

const totalValue = competencies.reduce((sum, c) => sum + c.value, 0)
const colors = ['#D4FF00', '#FF5722', '#00E5FF', '#7C3AED', '#F59E0B']
</script>

<template>
  <section ref="sectionRef" class="stats-page">
    <div class="container">
      <header ref="headerRef" class="section-header">
        <span class="section-label header-animate mono">ЦИФРЫ</span>
        <h2 class="section-title header-animate">
          на пути реальности
        </h2>
        <p class="section-subtitle header-animate text-dim">
          Каждый показатель — это реальный человек, команда или событие
        </p>
      </header>

      <div class="stats-content">
        <!-- Stats Grid -->
        <div ref="statsRef" class="stats-grid">
          <article
            v-for="stat in stats"
            :key="stat.label"
            class="stat-card"
          >
            <div class="stat-icon">
              <component :is="stat.icon" :size="24" stroke-width="1.5" />
            </div>
            <div class="stat-data">
              <span class="stat-value text-dim">0</span>
              <span class="stat-suffix">{{ stat.suffix }}</span>
            </div>
            <span class="stat-label text-dim">{{ stat.label }}</span>
            <div class="stat-line" />
          </article>
        </div>

        <!-- Competency Chart -->
        <div ref="chartRef" class="chart-container">
          <div class="chart-wrapper">
            <svg viewBox="0 0 200 200" class="donut-chart">
              <defs>
                <filter id="glow">
                  <feGaussianBlur stdDeviation="3" result="coloredBlur"/>
                  <feMerge>
                    <feMergeNode in="coloredBlur"/>
                    <feMergeNode in="SourceGraphic"/>
                  </feMerge>
                </filter>
              </defs>

            <g transform="translate(100, 100) rotate(-90)">
              <!-- Background circle -->
              <circle
                r="80"
                fill="none"
                stroke="rgba(255,255,255,0.05)"
                stroke-width="6"
              />

              <!-- Segments as paths with dash animation -->
              <path
                v-for="(comp, index) in competencies"
                :key="comp.name"
                class="chart-segment"
                fill="none"
                :stroke="colors[index]"
                stroke-width="6"
                stroke-linecap="round"
                :d="getArcProps(comp.value, competencies.slice(0, index).reduce((sum, c) => sum + c.value, 0), totalValue).d"
                :stroke-dasharray="getArcProps(comp.value, competencies.slice(0, index).reduce((sum, c) => sum + c.value, 0), totalValue).arcLength"
                :stroke-dashoffset="getArcProps(comp.value, competencies.slice(0, index).reduce((sum, c) => sum + c.value, 0), totalValue).arcLength"
                :style="{ filter: hoveredSegment === index ? 'url(#glow) brightness(1.4)' : 'url(#glow)' }"
                :data-index="index"
                @mouseenter="hoveredSegment = index"
                @mouseleave="hoveredSegment = null"
              />
            </g>

              <!-- Center text -->
              <text
                x="100"
                y="95"
                text-anchor="middle"
                fill="#D4FF00"
                font-family="Onest, sans-serif"
                font-size="14"
                font-weight="500"
              >
                компетенции
              </text>
              <text
                x="100"
                y="115"
                text-anchor="middle"
                fill="rgba(255,255,255,0.5)"
                font-family="Onest, sans-serif"
                font-size="11"
              >
                распределение
              </text>
            </svg>
          </div>

            <!-- Legend -->
            <div class="chart-legend">
              <div
                v-for="(comp, index) in competencies"
                :key="comp.name"
                class="legend-item"
                :class="{ 'legend-item--active': hoveredSegment === index }"
                @mouseenter="hoveredSegment = index"
                @mouseleave="hoveredSegment = null"
              >
                <div
                  class="legend-dot"
                  :class="{ 'legend-dot--active': hoveredSegment === index }"
                  :style="{ backgroundColor: colors[index] }"
                />
                <span class="legend-name">{{ comp.name }}</span>
                <span class="legend-value">{{ comp.value }}%</span>
              </div>
            </div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped lang="scss">
.stats-page {
  position: relative;
  min-height: 100vh;
  padding: 120px 0;
  background: linear-gradient(180deg, #0c0c0c 0%, #0a0a0a 100%);
  overflow: hidden;
}

.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 40px;

  @media (max-width: 768px) {
    padding: 0 20px;
  }
}

// Header
.section-header {
  text-align: center;
  margin-bottom: 80px;
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
  max-width: 480px;
  margin: 0 auto;
}

// Content layout
.stats-content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 64px;
  align-items: center;

  @media (max-width: 968px) {
    grid-template-columns: 1fr;
    gap: 48px;
  }
}

// Stats Grid
.stats-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 24px;

  @media (max-width: 640px) {
    grid-template-columns: 1fr;
  }
}

.stat-card {
  position: relative;
  padding: 28px;
  background: rgba($color-surface, 0.3);
  border: 1px solid $color-border;
  border-radius: 12px;
  overflow: hidden;
  transition: all 0.3s ease;

  &:hover {
    border-color: rgba($color-accent, 0.2);
    transform: translateY(-4px);

    .stat-line {
      transform: scaleX(1);
    }
  }
}

.stat-icon {
  color: $color-accent;
  margin-bottom: 16px;
}

.stat-data {
  display: flex;
  align-items: baseline;
  gap: 4px;
  margin-bottom: 8px;
}

.stat-value {
  font-family: $font-display;
  font-size: 42px;
  font-weight: 700;
  line-height: 1;
  color: $color-text;
}

.stat-suffix {
  font-family: $font-display;
  font-size: 24px;
  color: $color-accent;
}

.stat-label {
  font-size: 13px;
  display: block;
}

.stat-line {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: linear-gradient(90deg, $color-accent, transparent);
  transform: scaleX(0);
  transform-origin: left;
  transition: transform 0.4s ease;
}

// Chart
.chart-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 40px;
}

.chart-wrapper {
  width: 280px;
  height: 280px;
}

.donut-chart {
  width: 100%;
  height: 100%;
}

.chart-segment {
  transition: all 0.3s ease;

  &:hover {
    stroke-width: 10;
    filter: url(#glow) brightness(1.3) !important;
  }
}

// Legend
.chart-legend {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 16px 32px;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 10px;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.3s ease;
  border: 1px solid transparent;

  &--active,
  &:hover {
    background: rgba($color-surface, 0.5);
    border-color: rgba($color-accent, 0.2);

    .legend-name {
      color: $color-text;
    }

    .legend-value {
      color: $color-accent;
    }
  }
}

.legend-dot {
  width: 8px;
  height: 8px;
  border-radius: 2px;
  transition: all 0.3s ease;

  &--active {
    transform: scale(1.3);
    box-shadow: 0 0 8px currentColor;
  }
}

.legend-name {
  font-size: 14px;
  color: $color-text-dim;
  transition: color 0.3s ease;
}

.legend-value {
  font-size: 13px;
  font-weight: 500;
  color: $color-text;
  transition: color 0.3s ease;
}
</style>
