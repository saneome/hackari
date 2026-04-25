<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue'
import Lenis from 'lenis'
import { gsap } from 'gsap'
import { ScrollTrigger } from 'gsap/ScrollTrigger'
import {
  Trophy,
  Users,
  Code2,
  Target,
  Medal,
  Hash,
  BarChart3,
  Crown,
} from 'lucide-vue-next'
import FooterPage from '@/components/FooterPage.vue'
import { teamApi, type TeamCompetencyRating } from '@/services/api'

gsap.registerPlugin(ScrollTrigger)

const lenis = ref<Lenis | null>(null)

const handleLenisRaf = (time: number) => {
  lenis.value?.raf(time * 1000)
}

const ratings = ref<TeamCompetencyRating[]>([])
const isLoading = ref(true)
const isLeaderboardLoading = ref(false)
const error = ref<string | null>(null)
const selectedHackathon = ref<string | null>(null)
const selectedCategory = ref<string | null>(null)
const categories = ref<string[]>([])

// Template refs
const heroSection = ref<HTMLElement | null>(null)
const titleWrapper = ref<HTMLElement | null>(null)
const bentoGrid = ref<HTMLDivElement | null>(null)
const geometricShape = ref<HTMLElement | null>(null)
const leaderboardSection = ref<HTMLElement | null>(null)

// Computed
const uniqueHackathons = computed(() => {
  const hackathons = new Map<string, string>()
  ratings.value.forEach(r => hackathons.set(r.hackathon_id, r.hackathon_name))
  return Array.from(hackathons.entries()).map(([id, name]) => ({ id, name }))
})

const filteredRatings = computed(() => {
  let result = ratings.value
  if (selectedHackathon.value) {
    result = result.filter(r => r.hackathon_id === selectedHackathon.value)
  }
  return result
})

const topThree = computed(() => filteredRatings.value.slice(0, 3))
const restTeams = computed(() => filteredRatings.value.slice(3))

onMounted(async () => {
  // Initialize Lenis smooth scroll
  lenis.value = new Lenis({
    duration: 1.2,
    easing: (t) => Math.min(1, 1.001 - Math.pow(2, -10 * t)),
    orientation: 'vertical',
    gestureOrientation: 'vertical',
    smoothWheel: true,
  })

  lenis.value.on('scroll', ScrollTrigger.update)

  gsap.ticker.add(handleLenisRaf)

  gsap.ticker.lagSmoothing(0)

  await Promise.all([loadRatings(), loadCategories()])
  animateHero()
})

onUnmounted(() => {
  gsap.ticker.remove(handleLenisRaf)
  lenis.value?.destroy()
  lenis.value = null
})

const loadCategories = async () => {
  try {
    const response = await teamApi.getRatingCategories()
    if (response.data) {
      categories.value = response.data
    }
  } catch {
    console.error('Failed to load categories')
  }
}

const loadRatings = async () => {
  try {
    isLeaderboardLoading.value = true
    error.value = null
    const response = await teamApi.getCompetencyRatings(selectedCategory.value || undefined)
    if (response.error) {
      error.value = response.error
    } else if (response.data) {
      ratings.value = response.data
    }
  } catch {
    error.value = 'Не удалось загрузить рейтинг'
  } finally {
    isLoading.value = false
    isLeaderboardLoading.value = false
  }
}


const selectCategory = async (category: string | null) => {
  selectedCategory.value = category
  await loadRatings()
}

// Format category name for display
const formatCategoryName = (category: string): string => {
  const names: Record<string, string> = {
    'frontend': 'Frontend',
    'backend': 'Backend',
    'ml': 'ML / AI',
    'devops': 'DevOps',
    'mobile': 'Mobile',
    'design': 'Design',
    'ai-tools': 'AI Tools',
  }
  return names[category] || category.charAt(0).toUpperCase() + category.slice(1)
}

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
  const filterSections = document.querySelectorAll('.filter-section')
  filterSections.forEach((section, index) => {
    gsap.fromTo(section,
      { y: 30, opacity: 0 },
      {
        y: 0,
        opacity: 1,
        duration: 0.6,
        ease: 'power3.out',
        delay: 1.4 + index * 0.1,
      }
    )
  })

  // Setup leaderboard reveal on scroll
  if (leaderboardSection.value) {
    gsap.set(leaderboardSection.value, { opacity: 0, y: 60 })

    ScrollTrigger.create({
      trigger: heroSection.value,
      start: 'bottom 80%',
      onEnter: () => {
        gsap.to(leaderboardSection.value, {
          opacity: 1,
          y: 0,
          duration: 0.8,
          ease: 'power3.out',
        })

        // Animate rows with stagger
        const rows = leaderboardSection.value?.querySelectorAll('.toplist-row')
        rows?.forEach((row, index) => {
          gsap.fromTo(row,
            { opacity: 0, x: -30 },
            {
              opacity: 1,
              x: 0,
              duration: 0.5,
              ease: 'power3.out',
              delay: 0.2 + index * 0.08,
            }
          )
        })
      },
      once: true,
    })
  }
}

const getRankIcon = (rank: number) => {
  if (rank === 1) return Crown
  if (rank <= 3) return Medal
  return Hash
}

const getRankColor = (rank: number) => {
  if (rank === 1) return 'gold'
  if (rank === 2) return 'silver'
  if (rank === 3) return 'bronze'
  return 'default'
}
</script>

<template>
  <div class="ratings-view">
    <main class="main-content">
      <!-- Loading State (initial load only) -->
      <div v-if="isLoading" class="loading-state">
        <div class="loading-spinner" />
        <span class="mono">загрузка рейтинга...</span>
      </div>

      <!-- Error State -->
      <div v-else-if="error" class="error-state">
        <div class="error-icon">
          <BarChart3 :size="48" />
        </div>
        <p class="error-text">{{ error }}</p>
        <button class="retry-btn" @click="loadRatings">
          <span>повторить</span>
        </button>
      </div>

      <!-- Content -->
      <template v-else>
        <!-- Hero Section -->
        <section ref="heroSection" class="hero-section">
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
                <span class="letter">р</span>
                <span class="letter">е</span>
                <span class="letter">й</span>
                <span class="letter letter-accent">т</span>
                <span class="letter letter-accent">и</span>
                <span class="letter letter-accent">н</span>
                <span class="letter">г</span>
                <span class="cursor cursor-blink">_</span>
              </h1>
            </div>

            <p class="hero-subtitle">
              <span class="mono text-dim">// рейтинг команд по компетенциям</span>
            </p>

      <!-- Category Filter -->
      <div class="filter-section">
        <span class="filter-label mono">компетенция:</span>
        <div class="filter-buttons">
          <button
            class="filter-btn"
            :class="{ active: selectedCategory === null }"
            @click="selectCategory(null)"
          >
            <span>все</span>
          </button>
          <button
            v-for="cat in categories"
            :key="cat"
            class="filter-btn"
            :class="{ active: selectedCategory === cat }"
            @click="selectCategory(cat)"
          >
            <span>{{ formatCategoryName(cat) }}</span>
          </button>
        </div>
      </div>

      <!-- Hackathon Filter -->
      <div v-if="uniqueHackathons.length > 1" class="filter-section">
        <span class="filter-label mono">хакатон:</span>
        <div class="filter-buttons">
          <button
            class="filter-btn"
            :class="{ active: selectedHackathon === null }"
            @click="selectedHackathon = null"
          >
            <span>все</span>
          </button>
          <button
            v-for="h in uniqueHackathons"
            :key="h.id"
            class="filter-btn"
            :class="{ active: selectedHackathon === h.id }"
            @click="selectedHackathon = h.id"
          >
            <span>{{ h.name }}</span>
          </button>
        </div>
      </div>
    </div>
  </section>

  <!-- Leaderboard Section -->
  <section ref="leaderboardSection" class="leaderboard-section">
    <div class="container">
      <!-- Leaderboard Loading State -->
      <div v-if="isLeaderboardLoading" class="leaderboard-loading">
        <div class="loading-spinner" />
        <span class="mono">загрузка...</span>
      </div>

      <!-- Vertical Top List -->
      <div v-else-if="filteredRatings.length > 0" class="toplist-section">
        <div class="toplist-header">
          <span class="header-rank">#</span>
          <span class="header-team">Команда</span>
          <span class="header-stats">Участники / Навыки / Уровень</span>
          <span class="header-score">Очки</span>
        </div>

        <div class="toplist-body">
          <article
            v-for="team in filteredRatings"
            :key="team.team_id"
            class="toplist-row"
            :class="`rank-${team.rank}`"
          >
            <div class="row-rank">
              <div class="rank-indicator" :class="getRankColor(team.rank)">
                <component :is="getRankIcon(team.rank)" :size="16" v-if="team.rank <= 3" />
                <span v-else class="rank-number">{{ team.rank }}</span>
              </div>
            </div>

            <div class="row-team">
              <h3 class="team-name">{{ team.team_name }}</h3>
              <p class="hackathon-name">{{ team.hackathon_name }}</p>
            </div>

            <div class="row-stats">
              <div class="stat-item">
                <Users :size="14" />
                <span>{{ team.member_count }}</span>
              </div>
              <div class="stat-item">
                <Code2 :size="14" />
                <span>{{ team.skills_count }}</span>
              </div>
              <div class="stat-item">
                <Target :size="14" />
                <span>{{ team.avg_skill_level.toFixed(1) }}</span>
              </div>
            </div>

            <div class="row-score">
              <span class="score-value">{{ team.total_skill_score }}</span>
            </div>

            <div v-if="team.rank <= 3" class="top-row-highlight" />
          </article>
        </div>
      </div>

      <!-- Empty State -->
      <div v-else class="empty-state">
        <Trophy :size="64" class="empty-icon" />
        <p class="empty-text">Пока нет данных для отображения</p>
        <p class="empty-subtitle text-muted">Создайте команды и добавьте навыки участникам</p>
      </div>
    </div>
  </section>
      </template>

      <FooterPage />
    </main>
  </div>
</template>

<style scoped lang="scss">
.ratings-view {
  position: relative;
  background: linear-gradient(180deg, #0a0a0a 0%, #111111 50%, #0a0a0a 100%);
  min-height: 100vh;
}

.main-content {
  position: relative;
  padding-top: 0;
}

// Loading State
.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  min-height: 100vh;
  color: $color-text-dim;

  .loading-spinner {
    width: 40px;
    height: 40px;
    border: 2px solid rgba($color-accent, 0.2);
    border-top-color: $color-accent;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

// Error State
.error-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  min-height: 100vh;
  padding: 40px;
  text-align: center;
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

// Leaderboard Section
.leaderboard-section {
  padding: 40px 40px 120px;

  @media (max-width: 768px) {
    padding: 20px 20px 80px;
  }
}

.leaderboard-loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  padding: 60px 40px;
  color: $color-text-dim;

  .loading-spinner {
    width: 32px;
    height: 32px;
    border: 2px solid rgba($color-accent, 0.2);
    border-top-color: $color-accent;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
}

.container {
  max-width: 900px;
  margin: 0 auto;
}

// Toplist Section (Vertical)
.toplist-section {
  background: rgba($color-surface, 0.3);
  border: 1px solid $color-border;
  border-radius: 16px;
  overflow: hidden;
}

.toplist-header {
  display: grid;
  grid-template-columns: 60px 1fr 200px 100px;
  gap: 16px;
  padding: 16px 24px;
  background: rgba($color-surface, 0.5);
  border-bottom: 1px solid $color-border;
  font-size: 12px;
  font-weight: 600;
  color: $color-text-dim;
  text-transform: uppercase;
  letter-spacing: 0.5px;

  @media (max-width: 768px) {
    grid-template-columns: 50px 1fr 80px;
    padding: 12px 16px;
  }

  .header-rank {
    text-align: center;
  }

  .header-stats {
    @media (max-width: 768px) {
      display: none;
    }
  }
}

.toplist-body {
  display: flex;
  flex-direction: column;
}

.toplist-row {
  display: grid;
  grid-template-columns: 60px 1fr 200px 100px;
  gap: 16px;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid rgba($color-border, 0.5);
  position: relative;
  transition: background 0.2s ease;

  @media (max-width: 768px) {
    grid-template-columns: 50px 1fr 80px;
    padding: 16px;
  }

  &:last-child {
    border-bottom: none;
  }

  &:hover {
    background: rgba($color-accent, 0.03);
  }

  &.rank-1 {
    background: linear-gradient(90deg, rgba(#ffd700, 0.08), transparent);

    .rank-indicator {
      background: linear-gradient(135deg, #ffd700, #ffb347);
      color: #0a0a0a;
    }
  }

  &.rank-2 {
    background: linear-gradient(90deg, rgba(#c0c0c0, 0.08), transparent);

    .rank-indicator {
      background: linear-gradient(135deg, #c0c0c0, #e8e8e8);
      color: #0a0a0a;
    }
  }

  &.rank-3 {
    background: linear-gradient(90deg, rgba(#cd7f32, 0.08), transparent);

    .rank-indicator {
      background: linear-gradient(135deg, #cd7f32, #daa520);
      color: #0a0a0a;
    }
  }
}

.row-rank {
  display: flex;
  justify-content: center;
  align-items: center;
}

.rank-indicator {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  font-weight: 700;
  font-size: 14px;

  &.default {
    background: rgba($color-accent, 0.1);
    color: $color-text-dim;
  }

  .rank-number {
    font-family: $font-display;
  }
}

.row-team {
  min-width: 0;

  .team-name {
    font-family: $font-display;
    font-size: 18px;
    font-weight: 600;
    margin-bottom: 4px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .hackathon-name {
    font-size: 12px;
    color: $color-text-muted;
  }
}

.row-stats {
  display: flex;
  gap: 20px;

  @media (max-width: 768px) {
    display: none;
  }

  .stat-item {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 13px;
    color: $color-text-dim;

    svg {
      color: $color-text-muted;
    }
  }
}

.row-score {
  text-align: right;

  .score-value {
    font-family: $font-display;
    font-size: 22px;
    font-weight: 700;
    color: $color-accent;
  }
}

.top-row-highlight {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 3px;
}

.rank-1 .top-row-highlight {
  background: linear-gradient(180deg, #ffd700, #ffb347);
}

.rank-2 .top-row-highlight {
  background: linear-gradient(180deg, #c0c0c0, #e8e8e8);
}

.rank-3 .top-row-highlight {
  background: linear-gradient(180deg, #cd7f32, #daa520);
}

.card-content {
  position: relative;
  padding: 28px;
  background: rgba($color-surface, 0.4);
  border: 1px solid $color-border;
  border-radius: 16px;
  z-index: 2;
  transition: transform 0.3s ease;

  @media (max-width: 768px) {
    padding: 20px;
  }
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

.rank-badge {
  position: absolute;
  top: -12px;
  left: 24px;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border-radius: 20px;
  font-size: 14px;
  font-weight: 600;

  &.default {
    background: rgba($color-accent, 0.2);
    color: $color-accent;
  }
}

.team-header {
  margin-top: 16px;
  margin-bottom: 20px;
}

.team-name {
  font-family: $font-display;
  font-size: 24px;
  font-weight: 600;
  margin-bottom: 4px;
}

.hackathon-name {
  font-size: 13px;
  color: $color-text-dim;
}

.team-hackathon {
  font-size: 12px;
  color: $color-text-muted;
}

// Stats Row
.stats-row {
  display: flex;
  gap: 16px;
  margin-bottom: 20px;
  flex-wrap: wrap;
}

.stat {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;

  .stat-icon {
    color: $color-text-dim;
  }

  .stat-value {
    font-weight: 600;
    color: $color-text;
  }

  .stat-label {
    color: $color-text-dim;
  }
}

// Top Skills
.top-skills {
  margin-bottom: 16px;
}

.skills-label {
  display: block;
  font-size: 11px;
  color: $color-text-muted;
  margin-bottom: 8px;
}

.skills-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.skill-tag {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 6px 12px;
  background: rgba($color-accent, 0.1);
  border: 1px solid rgba($color-accent, 0.2);
  border-radius: 16px;
  font-size: 12px;
  font-weight: 500;

  .skill-level {
    color: $color-accent;
    font-size: 11px;
  }
}

// Category Chart
.category-chart {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.category-bar {
  .category-info {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 4px;
  }

  .category-name {
    font-size: 12px;
    color: $color-text-dim;
  }

  .category-count {
    font-size: 11px;
    color: $color-text-muted;
  }

  .category-progress {
    height: 4px;
    background: rgba($color-text-dim, 0.1);
    border-radius: 2px;
    overflow: hidden;
  }

  .category-fill {
    height: 100%;
    background: linear-gradient(90deg, $color-accent, lighten($color-accent, 20%));
    border-radius: 2px;
    transition: width 0.8s ease;
  }
}

// Empty State
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  padding: 80px 40px;
  text-align: center;
}

.empty-icon {
  opacity: 0.3;
  color: $color-text-dim;
}

.empty-text {
  font-family: $font-display;
  font-size: 20px;
  font-weight: 600;
}

.empty-subtitle {
  font-size: 14px;
  color: $color-text-muted;
}

.text-dim {
  color: $color-text-dim;
}

.text-muted {
  color: $color-text-muted;
}

.mono {
  font-family: $font-mono;
}
</style>
