import { TILE_SIZE, BLOCK_TILES } from '../types/constants'
import type { MapData, Blockset, TileInfo, Warp, Sign, NpcData } from '../types'

export function getTileInfoAt(
  canvas: HTMLCanvasElement,
  map: MapData,
  blockset: Blockset | undefined,
  zoom: number,
  clientX: number,
  clientY: number,
): TileInfo | null {
  const rect = canvas.getBoundingClientRect()
  const x = Math.floor((clientX - rect.left) / (TILE_SIZE * zoom))
  const y = Math.floor((clientY - rect.top) / (TILE_SIZE * zoom))
  const blockX = Math.floor(x / BLOCK_TILES)
  const blockY = Math.floor(y / BLOCK_TILES)

  if (blockX < 0 || blockX >= map.width || blockY < 0 || blockY >= map.height) {
    return null
  }

  const subX = x % BLOCK_TILES
  const subY = y % BLOCK_TILES
  const blockIndex = blockY * map.width + blockX
  const blockId = map.blocks?.[blockIndex] ?? 0

  let tileId = 0
  let passable = false
  if (blockset && blockset.blocks[blockId]) {
    tileId = blockset.blocks[blockId][subY * 4 + subX]
    passable = map.passable_tiles.includes(tileId)
  }

  const metaX = Math.floor(x / 2)
  const metaY = Math.floor(y / 2)

  let warp: Warp | undefined
  if (map.warps) {
    warp = map.warps.find((w) => w.x === metaX && w.y === metaY)
  }

  let sign: Sign | undefined
  if (map.signs) {
    sign = map.signs.find((s) => s.x === metaX && s.y === metaY)
  }

  let npc: NpcData | undefined
  if (map.npcs) {
    npc = map.npcs.find((n) => n.x === metaX && n.y === metaY)
  }

  return { tileX: x, tileY: y, blockX, blockY, blockIndex, blockId, tileId, passable, warp, sign, npc }
}

export function hasClickableEntity(info: TileInfo): boolean {
  return !!(info.warp || info.sign || info.npc)
}
