export interface TextPage {
  line1: string
  line2: string
}

export interface MapHeaderJson {
  tileset: string
  music: string
  connectionFlags: number
  width: number
  height: number
  borderBlock: number
}

export interface ConnectionEntry {
  targetMap: string
  offset: number
}

export interface ConnectionsJson {
  north?: ConnectionEntry
  south?: ConnectionEntry
  west?: ConnectionEntry
  east?: ConnectionEntry
}

export interface WarpJson {
  x: number
  y: number
  destMap?: string
  destWarpId: number
}

export interface NpcJson {
  spriteId: number
  spriteName?: string
  x: number
  y: number
  movement: string
  facing: string
  range: number
  textId: number
  isTrainer: boolean
  trainerClass?: string
  trainerSet?: number
  itemId?: number
  // Runtime-only: JS function binding from script_config.json
  talk?: string
}

export interface SignJson {
  x: number
  y: number
  textId: number
  // Runtime-only: JS function binding from script_config.json
  talk?: string
}

export interface MapTextJson {
  npc?: Record<string, TextPage[]>
  sign?: Record<string, TextPage[]>
}

export interface WildMonJson {
  level: number
  species: string
}

export interface WildEncounterTableJson {
  encounterRate: number
  mons: WildMonJson[]
}

export interface VersionWildJson {
  grass: WildEncounterTableJson
  water: WildEncounterTableJson
}

export interface WildDataJson {
  red?: VersionWildJson
  blue?: VersionWildJson
}

export interface MapJson {
  id: number
  name: string
  header: MapHeaderJson
  connections: ConnectionsJson
  warps: WarpJson[]
  npcs: NpcJson[]
  signs: SignJson[]
  text: MapTextJson
  wild: WildDataJson | null
}

export interface MapScriptConfig {
  npcs: { id: number; talk: string }[]
  signs: { id: number; talk: string }[]
  coordEvents: { position: [number, number]; trigger: string }[]
}

export interface Blockset {
  tileset_name: string
  blocks: Record<number, number[]>
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
  showConnections: boolean
}

export interface CoordEvent {
  x: number
  y: number
  trigger: string
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
  warp?: WarpJson
  sign?: SignJson
  npc?: NpcJson
}

export type SelectedEntity =
  | { type: 'coordEvent'; data: CoordEvent; index: number }
  | { type: 'sign'; data: SignJson; index: number }
  | { type: 'npc'; data: NpcJson; index: number }
  | { type: 'warp'; data: WarpJson; index: number }
