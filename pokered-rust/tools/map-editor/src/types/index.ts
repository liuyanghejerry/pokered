export interface TextPage {
  line1: string
  line2: string
}

export interface Warp {
  x: number
  y: number
  dest_map?: number
  dest_map_name?: string
  dest_warp_id?: number
}

export interface Sign {
  talk?: string  // JS function name binding (from script config)
  x: number
  y: number
  text_id: number
  text_pages?: TextPage[]
}

export interface NpcData {
  talk?: string  // JS function name binding (from script config)
  sprite_id: number
  sprite_name: string
  x: number
  y: number
  movement: string
  facing: string
  range: number
  text_id: number
  is_trainer: boolean
  trainer_class?: string
  trainer_set?: number
  item_id?: number
  text_pages?: TextPage[]
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
  signs?: Sign[]
  npcs?: NpcData[]
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
  showCoordEvents: boolean
  showTiles: boolean
  showCollision: boolean
  showWarps: boolean
  showSigns: boolean
  showNpcs: boolean
  showGrid: boolean
}

export interface CoordEvent {
  x: number
  y: number
  trigger: string  // JS function name
}

export interface MapScriptConfig {
  mapScripts: string[]
  npcs: { id: number; talk: string }[]
  signs: { id: number; talk: string }[]
  coordEvents: { position: [number, number]; trigger: string }[]
}

export interface TileInfo {
  coordEvent?: CoordEvent
  tileX: number
  tileY: number
  blockX: number
  blockY: number
  blockIndex: number
  blockId: number
  tileId: number
  passable: boolean
  warp?: Warp
  sign?: Sign
  npc?: NpcData
}

export type SelectedEntity =
  | { type: 'coordEvent'; data: CoordEvent; index: number }
  | { type: 'sign'; data: Sign; index: number }
  | { type: 'npc'; data: NpcData; index: number }
  | { type: 'warp'; data: Warp; index: number }
