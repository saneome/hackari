import { ref, onMounted, onUnmounted } from 'vue'
import type { Point3D, MousePosition } from '@/types'

export function useMouse3D() {
  const mouse = ref<MousePosition>({ x: 0, y: 0 })
  const rotation = ref<Point3D>({ x: 0, y: 0, z: 0 })
  const targetRotation = ref<Point3D>({ x: 0, y: 0, z: 0 })

  let rafId: number | null = null
  const lerpFactor = 0.1

  const handleMouseMove = (e: MouseEvent) => {
    const centerX = window.innerWidth / 2
    const centerY = window.innerHeight / 2

    mouse.value = {
      x: (e.clientX - centerX) / centerX,
      y: (e.clientY - centerY) / centerY,
    }

    targetRotation.value = {
      x: mouse.value.y * 15,
      y: mouse.value.x * -15,
      z: 0,
    }
  }

  const animate = () => {
    rotation.value = {
      x: rotation.value.x + (targetRotation.value.x - rotation.value.x) * lerpFactor,
      y: rotation.value.y + (targetRotation.value.y - rotation.value.y) * lerpFactor,
      z: rotation.value.z + (targetRotation.value.z - rotation.value.z) * lerpFactor,
    }

    rafId = requestAnimationFrame(animate)
  }

  onMounted(() => {
    window.addEventListener('mousemove', handleMouseMove)
    rafId = requestAnimationFrame(animate)
  })

  onUnmounted(() => {
    window.removeEventListener('mousemove', handleMouseMove)
    if (rafId) cancelAnimationFrame(rafId)
  })

  return {
    mouse,
    rotation,
    targetRotation,
  }
}
