<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import Lenis from 'lenis'
import { gsap } from 'gsap'
import { ScrollTrigger } from 'gsap/ScrollTrigger'

import HeroPage from '@/components/HeroPage.vue'
import HowItWorksPage from '@/components/HowItWorksPage.vue'
import FeaturesPage from '@/components/FeaturesPage.vue'
import StatsPage from '@/components/StatsPage.vue'
import FooterPage from '@/components/FooterPage.vue'
import CookieBanner from '@/components/CookieBanner.vue'

gsap.registerPlugin(ScrollTrigger)

const lenis = ref<Lenis | null>(null)

const handleLenisRaf = (time: number) => {
  lenis.value?.raf(time * 1000)
}

onMounted(() => {
  // Reset scroll position
  window.scrollTo(0, 0)

  // Initialize Lenis smooth scroll
  lenis.value = new Lenis({
    duration: 1.2,
    easing: (t) => Math.min(1, 1.001 - Math.pow(2, -10 * t)),
    orientation: 'vertical',
    gestureOrientation: 'vertical',
    smoothWheel: true,
  })

  // Connect Lenis to GSAP ScrollTrigger
  lenis.value.on('scroll', ScrollTrigger.update)

  gsap.ticker.add(handleLenisRaf)

  gsap.ticker.lagSmoothing(0)
})

onUnmounted(() => {
  gsap.ticker.remove(handleLenisRaf)
  lenis.value?.destroy()
  lenis.value = null
})
</script>

<template>
  <div class="landing-view">
    <main class="pages-container">
      <HeroPage id="hero" class="page" />
      <HowItWorksPage id="how-it-works" class="page" />
      <FeaturesPage id="features" class="page" />
      <StatsPage id="stats" class="page" />
      <FooterPage class="page" />
    </main>
    <CookieBanner />
  </div>
</template>

<style scoped lang="scss">
.landing-view {
  position: relative;
  background-color: var(--color-bg, #0a0a0a);
}

.pages-container {
  position: relative;
}

.page {
  position: relative;
  z-index: 1;
}

/* Page stacking for turn effect */
.page:nth-child(1) { z-index: 5; }
.page:nth-child(2) { z-index: 4; }
.page:nth-child(3) { z-index: 3; }
.page:nth-child(4) { z-index: 2; }
.page:nth-child(5) { z-index: 1; }
</style>
