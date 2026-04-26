import { ref, watch } from 'vue'
import { lockBodyScroll, unlockBodyScroll } from './useScrollLock'

export interface ModalDialogOptions {
  title: string
  message: string
  confirmText?: string
  cancelText?: string
  type?: 'info' | 'success' | 'warning' | 'error'
}

export interface ModalDialogState extends ModalDialogOptions {
  isOpen: boolean
  resolve: ((value: boolean) => void) | null
}

const dialogState = ref<ModalDialogState>({
  isOpen: false,
  title: '',
  message: '',
  confirmText: 'ОК',
  cancelText: 'Отмена',
  type: 'info',
  resolve: null,
})

export function useModalDialog() {
  const confirm = (options: ModalDialogOptions): Promise<boolean> => {
    return new Promise((resolve) => {
      dialogState.value = {
        ...options,
        confirmText: options.confirmText || 'Подтвердить',
        cancelText: options.cancelText || 'Отмена',
        type: options.type || 'info',
        isOpen: true,
        resolve,
      }
    })
  }

  const alert = (options: Omit<ModalDialogOptions, 'cancelText'>): Promise<void> => {
    return new Promise((resolve) => {
      dialogState.value = {
        ...options,
        confirmText: options.confirmText || 'ОК',
        cancelText: '',
        type: options.type || 'info',
        isOpen: true,
        resolve: () => {
          resolve()
        },
      }
    })
  }

  const close = (result: boolean) => {
    if (dialogState.value.resolve) {
      dialogState.value.resolve(result)
    }
    dialogState.value.isOpen = false
    dialogState.value.resolve = null
  }

  const cancel = () => {
    close(false)
  }

  const confirmAction = () => {
    close(true)
  }

  return {
    dialogState,
    confirm,
    alert,
    close,
    cancel,
    confirmAction,
  }
}

watch(() => dialogState.value.isOpen, (isOpen: boolean) => {
  if (isOpen) {
    lockBodyScroll()
  } else {
    unlockBodyScroll()
  }
})
