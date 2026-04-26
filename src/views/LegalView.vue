<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import {
  PhFileText,
  PhShield,
  PhCookie,
  PhCheckSquare,
  PhHandshake,
  PhScales,
  PhArrowRight,
} from '@phosphor-icons/vue'

import TermsOfService from '@/components/legal/TermsOfService.vue'
import PrivacyPolicy from '@/components/legal/PrivacyPolicy.vue'
import CookiePolicy from '@/components/legal/CookiePolicy.vue'
import PersonalDataConsent from '@/components/legal/PersonalDataConsent.vue'
import OrganizerAgreement from '@/components/legal/OrganizerAgreement.vue'

interface LegalDoc {
  slug: string
  title: string
  short: string
  description: string
  icon: typeof PhFileText
  component: ReturnType<typeof Object.assign>
}

const documents: LegalDoc[] = [
  {
    slug: 'terms-of-service',
    title: 'Пользовательское соглашение',
    short: 'Условия использования',
    description: 'Публичная оферта об использовании платформы hackari.',
    icon: PhFileText,
    component: TermsOfService,
  },
  {
    slug: 'privacy-policy',
    title: 'Политика конфиденциальности',
    short: 'Конфиденциальность',
    description: 'Порядок обработки персональных данных по 152-ФЗ.',
    icon: PhShield,
    component: PrivacyPolicy,
  },
  {
    slug: 'personal-data-consent',
    title: 'Согласие на обработку ПД',
    short: 'Согласие на обработку ПД',
    description: 'Форма согласия по ст. 9 152-ФЗ для регистрации.',
    icon: PhCheckSquare,
    component: PersonalDataConsent,
  },
  {
    slug: 'cookie-policy',
    title: 'Политика использования cookie',
    short: 'Cookie',
    description: 'Категории cookie, цели, сроки и управление.',
    icon: PhCookie,
    component: CookiePolicy,
  },
  {
    slug: 'organizer-agreement',
    title: 'Договор-оферта для организаторов',
    short: 'Договор для организаторов',
    description: 'Условия размещения хакатонов на платформе.',
    icon: PhHandshake,
    component: OrganizerAgreement,
  },
]

const route = useRoute()

const activeSlug = computed<string | null>(() => {
  const param = route.params.slug
  if (typeof param === 'string') return param
  if (Array.isArray(param) && param.length) return param[0]
  return null
})

const activeDoc = computed(() => documents.find((doc) => doc.slug === activeSlug.value) ?? null)
</script>

<template>
  <div class="legal-view">
    <main class="legal-view__main">
      <div class="legal-view__container">
        <aside class="legal-view__sidebar">
          <div class="legal-view__brand">
            <PhScales :size="20" weight="duotone" class="legal-view__brand-icon" />
            <span class="legal-view__brand-text">правовые<br>документы</span>
          </div>

          <nav class="legal-view__nav">
            <router-link
              v-for="doc in documents"
              :key="doc.slug"
              :to="`/legal/${doc.slug}`"
              class="legal-view__nav-link"
              :class="{ 'legal-view__nav-link--active': activeSlug === doc.slug }"
            >
              <component :is="doc.icon" :size="16" class="legal-view__nav-icon" />
              <span class="legal-view__nav-label">{{ doc.short }}</span>
            </router-link>
          </nav>
        </aside>

        <section class="legal-view__content">
          <template v-if="activeDoc">
            <component :is="activeDoc.component" />
          </template>

          <template v-else>
            <header class="legal-view__index-header">
              <p class="legal-view__index-eyebrow mono">правовые документы</p>
              <h1 class="legal-view__index-title">всё, что регулирует работу hackari</h1>
              <p class="legal-view__index-subtitle">
                Документы составлены в соответствии с законодательством Российской Федерации:
                152-ФЗ, 149-ФЗ, ГК РФ, НК РФ. Действующая редакция вступает в силу 25 апреля 2026 г.
              </p>
            </header>

            <ul class="legal-view__index-list">
              <li
                v-for="doc in documents"
                :key="doc.slug"
                class="legal-view__index-item"
              >
                <router-link :to="`/legal/${doc.slug}`" class="legal-view__index-card">
                  <div class="legal-view__index-card-icon">
                    <component :is="doc.icon" :size="22" weight="duotone" />
                  </div>
                  <div class="legal-view__index-card-body">
                    <h2 class="legal-view__index-card-title">{{ doc.title }}</h2>
                    <p class="legal-view__index-card-desc">{{ doc.description }}</p>
                  </div>
                  <PhArrowRight :size="18" class="legal-view__index-card-arrow" />
                </router-link>
              </li>
            </ul>
          </template>
        </section>
      </div>
    </main>
  </div>
</template>

<style scoped lang="scss">
@use '@/styles/variables' as *;

.legal-view {
  min-height: 100vh;
  background: linear-gradient(180deg, $color-bg 0%, darken($color-bg, 2%) 100%);
  color: $color-text;

  &__main {
    padding: 120px 0 80px;
  }

  &__container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 40px;
    display: grid;
    grid-template-columns: 240px 1fr;
    gap: 64px;

    @media (max-width: $breakpoint-lg) {
      grid-template-columns: 1fr;
      gap: 32px;
      padding: 0 20px;
    }
  }

  &__sidebar {
    position: sticky;
    top: 100px;
    align-self: start;
    display: flex;
    flex-direction: column;
    gap: 24px;

    @media (max-width: $breakpoint-lg) {
      position: static;
    }
  }

  &__brand {
    display: flex;
    align-items: center;
    gap: 12px;
    padding-bottom: 20px;
    border-bottom: 1px solid $color-border;
  }

  &__brand-icon {
    color: $color-accent;
    flex-shrink: 0;
  }

  &__brand-text {
    font-family: $font-display;
    font-size: 13px;
    line-height: 1.3;
    text-transform: lowercase;
    color: $color-text-dim;
    letter-spacing: 0.02em;
  }

  &__nav {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  &__nav-link {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 12px;
    border-radius: 8px;
    color: $color-text-dim;
    text-decoration: none;
    font-size: 13px;
    line-height: 1.4;
    border: 1px solid transparent;
    transition: all 0.2s ease;

    &:hover {
      color: $color-text;
      background: rgba($color-text, 0.03);
      border-color: $color-border;
    }

    &--active {
      color: $color-accent;
      background: rgba($color-accent, 0.06);
      border-color: rgba($color-accent, 0.25);

      .legal-view__nav-icon {
        color: $color-accent;
      }
    }
  }

  &__nav-icon {
    color: $color-text-muted;
    flex-shrink: 0;
    transition: color 0.2s ease;
  }

  &__nav-label {
    font-family: $font-body;
  }

  &__content {
    min-width: 0;
  }

  &__index-header {
    margin-bottom: 48px;
  }

  &__index-eyebrow {
    color: $color-accent;
    font-size: 12px;
    letter-spacing: 0.2em;
    text-transform: uppercase;
    margin: 0 0 12px;
  }

  &__index-title {
    font-family: $font-display;
    font-size: clamp(32px, 5vw, 48px);
    font-weight: 600;
    line-height: 1.1;
    letter-spacing: -0.02em;
    margin: 0 0 16px;
    text-transform: lowercase;
  }

  &__index-subtitle {
    color: $color-text-dim;
    max-width: 640px;
    line-height: 1.6;
    font-size: 15px;
  }

  &__index-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  &__index-item {
    margin: 0;
  }

  &__index-card {
    display: flex;
    align-items: center;
    gap: 20px;
    padding: 20px 24px;
    background: rgba($color-text, 0.02);
    border: 1px solid $color-border;
    border-radius: 12px;
    color: $color-text;
    text-decoration: none;
    transition: all 0.3s $transition-smooth;

    &:hover {
      border-color: rgba($color-accent, 0.4);
      background: rgba($color-accent, 0.04);
      transform: translateX(4px);

      .legal-view__index-card-arrow {
        color: $color-accent;
        transform: translateX(4px);
      }
    }
  }

  &__index-card-icon {
    width: 48px;
    height: 48px;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba($color-accent, 0.08);
    border: 1px solid rgba($color-accent, 0.2);
    border-radius: 10px;
    color: $color-accent;
  }

  &__index-card-body {
    flex: 1;
    min-width: 0;
  }

  &__index-card-title {
    font-family: $font-display;
    font-size: 18px;
    font-weight: 500;
    margin: 0 0 6px;
    text-transform: lowercase;
    letter-spacing: -0.01em;
  }

  &__index-card-desc {
    font-size: 13px;
    color: $color-text-dim;
    margin: 0;
    line-height: 1.5;
  }

  &__index-card-arrow {
    color: $color-text-muted;
    transition: all 0.3s $transition-smooth;
    flex-shrink: 0;
  }
}
</style>
