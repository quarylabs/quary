export type LayoutItem = {
  w: number
  h: number
  x: number
  y: number
  i: string
  minW?: number
  minH?: number
  maxW?: number
  maxH?: number
  moved?: boolean
  static?: boolean
  isDraggable?: boolean
  isResizable?: boolean
  resizeHandles?: Array<ResizeHandleAxis>
  isBounded?: boolean
}

export type ResizeHandleAxis = 's' | 'w' | 'e' | 'n' | 'sw' | 'nw' | 'se' | 'ne'
