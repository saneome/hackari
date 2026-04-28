const API_BASE_URL = ((import.meta as unknown) as { env?: { VITE_API_URL?: string } }).env?.VITE_API_URL || ''

interface ApiResponse<T> {
  data?: T
  error?: string
  message?: string
  status?: number
}

interface LoginRequest {
  email: string
  password: string
}

interface RegisterRequest {
  email: string
  password: string
  name: string
  terms_accepted: boolean
  privacy_accepted: boolean
}

interface RequestResetRequest {
  email: string
}

interface ResetPasswordRequest {
  token: string
  new_password: string
}

interface ResendVerificationRequest {
  email: string
}

interface User {
  id: string
  email: string
  name: string
  isVerified: boolean
  isStaff?: boolean
  isSuperuser?: boolean
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
      const backendMessage = errorData.message || errorData.error

      const errorMessages: Record<number, string> = {
        401: 'неверный пароль',
        403: backendMessage || 'доступ запрещён',
        404: 'пользователь не найден',
        409: 'пользователь уже существует',
        429: 'слишком много запросов, попробуйте позже',
        500: 'ошибка сервера, попробуйте позже',
      }

      // Special handling for email not verified
      if (response.status === 403 && (backendMessage?.toLowerCase().includes('email не подтверждён') || backendMessage?.toLowerCase().includes('не подтверждён'))) {
        return { error: 'EMAIL_NOT_VERIFIED', message: backendMessage }
      }

      const errorMessage = errorMessages[response.status] || backendMessage || 'произошла ошибка'
      return { error: errorMessage, status: response.status }
    }

    // Handle empty response (e.g., 204 No Content)
    const contentType = response.headers.get('content-type')
    if (!contentType || !contentType.includes('application/json')) {
      return { data: undefined }
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

  resetPassword: (data: ResetPasswordRequest) =>
    fetchApi<{ message: string }>('/api/auth/password-reset/reset', {
      method: 'POST',
      body: JSON.stringify(data),
    }),

  resendVerification: (data: ResendVerificationRequest) =>
    fetchApi<{ message: string }>('/api/auth/resend-verification', {
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
export interface OrganizerInfo {
  id: string
  name: string
  avatar_url?: string
}

export interface Track {
  id: string
  name: string
  description?: string
  prize_description?: string
  max_teams?: number
  team_count: number
}

export interface Deadline {
  id: string
  name: string
  description?: string
  deadline_at: string
  is_milestone: boolean
}

export interface HackathonSkill {
  id: string
  name: string
  category: string
}

export interface Hackathon {
  id: string
  title: string
  banner_url?: string
  location_type: 'online' | 'offline' | 'hybrid'
  city?: string
  venue?: string
  registration_start: string
  registration_end: string
  event_start: string
  event_end: string
  max_participants?: number
  participant_count: number
  team_count: number
  description?: string
  organizer?: OrganizerInfo
  is_published: boolean
  status: string
  tracks: Track[]
  deadlines: Deadline[]
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
  skills: HackathonSkill[]
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

  delete: (id: string) =>
    fetchApi<void>(`/api/hackathons/${id}`, {
      method: 'DELETE',
    }),

  cancel: (id: string) =>
    fetchApi<void>(`/api/hackathons/${id}/cancel`, {
      method: 'POST',
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
  organizerTermsAcceptedAt?: string | null
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

export interface UserTeam {
  id: string
  name: string
  role: string
}

export const userApi = {
  getMe: () =>
    fetchApi<UserProfile>('/api/users/me', {
      method: 'GET',
    }),

  acceptOrganizerTerms: () =>
    fetchApi<UserProfile>('/api/users/me/organizer-terms/accept', {
      method: 'POST',
    }),

  updateMe: (data: UpdateProfileRequest) =>
    fetchApi<UserProfile>('/api/users/me', {
      method: 'PATCH',
      body: JSON.stringify(data),
    }),

  getMyTeamForHackathon: (hackathonId: string) =>
    fetchOptional<UserTeam>(`/api/users/me/teams?hackathon_id=${hackathonId}`, {
      method: 'GET',
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

  deleteMe: () =>
    fetchApi<void>('/api/users/me', {
      method: 'DELETE',
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

// Team Detail types
export interface TeamMember {
  id: string
  user_id: string
  name: string
  avatar_url?: string
  role: string
  joined_at: string
}

export interface TeamDetail {
  id: string
  name: string
  description?: string
  hackathon_id: string
  track?: {
    id: string
    name: string
  }
  repo_url?: string
  demo_url?: string
  presentation_url?: string
  status: string
  members: TeamMember[]
  created_at: string
}

export interface UpdateTeamData {
  name?: string
  description?: string
  track_id?: string
  repo_url?: string
  demo_url?: string
  presentation_url?: string
}

export const teamApi = {
  getCompetencyRatings: (category?: string) =>
    fetchApi<TeamCompetencyRating[]>(`/api/teams/ratings/competencies${category ? `?category=${encodeURIComponent(category)}` : ''}`, {
      method: 'GET',
    }),
  getRatingCategories: () =>
    fetchApi<string[]>('/api/teams/ratings/categories', {
      method: 'GET',
    }),

  getById: (id: string) =>
    fetchApi<TeamDetail>(`/api/teams/${id}`, {
      method: 'GET',
    }),

  update: (id: string, data: UpdateTeamData) =>
    fetchApi<TeamDetail>(`/api/teams/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    }),

  leave: (id: string) =>
    fetchApi<{ success: boolean; new_leader?: string; team_disbanded?: boolean }>(`/api/teams/${id}/leave`, {
      method: 'POST',
      body: JSON.stringify({}),
    }),

  delete: (id: string) =>
    fetchApi<null>(`/api/teams/${id}`, {
      method: 'DELETE',
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
  rejection_reason?: string
  created_at: string
}

export interface OrganizerHackathonSummary {
  id: string
  title: string
  banner_url?: string
  event_start: string
  event_end: string
  location_type: string
  is_published: boolean
  status: string
  participant_count: number
  team_count: number
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
  name?: string
  type_?: string
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
  new_organizer_id?: string
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
    fetchOptional<Organizer>('/api/organizers/me', {
      method: 'GET',
    }),

  getHackathons: (id: string) =>
    fetchApi<OrganizerHackathonSummary[]>(`/api/organizers/${id}/hackathons`, {
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

  updateHackathon: (id: string, data: Partial<CreateHackathonRequest>) =>
    fetchApi<unknown>(`/api/hackathons/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    }),
}

// Helper to fetch optional resources - 404 returns undefined silently
async function fetchOptional<T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<{ data?: T; error?: string }> {
  const result = await fetchApi<T>(endpoint, options)
  // 404 is expected for optional resources - silently return undefined
  if (result.error?.includes('не найден')) {
    return { data: undefined }
  }
  return result
}

// Rating Criteria types
export interface RatingCriteria {
  id: string
  hackathon_id: string
  name: string
  description?: string
  weight: number
  max_score: number
  sort_order: number
  created_at: string
  updated_at: string
}

export interface CreateCriteriaRequest {
  name: string
  description?: string
  weight: number
  max_score: number
}

export interface UpdateCriteriaRequest {
  name?: string
  description?: string
  weight?: number
}

export interface ReorderCriteriaRequest {
  criteria_ids: string[]
}

// Rating Score types
export interface ScoreInput {
  criteria_id: string
  score: number
}

export interface ScoreDetail extends ScoreInput {
  criteria_name: string
  max_score: number
  weight: number
  weighted_score: number
}

export interface Rating {
  id: string
  submission_id: string
  organizer_id: string
  organizer_name: string
  total_score: number
  feedback?: string
  is_final: boolean
  scores: ScoreDetail[]
  created_at: string
  updated_at: string
}

export interface CreateRatingRequest {
  submission_id: string
  scores: ScoreInput[]
  feedback?: string
  is_final: boolean
}

export interface UpdateRatingRequest {
  scores: ScoreInput[]
  feedback?: string
  is_final?: boolean
}

export interface TeamBrief {
  id: string
  name: string
}

export interface SubmissionWithRating {
  id: string
  title: string
  description?: string
  repo_url?: string
  demo_url?: string
  status: string
  submitted_at?: string
  team: TeamBrief
  rating?: Rating
}

export interface SubmissionsWithRatingsResponse {
  submissions: SubmissionWithRating[]
}

export interface PublicRatingEntry {
  rank: number
  team_id: string
  team_name: string
  submission_id: string
  submission_title: string
  total_score: number
  is_final: boolean
  feedback?: string
}

export interface PublicRatingsResponse {
  hackathon_id: string
  hackathon_title: string
  ratings: PublicRatingEntry[]
}

export const ratingApi = {
  // Criteria
  getCriteria: (hackathonId: string) =>
    fetchApi<RatingCriteria[]>(`/api/hackathons/${hackathonId}/criteria`, {
      method: 'GET',
    }),
  createCriteria: (hackathonId: string, data: CreateCriteriaRequest) =>
    fetchApi<RatingCriteria>(`/api/hackathons/${hackathonId}/criteria`, {
      method: 'POST',
      body: JSON.stringify(data),
    }),
  updateCriteria: (hackathonId: string, criteriaId: string, data: UpdateCriteriaRequest) =>
    fetchApi<RatingCriteria>(`/api/hackathons/${hackathonId}/criteria/${criteriaId}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    }),
  deleteCriteria: (hackathonId: string, criteriaId: string) =>
    fetchApi<null>(`/api/hackathons/${hackathonId}/criteria/${criteriaId}`, {
      method: 'DELETE',
    }),
  reorderCriteria: (hackathonId: string, data: ReorderCriteriaRequest) =>
    fetchApi<null>(`/api/hackathons/${hackathonId}/criteria/reorder`, {
      method: 'POST',
      body: JSON.stringify(data),
    }),

  // Ratings
  getSubmissionsWithRatings: (hackathonId: string) =>
    fetchApi<SubmissionsWithRatingsResponse>(`/api/hackathons/${hackathonId}/submissions/with-ratings`, {
      method: 'GET',
    }),
  createRating: (hackathonId: string, data: CreateRatingRequest) =>
    fetchApi<Rating>(`/api/hackathons/${hackathonId}/ratings`, {
      method: 'POST',
      body: JSON.stringify(data),
    }),
  updateRating: (hackathonId: string, ratingId: string, data: UpdateRatingRequest) =>
    fetchApi<Rating>(`/api/hackathons/${hackathonId}/ratings/${ratingId}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    }),
  deleteRating: (hackathonId: string, ratingId: string) =>
    fetchApi<null>(`/api/hackathons/${hackathonId}/ratings/${ratingId}`, {
      method: 'DELETE',
    }),
  getPublicRatings: (hackathonId: string) =>
    fetchApi<PublicRatingsResponse>(`/api/hackathons/${hackathonId}/ratings/public`, {
      method: 'GET',
    }),
}

// Report types
export interface CreateReportRequest {
  target_type: 'hackathon' | 'organizer' | 'team' | 'user'
  target_id: string
  reason: string
  description?: string
}

export interface Report {
  id: string
  targetType: string
  targetName: string
  reason: string
  description?: string
  status: 'open' | 'resolved' | 'closed'
  createdAt: string
  resolvedAt?: string
}

// Admin types
export interface DashboardStats {
  pending_hackathons_count: number
  unverified_organizers_count: number
  open_reports_count: number
}

export interface PendingHackathon {
  id: string
  title: string
  organizer: { id: string; name: string }
  created_at: string
  description?: string
}

export interface UnverifiedOrganizer {
  id: string
  name: string
  user_id: string
  user_name: string
  email: string
  description?: string
  rejection_reason?: string
  created_at: string
}

export interface AdminReport {
  id: string
  target_type: string
  target_name: string
  reason: string
  status: 'open' | 'resolved' | 'closed'
  created_at: string
  reporter_name: string
}

export interface ReportDetail extends AdminReport {
  description?: string
  reporter: { id: string; name: string; email: string }
  target_id: string
  resolved_by?: { id: string; name: string }
  resolved_at?: string
  resolution_note?: string
}

export interface AdminUser {
  id: string
  email: string
  name: string
  is_staff: boolean
  is_superuser: boolean
  created_at: string
}

// Report API
export const reportApi = {
  create: (data: CreateReportRequest) =>
    fetchApi<Report>('/api/reports', {
      method: 'POST',
      body: JSON.stringify(data),
    }),
  getMyReports: () =>
    fetchApi<Report[]>('/api/reports/my', {
      method: 'GET',
    }),
}

// Admin API - secret hash from env
const ADMIN_SECRET = ((import.meta as unknown) as { env?: { VITE_ADMIN_SECRET?: string } }).env?.VITE_ADMIN_SECRET || '9f2c7b6e5a1d4c8fbbd2a0c3e7f1a6d9'

export const adminApi = {
  // Dashboard
  getDashboardStats: () =>
    fetchApi<DashboardStats>(`/api/admin/${ADMIN_SECRET}/dashboard`, {
      method: 'GET',
    }),

  // Hackathon moderation
  getPendingHackathons: () =>
    fetchApi<{ hackathons: PendingHackathon[]; total: number }>(
      `/api/admin/${ADMIN_SECRET}/hackathons/pending`,
      { method: 'GET' }
    ),
  approveHackathon: (id: string) =>
    fetchApi<null>(`/api/admin/${ADMIN_SECRET}/hackathons/${id}/approve`, {
      method: 'POST',
    }),
  rejectHackathon: (id: string, reason?: string) =>
    fetchApi<null>(`/api/admin/${ADMIN_SECRET}/hackathons/${id}/reject`, {
      method: 'POST',
      body: JSON.stringify({ reason }),
    }),

  // Organizer verification
  getUnverifiedOrganizers: () =>
    fetchApi<{ organizers: UnverifiedOrganizer[]; total: number }>(
      `/api/admin/${ADMIN_SECRET}/organizers/unverified`,
      { method: 'GET' }
    ),
  verifyOrganizer: (id: string) =>
    fetchApi<null>(`/api/admin/${ADMIN_SECRET}/organizers/${id}/verify`, {
      method: 'POST',
    }),
  rejectOrganizer: (id: string, reason: string) =>
    fetchApi<null>(`/api/admin/${ADMIN_SECRET}/organizers/${id}/reject`, {
      method: 'POST',
      body: JSON.stringify({ reason }),
    }),

  // Reports management
  getReports: (status?: 'open' | 'resolved' | 'closed') =>
    fetchApi<{ reports: AdminReport[]; total: number }>(
      `/api/admin/${ADMIN_SECRET}/reports${status ? `?status=${status}` : ''}`,
      { method: 'GET' }
    ),
  getReportDetail: (id: string) =>
    fetchApi<ReportDetail>(`/api/admin/${ADMIN_SECRET}/reports/${id}`, {
      method: 'GET',
    }),
  resolveReport: (id: string, resolutionNote: string) =>
    fetchApi<null>(`/api/admin/${ADMIN_SECRET}/reports/${id}/resolve`, {
      method: 'POST',
      body: JSON.stringify({ resolution_note: resolutionNote }),
    }),
  closeReport: (id: string) =>
    fetchApi<null>(`/api/admin/${ADMIN_SECRET}/reports/${id}/close`, {
      method: 'POST',
    }),

  // User management (superuser only)
  listUsers: () =>
    fetchApi<{ users: AdminUser[]; total: number }>(`/api/admin/${ADMIN_SECRET}/users`, {
      method: 'GET',
    }),
  toggleUserStaff: (id: string) =>
    fetchApi<null>(`/api/admin/${ADMIN_SECRET}/users/${id}/toggle-staff`, {
      method: 'POST',
    }),
}

export interface MyHackathonWithStatus {
  id: string
  title: string
  bannerUrl?: string
  status: 'pending' | 'approved' | 'rejected'
  isPublished: boolean
  createdAt: string
  eventStart: string
  eventEnd: string
}

export { fetchApi, fetchOptional }
