<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { gsap } from 'gsap'
import { ScrollTrigger } from 'gsap/ScrollTrigger'
import { Calendar, MapPin, Users, ArrowRight, Globe, Building2, Trophy } from 'lucide-vue-next'
import { hackathonApi, type Hackathon } from '@/services/api'

gsap.registerPlugin(ScrollTrigger)

const sectionRef = ref<HTMLElement | null>(null)
const titleWrapper = ref<HTMLElement | null>(null)
const geometricShape = ref<HTMLElement | null>(null)
const hackathons = ref<Hackathon[]>([])
const isLoading = ref(true)
const error = ref<string | null>(null)
const filter = ref<'all' | 'online' | 'offline'>('all')

const filteredHackathons = computed(() => {
  if (filter.value === 'all') return hackathons.value
  return hackathons.value.filter(h => h.location_type === filter.value)
})

const fetchHackathons = async () => {
  isLoading.value = true
  error.value = null

  const response = await hackathonApi.list()

  if (response.error) {
    error.value = response.error
  } else if (response.data) {
    hackathons.value = response.data.hackathons
  }

  isLoading.value = false
}

onMounted(async () => {
  await fetchHackathons()
  animateHero()
})

const animateHero = () => {
  // Animate title
  const letters = titleWrapper.value?.querySelectorAll('.letter')
  if (letters && letters.length > 0) {
    gsap.fromTo(letters,
      { y: 80, opacity: 0, rotateX: -90 },
      {
        y: 0,
        opacity: 1,
        rotateX: 0,
        duration: 1,
        stagger: 0.05,
        ease: 'power4.out',
        delay: 0.2,
      }
    )
  }

  // Geometric shape
  if (geometricShape.value) {
    gsap.fromTo(geometricShape.value,
      { scale: 0, opacity: 0 },
      {
        scale: 1,
        opacity: 0.5,
        duration: 1.5,
        ease: 'back.out(1.7)',
        delay: 0.4,
      }
    )
  }

  // Hero icon
  const heroIcon = document.querySelector('.hero-icon')
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

  // Hero subtitle
  const heroSubtitle = document.querySelector('.hero-subtitle')
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

  // Filter buttons
  const filterSection = document.querySelector('.filter-section')
  if (filterSection) {
    gsap.fromTo(filterSection,
      { y: 30, opacity: 0 },
      {
        y: 0,
        opacity: 1,
        duration: 0.6,
        ease: 'power3.out',
        delay: 1.4,
      }
    )
  }

  // Animate cards
  const cards = sectionRef.value?.querySelectorAll('.hackathon-card')
  if (cards && cards.length > 0) {
    gsap.fromTo(cards,
      { y: 60, opacity: 0 },
      {
        y: 0,
        opacity: 1,
        duration: 0.8,
        stagger: 0.1,
        ease: 'power3.out',
        delay: 0.6,
      }
    )
  }
}

const formatDate = (dateStr: string) => {
  const date = new Date(dateStr)
  return new Intl.DateTimeFormat('ru-RU', {
    month: 'short',
    day: 'numeric',
  }).format(date)
}

const formatDateRange = (start: string, end: string) => {
  const startDate = new Date(start)
  const endDate = new Date(end)
  const sameMonth = startDate.getMonth() === endDate.getMonth()

  if (sameMonth) {
    return `${startDate.getDate()}-${endDate.getDate()} ${new Intl.DateTimeFormat('ru-RU', { month: 'short' }).format(endDate)}`
  }
  return `${formatDate(start)} - ${formatDate(end)}`
}

const getLocationIcon = (type: string) => {
  return type === 'online' ? Globe : Building2
}

const getLocationLabel = (type: string) => {
  return type === 'online' ? 'онлайн' : 'офлайн'
}
</script>

<template>
  <section ref="sectionRef" class="hackathons-page">
    <!-- Hero Section -->
    <section class="hero-section">
      <div class="hero-content">
        <!-- Geometric Decoration -->
        <div ref="geometricShape" class="geometric-shape">
          <div class="shape-inner">
            <div class="face face-1" />
            <div class="face face-2" />
            <div class="face face-3" />
            <div class="face face-4" />
            <div class="edges">
              <div v-for="i in 4" :key="i" class="edge" />
            </div>
          </div>
        </div>

        <!-- Trophy Icon -->
        <div class="hero-icon-wrapper">
          <div class="hero-icon">
            <Trophy :size="48" />
          </div>
        </div>

        <!-- Title -->
        <div ref="titleWrapper" class="title-wrapper">
          <h1 class="hero-title">
            <span class="letter">х</span>
            <span class="letter">а</span>
            <span class="letter">к</span>
            <span class="letter letter-accent">а</span>
            <span class="letter letter-accent">т</span>
            <span class="letter letter-accent">о</span>
            <span class="letter letter-accent">н</span>
            <span class="letter letter-accent">ы</span>
            <span class="cursor cursor-blink">_</span>
          </h1>
        </div>

        <p class="hero-subtitle">
          <span class="mono text-dim">// найди событие по душе и собери команду мечты</span>
        </p>

        <!-- Filters -->
        <div class="filter-section">
          <span class="filter-label mono">формат:</span>
          <div class="filter-buttons">
            <button
              class="filter-btn"
              :class="{ active: filter === 'all' }"
              @click="filter = 'all'"
            >
              <span>все</span>
            </button>
            <button
              class="filter-btn"
              :class="{ active: filter === 'online' }"
              @click="filter = 'online'"
            >
              <Globe :size="14" />
              <span>онлайн</span>
            </button>
            <button
              class="filter-btn"
              :class="{ active: filter === 'offline' }"
              @click="filter = 'offline'"
            >
              <MapPin :size="14" />
              <span>офлайн</span>
            </button>
          </div>
        </div>
      </div>
    </section>

    <!-- Content Section -->
    <section class="content-section">
      <div class="container">
        <!-- Loading State -->
        <div v-if="isLoading" class="loading-state">
          <div class="loading-spinner" />
          <span class="mono">загрузка хакатонов...</span>
        </div>

        <!-- Error State -->
        <div v-else-if="error" class="error-state">
          <div class="error-icon">
            <Calendar :size="48" />
          </div>
          <p class="error-text">{{ error }}</p>
          <button class="retry-btn" @click="fetchHackathons">
            <span>повторить</span>
          </button>
        </div>

        <!-- Empty State -->
        <div v-else-if="filteredHackathons.length === 0" class="empty-state">
          <Trophy :size="64" class="empty-icon" />
          <p class="empty-text">
            {{ filter === 'all' ? 'пока нет активных хакатонов' : 'в этой категории пока ничего нет' }}
          </p>
        </div>

        <!-- Hackathons Grid -->
        <div v-else class="hackathons-grid">
          <article
            v-for="hackathon in filteredHackathons"
            :key="hackathon.id"
            class="hackathon-card"
          >
            <!-- Banner -->
            <div class="card-banner">
              <img
                v-if="hackathon.banner_url"
                :src="hackathon.banner_url"
                :alt="hackathon.title"
                class="banner-image"
              >
              <div v-else class="banner-placeholder">
                <span class="placeholder-text">hackari</span>
              </div>
              <div class="banner-overlay" />
              <span class="location-badge">
                <component :is="getLocationIcon(hackathon.location_type)" :size="12" />
                {{ getLocationLabel(hackathon.location_type) }}
              </span>
            </div>

            <!-- Content -->
            <div class="card-content">
              <h3 class="card-title">{{ hackathon.title }}</h3>

              <div class="card-meta">
                <div class="meta-item">
                  <Calendar :size="14" class="meta-icon" />
                  <span class="meta-text">
                    {{ formatDateRange(hackathon.event_start, hackathon.event_end) }}
                  </span>
                </div>
                <div class="meta-item">
                  <Users :size="14" class="meta-icon" />
                  <span class="meta-text">
                    {{ hackathon.participant_count }} участников
                  </span>
                </div>
              </div>

              <router-link
                :to="`/hackathons/${hackathon.id}`"
                class="card-link"
              >
                <span>подробнее</span>
                <ArrowRight :size="14" />
              </router-link>
            </div>

            <div class="card-highlight" />
            <div class="card-border" />
          </article>
        </div>
      </div>
    </section>
  </section>
</template>

<style scoped lang="scss">
.hackathons-page {
  min-height: 100vh;
  background: linear-gradient(180deg, #0a0a0a 0%, #111111 50%, #0a0a0a 100%);
}

// Hero Section
.hero-section {
  position: relative;
  min-height: 45vh;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 120px 40px 60px;
  overflow: hidden;
}

.hero-content {
  position: relative;
  z-index: 10;
  text-align: center;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
}

// Geometric Shape
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
  width: 80px;
  height: 80px;
  left: 50%;
  top: 50%;
  margin-left: -40px;
  margin-top: -40px;
  background: rgba($color-accent, 0.05);
  border: 1px solid rgba($color-accent, 0.15);

  &.face-1 { transform: translateZ(40px); }
  &.face-2 { transform: rotateY(90deg) translateZ(40px); }
  &.face-3 { transform: translateZ(-40px); }
  &.face-4 { transform: rotateY(-90deg) translateZ(40px); }
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
  background: rgba($color-accent, 0.3);
  transform-origin: center;

  &:nth-child(1) { transform: translateZ(40px); }
  &:nth-child(2) { transform: translateZ(40px) rotateY(90deg); }
  &:nth-child(3) { transform: translateZ(-40px); }
  &:nth-child(4) { transform: translateZ(-40px) rotateY(90deg); }
}

@keyframes float {
  0%, 100% { transform: translateY(0) rotateZ(0deg); }
  50% { transform: translateY(-15px) rotateZ(2deg); }
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
  font-size: clamp(48px, 10vw, 96px);
  font-weight: 700;
  letter-spacing: -0.03em;
  line-height: 1;
  display: flex;
  justify-content: center;
  position: relative;
}

.letter {
  display: inline-block;
  transform-origin: center bottom;
  transform-style: preserve-3d;

  &-accent {
    color: $color-accent;
  }

  &:hover {
    color: $color-accent;
    transition: color 0.3s ease;
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
  font-size: 14px;
  letter-spacing: 0.05em;
  opacity: 0;
}

// Filter Section
.filter-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  margin-top: 16px;
  opacity: 0;
}

.filter-label {
  font-size: 12px;
  color: $color-text-dim;
}

.filter-buttons {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 8px;
}

.filter-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  background: rgba($color-surface, 0.5);
  border: 1px solid $color-border;
  border-radius: 8px;
  color: $color-text-dim;
  font-family: $font-body;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.3s ease;

  &:hover, &.active {
    border-color: $color-accent;
    color: $color-text;
    background: rgba($color-accent, 0.1);
  }
}

// Content Section
.content-section {
  padding: 40px 40px 120px;

  @media (max-width: 768px) {
    padding: 20px 20px 80px;
  }
}

.container {
  max-width: 1200px;
  margin: 0 auto;
}

// States
.loading-state, .error-state, .empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  padding: 80px 20px;
  text-align: center;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 2px solid rgba($color-accent, 0.2);
  border-top-color: $color-accent;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.error-icon {
  color: $color-text-dim;
  opacity: 0.5;
}

.error-text {
  color: $color-text-dim;
  font-size: 16px;
}

.retry-btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  background: transparent;
  border: 1px solid $color-border;
  border-radius: 8px;
  color: $color-text;
  font-family: $font-body;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.3s ease;

  &:hover {
    border-color: $color-accent;
    color: $color-accent;
  }
}

.empty-icon {
  opacity: 0.3;
  color: $color-text-dim;
}

.empty-text {
  font-family: $font-display;
  font-size: 20px;
  font-weight: 600;
  color: $color-text;
}

// Grid
.hackathons-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 24px;
}

// Card
.hackathon-card {
  position: relative;
  transform-style: preserve-3d;
  perspective: 800px;
  transition: all 0.4s $transition-smooth;

  &:hover {
    transform: translateY(-4px);

    .banner-image,
    .banner-placeholder {
      transform: scale(1.05);
    }

    .card-link {
      color: $color-accent;
    }

    .card-highlight {
      opacity: 1;
    }

    .card-border {
      border-color: rgba($color-accent, 0.3);
    }
  }
}

.card-banner {
  position: relative;
  height: 180px;
  overflow: hidden;
  border-radius: 16px 16px 0 0;
}

.banner-image, .banner-placeholder {
  width: 100%;
  height: 100%;
  transition: transform 0.6s $transition-smooth;
}

.banner-image {
  object-fit: cover;
}

.banner-placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, $color-surface 0%, darken($color-surface, 5%) 100%);
}

.placeholder-text {
  font-family: $font-display;
  font-size: 24px;
  font-weight: 600;
  color: rgba($color-text, 0.1);
}

.banner-overlay {
  position: absolute;
  inset: 0;
  background: linear-gradient(180deg, transparent 0%, rgba($color-bg, 0.8) 100%);
}

.location-badge {
  position: absolute;
  top: 12px;
  right: 12px;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: rgba($color-bg, 0.8);
  backdrop-filter: blur(8px);
  border: 1px solid $color-border;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 500;
  color: $color-text;
}

.card-content {
  position: relative;
  padding: 24px;
  background: rgba($color-surface, 0.4);
  border: 1px solid $color-border;
  border-top: none;
  border-radius: 0 0 16px 16px;
  z-index: 2;
}

.card-highlight {
  position: absolute;
  inset: 0;
  background: radial-gradient(circle at 50% 50%, rgba($color-accent, 0.08) 0%, transparent 70%);
  border-radius: 16px;
  opacity: 0;
  transition: opacity 0.4s ease;
  pointer-events: none;
  z-index: 1;
}

.card-border {
  position: absolute;
  inset: 0;
  border: 1px solid transparent;
  border-radius: 16px;
  transition: border-color 0.3s ease;
  pointer-events: none;
  z-index: 3;
}

.card-title {
  font-size: 20px;
  font-weight: 600;
  line-height: 1.3;
  margin-bottom: 16px;
  color: $color-text;
}

.card-meta {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-bottom: 20px;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 10px;
  color: $color-text-dim;
}

.meta-icon {
  color: $color-accent;
  flex-shrink: 0;
}

.meta-text {
  font-size: 14px;
}

.card-link {
  display: flex;
  align-items: center;
  gap: 8px;
  color: $color-text-dim;
  font-size: 14px;
  font-weight: 500;
  text-decoration: none;
  transition: color 0.3s ease;
}

// Utilities
.mono {
  font-family: $font-mono;
}

.text-dim {
  color: $color-text-dim;
}
</style>
