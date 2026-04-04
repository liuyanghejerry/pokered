<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { useMapStore } from '../stores/mapStore'
import { storeToRefs } from 'pinia'
import { TOWN_MAP_COORDS, type TownMapCoord } from '../types/constants'

const store = useMapStore()
const { currentMap } = storeToRefs(store)

const canvasRef = ref<HTMLCanvasElement | null>(null)
const hoveredLocation = ref<TownMapCoord | null>(null)
const tooltipPos = ref({ x: 0, y: 0 })

const CELL_SIZE = 16
const MAP_WIDTH = 16
const MAP_HEIGHT = 16
const PADDING = 4

function getMapColor(coord: TownMapCoord, isCurrentMap: boolean): string {
  if (isCurrentMap) return '#e74c3c'
  if (coord.mapName.includes('City') || coord.mapName.includes('Town')) return '#f1c40f'
  if (coord.mapName.includes('Route')) return '#2ecc71'
  if (coord.mapName === 'IndigoPlateau') return '#9b59b6'
  return '#3498db'
}

function getCurrentMapCoord(): TownMapCoord | undefined {
  if (!currentMap.value) return undefined
  return TOWN_MAP_COORDS.find(c => c.mapName === currentMap.value?.name)
}

function render() {
  const canvas = canvasRef.value
  if (!canvas) return

  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const width = MAP_WIDTH * CELL_SIZE + PADDING * 2
  const height = MAP_HEIGHT * CELL_SIZE + PADDING * 2
  canvas.width = width
  canvas.height = height

  ctx.fillStyle = '#1a1a2e'
  ctx.fillRect(0, 0, width, height)

  ctx.strokeStyle = '#2d2d44'
  ctx.lineWidth = 0.5
  for (let x = 0; x <= MAP_WIDTH; x++) {
    ctx.beginPath()
    ctx.moveTo(PADDING + x * CELL_SIZE, PADDING)
    ctx.lineTo(PADDING + x * CELL_SIZE, PADDING + MAP_HEIGHT * CELL_SIZE)
    ctx.stroke()
  }
  for (let y = 0; y <= MAP_HEIGHT; y++) {
    ctx.beginPath()
    ctx.moveTo(PADDING, PADDING + y * CELL_SIZE)
    ctx.lineTo(PADDING + MAP_WIDTH * CELL_SIZE, PADDING + y * CELL_SIZE)
    ctx.stroke()
  }

  const currentCoord = getCurrentMapCoord()

  TOWN_MAP_COORDS.forEach(coord => {
    if (coord.mapName === 'UnusedMap0B') return

    const isCurrent = currentCoord?.mapName === coord.mapName
    const px = PADDING + coord.x * CELL_SIZE + CELL_SIZE / 2
    const py = PADDING + coord.y * CELL_SIZE + CELL_SIZE / 2
    const radius = isCurrent ? 7 : 5

    ctx.beginPath()
    ctx.arc(px, py, radius, 0, Math.PI * 2)
    ctx.fillStyle = getMapColor(coord, isCurrent)
    ctx.fill()

    if (isCurrent) {
      ctx.strokeStyle = '#fff'
      ctx.lineWidth = 2
      ctx.stroke()
    }
  })

  if (currentCoord && currentCoord.mapName !== 'UnusedMap0B') {
    const px = PADDING + currentCoord.x * CELL_SIZE + CELL_SIZE / 2
    const py = PADDING + currentCoord.y * CELL_SIZE + CELL_SIZE / 2
    ctx.beginPath()
    ctx.arc(px, py, 10, 0, Math.PI * 2)
    ctx.strokeStyle = 'rgba(231, 76, 60, 0.5)'
    ctx.lineWidth = 2
    ctx.stroke()
  }
}

function handleMouseMove(e: MouseEvent) {
  const canvas = canvasRef.value
  if (!canvas) return

  const rect = canvas.getBoundingClientRect()
  const x = e.clientX - rect.left - PADDING
  const y = e.clientY - rect.top - PADDING

  const gridX = Math.floor(x / CELL_SIZE)
  const gridY = Math.floor(y / CELL_SIZE)

  const coord = TOWN_MAP_COORDS.find(c => c.x === gridX && c.y === gridY && c.mapName !== 'UnusedMap0B')
  hoveredLocation.value = coord || null
  tooltipPos.value = { x: e.clientX, y: e.clientY }
}

function handleMouseLeave() {
  hoveredLocation.value = null
}

function handleClick() {
  if (!hoveredLocation.value) return

  const mapIndex = store.maps.findIndex(m => m.name === hoveredLocation.value?.mapName)
  if (mapIndex >= 0) {
    store.selectMap(mapIndex)
  }
}

onMounted(() => {
  render()
})

onUnmounted(() => {
})

watch(currentMap, () => {
  render()
})
</script>

<template>
  <div class="bg-bg-inset p-2.5 rounded-md">
    <h3 class="text-accent text-[13px] font-bold mb-2 font-sans">World Map</h3>
    <div class="relative">
      <canvas
        ref="canvasRef"
        class="block cursor-pointer border border-accent/30 rounded"
        @mousemove="handleMouseMove"
        @mouseleave="handleMouseLeave"
        @click="handleClick"
      ></canvas>
      
      <Teleport to="body">
        <div
          v-if="hoveredLocation"
          class="fixed bg-bg-panel/95 px-2 py-1 rounded border border-accent text-[11px] font-mono pointer-events-none z-50"
          :style="{ left: tooltipPos.x + 10 + 'px', top: tooltipPos.y + 10 + 'px' }"
        >
          <b>{{ hoveredLocation.displayName }}</b>
        </div>
      </Teleport>
    </div>
    
    <div class="mt-2 flex flex-wrap gap-x-3 gap-y-1 text-[10px]">
      <div class="flex items-center gap-1">
        <div class="w-2.5 h-2.5 rounded-full bg-[#f1c40f]"></div>
        <span>City</span>
      </div>
      <div class="flex items-center gap-1">
        <div class="w-2.5 h-2.5 rounded-full bg-[#2ecc71]"></div>
        <span>Route</span>
      </div>
      <div class="flex items-center gap-1">
        <div class="w-2.5 h-2.5 rounded-full bg-[#e74c3c] border border-white"></div>
        <span>Current</span>
      </div>
    </div>
  </div>
</template>