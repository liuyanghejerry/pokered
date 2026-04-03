import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { ExportData, MapData, Blockset, EditorTool, DisplayOptions, SelectedEntity, MapScriptConfig } from '../types'
import { TILESET_FILES } from '../types/constants'

export const useMapStore = defineStore('map', () => {
  const exportData = ref<ExportData | null>(null)
  const maps = ref<MapData[]>([])
  const blocksets = ref<Blockset[]>([])
  const currentMapIndex = ref(0)
  const zoom = ref(2)
  const currentTool = ref<EditorTool>('view')
  const hasUnsavedChanges = ref(false)
  const statusMessage = ref('Ready')
  const searchQuery = ref('')
  const tilesetImages = ref<Record<string, HTMLImageElement>>({})
  const selectedEntity = ref<SelectedEntity | null>(null)
  const mapHistory = ref<number[]>([])

  const scriptConfigs = ref<Record<string, MapScriptConfig>>({})

const displayOptions = ref<DisplayOptions>({
    showTiles: true,
    showCollision: true,
    showWarps: true,
    showSigns: true,
    showNpcs: true,
    showGrid: false,
    showCoordEvents: true,
  })

  const currentMap = computed<MapData | null>(() => {
    if (maps.value.length === 0) return null
    return maps.value[currentMapIndex.value] ?? null
  })

  const canGoBack = computed(() => mapHistory.value.length > 0)

  const filteredMaps = computed(() => {
    if (!searchQuery.value) return maps.value.map((m, i) => ({ map: m, index: i }))
    const q = searchQuery.value.toLowerCase()
    return maps.value
      .map((m, i) => ({ map: m, index: i }))
      .filter(({ map }) => map.name.toLowerCase().includes(q))
  })

  function getBlockset(tilesetName: string): Blockset | undefined {
    return blocksets.value.find((b) => b.tileset_name === tilesetName)
  }

  async function loadFile(file: File) {
    const text = await file.text()
    const data: ExportData = JSON.parse(text)
    exportData.value = data
    maps.value = data.maps || []
    blocksets.value = data.blocksets || []
    await loadTilesets()
    currentMapIndex.value = 0
    selectedEntity.value = null
    mapHistory.value = []
    updateStatus(`Loaded ${maps.value.length} maps`)
  }

  async function loadTilesets() {
    const basePath = '/gfx/tilesets/'
    const loaded: Record<string, HTMLImageElement> = {}
    const promises = Object.entries(TILESET_FILES).map(async ([name, file]) => {
      try {
        const img = new Image()
        img.src = basePath + file
        await img.decode()
        loaded[name] = img
      } catch {
        console.warn('Could not load tileset:', name)
      }
    })
    await Promise.all(promises)
    tilesetImages.value = loaded
  }

  function selectMap(index: number) {
    if (index >= 0 && index < maps.value.length) {
      currentMapIndex.value = index
      selectedEntity.value = null
    }
  }

  function navigateToMap(mapName: string) {
    const targetIndex = maps.value.findIndex((m) => m.name === mapName)
    if (targetIndex < 0) {
      updateStatus(`Map "${mapName}" not found`)
      return
    }
    mapHistory.value.push(currentMapIndex.value)
    currentMapIndex.value = targetIndex
    selectedEntity.value = null
    updateStatus(`Navigated to ${mapName}`)
  }

  function goBack() {
    const prev = mapHistory.value.pop()
    if (prev != null) {
      currentMapIndex.value = prev
      selectedEntity.value = null
      updateStatus(`Back to ${maps.value[prev]?.name ?? 'unknown'}`)
    }
  }

  function selectEntity(entity: SelectedEntity | null) {
    selectedEntity.value = entity
  }

  function nextMap() {
    if (currentMapIndex.value < maps.value.length - 1) {
      currentMapIndex.value++
      selectedEntity.value = null
    }
  }

  function prevMap() {
    if (currentMapIndex.value > 0) {
      currentMapIndex.value--
      selectedEntity.value = null
    }
  }

  function setTool(tool: EditorTool) {
    currentTool.value = tool
    if (tool === 'edit') {
      selectedEntity.value = null
    }
  }

  function zoomIn() {
    zoom.value = Math.min(4, zoom.value + 1)
  }

  function zoomOut() {
    zoom.value = Math.max(1, zoom.value - 1)
  }

  function togglePassableTile(tileId: number) {
    const map = currentMap.value
    if (!map) return
    const idx = map.passable_tiles.indexOf(tileId)
    if (idx >= 0) {
      map.passable_tiles.splice(idx, 1)
      updateStatus(`Tile 0x${tileId.toString(16).padStart(2, '0')} marked as IMPASSABLE`)
    } else {
      map.passable_tiles.push(tileId)
      updateStatus(`Tile 0x${tileId.toString(16).padStart(2, '0')} marked as PASSABLE`)
    }
    hasUnsavedChanges.value = true
  }

  function exportJson() {
    if (!exportData.value) return
    const json = JSON.stringify(exportData.value, null, 2)
    const blob = new Blob([json], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = 'map_data_edited.json'
    a.click()
    URL.revokeObjectURL(url)
    hasUnsavedChanges.value = false
    updateStatus('Exported to map_data_edited.json')
  }

    function loadScriptConfig(mapName: string, config: MapScriptConfig) {
    scriptConfigs.value[mapName] = config
    const map = maps.value.find((m) => m.name === mapName)
    if (!map) return

    config.npcs.forEach(({ id, talk }) => {
      const npc = map.npcs?.find((n) => n.text_id === id)
      if (npc) npc.talk = talk
    })

    config.signs.forEach(({ id, talk }) => {
      const sign = map.signs?.find((s) => s.text_id === id)
      if (sign) sign.talk = talk
    })
  }

  async function loadScriptConfigFile(file: File) {
    const text = await file.text()
    const config: MapScriptConfig = JSON.parse(text)
    if (currentMap.value) {
      loadScriptConfig(currentMap.value.name, config)
      updateStatus(`Loaded script config for ${currentMap.value.name}`)
    }
  }

  const currentScriptConfig = computed(() => {
    return currentMap.value ? scriptConfigs.value[currentMap.value.name] : undefined
  })

  function updateNpcTalk(npcIndex: number, talk: string) {
    const map = currentMap.value
    if (!map || !map.npcs) return
    const npc = map.npcs[npcIndex]
    if (npc) {
      npc.talk = talk
      const config = scriptConfigs.value[map.name]
      const configNpc = config?.npcs.find((n) => n.id === npc.text_id)
      if (configNpc) configNpc.talk = talk
      hasUnsavedChanges.value = true
    }
  }

  function updateSignTalk(signIndex: number, talk: string) {
    const map = currentMap.value
    if (!map || !map.signs) return
    const sign = map.signs[signIndex]
    if (sign) {
      sign.talk = talk
      const config = scriptConfigs.value[map.name]
      const configSign = config?.signs.find((s) => s.id === sign.text_id)
      if (configSign) configSign.talk = talk
      hasUnsavedChanges.value = true
    }
  }

  function addCoordEvent(x: number, y: number, trigger: string) {
    const config = currentScriptConfig.value
    if (config) {
      config.coordEvents.push({ position: [x, y], trigger })
      hasUnsavedChanges.value = true
    }
  }

  function removeCoordEvent(index: number) {
    const config = currentScriptConfig.value
    if (config) {
      config.coordEvents.splice(index, 1)
      hasUnsavedChanges.value = true
    }
  }

  function updateMapScripts(scripts: string[]) {
    const config = currentScriptConfig.value
    if (config) {
      config.mapScripts = scripts
      hasUnsavedChanges.value = true
    }
  }

  function exportScriptConfig() {
    const config = currentScriptConfig.value
    if (!config || !currentMap.value) return
    const json = JSON.stringify(config, null, 2)
    const blob = new Blob([json], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `${currentMap.value.name}.json`
    a.click()
    URL.revokeObjectURL(url)
    updateStatus(`Exported script config for ${currentMap.value.name}`)
  }

  function updateStatus(msg: string) {
    statusMessage.value = msg
  }

  return {
    exportData,
    maps,
    blocksets,
    currentMapIndex,
    zoom,
    currentTool,
    hasUnsavedChanges,
    statusMessage,
    searchQuery,
    tilesetImages,
    displayOptions,
    selectedEntity,
    mapHistory,
    currentMap,
    canGoBack,
    filteredMaps,
    getBlockset,
    loadFile,
    selectMap,
    navigateToMap,
    goBack,
    selectEntity,
    nextMap,
    prevMap,
    setTool,
    zoomIn,
    zoomOut,
    togglePassableTile,
    exportJson,
    scriptConfigs,
    loadScriptConfig,
    loadScriptConfigFile,
    currentScriptConfig,
    updateNpcTalk,
    updateSignTalk,
    addCoordEvent,
    removeCoordEvent,
    updateMapScripts,
    exportScriptConfig,
    updateStatus,
  }
})
