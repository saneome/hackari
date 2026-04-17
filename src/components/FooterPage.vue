<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { gsap } from 'gsap'
import {
  ArrowUpRight,
  Github,
  Twitter,
  MessageCircle,
  Terminal,
} from 'lucide-vue-next'

const sectionRef = ref<HTMLElement | null>(null)
const contentRef = ref<HTMLDivElement | null>(null)
const showEasterEgg = ref(false)

const ecosystemProducts = [
  { name: 'XaneoConnect', description: 'Интеграции и API', href: '#' },
  { name: 'XaneoID', description: 'Единая авторизация', href: '#' },
]

const socialLinks = [
  { icon: Github, href: '#', label: 'GitHub' },
  { icon: Twitter, href: '#', label: 'Twitter' },
  { icon: MessageCircle, href: '#', label: 'Discord' },
]

const triggerEasterEgg = () => {
  showEasterEgg.value = true

  // Matrix rain effect
  const canvas = document.querySelector('.matrix-canvas') as HTMLCanvasElement
  if (!canvas) return

  const ctx = canvas.getContext('2d')
  if (!ctx) return

  canvas.width = window.innerWidth
  canvas.height = window.innerHeight

  const chars = '01アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲンヴヵヶ'
  const matrix = chars.split('')
  const fontSize = 14
  const columns = canvas.width / fontSize
  const drops: number[] = Array(Math.floor(columns)).fill(1)

  let frameCount = 0
  let animationId: number

  const draw = () => {
    frameCount++
    if (frameCount > 180) {
      cancelAnimationFrame(animationId)
      showEasterEgg.value = false
      return
    }

    ctx.fillStyle = 'rgba(10, 10, 10, 0.05)'
    ctx.fillRect(0, 0, canvas.width, canvas.height)

    ctx.fillStyle = '#D4FF00'
    ctx.font = `${fontSize}px "JetBrains Mono", monospace`

    for (let i = 0; i < drops.length; i++) {
      const char = matrix[Math.floor(Math.random() * matrix.length)]
      ctx.fillText(char, i * fontSize, drops[i] * fontSize)

      if (drops[i] * fontSize > canvas.height && Math.random() > 0.975) {
        drops[i] = 0
      }
      drops[i]++
    }

    animationId = requestAnimationFrame(draw)
  }

  draw()
}

onMounted(() => {
  if (contentRef.value) {
    const elements = contentRef.value.querySelectorAll('.animate-in')
    gsap.fromTo(elements,
      { y: 40, opacity: 0 },
      {
        y: 0,
        opacity: 1,
        duration: 0.8,
        stagger: 0.1,
        ease: 'power3.out',
        scrollTrigger: {
          trigger: contentRef.value,
          start: 'top 80%',
          toggleActions: 'play none none reverse',
        },
      }
    )
  }
})
</script>

<template>
  <section ref="sectionRef" class="footer-page">
    <!-- Easter Egg Overlay -->
    <Transition name="fade">
      <div v-if="showEasterEgg" class="matrix-overlay">
        <canvas class="matrix-canvas" />
        <div class="matrix-message">
          <Terminal :size="48" class="matrix-icon" />
          <p>
            <span class="japanese">現実逃避</span>
            <span class="subtitle">симуляция продолжается</span>
          </p>
        </div>
      </div>
    </Transition>

    <div ref="contentRef" class="container">
      <!-- Main Footer Content -->
      <div class="footer-main">
        <div class="footer-brand animate-in">
          <h2 class="brand-name">
            hackari
            <span class="brand-dot" />
          </h2>
          <p class="brand-tagline text-dim">
            хакатоны без воды и бюрократии
          </p>
        </div>

        <nav class="footer-links animate-in">
          <div class="links-group">
            <h3 class="group-title mono">ПРОДУКТЫ</h3>
            <ul class="links-list">
              <li v-for="product in ecosystemProducts" :key="product.name">
                <a :href="product.href" class="product-link">
                  <span class="link-name">{{ product.name }}</span>
                  <span class="link-desc text-dim">{{ product.description }}</span>
                  <ArrowUpRight :size="14" class="link-arrow" />
                </a>
              </li>
            </ul>
          </div>

          <div class="links-group">
            <h3 class="group-title mono">СВЯЗЬ</h3>
            <ul class="links-list">
              <li>
                <a href="mailto:hello@hackari.online" class="contact-link">
                  hello@hackari.online
                </a>
              </li>
              <li class="socials">
                <a
                  v-for="link in socialLinks"
                  :key="link.label"
                  :href="link.href"
                  :aria-label="link.label"
                  class="social-link"
                >
                  <component :is="link.icon" :size="20" stroke-width="1.5" />
                </a>
              </li>
            </ul>
          </div>
        </nav>
      </div>

      <!-- Bottom Bar -->
      <div class="footer-bottom animate-in">
        <div class="bottom-left">
          <p class="copyright text-dim">
            © 2024 hackari. Все права защищены.
          </p>
        </div>

        <div class="bottom-center">
          <button class="easter-egg-trigger" @click="triggerEasterEgg">
            <span class="mono text-muted">A Xaneo Product</span>
          </button>
        </div>

        <div class="bottom-right">
          <span class="mono text-dim" style="font-size: 11px;">
            Made with &lt;/&gt; and caffeine
          </span>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped lang="scss">
.footer-page {
  position: relative;
  min-height: 100vh;
  padding: 120px 0 40px;
  background: linear-gradient(180deg, #0a0a0a 0%, #050505 100%);
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  overflow: hidden;
}

.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 40px;
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: space-between;

  @media (max-width: 768px) {
    padding: 0 20px;
  }
}

// Matrix Easter Egg Overlay
.matrix-overlay {
  position: fixed;
  inset: 0;
  z-index: 100;
  background: $color-bg;
}

.matrix-canvas {
  position: absolute;
  inset: 0;
}

.matrix-message {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  text-align: center;
  z-index: 101;
}

.matrix-icon {
  color: $color-accent;
  margin-bottom: 24px;
  animation: pulse-icon 1s ease-in-out infinite;
}

@keyframes pulse-icon {
  0%, 100% { opacity: 1; transform: scale(1); }
  50% { opacity: 0.5; transform: scale(0.95); }
}

.japanese {
  display: block;
  font-size: 72px;
  font-weight: 700;
  color: $color-accent;
  letter-spacing: 0.2em;
  font-family: 'Noto Sans JP', sans-serif;
  margin-bottom: 16px;

  @media (max-width: 768px) {
    font-size: 48px;
  }
}

.subtitle {
  display: block;
  font-size: 14px;
  color: $color-text-dim;
  letter-spacing: 0.3em;
  text-transform: uppercase;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

// Main Content
.footer-main {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 80px;
  margin-bottom: 120px;

  @media (max-width: 768px) {
    grid-template-columns: 1fr;
    gap: 48px;
    margin-bottom: 60px;
  }
}

.footer-brand {
  .brand-name {
    font-size: 64px;
    font-weight: 700;
    line-height: 1;
    margin-bottom: 16px;
    display: inline-flex;
    align-items: baseline;
    gap: 4px;

    @media (max-width: 768px) {
      font-size: 48px;
    }
  }

  .brand-dot {
    width: 12px;
    height: 12px;
    background: $color-accent;
    border-radius: 50%;
    display: inline-block;
    margin-left: 4px;
  }

  .brand-tagline {
    font-size: 18px;
    max-width: 300px;
  }
}

// Links
.footer-links {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 48px;

  @media (max-width: 640px) {
    grid-template-columns: 1fr;
  }
}

.links-group {
  .group-title {
    font-size: 11px;
    letter-spacing: 0.2em;
    color: $color-text-muted;
    margin-bottom: 20px;
  }
}

.links-list {
  list-style: none;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.product-link {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 12px 16px;
  border: 1px solid $color-border;
  border-radius: 8px;
  color: inherit;
  text-decoration: none;
  position: relative;
  transition: all 0.3s ease;

  &:hover {
    border-color: rgba($color-accent, 0.3);
    background: rgba($color-accent, 0.05);

    .link-arrow {
      transform: translate(2px, -2px);
    }
  }
}

.link-name {
  font-weight: 500;
}

.link-desc {
  font-size: 13px;
}

.link-arrow {
  position: absolute;
  top: 12px;
  right: 12px;
  color: $color-text-dim;
  transition: transform 0.2s ease;
}

.contact-link {
  color: $color-accent;
  font-size: 14px;
  text-decoration: none;
  font-family: 'JetBrains Mono', monospace;
  letter-spacing: -0.01em;

  &:hover {
    text-decoration: underline;
  }
}

.socials {
  display: flex;
  gap: 8px;
}

.social-link {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid $color-border;
  border-radius: 8px;
  color: $color-text-dim;
  transition: all 0.3s ease;

  &:hover {
    border-color: $color-accent;
    color: $color-accent;
    background: rgba($color-accent, 0.05);
  }
}

// Bottom Bar
.footer-bottom {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-top: 40px;
  border-top: 1px solid $color-border;

  @media (max-width: 768px) {
    flex-direction: column;
    gap: 16px;
    text-align: center;
  }
}

.copyright {
  font-size: 13px;
}

.easter-egg-trigger {
  background: none;
  border: none;
  padding: 8px 16px;
  cursor: pointer;
  opacity: 0.5;
  transition: opacity 0.3s ease;

  &:hover {
    opacity: 1;
  }
}
</style>
