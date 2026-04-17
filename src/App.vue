<script setup lang="ts">
import { onMounted } from 'vue'
import { gsap } from 'gsap'
import { ScrollTrigger } from 'gsap/ScrollTrigger'

gsap.registerPlugin(ScrollTrigger)

onMounted(() => {
  // Refresh ScrollTrigger on route change
  ScrollTrigger.refresh()
})
</script>

<template>
  <div class="app">
    <div class="grid-lines" />
    <router-view />
  </div>
</template>

<style>
.app {
  position: relative;
  background-color: var(--color-bg, #0a0a0a);
  min-height: 100vh;
}

.grid-lines {
  position: fixed;
  inset: 0;
  pointer-events: none;
  background-image:
    linear-gradient(rgba(212, 255, 0, 0.02) 1px, transparent 1px),
    linear-gradient(90deg, rgba(212, 255, 0, 0.02) 1px, transparent 1px);
  background-size: 100px 100px;
  z-index: 0;
}
</style>
