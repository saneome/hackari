<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useAuth } from '@/composables/useAuth'
import { useAdminModal } from '@/composables/useAdminModal'
import { adminApi } from '@/services/api'
import type { AdminUser } from '@/services/api'

const { confirm } = useAdminModal()

const { user, init } = useAuth()

const users = ref<AdminUser[]>([])
const isLoading = ref(true)
const isReady = ref(false)
const isUpdatingUserId = ref('')
const errorMessage = ref('')
const successMessage = ref('')

const isSuperuser = computed(() => user.value?.isSuperuser ?? false)
const currentUserId = computed(() => user.value?.id ?? '')

const sortedUsers = computed(() => {
  if (!Array.isArray(users.value)) {
    return []
  }
  return [...users.value].sort((left, right) => {
    if (left.is_superuser !== right.is_superuser) {
      return left.is_superuser ? -1 : 1
    }

    if (left.is_staff !== right.is_staff) {
      return left.is_staff ? -1 : 1
    }

    return left.name.localeCompare(right.name, 'ru')
  })
})

const getInitial = (account: AdminUser) => {
  const source = account.name.trim() || account.email.trim() || 'U'
  return source.charAt(0).toUpperCase()
}

const getRoleLabel = (account: AdminUser) => {
  if (account.is_superuser) {
    return 'superuser'
  }

  if (account.is_staff) {
    return 'staff'
  }

  return 'user'
}

const loadUsers = async () => {
  errorMessage.value = ''

  const response = await adminApi.listUsers()

  if (response.error) {
    errorMessage.value = response.error
    users.value = []
    return
  }

  users.value = response.data ?? []
}

const toggleStaff = async (account: AdminUser) => {
  if (account.is_superuser || isUpdatingUserId.value) {
    return
  }

  const confirmed = await confirm({
    title: 'Подтверждение',
    message: account.is_staff
      ? `Убрать staff-права у ${account.name}?`
      : `Выдать staff-права пользователю ${account.name}?`,
    type: 'warning',
    confirmText: account.is_staff ? 'Убрать права' : 'Выдать права',
    cancelText: 'Отмена',
  })

  if (!confirmed) {
    return
  }

  isUpdatingUserId.value = account.id
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const response = await adminApi.toggleUserStaff(account.id)

    if (response.error) {
      errorMessage.value = response.error
      return
    }

    users.value = users.value.map((item) =>
      item.id === account.id ? { ...item, is_staff: !item.is_staff } : item
    )

    successMessage.value = account.is_staff
      ? `Staff-права сняты у ${account.name}`
      : `Staff-права выданы пользователю ${account.name}`
  } catch {
    errorMessage.value = 'Не удалось обновить пользователя'
  } finally {
    isUpdatingUserId.value = ''
  }
}

onMounted(async () => {
  await init()

  isReady.value = true

  if (!isSuperuser.value) {
    isLoading.value = false
    return
  }

  await loadUsers()
  isLoading.value = false

  // Trigger entrance animations after data loads
  requestAnimationFrame(() => {
    document.querySelectorAll('.animate-in').forEach((el) => {
      el.classList.add('animate-visible')
    })
  })
})
</script>

<template>
  <section class="user-management">
    <header class="management-header animate-in" style="animation-delay: 0ms">
      <div>
        <h1 class="management-title">Пользователи</h1>
        <span class="superuser-badge">superuser only</span>
      </div>
    </header>

    <div v-if="!isReady || isLoading" class="loading-state">
      <div class="spinner"></div>
      <div class="loading-text">Загружаем список пользователей...</div>
    </div>

    <div v-else-if="!isSuperuser" class="access-denied">
      <div class="denied-icon" aria-hidden="true">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="11" width="18" height="10" rx="2" />
          <path d="M7 11V7a5 5 0 0 1 10 0v4" />
        </svg>
      </div>
      <div class="denied-text">Доступ запрещён</div>
      <div class="denied-subtext">Раздел доступен только superuser</div>
    </div>

    <template v-else>
      <div v-if="errorMessage" class="alert alert-error animate-in" style="animation-delay: 50ms">
        {{ errorMessage }}
      </div>

      <div v-if="successMessage" class="alert alert-success animate-in" style="animation-delay: 100ms">
        {{ successMessage }}
      </div>

      <div v-if="sortedUsers.length === 0" class="empty-state animate-in" style="animation-delay: 150ms">
        <div class="empty-icon" aria-hidden="true">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />
            <circle cx="9" cy="7" r="4" />
            <path d="M23 21v-2a4 4 0 0 0-3-3.87" />
            <path d="M16 3.13a4 4 0 0 1 0 7.75" />
          </svg>
        </div>
        <div class="empty-text">Пользователи не найдены</div>
        <div class="denied-subtext">Список появится после первого входа в систему</div>
      </div>

      <div v-else class="users-table animate-in" style="animation-delay: 150ms">
        <div class="table-header">
          <span>Пользователь</span>
          <span>Роль</span>
          <span>Действия</span>
        </div>

        <div
          v-for="(account, index) in sortedUsers"
          :key="account.id"
          class="table-row row-animate"
          :class="{ current: account.id === currentUserId }"
          :style="{ animationDelay: `${200 + index * 30}ms` }"
        >
          <div class="user-cell">
            <div class="user-avatar" :class="{ 'avatar-superuser': account.is_superuser }">
              {{ getInitial(account) }}
            </div>
            <div class="user-info">
              <div class="user-name">{{ account.name }}</div>
              <div class="user-email">{{ account.email }}</div>
              <div class="user-role">
                <span v-if="account.is_superuser" class="role-superuser">superuser</span>
                <template v-else>{{ account.is_staff ? 'staff' : 'user' }}</template>
              </div>
            </div>
          </div>

          <div class="role-cell">
            <span class="role-badge" :class="account.is_staff || account.is_superuser ? 'role-staff' : 'role-user'">
              {{ getRoleLabel(account) }}
            </span>
          </div>

          <div class="actions-cell">
            <span v-if="account.id === currentUserId" class="self-badge">это вы</span>
            <span v-else-if="account.is_superuser" class="self-badge">superuser</span>
            <button
              v-else
              type="button"
              class="action-btn"
              :class="{ loading: isUpdatingUserId === account.id }"
              :disabled="isUpdatingUserId === account.id"
              :title="account.is_staff ? 'Убрать staff' : 'Выдать staff'"
              @click="toggleStaff(account)"
            >
              <svg
                v-if="account.is_staff"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                width="16"
                height="16"
              >
                <path d="M18 6L6 18" />
                <path d="M6 6l12 12" />
              </svg>
              <svg
                v-else
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                width="16"
                height="16"
              >
                <path d="M12 5v14" />
                <path d="M5 12h14" />
              </svg>
            </button>
          </div>
        </div>
      </div>
    </template>
  </section>
</template>


<style scoped lang="scss">
.user-management {
  max-width: 900px;
}

.management-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 24px;
}

.management-title {
  font-size: 24px;
  font-weight: 600;
  color: #ffffff;
  margin: 0;
  font-family: 'Unbounded', sans-serif;
}

.superuser-badge {
  font-size: 10px;
  color: rgba(255, 255, 255, 0.6);
  padding: 4px 8px;
  background: rgba(255, 255, 255, 0.08);
  border-radius: 4px;
  letter-spacing: 0.05em;
}

.access-denied {
  text-align: center;
  padding: 80px 20px;
}

.denied-icon {
  width: 64px;
  height: 64px;
  margin: 0 auto 20px;
  color: rgba(255, 255, 255, 0.2);

  svg {
    width: 100%;
    height: 100%;
  }
}

.denied-text {
  font-size: 18px;
  color: rgba(255, 255, 255, 0.7);
  margin-bottom: 8px;
}

.denied-subtext {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.4);
}

.alert {
  padding: 12px 16px;
  border-radius: 6px;
  margin-bottom: 16px;
  font-size: 14px;

  &.alert-success {
    background: rgba(34, 197, 94, 0.1);
    border: 1px solid rgba(34, 197, 94, 0.2);
    color: #22c55e;
  }

  &.alert-error {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.2);
    color: #ef4444;
  }
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  gap: 16px;
}

.spinner {
  width: 32px;
  height: 32px;
  border: 2px solid rgba(255, 255, 255, 0.1);
  border-top-color: #ffffff;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.loading-text {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.5);
}

.users-table {
  width: 100%;
  background: #111111;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 8px;
  overflow: hidden;
}

.table-header {
  display: grid;
  grid-template-columns: 1fr 120px 100px;
  background: rgba(255, 255, 255, 0.03);
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  padding: 12px 16px;
  font-size: 12px;
  font-weight: 500;
  color: rgba(255, 255, 255, 0.5);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.table-row {
  display: grid;
  grid-template-columns: 1fr 120px 100px;
  padding: 12px 16px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  align-items: center;
  transition: background 0.2s;

  &:hover {
    background: rgba(255, 255, 255, 0.02);
  }

  &:last-child {
    border-bottom: none;
  }

  &.current {
    background: rgba(255, 255, 255, 0.03);
  }
}

.user-cell {
  display: flex;
  align-items: center;
  gap: 12px;
}

.user-avatar {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.08);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  font-weight: 600;
  color: #ffffff;
  flex-shrink: 0;

  &.avatar-superuser {
    border: 2px solid rgba(255, 255, 255, 0.3);
  }
}

.user-info {
  flex: 1;
  min-width: 0;
}

.user-name {
  font-size: 14px;
  font-weight: 500;
  color: #ffffff;
  margin-bottom: 2px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.user-email {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.4);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.user-role {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.5);
  margin-top: 4px;
  font-family: 'JetBrains Mono', monospace;

  .role-superuser {
    color: #ffffff;
    font-weight: 500;
  }
}

.role-cell {
  display: flex;
  align-items: center;
}

.role-badge {
  font-size: 10px;
  padding: 4px 8px;
  border-radius: 4px;
  font-weight: 600;
  letter-spacing: 0.05em;
  font-family: 'JetBrains Mono', monospace;

  &.role-staff {
    background: rgba(255, 255, 255, 0.12);
    color: #ffffff;
  }

  &.role-user {
    background: rgba(255, 255, 255, 0.05);
    color: rgba(255, 255, 255, 0.5);
  }
}

.actions-cell {
  display: flex;
  align-items: center;
  justify-content: center;
}

.self-badge {
  font-size: 10px;
  color: rgba(255, 255, 255, 0.4);
  padding: 4px 8px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 4px;
}

.action-btn {
  width: 36px;
  height: 36px;
  border-radius: 6px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  color: rgba(255, 255, 255, 0.5);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;

  &:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(255, 255, 255, 0.2);
    color: #ffffff;
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  &.loading {
    animation: pulse 1.5s infinite;
  }
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

.empty-state {
  text-align: center;
  padding: 80px 20px;
}

.empty-icon {
  width: 64px;
  height: 64px;
  margin: 0 auto 20px;
  color: rgba(255, 255, 255, 0.2);

  svg {
    width: 100%;
    height: 100%;
  }
}

.empty-text {
  font-size: 18px;
  color: rgba(255, 255, 255, 0.7);
  margin-bottom: 8px;
}

.mono {
  font-family: 'JetBrains Mono', monospace;
}

// Entrance animations
.animate-in {
  opacity: 0;
  transform: translateY(12px);
  animation: fadeSlideIn 0.5s cubic-bezier(0.23, 1, 0.32, 1) forwards;
}

.row-animate {
  opacity: 0;
  transform: translateX(-8px);
  animation: slideInRow 0.4s cubic-bezier(0.23, 1, 0.32, 1) forwards;
}

@keyframes fadeSlideIn {
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes slideInRow {
  to {
    opacity: 1;
    transform: translateX(0);
  }
}
</style>
