import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type {
  MapJson, MapScriptConfig, Blockset, EditorTool, DisplayOptions,
  SelectedEntity, ConnectionEntry,
} from '../types'
import { TILESET_FILES, MUSIC_LIST } from '../types/constants'

export const useMapStore = defineStore('map', () => {
  const maps = ref<MapJson[]>([])
  const blockData = ref<Record<string, number[]>>({})
  const blocksets = ref<Record<string, Record<number, number[]>>>({})
  const passableTiles = ref<Record<string, number[]>>({})
  const currentMapIndex = ref(0)
  const zoom = ref(2)
  const currentTool = ref<EditorTool>('view')
  const hasUnsavedChanges = ref(false)
  const statusMessage = ref('Loading...')
  const searchQuery = ref('')
  const tilesetImages = ref<Record<string, HTMLImageElement>>({})
  const selectedEntity = ref<SelectedEntity | null>(null)
  const mapHistory = ref<number[]>([])
  const scriptConfigs = ref<Record<string, MapScriptConfig>>({})
  const loading = ref(false)
  const scriptFiles = ref<Record<string, string>>({})
  const scriptEditorOpen = ref(false)
  const scriptJumpTarget = ref<string | null>(null)
  const scriptDirty = ref(false)

  const displayOptions = ref<DisplayOptions>({
    showTiles: true,
    showCollision: true,
    showWarps: true,
    showSigns: true,
    showNpcs: true,
    showGrid: false,
    showCoordEvents: true,
    showConnections: true,
  })

  const currentMap = computed<MapJson | null>(() => {
    if (maps.value.length === 0) return null
    return maps.value[currentMapIndex.value] ?? null
  })

  const currentBlocks = computed<number[]>(() => {
    const map = currentMap.value
    if (!map) return []
    return blockData.value[map.name] ?? []
  })

  const currentPassableTiles = computed<number[]>(() => {
    const map = currentMap.value
    if (!map) return []
    return passableTiles.value[map.header.tileset] ?? []
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
    const blocks = blocksets.value[tilesetName]
    if (!blocks) return undefined
    return { tileset_name: tilesetName, blocks }
  }

  async function loadAllMaps() {
    loading.value = true
    statusMessage.value = 'Loading maps...'
    try {
      const [mapNames, allBlocksets, allPassable] = await Promise.all([
        fetch('/api/maps').then(r => r.json()) as Promise<string[]>,
        fetch('/api/blocksets').then(r => r.json()) as Promise<Record<string, Record<number, number[]>>>,
        fetch('/api/passable-tiles').then(r => r.json()) as Promise<Record<string, number[]>>,
      ])

      blocksets.value = allBlocksets
      passableTiles.value = allPassable

      const batchSize = 50
      const allMaps: MapJson[] = []
      const allBlocks: Record<string, number[]> = {}
      const allConfigs: Record<string, MapScriptConfig> = {}

      for (let i = 0; i < mapNames.length; i += batchSize) {
        const batch = mapNames.slice(i, i + batchSize)
        const results = await Promise.all(
          batch.map(async (name) => {
            const [mapJson, blk, config] = await Promise.all([
              fetch(`/api/maps/${name}/map.json`).then(r => r.json()) as Promise<MapJson>,
              fetch(`/api/maps/${name}/map.blk`).then(r => r.json()).catch(() => []) as Promise<number[]>,
              fetch(`/api/maps/${name}/script_config.json`).then(r => r.ok ? r.json() : null).catch(() => null) as Promise<MapScriptConfig | null>,
            ])
            return { name, mapJson, blk, config }
          })
        )
        for (const { name, mapJson, blk, config } of results) {
          allMaps.push(mapJson)
          allBlocks[name] = blk
          if (config) {
            allConfigs[name] = config
            applyScriptBindings(mapJson, config)
          }
        }
        statusMessage.value = `Loading maps... ${Math.min(i + batchSize, mapNames.length)}/${mapNames.length}`
      }

      allMaps.sort((a, b) => a.id - b.id)
      maps.value = allMaps
      blockData.value = allBlocks
      scriptConfigs.value = allConfigs

      await loadTilesets()
      currentMapIndex.value = 0
      selectedEntity.value = null
      mapHistory.value = []
      statusMessage.value = `Loaded ${allMaps.length} maps`
    } catch (err) {
      statusMessage.value = `Error: ${(err as Error).message}`
    } finally {
      loading.value = false
    }
  }

  function applyScriptBindings(mapJson: MapJson, config: MapScriptConfig) {
    config.npcs.forEach(({ id, talk }) => {
      const npc = mapJson.npcs?.find(n => n.textId === id)
      if (npc) npc.talk = talk
    })
    config.signs.forEach(({ id, talk }) => {
      const sign = mapJson.signs?.find(s => s.textId === id)
      if (sign) sign.talk = talk
    })
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
    const targetIndex = maps.value.findIndex(m => m.name === mapName)
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
      const configNpc = config?.npcs.find(n => n.id === npc.textId)
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
      const configSign = config?.signs.find(s => s.id === sign.textId)
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
      selectedEntity.value = null
    }
  }

  function updateCoordEvent(index: number, updates: { x?: number; y?: number; trigger?: string }) {
    const config = currentScriptConfig.value
    if (!config || index < 0 || index >= config.coordEvents.length) return
    const ce = config.coordEvents[index]
    if (updates.x != null) ce.position[0] = updates.x
    if (updates.y != null) ce.position[1] = updates.y
    if (updates.trigger != null) ce.trigger = updates.trigger
    hasUnsavedChanges.value = true
    if (selectedEntity.value?.type === 'coordEvent' && selectedEntity.value.index === index) {
      selectedEntity.value = {
        type: 'coordEvent',
        data: { x: ce.position[0], y: ce.position[1], trigger: ce.trigger },
        index,
      }
    }
  }

  function updateMapScripts(scripts: string[]) {
    const config = currentScriptConfig.value
    if (config) {
      config.mapScripts = scripts
      hasUnsavedChanges.value = true
    }
  }

  async function saveCurrentMap() {
    const map = currentMap.value
    if (!map) return

    try {
      const mapCopy = { ...map } as Record<string, unknown>
      if (map.npcs) {
        mapCopy.npcs = map.npcs.map(({ talk, ...rest }) => rest)
      }
      if (map.signs) {
        mapCopy.signs = map.signs.map(({ talk, ...rest }) => rest)
      }

      await fetch(`/api/maps/${map.name}/map.json`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(mapCopy),
      })

      const config = scriptConfigs.value[map.name]
      if (config) {
        await fetch(`/api/maps/${map.name}/script_config.json`, {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(config),
        })
      }

      hasUnsavedChanges.value = false
      updateStatus(`Saved ${map.name}`)
    } catch (err) {
      updateStatus(`Save error: ${(err as Error).message}`)
    }
  }

  async function loadScriptFile(mapName: string): Promise<string> {
    if (scriptFiles.value[mapName] != null) {
      return scriptFiles.value[mapName]
    }
    const res = await fetch(`/api/maps/${mapName}/script.js`)
    const text = res.ok ? await res.text() : ''
    scriptFiles.value[mapName] = text
    return text
  }

  async function saveScriptFile(mapName: string, content: string) {
    try {
      await fetch(`/api/maps/${mapName}/script.js`, {
        method: 'PUT',
        headers: { 'Content-Type': 'text/plain' },
        body: content,
      })
      scriptFiles.value[mapName] = content
      scriptDirty.value = false
      updateStatus(`Saved script for ${mapName}`)
    } catch (err) {
      updateStatus(`Script save error: ${(err as Error).message}`)
    }
  }

  function updateScriptContent(mapName: string, content: string) {
    scriptFiles.value[mapName] = content
    scriptDirty.value = true
  }

  function openScriptEditor() {
    scriptEditorOpen.value = true
  }

  function closeScriptEditor() {
    scriptEditorOpen.value = false
    scriptJumpTarget.value = null
  }

  function jumpToFunction(funcName: string) {
    scriptEditorOpen.value = true
    scriptJumpTarget.value = funcName
  }

  function clearJumpTarget() {
    scriptJumpTarget.value = null
  }

  function updateStatus(msg: string) {
    statusMessage.value = msg
  }

  function updateMapMusic(music: string) {
    const map = currentMap.value
    if (!map) return
    map.header.music = music
    hasUnsavedChanges.value = true
  }

  function updateMapConnection(direction: 'north' | 'south' | 'west' | 'east', entry: ConnectionEntry | null) {
    const map = currentMap.value
    if (!map) return
    if (entry) {
      map.connections[direction] = entry
    } else {
      delete map.connections[direction]
    }
    hasUnsavedChanges.value = true
  }

  function getMapNames(): string[] {
    return maps.value.map(m => m.name)
  }

  return {
    maps,
    blockData,
    blocksets,
    passableTiles,
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
    loading,
    currentMap,
    currentBlocks,
    currentPassableTiles,
    canGoBack,
    filteredMaps,
    getBlockset,
    loadAllMaps,
    selectMap,
    navigateToMap,
    goBack,
    selectEntity,
    nextMap,
    prevMap,
    setTool,
    zoomIn,
    zoomOut,
    scriptConfigs,
    currentScriptConfig,
    updateNpcTalk,
    updateSignTalk,
    addCoordEvent,
    removeCoordEvent,
    updateCoordEvent,
    updateMapScripts,
    saveCurrentMap,
    updateStatus,
    scriptFiles,
    scriptEditorOpen,
    scriptJumpTarget,
    scriptDirty,
    loadScriptFile,
    saveScriptFile,
    updateScriptContent,
    openScriptEditor,
    closeScriptEditor,
    jumpToFunction,
    clearJumpTarget,
    updateMapMusic,
    updateMapConnection,
    getMapNames,
    MUSIC_LIST,
  }
})
