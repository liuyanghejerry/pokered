//! Overworld system — map loading, player movement, collision, and map connections.
//!
//! Implements M4.1 (地图加载和瓦片渲染) and M4.2 (玩家移动和碰撞检测)
//! of the Rust rewrite plan. This module provides the core data types,
//! loading functions, collision detection, and player movement for
//! the game's overworld map system.

pub mod collision;
pub mod doors_elevators;
pub mod event_flags;
pub mod hm_effects;
pub mod map_loading;
pub mod map_scripts;
pub mod map_transitions;
pub mod npc_interaction;
pub mod npc_movement;
pub mod player_movement;
pub mod script_engine;
pub mod special_terrain;
pub mod trainer_engine;
pub mod wild_encounters;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_collision;
#[cfg(test)]
mod tests_connections;
#[cfg(test)]
mod tests_doors_elevators;
#[cfg(test)]
mod tests_hm_effects;
#[cfg(test)]
mod tests_movement;
#[cfg(test)]
mod tests_npc;
#[cfg(test)]
mod tests_scripts;
#[cfg(test)]
mod tests_special_terrain;
#[cfg(test)]
mod tests_wild_encounters;

use pokered_data::maps::MapId;
use pokered_data::music::MusicId;
use pokered_data::tilesets::TilesetId;

// ── Direction ──────────────────────────────────────────────────────

/// Cardinal direction for movement and connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}

/// Transport mode for player movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportMode {
    Walking,
    Biking,
    Surfing,
}

/// Player movement state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovementState {
    Idle,
    Walking,
    Jumping,
}

// ── Map Connection ─────────────────────────────────────────────────

/// A single map connection (e.g., north exit leads to Route 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapConnection {
    pub direction: Direction,
    pub target_map: MapId,
    /// Offset in blocks for alignment when crossing the boundary.
    pub offset: i8,
}

/// All connections for a map (up to one per cardinal direction).
#[derive(Debug, Clone, Default)]
pub struct MapConnections {
    pub north: Option<MapConnection>,
    pub south: Option<MapConnection>,
    pub west: Option<MapConnection>,
    pub east: Option<MapConnection>,
}

impl MapConnections {
    /// Number of active connections.
    pub fn count(&self) -> usize {
        self.north.is_some() as usize
            + self.south.is_some() as usize
            + self.west.is_some() as usize
            + self.east.is_some() as usize
    }

    /// Get connection for a direction, if any.
    pub fn get(&self, dir: Direction) -> Option<&MapConnection> {
        match dir {
            Direction::Up => self.north.as_ref(),
            Direction::Down => self.south.as_ref(),
            Direction::Left => self.west.as_ref(),
            Direction::Right => self.east.as_ref(),
        }
    }
}

// ── Warp Point ─────────────────────────────────────────────────────

/// A warp point within a map (door, staircase, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpPoint {
    /// Position in the map (block coordinates).
    pub x: u8,
    pub y: u8,
    /// Target map to warp to.
    pub target_map: MapId,
    /// Index of the target warp in the destination map.
    pub target_warp_id: u8,
}

// ── Sign ───────────────────────────────────────────────────────────

/// A sign in the map that displays text when interacted with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sign {
    pub x: u8,
    pub y: u8,
    /// Index into the map's text table.
    pub text_id: u8,
}

// ── NPC Definition ─────────────────────────────────────────────────

/// NPC movement pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpcMovementType {
    /// NPC stays in place and faces a fixed direction.
    Stationary,
    /// NPC walks randomly within their range.
    Wander,
    /// NPC walks a fixed path.
    FixedPath,
    /// NPC turns to face the player when spoken to.
    FacePlayer,
}

/// Definition of an NPC placed on the map (static data from map objects).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NpcDefinition {
    /// Sprite ID (index into sprite table).
    pub sprite_id: u8,
    /// Starting position.
    pub x: u8,
    pub y: u8,
    /// Movement type.
    pub movement: NpcMovementType,
    /// Facing direction.
    pub facing: Direction,
    /// Range of movement (0 = stationary).
    pub range: u8,
    /// Text ID triggered on interaction.
    pub text_id: u8,
    /// Whether this NPC is a trainer.
    pub is_trainer: bool,
    /// Trainer class (if is_trainer).
    pub trainer_class: u8,
    /// Trainer set index (if is_trainer).
    pub trainer_set: u8,
    /// Item given on interaction (0 = none).
    pub item_id: u8,
}

// ── Map Data ───────────────────────────────────────────────────────

/// Complete runtime data for a loaded map.
#[derive(Debug, Clone)]
pub struct MapData {
    pub id: MapId,
    pub width: u8,
    pub height: u8,
    pub tileset: TilesetId,
    pub music: MusicId,
    /// Block data — the actual tile layout. Each byte is a block index
    /// into the tileset's block definitions. Size = width * height.
    pub blocks: Vec<u8>,
    pub warps: Vec<WarpPoint>,
    pub npcs: Vec<NpcDefinition>,
    pub signs: Vec<Sign>,
    pub connections: MapConnections,
}

// ── Player State ───────────────────────────────────────────────────

/// Runtime player state in the overworld.
#[derive(Debug, Clone)]
pub struct PlayerState {
    pub x: u16,
    pub y: u16,
    pub facing: Direction,
    pub movement_state: MovementState,
    pub transport: TransportMode,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            facing: Direction::Down,
            movement_state: MovementState::Idle,
            transport: TransportMode::Walking,
        }
    }
}

// ── Overworld State ────────────────────────────────────────────────

/// Top-level overworld state, holding the current map and player.
#[derive(Debug)]
pub struct OverworldState {
    pub current_map: MapId,
    pub player: PlayerState,
    /// Walk animation counter (0-15).
    pub walk_counter: u8,
    /// Steps until next wild encounter check resets.
    pub encounter_cooldown: u8,
    /// Remaining repel steps (0 = inactive).
    pub repel_steps: u16,
}

impl OverworldState {
    /// Create a new overworld state starting at the given map.
    pub fn new(start_map: MapId) -> Self {
        Self {
            current_map: start_map,
            player: PlayerState::default(),
            walk_counter: 0,
            encounter_cooldown: 0,
            repel_steps: 0,
        }
    }
}
