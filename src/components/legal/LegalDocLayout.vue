<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  title: string
  effectiveDate?: string
  intro?: string
}

const props = withDefaults(defineProps<Props>(), {
  effectiveDate: '25 апреля 2026 г.',
  intro: '',
})

const hasIntro = computed(() => Boolean(props.intro))
</script>

<template>
  <article class="legal-doc">
    <header class="legal-doc__header">
      <h1 class="legal-doc__title">{{ title }}</h1>
      <p class="legal-doc__date mono">
        Дата вступления в силу: {{ effectiveDate }}
      </p>
    </header>

    <p v-if="hasIntro" class="legal-doc__intro">{{ intro }}</p>

    <div class="legal-doc__body">
      <slot />
    </div>
  </article>
</template>

<style scoped lang="scss">
@use '@/styles/variables' as *;

.legal-doc {
  max-width: 820px;
  margin: 0 auto;
  color: $color-text;
  font-family: $font-body;
  line-height: 1.7;
  font-size: 15px;

  &__header {
    margin-bottom: 32px;
    padding-bottom: 24px;
    border-bottom: 1px solid $color-border;
  }

  &__title {
    font-family: $font-display;
    font-size: clamp(28px, 4vw, 40px);
    font-weight: 600;
    letter-spacing: -0.02em;
    margin: 0 0 12px;
    text-transform: lowercase;
    color: $color-text;
  }

  &__date {
    font-size: 12px;
    color: $color-accent;
    letter-spacing: 0.05em;
    text-transform: uppercase;
  }

  &__intro {
    color: $color-text-dim;
    margin-bottom: 32px;
    font-size: 15px;
  }

  &__body {
    :deep(h2) {
      font-family: $font-display;
      font-size: 22px;
      font-weight: 500;
      margin: 40px 0 16px;
      color: $color-text;
      letter-spacing: -0.01em;
    }

    :deep(h3) {
      font-family: $font-display;
      font-size: 17px;
      font-weight: 500;
      margin: 24px 0 12px;
      color: $color-text;
    }

    :deep(p) {
      margin: 0 0 12px;
      color: $color-text-dim;
    }

    :deep(strong) {
      color: $color-text;
      font-weight: 600;
    }

    :deep(ul),
    :deep(ol) {
      margin: 0 0 16px;
      padding-left: 24px;
      color: $color-text-dim;

      li {
        margin-bottom: 6px;
      }
    }

    :deep(a) {
      color: $color-accent;
      text-decoration: none;
      border-bottom: 1px solid rgba($color-accent, 0.3);
      transition: border-color 0.2s ease;

      &:hover {
        border-bottom-color: $color-accent;
      }
    }

    :deep(code) {
      font-family: $font-mono;
      font-size: 13px;
      color: $color-accent;
      background: rgba($color-accent, 0.08);
      padding: 2px 6px;
      border-radius: 4px;
    }

    :deep(.clause) {
      margin: 0 0 12px;
      color: $color-text-dim;
    }

    :deep(.clause__num) {
      display: inline-block;
      min-width: 42px;
      margin-right: 8px;
      font-family: $font-mono;
      color: $color-text-muted;
      font-size: 13px;
    }

    :deep(.sub) {
      margin: 6px 0 6px 24px;
      color: $color-text-dim;
      font-size: 14px;
    }
  }
}
</style>
