//! Collision detection for the overworld.
//!
//! Implements M4.2 collision logic ported from:
//! - engine/overworld/player_state.asm (CheckTilePassable, IsPlayerFacingEdgeOfMap)
//! - engine/overworld/movement.asm (CanWalkOntoTile, TryWalking)
//! - engine/overworld/sprite_collisions.asm (DetectCollisionBetweenSprites)
//! - home/overworld.asm (CollisionCheckOnLand, CollisionCheckOnWater)

use pokered_data::collision::{
    check_ledge_jump, check_tile_pair_collision, is_tile_passable, SPRITE_FACING_DOWN,
    SPRITE_FACING_LEFT, SPRITE_FACING_RIGHT, SPRITE_FACING_UP,
};
use pokered_data::tilesets::TilesetId;

use super::{Direction, MapData, TransportMode};

/// Result of a collision check when the player tries to move.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionResult {
    /// Movement is allowed — the target tile is clear.
    Passable,
    /// The tile itself is impassable (wall, obstacle, etc.).
    TileBlocked,
    /// A tile pair collision prevents crossing (elevation difference).
    TilePairBlocked,
    /// An NPC or other sprite is blocking the way.
    SpriteBlocked,
    /// The player is at the edge of the map (triggers map connection).
    MapEdge,
    /// The player should jump a ledge (special movement).
    LedgeJump,
    /// Cannot surf here (water tile but no surf).
    WaterBlocked,
    /// Counter tile — can talk across but not walk onto.
    CounterTile,
}

/// Convert a Direction to the sprite facing constant used in the original game.
pub fn direction_to_sprite_facing(dir: Direction) -> u8 {
    match dir {
        Direction::Down => SPRITE_FACING_DOWN,
        Direction::Up => SPRITE_FACING_UP,
        Direction::Left => SPRITE_FACING_LEFT,
        Direction::Right => SPRITE_FACING_RIGHT,
    }
}

/// Convert a Direction to the d-pad input bitmask.
pub fn direction_to_pad_input(dir: Direction) -> u8 {
    match dir {
        Direction::Down => pokered_data::collision::PAD_DOWN,
        Direction::Up => pokered_data::collision::PAD_UP,
        Direction::Left => pokered_data::collision::PAD_LEFT,
        Direction::Right => pokered_data::collision::PAD_RIGHT,
    }
}

/// Get the target tile coordinates when moving in a direction.
/// Returns None if movement would go out of map bounds.
///
/// Coordinates are in tile space (2x block space).
/// Map dimensions (width, height) are in blocks.
/// So the valid tile range is 0..(width*2 - 1) and 0..(height*2 - 1).
///
/// In the original game, the player's position is stored in tile coordinates.
/// The map has a 2-tile border (CONNECTION_STRIP_LENGTH) on each side for
/// map connections, but for collision purposes we check against the actual
/// map bounds.
pub fn get_target_coords(
    x: u16,
    y: u16,
    direction: Direction,
    map_width_blocks: u8,
    map_height_blocks: u8,
) -> Option<(u16, u16)> {
    let max_x = (map_width_blocks as u16) * 2;
    let max_y = (map_height_blocks as u16) * 2;

    match direction {
        Direction::Up => {
            if y == 0 {
                None // map edge
            } else {
                Some((x, y - 1))
            }
        }
        Direction::Down => {
            if y + 1 >= max_y {
                None // map edge
            } else {
                Some((x, y + 1))
            }
        }
        Direction::Left => {
            if x == 0 {
                None // map edge
            } else {
                Some((x - 1, y))
            }
        }
        Direction::Right => {
            if x + 1 >= max_x {
                None // map edge
            } else {
                Some((x + 1, y))
            }
        }
    }
}

/// Get the tile ID at a position in the map's block data.
///
/// Each block is a 2x2 grid of tiles. Given tile coordinates (tx, ty),
/// the block index is (ty/2) * map_width + (tx/2), and the tile within
/// the block is determined by (tx%2, ty%2):
///   (0,0) = top-left, (1,0) = top-right
///   (0,1) = bottom-left, (1,1) = bottom-right
///
/// In the original game, tiles are looked up from the map block data
/// plus the tileset's block definitions. For our purposes, we store
/// a pre-expanded tile map or use the block index directly.
///
/// Since we don't have the full tile expansion (that requires tileset
/// block definitions which are graphics data), we work at the block level.
/// The `blocks` array contains block IDs, not individual tile IDs.
///
/// For collision purposes, the original game reads from wCurrentTileBlockMapViewPointer
/// which points to a buffer of tile IDs. We simulate this by accepting
/// a tile_map parameter (the expanded tile buffer for the current viewport).
pub fn get_block_at(x: u16, y: u16, map_width_blocks: u8, blocks: &[u8]) -> Option<u8> {
    let bx = (x / 2) as usize;
    let by = (y / 2) as usize;
    let w = map_width_blocks as usize;
    let idx = by * w + bx;
    blocks.get(idx).copied()
}

/// Represents the state needed for a sprite collision check.
/// In the original game, each sprite has a 16x16 pixel bounding box
/// and collision is checked against all other sprites on the map.
#[derive(Debug, Clone, Copy)]
pub struct SpritePosition {
    /// Tile X coordinate.
    pub x: u16,
    /// Tile Y coordinate.
    pub y: u16,
}

/// Check if an NPC sprite occupies the target tile.
///
/// In the original game (DetectCollisionBetweenSprites), this is done
/// by comparing pixel positions with an 8-pixel threshold. Since we
/// work in tile coordinates, we simply check for exact tile match.
pub fn check_sprite_collision(
    target_x: u16,
    target_y: u16,
    npc_positions: &[SpritePosition],
) -> bool {
    npc_positions
        .iter()
        .any(|npc| npc.x == target_x && npc.y == target_y)
}

/// Full collision check for player movement.
///
/// This is the main entry point that combines all collision checks
/// in the same order as the original game:
///
/// 1. Check map edge (triggers map connection)
/// 2. Check ledge jump (special movement)
/// 3. Check sprite collision (NPC blocking)
/// 4. Check tile pair collision (elevation)
/// 5. Check tile passability (wall/obstacle)
///
/// The `standing_tile` and `target_tile` parameters represent the tile IDs
/// the player is standing on and trying to walk onto. These come from
/// the expanded tile buffer (not raw block IDs).
///
/// For the initial implementation, callers provide tile IDs directly.
/// The full tile expansion (block → 4 tiles) requires tileset graphics data
/// which is loaded at runtime.
pub fn check_movement_collision(
    player_x: u16,
    player_y: u16,
    direction: Direction,
    tileset: TilesetId,
    map_width_blocks: u8,
    map_height_blocks: u8,
    standing_tile: u8,
    target_tile: u8,
    transport: TransportMode,
    npc_positions: &[SpritePosition],
    held_input: u8,
) -> CollisionResult {
    // 1. Check map edge
    let target_coords = get_target_coords(
        player_x,
        player_y,
        direction,
        map_width_blocks,
        map_height_blocks,
    );

    if target_coords.is_none() {
        return CollisionResult::MapEdge;
    }

    let (tx, ty) = target_coords.unwrap();

    // 2. Check ledge jump (only on land, only overworld tileset)
    if transport == TransportMode::Walking || transport == TransportMode::Biking {
        let sprite_facing = direction_to_sprite_facing(direction);
        if check_ledge_jump(sprite_facing, standing_tile, target_tile, held_input) {
            return CollisionResult::LedgeJump;
        }
    }

    // 3. Check sprite collision
    if check_sprite_collision(tx, ty, npc_positions) {
        return CollisionResult::SpriteBlocked;
    }

    // 4. Check tile pair collision
    let on_water = transport == TransportMode::Surfing;
    if check_tile_pair_collision(tileset, standing_tile, target_tile, on_water) {
        return CollisionResult::TilePairBlocked;
    }

    // 5. Check counter tile (can interact across but not walk through)
    let header = pokered_data::tileset_data::get_tileset_header(tileset);
    if header.is_counter_tile(target_tile) {
        return CollisionResult::CounterTile;
    }

    // 6. Check tile passability
    if !is_tile_passable(tileset, target_tile) {
        return CollisionResult::TileBlocked;
    }

    // 7. Water movement check: if surfing, target must be water-passable;
    //    if walking, water tiles block (need Surf)
    // Note: In the original game, water tiles ARE in the passable list
    // (e.g., 0x14 for water in overworld). The game uses the transport mode
    // to determine if the player can enter water.
    // For simplicity, this is handled at the movement layer.

    CollisionResult::Passable
}

/// Check if the player is facing the edge of the map.
/// This determines if a map connection should be triggered.
///
/// Ported from engine/overworld/player_state.asm IsPlayerFacingEdgeOfMap.
pub fn is_facing_map_edge(
    player_x: u16,
    player_y: u16,
    direction: Direction,
    map_width_blocks: u8,
    map_height_blocks: u8,
) -> bool {
    get_target_coords(
        player_x,
        player_y,
        direction,
        map_width_blocks,
        map_height_blocks,
    )
    .is_none()
}

/// Check if a tile is a door/warp tile.
/// In the original game, stepping onto certain tiles triggers warps.
/// The warp tiles are defined per-map in the map objects data.
///
/// This function checks if the player's current position matches
/// any warp point in the map data.
pub fn check_warp_at_position(x: u16, y: u16, map: &MapData) -> Option<usize> {
    // Warp positions are in block coordinates in the original game,
    // but our WarpPoint stores them as provided by map data.
    // Convert tile coords to block coords for comparison.
    let bx = (x / 2) as u8;
    let by = (y / 2) as u8;

    map.warps.iter().position(|w| w.x == bx && w.y == by)
}
