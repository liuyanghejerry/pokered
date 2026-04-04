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
        {{ selectedEntity.type === 'sign' ? 'Sign Detail' : selectedEntity.type === 'npc' ? 'NPC Detail' : selectedEntity.type === 'coordEvent' ? 'Coord Event Detail' : 'Warp Detail' }}
      </h3>
      <button
        class="text-[10px] text-text-muted hover:text-text cursor-pointer bg-transparent border-none"
        @click="store.selectEntity(null)"
      >
        ✕ Close
      </button>
    </div>

    <template v-if="selectedEntity.type === 'sign'">
  <div v-if="selectedEntity.data.talk" class="mt-1">
    <span class="text-text-muted">Script: </span>
    <span class="text-accent cursor-pointer hover:underline" @click="store.jumpToFunction(selectedEntity!.data.talk!)">{{ selectedEntity.data.talk }}</span>
  </div>
  <label class="block text-[10px] text-text-muted mt-2">Script Function:</label>
  <input
    type="text"
    :value="selectedEntity.data.talk || ''"
    class="w-full p-1 rounded border border-accent bg-bg text-text text-[11px] font-mono mt-0.5"
    placeholder="e.g. signOakLab"
    @change="store.updateSignTalk(selectedEntity!.index, ($event.target as HTMLInputElement).value)"
  />
      <div class="font-mono text-[11px] space-y-1">
        <p>Position: ({{ selectedEntity.data.x }}, {{ selectedEntity.data.y }})</p>
        <p>Text ID: {{ selectedEntity.data.textId }}</p>
      </div>
      <div
        v-if="selectedEntity.data.textId != null"
        class="mt-2"
      >
        <p class="text-[10px] text-text-muted italic">Text data in map.json text section</p>
      </div>
    </template>

    <template v-if="selectedEntity.type === 'npc'">
  <div v-if="selectedEntity.data.talk" class="mt-1">
    <span class="text-text-muted">Script: </span>
    <span class="text-accent cursor-pointer hover:underline" @click="store.jumpToFunction(selectedEntity!.data.talk!)">{{ selectedEntity.data.talk }}</span>
  </div>
  <label class="block text-[10px] text-text-muted mt-2">Script Function:</label>
  <input
    type="text"
    :value="selectedEntity.data.talk || ''"
    class="w-full p-1 rounded border border-accent bg-bg text-text text-[11px] font-mono mt-0.5"
    placeholder="e.g. talkOak"
    @change="store.updateNpcTalk(selectedEntity!.index, ($event.target as HTMLInputElement).value)"
  />
      <div class="font-mono text-[11px] space-y-1">
        <p>
          <span
            :class="selectedEntity.data.isTrainer ? 'text-danger' : selectedEntity.data.itemId != null ? 'text-accent' : 'text-[#9b59b6]'"
            class="font-bold"
          >
            {{ selectedEntity.data.spriteName }}
          </span>
        </p>
        <p>Position: ({{ selectedEntity.data.x }}, {{ selectedEntity.data.y }})</p>
        <p>Movement: {{ selectedEntity.data.movement }} / {{ selectedEntity.data.facing }}</p>
        <p v-if="selectedEntity.data.range > 0">Range: {{ selectedEntity.data.range }}</p>
        <p v-if="selectedEntity.data.isTrainer">
          Trainer: {{ selectedEntity.data.trainerClass }} #{{ selectedEntity.data.trainerSet }}
        </p>
        <p v-if="selectedEntity.data.itemId != null">
          Item: {{ toHex(selectedEntity.data.itemId) }}
        </p>
      </div>
      <div
        v-if="selectedEntity.data.textId != null"
        class="mt-2"
      >
        <p class="text-[10px] text-text-muted italic">Text data in map.json text section</p>
      </div>
    </template>

    <template v-if="selectedEntity.type === 'coordEvent'">
  <div class="font-mono text-[11px] space-y-1">
    <p>Position: ({{ selectedEntity.data.x }}, {{ selectedEntity.data.y }})</p>
    <p>Trigger: <span class="text-accent cursor-pointer hover:underline" @click="store.jumpToFunction(selectedEntity!.data.trigger)">{{ selectedEntity.data.trigger }}</span></p>
  </div>
</template>

<template v-if="selectedEntity.type === 'warp'">
      <div class="font-mono text-[11px] space-y-1">
        <p>Position: ({{ selectedEntity.data.x }}, {{ selectedEntity.data.y }})</p>
        <p v-if="selectedEntity.data.destMap">
          Destination: {{ selectedEntity.data.destMap }}
        </p>
        <p v-if="selectedEntity.data.destWarpId != null">
          Dest Warp ID: {{ selectedEntity.data.destWarpId }}
        </p>
      </div>
      <button
        v-if="selectedEntity.data.destMap"
        class="mt-2 px-3 py-1.5 bg-[#3498db] text-white border-none rounded cursor-pointer text-[11px] font-bold hover:bg-[#2980b9] w-full"
        @click="store.navigateToMap(selectedEntity!.type === 'warp' ? selectedEntity!.data.destMap! : '')"
      >
        Go to {{ selectedEntity.data.destMap }} →
      </button>
    </template>
  </div>
</template>
