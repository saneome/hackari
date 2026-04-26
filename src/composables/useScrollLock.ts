import { watch, type Ref } from 'vue'

let lockCount = 0

function getScrollbarWidth(): number {
  return window.innerWidth - document.documentElement.clientWidth
}

export function lockBodyScroll(): void {
  if (lockCount === 0) {
    const scrollbarWidth = getScrollbarWidth()
    document.body.style.overflow = 'hidden'
    if (scrollbarWidth > 0) {
      document.body.style.paddingRight = `${scrollbarWidth}px`
    }
  }
  lockCount++
}

export function unlockBodyScroll(): void {
  lockCount = Math.max(0, lockCount - 1)
  if (lockCount === 0) {
    document.body.style.overflow = ''
    document.body.style.paddingRight = ''
  }
}

export function useScrollLock(isOpen: Ref<boolean> | (() => boolean)): void {
  watch(isOpen, (val: boolean) => {
    if (val) {
      lockBodyScroll()
    } else {
      unlockBodyScroll()
    }
  }, { immediate: true })
}
