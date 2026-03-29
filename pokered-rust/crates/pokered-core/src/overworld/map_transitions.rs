//! Map transition logic — connection crossings and warp execution.
//!
//! Implements M4.3 (地图连接和传送点) ported from:
//! - home/overworld.asm: CheckMapConnections (line 548+)
//! - home/overworld.asm: CheckWarpsNoCollision / CheckWarpsCollision
//! - engine/overworld/special_warps.asm: special warp handling
//!
//! When the player walks off a map edge, `calculate_connection_transition`
//! determines the new map and coordinates based on map connection data.
//! When the player steps on a warp tile, `execute_warp` handles the
//! destination lookup.

use pokered_data::map_connections::get_map_connections;
use pokered_data::map_objects::get_map_warps;
use pokered_data::maps::MapId;

use super::Direction;

// ── Connection Transition ──────────────────────────────────────────

/// Result of a connection transition — the new map, and the player's
/// position in that map (in 2×2 tile coordinates, matching the original
/// game's coordinate system where each map block = 2×2 tiles).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConnectionTransition {
    pub new_map: MapId,
    /// Player X in the destination map (tile coordinates, 2 per block).
    pub new_x: u16,
    /// Player Y in the destination map (tile coordinates, 2 per block).
    pub new_y: u16,
}

/// Calculate the connection transition when the player walks off a map edge.
///
/// The original game (CheckMapConnections in home/overworld.asm) checks
/// the player's coordinates after they wrap (e.g., X becomes 0xFF for west).
/// Here we check BEFORE the move: the player is at `(px, py)` and wants to
/// move in `direction`. The current map has dimensions `(map_w, map_h)` in blocks,
/// so tile dimensions are `(map_w*2, map_h*2)`.
///
/// The `offset` field in ConnectionData is in block coordinates.
/// It represents the alignment shift between the two connected maps.
/// A positive offset means the destination map's origin is shifted right/down
/// relative to the current map's edge.
///
/// # Arguments
/// - `current_map` — the map the player is currently on
/// - `px`, `py` — player's current tile coordinates
/// - `direction` — the direction the player is moving
///
/// # Returns
/// `Some(ConnectionTransition)` if there is a valid connection, `None` otherwise.
pub fn calculate_connection_transition(
    current_map: MapId,
    px: u16,
    py: u16,
    direction: Direction,
) -> Option<ConnectionTransition> {
    let conns = get_map_connections(current_map);
    let current_w = current_map.width() as u16 * 2; // tile width
    let current_h = current_map.height() as u16 * 2; // tile height

    match direction {
        Direction::Up => {
            // Player is at Y=0, moving up → north connection
            if py != 0 {
                return None;
            }
            let conn = conns.north.as_ref()?;
            let dest = conn;
            let dest_h = dest.target_map.height() as u16 * 2;
            // New Y: bottom edge of destination map
            let new_y = dest_h - 1;
            // New X: current X adjusted by connection offset (in blocks → *2 for tiles)
            let new_x = apply_offset(px, dest.offset);
            Some(ConnectionTransition {
                new_map: dest.target_map,
                new_x,
                new_y,
            })
        }
        Direction::Down => {
            // Player is at Y=map_h*2-1, moving down → south connection
            if py != current_h - 1 {
                return None;
            }
            let conn = conns.south.as_ref()?;
            let dest = conn;
            // New Y: top edge of destination map
            let new_y = 0;
            let new_x = apply_offset(px, dest.offset);
            Some(ConnectionTransition {
                new_map: dest.target_map,
                new_x,
                new_y,
            })
        }
        Direction::Left => {
            // Player is at X=0, moving left → west connection
            if px != 0 {
                return None;
            }
            let conn = conns.west.as_ref()?;
            let dest = conn;
            let dest_w = dest.target_map.width() as u16 * 2;
            // New X: right edge of destination map
            let new_x = dest_w - 1;
            let new_y = apply_offset(py, dest.offset);
            Some(ConnectionTransition {
                new_map: dest.target_map,
                new_x,
                new_y,
            })
        }
        Direction::Right => {
            // Player is at X=map_w*2-1, moving right → east connection
            if px != current_w - 1 {
                return None;
            }
            let conn = conns.east.as_ref()?;
            let dest = conn;
            // New X: left edge of destination map
            let new_x = 0;
            let new_y = apply_offset(py, dest.offset);
            Some(ConnectionTransition {
                new_map: dest.target_map,
                new_x,
                new_y,
            })
        }
    }
}

/// Apply a connection offset (in blocks) to a tile coordinate.
///
/// The offset represents how many blocks the connected map is shifted.
/// In the original game:
/// - Positive offset: the connected map starts further along the axis
/// - Negative offset: the connected map starts before the current map's edge
///
/// The formula: new_coord = current_coord + offset * 2
/// (offset is in blocks, coordinates are in tiles, 2 tiles per block)
fn apply_offset(coord: u16, offset: i8) -> u16 {
    let adjusted = coord as i32 + (offset as i32 * 2);
    // Clamp to valid range (shouldn't go negative in practice for valid data)
    adjusted.max(0) as u16
}

// ── Warp Transition ────────────────────────────────────────────────

/// Result of a warp transition — the destination map and coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WarpTransition {
    pub new_map: MapId,
    /// Destination warp index in the target map.
    pub dest_warp_id: u8,
    /// Whether this is a "last map" warp (return to previous map).
    pub is_last_map: bool,
}

/// Check if the player is standing on a warp tile and execute it.
///
/// In the original game, warps are checked in CheckWarpsNoCollision
/// (home/overworld.asm). When the player's (x, y) matches a warp entry,
/// the game loads the destination map and positions the player at the
/// destination warp's coordinates.
///
/// Warp coordinates are in 2x2 meta-tile units, so a warp at (7, 1)
/// covers tiles (14-15, 2-3). Player coordinates are in tile units.
///
/// # Arguments
/// - `current_map` — the map the player is on
/// - `px`, `py` — player's tile coordinates
///
/// # Returns
/// `Some(WarpTransition)` if standing on a warp, `None` otherwise.
pub fn check_warp_at(current_map: MapId, px: u8, py: u8) -> Option<WarpTransition> {
    let warps = get_map_warps(current_map);
    for warp in warps {
        // Convert warp coords from 2x2 meta-tile units to tile units
        let warp_tile_x = (warp.x as u16) * 2;
        let warp_tile_y = (warp.y as u16) * 2;
        // Check if player is within the 2x2 warp area
        if (px as u16) >= warp_tile_x
            && (px as u16) < warp_tile_x + 2
            && (py as u16) >= warp_tile_y
            && (py as u16) < warp_tile_y + 2
        {
            return Some(WarpTransition {
                new_map: warp.dest_map.unwrap_or(current_map),
                dest_warp_id: warp.dest_warp_id,
                is_last_map: warp.dest_map.is_none(),
            });
        }
    }
    None
}

/// Resolve the destination position for a warp by looking up the target
/// warp in the destination map.
///
/// When warping, the game places the player at the position of the
/// destination warp (indexed by dest_warp_id) in the target map.
///
/// Warp coordinates are in 2x2 meta-tile units, so they are multiplied
/// by 2 to return tile coordinates.
///
/// # Arguments
/// - `dest_map` — the destination map
/// - `dest_warp_id` — the warp index in the destination map (0-based)
///
/// # Returns
/// `Some((x, y))` tile coordinates of the destination warp, or `None`
/// if the warp index is out of bounds.
pub fn resolve_warp_destination(dest_map: MapId, dest_warp_id: u8) -> Option<(u8, u8)> {
    let warps = get_map_warps(dest_map);
    let idx = dest_warp_id as usize;
    if idx < warps.len() {
        // Convert from 2x2 meta-tile coords to tile coords
        Some((warps[idx].x * 2, warps[idx].y * 2))
    } else {
        None
    }
}

/// Full warp execution: check if at a warp, resolve destination coordinates.
///
/// Combines `check_warp_at` and `resolve_warp_destination` into one call.
/// Returns the new map and the player's destination tile coordinates.
///
/// For "last map" warps, the caller must supply `last_map` (the map the
/// player came from). If `last_map` is None, last-map warps return None.
///
/// # Arguments
/// - `current_map` — current map
/// - `px`, `py` — player tile position
/// - `last_map` — the previous map (for LAST_MAP warps)
///
/// # Returns
/// `Some((map, x, y))` if a warp is found and resolvable.
pub fn execute_warp(
    current_map: MapId,
    px: u8,
    py: u8,
    last_map: Option<MapId>,
) -> Option<(MapId, u8, u8)> {
    let transition = check_warp_at(current_map, px, py)?;

    let dest_map = if transition.is_last_map {
        last_map?
    } else {
        transition.new_map
    };

    let (dx, dy) = resolve_warp_destination(dest_map, transition.dest_warp_id)?;
    Some((dest_map, dx, dy))
}
