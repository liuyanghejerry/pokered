<script setup lang="ts">
import SidebarPanel from './components/SidebarPanel.vue'
import EditorToolbar from './components/EditorToolbar.vue'
import MapCanvas from './components/MapCanvas.vue'
</script>

<template>
  <div class="flex gap-4 min-h-[calc(100vh-20px)]">
    <SidebarPanel />
    <div class="flex-1 flex flex-col">
      <EditorToolbar />
      <MapCanvas />
    </div>
  </div>
</template>
