import { reactive, nextTick, watch } from 'vue'
import { gsap } from 'gsap'
import { lockBodyScroll, unlockBodyScroll } from './useScrollLock'

export interface AdminModalOptions {
  title: string
  message: string
  confirmText?: string
  cancelText?: string
}

interface AdminModalState extends AdminModalOptions {
  isOpen: boolean
  resolve: ((value: boolean) => void) | null
}

const modalState = reactive<AdminModalState>({
  isOpen: false,
  title: '',
  message: '',
  confirmText: 'Подтвердить',
  cancelText: 'Отмена',
  resolve: null,
})

const animateModalOpen = () => {
  nextTick(() => {
    const overlay = document.querySelector('.admin-modal-overlay') as HTMLElement
    const modal = document.querySelector('.admin-modal') as HTMLElement
    if (!overlay || !modal) return

    gsap.fromTo(overlay,
      { opacity: 0 },
      { opacity: 1, duration: 0.2, ease: 'power2.out' }
    )

    gsap.fromTo(modal,
      { y: 20, opacity: 0, scale: 0.98 },
      { y: 0, opacity: 1, scale: 1, duration: 0.3, ease: 'power2.out' }
    )
  })
}

const animateModalClose = (callback: () => void) => {
  const overlay = document.querySelector('.admin-modal-overlay') as HTMLElement
  const modal = document.querySelector('.admin-modal') as HTMLElement

  if (!overlay || !modal) {
    callback()
    return
  }

  const tl = gsap.timeline({ onComplete: callback })
  tl.to(modal, { y: 10, opacity: 0, scale: 0.98, duration: 0.15, ease: 'power2.in' })
  tl.to(overlay, { opacity: 0, duration: 0.1, ease: 'power2.in' }, '<')
}

const closeModal = () => {
  animateModalClose(() => {
    modalState.isOpen = false
  })
}

export const handleConfirm = () => {
  if (modalState.resolve) {
    modalState.resolve(true)
  }
}

export const handleCancel = () => {
  if (modalState.resolve) {
    modalState.resolve(false)
  }
}

export const handleOverlayClick = () => {
  handleCancel()
}

export const useAdminModal = () => {
  const confirm = (options: AdminModalOptions): Promise<boolean> => {
    return new Promise((resolve) => {
      Object.assign(modalState, {
        isOpen: true,
        title: options.title,
        message: options.message,
        confirmText: options.confirmText || 'Подтвердить',
        cancelText: options.cancelText || 'Отмена',
        resolve: (value: boolean) => {
          closeModal()
          resolve(value)
        },
      })

      animateModalOpen()
    })
  }

  return {
    modalState,
    confirm,
  }
}

watch(() => modalState.isOpen, (isOpen: boolean) => {
  if (isOpen) {
    lockBodyScroll()
  } else {
    unlockBodyScroll()
  }
})

export { modalState }
