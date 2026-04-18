<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useProfile } from '@/composables/useProfile'
import { useAuth } from '@/composables/useAuth'
import { useRouter } from 'vue-router'
import AppHeader from '@/components/AppHeader.vue'
import { gsap } from 'gsap'
import { ScrollTrigger } from 'gsap/ScrollTrigger'
import {
  Settings,
  Github,
  Send,
  Plus,
  X,
  Save,
  Loader2,
  User,
  Mail,
  Calendar,
  Code2,
  Sparkles
} from 'lucide-vue-next'

gsap.registerPlugin(ScrollTrigger)

const router = useRouter()
const { isAuthenticated } = useAuth()
const {
  profile,
  isLoading,
  isSaving,
  errors,
  fullName,
  initials,
  joinedDate,
  hasSocialLinks,
  skillsByCategory,
  skillCategories,
  availableSkills,
  loadProfile,
  loadAvailableSkills,
  updateProfile,
  addSkill,
  removeSkill,
} = useProfile()

const isEditing = ref(false)
const isSkillModalOpen = ref(false)
const selectedSkillId = ref('')
const selectedLevel = ref(3)
const formData = ref({
  name: '',
  bio: '',
  githubUrl: '',
  telegramUsername: '',
})

const skillLevels = [1, 2, 3, 4, 5]
const levelLabels = ['новичок', 'начинающий', 'средний', 'продвинутый', 'эксперт']

// Template refs for animations
const heroSection = ref<HTMLElement | null>(null)
const titleWrapper = ref<HTMLElement | null>(null)
const bentoGrid = ref<HTMLDivElement | null>(null)
const geometricShape = ref<HTMLElement | null>(null)

// Computed for display name animation
const displayNameLetters = computed(() => fullName.value?.split('') || [])

onMounted(async () => {
  if (!isAuthenticated.value) {
    router.push('/auth')
    return
  }
  await loadProfile()
  if (profile.value) {
    formData.value = {
      name: profile.value.name || '',
      bio: profile.value.bio || '',
      githubUrl: profile.value.githubUrl || '',
      telegramUsername: profile.value.telegramUsername || '',
    }
  }

  // Animate hero entrance
  animateHero()
})

const animateHero = () => {
  // Kinetic typography for name
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
        opacity: 1,
        duration: 1.5,
        ease: 'back.out(1.7)',
        delay: 0.4,
      }
    )
  }

  // Bento cards stagger
  const cards = bentoGrid.value?.querySelectorAll('.bento-card')
  cards?.forEach((card, index) => {
    gsap.fromTo(card,
      { scale: 0.9, opacity: 0, y: 40 },
      {
        scale: 1,
        opacity: 1,
        y: 0,
        duration: 0.8,
        ease: 'power3.out',
        delay: 0.6 + (index % 4) * 0.1,
      }
    )
  })
}

const startEditing = () => {
  if (profile.value) {
    formData.value = {
      name: profile.value.name || '',
      bio: profile.value.bio || '',
      githubUrl: profile.value.githubUrl || '',
      telegramUsername: profile.value.telegramUsername || '',
    }
  }
  isEditing.value = true
}

const cancelEditing = () => {
  isEditing.value = false
}

const saveProfile = async () => {
  const success = await updateProfile(formData.value)
  if (success) {
    isEditing.value = false
  }
}

const getGithubUrl = (username: string) => {
  if (username.startsWith('http')) return username
  return `https://github.com/${username}`
}

const getTelegramUrl = (username: string) => {
  const handle = username.startsWith('@') ? username.slice(1) : username
  return `https://t.me/${handle}`
}

const getSkillLevelLabel = (level: number) => levelLabels[level - 1] || 'новичок'

const openSkillModal = async () => {
  await loadAvailableSkills()
  selectedSkillId.value = availableSkills.value[0]?.id || ''
  selectedLevel.value = 3
  isSkillModalOpen.value = true
}

const closeSkillModal = () => {
  isSkillModalOpen.value = false
  selectedSkillId.value = ''
  selectedLevel.value = 3
}

const handleAddSkill = async () => {
  if (!selectedSkillId.value) return
  const success = await addSkill(selectedSkillId.value, selectedLevel.value)
  if (success) {
    closeSkillModal()
    // Animate new skill
    const cards = bentoGrid.value?.querySelectorAll('.bento-card')
    const lastCard = cards?.[cards.length - 1]
    if (lastCard) {
      gsap.fromTo(lastCard,
        { scale: 0.8, opacity: 0 },
        { scale: 1, opacity: 1, duration: 0.5, ease: 'back.out(1.5)' }
      )
    }
  }
}

const getAvailableSkillsOptions = () => {
  const existingSkillIds = new Set(profile.value?.skills.map(s => s.id) || [])
  return availableSkills.value.filter(s => !existingSkillIds.has(s.id))
}
</script>

<template>
  <div class="profile-view">
    <AppHeader />

    <!-- Loading State -->
    <div v-if="isLoading" class="loading-state">
      <Loader2 class="loading-icon" :size="32" />
      <span class="mono">загрузка профиля...</span>
    </div>

    <!-- Profile Content -->
    <template v-else-if="profile">
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

          <!-- Avatar -->
          <div class="avatar-wrapper">
            <div class="profile-avatar">
              <span class="avatar-initials">{{ initials }}</span>
              <div class="avatar-glow" />
            </div>
          </div>

          <!-- Name with Kinetic Typography -->
          <div ref="titleWrapper" class="title-wrapper">
            <h1 class="profile-name">
              <span
                v-for="(letter, index) in displayNameLetters"
                :key="index"
                class="letter"
                :class="{ 'letter-accent': letter === ' ' }"
              >
                {{ letter === ' ' ? '\u00A0' : letter }}
              </span>
              <span class="cursor cursor-blink">_</span>
            </h1>
          </div>

          <p class="profile-meta">
            <span class="mono text-dim">// {{ profile.email }}</span>
          </p>
        </div>
      </section>

      <!-- Bento Grid Content -->
      <section class="content-section">
        <div class="container">
          <div ref="bentoGrid" class="bento-grid">
            <!-- About Card -->
            <article class="bento-card size-large">
              <div class="card-content">
                <div class="card-header">
                  <div class="card-icon">
                    <User :size="24" />
                  </div>
                  <span class="mono card-label">О СЕБЕ</span>
                </div>
                <div v-if="isEditing" class="edit-content">
                  <div class="form-group">
                    <label class="mono">имя</label>
                    <input v-model="formData.name" type="text" placeholder="как вас зовут?" />
                  </div>
                  <div class="form-group">
                    <label class="mono">описание</label>
                    <textarea v-model="formData.bio" rows="3" placeholder="расскажите о себе..." />
                  </div>
                </div>
                <div v-else class="card-body">
                  <p v-if="profile.bio" class="bio-text">{{ profile.bio }}</p>
                  <p v-else class="text-muted">заполните информацию о себе</p>
                </div>
                <div v-if="!isEditing" class="card-actions">
                  <button class="action-btn" @click="startEditing">
                    <Settings :size="16" />
                    <span class="mono">редактировать</span>
                  </button>
                </div>
              </div>
              <div class="card-highlight" />
              <div class="card-border" />
            </article>

            <!-- Contacts Card -->
            <article class="bento-card size-medium">
              <div class="card-content">
                <div class="card-header">
                  <div class="card-icon">
                    <Mail :size="24" />
                  </div>
                  <span class="mono card-label">КОНТАКТЫ</span>
                </div>
                <div v-if="isEditing" class="edit-content">
                  <div class="form-group">
                    <label class="mono">
                      <Github :size="14" /> github
                    </label>
                    <input v-model="formData.githubUrl" type="text" placeholder="username" />
                  </div>
                  <div class="form-group">
                    <label class="mono">
                      <Send :size="14" /> telegram
                    </label>
                    <input v-model="formData.telegramUsername" type="text" placeholder="@username" />
                  </div>
                </div>
                <div v-else class="card-body">
                  <div v-if="hasSocialLinks" class="social-list">
                    <a v-if="profile.githubUrl" :href="getGithubUrl(profile.githubUrl)" target="_blank" class="social-item">
                      <Github :size="16" />
                      <div class="social-info">
                        <span class="mono social-name">github</span>
                        <span class="social-value">{{ profile.githubUrl }}</span>
                      </div>
                    </a>
                    <a v-if="profile.telegramUsername" :href="getTelegramUrl(profile.telegramUsername)" target="_blank" class="social-item">
                      <Send :size="16" />
                      <div class="social-info">
                        <span class="mono social-name">telegram</span>
                        <span class="social-value">{{ profile.telegramUsername }}</span>
                      </div>
                    </a>
                  </div>
                  <p v-else class="text-muted">добавьте ссылки на соцсети</p>
                </div>
              </div>
              <div class="card-highlight" />
              <div class="card-border" />
            </article>

            <!-- Member Since Card -->
            <article class="bento-card size-small">
              <div class="card-content">
                <div class="card-header">
                  <div class="card-icon">
                    <Calendar :size="24" />
                  </div>
                  <span class="mono card-label">НА ПЛАТФОРМЕ</span>
                </div>
                <div class="card-body centered">
                  <div class="stat-value">{{ joinedDate }}</div>
                  <div class="stat-label mono text-dim">участник с</div>
                </div>
              </div>
              <div class="card-highlight" />
              <div class="card-border" />
            </article>

            <!-- Skills Card -->
            <article class="bento-card size-wide">
              <div class="card-content">
                <div class="card-header">
                  <div class="card-icon">
                    <Code2 :size="24" />
                  </div>
                  <span class="mono card-label">НАВЫКИ</span>
                  <button v-if="!isEditing" class="add-btn" @click="openSkillModal">
                    <Plus :size="16" />
                  </button>
                </div>
                <div class="card-body">
                  <div v-if="!profile.skills || profile.skills.length === 0" class="empty-skills">
                    <Sparkles :size="32" class="sparkle-icon" />
                    <p class="text-muted">у вас пока нет добавленных навыков</p>
                    <button class="action-btn" @click="openSkillModal">
                      <Plus :size="16" />
                      <span class="mono">добавить первый</span>
                    </button>
                  </div>
                  <div v-else class="skills-bento">
                    <div v-for="category in skillCategories" :key="category" class="skill-group">
                      <h3 class="group-title mono">{{ category }}</h3>
                      <div class="group-skills">
                        <div
                          v-for="skill in (skillsByCategory[category] || [])"
                          :key="skill.id"
                          class="skill-chip"
                        >
                          <span class="skill-name">{{ skill.name }}</span>
                          <div class="skill-dots">
                            <span
                              v-for="n in 5"
                              :key="n"
                              class="dot"
                              :class="{ active: n <= skill.level }"
                            />
                          </div>
                          <button class="remove-skill" @click="removeSkill(skill.id)">
                            <X :size="12" />
                          </button>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
              <div class="card-highlight" />
              <div class="card-border" />
            </article>

            <!-- Edit Actions Card -->
            <article v-if="isEditing" class="bento-card size-small">
              <div class="card-content">
                <div class="card-header">
                  <div class="card-icon accent">
                    <Save :size="24" />
                  </div>
                  <span class="mono card-label">ДЕЙСТВИЯ</span>
                </div>
                <div v-if="errors.general" class="error-message">{{ errors.general }}</div>
                <div class="card-body actions">
                  <button class="btn btn-secondary" @click="cancelEditing">
                    <X :size="16" />
                    <span>отмена</span>
                  </button>
                  <button class="btn btn-primary" :disabled="isSaving" @click="saveProfile">
                    <Save v-if="!isSaving" :size="16" />
                    <Loader2 v-else class="spin" :size="16" />
                    {{ isSaving ? 'сохранение...' : 'сохранить' }}
                  </button>
                </div>
              </div>
              <div class="card-highlight" />
              <div class="card-border" />
            </article>
          </div>
        </div>
      </section>
    </template>

    <!-- Add Skill Modal -->
    <div v-if="isSkillModalOpen" class="modal-overlay" @click.self="closeSkillModal">
      <div class="modal">
        <div class="modal-header">
          <div class="modal-title-wrapper">
            <Sparkles :size="20" class="modal-icon" />
            <h3 class="modal-title">добавить навык</h3>
          </div>
          <button class="modal-close" @click="closeSkillModal">
            <X :size="20" />
          </button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label class="mono">навык</label>
            <select v-model="selectedSkillId" class="skill-select">
              <option v-for="skill in getAvailableSkillsOptions()" :key="skill.id" :value="skill.id">
                {{ skill.name }}
              </option>
            </select>
          </div>
          <div class="form-group">
            <label class="mono">уровень владения</label>
            <span class="level-display">{{ levelLabels[selectedLevel - 1] }}</span>
            <div class="level-selector">
              <button
                v-for="n in skillLevels"
                :key="n"
                type="button"
                class="level-dot"
                :class="{ active: n <= selectedLevel }"
                @click="selectedLevel = n"
              />
            </div>
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn btn-secondary" @click="closeSkillModal">отмена</button>
          <button class="btn btn-primary" :disabled="!selectedSkillId" @click="handleAddSkill">
            добавить
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.profile-view {
  min-height: 100vh;
  background: linear-gradient(180deg, #0a0a0a 0%, #111111 50%, #0a0a0a 100%);
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

  .loading-icon {
    animation: spin 1s linear infinite;
  }
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

// Hero Section
.hero-section {
  position: relative;
  min-height: 50vh;
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
  gap: 24px;
}

// Avatar
.avatar-wrapper {
  transform-style: preserve-3d;
  perspective: 600px;
}

.profile-avatar {
  position: relative;
  width: 120px;
  height: 120px;
  background: linear-gradient(135deg, $color-accent, lighten($color-accent, 20%));
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  overflow: hidden;
  box-shadow: 0 0 60px rgba($color-accent, 0.3);

  @media (max-width: 768px) {
    width: 100px;
    height: 100px;
  }
}

.avatar-initials {
  font-family: $font-display;
  font-size: 48px;
  font-weight: 700;
  color: $color-bg;
  position: relative;
  z-index: 2;

  @media (max-width: 768px) {
    font-size: 40px;
  }
}

.avatar-glow {
  position: absolute;
  inset: -20%;
  background: radial-gradient(circle, rgba($color-accent, 0.6) 0%, transparent 70%);
  filter: blur(20px);
  animation: pulse-glow 4s ease-in-out infinite;
}

@keyframes pulse-glow {
  0%, 100% { opacity: 0.5; transform: scale(1); }
  50% { opacity: 0.8; transform: scale(1.1); }
}

// Title with Kinetic Typography
.title-wrapper {
  overflow: hidden;
  perspective: 600px;
}

.profile-name {
  font-family: $font-display;
  font-size: clamp(40px, 8vw, 96px);
  font-weight: 700;
  letter-spacing: -0.03em;
  line-height: 1;
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  position: relative;
}

.letter {
  display: inline-block;
  transform-origin: center bottom;
  transform-style: preserve-3d;
  transition: color 0.3s ease;

  &:hover {
    color: $color-accent;
  }

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

.profile-meta {
  font-size: 14px;
  letter-spacing: 0.05em;

  .mono {
    font-size: 12px;
  }
}

// Geometric Shape
.geometric-shape {
  position: absolute;
  top: 50%;
  right: 15%;
  width: 200px;
  height: 200px;
  transform-style: preserve-3d;
  perspective: 1000px;
  margin-top: -100px;
  z-index: 0;
  pointer-events: none;
  opacity: 0.6;

  @media (max-width: 1024px) {
    width: 150px;
    height: 150px;
    right: 5%;
  }

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
  width: 100px;
  height: 100px;
  left: 50%;
  top: 50%;
  margin-left: -50px;
  margin-top: -50px;
  background: rgba($color-accent, 0.05);
  border: 1px solid rgba($color-accent, 0.2);
  backdrop-filter: blur(2px);

  &.face-1 { transform: translateZ(50px) rotateY(0deg); }
  &.face-2 { transform: rotateY(90deg) translateZ(50px); }
  &.face-3 { transform: translateZ(-50px) rotateY(0deg); }
  &.face-4 { transform: rotateY(-90deg) translateZ(50px); }
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

  &:nth-child(1) { transform: translateZ(50px) rotateY(0deg); }
  &:nth-child(2) { transform: translateZ(50px) rotateY(90deg); }
  &:nth-child(3) { transform: translateZ(-50px) rotateY(0deg); }
  &:nth-child(4) { transform: translateZ(-50px) rotateY(90deg); }
}

@keyframes float {
  0%, 100% { transform: translateY(0) rotateZ(0deg); }
  50% { transform: translateY(-15px) rotateZ(2deg); }
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

// Bento Grid
.bento-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  grid-auto-rows: minmax(180px, auto);
  gap: 16px;

  @media (max-width: 1024px) {
    grid-template-columns: repeat(2, 1fr);
  }

  @media (max-width: 640px) {
    grid-template-columns: 1fr;
  }
}

// Card Sizes
.bento-card {
  transform-style: preserve-3d;
  perspective: 800px;

  &.size-small {
    grid-column: span 1;
    grid-row: span 1;
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

    @media (max-width: 640px) {
      grid-column: span 1;
      grid-row: span 1;
    }
  }

  &.size-wide {
    grid-column: span 2;

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

    .card-content {
      transform: translateY(-2px);
    }
  }
}

.card-content {
  position: relative;
  height: 100%;
  padding: 24px;
  background: rgba($color-surface, 0.4);
  border: 1px solid $color-border;
  border-radius: 16px;
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

.card-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 8px;
}

.card-icon {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: $color-text-dim;
  background: rgba($color-accent, 0.1);
  border-radius: 10px;
  flex-shrink: 0;
  transition: all 0.3s ease;

  &.accent {
    background: rgba($color-accent, 0.2);
    color: $color-accent;
  }
}

.card-label {
  font-size: 11px;
  letter-spacing: 0.15em;
  color: $color-text-dim;
}

.add-btn {
  margin-left: auto;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: $color-accent;
  border: none;
  border-radius: 8px;
  color: $color-bg;
  cursor: pointer;
  transition: opacity 0.3s ease;

  &:hover {
    opacity: 0.9;
  }
}

.card-body {
  flex: 1;
  display: flex;
  flex-direction: column;

  &.centered {
    align-items: center;
    justify-content: center;
    text-align: center;
  }

  &.actions {
    gap: 12px;
  }
}

// Stats
.stat-value {
  font-family: $font-display;
  font-size: 32px;
  font-weight: 700;
  color: $color-text;
}

.stat-label {
  font-size: 12px;
  margin-top: 4px;
}

// Bio
.bio-text {
  font-size: 16px;
  line-height: 1.7;
  color: $color-text;
}

.text-muted {
  color: $color-text-muted;
  font-style: italic;
}

// Social Links
.social-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.social-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: rgba($color-bg, 0.5);
  border: 1px solid $color-border;
  border-radius: 10px;
  color: $color-text;
  text-decoration: none;
  transition: all 0.3s ease;

  &:hover {
    border-color: $color-accent;
    background: rgba($color-accent, 0.05);

    .social-name {
      color: $color-accent;
    }
  }
}

.social-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.social-name {
  font-size: 11px;
  letter-spacing: 0.1em;
  color: $color-text-dim;
  transition: color 0.3s ease;
}

.social-value {
  font-size: 14px;
  color: $color-text;
}

// Card Actions
.card-actions {
  margin-top: auto;
  padding-top: 16px;
}

.action-btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  background: transparent;
  border: 1px solid $color-border;
  border-radius: 8px;
  color: $color-text-dim;
  font-family: $font-body;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.3s ease;

  &:hover {
    border-color: $color-accent;
    color: $color-text;
    background: rgba($color-accent, 0.05);
  }
}

// Edit Form
.edit-content {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;

  label {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    letter-spacing: 0.05em;
    color: $color-text-dim;
  }

  input,
  textarea,
  select {
    padding: 12px 16px;
    background: rgba($color-bg, 0.5);
    border: 1px solid $color-border;
    border-radius: 10px;
    color: $color-text;
    font-family: $font-body;
    font-size: 14px;
    transition: all 0.3s ease;

    &:focus {
      outline: none;
      border-color: $color-accent;
      background: rgba($color-bg, 0.8);
    }

    &::placeholder {
      color: $color-text-muted;
    }
  }

  textarea {
    resize: vertical;
    min-height: 80px;
  }
}

// Skills
.empty-skills {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  padding: 32px;
  text-align: center;
}

.sparkle-icon {
  color: $color-accent;
  animation: float 3s ease-in-out infinite;
}

.skills-bento {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.skill-group {
  .group-title {
    font-size: 11px;
    letter-spacing: 0.15em;
    color: $color-text-muted;
    margin-bottom: 12px;
    text-transform: uppercase;
  }
}

.group-skills {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.skill-chip {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 8px 14px;
  background: rgba($color-accent, 0.1);
  border: 1px solid rgba($color-accent, 0.2);
  border-radius: 20px;
  transition: all 0.3s ease;

  &:hover {
    background: rgba($color-accent, 0.15);
    border-color: rgba($color-accent, 0.3);
  }
}

.skill-name {
  font-size: 13px;
  font-weight: 500;
}

.skill-dots {
  display: flex;
  gap: 3px;
}

.dot {
  width: 4px;
  height: 4px;
  border-radius: 50%;
  background: rgba($color-text-dim, 0.3);

  &.active {
    background: $color-accent;
  }
}

.remove-skill {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  padding: 0;
  background: rgba($color-error, 0.1);
  border: none;
  border-radius: 50%;
  color: $color-text-muted;
  cursor: pointer;
  transition: all 0.3s ease;
  opacity: 0;

  .skill-chip:hover & {
    opacity: 1;
  }

  &:hover {
    background: rgba($color-error, 0.3);
    color: $color-error;
  }
}

// Error
.error-message {
  padding: 12px 16px;
  background: rgba($color-error, 0.1);
  border: 1px solid rgba($color-error, 0.3);
  border-radius: 8px;
  color: $color-error;
  font-size: 13px;
  margin-bottom: 8px;
}

// Buttons
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px 24px;
  font-family: $font-body;
  font-size: 14px;
  font-weight: 500;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.3s ease;

  &-secondary {
    background: transparent;
    border: 1px solid $color-border;
    color: $color-text-dim;

    &:hover {
      border-color: $color-text;
      color: $color-text;
    }
  }

  &-primary {
    background: $color-accent;
    border: 1px solid $color-accent;
    color: $color-bg;

    &:hover:not(:disabled) {
      background: lighten($color-accent, 10%);
    }

    &:disabled {
      opacity: 0.7;
      cursor: not-allowed;
    }
  }
}

// Modal
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba($color-bg, 0.8);
  backdrop-filter: blur(12px);
  z-index: 200;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
}

.modal {
  background: rgba($color-surface, 0.9);
  border: 1px solid $color-border;
  border-radius: 16px;
  width: 100%;
  max-width: 420px;
  overflow: hidden;
  animation: modal-in 0.3s ease;
  backdrop-filter: blur(20px);
}

@keyframes modal-in {
  from {
    opacity: 0;
    transform: scale(0.95) translateY(20px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px;
  border-bottom: 1px solid $color-border;
}

.modal-title-wrapper {
  display: flex;
  align-items: center;
  gap: 12px;
}

.modal-icon {
  color: $color-accent;
}

.modal-title {
  font-family: $font-display;
  font-size: 18px;
  font-weight: 600;
  text-transform: lowercase;
}

.modal-close {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 8px;
  background: transparent;
  border: none;
  border-radius: 8px;
  color: $color-text-dim;
  cursor: pointer;
  transition: all 0.3s ease;

  &:hover {
    color: $color-text;
    background: rgba($color-text, 0.05);
  }
}

.modal-body {
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.level-display {
  font-size: 14px;
  color: $color-accent;
  margin-top: -4px;
}

.level-selector {
  display: flex;
  gap: 8px;
}

.level-dot {
  flex: 1;
  height: 8px;
  background: rgba($color-text-dim, 0.2);
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.3s ease;

  &.active {
    background: $color-accent;
  }

  &:hover:not(.active) {
    background: rgba($color-text-dim, 0.4);
  }
}

.skill-select {
  width: 100%;
  padding: 12px 16px;
  background: rgba($color-bg, 0.5);
  border: 1px solid $color-border;
  border-radius: 10px;
  color: $color-text;
  font-family: $font-body;
  font-size: 14px;
  cursor: pointer;
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='%236b7280' stroke-width='2'%3E%3Cpath d='m6 9 6 6 6-6'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 12px center;

  &:focus {
    outline: none;
    border-color: $color-accent;
  }
}

.modal-footer {
  display: flex;
  gap: 12px;
  padding: 16px 24px 24px;

  .btn {
    flex: 1;
  }
}
</style>
