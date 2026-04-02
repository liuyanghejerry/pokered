import { ref, watch, type Ref } from 'vue'
import { useMapStore } from '../stores/mapStore'
import { TILE_SIZE, BLOCK_TILES } from '../types/constants'
import type { TileInfo } from '../types'
import { renderTilesAndCollision, renderGrid } from './renderTiles'
import { renderWarps, renderSigns, renderNpcs, renderSelectionHighlight } from './renderOverlays'
import { getTileInfoAt, hasClickableEntity } from './hitTest'

export function useMapRenderer(canvasRef: Ref<HTMLCanvasElement | null>) {
  const store = useMapStore()
  const tooltip = ref<TileInfo | null>(null)
  const tooltipPosition = ref({ x: 0, y: 0 })
  const hoveringClickable = ref(false)
  let animationFrameId: number | null = null

  function render() {
    const canvas = canvasRef.value
    const map = store.currentMap
    if (!canvas || !map) return

    const ctx = canvas.getContext('2d')
    if (!ctx) return

    const pw = map.width * BLOCK_TILES * TILE_SIZE
    const ph = map.height * BLOCK_TILES * TILE_SIZE
    canvas.width = pw * store.zoom
    canvas.height = ph * store.zoom
    canvas.style.width = `${pw * store.zoom}px`
    canvas.style.height = `${ph * store.zoom}px`

    ctx.imageSmoothingEnabled = false
    ctx.scale(store.zoom, store.zoom)
    ctx.fillStyle = '#000'
    ctx.fillRect(0, 0, pw, ph)

    const { showTiles, showCollision, showGrid, showWarps, showSigns, showNpcs } = store.displayOptions
    const blockset = store.getBlockset(map.tileset_name)
    const tilesetImg = store.tilesetImages[map.tileset_name]

    renderTilesAndCollision(ctx, map, blockset, tilesetImg, showTiles, showCollision)

    if (showGrid) {
      renderGrid(ctx, pw, ph)
    }

    if (showWarps && map.warps) {
      renderWarps(ctx, map.warps)
    }

    if (showSigns && map.signs) {
      renderSigns(ctx, map.signs)
    }

    if (showNpcs && map.npcs) {
      renderNpcs(ctx, map.npcs)
    }

    if (store.selectedEntity) {
      renderSelectionHighlight(ctx, store.selectedEntity)
    }
  }

  function startSelectionAnimation() {
    if (animationFrameId != null) return
    function loop() {
      render()
      if (store.selectedEntity) {
        animationFrameId = requestAnimationFrame(loop)
      } else {
        animationFrameId = null
      }
    }
    loop()
  }

  function stopSelectionAnimation() {
    if (animationFrameId != null) {
      cancelAnimationFrame(animationFrameId)
      animationFrameId = null
    }
  }

  function handleCanvasClick(e: MouseEvent) {
    const canvas = canvasRef.value
    const map = store.currentMap
    if (!canvas || !map) return

    const info = getTileInfoAt(canvas, map, store.getBlockset(map.tileset_name), store.zoom, e.clientX, e.clientY)
    if (!info) return

    if (store.currentTool === 'edit') {
      store.togglePassableTile(info.tileId)
      render()
      return
    }

    if (info.warp) {
      const warpIndex = map.warps?.indexOf(info.warp) ?? -1
      if (info.warp.dest_map_name) {
        store.navigateToMap(info.warp.dest_map_name)
      } else {
        store.selectEntity({ type: 'warp', data: info.warp, index: warpIndex })
      }
      return
    }

    if (info.sign) {
      const signIndex = map.signs?.indexOf(info.sign) ?? -1
      store.selectEntity({ type: 'sign', data: info.sign, index: signIndex })
      return
    }

    if (info.npc) {
      const npcIndex = map.npcs?.indexOf(info.npc) ?? -1
      store.selectEntity({ type: 'npc', data: info.npc, index: npcIndex })
      return
    }

    store.selectEntity(null)
  }

  function handleCanvasMouseMove(e: MouseEvent) {
    const canvas = canvasRef.value
    const map = store.currentMap
    if (!canvas || !map) return

    const info = getTileInfoAt(canvas, map, store.getBlockset(map.tileset_name), store.zoom, e.clientX, e.clientY)
    tooltip.value = info
    tooltipPosition.value = { x: e.clientX + 15, y: e.clientY + 15 }

    if (store.currentTool === 'view' && info) {
      hoveringClickable.value = hasClickableEntity(info)
    } else {
      hoveringClickable.value = false
    }
  }

  function handleCanvasMouseLeave() {
    tooltip.value = null
    hoveringClickable.value = false
  }

  watch(
    () => [
      store.currentMapIndex,
      store.zoom,
      store.displayOptions.showTiles,
      store.displayOptions.showCollision,
      store.displayOptions.showWarps,
      store.displayOptions.showSigns,
      store.displayOptions.showNpcs,
      store.displayOptions.showGrid,
      store.tilesetImages,
    ],
    () => render(),
    { deep: true },
  )

  watch(
    () => store.selectedEntity,
    (entity) => {
      if (entity) {
        startSelectionAnimation()
      } else {
        stopSelectionAnimation()
        render()
      }
    },
  )

  return {
    render,
    tooltip,
    tooltipPosition,
    hoveringClickable,
    handleCanvasClick,
    handleCanvasMouseMove,
    handleCanvasMouseLeave,
  }
}
