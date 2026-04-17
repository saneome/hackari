<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { gsap } from 'gsap'
import { ScrollTrigger } from 'gsap/ScrollTrigger'
import { useMouse3D } from '@/composables/useMouse3D'

const heroSection = ref<HTMLElement | null>(null)
const titleWrapper = ref<HTMLElement | null>(null)
const geometricShape = ref<HTMLElement | null>(null)
const ctaButton = ref<HTMLButtonElement | null>(null)
const coordinates = ref<HTMLElement | null>(null)

const { rotation, mouse } = useMouse3D()

// Kinetic typography letters
const titleLetters = 'hackari'.split('')

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
        stagger: 0.08,
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
        opacity: 1,
        duration: 1.5,
        ease: 'back.out(1.7)',
        delay: 0.6,
      }
    )
  }

  // Slogan entrance
  const slogan = heroSection.value?.querySelector('.slogan')
  if (slogan) {
    gsap.fromTo(slogan,
      { y: 30, opacity: 0 },
      {
        y: 0,
        opacity: 1,
        duration: 1,
        ease: 'power3.out',
        delay: 1.2,
      }
    )
  }

  // CTA button entrance
  if (ctaButton.value) {
    gsap.fromTo(ctaButton.value,
      { y: 30, opacity: 0 },
      {
        y: 0,
        opacity: 1,
        duration: 0.8,
        ease: 'power3.out',
        delay: 1.5,
      }
    )
  }

  // Scroll-triggered page turn effect
  const shapeInner = heroSection.value?.querySelector('.shape-inner') as HTMLElement
  ScrollTrigger.create({
    trigger: heroSection.value,
    start: 'top top',
    end: 'bottom top',
    scrub: 0.5,
    onUpdate: (self) => {
      const progress = self.progress
      if (shapeInner) {
        shapeInner.style.transform = `
          translateY(${-20 + progress * -30}px)
          rotateZ(${2 + progress * -4}deg)
          rotateX(${progress * -15}deg)
        `
      }
      if (heroSection.value) {
        heroSection.value.style.opacity = String(1 - progress * 0.5)
      }
    },
  })
})
</script>

<template>
  <section ref="heroSection" class="hero-page">
    <!-- Coordinate display -->
    <div ref="coordinates" class="coordinates mono text-dim">
      <span>POS</span>
      <span class="values">
        X:{{ (mouse.x * 100).toFixed(2).padStart(6, '0') }}
        Y:{{ (mouse.y * 100).toFixed(2).padStart(6, '0') }}
      </span>
    </div>

    <!-- Main content -->
    <div class="hero-content">
      <div ref="titleWrapper" class="title-wrapper">
        <h1 class="title">
          <span
            v-for="(letter, index) in titleLetters"
            :key="index"
            class="letter"
            :class="{ 'letter-accent': index >= 4 }"
          >
            {{ letter }}
          </span>
          <span class="cursor cursor-blink" />
        </h1>
      </div>

      <p class="slogan text-dim">
        хакатоны, адаптированные под реальность
        <span class="accent">.online</span>
      </p>

      <button ref="ctaButton" class="cta-button">
        <span class="button-text">начать организацию</span>
        <span class="button-icon">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M7 17L17 7M17 7H7M17 7V17" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </span>
        <span class="button-bg" />
      </button>
    </div>

    <!-- Corner decorations -->
    <div class="corner top-left">
      <span class="mono text-muted">v2.024</span>
    </div>
    <div class="corner top-right">
      <span class="mono text-muted">SECURE</span>
    </div>
    <div class="corner bottom-left">
      <div class="status-dot" />
      <span class="mono text-muted">SYSTEM ONLINE</span>
    </div>

    <!-- 3D Geometric Shape Background -->
    <div
      ref="geometricShape"
      class="geometric-shape"
      :style="{
        transform: `rotateX(${rotation.x}deg) rotateY(${rotation.y}deg) rotateZ(${rotation.z}deg)`,
      }"
    >
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
  </section>
</template>

<style scoped lang="scss">
.hero-page {
  position: relative;
  height: 100vh;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(180deg, #0a0a0a 0%, #111111 100%);
}

// 3D Geometric Shape
.geometric-shape {
  position: absolute;
  top: 50%;
  left: 50%;
  width: 300px;
  height: 300px;
  transform-style: preserve-3d;
  perspective: 1000px;
  margin-left: -150px;
  margin-top: -150px;
  z-index: 0;
  pointer-events: none;
  isolation: isolate;

  @media (max-width: 768px) {
    width: 250px;
    height: 250px;
    margin-left: -125px;
    margin-top: -125px;
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
  width: 150px;
  height: 150px;
  left: 50%;
  top: 50%;
  margin-left: -75px;
  margin-top: -75px;
  background: rgba($color-accent, 0.03);
  border: 1px solid rgba($color-accent, 0.15);
  backdrop-filter: blur(2px);

  &.face-1 { transform: translateZ(75px); }
  &.face-2 { transform: rotateX(-90deg) translateZ(75px); }
  &.face-3 { transform: rotateY(90deg) translateZ(75px); }
  &.face-4 { transform: rotateY(-90deg) translateZ(75px); }
  &.face-5 { transform: rotateX(90deg) translateZ(75px); }
  &.face-6 { transform: rotateY(180deg) translateZ(75px); }
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

  &:nth-child(1) { transform: translateZ(75px) rotateY(0deg); }
  &:nth-child(2) { transform: translateZ(75px) rotateY(90deg); }
  &:nth-child(3) { transform: translateZ(-75px) rotateY(0deg); }
  &:nth-child(4) { transform: translateZ(-75px) rotateY(90deg); }
  &:nth-child(5) { transform: rotateX(90deg) translateZ(75px); }
  &:nth-child(6) { transform: rotateX(90deg) translateZ(-75px); }
}

@keyframes float {
  0%, 100% { transform: translateY(0) rotateZ(0deg); }
  50% { transform: translateY(-20px) rotateZ(2deg); }
}

// Coordinates
.coordinates {
  position: absolute;
  bottom: 40px;
  right: 40px;
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 4px;
  font-size: 11px;
  letter-spacing: 0.05em;
  opacity: 0.6;

  @media (max-width: 768px) {
    bottom: 20px;
    right: 20px;
    font-size: 10px;
  }
}

// Content
.hero-content {
  position: relative;
  z-index: 10;
  text-align: center;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 24px;
}

.title-wrapper {
  overflow: hidden;
  perspective: 600px;
}

.title {
  font-family: $font-display;
  font-size: clamp(48px, 12vw, 140px);
  font-weight: 700;
  letter-spacing: -0.04em;
  line-height: 1;
  display: flex;
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

.slogan {
  font-size: clamp(16px, 2vw, 20px);
  line-height: 1.5;
  max-width: 480px;

  .accent {
    color: $color-accent;
    font-weight: 500;
  }
}

// CTA Button
.cta-button {
  position: relative;
  display: inline-flex;
  align-items: center;
  gap: 12px;
  padding: 16px 32px;
  margin-top: 16px;
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
      transform: rotate(45deg);
    }
  }

  &:active {
    transform: scale(0.98);
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

// Corners
.corner {
  position: absolute;
  font-size: 11px;
  letter-spacing: 0.1em;

  &.top-left {
    top: 40px;
    left: 40px;
  }

  &.top-right {
    top: 40px;
    right: 40px;
  }

  &.bottom-left {
    bottom: 40px;
    left: 40px;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  @media (max-width: 768px) {
    &.top-left,
    &.top-right,
    &.bottom-left {
      top: 20px;
      bottom: auto;
    }

    &.bottom-left {
      display: none;
    }
  }
}

.status-dot {
  width: 6px;
  height: 6px;
  background: $color-accent;
  border-radius: 50%;
  animation: pulse 2s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; transform: scale(1); }
  50% { opacity: 0.5; transform: scale(0.8); }
}
</style>
