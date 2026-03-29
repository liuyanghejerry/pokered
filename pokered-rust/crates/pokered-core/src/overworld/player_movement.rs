//! Player movement system for the overworld.
//!
//! Implements M4.2 movement logic ported from:
//! - home/overworld.asm (OverworldLoop, movement input handling)
//! - engine/overworld/movement.asm (TryWalking, AdvancePlayerSprite)
//! - engine/overworld/player_state.asm (player state transitions)

use super::collision::{
    check_movement_collision, check_warp_at_position, is_facing_map_edge, CollisionResult,
    SpritePosition,
};
use super::{Direction, MapData, MovementState, OverworldState, TransportMode};
use pokered_data::maps::MapId;
use pokered_data::tileset_data;
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
            state.standing_on_warp = false;
            state.player.movement_state = MovementState::Walking;
            state.walk_counter = WALK_COUNTER_INIT;
            MoveResult::Walking
        }
        CollisionResult::LedgeJump => {
            state.standing_on_warp = false;
            state.player.movement_state = MovementState::Jumping;
            state.walk_counter = WALK_COUNTER_INIT * 2;
            MoveResult::LedgeJump
        }
        CollisionResult::MapEdge => MoveResult::ReachedMapEdge,
        _ => {
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

/// Convert Direction to the facing index used by warp carpet tile lookups.
/// 0=Down, 1=Up, 2=Left, 3=Right (matches original sprite facing / 4).
fn direction_to_facing_index(dir: Direction) -> u8 {
    match dir {
        Direction::Down => 0,
        Direction::Up => 1,
        Direction::Left => 2,
        Direction::Right => 3,
    }
}

/// Determine whether ExtraWarpCheck should use "function 2"
/// (IsWarpTileInFrontOfPlayer) instead of the default "function 1"
/// (IsPlayerFacingEdgeOfMap).
///
/// Ported from home/overworld.asm ExtraWarpCheck (line 719).
/// Function 2 is used for: OVERWORLD, SHIP, SHIP_PORT, PLATEAU tilesets,
/// plus ROCKET_HIDEOUT_B1F/B2F/B4F and ROCK_TUNNEL_1F maps.
/// Exception: SS_ANNE_3F forces function 1 despite SHIP tileset.
fn uses_warp_tile_in_front_check(map_id: MapId, tileset: TilesetId) -> bool {
    if map_id == MapId::SSAnne3F {
        return false;
    }
    matches!(
        map_id,
        MapId::RocketHideoutB1F
            | MapId::RocketHideoutB2F
            | MapId::RocketHideoutB4F
            | MapId::RockTunnel1F
    ) || matches!(
        tileset,
        TilesetId::Overworld | TilesetId::Ship | TilesetId::ShipPort | TilesetId::Plateau
    )
}

/// ExtraWarpCheck equivalent from home/overworld.asm.
/// Returns true if the extra warp condition passes.
///
/// For function 1 (most indoor maps): checks if the player is facing the map edge.
/// For function 2 (overworld + special maps): checks if the tile in front of the
/// player is a warp carpet tile for the current facing direction.
/// Special case: SS_ANNE_BOW checks for tile 0x15 when facing any direction.
fn extra_warp_check(map: &MapData, player_x: u16, player_y: u16, facing: Direction) -> bool {
    if map.id == MapId::SSAnneBow {
        let tile_in_front = get_target_tile_for_direction(map, player_x, player_y, facing);
        return tile_in_front == 0x15;
    }

    if uses_warp_tile_in_front_check(map.id, map.tileset) {
        let tile_in_front = get_target_tile_for_direction(map, player_x, player_y, facing);
        let facing_idx = direction_to_facing_index(facing);
        tileset_data::is_warp_carpet_tile_in_front(facing_idx, tile_in_front)
    } else {
        is_facing_map_edge(player_x, player_y, facing, map.width, map.height)
    }
}

/// Two-phase warp check after a step completes onto a warp position.
///
/// Ported from home/overworld.asm CheckWarpsNoCollision (line 391).
/// Phase 1: position matches warp → set BIT_STANDING_ON_WARP →
///   check IsPlayerStandingOnDoorTileOrWarpTile → if door → immediate warp.
///   If warp tile → clear standing_on_warp, fall through to ExtraWarpCheck.
///   If neither → fall through to ExtraWarpCheck.
/// Phase 2 (ExtraWarpCheck): if passes AND direction held → warp.
///   Otherwise → don't warp (standing_on_warp remains set for collision path).
fn check_warps_no_collision(
    state: &mut OverworldState,
    map: &MapData,
    standing_tile: u8,
    direction_held: bool,
) -> Option<usize> {
    let warp_idx = check_warp_at_position(state.player.x, state.player.y, map)?;

    state.standing_on_warp = true;

    // IsPlayerStandingOnDoorTileOrWarpTile
    if tileset_data::is_door_tile(map.tileset, standing_tile) {
        return Some(warp_idx);
    }

    if tileset_data::is_warp_tile(map.tileset, standing_tile) {
        state.standing_on_warp = false;
        // Fall through to ExtraWarpCheck
    }
    // If neither door nor warp tile, also fall through to ExtraWarpCheck

    if extra_warp_check(map, state.player.x, state.player.y, state.player.facing) {
        if direction_held {
            return Some(warp_idx);
        }
    }

    // Warp didn't fire — standing_on_warp stays set (unless cleared by warp tile path)
    None
}

/// CheckWarpsCollision path: when collision occurs while standing_on_warp is set,
/// run ExtraWarpCheck and if it passes, warp at current position.
/// Ported from home/overworld.asm lines 244-257.
fn check_collision_warp(
    state: &mut OverworldState,
    map: &MapData,
    move_result: MoveResult,
) -> MoveResult {
    match move_result {
        MoveResult::Blocked(_) | MoveResult::ReachedMapEdge => {
            if state.standing_on_warp {
                if extra_warp_check(map, state.player.x, state.player.y, state.player.facing) {
                    if let Some(warp_idx) =
                        check_warp_at_position(state.player.x, state.player.y, map)
                    {
                        return MoveResult::Warped {
                            warp_index: warp_idx,
                        };
                    }
                }
            }
            move_result
        }
        _ => move_result,
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
            // Step complete — run two-phase warp check (CheckWarpsNoCollision)
            let new_standing_tile = get_tile_at_position(map, state.player.x, state.player.y);
            let direction_held = input.direction_pressed().is_some();

            if let Some(warp_idx) =
                check_warps_no_collision(state, map, new_standing_tile, direction_held)
            {
                return MoveResult::Warped {
                    warp_index: warp_idx,
                };
            }

            // If direction still held, immediately start next step
            if let Some(direction) = input.direction_pressed() {
                let held_input = input.to_pad_bits();

                let new_target_tile =
                    get_target_tile_for_direction(map, state.player.x, state.player.y, direction);

                let move_result = try_move(
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

                return check_collision_warp(state, map, move_result);
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

    let move_result = try_move(
        state,
        direction,
        map.tileset,
        map.width,
        map.height,
        standing_tile,
        target_tile,
        npc_positions,
        held_input,
    );

    check_collision_warp(state, map, move_result)
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
