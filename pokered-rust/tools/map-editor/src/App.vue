<script setup lang="ts">
import { onMounted } from 'vue'
import SidebarPanel from './components/SidebarPanel.vue'
import EditorToolbar from './components/EditorToolbar.vue'
import MapCanvas from './components/MapCanvas.vue'
import ScriptEditorPanel from './components/ScriptEditorPanel.vue'
import { useMapStore } from './stores/mapStore'
import { storeToRefs } from 'pinia'

const store = useMapStore()
const { scriptEditorOpen, currentMap, scriptDirty } = storeToRefs(store)

onMounted(() => {
  store.loadAllMaps()
})
</script>

<template>
  <div class="flex gap-4 h-[calc(100vh-20px)]">
    <SidebarPanel />
    <div class="flex-1 flex flex-col min-h-0 min-w-0">
      <!-- Tab bar -->
      <div class="tab-bar">
        <button
          class="tab-btn"
          :class="{ active: !scriptEditorOpen }"
          @click="store.closeScriptEditor()"
        >
          <span class="tab-icon">🗺</span>
          Map
        </button>
        <button
          class="tab-btn"
          :class="{ active: scriptEditorOpen }"
          @click="store.openScriptEditor()"
        >
          <span class="tab-icon">{ }</span>
          Script
          <span v-if="scriptDirty" class="tab-dot"></span>
        </button>
        <!-- Toolbar sits to the right of tabs when on Map tab -->
        <div v-if="!scriptEditorOpen" class="tab-toolbar">
          <EditorToolbar />
        </div>
        <!-- Script file name when on Script tab -->
        <div v-else class="tab-toolbar">
          <span class="text-text-muted text-xs font-mono">{{ currentMap?.name ?? '' }}/script.js</span>
        </div>
      </div>

      <!-- Tab content -->
      <div class="flex-1 min-h-0 relative">
        <div v-show="!scriptEditorOpen" class="absolute inset-0">
          <MapCanvas />
        </div>
        <div v-show="scriptEditorOpen" class="absolute inset-0">
          <ScriptEditorPanel />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.tab-bar {
  display: flex;
  align-items: center;
  gap: 0;
  background: var(--color-bg-inset);
  border-bottom: 1px solid rgba(255, 255, 255, 0.06);
  padding: 0 0 0 0;
  flex-shrink: 0;
}

.tab-btn {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 7px 16px;
  font-size: 12px;
  font-weight: 600;
  border: none;
  background: transparent;
  color: var(--color-text-muted);
  cursor: pointer;
  border-bottom: 2px solid transparent;
  transition: color 0.15s, border-color 0.15s, background 0.15s;
  position: relative;
}

.tab-btn:hover {
  color: var(--color-text);
  background: rgba(255, 255, 255, 0.03);
}

.tab-btn.active {
  color: var(--color-accent);
  border-bottom-color: var(--color-accent);
  background: rgba(78, 204, 163, 0.05);
}

.tab-icon {
  font-family: monospace;
  font-size: 11px;
}

.tab-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--color-warning);
  flex-shrink: 0;
}

.tab-toolbar {
  flex: 1;
  display: flex;
  align-items: center;
  padding: 0 10px;
}
</style>
