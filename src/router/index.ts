import { createRouter, createWebHistory } from 'vue-router'
import LandingView from '@/views/LandingView.vue'
import AuthView from '@/views/AuthView.vue'
import HackathonsView from '@/views/HackathonsView.vue'
import HackathonDetailView from '@/views/HackathonDetailView.vue'
import ProfileView from '@/views/ProfileView.vue'
import RatingsView from '@/views/RatingsView.vue'
import OrganizersView from '@/views/OrganizersView.vue'
import OrganizerDashboardView from '@/views/OrganizerDashboardView.vue'
import OrganizerProfileView from '@/views/OrganizerProfileView.vue'
import HackathonCreateView from '@/views/HackathonCreateView.vue'
import HackathonEditView from '@/views/HackathonEditView.vue'
import OrganizerTermsAcceptance from '@/components/OrganizerTermsAcceptance.vue'
import ResetPasswordView from '@/views/ResetPasswordView.vue'
import EmailVerificationView from '@/views/EmailVerificationView.vue'
import NotFoundView from '@/views/NotFoundView.vue'
import AdminDashboardView from '@/views/AdminDashboardView.vue'
import HackathonCriteriaPage from '@/components/HackathonCriteriaPage.vue'
import HackathonRatingsPage from '@/components/HackathonRatingsPage.vue'
import LegalView from '@/views/LegalView.vue'

// Secret admin path from environment variable
const ADMIN_SECRET = (import.meta as any).env?.VITE_ADMIN_SECRET || '9f2c7b6e5a1d4c8fbbd2a0c3e7f1a6d9'

const hasAdminAccess = (account: unknown) => {
  if (!account || typeof account !== 'object') {
    return false
  }

  const user = account as Record<string, unknown>

  return user.isStaff === true || user.isSuperuser === true || user.is_staff === true || user.is_superuser === true
}

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'landing',
      component: LandingView,
    },
    {
      path: '/auth',
      name: 'auth',
      component: AuthView,
    },
    {
      path: '/hackathons',
      name: 'hackathons',
      component: HackathonsView,
    },
    {
      path: '/profile',
      name: 'profile',
      component: ProfileView,
    },
    {
      path: '/ratings',
      name: 'ratings',
      component: RatingsView,
    },
    {
      path: '/organizers',
      name: 'organizers',
      component: OrganizersView,
    },
    {
      path: '/organizers/rules',
      name: 'organizer-terms',
      component: OrganizerTermsAcceptance,
    },
    {
      path: '/organizers/dashboard',
      name: 'organizer-dashboard',
      component: OrganizerDashboardView,
    },
    {
      path: '/organizers/profile',
      name: 'organizer-profile',
      component: OrganizerProfileView,
    },
    {
      path: '/hackathons/create',
      name: 'hackathon-create',
      component: HackathonCreateView,
    },
    {
      path: '/hackathons/:id/edit',
      name: 'hackathon-edit',
      component: HackathonEditView,
      props: true,
    },
    {
      path: '/hackathons/:id',
      name: 'hackathon-detail',
      component: HackathonDetailView,
      props: true,
    },
    {
      path: '/hackathons/:id/criteria',
      name: 'hackathon-criteria',
      component: HackathonCriteriaPage,
      props: true,
    },
    {
      path: '/hackathons/:id/ratings',
      name: 'hackathon-ratings',
      component: HackathonRatingsPage,
      props: true,
    },
    {
      path: '/legal',
      name: 'legal',
      component: LegalView,
    },
    {
      path: '/legal/:slug',
      name: 'legal-document',
      component: LegalView,
      props: true,
    },
    {
      path: '/login',
      redirect: '/auth',
    },
    {
      path: '/register',
      redirect: '/auth',
    },
    {
      path: '/auth/reset-password',
      name: 'reset-password',
      component: ResetPasswordView,
    },
    {
      path: '/auth/verify-email',
      name: 'verify-email',
      component: EmailVerificationView,
    },
    {
      path: `/admin/${ADMIN_SECRET}`,
      name: 'admin',
      component: AdminDashboardView,
    },
    {
      path: `/admin/${ADMIN_SECRET}/hackathons`,
      name: 'admin-hackathons',
      component: AdminDashboardView,
    },
    {
      path: `/admin/${ADMIN_SECRET}/organizers`,
      name: 'admin-organizers',
      component: AdminDashboardView,
    },
    {
      path: `/admin/${ADMIN_SECRET}/reports`,
      name: 'admin-reports',
      component: AdminDashboardView,
    },
    {
      path: `/admin/${ADMIN_SECRET}/users`,
      name: 'admin-users',
      component: AdminDashboardView,
    },
    {
      path: '/admin/:pathMatch(.*)*',
      component: NotFoundView,
    },
    {
      path: '/:pathMatch(.*)*',
      name: 'not-found',
      component: NotFoundView,
    },
  ],
  scrollBehavior() {
    return { top: 0, left: 0, behavior: 'auto' }
  },
})

// Global hook to reset scroll on route change
router.afterEach(() => {
  window.scrollTo(0, 0)
})

export default router
