<template>
  <HackathonCreateWizard :edit-id="id" />
</template>

<script setup lang="ts">
import HackathonCreateWizard from '@/components/HackathonCreateWizard.vue'

defineProps<{
  id: string
}>()
</script>
