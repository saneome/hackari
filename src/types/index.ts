export interface Point3D {
  x: number
  y: number
  z: number
}

export interface MousePosition {
  x: number
  y: number
}

export interface Step {
  id: number
  icon: string
  title: string
  description: string
}

export interface Feature {
  id: number
  title: string
  description: string
  icon: string
  size: 'small' | 'medium' | 'large'
}

export interface StatData {
  label: string
  value: number
  suffix: string
  prefix?: string
}

export interface CompetencyData {
  name: string
  value: number
}
