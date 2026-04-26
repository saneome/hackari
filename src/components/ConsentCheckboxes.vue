<script setup lang="ts">
import BaseCheckbox from './BaseCheckbox.vue'

interface Props {
  termsAccepted: boolean
  privacyAccepted: boolean
  errors?: {
    terms?: string
    privacy?: string
  }
}

const props = withDefaults(defineProps<Props>(), {
  errors: () => ({}),
})

const emit = defineEmits<{
  (e: 'update:termsAccepted', value: boolean): void
  (e: 'update:privacyAccepted', value: boolean): void
}>()
</script>

<template>
  <div class="consent-checkboxes">
    <BaseCheckbox
      :model-value="termsAccepted"
      @update:model-value="emit('update:termsAccepted', $event)"
      :error="errors.terms"
    >
      я принимаю
      <router-link to="/legal/terms-of-service" target="_blank" class="doc-link">
        условия использования
      </router-link>
    </BaseCheckbox>

    <BaseCheckbox
      :model-value="privacyAccepted"
      @update:model-value="emit('update:privacyAccepted', $event)"
      :error="errors.privacy"
    >
      я даю согласие на
      <router-link to="/legal/personal-data-consent" target="_blank" class="doc-link">
        обработку персональных данных
      </router-link>
    </BaseCheckbox>
  </div>
</template>

<style scoped lang="scss">
.consent-checkboxes {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 12px 0;
}
</style>
