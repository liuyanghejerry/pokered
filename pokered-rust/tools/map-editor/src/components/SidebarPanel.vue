<script setup lang="ts">
import { useMapStore } from '../stores/mapStore'
import { storeToRefs } from 'pinia'

const store = useMapStore()
const {
  currentMap,
  filteredMaps,
  currentMapIndex,
  searchQuery,
  displayOptions,
  hasUnsavedChanges,
} = storeToRefs(store)

async function handleFileChange(e: Event) {
  const input = e.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return
  try {
    await store.loadFile(file)
  } catch (err) {
    store.updateStatus(`Error: ${(err as Error).message}`)
  }
}

function handleMapChange(e: Event) {
  const select = e.target as HTMLSelectElement
  store.selectMap(parseInt(select.value))
}

function toHex(n: number, pad = 2): string {
  return '0x' + n.toString(16).padStart(pad, '0')
}
</script>

<template>
  <aside
    class="w-80 shrink-0 bg-bg-panel p-4 rounded-lg overflow-y-auto"
    style="max-height: calc(100vh - 40px)"
  >
    <h2 class="text-accent text-base font-bold mb-4">Map Editor</h2>

    <label class="block text-xs mb-1 mt-3">Data File:</label>
    <input
      type="file"
      accept=".json"
      class="text-[11px] w-full"
      @change="handleFileChange"
    />

    <label class="block text-xs mb-1 mt-3">Map:</label>
    <select
      class="w-full p-1.5 rounded border border-accent bg-bg text-text text-xs"
      :value="currentMapIndex"
      @change="handleMapChange"
    >
      <option
        v-for="{ map, index } in filteredMaps"
        :key="index"
        :value="index"
      >
        {{ map.name }}
      </option>
    </select>

    <label class="block text-xs mb-1 mt-3">Search:</label>
    <input
      v-model="searchQuery"
      type="text"
      placeholder="Filter maps..."
      class="w-full p-1.5 rounded border border-accent bg-bg text-text text-xs"
    />

    <div class="mt-3 space-y-1">
      <label class="flex items-center gap-1.5 cursor-pointer text-xs">
        <input v-model="displayOptions.showTiles" type="checkbox" class="w-auto" />
        Show Tiles
      </label>
      <label class="flex items-center gap-1.5 cursor-pointer text-xs">
        <input v-model="displayOptions.showCollision" type="checkbox" class="w-auto" />
        Show Collision
      </label>
      <label class="flex items-center gap-1.5 cursor-pointer text-xs">
        <input v-model="displayOptions.showWarps" type="checkbox" class="w-auto" />
        Show Warps
      </label>
      <label class="flex items-center gap-1.5 cursor-pointer text-xs">
        <input v-model="displayOptions.showGrid" type="checkbox" class="w-auto" />
        Show Grid
      </label>
    </div>

    <div class="flex gap-1.5 flex-wrap mt-3">
      <button
        class="px-3 py-1.5 bg-accent text-bg-panel border-none rounded cursor-pointer text-[11px] font-bold hover:bg-accent-hover"
        @click="store.exportJson()"
      >
        Export JSON
      </button>
      <button
        class="px-3 py-1.5 bg-[#333] text-text border-none rounded cursor-pointer text-[11px] font-bold hover:bg-[#444]"
        @click="store.prevMap()"
      >
        ◀
      </button>
      <button
        class="px-3 py-1.5 bg-[#333] text-text border-none rounded cursor-pointer text-[11px] font-bold hover:bg-[#444]"
        @click="store.nextMap()"
      >
        ▶
      </button>
    </div>

    <div v-if="hasUnsavedChanges" class="mt-2 text-warning text-[11px] font-bold">
      *Unsaved Changes*
    </div>

    <div class="mt-4 bg-bg-inset p-2.5 rounded-md">
      <h3 class="text-accent text-[13px] font-bold mb-2">Passable Tiles</h3>
      <p class="text-[10px] text-text-muted mb-1">Click tiles on map to toggle passable</p>
      <div
        v-if="currentMap"
        class="max-h-[150px] overflow-y-auto font-mono text-[10px] space-y-0.5"
      >
        <div
          v-for="tileId in currentMap.passable_tiles"
          :key="tileId"
          class="flex items-center gap-1.5 p-0.5 hover:bg-bg"
        >
          <span class="w-[30px]">{{ toHex(tileId) }}</span>
          <span class="text-accent">Passable</span>
        </div>
      </div>
    </div>

    <div class="mt-4 bg-bg-inset p-2.5 rounded-md font-mono text-[11px]">
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
            class="my-0.5 ml-2.5"
          >
            Warp {{ i }}: tiles ({{ warp.x * 2 }}-{{ warp.x * 2 + 1 }}, {{ warp.y * 2 }}-{{ warp.y * 2 + 1 }})
            <template v-if="warp.dest_map_name"> → {{ warp.dest_map_name }}</template>
          </p>
        </template>
        <p class="my-0.5">Passable tiles: {{ currentMap.passable_tiles.length }}</p>
      </template>
      <template v-else>
        <p>Load data file to begin</p>
      </template>
    </div>

    <div class="mt-4 bg-bg-inset p-2.5 rounded-md">
      <h3 class="text-accent text-[13px] font-bold mb-2">Legend</h3>
      <div class="flex items-center gap-2 text-[11px] my-1">
        <div class="w-3.5 h-3.5 rounded-sm" style="background: rgba(78, 204, 163, 0.5)"></div>
        Passable
      </div>
      <div class="flex items-center gap-2 text-[11px] my-1">
        <div class="w-3.5 h-3.5 rounded-sm" style="background: rgba(231, 76, 60, 0.5)"></div>
        Blocked
      </div>
      <div class="flex items-center gap-2 text-[11px] my-1">
        <div class="w-3.5 h-3.5 rounded-sm" style="background: rgba(52, 152, 219, 0.8)"></div>
        Warp
      </div>
    </div>
  </aside>
</template>
