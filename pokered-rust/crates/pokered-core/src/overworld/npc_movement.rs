//! NPC movement system — runtime state machine for NPC sprites.
//!
//! Implements M4.4 (NPC移动和交互) ported from:
//! - engine/overworld/movement.asm (DoMovementForAllSprites, TryWalkingNPC)
//! - engine/overworld/npc_movement.asm (NPC movement patterns)
//! - engine/overworld/sprite_collisions.asm (DetectCollisionBetweenSprites)
//! - home/overworld.asm (UpdateNPCSprite, GetSpriteMovementByte2)
//!
//! Each NPC on a map has a runtime state (`NpcRuntimeState`) that tracks
//! its current position, facing, walk counter, and movement pattern.
//! The `update_npc_movement` function advances all NPCs one frame.

use pokered_data::npc_data::{NpcEntry, NpcFacing, NpcMovement};

use super::collision::SpritePosition;
use super::player_movement::direction_delta;
use super::{Direction, NpcMovementType};

// ── NPC Runtime State ──────────────────────────────────────────────

/// Runtime state for a single NPC on the current map.
#[derive(Debug, Clone)]
pub struct NpcRuntimeState {
    /// Index into the map's NPC list (0-based).
    pub npc_index: u8,
    /// Sprite ID for rendering.
    pub sprite_id: u8,
    /// Current tile X position.
    pub x: u16,
    /// Current tile Y position.
    pub y: u16,
    /// Starting tile X (for wander range checks).
    pub home_x: u16,
    /// Starting tile Y (for wander range checks).
    pub home_y: u16,
    /// Current facing direction.
    pub facing: Direction,
    /// Movement pattern.
    pub movement_type: NpcMovementType,
    /// Wander range in tiles (0 = stationary).
    pub range: u8,
    /// Walk counter (frames until current step completes). 0 = idle.
    pub walk_counter: u8,
    /// Frame counter for movement delay (NPCs wait a random delay
    /// between steps, originally 0-63 frames).
    pub delay_counter: u8,
    /// Text ID for interaction.
    pub text_id: u8,
    /// Whether this NPC is a trainer.
    pub is_trainer: bool,
    /// Trainer class ID (0 if not a trainer).
    pub trainer_class: u8,
    /// Trainer set index (0 if not a trainer).
    pub trainer_set: u8,
    /// Item ID (for item-ball NPCs, 0 = none).
    pub item_id: u8,
    /// Whether this NPC has been "defeated" (trainer defeated or item taken).
    /// Checked via event flags in the real game; here we store runtime state.
    pub defeated: bool,
    /// Whether this NPC is currently visible (some NPCs are hidden by scripts).
    pub visible: bool,
}

/// Convert pokered-data NpcMovement to pokered-core NpcMovementType.
pub fn convert_movement(m: NpcMovement) -> NpcMovementType {
    match m.0 {
        0 => NpcMovementType::Stationary,
        1 => NpcMovementType::Wander,
        2 => NpcMovementType::FixedPath,
        3 => NpcMovementType::FacePlayer,
        _ => NpcMovementType::Stationary,
    }
}

/// Convert pokered-data NpcFacing to pokered-core Direction.
pub fn convert_facing(f: NpcFacing) -> Direction {
    match f.0 {
        0 => Direction::Down,
        1 => Direction::Up,
        2 => Direction::Left,
        3 => Direction::Right,
        _ => Direction::Down,
    }
}

/// Walk counter value for NPC steps (same as player: 8 frames per tile).
pub const NPC_WALK_FRAMES: u8 = 8;

/// Maximum wander delay (NPCs wait 0..=63 frames between steps in the original game).
pub const NPC_MAX_DELAY: u8 = 63;

/// Create runtime NPC states from static data for a map.
pub fn load_map_npcs(npcs: &[NpcEntry]) -> Vec<NpcRuntimeState> {
    npcs.iter()
        .enumerate()
        .map(|(i, npc)| NpcRuntimeState {
            npc_index: i as u8,
            sprite_id: npc.sprite_id,
            x: npc.x as u16,
            y: npc.y as u16,
            home_x: npc.x as u16,
            home_y: npc.y as u16,
            facing: convert_facing(npc.facing),
            movement_type: convert_movement(npc.movement),
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

/// Get sprite positions for all visible NPCs (used for collision checks).
pub fn get_npc_positions(npcs: &[NpcRuntimeState]) -> Vec<SpritePosition> {
    npcs.iter()
        .filter(|n| n.visible)
        .map(|n| SpritePosition { x: n.x, y: n.y })
        .collect()
}

/// Advance all NPC movements by one frame.
///
/// In the original game (DoMovementForAllSprites), each NPC is updated
/// per frame. The movement depends on the NPC's movement byte:
/// - $FF (STAY) = stationary, don't move
/// - $FE (WALK) = wander within range
/// - Others = fixed path (scripted movements)
///
/// NPCs with WALK movement pick a random direction every N frames
/// and attempt to walk there if within range and not blocked.
///
/// `rng_value` is a pseudo-random byte provided by the caller for
/// NPC movement decisions. In the original game this comes from
/// the random number generator seeded by the frame counter.
///
/// `player_x`, `player_y` — player tile position (for FacePlayer NPCs).
/// `map_width_blocks`, `map_height_blocks` — map dimensions for bounds checking.
pub fn update_npc_movement(
    npcs: &mut [NpcRuntimeState],
    player_x: u16,
    player_y: u16,
    map_width_blocks: u8,
    map_height_blocks: u8,
    rng_value: u8,
) {
    let max_x = (map_width_blocks as u16) * 2;
    let max_y = (map_height_blocks as u16) * 2;

    // Collect current positions to avoid two NPCs walking into each other
    let positions: Vec<(u16, u16)> = npcs.iter().map(|n| (n.x, n.y)).collect();

    for i in 0..npcs.len() {
        let npc = &mut npcs[i];
        if !npc.visible {
            continue;
        }

        // If currently walking, advance the step
        if npc.walk_counter > 0 {
            npc.walk_counter -= 1;
            if npc.walk_counter == 0 {
                // Step complete — update position
                let (dx, dy) = direction_delta(npc.facing);
                npc.x = (npc.x as i32 + dx as i32).max(0) as u16;
                npc.y = (npc.y as i32 + dy as i32).max(0) as u16;
            }
            continue;
        }

        match npc.movement_type {
            NpcMovementType::Stationary => {
                // Do nothing — stays in place.
            }
            NpcMovementType::Wander => {
                // Delay between steps
                if npc.delay_counter > 0 {
                    npc.delay_counter -= 1;
                    continue;
                }

                // Pick a random direction using the rng_value mixed with NPC index
                let dir_bits = (rng_value.wrapping_add(i as u8)) & 0x03;
                let dir = match dir_bits {
                    0 => Direction::Down,
                    1 => Direction::Up,
                    2 => Direction::Left,
                    3 => Direction::Right,
                    _ => unreachable!(),
                };

                // Calculate target position
                let (dx, dy) = direction_delta(dir);
                let tx = (npc.x as i32 + dx as i32) as u16;
                let ty = (npc.y as i32 + dy as i32) as u16;

                // Check bounds
                if tx >= max_x || ty >= max_y {
                    npc.facing = dir;
                    npc.delay_counter = rng_value & NPC_MAX_DELAY;
                    continue;
                }

                // Check wander range (Manhattan distance from home)
                if npc.range > 0 {
                    let dist_x = (tx as i32 - npc.home_x as i32).unsigned_abs();
                    let dist_y = (ty as i32 - npc.home_y as i32).unsigned_abs();
                    if dist_x > npc.range as u32 || dist_y > npc.range as u32 {
                        npc.facing = dir;
                        npc.delay_counter = rng_value & NPC_MAX_DELAY;
                        continue;
                    }
                }

                // Check collision with other NPCs
                let blocked = positions
                    .iter()
                    .enumerate()
                    .any(|(j, &(ox, oy))| j != i && ox == tx && oy == ty);
                // Also check player position
                let player_blocked = tx == player_x && ty == player_y;

                if blocked || player_blocked {
                    npc.facing = dir;
                    npc.delay_counter = rng_value & NPC_MAX_DELAY;
                    continue;
                }

                // Start walking
                npc.facing = dir;
                npc.walk_counter = NPC_WALK_FRAMES;
                // Set delay for after this step completes
                npc.delay_counter = rng_value & NPC_MAX_DELAY;
            }
            NpcMovementType::FacePlayer => {
                // Turn to face the player (only when not walking).
                // In the original game, this is checked every frame for
                // NPCs with movement type $FF that have the FacePlayer flag.
                let dx = player_x as i32 - npc.x as i32;
                let dy = player_y as i32 - npc.y as i32;

                // Face toward the player using the axis with the greater distance
                if dx.abs() >= dy.abs() {
                    npc.facing = if dx > 0 {
                        Direction::Right
                    } else {
                        Direction::Left
                    };
                } else {
                    npc.facing = if dy > 0 {
                        Direction::Down
                    } else {
                        Direction::Up
                    };
                }
            }
            NpcMovementType::FixedPath => {
                // Fixed-path NPCs follow scripted movement sequences.
                // In the original game, the movement data comes from map
                // scripts. For now, fixed-path NPCs behave as stationary
                // since the movement scripts are not yet implemented.
                // TODO: Implement scripted movement paths (M4.7 or later).
            }
        }
    }
}

/// Find the NPC at a given tile position, if any.
pub fn npc_at_position(npcs: &[NpcRuntimeState], x: u16, y: u16) -> Option<&NpcRuntimeState> {
    npcs.iter().find(|n| n.visible && n.x == x && n.y == y)
}

/// Find the NPC at a given tile position (mutable), if any.
pub fn npc_at_position_mut(
    npcs: &mut [NpcRuntimeState],
    x: u16,
    y: u16,
) -> Option<&mut NpcRuntimeState> {
    npcs.iter_mut().find(|n| n.visible && n.x == x && n.y == y)
}

/// Check if an NPC is within interaction range of the player.
/// The player can interact with an NPC that is exactly one tile away
/// in the direction the player is facing.
pub fn npc_in_front_of_player(
    npcs: &[NpcRuntimeState],
    player_x: u16,
    player_y: u16,
    facing: Direction,
) -> Option<&NpcRuntimeState> {
    let (dx, dy) = direction_delta(facing);
    let target_x = (player_x as i32 + dx as i32) as u16;
    let target_y = (player_y as i32 + dy as i32) as u16;
    npc_at_position(npcs, target_x, target_y)
}
