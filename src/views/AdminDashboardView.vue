<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuth } from '@/composables/useAuth'
import AdminLayout from '@/components/admin/AdminLayout.vue'
import DashboardStats from '@/components/admin/DashboardStats.vue'
import HackathonModerationQueue from '@/components/admin/HackathonModerationQueue.vue'
import OrganizerVerificationQueue from '@/components/admin/OrganizerVerificationQueue.vue'
import ReportsManagement from '@/components/admin/ReportsManagement.vue'
import UserManagement from '@/components/admin/UserManagement.vue'
import NotFoundPage from '@/components/NotFoundPage.vue'

const router = useRouter()
const route = useRoute()
const { user, init } = useAuth()

const isReady = ref(false)
const accessDenied = ref(false)

const views: Record<string, any> = {
  dashboard: DashboardStats,
  'admin-hackathons': HackathonModerationQueue,
  'admin-organizers': OrganizerVerificationQueue,
  'admin-reports': ReportsManagement,
  'admin-users': UserManagement,
}

const currentView = computed(() => {
  const routeName = typeof route.name === 'string' ? route.name : 'dashboard'

  if (routeName === 'admin') {
    return 'dashboard'
  }

  return routeName
})

const handleNavigate = (viewName: string) => {
  router.replace({ name: viewName === 'dashboard' ? 'admin' : viewName })
}

onMounted(async () => {
  await init()

  if (!user.value?.isStaff && !user.value?.isSuperuser) {
    accessDenied.value = true
    return
  }

  isReady.value = true
})
</script>

<template>
  <NotFoundPage v-if="accessDenied" />

  <div v-else-if="isReady">
    <AdminLayout @navigate="handleNavigate">
      <div class="admin-content">
        <component :is="views[currentView]" v-if="views[currentView]" />
        <div v-else class="not-found">
          <p>Страница не найдена</p>
        </div>
      </div>
    </AdminLayout>
  </div>

  <div v-else class="admin-content">
    <div class="not-found">
      <p>Загрузка...</p>
    </div>
  </div>
</template>

<style scoped lang="scss">
.admin-content {
  padding: 32px;
  max-width: 1200px;
}

.not-found {
  text-align: center;
  padding: 80px 20px;
  color: rgba(255, 255, 255, 0.5);
}
</style>
