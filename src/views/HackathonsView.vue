<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import Lenis from 'lenis'
import { gsap } from 'gsap'
import { ScrollTrigger } from 'gsap/ScrollTrigger'
import AppHeader from '@/components/AppHeader.vue'
import HackathonsPage from '@/components/HackathonsPage.vue'
import FooterPage from '@/components/FooterPage.vue'

gsap.registerPlugin(ScrollTrigger)

const lenis = ref<Lenis | null>(null)

onMounted(() => {
  lenis.value = new Lenis({
    duration: 1.2,
    easing: (t) => Math.min(1, 1.001 - Math.pow(2, -10 * t)),
    orientation: 'vertical',
    gestureOrientation: 'vertical',
    smoothWheel: true,
  })

  lenis.value.on('scroll', ScrollTrigger.update)

  gsap.ticker.add((time) => {
    lenis.value?.raf(time * 1000)
  })

  gsap.ticker.lagSmoothing(0)
})

onUnmounted(() => {
  lenis.value?.destroy()
})
</script>

<template>
  <div class="hackathons-view">
    <AppHeader />
    <main>
      <HackathonsPage />
    </main>
    <FooterPage />
  </div>
</template>

<style scoped lang="scss">
.hackathons-view {
  position: relative;
  background-color: $color-bg;
  min-height: 100vh;
}

main {
  padding-top: 80px;
}
</style>
