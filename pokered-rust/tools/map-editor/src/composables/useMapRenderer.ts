import { ref, watch, type Ref } from 'vue'
import { useMapStore } from '../stores/mapStore'
import { TILE_SIZE, BLOCK_TILES } from '../types/constants'
import type { TileInfo } from '../types'

export function useMapRenderer(canvasRef: Ref<HTMLCanvasElement | null>) {
  const store = useMapStore()
  const tooltip = ref<TileInfo | null>(null)
  const tooltipPosition = ref({ x: 0, y: 0 })

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

    const { showTiles, showCollision, showGrid, showWarps } = store.displayOptions
    const blockset = store.getBlockset(map.tileset_name)
    const tilesetImg = store.tilesetImages[map.tileset_name]

    for (let by = 0; by < map.height; by++) {
      for (let bx = 0; bx < map.width; bx++) {
        const blockIdx = by * map.width + bx
        const blockId = map.blocks[blockIdx]
        const px = bx * BLOCK_TILES * TILE_SIZE
        const py = by * BLOCK_TILES * TILE_SIZE

        if (showTiles && blockset && tilesetImg) {
          const blockTiles = blockset.blocks[blockId]
          if (blockTiles) {
            const tilesPerRow = Math.floor(tilesetImg.width / TILE_SIZE)
            for (let ty = 0; ty < 4; ty++) {
              for (let tx = 0; tx < 4; tx++) {
                const tileId = blockTiles[ty * 4 + tx]
                const srcX = (tileId % tilesPerRow) * TILE_SIZE
                const srcY = Math.floor(tileId / tilesPerRow) * TILE_SIZE
                ctx.drawImage(
                  tilesetImg,
                  srcX, srcY, TILE_SIZE, TILE_SIZE,
                  px + tx * TILE_SIZE, py + ty * TILE_SIZE, TILE_SIZE, TILE_SIZE,
                )
              }
            }
          }
        } else {
          ctx.fillStyle = `hsl(${blockId * 7}, 50%, 40%)`
          ctx.fillRect(px, py, BLOCK_TILES * TILE_SIZE, BLOCK_TILES * TILE_SIZE)
        }

        if (showCollision && blockset && blockset.blocks[blockId]) {
          const blockTiles = blockset.blocks[blockId]
          for (let ty = 0; ty < 4; ty++) {
            for (let tx = 0; tx < 4; tx++) {
              const tileId = blockTiles[ty * 4 + tx]
              const passable = map.passable_tiles.includes(tileId)
              ctx.fillStyle = passable
                ? 'rgba(78, 204, 163, 0.25)'
                : 'rgba(231, 76, 60, 0.35)'
              ctx.fillRect(
                px + tx * TILE_SIZE,
                py + ty * TILE_SIZE,
                TILE_SIZE,
                TILE_SIZE,
              )
            }
          }
        }
      }
    }

    if (showGrid) {
      ctx.strokeStyle = 'rgba(255, 255, 0, 0.3)'
      ctx.lineWidth = 0.5
      for (let x = 0; x <= pw; x += BLOCK_TILES * TILE_SIZE) {
        ctx.beginPath()
        ctx.moveTo(x, 0)
        ctx.lineTo(x, ph)
        ctx.stroke()
      }
      for (let y = 0; y <= ph; y += BLOCK_TILES * TILE_SIZE) {
        ctx.beginPath()
        ctx.moveTo(0, y)
        ctx.lineTo(pw, y)
        ctx.stroke()
      }
    }

    if (showWarps && map.warps) {
      map.warps.forEach((warp) => {
        const tileX = warp.x * 2
        const tileY = warp.y * 2
        const wpx = tileX * TILE_SIZE
        const wpy = tileY * TILE_SIZE
        ctx.fillStyle = 'rgba(52, 152, 219, 0.8)'
        ctx.fillRect(wpx, wpy, TILE_SIZE * 2, TILE_SIZE * 2)
        ctx.strokeStyle = '#3498db'
        ctx.lineWidth = 2
        ctx.strokeRect(wpx, wpy, TILE_SIZE * 2, TILE_SIZE * 2)
      })
    }
  }

  function getTileInfoAt(clientX: number, clientY: number): TileInfo | null {
    const canvas = canvasRef.value
    const map = store.currentMap
    if (!canvas || !map) return null

    const rect = canvas.getBoundingClientRect()
    const x = Math.floor((clientX - rect.left) / (TILE_SIZE * store.zoom))
    const y = Math.floor((clientY - rect.top) / (TILE_SIZE * store.zoom))
    const blockX = Math.floor(x / BLOCK_TILES)
    const blockY = Math.floor(y / BLOCK_TILES)
    const subX = x % BLOCK_TILES
    const subY = y % BLOCK_TILES
    const blockIndex = blockY * map.width + blockX
    const blockId = map.blocks?.[blockIndex] ?? 0

    let tileId = 0
    let passable = false
    const blockset = store.getBlockset(map.tileset_name)
    if (blockset && blockset.blocks[blockId]) {
      tileId = blockset.blocks[blockId][subY * 4 + subX]
      passable = map.passable_tiles.includes(tileId)
    }

    return { tileX: x, tileY: y, blockX, blockY, blockIndex, blockId, tileId, passable }
  }

  function handleCanvasClick(e: MouseEvent) {
    if (store.currentTool !== 'edit') return
    const info = getTileInfoAt(e.clientX, e.clientY)
    if (!info) return
    store.togglePassableTile(info.tileId)
    render()
  }

  function handleCanvasMouseMove(e: MouseEvent) {
    const info = getTileInfoAt(e.clientX, e.clientY)
    tooltip.value = info
    tooltipPosition.value = { x: e.clientX + 15, y: e.clientY + 15 }
  }

  function handleCanvasMouseLeave() {
    tooltip.value = null
  }

  watch(
    () => [
      store.currentMapIndex,
      store.zoom,
      store.displayOptions.showTiles,
      store.displayOptions.showCollision,
      store.displayOptions.showWarps,
      store.displayOptions.showGrid,
      store.tilesetImages,
    ],
    () => render(),
    { deep: true },
  )

  return {
    render,
    tooltip,
    tooltipPosition,
    handleCanvasClick,
    handleCanvasMouseMove,
    handleCanvasMouseLeave,
  }
}
