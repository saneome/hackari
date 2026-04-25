<script setup lang="ts">
import { ref, computed, onMounted, watch, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { gsap } from 'gsap'
import { ScrollTrigger } from 'gsap/ScrollTrigger'
import { Calendar, MapPin, Users, Trophy, Mail, Globe, ChevronRight, Plus, UserPlus, X, ExternalLink, Github, Play, FileText, Crown, LogOut, Trash2, Edit3, Check, CheckCircle, Flag } from 'lucide-vue-next'
import { hackathonApi, organizerApi, fetchApi, userApi, teamApi } from '@/services/api'
import { useAuth } from '@/composables/useAuth'
import { useModal } from '@/composables/useModal'
import CustomSelect from '@/components/CustomSelect.vue'
import ReportModal from '@/components/ReportModal.vue'
import NotFoundPage from '@/components/NotFoundPage.vue'
import type { Hackathon, TeamDetail } from '@/services/api'

gsap.registerPlugin(ScrollTrigger)

const props = defineProps<{
  hackathonId: string
}>()

const router = useRouter()
const { user, isAuthenticated } = useAuth()
const { alert, confirm } = useModal()

// Data
const hackathon = ref<Hackathon | null>(null)
const isLoading = ref(true)
const error = ref('')
const notFound = ref(false)
const userOrganizerId = ref<string | null>(null)
const userTeam = ref<{ id: string; name: string; role: string } | null>(null)
const teams = ref<{ id: string; name: string; member_count: number; track_name?: string }[]>([])
const isCreatingTeam = ref(false)
const isJoiningTeam = ref<string | null>(null)

// New team form
const newTeamName = ref('')
const newTeamDescription = ref('')
const selectedTrackId = ref('')
const showCreateTeamModal = ref(false)

// Report modal
const showReportModal = ref(false)

// Team detail modal
const selectedTeam = ref<TeamDetail | null>(null)
const showTeamModal = ref(false)
const isLoadingTeam = ref(false)
const isEditingTeam = ref(false)
const isSavingTeam = ref(false)
const isLeavingTeam = ref(false)
const isDeletingTeam = ref(false)

// Edit team form
const editTeamName = ref('')
const editTeamDescription = ref('')
const editTeamTrackId = ref('')
const editTeamRepoUrl = ref('')
const editTeamDemoUrl = ref('')
const editTeamPresentationUrl = ref('')

// Computed
const isOrganizer = computed(() => {
  if (!hackathon.value?.organizer || !userOrganizerId.value) return false
  return hackathon.value.organizer.id === userOrganizerId.value
})

const canCreateOrJoinTeam = computed(() => {
  if (!isAuthenticated.value) return false
  if (isOrganizer.value) return false
  if (userTeam.value) return false
  if (hackathon.value?.status === 'cancelled') return false
  return true
})

const formattedDateRange = computed(() => {
  if (!hackathon.value) return ''
  const start = new Date(hackathon.value.event_start)
  const end = new Date(hackathon.value.event_end)
  const options: Intl.DateTimeFormatOptions = { day: 'numeric', month: 'long' }
  return `${start.toLocaleDateString('ru-RU', options)} — ${end.toLocaleDateString('ru-RU', options)}`
})

const formattedRegistrationPeriod = computed(() => {
  if (!hackathon.value) return ''
  const start = new Date(hackathon.value.registration_start)
  const end = new Date(hackathon.value.registration_end)
  const now = new Date()

  if (now < start) {
    return `Регистрация начнется ${start.toLocaleDateString('ru-RU', { day: 'numeric', month: 'long' })}`
  } else if (now > end) {
    return 'Регистрация закрыта'
  } else {
    return `Регистрация до ${end.toLocaleDateString('ru-RU', { day: 'numeric', month: 'long' })}`
  }
})

const isCancelled = computed(() => hackathon.value?.status === 'cancelled')

const isRegistrationOpen = computed(() => {
  if (!hackathon.value) return false
  if (isCancelled.value) return false
  const now = new Date()
  const start = new Date(hackathon.value.registration_start)
  const end = new Date(hackathon.value.registration_end)
  return now >= start && now <= end
})

const locationText = computed(() => {
  if (!hackathon.value) return ''
  const loc = hackathon.value.location_type
  if (loc === 'online') return 'Онлайн'
  if (loc === 'offline') {
    const city = hackathon.value.city
    const venue = hackathon.value.venue
    return [city, venue].filter(Boolean).join(', ') || 'Офлайн'
  }
  if (loc === 'hybrid') {
    const city = hackathon.value.city
    return city ? `Гибрид (${city})` : 'Гибрид'
  }
  return loc
})

// Methods
const fetchData = async () => {
  isLoading.value = true
  error.value = ''
  notFound.value = false

  try {
    // Fetch hackathon details
    const response = await hackathonApi.getById(props.hackathonId)
    if (response.data) {
      hackathon.value = response.data
    } else {
      if (response.status === 404) {
        notFound.value = true
      } else {
        error.value = response.error || 'Хакатон не найден'
      }
      return
    }

    // Fetch user's organizer profile if authenticated (404 is ok - not an organizer yet)
    if (isAuthenticated.value) {
      const orgResponse = await organizerApi.getMyOrganizer()
      if (orgResponse.data) {
        userOrganizerId.value = orgResponse.data.id
      }

      // Fetch user's team in this hackathon (404 is ok - not in a team)
      await fetchUserTeam()
    }

    // Fetch teams
    await fetchTeams()
  } catch (e) {
    error.value = 'Ошибка при загрузке данных'
    console.error(e)
  } finally {
    isLoading.value = false
  }
}

const fetchUserTeam = async () => {
  const response = await userApi.getMyTeamForHackathon(props.hackathonId)
  if (response.data) {
    userTeam.value = response.data
  } else {
    userTeam.value = null
  }
}

const fetchTeams = async () => {
  try {
    const response = await fetchApi<{ id: string; name: string; description?: string; member_count: number; track: { name: string } | null }[]>(`/api/hackathons/${props.hackathonId}/teams`)
    if (response.data) {
      teams.value = response.data.map(t => ({
        id: t.id,
        name: t.name,
        member_count: t.member_count || 0,
        track_name: t.track?.name
      }))
    }
  } catch {
    teams.value = []
  }
}

const openCreateTeamModal = () => {
  if (!canCreateOrJoinTeam.value || !isRegistrationOpen.value) return
  showCreateTeamModal.value = true
}

const closeCreateTeamModal = () => {
  showCreateTeamModal.value = false
  newTeamName.value = ''
  newTeamDescription.value = ''
  selectedTrackId.value = ''
}

const createTeam = async () => {
  if (!newTeamName.value.trim()) return
  if (isOrganizer.value) {
    await alert({
      title: 'Действие недоступно',
      message: 'Организатор не может участвовать в хакатоне',
      type: 'warning'
    })
    return
  }

  isCreatingTeam.value = true
  try {
    const response = await fetchApi<{ id: string }>(`/api/teams`, {
      method: 'POST',
      body: JSON.stringify({
        hackathon_id: props.hackathonId,
        name: newTeamName.value,
        description: newTeamDescription.value || undefined,
        track_id: selectedTrackId.value || undefined
      })
    })

    if (response.data) {
      userTeam.value = { id: response.data.id, name: newTeamName.value, role: 'leader' }
      await fetchTeams()
      closeCreateTeamModalAnimated()
    }
  } catch (e: any) {
    await alert({
      title: 'Ошибка',
      message: e?.error || 'Ошибка при создании команды',
      type: 'error'
    })
  } finally {
    isCreatingTeam.value = false
  }
}

const joinTeam = async (teamId: string) => {
  if (!canCreateOrJoinTeam.value || !isRegistrationOpen.value) return
  if (isOrganizer.value) {
    await alert({
      title: 'Действие недоступно',
      message: 'Организатор не может участвовать в хакатоне',
      type: 'warning'
    })
    return
  }

  isJoiningTeam.value = teamId
  try {
    const response = await fetchApi<{ id: string }>(`/api/teams/${teamId}/join`, {
      method: 'POST',
      body: JSON.stringify({})
    })

    if (response.data) {
      await fetchUserTeam()
      await fetchTeams()
    }
  } catch (e: any) {
    await alert({
      title: 'Ошибка',
      message: e?.error || 'Ошибка при присоединении к команде',
      type: 'error'
    })
  } finally {
    isJoiningTeam.value = null
  }
}

const openTeamModal = async (teamId: string) => {
  showTeamModal.value = true
  isLoadingTeam.value = true
  selectedTeam.value = null
  isEditingTeam.value = false

  try {
    const response = await teamApi.getById(teamId)
    if (response.data) {
      selectedTeam.value = response.data
    } else {
      showTeamModal.value = false
    }
  } catch (e) {
    console.error(e)
    showTeamModal.value = false
  } finally {
    isLoadingTeam.value = false
  }
}

const closeTeamModal = () => {
  showTeamModal.value = false
  selectedTeam.value = null
  isEditingTeam.value = false
}

const startEditingTeam = () => {
  if (!selectedTeam.value) return
  editTeamName.value = selectedTeam.value.name
  editTeamDescription.value = selectedTeam.value.description || ''
  editTeamTrackId.value = selectedTeam.value.track?.id || ''
  editTeamRepoUrl.value = selectedTeam.value.repo_url || ''
  editTeamDemoUrl.value = selectedTeam.value.demo_url || ''
  editTeamPresentationUrl.value = selectedTeam.value.presentation_url || ''
  isEditingTeam.value = true
}

const cancelEditingTeam = () => {
  isEditingTeam.value = false
}

const saveTeamChanges = async () => {
  if (!selectedTeam.value) return
  isSavingTeam.value = true

  try {
    const response = await teamApi.update(selectedTeam.value.id, {
      name: editTeamName.value,
      description: editTeamDescription.value || undefined,
      track_id: editTeamTrackId.value || undefined,
      repo_url: editTeamRepoUrl.value || undefined,
      demo_url: editTeamDemoUrl.value || undefined,
      presentation_url: editTeamPresentationUrl.value || undefined,
    })

    if (response.data) {
      selectedTeam.value = response.data
      isEditingTeam.value = false
      // Refresh teams list
      await fetchTeams()
      // Update userTeam if it's the same team
      if (userTeam.value?.id === selectedTeam.value.id) {
        userTeam.value = { ...userTeam.value, name: response.data.name }
      }
    }
  } catch (e: any) {
    await alert({
      title: 'Ошибка',
      message: e?.error || 'Ошибка при сохранении',
      type: 'error'
    })
  } finally {
    isSavingTeam.value = false
  }
}

const leaveTeam = async () => {
  if (!selectedTeam.value) return
  const shouldLeave = await confirm({
    title: 'Покинуть команду',
    message: 'Вы уверены, что хотите покинуть команду?',
    type: 'warning',
    confirmText: 'Покинуть',
    cancelText: 'Отмена'
  })
  if (!shouldLeave) return

  isLeavingTeam.value = true
  const leftTeamId = selectedTeam.value.id

  try {
    const response = await teamApi.leave(selectedTeam.value.id)
    if (response.data?.success) {
      // Close modal first
      showTeamModal.value = false
      selectedTeam.value = null
      isEditingTeam.value = false

      if (response.data.team_disbanded) {
        // Remove team from list immediately
        teams.value = teams.value.filter(t => t.id !== leftTeamId)
        userTeam.value = null
        await alert({
          title: 'Команда расформирована',
          message: 'Команда расформирована, так как вы были последним участником',
          type: 'info'
        })
      } else if (response.data.new_leader) {
        userTeam.value = null
        await alert({
          title: 'Успешно',
          message: 'Вы покинули команду. Новый лидер назначен.',
          type: 'success'
        })
      }

      // Sync with server
      await fetchTeams()
    }
  } catch (e: any) {
    await alert({
      title: 'Ошибка',
      message: e?.error || 'Ошибка при выходе из команды',
      type: 'error'
    })
  } finally {
    isLeavingTeam.value = false
    // Ensure body scroll is unlocked
    unlockBodyScroll()
  }
}

const deleteTeam = async () => {
  if (!selectedTeam.value) return
  const shouldDelete = await confirm({
    title: 'Удалить команду',
    message: 'Вы уверены, что хотите удалить команду? Это действие нельзя отменить.',
    type: 'error',
    confirmText: 'Удалить',
    cancelText: 'Отмена'
  })
  if (!shouldDelete) return

  isDeletingTeam.value = true
  const deletedTeamId = selectedTeam.value.id

  // Store reference before closing modal clears it
  const wasUserTeam = userTeam.value?.id === deletedTeamId

  try {
    // Close modal first, then delete
    showTeamModal.value = false
    selectedTeam.value = null
    isEditingTeam.value = false

    const response = await teamApi.delete(deletedTeamId)

    if (!response.error) {
      // Update local state immediately
      if (wasUserTeam) {
        userTeam.value = null
      }

      // Force new array reference for reactivity
      teams.value = teams.value.filter(t => t.id !== deletedTeamId)

      // Sync with server
      await fetchTeams()
    }
  } catch (e: any) {
    await alert({
      title: 'Ошибка',
      message: e?.error || 'Ошибка при удалении команды',
      type: 'error'
    })
  } finally {
    isDeletingTeam.value = false
    // Ensure body scroll is unlocked
    unlockBodyScroll()
  }
}

const isCurrentUserLeader = computed(() => {
  if (!selectedTeam.value || !user.value) return false
  const member = selectedTeam.value.members.find(m => m.user_id === user.value?.id)
  return member?.role === 'leader'
})

const isCurrentUserMember = computed(() => {
  if (!selectedTeam.value || !user.value) return false
  return selectedTeam.value.members.some(m => m.user_id === user.value?.id)
})

const goToLogin = () => {
  router.push('/auth')
}

// Animations
const animateSections = () => {
  const sections = document.querySelectorAll('.animate-section')
  sections.forEach((section) => {
    gsap.fromTo(section,
      { y: 40, opacity: 0 },
      {
        y: 0,
        opacity: 1,
        duration: 0.6,
        ease: 'power2.out',
        scrollTrigger: {
          trigger: section,
          start: 'top 80%',
          toggleActions: 'play none none none'
        }
      }
    )
  })
}

// Scroll lock management
let scrollY = 0
const lockBodyScroll = () => {
  scrollY = window.scrollY
  document.body.style.position = 'fixed'
  document.body.style.top = `-${scrollY}px`
  document.body.style.left = '0'
  document.body.style.right = '0'
  document.body.style.overflow = 'hidden'
  document.body.classList.add('modal-open')
}

const unlockBodyScroll = () => {
  document.body.style.position = ''
  document.body.style.top = ''
  document.body.style.left = ''
  document.body.style.right = ''
  document.body.style.overflow = ''
  document.body.classList.remove('modal-open')
  window.scrollTo(0, scrollY)
}

// Modal animations and scroll lock
const animateModalOpen = (modalSelector: string) => {
  const overlay = document.querySelector(modalSelector)
  if (!overlay) return

  const modal = overlay.querySelector('.modal')
  if (!modal) return

  // Block body scroll
  lockBodyScroll()

  // Animate overlay
  gsap.fromTo(overlay,
    { opacity: 0 },
    { opacity: 1, duration: 0.2, ease: 'power2.out' }
  )

  // Animate modal
  gsap.fromTo(modal,
    { y: 30, opacity: 0, scale: 0.95 },
    { y: 0, opacity: 1, scale: 1, duration: 0.35, ease: 'back.out(1.2)' }
  )
}

const animateModalClose = (modalSelector: string, callback?: () => void) => {
  const overlay = document.querySelector(modalSelector)
  if (!overlay) {
    unlockBodyScroll()
    callback?.()
    return
  }

  const modal = overlay.querySelector('.modal')

  // Animate out
  const tl = gsap.timeline({
    onComplete: () => {
      unlockBodyScroll()
      callback?.()
    }
  })

  if (modal) {
    tl.to(modal, { y: 20, opacity: 0, scale: 0.98, duration: 0.2, ease: 'power2.in' })
  }
  tl.to(overlay, { opacity: 0, duration: 0.15, ease: 'power2.in' }, '<')
}

// Watchers for modal animations
watch(showCreateTeamModal, (newVal) => {
  if (newVal) {
    nextTick(() => animateModalOpen('.modal-overlay:not(.team-modal-overlay)'))
  }
})

watch(showTeamModal, (newVal) => {
  if (newVal) {
    nextTick(() => animateModalOpen('.team-modal-overlay'))
  }
})

// Updated close functions with animation
const closeCreateTeamModalAnimated = () => {
  animateModalClose('.modal-overlay:not(.team-modal-overlay)', () => {
    showCreateTeamModal.value = false
    newTeamName.value = ''
    newTeamDescription.value = ''
    selectedTrackId.value = ''
  })
}

const closeTeamModalAnimated = () => {
  animateModalClose('.team-modal-overlay', () => {
    showTeamModal.value = false
    selectedTeam.value = null
    isEditingTeam.value = false
  })
}

// Format description: insert soft hyphens for long words
const formatDescription = (text: string): string => {
  return text.replace(/(\S{15,})/g, (match) => {
    return match.split('').join('&shy;')
  })
}

onMounted(() => {
  fetchData()
  setTimeout(animateSections, 100)
})
</script>

<template>
  <div class="hackathon-detail-page">
    <!-- Loading State -->
    <div v-if="isLoading" class="loading-container">
      <div class="spinner"></div>
      <span>Загрузка...</span>
    </div>

    <!-- Not Found -->
    <NotFoundPage v-else-if="notFound" />

    <!-- Error State -->
    <div v-else-if="error" class="error-container">
      <p>{{ error }}</p>
      <router-link to="/hackathons" class="back-link">
        ← Вернуться к списку хакатонов
      </router-link>
    </div>

    <!-- Content -->
    <template v-else-if="hackathon">
      <div v-if="isCancelled" class="cancelled-banner">
        <strong>Хакатон отменён организатором.</strong>
        <span>Регистрация закрыта. Если у вас остались вопросы — свяжитесь с организатором.</span>
      </div>
      <div v-else-if="hackathon.status === 'pending'" class="moderation-banner pending">
        <strong>Хакатон на модерации.</strong>
        <span>Эта страница видна только вам и администраторам, пока хакатон не пройдёт проверку.</span>
      </div>
      <div v-else-if="hackathon.status === 'rejected'" class="moderation-banner rejected">
        <strong>Хакатон отклонён модератором.</strong>
        <span>Внесите изменения и сохраните — хакатон автоматически вернётся на повторную проверку.</span>
      </div>
      <!-- Hero Section -->
      <section class="hero-section">
        <div v-if="hackathon.banner_url" class="hero-banner">
          <img :src="hackathon.banner_url" alt="" />
          <div class="banner-overlay"></div>
        </div>
        <div class="hero-content">
          <div class="container">
            <div class="hero-badge" :class="{ 'open': isRegistrationOpen }">
              {{ formattedRegistrationPeriod }}
            </div>
            <h1 class="hero-title">{{ hackathon.title }}</h1>
            <div class="hero-meta">
              <div class="meta-item">
                <Calendar :size="18" />
                <span>{{ formattedDateRange }}</span>
              </div>
              <div class="meta-item">
                <MapPin :size="18" />
                <span>{{ locationText }}</span>
              </div>
              <div class="meta-item">
                <Users :size="18" />
                <span>{{ hackathon.team_count }} команд · {{ hackathon.participant_count }} участников</span>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- Main Content -->
      <div class="content-wrapper">
        <!-- Organizer Notice -->
        <div v-if="isOrganizer" class="organizer-notice animate-section">
          <div class="container">
            <div class="notice-content">
              <span class="notice-icon">👑</span>
              <p>Вы организатор этого хакатона. Организаторы не могут участвовать в хакатоне.</p>
            </div>
          </div>
        </div>

        <!-- Description -->
        <section v-if="hackathon.description" class="section description-section animate-section">
          <div class="container">
            <h2 class="section-title">Описание</h2>
            <div class="description-content">
              <p v-html="formatDescription(hackathon.description)"></p>
            </div>
          </div>
        </section>

        <!-- Prize Pool -->
        <section v-if="hackathon.prize_pool" class="section prize-section animate-section">
          <div class="container">
            <div class="prize-card">
              <div class="prize-icon-wrapper">
                <Trophy :size="32" />
              </div>
              <div class="prize-info">
                <span class="prize-label">Призовой фонд</span>
                <span class="prize-value">
                  {{ hackathon.prize_pool }}
                  <span v-if="hackathon.prize_currency" class="prize-currency">
                    {{ hackathon.prize_currency }}
                  </span>
                </span>
                <p v-if="hackathon.prize_description" class="prize-description">
                  {{ hackathon.prize_description }}
                </p>
              </div>
            </div>
          </div>
        </section>

        <!-- Tracks -->
        <section v-if="hackathon.tracks?.length" class="section tracks-section animate-section">
          <div class="container">
            <h2 class="section-title">Треки</h2>
            <div class="tracks-grid">
              <div
                v-for="track in hackathon.tracks"
                :key="track.id"
                class="track-card"
              >
                <h3 class="track-name">{{ track.name }}</h3>
                <p v-if="track.description" class="track-description">
                  {{ track.description }}
                </p>
                <div v-if="track.prize_description" class="track-prize">
                  <Trophy :size="14" />
                  <span>{{ track.prize_description }}</span>
                </div>
                <div v-if="track.max_teams" class="track-limit">
                  Максимум команд: {{ track.max_teams }}
                </div>
              </div>
            </div>
          </div>
        </section>

        <!-- Teams -->
        <section class="section teams-section animate-section">
          <div class="container">
            <div class="section-header">
              <h2 class="section-title">Команды</h2>
              <div class="teams-actions">
                <!-- User has team -->
          <button
            v-if="userTeam"
            class="btn btn-primary"
            @click="openTeamModal(userTeam.id)"
          >
            Моя команда
            <ChevronRight :size="16" />
          </button>

                <!-- Create/Join buttons -->
                <template v-else-if="isAuthenticated">
                  <button
                    v-if="canCreateOrJoinTeam && isRegistrationOpen"
                    class="btn btn-primary"
                    @click="openCreateTeamModal"
                  >
                    <Plus :size="18" />
                    Создать команду
                  </button>
                  <span v-else-if="isOrganizer" class="restricted-text">
                    Организатор не может участвовать
                  </span>
                  <span v-else-if="!isRegistrationOpen" class="restricted-text">
                    Регистрация закрыта
                  </span>
                </template>

                <!-- Not logged in -->
                <button
                  v-else
                  class="btn btn-primary"
                  @click="goToLogin"
                >
                  Войти, чтобы участвовать
                </button>
              </div>
            </div>

            <!-- Teams List -->
            <div class="teams-list">
        <div
          v-for="team in teams"
          :key="team.id"
          class="team-card"
          :class="{ 'my-team': userTeam?.id === team.id }"
          @click="openTeamModal(team.id)"
        >
                <div class="team-info">
                  <h4 class="team-name">{{ team.name }}</h4>
                  <span v-if="team.track_name" class="team-track">{{ team.track_name }}</span>
                </div>
                <div class="team-stats">
                  <div class="team-members">
                    <Users :size="14" />
                    <span>{{ team.member_count }}</span>
                  </div>
                  <button
                    v-if="canCreateOrJoinTeam && isRegistrationOpen && userTeam?.id !== team.id"
                    class="btn-join"
                    :disabled="isJoiningTeam === team.id"
                    @click.stop="joinTeam(team.id)"
                  >
                    <UserPlus v-if="isJoiningTeam !== team.id" :size="14" />
                    <span v-else class="join-spinner"></span>
                    <span>{{ isJoiningTeam === team.id ? '...' : 'Вступить' }}</span>
                  </button>
                </div>
              </div>

              <div v-if="!teams.length" class="teams-empty">
                <p>Пока нет команд</p>
                <span v-if="canCreateOrJoinTeam && isRegistrationOpen">Станьте первым!</span>
              </div>
            </div>
          </div>
        </section>

        <!-- Deadlines -->
        <section v-if="hackathon.deadlines?.length" class="section deadlines-section animate-section">
          <div class="container">
            <h2 class="section-title">Этапы и дедлайны</h2>
            <div class="deadlines-timeline">
          <div
            v-for="deadline in hackathon.deadlines"
            :key="deadline.id"
            class="deadline-item"
            :class="{ 'milestone': deadline.is_milestone }"
          >
                <div class="deadline-marker"></div>
                <div class="deadline-content">
                  <div class="deadline-header">
                    <h4 class="deadline-name">{{ deadline.name }}</h4>
                    <span v-if="deadline.is_milestone" class="milestone-badge">Майлстоун</span>
                  </div>
                  <p v-if="deadline.description" class="deadline-description">
                    {{ deadline.description }}
                  </p>
                  <span class="deadline-date">
                    {{ new Date(deadline.deadline_at).toLocaleDateString('ru-RU', {
                      day: 'numeric',
                      month: 'long',
                      hour: '2-digit',
                      minute: '2-digit'
                    }) }}
                  </span>
                </div>
              </div>
            </div>
          </div>
        </section>

        <!-- Requirements -->
        <section v-if="hackathon.requirements" class="section requirements-section animate-section">
          <div class="container">
            <h2 class="section-title">Требования к участникам</h2>
            <div class="requirements-card">
              <p>{{ hackathon.requirements }}</p>
              <div v-if="hackathon.team_size_min || hackathon.team_size_max" class="team-size-info">
                <Users :size="16" />
                <span>
                  Размер команды:
                  <strong>
                    {{ hackathon.team_size_min || 1 }}
                    {{ hackathon.team_size_max ? `— ${hackathon.team_size_max}` : '+' }}
                    человек
                  </strong>
                </span>
              </div>
              <div v-if="hackathon.age_restriction" class="age-info">
                <span>Возрастное ограничение: <strong>{{ hackathon.age_restriction }}</strong></span>
              </div>
            </div>
          </div>
        </section>

        <!-- Skills -->
        <section v-if="hackathon.skills?.length" class="section skills-section animate-section">
          <div class="container">
            <h2 class="section-title">Компетенции хакатона</h2>
            <div class="skills-cloud">
              <span
                v-for="skill in hackathon.skills"
                :key="skill.id"
                class="skill-tag"
              >
                {{ skill.name }}
              </span>
            </div>
          </div>
        </section>

        <!-- Organizer -->
        <section v-if="hackathon.organizer" class="section organizer-section animate-section">
          <div class="container">
            <h2 class="section-title">Организатор</h2>
            <div class="organizer-card">
              <div class="organizer-avatar">
                <img v-if="hackathon.organizer.avatar_url" :src="hackathon.organizer.avatar_url" alt="" />
                <span v-else class="avatar-placeholder">{{ hackathon.organizer.name[0] }}</span>
              </div>
              <div class="organizer-info">
                <div class="organizer-name-row">
            <span class="organizer-name">{{ hackathon.organizer.name }}</span>
            <span v-if="hackathon.organizer.is_verified" class="verified-badge" title="Верифицированный организатор">
              <CheckCircle :size="14" />
            </span>
          </div>
                <div v-if="hackathon.contact_email" class="organizer-contact">
                  <Mail :size="14" />
                  <a :href="`mailto:${hackathon.contact_email}`">{{ hackathon.contact_email }}</a>
                </div>
                <a
                  v-if="hackathon.website_url"
                  :href="hackathon.website_url"
                  target="_blank"
                  rel="noopener"
                  class="organizer-link"
                >
                  <Globe :size="14" />
                  <span>Сайт хакатона</span>
                </a>
              </div>
            </div>
          </div>
        </section>
  <!-- Report Section -->
  <section class="section report-section animate-section">
    <div class="container">
      <div class="report-card">
        <div class="report-icon">
          <Flag :size="24" />
        </div>
        <div class="report-info">
          <span class="report-title">Заметили нарушение?</span>
          <span class="report-description">Сообщите нам о проблеме с этим хакатоном</span>
        </div>
        <button class="btn btn-secondary" @click="showReportModal = true">
          <Flag :size="16" />
          Пожаловаться
        </button>
      </div>
    </div>
  </section>
      </div>
    </template>

<!-- Create Team Modal -->
<div v-if="showCreateTeamModal" class="modal-overlay" @click.self="closeCreateTeamModalAnimated">
  <div class="modal">
    <div class="modal-header">
      <h3>Создать команду</h3>
      <button class="modal-close" @click="closeCreateTeamModalAnimated">×</button>
    </div>
    <div class="modal-body">
      <div class="form-field">
        <label>Название команды <span class="required">*</span></label>
        <input v-model="newTeamName" type="text" placeholder="Например: Team Alpha" />
      </div>
      <div class="form-field">
        <label>Описание</label>
        <textarea v-model="newTeamDescription" rows="3" placeholder="Краткое описание вашей команды..."></textarea>
      </div>
<div v-if="hackathon?.tracks?.length" class="form-field">
            <label>Трек</label>
            <CustomSelect
              v-model="selectedTrackId"
              :options="[{ value: '', label: 'Без трека' }, ...hackathon.tracks.map(t => ({ value: t.id, label: t.name }))]"
              placeholder="Выберите трек"
            />
          </div>
      <div v-if="isOrganizer" class="form-error">
        ⚠️ Организатор не может создавать команды в своем хакатоне
      </div>
    </div>
    <div class="modal-footer">
      <button class="btn btn-secondary" @click="closeCreateTeamModalAnimated">Отмена</button>
      <button
        class="btn btn-primary"
        :disabled="!newTeamName.trim() || isCreatingTeam || isOrganizer"
        @click="createTeam"
      >
        <span v-if="isCreatingTeam" class="spinner"></span>
        <template v-else>Создать</template>
      </button>
    </div>
  </div>
</div>

<!-- Team Detail Modal -->
<div v-if="showTeamModal" class="modal-overlay team-modal-overlay" @click.self="closeTeamModalAnimated">
  <div class="modal team-modal">
    <!-- Loading State -->
    <div v-if="isLoadingTeam" class="team-modal-loading">
      <div class="spinner"></div>
      <span>Загрузка...</span>
    </div>

    <!-- Team Content -->
    <template v-else-if="selectedTeam">
      <!-- Modal Header -->
      <div class="team-modal-header">
        <div class="team-modal-title-section">
          <div v-if="!isEditingTeam" class="team-name-row">
            <h2 class="team-modal-title">{{ selectedTeam.name }}</h2>
            <button
              v-if="isCurrentUserLeader"
              class="btn-icon"
              @click="startEditingTeam"
              title="Редактировать"
            >
              <Edit3 :size="18" />
            </button>
          </div>
          <div v-else class="team-name-edit">
            <input
              v-model="editTeamName"
              type="text"
              class="team-name-input"
              placeholder="Название команды"
            />
          </div>
          <div v-if="selectedTeam.track" class="team-track-badge">
            {{ selectedTeam.track.name }}
          </div>
        </div>
        <button class="modal-close" @click="closeTeamModalAnimated">
          <X :size="24" />
        </button>
      </div>

      <!-- Modal Body -->
      <div class="team-modal-body">
        <!-- Description Section -->
        <div class="team-section">
          <h4 class="section-label">Описание</h4>
          <p v-if="!isEditingTeam" class="team-description">
            {{ selectedTeam.description || 'Нет описания' }}
          </p>
          <textarea
            v-else
            v-model="editTeamDescription"
            rows="3"
            class="team-textarea"
            placeholder="Описание команды..."
          ></textarea>
        </div>

        <!-- Links Section (only show if leader or has links) -->
        <div v-if="!isEditingTeam && (selectedTeam.repo_url || selectedTeam.demo_url || selectedTeam.presentation_url)" class="team-section">
          <h4 class="section-label">Ссылки</h4>
          <div class="team-links">
            <a
              v-if="selectedTeam.repo_url"
              :href="selectedTeam.repo_url"
              target="_blank"
              rel="noopener"
              class="team-link"
            >
              <Github :size="16" />
              <span>Репозиторий</span>
              <ExternalLink :size="14" />
            </a>
            <a
              v-if="selectedTeam.demo_url"
              :href="selectedTeam.demo_url"
              target="_blank"
              rel="noopener"
              class="team-link"
            >
              <Play :size="16" />
              <span>Демо</span>
              <ExternalLink :size="14" />
            </a>
            <a
              v-if="selectedTeam.presentation_url"
              :href="selectedTeam.presentation_url"
              target="_blank"
              rel="noopener"
              class="team-link"
            >
              <FileText :size="16" />
              <span>Презентация</span>
              <ExternalLink :size="14" />
            </a>
          </div>
        </div>

        <!-- Edit Links Section -->
        <div v-if="isEditingTeam" class="team-section">
          <h4 class="section-label">Ссылки</h4>
          <div class="edit-links">
            <div class="form-field">
              <label><Github :size="14" /> Репозиторий</label>
              <input v-model="editTeamRepoUrl" type="url" placeholder="https://github.com/..." />
            </div>
            <div class="form-field">
              <label><Play :size="14" /> Демо</label>
              <input v-model="editTeamDemoUrl" type="url" placeholder="https://..." />
            </div>
            <div class="form-field">
              <label><FileText :size="14" /> Презентация</label>
              <input v-model="editTeamPresentationUrl" type="url" placeholder="https://..." />
            </div>
          </div>
        </div>

<!-- Track Selection (when editing) -->
          <div v-if="isEditingTeam && hackathon?.tracks?.length" class="team-section">
            <h4 class="section-label">Трек</h4>
            <div class="form-field">
              <CustomSelect
                v-model="editTeamTrackId"
                :options="[{ value: '', label: 'Без трека' }, ...hackathon.tracks.map(t => ({ value: t.id, label: t.name }))]"
                placeholder="Выберите трек"
              />
            </div>
          </div>

        <!-- Members Section -->
        <div class="team-section">
          <h4 class="section-label">
            Участники
            <span class="member-count">{{ selectedTeam.members.length }}</span>
          </h4>
          <div class="team-members-list">
            <div
              v-for="member in selectedTeam.members"
              :key="member.id"
              class="team-member-item"
              :class="{ 'is-me': member.user_id === user?.id, 'is-leader': member.role === 'leader' }"
            >
              <div class="member-avatar">
                <img v-if="member.avatar_url" :src="member.avatar_url" :alt="member.name" />
                <span v-else class="avatar-placeholder">{{ member.name[0] }}</span>
              </div>
              <div class="member-info">
                <span class="member-name">{{ member.name }}</span>
                <span v-if="member.role === 'leader'" class="member-role">
                  <Crown :size="12" />
                  Лидер
                </span>
              </div>
              <span v-if="member.user_id === user?.id" class="member-badge">вы</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Modal Footer -->
      <div class="team-modal-footer">
        <!-- View Mode Actions -->
        <template v-if="!isEditingTeam">
          <div class="footer-actions">
<button
            v-if="isCurrentUserMember"
            class="btn btn-danger-outline"
            :disabled="isLeavingTeam"
            @click="leaveTeam"
          >
            <LogOut :size="16" />
            <span v-if="isLeavingTeam">...</span>
            <template v-else>Покинуть</template>
          </button>
<button
            v-if="isCurrentUserLeader"
            class="btn btn-secondary"
            :disabled="isDeletingTeam"
            @click="deleteTeam"
          >
            <Trash2 :size="16" />
            <span v-if="isDeletingTeam">...</span>
            <template v-else>Удалить команду</template>
          </button>
        </div>
      </template>

      <!-- Edit Mode Actions -->
        <template v-else>
          <div class="footer-actions">
            <button class="btn btn-secondary" @click="cancelEditingTeam">
              Отмена
            </button>
            <button
              class="btn btn-primary"
              :disabled="!editTeamName.trim() || isSavingTeam"
              @click="saveTeamChanges"
            >
              <span v-if="isSavingTeam" class="spinner"></span>
              <Check v-else :size="16" />
              <span v-if="isSavingTeam">Сохранение...</span>
              <template v-else>Сохранить</template>
            </button>
          </div>
        </template>
      </div>
    </template>
  </div>
</div>
</div>

<!-- Report Modal -->
<ReportModal
  v-if="hackathon"
  v-model:show="showReportModal"
  target-type="hackathon"
  :target-id="hackathon.id"
  :target-name="hackathon.title"
/>
</template>

<style scoped lang="scss">
@use '../styles/variables' as *;

.hackathon-detail-page {
  min-height: 100vh;
  background: $color-bg;
  color: $color-text;
}

// Loading & Error
.loading-container,
.error-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 60vh;
  gap: 1rem;
  color: $color-text-dim;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 2px solid $color-border;
  border-top-color: $color-accent;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.back-link {
  color: $color-accent;
  text-decoration: none;

  &:hover {
    text-decoration: underline;
  }
}

.cancelled-banner,
.moderation-banner {
  position: relative;
  z-index: 2;
  margin: 6rem auto 0;
  max-width: 1200px;
  padding: 1rem 1.5rem;
  border-radius: 8px;
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem 1rem;
  align-items: baseline;
  border: 1px solid;
}

.cancelled-banner {
  background: rgba(220, 80, 80, 0.12);
  border-color: rgba(220, 80, 80, 0.5);
  color: #f5b7b7;

  strong {
    color: #ffd6d6;
  }
}

.moderation-banner.pending {
  background: rgba(245, 158, 11, 0.12);
  border-color: rgba(245, 158, 11, 0.5);
  color: #fcd9a0;

  strong {
    color: #fde7c2;
  }
}

.moderation-banner.rejected {
  background: rgba(220, 80, 80, 0.12);
  border-color: rgba(220, 80, 80, 0.5);
  color: #f5b7b7;

  strong {
    color: #ffd6d6;
  }
}

// Hero Section
.hero-section {
  position: relative;
  min-height: 60vh;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 120px 40px 60px;
  background: linear-gradient(180deg, $color-bg 0%, rgba($color-accent, 0.03) 100%);
}

.hero-banner {
  position: absolute;
  inset: 0;
  z-index: 0;

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .banner-overlay {
    position: absolute;
    inset: 0;
    background: linear-gradient(180deg, rgba($color-bg, 0.7) 0%, $color-bg 100%);
  }
}

.hero-content {
  position: relative;
  z-index: 1;
  width: 100%;
  max-width: 900px;
  text-align: center;
}

.container {
  width: 100%;
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 20px;
}

.hero-badge {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: rgba($color-accent, 0.1);
  border: 1px solid rgba($color-accent, 0.3);
  border-radius: 20px;
  font-size: 0.875rem;
  color: $color-accent;
  margin-bottom: 1.5rem;

  &.open {
    background: rgba($color-accent, 0.15);
    border-color: $color-accent;
  }
}

.hero-title {
  font-family: $font-display;
  font-size: clamp(2rem, 6vw, 4rem);
  font-weight: 600;
  line-height: 1.1;
  margin-bottom: 1rem;
  letter-spacing: -0.02em;
}

// Description Section
.description-section {
  margin-bottom: 2rem;

  .section-title {
    font-size: 1.75rem;
    font-weight: 600;
    margin-bottom: 1rem;
  }
}

.description-content {
  font-size: 1.125rem;
  line-height: 1.7;
  color: $color-text-dim;
  max-width: 800px;

  p {
    margin: 0;
    white-space: pre-wrap;
    overflow-wrap: anywhere;
    word-break: break-all;
  }
}

.hero-meta {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 1.5rem;
  color: $color-text-dim;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 8px;

  svg {
    color: $color-accent;
  }
}

// Content Wrapper
.content-wrapper {
  padding: 40px 0;
}

// Organizer Notice
.organizer-notice {
  margin-bottom: 2rem;

  .notice-content {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px 24px;
    background: rgba($color-secondary, 0.1);
    border: 1px solid rgba($color-secondary, 0.3);
    border-radius: 8px;
    color: $color-text;

    .notice-icon {
      font-size: 1.5rem;
    }

    p {
      margin: 0;
    }
  }
}

// Sections
.section {
  padding: 40px 0;
}

.section-title {
  font-family: $font-display;
  font-size: 1.5rem;
  font-weight: 600;
  margin-bottom: 1.5rem;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

// Prize Section
.prize-card {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  padding: 2rem;
  background: rgba($color-accent, 0.05);
  border: 1px solid rgba($color-accent, 0.2);
  border-radius: 12px;
}

.prize-icon-wrapper {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 64px;
  height: 64px;
  background: rgba($color-accent, 0.1);
  border-radius: 12px;
  color: $color-accent;
}

.prize-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.prize-label {
  font-size: 0.875rem;
  color: $color-text-dim;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.prize-value {
  font-family: $font-display;
  font-size: 2rem;
  font-weight: 600;
  color: $color-accent;
}

.prize-currency {
  font-size: 1.25rem;
  color: $color-text;
}

.prize-description {
  margin: 8px 0 0;
  color: $color-text-dim;
  font-size: 0.95rem;
}

// Tracks Section
.tracks-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1rem;
}

.track-card {
  padding: 1.5rem;
  background: $color-surface;
  border: 1px solid $color-border;
  border-radius: 8px;
  transition: border-color 0.3s ease;

  &:hover {
    border-color: rgba($color-accent, 0.3);
  }
}

.track-name {
  font-family: $font-display;
  font-size: 1.125rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
}

.track-description {
  font-size: 0.9rem;
  color: $color-text-dim;
  margin-bottom: 1rem;
}

.track-prize {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 0.875rem;
  color: $color-accent;
  margin-bottom: 0.5rem;
}

.track-limit {
  font-size: 0.8rem;
  color: $color-text-muted;
}

// Teams Section
.teams-actions {
  display: flex;
  gap: 0.75rem;
}

.restricted-text {
  font-size: 0.875rem;
  color: $color-text-muted;
  padding: 0.5rem 1rem;
  background: rgba($color-text-muted, 0.1);
  border-radius: 6px;
}

.teams-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.team-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.25rem;
  background: $color-surface;
  border: 1px solid $color-border;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;

  &:hover {
    border-color: rgba($color-accent, 0.3);
  }

  &.my-team {
    border-color: $color-accent;
    background: rgba($color-accent, 0.05);
  }
}

.team-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.team-name {
  font-weight: 600;
  font-size: 1rem;
}

.team-track {
  font-size: 0.8rem;
  color: $color-text-dim;
}

.team-stats {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.team-members {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 0.875rem;
  color: $color-text-dim;
}

.btn-join {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: transparent;
  border: 1px solid $color-accent;
  color: $color-accent;
  font-size: 0.875rem;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;

  &:hover:not(:disabled) {
    background: $color-accent;
    color: $color-bg;
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
}

.join-spinner {
  width: 14px;
  height: 14px;
  border: 2px solid currentColor;
  border-top-color: transparent;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

.teams-empty {
  text-align: center;
  padding: 3rem;
  color: $color-text-dim;

  p {
    margin-bottom: 0.5rem;
  }

  span {
    font-size: 0.875rem;
    opacity: 0.7;
  }
}

// Deadlines Section
.deadlines-timeline {
  position: relative;
  padding-left: 24px;

  &::before {
    content: '';
    position: absolute;
    left: 6px;
    top: 8px;
    bottom: 8px;
    width: 2px;
    background: $color-border;
  }
}

.deadline-item {
  position: relative;
  padding-left: 24px;
  padding-bottom: 1.5rem;

  &:last-child {
    padding-bottom: 0;
  }
}

.deadline-marker {
  position: absolute;
  left: -18px;
  top: 6px;
  width: 10px;
  height: 10px;
  background: $color-text-dim;
  border-radius: 50%;
  border: 2px solid $color-bg;

  .milestone & {
    background: $color-accent;
    width: 14px;
    height: 14px;
    left: -20px;
  }
}

.deadline-content {
  background: $color-surface;
  border: 1px solid $color-border;
  border-radius: 8px;
  padding: 1rem 1.25rem;
}

.deadline-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
  margin-bottom: 0.5rem;
}

.deadline-name {
  font-weight: 600;
}

.milestone-badge {
  font-size: 0.7rem;
  padding: 2px 8px;
  background: rgba($color-accent, 0.1);
  color: $color-accent;
  border-radius: 4px;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.deadline-description {
  font-size: 0.9rem;
  color: $color-text-dim;
  margin-bottom: 0.5rem;
}

.deadline-date {
  font-size: 0.8rem;
  color: $color-text-muted;
}

// Requirements Section
.requirements-card {
  padding: 1.5rem;
  background: $color-surface;
  border: 1px solid $color-border;
  border-radius: 8px;

  p {
    margin: 0 0 1rem;
    line-height: 1.6;
  }
}

.team-size-info,
.age-info {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  background: rgba($color-accent, 0.03);
  border: 1px solid $color-border;
  border-radius: 6px;
  margin-bottom: 0.75rem;
  font-size: 0.9rem;

  &:last-child {
    margin-bottom: 0;
  }

  svg {
    color: $color-accent;
  }
}

// Skills Section
.skills-cloud {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.skill-tag {
  padding: 6px 14px;
  background: rgba($color-accent, 0.08);
  border: 1px solid rgba($color-accent, 0.2);
  border-radius: 20px;
  font-size: 0.875rem;
  color: $color-text;
  transition: all 0.2s ease;

  &:hover {
    background: rgba($color-accent, 0.15);
    border-color: rgba($color-accent, 0.4);
  }
}

// Organizer Section
.organizer-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.25rem;
  background: $color-surface;
  border: 1px solid $color-border;
  border-radius: 8px;
}

.organizer-avatar {
  width: 56px;
  height: 56px;
  border-radius: 50%;
  overflow: hidden;
  background: rgba($color-accent, 0.1);
  display: flex;
  align-items: center;
  justify-content: center;

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .avatar-placeholder {
    font-family: $font-display;
    font-size: 1.5rem;
    color: $color-accent;
    font-weight: 600;
  }
}

.organizer-info {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.organizer-name-row {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.organizer-name {
  font-weight: 600;
  font-size: 1.1rem;
}

.verified-badge {
  display: flex;
  align-items: center;
  color: $color-accent;

  svg {
    fill: rgba($color-accent, 0.2);
  }
}

.organizer-contact,
.organizer-link {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 0.875rem;
  color: $color-text-dim;

  a {
    color: $color-accent;
    text-decoration: none;

    &:hover {
      text-decoration: underline;
    }
  }
}

// Buttons
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 10px 20px;
  font-family: $font-body;
  font-size: 0.95rem;
  font-weight: 500;
  text-transform: lowercase;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
  border: 1px solid transparent;

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
}

.btn-primary {
  background: $color-accent;
  color: $color-bg;
  border-color: $color-accent;

  &:hover:not(:disabled) {
    background: transparent;
    color: $color-accent;
  }
}

.btn-secondary {
  background: transparent;
  color: $color-text;
  border-color: $color-border;

  &:hover:not(:disabled) {
    border-color: $color-text;
  }
}

// Report Section
.report-section {
  padding: 20px 0 60px;
}

.report-card {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  padding: 1.5rem;
  background: $color-surface;
  border: 1px solid $color-border;
  border-radius: 12px;

  .report-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 56px;
    height: 56px;
    background: rgba($color-secondary, 0.1);
    border-radius: 12px;
    color: $color-secondary;
    flex-shrink: 0;
  }

  .report-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 4px;

    .report-title {
      font-weight: 600;
      font-size: 1.1rem;
    }

    .report-description {
      font-size: 0.9rem;
      color: $color-text-dim;
    }
  }
}

@media (max-width: 768px) {
  .report-card {
    flex-direction: column;
    text-align: center;
    gap: 1rem;

    .btn {
      width: 100%;
    }
  }
}

// Modal
.modal-overlay {
  position: fixed;
  inset: 0;
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
  background: rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(4px);
}

.modal {
  width: 100%;
  max-width: 480px;
  background: $color-surface;
  border: 1px solid $color-border;
  border-radius: 12px;
  overflow: hidden;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  border-bottom: 1px solid $color-border;

  h3 {
    font-family: $font-display;
    font-size: 1.25rem;
  }
}

.modal-close {
  background: transparent;
  border: none;
  color: $color-text-dim;
  font-size: 1.5rem;
  cursor: pointer;

  &:hover {
    color: $color-text;
  }
}

.modal-body {
  padding: 1.5rem;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  padding: 1rem 1.5rem;
  border-top: 1px solid $color-border;
}

// Form in Modal
.form-field {
  margin-bottom: 1rem;

  &:last-child {
    margin-bottom: 0;
  }

  label {
    display: block;
    font-size: 0.875rem;
    font-weight: 500;
    margin-bottom: 6px;
    color: $color-text-dim;

    .required {
      color: $color-secondary;
    }
  }

  input,
  textarea,
  select {
    width: 100%;
    padding: 10px 14px;
    background: $color-bg;
    border: 1px solid $color-border;
    border-radius: 6px;
    color: $color-text;
    font-family: inherit;
    font-size: 0.95rem;
    transition: border-color 0.2s ease;

    &:focus {
      outline: none;
      border-color: $color-accent;
    }

    &::placeholder {
      color: $color-text-muted;
    }
  }

  textarea {
    resize: vertical;
    min-height: 80px;
  }
}

.form-error {
  padding: 12px;
  background: rgba($color-secondary, 0.1);
  border: 1px solid rgba($color-secondary, 0.3);
  border-radius: 6px;
  color: $color-secondary;
  font-size: 0.875rem;
}

// Responsive
@media (max-width: 768px) {
  .hero-section {
    padding: 100px 20px 40px;
    min-height: 50vh;
  }

  .hero-meta {
    flex-direction: column;
    gap: 0.75rem;
  }

  .prize-card {
    flex-direction: column;
    text-align: center;
  }

  .prize-value {
    font-size: 1.5rem;
  }

  .tracks-grid {
    grid-template-columns: 1fr;
  }

  .section-header {
    flex-direction: column;
    align-items: flex-start;
  }

  .team-card {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }

  .team-stats {
    width: 100%;
    justify-content: space-between;
  }

  .organizer-card {
    flex-direction: column;
    text-align: center;
  }
}

// Team Modal Styles
.team-modal-overlay {
  z-index: 1100;
}

.team-modal {
  max-width: 560px;
  width: 100%;
  max-height: 85vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.team-modal-loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem;
  gap: 1rem;
  color: $color-text-dim;

  .spinner {
    width: 40px;
    height: 40px;
  }
}

.team-modal-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 1.5rem;
  border-bottom: 1px solid $color-border;
  gap: 1rem;
}

.team-modal-title-section {
  flex: 1;
  min-width: 0;
}

.team-name-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.team-modal-title {
  font-family: $font-display;
  font-size: 1.5rem;
  font-weight: 600;
  margin: 0;
  word-break: break-word;
}

.team-name-edit {
  .team-name-input {
    width: 100%;
    padding: 10px 14px;
    background: $color-bg;
    border: 1px solid $color-border;
    border-radius: 8px;
    color: $color-text;
    font-family: $font-display;
    font-size: 1.25rem;
    font-weight: 600;

    &:focus {
      outline: none;
      border-color: $color-accent;
    }
  }
}

.team-track-badge {
  display: inline-flex;
  margin-top: 0.5rem;
  padding: 4px 12px;
  background: rgba($color-accent, 0.1);
  border: 1px solid rgba($color-accent, 0.3);
  border-radius: 20px;
  font-size: 0.8rem;
  color: $color-accent;
}

.btn-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  background: transparent;
  border: 1px solid $color-border;
  border-radius: 8px;
  color: $color-text-dim;
  cursor: pointer;
  transition: all 0.2s ease;
  flex-shrink: 0;

  &:hover {
    border-color: $color-accent;
    color: $color-accent;
  }
}

.team-modal-body {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
}

.team-section {
  margin-bottom: 1.5rem;

  &:last-child {
    margin-bottom: 0;
  }
}

.section-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: $color-text-dim;
  margin-bottom: 0.75rem;

  .member-count {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 20px;
    height: 20px;
    padding: 0 6px;
    background: $color-bg;
    border-radius: 10px;
    font-size: 0.7rem;
    font-weight: 500;
  }
}

.team-description {
  color: $color-text-dim;
  line-height: 1.6;
  margin: 0;
}

.team-textarea {
  width: 100%;
  padding: 12px;
  background: $color-bg;
  border: 1px solid $color-border;
  border-radius: 8px;
  color: $color-text;
  font-size: 0.95rem;
  line-height: 1.6;
  resize: vertical;
  min-height: 80px;

  &:focus {
    outline: none;
    border-color: $color-accent;
  }

  &::placeholder {
    color: $color-text-muted;
  }
}

.team-links {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.team-link {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  background: $color-bg;
  border: 1px solid $color-border;
  border-radius: 8px;
  color: $color-text;
  text-decoration: none;
  font-size: 0.875rem;
  transition: all 0.2s ease;

  svg {
    color: $color-accent;
  }

  &:hover {
    border-color: rgba($color-accent, 0.4);
    background: rgba($color-accent, 0.05);
  }
}

.edit-links {
  display: flex;
  flex-direction: column;
  gap: 1rem;

  .form-field {
    margin-bottom: 0;

    label {
      display: flex;
      align-items: center;
      gap: 6px;

      svg {
        color: $color-accent;
      }
    }
  }
}

.team-select {
  width: 100%;
  padding: 10px 14px;
  background: $color-bg;
  border: 1px solid $color-border;
  border-radius: 8px;
  color: $color-text;
  font-size: 0.95rem;
  cursor: pointer;

  &:focus {
    outline: none;
    border-color: $color-accent;
  }
}

.team-members-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.team-member-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem;
  background: $color-bg;
  border: 1px solid $color-border;
  border-radius: 10px;
  transition: all 0.2s ease;

  &.is-me {
    border-color: rgba($color-accent, 0.4);
    background: rgba($color-accent, 0.05);
  }

  &.is-leader {
    border-color: rgba($color-secondary, 0.3);
  }
}

.member-avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  overflow: hidden;
  background: rgba($color-accent, 0.1);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .avatar-placeholder {
    font-family: $font-display;
    font-size: 1rem;
    color: $color-accent;
    font-weight: 600;
  }
}

.member-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.member-name {
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.member-role {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 0.75rem;
  color: $color-secondary;

  svg {
    fill: currentColor;
  }
}

.member-badge {
  padding: 2px 8px;
  background: $color-accent;
  color: $color-bg;
  border-radius: 4px;
  font-size: 0.7rem;
  font-weight: 600;
  text-transform: uppercase;
}

.team-modal-footer {
  padding: 1rem 1.5rem;
  border-top: 1px solid $color-border;
}

.footer-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  flex-wrap: wrap;
}

.btn-danger-outline {
  background: transparent;
  color: $color-error;
  border-color: rgba($color-error, 0.5);

  &:hover:not(:disabled) {
    background: rgba($color-error, 0.1);
    border-color: $color-error;
  }
}

// Responsive Team Modal
@media (max-width: 640px) {
  .team-modal {
    max-height: 90vh;
    margin: 10px;
  }

  .team-modal-header {
    padding: 1rem;
  }

  .team-modal-title {
    font-size: 1.25rem;
  }

  .team-modal-body {
    padding: 1rem;
  }

  .team-modal-footer {
    padding: 1rem;
  }

  .footer-actions {
    flex-direction: column;

    .btn {
      width: 100%;
    }
  }

  .team-links {
    flex-direction: column;

    .team-link {
      justify-content: center;
    }
  }
}
</style>
