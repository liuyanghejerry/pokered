<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useMapStore } from '../stores/mapStore'
import { useMapRenderer } from '../composables/useMapRenderer'
import { storeToRefs } from 'pinia'

const store = useMapStore()
const { statusMessage, hasUnsavedChanges, currentTool } = storeToRefs(store)
const canvasRef = ref<HTMLCanvasElement | null>(null)

const {
  render,
  tooltip,
  tooltipPosition,
  handleCanvasClick,
  handleCanvasMouseMove,
  handleCanvasMouseLeave,
} = useMapRenderer(canvasRef)

function handleKeydown(e: KeyboardEvent) {
  if ((e.target as HTMLElement).tagName === 'INPUT') return
  switch (e.key) {
    case 'ArrowLeft':
      store.prevMap()
      break
    case 'ArrowRight':
      store.nextMap()
      break
    case 'v':
      store.setTool('view')
      break
    case 'e':
      store.setTool('edit')
      break
  }
}

function toHex(n: number, pad = 2): string {
  return '0x' + n.toString(16).padStart(pad, '0')
}

onMounted(() => {
  document.addEventListener('keydown', handleKeydown)
  render()
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
  <div class="flex-1 relative">
    <div class="relative">
      <canvas
        ref="canvasRef"
        class="border-2 border-accent rounded"
        :class="currentTool === 'edit' ? 'cursor-crosshair' : 'cursor-default'"
        @click="handleCanvasClick"
        @mousemove="handleCanvasMouseMove"
        @mouseleave="handleCanvasMouseLeave"
      ></canvas>
    </div>

    <div class="mt-2.5 text-[11px] text-text-muted">
      <span v-if="hasUnsavedChanges" class="text-warning font-bold mr-1">*Unsaved*</span>
      {{ statusMessage }}
    </div>

    <Teleport to="body">
      <div
        v-if="tooltip"
        class="fixed bg-bg-panel/95 px-2 py-1.5 rounded border border-accent text-[11px] font-mono pointer-events-none z-50"
        :style="{ left: tooltipPosition.x + 'px', top: tooltipPosition.y + 'px' }"
      >
        Tile: ({{ tooltip.tileX }}, {{ tooltip.tileY }})<br />
        Block: ({{ tooltip.blockX }}, {{ tooltip.blockY }}) #{{ tooltip.blockIndex }}<br />
        BlockID: {{ toHex(tooltip.blockId) }}<br />
        TileID: {{ toHex(tooltip.tileId) }}<br />
        <b :class="tooltip.passable ? 'text-accent' : 'text-danger'">
          {{ tooltip.passable ? 'PASSABLE' : 'BLOCKED' }}
        </b>
      </div>
    </Teleport>
  </div>
</template>
