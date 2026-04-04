<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from 'vue'
import { useMapStore } from '../stores/mapStore'
import { storeToRefs } from 'pinia'
import { useCodeMirror, parseFunctions, type ScriptFunction } from '../composables/useCodeMirror'
import ScriptFunctionList from './ScriptFunctionList.vue'

const store = useMapStore()
const { currentMap, scriptEditorOpen, scriptJumpTarget, scriptDirty } = storeToRefs(store)

const editorContainer = ref<HTMLElement | null>(null)
const functions = ref<ScriptFunction[]>([])
const activeFunction = ref<string | null>(null)

const cm = useCodeMirror(editorContainer, (content) => {
  if (currentMap.value) {
    store.updateScriptContent(currentMap.value.name, content)
    functions.value = parseFunctions(content)
  }
})

const mapName = computed(() => currentMap.value?.name ?? '')

// Load script when map changes or editor tab becomes visible
watch([mapName, scriptEditorOpen], async ([name, open]) => {
  if (!name || !open) return
  const content = await store.loadScriptFile(name)
  await nextTick()
  if (editorContainer.value) {
    cm.create(content)
    functions.value = parseFunctions(content)
    activeFunction.value = null
  }
}, { immediate: true })

// Handle jump-to-function requests
watch(scriptJumpTarget, (target) => {
  if (target) {
    cm.jumpToFunction(target)
    activeFunction.value = target
    store.clearJumpTarget()
  }
})

function handleFunctionSelect(fn: ScriptFunction) {
  cm.jumpToLine(fn.line)
  activeFunction.value = fn.name
}

async function handleSave() {
  if (!currentMap.value) return
  const content = cm.getContent()
  await store.saveScriptFile(currentMap.value.name, content)
}

// Ctrl+S / Cmd+S
function handleKeydown(e: KeyboardEvent) {
  if ((e.metaKey || e.ctrlKey) && e.key === 's') {
    e.preventDefault()
    if (scriptEditorOpen.value) {
      handleSave()
    }
  }
}

onMounted(() => {
  document.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
  <div class="script-editor-panel">
    <!-- Toolbar row -->
    <div class="panel-header">
      <div class="header-left">
        <span v-if="scriptDirty" class="dirty-badge">Modified</span>
        <span v-else class="text-text-muted text-[10px]">Saved</span>
      </div>
      <div class="header-right">
        <button
          class="save-btn"
          :disabled="!scriptDirty"
          :class="{ disabled: !scriptDirty }"
          @click="handleSave"
        >
          Save Script
        </button>
      </div>
    </div>

    <!-- Body -->
    <div class="panel-body">
      <!-- Function list sidebar -->
      <div class="fn-sidebar">
        <ScriptFunctionList
          :functions="functions"
          :active-function="activeFunction"
          @select="handleFunctionSelect"
        />
      </div>

      <!-- CodeMirror editor -->
      <div class="editor-area" ref="editorContainer"></div>
    </div>
  </div>
</template>

<style scoped>
.script-editor-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--color-bg-panel);
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 4px 10px;
  background: var(--color-bg-inset);
  border-bottom: 1px solid rgba(255, 255, 255, 0.06);
  flex-shrink: 0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
}

.dirty-badge {
  font-size: 10px;
  padding: 1px 5px;
  border-radius: 3px;
  background: rgba(241, 196, 15, 0.2);
  color: var(--color-warning);
}

.header-right {
  display: flex;
  align-items: center;
  gap: 6px;
}

.save-btn {
  padding: 2px 10px;
  font-size: 11px;
  font-weight: bold;
  border: none;
  border-radius: 3px;
  cursor: pointer;
  background: var(--color-accent);
  color: var(--color-bg);
  transition: opacity 0.15s;
}

.save-btn:hover:not(.disabled) {
  opacity: 0.85;
}

.save-btn.disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.panel-body {
  flex: 1;
  display: flex;
  min-height: 0;
  overflow: hidden;
}

.fn-sidebar {
  width: 180px;
  flex-shrink: 0;
  border-right: 1px solid rgba(255, 255, 255, 0.06);
  overflow-y: auto;
  background: var(--color-bg-inset);
}

.editor-area {
  flex: 1;
  min-width: 0;
  overflow: hidden;
}

/* Make CodeMirror fill the editor area */
.editor-area :deep(.cm-editor) {
  height: 100%;
}

.editor-area :deep(.cm-scroller) {
  overflow: auto;
}
</style>
