import { ref, computed } from 'vue'
import { userApi, type UserProfile, type Skill, type AvailableSkill } from '@/services/api'

export interface ProfileFormData {
  name: string
  bio: string
  githubUrl: string
  telegramUsername: string
}

class ProfileManager {
  private _profile = ref<UserProfile | null>(null)
  private _isLoading = ref(false)
  private _isSaving = ref(false)
  private _errors = ref<Record<string, string>>({})
  private _availableSkills = ref<AvailableSkill[]>([])

  get profile() {
    return computed(() => this._profile.value)
  }

  get isLoading() {
    return computed(() => this._isLoading.value)
  }

  get isSaving() {
    return computed(() => this._isSaving.value)
  }

  get errors() {
    return computed(() => this._errors.value)
  }

  get availableSkills() {
    return computed(() => this._availableSkills.value)
  }

  get fullName() {
    return computed(() => this._profile.value?.name ?? '')
  }

  get initials() {
    return computed(() => {
      const name = this._profile.value?.name ?? ''
      if (!name) return '?'
      return name
        .split(' ')
        .map(n => n[0])
        .slice(0, 2)
        .join('')
        .toUpperCase()
    })
  }

  get joinedDate() {
    return computed(() => {
      if (!this._profile.value?.createdAt) return ''
      const date = new Date(this._profile.value.createdAt)
      return new Intl.DateTimeFormat('ru-RU', {
        year: 'numeric',
        month: 'long',
      }).format(date)
    })
  }

  get hasSocialLinks() {
    return computed(() => {
      const profile = this._profile.value
      return !!(profile?.githubUrl || profile?.telegramUsername)
    })
  }

  get skillsByCategory() {
    return computed(() => {
      const skills = this._profile.value?.skills ?? []
      const grouped: Record<string, Skill[]> = {}

      skills.forEach(skill => {
        if (!grouped[skill.category]) {
          grouped[skill.category] = []
        }
        grouped[skill.category].push(skill)
      })

      return grouped
    })
  }

  get skillCategories() {
    return computed(() => {
      const grouped = this.skillsByCategory.value
      return Object.keys(grouped || {})
    })
  }

  async loadProfile(): Promise<boolean> {
    this._isLoading.value = true
    this._errors.value = {}

    const response = await userApi.getMe()

    if (response.error) {
      this._errors.value = { general: response.error }
      this._isLoading.value = false
      return false
    }

    if (response.data) {
      this._profile.value = response.data
    }

    this._isLoading.value = false
    return true
  }

  async updateProfile(data: ProfileFormData): Promise<boolean> {
    this._isSaving.value = true
    this._errors.value = {}

    const response = await userApi.updateMe({
      name: data.name || undefined,
      bio: data.bio || undefined,
      githubUrl: data.githubUrl || undefined,
      telegramUsername: data.telegramUsername || undefined,
    })

    if (response.error) {
      this._errors.value = { general: response.error }
      this._isSaving.value = false
      return false
    }

    if (response.data) {
      this._profile.value = response.data
    }

    this._isSaving.value = false
    return true
  }

  async loadAvailableSkills(): Promise<boolean> {
    const response = await userApi.getAvailableSkills()
    if (response.error) return false
    if (response.data) {
      this._availableSkills.value = response.data
    }
    return true
  }

  async addSkill(skillId: string, level: number): Promise<boolean> {
    const response = await userApi.addSkill({ skillId, level })

    if (response.error) {
      return false
    }

    if (response.data && this._profile.value) {
      this._profile.value.skills = response.data
    }

    return true
  }

  async removeSkill(skillId: string): Promise<boolean> {
    const response = await userApi.removeSkill({ skillId })

    if (response.error) {
      return false
    }

    if (response.data && this._profile.value) {
      this._profile.value.skills = response.data
    }

    return true
  }

  async updateSkillLevel(skillId: string, level: number): Promise<boolean> {
    const response = await userApi.updateSkillLevel(skillId, { level })

    if (response.error) {
      return false
    }

    if (response.data && this._profile.value) {
      this._profile.value.skills = response.data
    }

    return true
  }

  clearErrors() {
    this._errors.value = {}
  }

  clearProfile() {
    this._profile.value = null
    this._errors.value = {}
  }
}

const manager = new ProfileManager()

export function useProfile() {
  return {
    profile: manager.profile,
    isLoading: manager.isLoading,
    isSaving: manager.isSaving,
    errors: manager.errors,
    fullName: manager.fullName,
    initials: manager.initials,
    joinedDate: manager.joinedDate,
    hasSocialLinks: manager.hasSocialLinks,
    skillsByCategory: manager.skillsByCategory,
    skillCategories: manager.skillCategories,
    availableSkills: manager.availableSkills,
    loadProfile: manager.loadProfile.bind(manager),
    loadAvailableSkills: manager.loadAvailableSkills.bind(manager),
    updateProfile: manager.updateProfile.bind(manager),
    addSkill: manager.addSkill.bind(manager),
    removeSkill: manager.removeSkill.bind(manager),
    updateSkillLevel: manager.updateSkillLevel.bind(manager),
    clearErrors: manager.clearErrors.bind(manager),
    clearProfile: manager.clearProfile.bind(manager),
  }
}
