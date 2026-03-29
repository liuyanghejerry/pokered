//! Player movement system for the overworld.
//!
//! Implements M4.2 movement logic ported from:
//! - home/overworld.asm (OverworldLoop, movement input handling)
//! - engine/overworld/movement.asm (TryWalking, AdvancePlayerSprite)
//! - engine/overworld/player_state.asm (player state transitions)

use super::collision::{
    check_movement_collision, check_warp_at_position, CollisionResult, SpritePosition,
};
use super::{Direction, MapData, MovementState, OverworldState, TransportMode};
use pokered_data::tilesets::TilesetId;

/// Walk counter initial value.
/// In the original game, the walk counter starts at 8 and counts down to 0.
/// Each frame decrements by 1. At 0, the step is complete.
pub const WALK_COUNTER_INIT: u8 = 8;

/// Input state from the player's controller.
/// Maps to the Game Boy's joypad register bits.
#[derive(Debug, Clone, Copy, Default)]
pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a_button: bool,
    pub b_button: bool,
    pub start: bool,
    pub select: bool,
}

impl InputState {
    /// Get the direction being pressed, if any.
    /// Priority matches the original game: Down > Up > Left > Right.
    /// In the original game, the hardware joypad register is read with
    /// Down/Up in bits 7/6, Left/Right in bits 5/4.
    pub fn direction_pressed(&self) -> Option<Direction> {
        if self.down {
            Some(Direction::Down)
        } else if self.up {
            Some(Direction::Up)
        } else if self.left {
            Some(Direction::Left)
        } else if self.right {
            Some(Direction::Right)
        } else {
            None
        }
    }

    /// Convert to the raw d-pad bitmask used in the original game.
    pub fn to_pad_bits(&self) -> u8 {
        let mut bits = 0u8;
        if self.down {
            bits |= pokered_data::collision::PAD_DOWN;
        }
        if self.up {
            bits |= pokered_data::collision::PAD_UP;
        }
        if self.left {
            bits |= pokered_data::collision::PAD_LEFT;
        }
        if self.right {
            bits |= pokered_data::collision::PAD_RIGHT;
        }
        bits
    }
}

/// Result of processing a movement attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveResult {
    /// Player started walking to the target tile.
    Walking,
    /// Player started a ledge jump.
    LedgeJump,
    /// Player only turned to face a new direction (no movement).
    TurnedOnly,
    /// Movement was blocked (wall, NPC, etc.).
    Blocked(CollisionResult),
    /// Player reached the edge of the map (connection should be checked).
    ReachedMapEdge,
    /// Player stepped onto a warp tile.
    Warped { warp_index: usize },
    /// Player is still mid-step from previous movement.
    StillMoving,
    /// No input was pressed.
    NoInput,
}

/// Try to move the player in a direction.
///
/// This is the main movement entry point, called each frame when the
/// player presses a d-pad direction.
///
/// In the original game (home/overworld.asm), the flow is:
/// 1. Read joypad input
/// 2. If walk counter > 0, continue current movement
/// 3. Otherwise, check collision for the new direction
/// 4. If passable, start walking (set walk counter = 8)
/// 5. If blocked, just face that direction
///
/// Returns a MoveResult indicating what happened.
pub fn try_move(
    state: &mut OverworldState,
    direction: Direction,
    tileset: TilesetId,
    map_width_blocks: u8,
    map_height_blocks: u8,
    standing_tile: u8,
    target_tile: u8,
    npc_positions: &[SpritePosition],
    held_input: u8,
) -> MoveResult {
    // If already moving, can't start a new movement
    if state.player.movement_state != MovementState::Idle {
        return MoveResult::StillMoving;
    }

    // Check if just turning (facing a different direction)
    let was_facing = state.player.facing;
    state.player.facing = direction;

    // In the original game, if the player turns 180 degrees,
    // there's an intermediate frame, but it's barely visible.
    // We simplify by allowing instant turn.

    // Check collision
    let result = check_movement_collision(
        state.player.x,
        state.player.y,
        direction,
        tileset,
        map_width_blocks,
        map_height_blocks,
        standing_tile,
        target_tile,
        state.player.transport,
        npc_positions,
        held_input,
    );

    match result {
        CollisionResult::Passable => {
            // Start walking
            state.player.movement_state = MovementState::Walking;
            state.walk_counter = WALK_COUNTER_INIT;
            MoveResult::Walking
        }
        CollisionResult::LedgeJump => {
            // Start ledge jump (two tiles in the facing direction)
            state.player.movement_state = MovementState::Jumping;
            state.walk_counter = WALK_COUNTER_INIT * 2; // Ledge jump takes 16 frames
            MoveResult::LedgeJump
        }
        CollisionResult::MapEdge => {
            // Signal that we need to check map connections
            MoveResult::ReachedMapEdge
        }
        _ => {
            // Blocked — player just faces the direction
            if was_facing != direction {
                MoveResult::TurnedOnly
            } else {
                MoveResult::Blocked(result)
            }
        }
    }
}

/// Advance the player's position by one pixel-step during movement.
///
/// In the original game (AdvancePlayerSprite), the player moves 2 pixels
/// per frame (1 tile = 16 pixels, so 8 frames per tile).
/// On a bike, DoBikeSpeedup calls this twice per frame, doubling speed.
///
/// We work in tile coordinates, so instead of pixel-by-pixel movement,
/// we decrement the walk counter and move a full tile when it reaches 0.
///
/// Returns true when the step is complete (walk counter reached 0).
pub fn advance_step(state: &mut OverworldState) -> bool {
    if state.walk_counter == 0 {
        return true;
    }

    // On bike, decrement twice per frame (double speed)
    let decrement = if state.player.transport == TransportMode::Biking {
        2
    } else {
        1
    };

    state.walk_counter = state.walk_counter.saturating_sub(decrement);

    if state.walk_counter == 0 {
        // Step complete — update tile position
        let (dx, dy) = direction_delta(state.player.facing);
        let new_x = (state.player.x as i32 + dx as i32) as u16;
        let new_y = (state.player.y as i32 + dy as i32) as u16;

        if state.player.movement_state == MovementState::Jumping {
            // Ledge jump moves 2 tiles
            let new_x = (state.player.x as i32 + dx as i32 * 2) as u16;
            let new_y = (state.player.y as i32 + dy as i32 * 2) as u16;
            state.player.x = new_x;
            state.player.y = new_y;
        } else {
            state.player.x = new_x;
            state.player.y = new_y;
        }

        state.player.movement_state = MovementState::Idle;

        // Decrement encounter cooldown
        if state.encounter_cooldown > 0 {
            state.encounter_cooldown -= 1;
        }

        // Decrement repel steps
        if state.repel_steps > 0 {
            state.repel_steps -= 1;
        }

        return true;
    }

    false
}

/// Get the x/y delta for a direction.
/// Down = +Y, Up = -Y, Left = -X, Right = +X.
pub fn direction_delta(dir: Direction) -> (i8, i8) {
    match dir {
        Direction::Down => (0, 1),
        Direction::Up => (0, -1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    }
}

/// Get the opposite direction.
pub fn opposite_direction(dir: Direction) -> Direction {
    match dir {
        Direction::Down => Direction::Up,
        Direction::Up => Direction::Down,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}

/// Process one frame of overworld movement.
///
/// This is the high-level frame-by-frame update, combining input
/// processing and step advancement.
///
/// Flow per frame:
/// 1. If walk counter > 0, advance the current step
/// 2. If walk counter == 0 and input pressed, try new movement
/// 3. Check for warp after step completes
///
/// Returns the frame result.
pub fn process_frame(
    state: &mut OverworldState,
    input: &InputState,
    map: &MapData,
    standing_tile: u8,
    target_tile: u8,
    npc_positions: &[SpritePosition],
) -> MoveResult {
    // If currently moving, advance the step
    if state.player.movement_state != MovementState::Idle {
        let step_done = advance_step(state);
        if step_done {
            // Check if we stepped onto a warp
            if let Some(warp_idx) = check_warp_at_position(state.player.x, state.player.y, map) {
                return MoveResult::Warped {
                    warp_index: warp_idx,
                };
            }
            // Step complete - if direction still held, immediately start next step
            // This matches original game behavior where holding direction continuously moves
            if let Some(direction) = input.direction_pressed() {
                let held_input = input.to_pad_bits();
                // Recalculate standing_tile from the NEW player position
                let new_standing_tile = get_tile_at_position(map, state.player.x, state.player.y);

                // Calculate new target tile
                let new_target_tile =
                    get_target_tile_for_direction(map, state.player.x, state.player.y, direction);

                return try_move(
                    state,
                    direction,
                    map.tileset,
                    map.width,
                    map.height,
                    new_standing_tile,
                    new_target_tile,
                    npc_positions,
                    held_input,
                );
            }
        }
        return MoveResult::StillMoving;
    }

    // Not moving — check for new input
    let direction = match input.direction_pressed() {
        Some(dir) => dir,
        None => return MoveResult::NoInput,
    };

    let held_input = input.to_pad_bits();

    try_move(
        state,
        direction,
        map.tileset,
        map.width,
        map.height,
        standing_tile,
        target_tile,
        npc_positions,
        held_input,
    )
}

/// Get the tile ID at a specific position in the map.
/// Used for recalculating standing_tile after movement.
pub fn get_tile_at_position(map: &MapData, x: u16, y: u16) -> u8 {
    // Player coordinates are in step units (16px each).
    // Each map block is 32×32px = 2×2 steps, so divide by 2 to get block index.
    let block_x = (x / 2) as usize;
    let block_y = (y / 2) as usize;
    // Sub-position within the block: 0 = left/top half, 1 = right/bottom half
    let sub_x = (x % 2) as usize;
    let sub_y = (y % 2) as usize;

    if block_x < map.width as usize {
        let block_idx = block_y * (map.width as usize) + block_x;
        if block_idx < map.blocks.len() {
            let block_id = map.blocks[block_idx];
            // Each block has a 4×4 tile grid (16 bytes, row-major). Each step
            // position covers a 2×2 quadrant of tiles. The original game reads
            // the bottom-left tile of each quadrant for collision (matching
            // screen position (8,9) — the player's feet row in wTileMap).
            // Index formula: (sub_y * 2 + 1) * 4 + sub_x * 2
            return pokered_data::blockset_data::block_tiles(map.tileset, block_id)
                .map(|t| t[(sub_y * 2 + 1) * 4 + sub_x * 2])
                .unwrap_or(0);
        }
    }
    0
}

fn get_target_tile_for_direction(map: &MapData, x: u16, y: u16, dir: Direction) -> u8 {
    let (dx, dy) = direction_delta(dir);
    let target_x = ((x as i32) + dx as i32).max(0) as u16;
    let target_y = ((y as i32) + dy as i32).max(0) as u16;

    // Player coordinates are in step units (16px each).
    // Each map block is 32×32px = 2×2 steps.
    let block_x = (target_x / 2) as usize;
    let block_y = (target_y / 2) as usize;
    let sub_x = (target_x % 2) as usize;
    let sub_y = (target_y % 2) as usize;

    if block_x < map.width as usize {
        let block_idx = block_y * (map.width as usize) + block_x;
        if block_idx < map.blocks.len() {
            let block_id = map.blocks[block_idx];
            return pokered_data::blockset_data::block_tiles(map.tileset, block_id)
                .map(|t| t[(sub_y * 2 + 1) * 4 + sub_x * 2])
                .unwrap_or(0);
        }
    }
    0
}

/// Check if the player is currently on a grass tile.
/// Used to determine if wild encounters should be checked.
pub fn is_on_grass(standing_tile: u8, tileset: TilesetId) -> bool {
    pokered_data::tileset_data::get_tileset_header(tileset).is_grass_tile(standing_tile)
}

/// Calculate the number of frames for a step based on transport mode.
///
/// Walking: 8 frames per tile
/// Biking: 4 frames per tile (walk counter decrements by 2)
/// Surfing: 8 frames per tile (same as walking)
pub fn frames_per_step(transport: TransportMode) -> u8 {
    match transport {
        TransportMode::Walking => WALK_COUNTER_INIT,
        TransportMode::Biking => WALK_COUNTER_INIT / 2, // Effectively 4 frames
        TransportMode::Surfing => WALK_COUNTER_INIT,
    }
}
