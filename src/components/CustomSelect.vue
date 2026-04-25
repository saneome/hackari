<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick, watch } from 'vue'
import { PhCaretDown, PhCheck } from '@phosphor-icons/vue'

interface Option {
  value: string
  label: string
  disabled?: boolean
}

interface Props {
  modelValue: string
  options: Option[]
  placeholder?: string
  label?: string
  disabled?: boolean
  error?: string
  required?: boolean
  zIndex?: number
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: 'Выберите...',
  label: '',
  disabled: false,
  error: '',
  required: false,
  zIndex: 90,
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
  (e: 'blur'): void
}>()

const isOpen = ref(false)
const selectRef = ref<HTMLElement | null>(null)
const triggerRef = ref<HTMLElement | null>(null)
const optionsRef = ref<HTMLElement | null>(null)
const focusedIndex = ref(-1)
const dropdownPosition = ref({ top: 0, left: 0, width: 0 })

const dropdownStyle = computed(() => ({
  position: 'fixed',
  top: `${dropdownPosition.value.top}px`,
  left: `${dropdownPosition.value.left}px`,
  width: `${dropdownPosition.value.width}px`,
  zIndex: props.zIndex,
}))

const updateDropdownPosition = () => {
  if (!triggerRef.value) return
  const rect = triggerRef.value.getBoundingClientRect()
  dropdownPosition.value = {
    top: rect.bottom + 8,
    left: rect.left,
    width: rect.width,
  }
}

const toggle = () => {
  if (props.disabled) return
  isOpen.value = !isOpen.value
  if (isOpen.value) {
    updateDropdownPosition()
    focusedIndex.value = props.options.findIndex(opt => opt.value === props.modelValue)
    nextTick(() => {
      scrollToFocused()
    })
  }
}

const selectedOption = computed(() => {
  return props.options.find(opt => opt.value === props.modelValue)
})

const displayLabel = computed(() => {
  return selectedOption.value?.label || props.placeholder
})

const hasError = computed(() => !!props.error)

const selectOption = (option: Option) => {
  if (option.disabled) return
  emit('update:modelValue', option.value)
  isOpen.value = false
  emit('blur')
}

const close = () => {
  isOpen.value = false
  emit('blur')
}

const scrollToFocused = () => {
  if (!optionsRef.value || focusedIndex.value < 0) return
  const optionEl = optionsRef.value.children[focusedIndex.value] as HTMLElement
  if (optionEl) {
    optionEl.scrollIntoView({ block: 'nearest' })
  }
}

const handleKeydown = (e: KeyboardEvent) => {
  if (props.disabled) return

  switch (e.key) {
    case 'Enter':
    case ' ':
      e.preventDefault()
      if (isOpen.value && focusedIndex.value >= 0) {
        selectOption(props.options[focusedIndex.value])
      } else {
        toggle()
      }
      break
    case 'Escape':
      if (isOpen.value) {
        e.preventDefault()
        close()
      }
      break
    case 'ArrowDown':
      e.preventDefault()
      if (!isOpen.value) {
        toggle()
      } else {
        const nextIndex = focusedIndex.value + 1
        if (nextIndex < props.options.length) {
          focusedIndex.value = nextIndex
          scrollToFocused()
        }
      }
      break
    case 'ArrowUp':
      e.preventDefault()
      if (!isOpen.value) {
        toggle()
      } else {
        const prevIndex = focusedIndex.value - 1
        if (prevIndex >= 0) {
          focusedIndex.value = prevIndex
          scrollToFocused()
        }
      }
      break
    case 'Tab':
      if (isOpen.value) {
        close()
      }
      break
  }
}

const handleClickOutside = (e: MouseEvent) => {
  if (selectRef.value && !selectRef.value.contains(e.target as Node)) {
    close()
  }
}

const handleReposition = () => {
  if (isOpen.value) updateDropdownPosition()
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
  window.addEventListener('scroll', handleReposition, true)
  window.addEventListener('resize', handleReposition)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
  window.removeEventListener('scroll', handleReposition, true)
  window.removeEventListener('resize', handleReposition)
})

watch(() => props.modelValue, () => {
  focusedIndex.value = props.options.findIndex(opt => opt.value === props.modelValue)
})
</script>

<template>
  <div
    ref="selectRef"
    class="custom-select"
    :class="{ 'is-open': isOpen, 'is-disabled': disabled, 'has-error': hasError }"
  >
    <label v-if="label" class="select-label">
      {{ label }}
      <span v-if="required" class="required-mark">*</span>
    </label>

    <div
      ref="triggerRef"
      class="select-trigger"
      tabindex="0"
      role="combobox"
      :aria-expanded="isOpen"
      :aria-disabled="disabled"
      @click="toggle"
      @keydown="handleKeydown"
    >
      <span class="select-value" :class="{ 'is-placeholder': !selectedOption }">
        {{ displayLabel }}
      </span>
      <PhCaretDown class="select-arrow" :size="16" :class="{ 'is-open': isOpen }" />
    </div>

    <Teleport to="body">
    <Transition name="dropdown">
      <div
        v-if="isOpen"
        ref="optionsRef"
        class="select-dropdown"
        role="listbox"
        :style="dropdownStyle"
        @wheel.stop
      >
        <div
          v-for="(option, index) in options"
          :key="option.value"
          class="select-option"
          :class="{
            'is-selected': option.value === modelValue,
            'is-focused': index === focusedIndex,
            'is-disabled': option.disabled
          }"
          role="option"
          :aria-selected="option.value === modelValue"
          @click="selectOption(option)"
          @mouseenter="focusedIndex = index"
        >
          <span class="option-label">{{ option.label }}</span>
          <PhCheck
            v-if="option.value === modelValue"
            class="option-check"
            :size="16"
          />
        </div>
      </div>
    </Transition>
  </Teleport>

    <span v-if="error" class="select-error">{{ error }}</span>
  </div>
</template>

<style scoped lang="scss">
.custom-select {
  position: relative;
  width: 100%;
}

.select-label {
  display: block;
  font-size: 0.875rem;
  font-weight: 500;
  color: rgba(255, 255, 255, 0.7);
  margin-bottom: 0.5rem;
}

.required-mark {
  color: #FF5722;
  margin-left: 0.25rem;
}

.select-trigger {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  width: 100%;
  padding: 0.75rem 1rem;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  outline: none;

  &:hover:not(.is-disabled) {
    border-color: rgba(255, 255, 255, 0.2);
  }

  &:focus-visible {
    border-color: #D4FF00;
    box-shadow: 0 0 0 2px rgba(212, 255, 0, 0.2);
  }
}

.custom-select.is-open .select-trigger {
  border-color: #D4FF00;
}

.custom-select.is-disabled .select-trigger {
  opacity: 0.5;
  cursor: not-allowed;
}

.custom-select.has-error .select-trigger {
  border-color: #FF5722;
}

.select-value {
  flex: 1;
  font-size: 0.95rem;
  color: #ffffff;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;

  &.is-placeholder {
    color: rgba(255, 255, 255, 0.5);
  }
}

.select-arrow {
  color: rgba(255, 255, 255, 0.5);
  transition: transform 0.2s ease;
  flex-shrink: 0;

  &.is-open {
    transform: rotate(180deg);
  }
}

.select-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  margin-top: 0.5rem;
  max-height: 240px;
  overflow-y: auto;
  background: #1a1a1a;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.5);
  z-index: 100;

  &::-webkit-scrollbar {
    width: 6px;
  }

  &::-webkit-scrollbar-track {
    background: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.2);
    border-radius: 3px;
  }
}

.select-option {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem 1rem;
  cursor: pointer;
  transition: all 0.15s ease;
  outline: none;

  &:hover:not(.is-disabled),
  &.is-focused {
    background: rgba(255, 255, 255, 0.05);
  }

  &.is-selected {
    background: rgba(212, 255, 0, 0.1);
    color: #D4FF00;
  }

  &.is-disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
}

.option-label {
  font-size: 0.95rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.option-check {
  flex-shrink: 0;
  color: #D4FF00;
}

.select-error {
  display: block;
  margin-top: 0.5rem;
  font-size: 0.8rem;
  color: #FF5722;
}

// Transitions
.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.2s ease;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}
</style>
