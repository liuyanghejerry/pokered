export interface Warp {
  x: number
  y: number
  dest_map_name?: string
}

export interface MapData {
  id: number
  name: string
  width: number
  height: number
  tileset_name: string
  blocks: number[]
  passable_tiles: number[]
  warps?: Warp[]
}

export interface Blockset {
  tileset_name: string
  blocks: Record<number, number[]>
}

export interface ExportData {
  maps: MapData[]
  blocksets: Blockset[]
}

export type EditorTool = 'view' | 'edit'

export interface DisplayOptions {
  showTiles: boolean
  showCollision: boolean
  showWarps: boolean
  showGrid: boolean
}

export interface TileInfo {
  tileX: number
  tileY: number
  blockX: number
  blockY: number
  blockIndex: number
  blockId: number
  tileId: number
  passable: boolean
}
