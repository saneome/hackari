import { ref, computed, onMounted, onUnmounted } from 'vue'
import { authApi, setAuthFailureCallback } from '@/services/api'

interface User {
  id: string
  email: string
  name: string
  isVerified: boolean
}

const user = ref<User | null>(null)
const isLoading = ref(false)
const isInitialized = ref(false)

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
        user.value = JSON.parse(storedUser)
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
      user.value = response.data
      localStorage.setItem('user', JSON.stringify(response.data))
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
      user.value = response.data.user
      localStorage.setItem('user', JSON.stringify(response.data.user))
      return { success: true }
    }

    return { success: false, error: response.error }
  }

  const register = async (email: string, password: string, name: string) => {
    isLoading.value = true
    const response = await authApi.register({ email, password, name })
    isLoading.value = false

    if (response.data?.user) {
      user.value = response.data.user
      localStorage.setItem('user', JSON.stringify(response.data.user))
      return { success: true }
    }

    return { success: false, error: response.error }
  }

  const logout = async () => {
    isLoading.value = true
    await authApi.logout()
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
