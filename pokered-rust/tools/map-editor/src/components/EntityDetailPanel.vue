<script setup lang="ts">
import { useMapStore } from '../stores/mapStore'
import { storeToRefs } from 'pinia'

const store = useMapStore()
const { selectedEntity } = storeToRefs(store)

function toHex(n: number, pad = 2): string {
  return '0x' + n.toString(16).padStart(pad, '0')
}
</script>

<template>
  <div v-if="selectedEntity" class="bg-bg-inset p-2.5 rounded-md">
    <div class="flex items-center justify-between mb-2">
      <h3 class="text-accent text-[13px] font-bold">
        {{ selectedEntity.type === 'sign' ? 'Sign Detail' : selectedEntity.type === 'npc' ? 'NPC Detail' : 'Warp Detail' }}
      </h3>
      <button
        class="text-[10px] text-text-muted hover:text-text cursor-pointer bg-transparent border-none"
        @click="store.selectEntity(null)"
      >
        ✕ Close
      </button>
    </div>

    <template v-if="selectedEntity.type === 'sign'">
      <div class="font-mono text-[11px] space-y-1">
        <p>Position: ({{ selectedEntity.data.x * 2 }}, {{ selectedEntity.data.y * 2 }})</p>
        <p>Text ID: {{ selectedEntity.data.text_id }}</p>
      </div>
      <div
        v-if="selectedEntity.data.text_pages && selectedEntity.data.text_pages.length > 0"
        class="mt-2 space-y-1.5"
      >
        <div
          v-for="(page, pi) in selectedEntity.data.text_pages"
          :key="pi"
          class="bg-bg p-2 rounded border border-[#333] font-mono text-[11px]"
        >
          <div v-if="page.line1" class="text-text">{{ page.line1 }}</div>
          <div v-if="page.line2" class="text-text-muted">{{ page.line2 }}</div>
        </div>
      </div>
      <p v-else class="text-[10px] text-text-muted mt-1 italic">No text data</p>
    </template>

    <template v-if="selectedEntity.type === 'npc'">
      <div class="font-mono text-[11px] space-y-1">
        <p>
          <span
            :class="selectedEntity.data.is_trainer ? 'text-danger' : selectedEntity.data.item_id != null ? 'text-accent' : 'text-[#9b59b6]'"
            class="font-bold"
          >
            {{ selectedEntity.data.sprite_name }}
          </span>
        </p>
        <p>Position: ({{ selectedEntity.data.x * 2 }}, {{ selectedEntity.data.y * 2 }})</p>
        <p>Movement: {{ selectedEntity.data.movement }} / {{ selectedEntity.data.facing }}</p>
        <p v-if="selectedEntity.data.range > 0">Range: {{ selectedEntity.data.range }}</p>
        <p v-if="selectedEntity.data.is_trainer">
          Trainer: {{ selectedEntity.data.trainer_class }} #{{ selectedEntity.data.trainer_set }}
        </p>
        <p v-if="selectedEntity.data.item_id != null">
          Item: {{ toHex(selectedEntity.data.item_id) }}
        </p>
      </div>
      <div
        v-if="selectedEntity.data.text_pages && selectedEntity.data.text_pages.length > 0"
        class="mt-2 space-y-1.5"
      >
        <p class="text-[10px] text-text-muted">
          Dialog ({{ selectedEntity.data.text_pages.length }} page{{ selectedEntity.data.text_pages.length > 1 ? 's' : '' }}):
        </p>
        <div
          v-for="(page, pi) in selectedEntity.data.text_pages"
          :key="pi"
          class="bg-bg p-2 rounded border border-[#333] font-mono text-[11px]"
        >
          <div v-if="page.line1" class="text-text">{{ page.line1 }}</div>
          <div v-if="page.line2" class="text-text-muted">{{ page.line2 }}</div>
        </div>
      </div>
      <p v-else class="text-[10px] text-text-muted mt-1 italic">No text data</p>
    </template>

    <template v-if="selectedEntity.type === 'warp'">
      <div class="font-mono text-[11px] space-y-1">
        <p>Position: ({{ selectedEntity.data.x * 2 }}, {{ selectedEntity.data.y * 2 }})</p>
        <p v-if="selectedEntity.data.dest_map_name">
          Destination: {{ selectedEntity.data.dest_map_name }}
        </p>
        <p v-if="selectedEntity.data.dest_warp_id != null">
          Dest Warp ID: {{ selectedEntity.data.dest_warp_id }}
        </p>
      </div>
      <button
        v-if="selectedEntity.data.dest_map_name"
        class="mt-2 px-3 py-1.5 bg-[#3498db] text-white border-none rounded cursor-pointer text-[11px] font-bold hover:bg-[#2980b9] w-full"
        @click="store.navigateToMap(selectedEntity!.type === 'warp' ? selectedEntity!.data.dest_map_name! : '')"
      >
        Go to {{ selectedEntity.data.dest_map_name }} →
      </button>
    </template>
  </div>
</template>
