<script setup lang="ts">
import { useMapStore } from '../stores/mapStore'
import { storeToRefs } from 'pinia'

const store = useMapStore()
const { currentTool, zoom } = storeToRefs(store)

const tools = [
  { id: 'view' as const, label: 'View' },
  { id: 'edit' as const, label: 'Edit Collision' },
]
</script>

<template>
  <div class="flex items-center gap-2.5 mb-2.5">
    <button
      v-for="tool in tools"
      :key="tool.id"
      class="px-4 py-2 rounded text-xs cursor-pointer border-2 transition-colors"
      :class="
        currentTool === tool.id
          ? 'border-accent bg-[#2a4a3e] text-text'
          : 'border-transparent bg-[#333] text-text hover:bg-[#444]'
      "
      @click="store.setTool(tool.id)"
    >
      {{ tool.label }}
    </button>

    <div class="flex items-center gap-1.5 ml-4">
      <button
        class="px-3 py-1.5 bg-[#333] text-text border-none rounded cursor-pointer text-xs hover:bg-[#444]"
        @click="store.zoomOut()"
      >
        -
      </button>
      <span class="text-xs min-w-[50px] text-center">{{ zoom }}x</span>
      <button
        class="px-3 py-1.5 bg-[#333] text-text border-none rounded cursor-pointer text-xs hover:bg-[#444]"
        @click="store.zoomIn()"
      >
        +
      </button>
    </div>
  </div>
</template>
