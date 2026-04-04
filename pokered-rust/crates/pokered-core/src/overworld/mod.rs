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
pub mod map_transitions;
pub mod npc_interaction;
pub mod npc_movement;
pub mod player_movement;
pub mod script_bridge;
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
use pokered_data::tileset_data;
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
    /// Whether the player is currently standing on a warp coordinate.
    /// Equivalent to BIT_STANDING_ON_WARP in the original game's wMovementFlags.
    /// Set when the player steps onto a warp position but the warp doesn't
    /// fire immediately (e.g., indoor door mats that require walking further).
    /// Checked on the next collision to trigger the warp.
    pub standing_on_warp: bool,
    /// Whether the player just warped onto a door tile and needs to auto-step off.
    /// Equivalent to BIT_STANDING_ON_DOOR in the original game's wMovementFlags.
    /// Set by commit_pending_warp when the destination tile is a door/cave tile.
    /// Consumed by update_frame to inject a simulated DOWN press (PlayerStepOutFromDoor).
    pub standing_on_door: bool,
    /// Whether the player is currently performing the auto-step out of a door.
    /// Equivalent to BIT_EXITING_DOOR in the original game. While true, real
    /// player input is ignored and a simulated DOWN movement is in progress.
    pub exiting_door: bool,
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
            standing_on_warp: false,
            standing_on_door: false,
            exiting_door: false,
        }
    }
}

// ── Overworld SFX Events ──────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverworldSfxEvent {
    None,
    /// SFX_GO_INSIDE — standing on door tile ($0b). home/overworld.asm:PlayMapChangeSound
    GoInside,
    /// SFX_GO_OUTSIDE — non-door warp (stairs, cave). home/overworld.asm:PlayMapChangeSound
    GoOutside,
    /// SFX_COLLISION — bumped into wall. home/overworld.asm:1246,1923
    Collision,
    /// SFX_LEDGE — jumped a ledge. engine/overworld/ledges.asm:53
    Ledge,
}

// ── Overworld Screen (frame-loop adapter) ─────────────────────────

use crate::game_state::ScreenAction;
use pokered_script::{CommandResult, MapScriptConfig, ScriptEngine, ScriptLoader};

// ── Wild Encounter ─────────────────────────────────────────────────

/// Wild encounter data ready to be passed to BattleScreen.
#[derive(Debug, Clone)]
pub struct PendingWildEncounter {
    pub species: pokered_data::species::Species,
    pub level: u8,
}

// ── Warp Fade Transition ──────────────────────────────────────────
//
// Mirrors the original game's map transition:
//   1. PlayMapChangeSound → GBFadeOutToBlack (4 palette steps × 8 frames = 32 frames)
//   2. LoadMapData (while screen is black)
//   3. GBFadeInFromWhite (3 palette steps × 8 frames = 24 frames)
//
// During fade, player input is frozen (the original sets wJoyIgnore).

/// Number of frames per fade palette step (matches FADE_DELAY_FRAMES in transition.rs).
const WARP_FADE_DELAY: u8 = 8;
/// Fade-out: 4 palette steps (FadePal4→FadePal1).
const WARP_FADE_OUT_STEPS: u8 = 4;
/// Fade-in: 3 palette steps (FadePal7→FadePal5 for InFromWhite, or 4 for InFromBlack).
const WARP_FADE_IN_STEPS: u8 = 3;

/// Total frames for fade-out phase.
const WARP_FADE_OUT_FRAMES: u8 = WARP_FADE_OUT_STEPS * WARP_FADE_DELAY;
/// Total frames for fade-in phase.
const WARP_FADE_IN_FRAMES: u8 = WARP_FADE_IN_STEPS * WARP_FADE_DELAY;

/// Warp transition visual state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarpFadeState {
    /// No warp transition in progress.
    Idle,
    /// Fading screen to black before loading new map.
    FadingOut { frames_remaining: u8 },
    /// Screen is fully black; map data is being swapped this frame.
    BlackScreen,
    /// Fading screen back in after loading new map.
    FadingIn { frames_remaining: u8 },
}

/// Pending warp destination, stored when a warp is detected during fade-out.
#[derive(Debug, Clone, Copy)]
pub struct PendingWarp {
    pub dest_map: MapId,
    pub dest_x: u8,
    pub dest_y: u8,
    /// Whether we should update last_map (only for outside→inside transitions).
    pub save_last_map: bool,
}

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
    pub fn new(player_name: &str) -> Self {
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

    pub fn from_text_pages(
        text_pages: &[pokered_data::map_json::TextPageJson],
        player_name: &str,
        rival_name: &str,
    ) -> Self {
        let pages = text_pages
            .iter()
            .map(|tp| {
                let l1 = resolve_placeholders(&tp.line1, player_name, rival_name);
                let l2 = resolve_placeholders(&tp.line2, player_name, rival_name);
                DialoguePage {
                    line1: Box::leak(l1.into_boxed_str()),
                    line2: Box::leak(l2.into_boxed_str()),
                }
            })
            .collect();
        Self {
            pages,
            current_page: 0,
        }
    }

    pub fn from_pages(pages: Vec<DialoguePage>) -> Self {
        Self {
            pages,
            current_page: 0,
        }
    }

    pub fn current(&self) -> Option<&DialoguePage> {
        self.pages.get(self.current_page)
    }

    pub fn advance(&mut self) -> bool {
        self.current_page += 1;
        self.current_page < self.pages.len()
    }

    pub fn has_more_pages(&self) -> bool {
        self.current_page + 1 < self.pages.len()
    }

    pub fn is_done(&self) -> bool {
        self.current_page >= self.pages.len()
    }
}

fn resolve_placeholders(text: &str, player_name: &str, rival_name: &str) -> String {
    text.replace("<PLAYER>", player_name)
        .replace("<RIVAL>", rival_name)
}

fn build_npc_runtime_states(npcs: &[NpcDefinition]) -> Vec<npc_movement::NpcRuntimeState> {
    npcs.iter()
        .enumerate()
        .map(|(i, npc)| npc_movement::NpcRuntimeState {
            npc_index: i as u8,
            sprite_id: npc.sprite_id,
            x: npc.x as u16,
            y: npc.y as u16,
            home_x: npc.x as u16,
            home_y: npc.y as u16,
            facing: npc.facing,
            movement_type: npc.movement,
            range: npc.range,
            walk_counter: 0,
            delay_counter: 0,
            text_id: npc.text_id,
            is_trainer: npc.is_trainer,
            trainer_class: npc.trainer_class,
            trainer_set: npc.trainer_set,
            item_id: npc.item_id,
            defeated: false,
            visible: true,
        })
        .collect()
}

fn get_npc_text_from_json(
    map_id: MapId,
    text_id: u8,
) -> Option<Vec<pokered_data::map_json::TextPageJson>> {
    let map_json = pokered_data::map_data_loader::get_map_json(map_id)?;
    let key = text_id.to_string();
    map_json.text.npc.get(&key).cloned()
}

fn get_sign_text_from_json(
    map_id: MapId,
    text_id: u8,
) -> Option<Vec<pokered_data::map_json::TextPageJson>> {
    let map_json = pokered_data::map_data_loader::get_map_json(map_id)?;
    let key = text_id.to_string();
    map_json.text.sign.get(&key).cloned()
}

pub struct OverworldScreen {
    pub state: OverworldState,
    pub map_data: Option<MapData>,
    pub npc_states: Vec<npc_movement::NpcRuntimeState>,
    pub pending_dialogue: Option<BedroomDialogue>,
    pub map_name_timer: u8,
    pub last_map: Option<MapId>,
    pub warp_fade_state: WarpFadeState,
    pub pending_warp: Option<PendingWarp>,
    pub pending_wild_encounter: Option<PendingWildEncounter>,
    pub sfx_event: OverworldSfxEvent,
    pub bump_anim_counter: u8,
    pub player_name: String,
    pub rival_name: String,
    pub frame_counter: u32,
    prev_a_pressed: bool,
    prev_movement_state: MovementState,
    script_engine: ScriptEngine,
    script_loader: ScriptLoader,
    map_script_config: MapScriptConfig,
    active_script_effect: Option<script_bridge::ScriptEffect>,
    joy_ignore_mask: u8,
    map_script_index: u8,
    scripts_dir: Option<std::path::PathBuf>,
}

const MAP_NAME_DISPLAY_FRAMES: u8 = 120;

impl OverworldScreen {
    pub fn new(start_map: MapId, scripts_dir: Option<std::path::PathBuf>) -> Self {
        let map_data = Some(map_data_loading::load_full_map_data(start_map));
        let npc_states = map_data
            .as_ref()
            .map(|md| build_npc_runtime_states(&md.npcs))
            .unwrap_or_default();

        let mut script_loader = ScriptLoader::new();
        match script_loader.load_auto(scripts_dir.as_deref()) {
            Ok(count) => log::info!("ScriptLoader: loaded {} files via load_auto", count),
            Err(e) => log::error!("ScriptLoader: load_auto failed: {}", e),
        }

        let mut script_engine = ScriptEngine::new();
        let map_key = script_bridge::map_id_to_script_key(start_map);
        if let Some(source) = script_loader.get_script(&map_key) {
            let _ = script_engine.load_script(source);
        }
        let map_script_config = script_loader
            .get_config(&map_key)
            .cloned()
            .unwrap_or_default();

        Self {
            state: OverworldState::new(start_map),
            map_data,
            npc_states,
            pending_dialogue: None,
            map_name_timer: 0,
            last_map: Some(MapId::PalletTown),
            warp_fade_state: WarpFadeState::Idle,
            pending_warp: None,
            pending_wild_encounter: None,
            sfx_event: OverworldSfxEvent::None,
            bump_anim_counter: 0,
            player_name: "RED".to_string(),
            rival_name: "BLUE".to_string(),
            frame_counter: 0,
            prev_a_pressed: false,
            prev_movement_state: MovementState::Idle,
            script_engine,
            script_loader,
            map_script_config,
            active_script_effect: None,
            joy_ignore_mask: 0,
            map_script_index: 0,
            scripts_dir,
        }
    }

    /// Queue the new-game bedroom SNES dialogue.
    pub fn start_bedroom_dialogue(&mut self, player_name: &str) {
        self.pending_dialogue = Some(BedroomDialogue::new(player_name));
    }

    /// Returns the current warp fade darkness level (0.0 = fully visible, 1.0 = fully black).
    /// Used by the renderer to draw a black overlay during map transitions.
    pub fn warp_fade_progress(&self) -> f32 {
        match self.warp_fade_state {
            WarpFadeState::Idle => 0.0,
            WarpFadeState::FadingOut { frames_remaining } => {
                1.0 - (frames_remaining as f32 / WARP_FADE_OUT_FRAMES as f32)
            }
            WarpFadeState::BlackScreen => 1.0,
            WarpFadeState::FadingIn { frames_remaining } => {
                frames_remaining as f32 / WARP_FADE_IN_FRAMES as f32
            }
        }
    }

    fn commit_pending_warp(&mut self) {
        if let Some(warp) = self.pending_warp.take() {
            if warp.save_last_map {
                self.last_map = Some(self.state.current_map);
            }
            self.state.current_map = warp.dest_map;
            self.state.player.x = warp.dest_x as u16;
            self.state.player.y = warp.dest_y as u16;
            self.map_data = Some(map_data_loading::load_full_map_data(warp.dest_map));
            self.npc_states = self
                .map_data
                .as_ref()
                .map(|md| build_npc_runtime_states(&md.npcs))
                .unwrap_or_default();
            self.load_map_script(warp.dest_map);
            if !warp.dest_map.is_indoor() {
                self.map_name_timer = MAP_NAME_DISPLAY_FRAMES;
            }

            // PlayerStepOutFromDoor: if the player landed on a door tile,
            // flag it so update_frame will auto-walk one step down.
            if let Some(map) = &self.map_data {
                let tile = player_movement::get_tile_at_position(
                    map,
                    self.state.player.x,
                    self.state.player.y,
                );
                if doors_elevators::is_standing_on_door(map.tileset, tile) {
                    self.state.standing_on_door = true;
                    self.state.player.facing = Direction::Down;
                }
            }
        }
    }

    pub fn update_frame(&mut self, input: OverworldInput) -> ScreenAction {
        self.frame_counter = self.frame_counter.wrapping_add(1);
        self.sfx_event = OverworldSfxEvent::None;

        // Edge detection for the A button must be computed BEFORE any early-return
        // paths (warp fade, script effects, door auto-step, etc.) so that
        // `prev_a_pressed` stays in sync with the physical key state every frame.
        // Without this, holding A across a dialogue-creation boundary causes the
        // first page to be skipped because the held state is mistaken for a new press.
        let a_just_pressed = input.a && !self.prev_a_pressed;
        self.prev_a_pressed = input.a;

        match self.warp_fade_state {
            WarpFadeState::FadingOut { frames_remaining } => {
                if frames_remaining <= 1 {
                    self.warp_fade_state = WarpFadeState::BlackScreen;
                } else {
                    self.warp_fade_state = WarpFadeState::FadingOut {
                        frames_remaining: frames_remaining - 1,
                    };
                }
                return ScreenAction::Continue;
            }
            WarpFadeState::BlackScreen => {
                self.commit_pending_warp();
                self.warp_fade_state = WarpFadeState::FadingIn {
                    frames_remaining: WARP_FADE_IN_FRAMES,
                };
                return ScreenAction::Continue;
            }
            WarpFadeState::FadingIn { frames_remaining } => {
                if frames_remaining <= 1 {
                    self.warp_fade_state = WarpFadeState::Idle;
                } else {
                    self.warp_fade_state = WarpFadeState::FadingIn {
                        frames_remaining: frames_remaining - 1,
                    };
                }
                return ScreenAction::Continue;
            }
            WarpFadeState::Idle => {}
        }

        // ── Script engine tick ────────────────────────────────────────
        if let Some(ref mut effect) = self.active_script_effect {
            let done = Self::tick_active_effect(effect, a_just_pressed, &mut self.pending_dialogue);
            if done {
                let result = Self::finish_effect(effect);
                let effect_done = self.active_script_effect.take();
                self.apply_finished_effect(effect_done);
                if let Ok(Some(next_cmd)) = self.script_engine.signal_done(result) {
                    self.active_script_effect = Some(script_bridge::dispatch_command(&next_cmd));
                }
            }
            return ScreenAction::Continue;
        }
        if let Some(cmd) = self.script_engine.tick() {
            self.active_script_effect = Some(script_bridge::dispatch_command(&cmd));
            return ScreenAction::Continue;
        }

        // Door exit auto-step (PlayerStepOutFromDoor / BIT_EXITING_DOOR).
        // When exiting_door is active, advance the walk animation ignoring real input.
        if self.state.exiting_door {
            if self.state.player.movement_state != MovementState::Idle {
                let step_done = player_movement::advance_step(&mut self.state);
                if step_done {
                    self.state.exiting_door = false;
                }
            } else {
                self.state.exiting_door = false;
            }
            return ScreenAction::Continue;
        }

        // Initiate the auto-step when standing_on_door is flagged after a warp.
        if self.state.standing_on_door {
            self.state.standing_on_door = false;
            self.state.player.facing = Direction::Down;
            self.state.player.movement_state = MovementState::Walking;
            self.state.walk_counter = player_movement::WALK_COUNTER_INIT;
            self.state.exiting_door = true;
            return ScreenAction::Continue;
        }

        // While a dialogue box is active, consume A-button to advance pages;
        // block all movement and Start input.
        if let Some(ref mut dlg) = self.pending_dialogue {
            if a_just_pressed {
                if !dlg.advance() {
                    self.pending_dialogue = None;
                }
            }
            return ScreenAction::Continue;
        }

        // A-button: check signs first, then NPCs (matches original game priority).
        if a_just_pressed && self.state.player.movement_state == MovementState::Idle {
            if let Some(map) = &self.map_data {
                let sign_tuples: Vec<(u8, u8, u8)> =
                    map.signs.iter().map(|s| (s.x, s.y, s.text_id)).collect();

                if let Some(sign_text_id) = npc_interaction::check_sign_interaction(
                    &sign_tuples,
                    self.state.player.x,
                    self.state.player.y,
                    self.state.player.facing,
                ) {
                    if self.try_call_script_sign_talk(sign_text_id) {
                        return ScreenAction::Continue;
                    }
                    if let Some(text_pages) =
                        get_sign_text_from_json(self.state.current_map, sign_text_id)
                    {
                        if !text_pages.is_empty() {
                            self.pending_dialogue = Some(BedroomDialogue::from_text_pages(
                                &text_pages,
                                &self.player_name,
                                &self.rival_name,
                            ));
                            return ScreenAction::Continue;
                        }
                    }
                }

                let interaction = npc_interaction::try_interact(
                    &self.npc_states,
                    self.state.player.x,
                    self.state.player.y,
                    self.state.player.facing,
                );

                match interaction {
                    npc_interaction::InteractionResult::Talk { npc_index, text_id }
                    | npc_interaction::InteractionResult::AlreadyDefeated { npc_index, text_id } => {
                        let face_dir =
                            player_movement::opposite_direction(self.state.player.facing);
                        if let Some(npc) = self
                            .npc_states
                            .iter_mut()
                            .find(|n| n.npc_index == npc_index)
                        {
                            npc.facing = face_dir;
                        }

                        if self.try_call_script_npc_talk(text_id) {
                            return ScreenAction::Continue;
                        }

                        if let Some(text_pages) =
                            get_npc_text_from_json(self.state.current_map, text_id)
                        {
                            if !text_pages.is_empty() {
                                self.pending_dialogue = Some(BedroomDialogue::from_text_pages(
                                    &text_pages,
                                    &self.player_name,
                                    &self.rival_name,
                                ));
                                return ScreenAction::Continue;
                            }
                        }
                    }
                    _ => {}
                }
            }
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
                // Player coordinates are in step units (16px each).
                // Each map block is 32×32px = 2×2 steps.
                let block_x = (x / 2) as usize;
                let block_y = (y / 2) as usize;
                let sub_x = (x % 2) as usize;
                let sub_y = (y % 2) as usize;

                if block_x < width as usize {
                    let block_idx = block_y * (width as usize) + block_x;
                    if block_idx < blocks.len() {
                        let block_id = blocks[block_idx];
                        // Bottom-left tile of the step's 2×2 quadrant (matches original wTileMap lookup)
                        return blockset_data::block_tiles(tileset, block_id)
                            .map(|t| t[(sub_y * 2 + 1) * 4 + sub_x * 2])
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

            let npc_positions: Vec<collision::SpritePosition> = self
                .npc_states
                .iter()
                .filter(|npc| npc.visible)
                .map(|npc| collision::SpritePosition { x: npc.x, y: npc.y })
                .collect();

            let movement_before = self.state.player.movement_state;

            let result = player_movement::process_frame(
                &mut self.state,
                &movement_input,
                map,
                standing_tile,
                target_tile,
                &npc_positions,
            );

            let step_completed = movement_before == MovementState::Walking
                && self.state.player.movement_state == MovementState::Idle;

            let encounter_check_data = if step_completed && self.pending_wild_encounter.is_none() {
                let new_standing_tile = player_movement::get_tile_at_position(
                    map,
                    self.state.player.x,
                    self.state.player.y,
                );
                Some((map.id, map.tileset, new_standing_tile))
            } else {
                None
            };

            self.prev_movement_state = self.state.player.movement_state;

            match result {
                MoveResult::Warped { warp_index: _ } => {
                    if let Some((dest_map, warp_x, warp_y)) = map_transitions::execute_warp(
                        self.state.current_map,
                        self.state.player.x as u8,
                        self.state.player.y as u8,
                        self.last_map,
                    ) {
                        // PlayMapChangeSound: door tile → GoInside, otherwise → GoOutside
                        if doors_elevators::is_standing_on_door(map.tileset, standing_tile) {
                            self.sfx_event = OverworldSfxEvent::GoInside;
                        } else {
                            self.sfx_event = OverworldSfxEvent::GoOutside;
                        }
                        let save_last_map = tileset_data::is_outside_tileset(map.tileset);
                        self.pending_warp = Some(PendingWarp {
                            dest_map,
                            dest_x: warp_x,
                            dest_y: warp_y,
                            save_last_map,
                        });
                        self.warp_fade_state = WarpFadeState::FadingOut {
                            frames_remaining: WARP_FADE_OUT_FRAMES,
                        };
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
                            if tileset_data::is_outside_tileset(map.tileset) {
                                self.last_map = Some(self.state.current_map);
                            }
                            self.state.current_map = new_map;
                            self.map_data = Some(map_data_loading::load_full_map_data(new_map));
                            self.npc_states = self
                                .map_data
                                .as_ref()
                                .map(|md| build_npc_runtime_states(&md.npcs))
                                .unwrap_or_default();
                            self.state.player.x = transition.new_x;
                            self.state.player.y = transition.new_y;
                            self.load_map_script(new_map);

                            if !new_map.is_indoor() {
                                self.map_name_timer = MAP_NAME_DISPLAY_FRAMES;
                            }
                        }
                    }
                }
                MoveResult::Blocked(_) => {
                    self.sfx_event = OverworldSfxEvent::Collision;
                    self.bump_anim_counter = self.bump_anim_counter.wrapping_add(1);
                }
                MoveResult::LedgeJump => {
                    self.sfx_event = OverworldSfxEvent::Ledge;
                    self.bump_anim_counter = 0;
                }
                _ => {
                    self.bump_anim_counter = 0;
                }
            }

            if self.map_name_timer > 0 {
                self.map_name_timer -= 1;
            }

            if let Some((map_id, tileset, standing_tile)) = encounter_check_data {
                self.check_wild_encounter_on_step(
                    map_id,
                    tileset,
                    standing_tile,
                    self.state.standing_on_warp,
                    self.active_script_effect.is_some(),
                );
            }
        } else {
            if let Some(dir) = movement_input.direction_pressed() {
                self.state.player.facing = dir;
                let (dx, dy) = player_movement::direction_delta(dir);
                self.state.player.x = (self.state.player.x as i32 + dx as i32).max(0) as u16;
                self.state.player.y = (self.state.player.y as i32 + dy as i32).max(0) as u16;
            }
        }

        // Advance NPC movement every frame (DoMovementForAllSprites).
        // In the original game, NPC movement is frozen while a text box is displayed
        // (UpdateSpriteFacingOffsetAndDelayMovement sets delay to $7F).
        // We skip the entire NPC update when dialogue is active.
        if self.pending_dialogue.is_none() {
            if let Some(ref map) = self.map_data {
                let rng_value = (self
                    .frame_counter
                    .wrapping_mul(1103515245)
                    .wrapping_add(12345)
                    >> 16) as u8;
                let player_dest = if self.state.player.movement_state != MovementState::Idle {
                    let (dx, dy) = player_movement::direction_delta(self.state.player.facing);
                    Some((
                        (self.state.player.x as i32 + dx as i32).max(0) as u16,
                        (self.state.player.y as i32 + dy as i32).max(0) as u16,
                    ))
                } else {
                    None
                };
                npc_movement::update_npc_movement(
                    &mut self.npc_states,
                    self.state.player.x,
                    self.state.player.y,
                    player_dest,
                    map.width,
                    map.height,
                    rng_value,
                    &map.blocks,
                    map.tileset,
                );
            }
        }

        ScreenAction::Continue
    }

    fn tick_active_effect(
        effect: &mut script_bridge::ScriptEffect,
        a_just_pressed: bool,
        pending_dialogue: &mut Option<BedroomDialogue>,
    ) -> bool {
        match effect {
            script_bridge::ScriptEffect::ShowDialogue { text } => {
                if pending_dialogue.is_none() {
                    *pending_dialogue = Some(script_bridge::text_to_dialogue(text));
                    false
                } else if pending_dialogue.as_ref().map_or(true, |d| d.is_done()) {
                    *pending_dialogue = None;
                    true
                } else {
                    if a_just_pressed {
                        if let Some(ref mut dlg) = pending_dialogue {
                            if !dlg.advance() {
                                *pending_dialogue = None;
                                return true;
                            }
                        }
                    }
                    false
                }
            }
            script_bridge::ScriptEffect::Delay {
                frames: _,
                ref mut frames_remaining,
            } => {
                if *frames_remaining == 0 {
                    return true;
                }
                *frames_remaining -= 1;
                *frames_remaining == 0
            }
            script_bridge::ScriptEffect::Immediate { .. } => true,
            script_bridge::ScriptEffect::SetJoyIgnore { .. } => true,
            script_bridge::ScriptEffect::ClearJoyIgnore => true,
            script_bridge::ScriptEffect::SetMapScript { .. } => true,
            _ => true,
        }
    }

    fn finish_effect(effect: &script_bridge::ScriptEffect) -> CommandResult {
        match effect {
            script_bridge::ScriptEffect::ShowChoice { .. } => CommandResult::Number(0.0),
            script_bridge::ScriptEffect::Immediate { result } => result.clone(),
            _ => CommandResult::Void,
        }
    }

    fn apply_finished_effect(&mut self, effect: Option<script_bridge::ScriptEffect>) {
        if let Some(eff) = effect {
            match eff {
                script_bridge::ScriptEffect::SetJoyIgnore { mask } => {
                    self.joy_ignore_mask = mask;
                }
                script_bridge::ScriptEffect::ClearJoyIgnore => {
                    self.joy_ignore_mask = 0;
                }
                script_bridge::ScriptEffect::SetMapScript { state_name } => {
                    if let Some(idx) = self.map_script_config.resolve_map_script_index(&state_name)
                    {
                        self.map_script_index = idx as u8;
                    } else {
                        log::warn!("SetMapScript: unknown state '{}'", state_name);
                    }
                }
                script_bridge::ScriptEffect::FaceNpc { npc_id, direction } => {
                    if let Some(idx) =
                        script_bridge::find_npc_index_by_id(&self.npc_states, &npc_id)
                    {
                        self.npc_states[idx].facing = direction;
                    }
                }
                script_bridge::ScriptEffect::FacePlayer { direction } => {
                    self.state.player.facing = direction;
                }
                script_bridge::ScriptEffect::ShowObject { object_index } => {
                    if let Some(npc) = self
                        .npc_states
                        .iter_mut()
                        .find(|n| n.npc_index == object_index)
                    {
                        npc.visible = true;
                    }
                }
                script_bridge::ScriptEffect::HideObject { object_index } => {
                    if let Some(npc) = self
                        .npc_states
                        .iter_mut()
                        .find(|n| n.npc_index == object_index)
                    {
                        npc.visible = false;
                    }
                }
                _ => {}
            }
        }
    }

    fn load_map_script(&mut self, map_id: MapId) {
        let map_key = script_bridge::map_id_to_script_key(map_id);
        self.script_engine = ScriptEngine::new();
        if let Some(source) = self.script_loader.get_script(&map_key) {
            let _ = self.script_engine.load_script(source);
        }
        self.map_script_config = self
            .script_loader
            .get_config(&map_key)
            .cloned()
            .unwrap_or_default();
        self.active_script_effect = None;
        self.map_script_index = 0;
    }

    fn try_call_script_npc_talk(&mut self, text_id: u8) -> bool {
        if let Some(fn_name) = self.map_script_config.npc_talk_fn(text_id) {
            if self.script_engine.has_function(fn_name) {
                if let Ok(Some(cmd)) = self.script_engine.call_function_no_args(fn_name) {
                    self.active_script_effect = Some(script_bridge::dispatch_command(&cmd));
                    return true;
                }
            }
        }
        false
    }

    fn try_call_script_sign_talk(&mut self, text_id: u8) -> bool {
        if let Some(fn_name) = self.map_script_config.sign_talk_fn(text_id) {
            if self.script_engine.has_function(fn_name) {
                if let Ok(Some(cmd)) = self.script_engine.call_function_no_args(fn_name) {
                    self.active_script_effect = Some(script_bridge::dispatch_command(&cmd));
                    return true;
                }
            }
        }
        false
    }

    fn check_wild_encounter_on_step(
        &mut self,
        map_id: pokered_data::maps::MapId,
        tileset: pokered_data::tilesets::TilesetId,
        standing_tile: u8,
        standing_on_warp: bool,
        has_script_effect: bool,
    ) {
        use crate::battle::wild::{EncounterContext, WildEncounterRandoms};
        use pokered_data::wild_data::GameVersion;

        let version = GameVersion::Red;

        let encounter_roll = (self
            .frame_counter
            .wrapping_mul(1103515245)
            .wrapping_add(12345)
            >> 8) as u8;
        let slot_roll = (self.frame_counter.wrapping_mul(22695477).wrapping_add(1) >> 8) as u8;

        let randoms = WildEncounterRandoms {
            encounter_roll,
            slot_roll,
        };

        let context = EncounterContext {
            repel_active: self.state.repel_steps > 0,
            party_lead_level: 5,
        };

        let result = wild_encounters::check_wild_encounter(
            map_id,
            tileset,
            standing_tile,
            version,
            &randoms,
            &context,
            standing_on_warp,
            has_script_effect,
            self.state.encounter_cooldown,
        );

        if let crate::battle::wild::WildEncounterResult::Encounter { level, species } = result {
            self.pending_wild_encounter = Some(PendingWildEncounter { species, level });
        }
    }
}
