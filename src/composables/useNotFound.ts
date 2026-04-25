import { ref } from 'vue'

const isNotFoundActive = ref(false)

export function useNotFound() {
  return {
    isNotFoundActive,
    setNotFound: (value: boolean) => {
      isNotFoundActive.value = value
    },
  }
}
