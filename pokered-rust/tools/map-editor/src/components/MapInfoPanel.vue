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

function selectCoordEvent(index: number) {
  const ce = store.currentScriptConfig?.coordEvents?.[index]
  if (ce) {
    store.selectEntity({
      type: 'coordEvent',
      data: { x: ce.position[0], y: ce.position[1], trigger: ce.trigger },
      index,
    })
  }
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
      <p class="my-0.5">Size: {{ currentMap.header.width }}x{{ currentMap.header.height }} blocks</p>
      <p class="my-0.5">Tiles: {{ currentMap.header.width * 4 }}x{{ currentMap.header.height * 4 }}</p>
      <p class="my-0.5">Tileset: {{ currentMap.header.tileset }}</p>
      <p class="my-0.5">Music: {{ currentMap.header.music }}</p>

      <template v-if="currentMap.connections">
        <p class="my-0.5"><b>Connections:</b></p>
        <p v-if="currentMap.connections.north" class="my-0.5 ml-2.5 cursor-pointer hover:text-accent transition-colors" @click="store.navigateToMap(currentMap.connections.north!.targetMap)">
          North → {{ currentMap.connections.north.targetMap }} (offset: {{ currentMap.connections.north.offset }})
        </p>
        <p v-if="currentMap.connections.south" class="my-0.5 ml-2.5 cursor-pointer hover:text-accent transition-colors" @click="store.navigateToMap(currentMap.connections.south!.targetMap)">
          South → {{ currentMap.connections.south.targetMap }} (offset: {{ currentMap.connections.south.offset }})
        </p>
        <p v-if="currentMap.connections.west" class="my-0.5 ml-2.5 cursor-pointer hover:text-accent transition-colors" @click="store.navigateToMap(currentMap.connections.west!.targetMap)">
          West → {{ currentMap.connections.west.targetMap }} (offset: {{ currentMap.connections.west.offset }})
        </p>
        <p v-if="currentMap.connections.east" class="my-0.5 ml-2.5 cursor-pointer hover:text-accent transition-colors" @click="store.navigateToMap(currentMap.connections.east!.targetMap)">
          East → {{ currentMap.connections.east.targetMap }} (offset: {{ currentMap.connections.east.offset }})
        </p>
      </template>

      <template v-if="currentMap.warps && currentMap.warps.length > 0">
        <p class="my-0.5"><b>Warps ({{ currentMap.warps.length }}):</b></p>
        <p
          v-for="(warp, i) in currentMap.warps"
          :key="i"
          class="my-0.5 ml-2.5 cursor-pointer hover:text-accent transition-colors"
          :class="isSelected('warp', i) ? 'text-accent font-bold' : ''"
          @click="selectWarp(i)"
        >
          Warp {{ i }}: ({{ warp.x }}, {{ warp.y }})
          <template v-if="warp.destMap"> → {{ warp.destMap }}</template>
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
          Sign {{ i }}: ({{ sign.x }}, {{ sign.y }}) text#{{ sign.textId }}
          <template v-if="sign.talk"> → <span class="text-accent cursor-pointer hover:underline" @click.stop="store.jumpToFunction(sign.talk!)">{{ sign.talk }}</span></template>
        </p>
      </template>

      <template v-if="store.currentScriptConfig?.mapScripts?.length">
        <p class="my-0.5"><b>Map Scripts ({{ store.currentScriptConfig.mapScripts.length }}):</b></p>
        <p v-for="(fn, i) in store.currentScriptConfig.mapScripts" :key="'ms-' + i" class="my-0.5 ml-2.5 font-mono text-accent cursor-pointer hover:underline" @click="store.jumpToFunction(fn)">
          [{{ i }}] {{ fn }}
        </p>
      </template>

      <template v-if="store.currentScriptConfig?.coordEvents?.length">
        <p class="my-0.5"><b>Coord Events ({{ store.currentScriptConfig.coordEvents.length }}):</b></p>
        <p v-for="(ce, i) in store.currentScriptConfig.coordEvents" :key="'ce-' + i"
           class="my-0.5 ml-2.5 cursor-pointer hover:text-[#e67e22] transition-colors"
           :class="isSelected('coordEvent', i) ? 'text-[#e67e22] font-bold' : ''"
           @click="selectCoordEvent(i)">
          ({{ ce.position[0] }}, {{ ce.position[1] }}) → <span class="cursor-pointer hover:underline" @click.stop="store.jumpToFunction(ce.trigger)">{{ ce.trigger }}</span>
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
          <span :class="npc.isTrainer ? 'text-danger' : npc.itemId != null ? 'text-accent' : 'text-text'">
            {{ npc.spriteName ?? `sprite#${npc.spriteId}` }}
          </span>
          ({{ npc.x }}, {{ npc.y }})
          <template v-if="npc.isTrainer"> {{ npc.trainerClass }}#{{ npc.trainerSet }}</template>
          <template v-if="npc.itemId != null"> item={{ toHex(npc.itemId) }}</template>
          <template v-if="npc.talk"> → <span class="text-accent cursor-pointer hover:underline" @click.stop="store.jumpToFunction(npc.talk!)">{{ npc.talk }}</span></template>
        </p>
      </template>

      <template v-if="currentMap.wild">
        <p class="my-0.5"><b>Wild Pokemon:</b></p>
        <template v-if="currentMap.wild.red?.grass?.mons?.length">
          <p class="my-0.5 ml-2.5 text-text-muted">Red Grass (rate: {{ currentMap.wild.red.grass.encounterRate }}):</p>
          <p v-for="(mon, i) in currentMap.wild.red.grass.mons.slice(0, 5)" :key="'rg-' + i" class="my-0.5 ml-5 text-[10px]">
            Lv{{ mon.level }} {{ mon.species }}
          </p>
          <p v-if="currentMap.wild.red.grass.mons.length > 5" class="my-0.5 ml-5 text-[10px] text-text-muted">
            ...+{{ currentMap.wild.red.grass.mons.length - 5 }} more
          </p>
        </template>
      </template>
    </template>
    <template v-else>
      <p>Loading map data...</p>
    </template>
  </div>
</template>
