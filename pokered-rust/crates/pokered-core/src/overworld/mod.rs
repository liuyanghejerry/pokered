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
pub mod map_data_loading;
pub mod map_loading;
pub mod map_scripts;
pub mod map_transitions;
pub mod npc_interaction;
pub mod npc_movement;
pub mod player_movement;
pub mod script_engine;
pub mod special_terrain;
pub mod sprites;
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

use pokered_data::blockset_data;
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

// ── Overworld Screen (frame-loop adapter) ─────────────────────────

use crate::game_state::ScreenAction;

#[derive(Debug, Clone, Copy)]
pub struct OverworldInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,
}

impl OverworldInput {
    pub fn none() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            a: false,
            b: false,
            start: false,
            select: false,
        }
    }
}

use crate::game_state::GameScreen;
use player_movement::{InputState as MovementInput, MoveResult};

/// State machine for the new-game bedroom dialogue sequence.
/// Mirrors the original "RED is playing the SNES!" hidden event text.
#[derive(Debug, Clone)]
pub struct DialoguePage {
    pub line1: &'static str,
    pub line2: &'static str,
}

#[derive(Debug, Clone)]
pub struct BedroomDialogue {
    pages: Vec<DialoguePage>,
    current_page: usize,
}

impl BedroomDialogue {
    /// Build the bedroom dialogue with the player's name.
    pub fn new(player_name: &str) -> Self {
        // The original text (data/text/text_2.asm _RedBedroomSNESText):
        //   text "<PLAYER> is"
        //   line "playing the SNES!"
        //   cont "...Okay!"
        //   cont "It's time to go!"
        //
        // We model it as two pages, each requiring an A-press to advance.
        // The player name is baked into page 1 at construction time.
        let line1 = Box::leak(format!("{} is", player_name).into_boxed_str()) as &'static str;
        Self {
            pages: vec![
                DialoguePage {
                    line1,
                    line2: "playing the SNES!",
                },
                DialoguePage {
                    line1: "...Okay!",
                    line2: "It's time to go!",
                },
            ],
            current_page: 0,
        }
    }

    /// Returns the currently visible page, or `None` if the dialogue is done.
    pub fn current(&self) -> Option<&DialoguePage> {
        self.pages.get(self.current_page)
    }

    /// Advance to the next page. Returns `true` if more pages remain, `false` if done.
    pub fn advance(&mut self) -> bool {
        self.current_page += 1;
        self.current_page < self.pages.len()
    }

    pub fn is_done(&self) -> bool {
        self.current_page >= self.pages.len()
    }
}

pub struct OverworldScreen {
    pub state: OverworldState,
    pub map_data: Option<MapData>,
    /// Active dialogue box, if any (blocks movement and Start until dismissed).
    pub pending_dialogue: Option<BedroomDialogue>,
    /// Frames remaining to show map name (0 = hidden).
    pub map_name_timer: u8,
}

const MAP_NAME_DISPLAY_FRAMES: u8 = 120;

impl OverworldScreen {
    pub fn new(start_map: MapId) -> Self {
        let map_data = Some(map_data_loading::load_full_map_data(start_map));
        Self {
            state: OverworldState::new(start_map),
            map_data,
            pending_dialogue: None,
            map_name_timer: 0,
        }
    }

    /// Queue the new-game bedroom SNES dialogue.
    pub fn start_bedroom_dialogue(&mut self, player_name: &str) {
        self.pending_dialogue = Some(BedroomDialogue::new(player_name));
    }

    pub fn update_frame(&mut self, input: OverworldInput) -> ScreenAction {
        // While a dialogue box is active, consume A-button to advance pages;
        // block all movement and Start input.
        if let Some(ref mut dlg) = self.pending_dialogue {
            if input.a {
                if !dlg.advance() {
                    // Last page dismissed — clear dialogue.
                    self.pending_dialogue = None;
                }
            }
            return ScreenAction::Continue;
        }

        if input.start {
            return ScreenAction::Transition(GameScreen::StartMenu);
        }

        let movement_input = MovementInput {
            up: input.up,
            down: input.down,
            left: input.left,
            right: input.right,
            a_button: input.a,
            b_button: input.b,
            start: input.start,
            select: input.select,
        };

        let get_tile_id_at_position =
            |blocks: &[u8], width: u8, tileset: TilesetId, x: u16, y: u16| -> u8 {
                let block_x = (x / 4) as usize;
                let block_y = (y / 4) as usize;
                let sub_x = (x % 4) as usize;
                let sub_y = (y % 4) as usize;

                if block_x < width as usize {
                    let block_idx = block_y * (width as usize) + block_x;
                    if block_idx < blocks.len() {
                        let block_id = blocks[block_idx];
                        return blockset_data::block_tiles(tileset, block_id)
                            .map(|t| t[sub_y * 4 + sub_x])
                            .unwrap_or(0);
                    }
                }
                0
            };

        if let Some(map) = &self.map_data {
            let standing_tile = get_tile_id_at_position(
                &map.blocks,
                map.width,
                map.tileset,
                self.state.player.x,
                self.state.player.y,
            );

            let target_tile = if let Some(dir) = movement_input.direction_pressed() {
                let (dx, dy) = player_movement::direction_delta(dir);
                let target_x = ((self.state.player.x as i32) + dx as i32).max(0) as u16;
                let target_y = ((self.state.player.y as i32) + dy as i32).max(0) as u16;

                get_tile_id_at_position(&map.blocks, map.width, map.tileset, target_x, target_y)
            } else {
                standing_tile
            };

            let npc_positions: Vec<collision::SpritePosition> = Vec::new();

            let result = player_movement::process_frame(
                &mut self.state,
                &movement_input,
                map,
                standing_tile,
                target_tile,
                &npc_positions,
            );

            match result {
                MoveResult::Warped { warp_index } => {
                    let warp_info = self.map_data.as_ref().and_then(|m| {
                        if warp_index < m.warps.len() {
                            Some((
                                m.warps[warp_index].target_map,
                                m.warps[warp_index].target_warp_id,
                            ))
                        } else {
                            None
                        }
                    });

                    if let Some((target_map, target_warp_id)) = warp_info {
                        let dest_warp_coords =
                            map_transitions::resolve_warp_destination(target_map, target_warp_id)
                                .unwrap_or((0, 0));

                        self.state.current_map = target_map;
                        self.state.player.x = dest_warp_coords.0 as u16;
                        self.state.player.y = dest_warp_coords.1 as u16;

                        self.map_data = Some(map_data_loading::load_full_map_data(target_map));

                        if !target_map.is_indoor() {
                            self.map_name_timer = MAP_NAME_DISPLAY_FRAMES;
                        }
                    }
                }
                MoveResult::ReachedMapEdge => {
                    if let Some(dir) = movement_input.direction_pressed() {
                        if let Some(transition) = map_transitions::calculate_connection_transition(
                            self.state.current_map,
                            self.state.player.x,
                            self.state.player.y,
                            dir,
                        ) {
                            let new_map = transition.new_map;
                            self.state.current_map = new_map;
                            self.map_data = Some(map_data_loading::load_full_map_data(new_map));
                            self.state.player.x = transition.new_x;
                            self.state.player.y = transition.new_y;

                            if !new_map.is_indoor() {
                                self.map_name_timer = MAP_NAME_DISPLAY_FRAMES;
                            }
                        }
                    }
                }
                _ => {}
            }

            if self.map_name_timer > 0 {
                self.map_name_timer -= 1;
            }
        } else {
            if let Some(dir) = movement_input.direction_pressed() {
                self.state.player.facing = dir;
                let (dx, dy) = player_movement::direction_delta(dir);
                self.state.player.x = (self.state.player.x as i32 + dx as i32).max(0) as u16;
                self.state.player.y = (self.state.player.y as i32 + dy as i32).max(0) as u16;
            }
        }

        ScreenAction::Continue
    }
}
