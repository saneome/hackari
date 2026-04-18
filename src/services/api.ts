const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3001'

interface ApiResponse<T> {
  data?: T
  error?: string
}

interface LoginRequest {
  email: string
  password: string
}

interface RegisterRequest {
  email: string
  password: string
  name: string
}

interface RequestResetRequest {
  email: string
}

interface ResetPasswordRequest {
  email: string
  code: string
  new_password: string
}

interface User {
  id: string
  email: string
  name: string
  isVerified: boolean
}

let isRefreshing = false
let refreshPromise: Promise<boolean> | null = null

// Global callback for auth failures (set by useAuth)
let onAuthFailureCallback: (() => void) | null = null

export function setAuthFailureCallback(callback: () => void) {
  onAuthFailureCallback = callback
}

async function refreshAccessToken(): Promise<boolean> {
  if (isRefreshing && refreshPromise) {
    return refreshPromise
  }

  isRefreshing = true
  refreshPromise = fetch(`${API_BASE_URL}/api/auth/refresh`, {
    method: 'POST',
    credentials: 'include',
  })
    .then(response => {
      if (response.ok) {
        return true
      }
      // Refresh failed - token expired or invalid
      if (onAuthFailureCallback) {
        onAuthFailureCallback()
      }
      return false
    })
    .catch(() => {
      if (onAuthFailureCallback) {
        onAuthFailureCallback()
      }
      return false
    })
    .finally(() => {
      isRefreshing = false
      refreshPromise = null
    })

  return refreshPromise
}

async function fetchApi<T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<ApiResponse<T>> {
  const url = `${API_BASE_URL}${endpoint}`

  const defaultHeaders: Record<string, string> = {
    'Content-Type': 'application/json',
    ...((options.headers as Record<string, string>) || {}),
  }

  try {
    let response = await fetch(url, {
      ...options,
      headers: defaultHeaders,
      credentials: 'include',
    })

    // If we get 401, try to refresh the token
    if (response.status === 401) {
      const refreshed = await refreshAccessToken()
      if (refreshed) {
        // Retry the original request with new access token
        response = await fetch(url, {
          ...options,
          headers: defaultHeaders,
          credentials: 'include',
        })
      } else {
        // Refresh failed, notify auth failure
        return { error: 'сессия истекла' }
      }
    }

    if (!response.ok) {
      const errorData = await response.json().catch(() => ({}))
      // Handle specific HTTP error codes with user-friendly messages
      const errorMessages: Record<number, string> = {
        401: 'неверный пароль',
        404: 'пользователь не найден',
        409: 'пользователь уже существует',
        429: 'слишком много запросов, попробуйте позже',
        500: 'ошибка сервера, попробуйте позже',
      }
      const errorMessage = errorMessages[response.status] || errorData.message || 'произошла ошибка'
      return { error: errorMessage }
    }

    const data = await response.json()
    return { data }
  } catch (error) {
    return { error: 'Network error' }
  }
}

export const authApi = {
  login: (credentials: LoginRequest) =>
    fetchApi<{ user: User }>('/api/auth/login', {
      method: 'POST',
      body: JSON.stringify(credentials),
    }),

  register: (data: RegisterRequest) =>
    fetchApi<{ user: User }>('/api/auth/register', {
      method: 'POST',
      body: JSON.stringify(data),
    }),

  requestReset: (data: RequestResetRequest) =>
    fetchApi<{ message: string }>('/api/auth/password-reset/request', {
      method: 'POST',
      body: JSON.stringify(data),
    }),

  verifyResetCode: (data: { email: string; code: string }) =>
    fetchApi<{ message: string }>('/api/auth/password-reset/verify', {
      method: 'POST',
      body: JSON.stringify(data),
    }),

  resetPassword: (data: ResetPasswordRequest) =>
    fetchApi<{ message: string }>('/api/auth/password-reset/reset', {
      method: 'POST',
      body: JSON.stringify(data),
    }),

  logout: () =>
    fetchApi('/api/auth/logout', {
      method: 'POST',
    }),

  me: () =>
    fetchApi<User>('/api/auth/me', {
      method: 'GET',
    }),
}

// Hackathon types
export interface Hackathon {
  id: string
  title: string
  banner_url?: string
  location_type: 'online' | 'offline'
  registration_start: string
  registration_end: string
  event_start: string
  event_end: string
  participant_count: number
  team_count: number
}

export interface HackathonListResponse {
  hackathons: Hackathon[]
  total: number
}

export const hackathonApi = {
  list: () =>
    fetchApi<HackathonListResponse>('/api/hackathons', {
      method: 'GET',
    }),

  getById: (id: string) =>
    fetchApi<Hackathon>(`/api/hackathons/${id}`, {
      method: 'GET',
    }),
}

// User Profile types
export interface Skill {
  id: string
  name: string
  category: string
  level: number
}

export interface UserProfile {
  id: string
  email: string
  name: string
  avatarUrl?: string
  bio?: string
  githubUrl?: string
  telegramUsername?: string
  isVerified: boolean
  createdAt: string
  skills: Skill[]
}

export interface AvailableSkill {
  id: string
  name: string
  category: string
}

export interface UpdateProfileRequest {
  name?: string
  bio?: string
  githubUrl?: string
  telegramUsername?: string
}

export interface AddSkillRequest {
  skillId: string
  level: number
}

export interface RemoveSkillRequest {
  skillId: string
}

export interface UpdateSkillLevelRequest {
  level: number
}

export const userApi = {
  getMe: () =>
    fetchApi<UserProfile>('/api/users/me', {
      method: 'GET',
    }),

  updateMe: (data: UpdateProfileRequest) =>
    fetchApi<UserProfile>('/api/users/me', {
      method: 'PATCH',
      body: JSON.stringify(data),
    }),

  getMySkills: () =>
    fetchApi<Skill[]>('/api/users/me/skills', {
      method: 'GET',
    }),

  addSkill: (data: AddSkillRequest) =>
    fetchApi<Skill[]>('/api/users/me/skills', {
      method: 'POST',
      body: JSON.stringify(data),
    }),

  removeSkill: (data: RemoveSkillRequest) =>
    fetchApi<Skill[]>('/api/users/me/skills', {
      method: 'DELETE',
      body: JSON.stringify(data),
    }),

  updateSkillLevel: (skillId: string, data: UpdateSkillLevelRequest) =>
    fetchApi<Skill[]>(`/api/users/me/skills/${skillId}`, {
      method: 'POST',
      body: JSON.stringify(data),
    }),

  getAvailableSkills: () =>
    fetchApi<AvailableSkill[]>('/api/users/skills', {
      method: 'GET',
    }),
}

// Team Rating types
export interface TeamSkillInfo {
  name: string
  level: number
}

export interface CategoryCompetency {
  name: string
  count: number
  avg_level: number
  percentage: number
}

export interface TeamCompetencyRating {
  team_id: string
  team_name: string
  hackathon_id: string
  hackathon_name: string
  member_count: number
  total_skill_score: number
  skills_count: number
  avg_skill_level: number
  top_skills: TeamSkillInfo[]
  categories: CategoryCompetency[]
  rank: number
}

export const teamApi = {
  getCompetencyRatings: () =>
    fetchApi<TeamCompetencyRating[]>('/api/teams/ratings/competencies', {
      method: 'GET',
    }),
}

// Organizer types
export interface Organizer {
  id: string
  user_id: string
  name: string
  type_: string
  description?: string
  website_url?: string
  logo_url?: string
  email: string
  social_links?: unknown
  is_verified: boolean
  created_at: string
}

export interface CreateOrganizerRequest {
  name: string
  type_: string
  description?: string
  website_url?: string
  logo_url?: string
  email: string
  social_links?: unknown
}

export interface UpdateOrganizerRequest {
  description?: string
  website_url?: string
  logo_url?: string
  email?: string
  social_links?: unknown
}

export interface CreateHackathonRequest {
  title: string
  description?: string
  location_type: string
  city?: string
  venue?: string
  registration_start: string
  registration_end: string
  event_start: string
  event_end: string
  max_participants?: number
  tracks: CreateTrackRequest[]
  deadlines: CreateDeadlineRequest[]
  contact_email?: string
  website_url?: string
  social_links?: unknown
  prize_pool?: string
  prize_currency?: string
  prize_description?: string
  requirements?: string
  team_size_min?: number
  team_size_max?: number
  age_restriction?: string
  skills: string[] // UUIDs
}

export interface CreateTrackRequest {
  name: string
  description?: string
  prize_description?: string
  max_teams?: number
}

export interface CreateDeadlineRequest {
  name: string
  description?: string
  deadline_at: string
  is_milestone: boolean
}

export const organizerApi = {
  getMyOrganizer: () =>
    fetchApi<Organizer>('/api/organizers/me', {
      method: 'GET',
    }),

  createOrganizer: (data: CreateOrganizerRequest) =>
    fetchApi<Organizer>('/api/organizers/me', {
      method: 'POST',
      body: JSON.stringify(data),
    }),

  updateOrganizer: (data: UpdateOrganizerRequest) =>
    fetchApi<Organizer>('/api/organizers/me', {
      method: 'PUT',
      body: JSON.stringify(data),
    }),

  createHackathon: (data: CreateHackathonRequest) =>
    fetchApi<unknown>('/api/hackathons', {
      method: 'POST',
      body: JSON.stringify(data),
    }),
}

export { fetchApi }
