//! Tileset header data — counter tiles, grass tile, and animation type.
//!
//! Ported from data/tilesets/tileset_headers.asm.

use crate::tilesets::{TilesetId, NUM_TILESETS};

/// Tile animation type for a tileset.
/// Controls which tiles are animated when the map is displayed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileAnimation {
    /// No animated tiles.
    None,
    /// Water tiles only (used in caves, ship, etc.).
    Water,
    /// Water + flower tiles (used in overworld, gyms, etc.).
    WaterFlower,
}

/// Header data for a tileset — metadata used by the overworld engine.
#[derive(Debug, Clone, Copy)]
pub struct TilesetHeader {
    /// Up to 3 counter tiles (NPC talks across these like a shop counter).
    /// None means no counter tile at that slot.
    pub counter_tiles: [Option<u8>; 3],
    /// The grass tile ID for this tileset (triggers wild encounters).
    /// None means no grass tile.
    pub grass_tile: Option<u8>,
    /// Animation type for this tileset.
    pub animation: TileAnimation,
}

impl TilesetHeader {
    /// Check if a tile is a counter tile for this tileset.
    pub fn is_counter_tile(&self, tile_id: u8) -> bool {
        self.counter_tiles.iter().any(|t| *t == Some(tile_id))
    }

    /// Check if a tile is the grass tile for this tileset.
    pub fn is_grass_tile(&self, tile_id: u8) -> bool {
        self.grass_tile == Some(tile_id)
    }
}

/// Helper to build a TilesetHeader with optional counter tiles and grass tile.
/// -1 in the ASM means "no tile" (0xFF), which we represent as None.
const fn header(c0: i16, c1: i16, c2: i16, grass: i16, animation: TileAnimation) -> TilesetHeader {
    TilesetHeader {
        counter_tiles: [
            if c0 < 0 { None } else { Some(c0 as u8) },
            if c1 < 0 { None } else { Some(c1 as u8) },
            if c2 < 0 { None } else { Some(c2 as u8) },
        ],
        grass_tile: if grass < 0 { None } else { Some(grass as u8) },
        animation,
    }
}

/// Tileset header data for all 24 tilesets.
/// Index by TilesetId as usize.
///
/// From data/tilesets/tileset_headers.asm:
/// ```text
/// tileset Name, counter1, counter2, counter3, grass, animation
/// ```
pub const TILESET_HEADERS: [TilesetHeader; NUM_TILESETS] = [
    //  0: Overworld     — no counters, grass=0x52, WATER_FLOWER
    header(-1, -1, -1, 0x52, TileAnimation::WaterFlower),
    //  1: RedsHouse1    — no counters, no grass, NONE
    header(-1, -1, -1, -1, TileAnimation::None),
    //  2: Mart          — counters=0x18,0x19,0x1E, no grass, NONE
    header(0x18, 0x19, 0x1E, -1, TileAnimation::None),
    //  3: Forest        — no counters, grass=0x20, WATER
    header(-1, -1, -1, 0x20, TileAnimation::Water),
    //  4: RedsHouse2    — no counters, no grass, NONE
    header(-1, -1, -1, -1, TileAnimation::None),
    //  5: Dojo          — counter=0x3A, no grass, WATER_FLOWER
    header(0x3A, -1, -1, -1, TileAnimation::WaterFlower),
    //  6: Pokecenter    — counters=0x18,0x19,0x1E, no grass, NONE
    header(0x18, 0x19, 0x1E, -1, TileAnimation::None),
    //  7: Gym           — counter=0x3A, no grass, WATER_FLOWER
    header(0x3A, -1, -1, -1, TileAnimation::WaterFlower),
    //  8: House         — no counters, no grass, NONE
    header(-1, -1, -1, -1, TileAnimation::None),
    //  9: ForestGate    — counters=0x17,0x32, no grass, NONE
    header(0x17, 0x32, -1, -1, TileAnimation::None),
    // 10: Museum        — counters=0x17,0x32, no grass, NONE
    header(0x17, 0x32, -1, -1, TileAnimation::None),
    // 11: Underground   — no counters, no grass, NONE
    header(-1, -1, -1, -1, TileAnimation::None),
    // 12: Gate          — counters=0x17,0x32, no grass, NONE
    header(0x17, 0x32, -1, -1, TileAnimation::None),
    // 13: Ship          — no counters, no grass, WATER
    header(-1, -1, -1, -1, TileAnimation::Water),
    // 14: ShipPort      — no counters, no grass, WATER
    header(-1, -1, -1, -1, TileAnimation::Water),
    // 15: Cemetery      — counter=0x12, no grass, NONE
    header(0x12, -1, -1, -1, TileAnimation::None),
    // 16: Interior      — no counters, no grass, NONE
    header(-1, -1, -1, -1, TileAnimation::None),
    // 17: Cavern        — no counters, no grass, WATER
    header(-1, -1, -1, -1, TileAnimation::Water),
    // 18: Lobby         — counters=0x15,0x36, no grass, NONE
    header(0x15, 0x36, -1, -1, TileAnimation::None),
    // 19: Mansion       — no counters, no grass, NONE
    header(-1, -1, -1, -1, TileAnimation::None),
    // 20: Lab           — no counters, no grass, NONE
    header(-1, -1, -1, -1, TileAnimation::None),
    // 21: Club          — counters=0x07,0x17, no grass, NONE
    header(0x07, 0x17, -1, -1, TileAnimation::None),
    // 22: Facility      — counter=0x12, no grass, WATER
    header(0x12, -1, -1, -1, TileAnimation::Water),
    // 23: Plateau       — no counters, grass=0x45, WATER
    header(-1, -1, -1, 0x45, TileAnimation::Water),
];

/// Get the tileset header data for a tileset.
pub fn get_tileset_header(tileset: TilesetId) -> &'static TilesetHeader {
    &TILESET_HEADERS[tileset as usize]
}

/// Get the grass tile for a tileset, if any.
pub fn get_grass_tile(tileset: TilesetId) -> Option<u8> {
    TILESET_HEADERS[tileset as usize].grass_tile
}

/// Check if a tileset has animated water tiles.
pub fn has_water_animation(tileset: TilesetId) -> bool {
    matches!(
        TILESET_HEADERS[tileset as usize].animation,
        TileAnimation::Water | TileAnimation::WaterFlower
    )
}

// ── Cut Tree Block Swaps ─────────────────────────────────────────────

/// Cut tree tile block swaps: (original_block, replacement_block).
/// When a tree is cut, the block containing the tree is replaced.
/// From data/tilesets/cut_tree_blocks.asm.
pub const CUT_TREE_BLOCK_SWAPS: &[(u8, u8)] = &[
    (0x32, 0x6D),
    (0x33, 0x6C),
    (0x34, 0x6F),
    (0x35, 0x4C),
    (0x60, 0x6E),
    (0x0B, 0x0A),
    (0x3C, 0x35),
    (0x3F, 0x35),
    (0x3D, 0x36),
];

/// Look up the replacement block for a cut tree.
/// Returns the block ID that should replace the original after cutting.
pub fn cut_tree_replacement(original_block: u8) -> Option<u8> {
    CUT_TREE_BLOCK_SWAPS
        .iter()
        .find(|(orig, _)| *orig == original_block)
        .map(|(_, repl)| *repl)
}

// ── Cut Tile Constants ───────────────────────────────────────────────

/// Overworld cuttable tree tile.
pub const CUT_TREE_TILE_OVERWORLD: u8 = 0x3D;
/// Gym cuttable tree tile.
pub const CUT_TREE_TILE_GYM: u8 = 0x50;
/// Cuttable grass tile (on overworld).
pub const CUT_GRASS_TILE: u8 = 0x52;

// ── Dungeon Tilesets ─────────────────────────────────────────────────

/// Tilesets that are considered "dungeons" (indoors/caves where Dig/Escape Rope work).
/// From data/tilesets/dungeon_tilesets.asm.
pub const DUNGEON_TILESETS: &[TilesetId] = &[
    TilesetId::Forest,
    TilesetId::Museum,
    TilesetId::Ship,
    TilesetId::Cavern,
    TilesetId::Lobby,
    TilesetId::Mansion,
    TilesetId::Gate,
    TilesetId::Lab,
    TilesetId::Facility,
    TilesetId::Cemetery,
    TilesetId::Gym,
];

/// Check if a tileset is a dungeon tileset.
pub fn is_dungeon_tileset(tileset: TilesetId) -> bool {
    DUNGEON_TILESETS.contains(&tileset)
}

// ── Water Tilesets ───────────────────────────────────────────────────

/// Tilesets that contain water tiles (where Surf can be used).
/// From data/tilesets/water_tilesets.asm.
pub const WATER_TILESETS: &[TilesetId] = &[
    TilesetId::Overworld,
    TilesetId::Forest,
    TilesetId::Dojo,
    TilesetId::Gym,
    TilesetId::Ship,
    TilesetId::ShipPort,
    TilesetId::Cavern,
    TilesetId::Facility,
    TilesetId::Plateau,
];

/// Check if a tileset has water tiles (and thus supports Surf).
pub fn is_water_tileset(tileset: TilesetId) -> bool {
    WATER_TILESETS.contains(&tileset)
}

// ── Outside Map Check ────────────────────────────────────────────────

/// Check if a tileset represents an "outside" map (town or route).
/// Outside maps use OVERWORLD or PLATEAU tilesets.
/// From home/overworld.asm CheckIfInOutsideMap.
pub fn is_outside_tileset(tileset: TilesetId) -> bool {
    matches!(tileset, TilesetId::Overworld | TilesetId::Plateau)
}

// ── Boulder Constants ────────────────────────────────────────────────

/// The movement byte2 value that identifies a sprite as a pushable boulder.
pub const BOULDER_MOVEMENT_BYTE_2: u8 = 0x10;

// ── Ledge Tiles ──────────────────────────────────────────────────────

/// Represents a joypad input direction for ledge jumping.
/// These match the Game Boy PAD constants used in ledge_tiles.asm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PadInput {
    Down,
    Left,
    Right,
}

/// Sprite facing direction constants matching the ASM values.
/// SPRITE_FACING_DOWN = 0, SPRITE_FACING_UP = 4, SPRITE_FACING_LEFT = 8, SPRITE_FACING_RIGHT = 12.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpriteFacing {
    Down = 0,
    Up = 4,
    Left = 8,
    Right = 12,
}

/// A single ledge tile entry.
/// Ledge jumping only works on the OVERWORLD tileset.
/// From data/tilesets/ledge_tiles.asm.
#[derive(Debug, Clone, Copy)]
pub struct LedgeTileEntry {
    /// Direction the player must be facing.
    pub facing: SpriteFacing,
    /// Tile the player is standing on.
    pub standing_tile: u8,
    /// Tile in front of the player (the ledge).
    pub ledge_tile: u8,
    /// Joypad input required (must be held).
    pub required_input: PadInput,
}

/// All 8 ledge tile entries from data/tilesets/ledge_tiles.asm.
/// Ledge jumping is only active on the OVERWORLD tileset (id 0).
pub const LEDGE_TILES: &[LedgeTileEntry] = &[
    LedgeTileEntry {
        facing: SpriteFacing::Down,
        standing_tile: 0x2C,
        ledge_tile: 0x37,
        required_input: PadInput::Down,
    },
    LedgeTileEntry {
        facing: SpriteFacing::Down,
        standing_tile: 0x39,
        ledge_tile: 0x36,
        required_input: PadInput::Down,
    },
    LedgeTileEntry {
        facing: SpriteFacing::Down,
        standing_tile: 0x39,
        ledge_tile: 0x37,
        required_input: PadInput::Down,
    },
    LedgeTileEntry {
        facing: SpriteFacing::Left,
        standing_tile: 0x2C,
        ledge_tile: 0x27,
        required_input: PadInput::Left,
    },
    LedgeTileEntry {
        facing: SpriteFacing::Left,
        standing_tile: 0x39,
        ledge_tile: 0x27,
        required_input: PadInput::Left,
    },
    LedgeTileEntry {
        facing: SpriteFacing::Right,
        standing_tile: 0x2C,
        ledge_tile: 0x0D,
        required_input: PadInput::Right,
    },
    LedgeTileEntry {
        facing: SpriteFacing::Right,
        standing_tile: 0x2C,
        ledge_tile: 0x1D,
        required_input: PadInput::Right,
    },
    LedgeTileEntry {
        facing: SpriteFacing::Right,
        standing_tile: 0x39,
        ledge_tile: 0x0D,
        required_input: PadInput::Right,
    },
];

// ── Warp Pad and Hole Data ───────────────────────────────────────────

/// Type of special warp tile the player is standing on.
/// From data/tilesets/warp_pad_hole_tile_ids.asm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarpPadOrHoleType {
    /// Not standing on a warp pad or hole.
    None,
    /// Standing on a warp pad (teleport spin animation).
    WarpPad,
    /// Standing on a hole (fall-through animation).
    Hole,
}

/// A single warp pad/hole data entry.
#[derive(Debug, Clone, Copy)]
pub struct WarpPadHoleEntry {
    pub tileset: TilesetId,
    pub tile_id: u8,
    pub warp_type: WarpPadOrHoleType,
}

/// Warp pad and hole tile data from data/tilesets/warp_pad_hole_tile_ids.asm.
pub const WARP_PAD_HOLE_DATA: &[WarpPadHoleEntry] = &[
    WarpPadHoleEntry {
        tileset: TilesetId::Facility,
        tile_id: 0x20,
        warp_type: WarpPadOrHoleType::WarpPad,
    },
    WarpPadHoleEntry {
        tileset: TilesetId::Facility,
        tile_id: 0x11,
        warp_type: WarpPadOrHoleType::Hole,
    },
    WarpPadHoleEntry {
        tileset: TilesetId::Cavern,
        tile_id: 0x22,
        warp_type: WarpPadOrHoleType::Hole,
    },
    WarpPadHoleEntry {
        tileset: TilesetId::Interior,
        tile_id: 0x55,
        warp_type: WarpPadOrHoleType::WarpPad,
    },
];

/// Check if the player is standing on a warp pad or hole.
/// Matches tileset + tile ID against the warp pad/hole data table.
/// From engine/overworld/player_animations.asm IsPlayerStandingOnWarpPadOrHole.
pub fn check_warp_pad_or_hole(tileset: TilesetId, standing_tile: u8) -> WarpPadOrHoleType {
    for entry in WARP_PAD_HOLE_DATA {
        if entry.tileset == tileset && entry.tile_id == standing_tile {
            return entry.warp_type;
        }
    }
    WarpPadOrHoleType::None
}

// ── Spinner Facing Cycle ─────────────────────────────────────────────

/// Spinner rotation cycle: each direction maps to the next direction.
/// From engine/overworld/spinners.asm SpinnerPlayerFacingDirections.
/// Spinners only exist in Facility and Gym tilesets.
///
/// Cycle: Down → Left → Up → Right → Down ...
/// The array is indexed by SpriteFacing value / 4:
///   0 (Down) → Left, 1 (Up) → Right, 2 (Left) → Up, 3 (Right) → Down
pub const SPINNER_FACING_CYCLE: [SpriteFacing; 4] = [
    SpriteFacing::Left,  // Down (0) -> Left
    SpriteFacing::Right, // Up (1) -> Right
    SpriteFacing::Up,    // Left (2) -> Up
    SpriteFacing::Down,  // Right (3) -> Down
];

/// Tilesets that can have spinner tiles.
pub const SPINNER_TILESETS: &[TilesetId] = &[TilesetId::Facility, TilesetId::Gym];

/// Check if a tileset supports spinner tiles.
pub fn has_spinner_tiles(tileset: TilesetId) -> bool {
    SPINNER_TILESETS.contains(&tileset)
}

/// Get the next facing direction after a spinner rotation.
/// Returns None if the tileset doesn't support spinners.
pub fn spinner_next_facing(
    tileset: TilesetId,
    current_facing: SpriteFacing,
) -> Option<SpriteFacing> {
    if !has_spinner_tiles(tileset) {
        return None;
    }
    let index = (current_facing as u8 / 4) as usize;
    if index < SPINNER_FACING_CYCLE.len() {
        Some(SPINNER_FACING_CYCLE[index])
    } else {
        None
    }
}

// ── Door Tile IDs ────────────────────────────────────────────────────

/// Door tile IDs per tileset, from data/tilesets/door_tile_ids.asm.
/// Used by IsPlayerStandingOnDoorTile in engine/overworld/doors.asm.
/// Returns true if `standing_tile` is a door tile for the given tileset.
pub fn is_door_tile(tileset: TilesetId, standing_tile: u8) -> bool {
    let door_tiles: &[u8] = match tileset {
        TilesetId::Overworld => &[0x1B, 0x58],
        TilesetId::Forest => &[0x3A],
        TilesetId::Mart => &[0x5E],
        TilesetId::House => &[0x54],
        TilesetId::ForestGate | TilesetId::Museum | TilesetId::Gate => &[0x3B],
        TilesetId::Ship => &[0x1E],
        TilesetId::Lobby => &[0x1C, 0x38, 0x1A],
        TilesetId::Mansion => &[0x1A, 0x1C, 0x53],
        TilesetId::Lab => &[0x34],
        TilesetId::Facility => &[0x43, 0x58, 0x1B],
        TilesetId::Plateau => &[0x3B, 0x1B],
        _ => return false,
    };
    door_tiles.contains(&standing_tile)
}

// ── Dark Cave Constants ──────────────────────────────────────────────

/// The palette offset value for Rock Tunnel (dark cave).
/// From home/overworld.asm: `ld a, $06` / `ld [wMapPalOffset], a`
pub const DARK_CAVE_PAL_OFFSET: u8 = 0x06;
