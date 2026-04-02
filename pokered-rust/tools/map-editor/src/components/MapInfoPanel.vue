<script setup lang="ts">
import { useMapStore } from '../stores/mapStore'
import { storeToRefs } from 'pinia'

const store = useMapStore()
const { currentMap } = storeToRefs(store)

function toHex(n: number, pad = 2): string {
  return '0x' + n.toString(16).padStart(pad, '0')
}

function selectWarp(index: number) {
  const warp = currentMap.value?.warps?.[index]
  if (warp) store.selectEntity({ type: 'warp', data: warp, index })
}

function selectSign(index: number) {
  const sign = currentMap.value?.signs?.[index]
  if (sign) store.selectEntity({ type: 'sign', data: sign, index })
}

function selectNpc(index: number) {
  const npc = currentMap.value?.npcs?.[index]
  if (npc) store.selectEntity({ type: 'npc', data: npc, index })
}

function isSelected(type: string, index: number): boolean {
  return store.selectedEntity?.type === type && store.selectedEntity?.index === index
}
</script>

<template>
  <div class="bg-bg-inset p-2.5 rounded-md font-mono text-[11px]">
    <h3 class="text-accent text-[13px] font-bold mb-2 font-sans">Map Info</h3>
    <template v-if="currentMap">
      <p class="my-0.5"><b>{{ currentMap.name }}</b></p>
      <p class="my-0.5">ID: <code class="text-accent">{{ toHex(currentMap.id) }}</code></p>
      <p class="my-0.5">Size: {{ currentMap.width }}x{{ currentMap.height }} blocks</p>
      <p class="my-0.5">Tiles: {{ currentMap.width * 4 }}x{{ currentMap.height * 4 }}</p>
      <p class="my-0.5">Tileset: {{ currentMap.tileset_name }}</p>

      <template v-if="currentMap.warps && currentMap.warps.length > 0">
        <p class="my-0.5"><b>Warps:</b></p>
        <p
          v-for="(warp, i) in currentMap.warps"
          :key="i"
          class="my-0.5 ml-2.5 cursor-pointer hover:text-accent transition-colors"
          :class="isSelected('warp', i) ? 'text-accent font-bold' : ''"
          @click="selectWarp(i)"
        >
          Warp {{ i }}: ({{ warp.x * 2 }}, {{ warp.y * 2 }})
          <template v-if="warp.dest_map_name"> → {{ warp.dest_map_name }}</template>
        </p>
      </template>

      <template v-if="currentMap.signs && currentMap.signs.length > 0">
        <p class="my-0.5"><b>Signs ({{ currentMap.signs.length }}):</b></p>
        <p
          v-for="(sign, i) in currentMap.signs"
          :key="'sign-' + i"
          class="my-0.5 ml-2.5 cursor-pointer hover:text-[#f1c40f] transition-colors"
          :class="isSelected('sign', i) ? 'text-[#f1c40f] font-bold' : ''"
          @click="selectSign(i)"
        >
          Sign {{ i }}: ({{ sign.x * 2 }}, {{ sign.y * 2 }}) text#{{ sign.text_id }}
        </p>
      </template>

      <template v-if="currentMap.npcs && currentMap.npcs.length > 0">
        <p class="my-0.5"><b>NPCs ({{ currentMap.npcs.length }}):</b></p>
        <p
          v-for="(npc, i) in currentMap.npcs"
          :key="'npc-' + i"
          class="my-0.5 ml-2.5 cursor-pointer hover:opacity-80 transition-colors"
          :class="isSelected('npc', i) ? 'font-bold' : ''"
          @click="selectNpc(i)"
        >
          <span :class="npc.is_trainer ? 'text-danger' : npc.item_id != null ? 'text-accent' : 'text-text'">
            {{ npc.sprite_name }}
          </span>
          ({{ npc.x * 2 }}, {{ npc.y * 2 }})
          <template v-if="npc.is_trainer"> 🗡{{ npc.trainer_class }}#{{ npc.trainer_set }}</template>
          <template v-if="npc.item_id != null"> item={{ toHex(npc.item_id) }}</template>
        </p>
      </template>

      <p class="my-0.5">Passable tiles: {{ currentMap.passable_tiles.length }}</p>
    </template>
    <template v-else>
      <p>Load data file to begin</p>
    </template>
  </div>
</template>
