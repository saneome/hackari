import { ref, computed, onMounted, onUnmounted } from 'vue'
import { authApi, setAuthFailureCallback } from '@/services/api'

interface User {
  id: string
  email: string
  name: string
  isVerified: boolean
  isStaff?: boolean
  isSuperuser?: boolean
}

type RawAuthUser = Partial<User> & {
  is_verified?: boolean
  is_staff?: boolean
  is_superuser?: boolean
}

const user = ref<User | null>(null)
const isLoading = ref(false)
const isInitialized = ref(false)

const normalizeUser = (value: unknown): User | null => {
  if (!value || typeof value !== 'object') {
    return null
  }

  const raw = value as RawAuthUser

  if (typeof raw.id !== 'string' || typeof raw.email !== 'string' || typeof raw.name !== 'string') {
    return null
  }

  return {
    id: raw.id,
    email: raw.email,
    name: raw.name,
    isVerified: raw.isVerified ?? raw.is_verified ?? false,
    isStaff: raw.isStaff ?? raw.is_staff ?? false,
    isSuperuser: raw.isSuperuser ?? raw.is_superuser ?? false,
  }
}

export function useAuth() {
  const isAuthenticated = computed(() => !!user.value)

  const handleAuthFailure = () => {
    user.value = null
    localStorage.removeItem('user')
  }

  const init = async () => {
    if (isInitialized.value) return

    // Setup auth failure callback for token refresh failures
    setAuthFailureCallback(handleAuthFailure)

    const storedUser = localStorage.getItem('user')
    if (storedUser) {
      try {
        const parsedUser = normalizeUser(JSON.parse(storedUser))

        if (parsedUser) {
          user.value = parsedUser
        } else {
          localStorage.removeItem('user')
        }
      } catch {
        localStorage.removeItem('user')
      }
    }

    // Verify session with backend - 401 will trigger token refresh automatically
    const response = await authApi.me()
    if (response.error) {
      // Check if it's a session expiration error
      if (response.error === 'сессия истекла') {
        // Token refresh failed, user is already logged out by handleAuthFailure
      }
      user.value = null
      localStorage.removeItem('user')
    } else if (response.data) {
      const normalizedUser = normalizeUser(response.data)

      if (normalizedUser) {
        user.value = normalizedUser
        localStorage.setItem('user', JSON.stringify(normalizedUser))
      } else {
        user.value = null
        localStorage.removeItem('user')
      }
    }

    isInitialized.value = true
  }

  onUnmounted(() => {
    // Clean up callback
    setAuthFailureCallback(() => {})
  })

  const login = async (email: string, password: string) => {
    isLoading.value = true
    const response = await authApi.login({ email, password })
    isLoading.value = false

    if (response.data?.user) {
      const normalizedUser = normalizeUser(response.data.user)

      if (normalizedUser) {
        user.value = normalizedUser
        localStorage.setItem('user', JSON.stringify(normalizedUser))
        return { success: true }
      }
    }

    return { success: false, error: response.error }
  }

  const register = async (email: string, password: string, name: string) => {
    isLoading.value = true
    const response = await authApi.register({ email, password, name })
    isLoading.value = false

    if (response.data?.user) {
      const normalizedUser = normalizeUser(response.data.user)

      if (normalizedUser) {
        user.value = normalizedUser
        localStorage.setItem('user', JSON.stringify(normalizedUser))
        return { success: true }
      }
    }

    return { success: false, error: response.error }
  }

  const logout = async () => {
    isLoading.value = true
    try {
      await authApi.logout()
    } catch {
      // Ignore API errors - always logout locally
    }
    isLoading.value = false

    user.value = null
    localStorage.removeItem('user')
    window.location.href = '/'
  }

  onMounted(() => {
    init()
  })

  return {
    user,
    isAuthenticated,
    isLoading,
    login,
    register,
    logout,
    init,
  }
}
