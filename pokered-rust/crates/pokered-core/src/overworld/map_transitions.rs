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

use pokered_data::map_data_loader::{get_map_json, resolve_map_id};
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
    let map_json = get_map_json(current_map)?;
    let conns = &map_json.connections;
    let current_w = current_map.width() as u16 * 2;
    let current_h = current_map.height() as u16 * 2;

    match direction {
        Direction::Up => {
            if py != 0 {
                return None;
            }
            let conn = conns.north.as_ref()?;
            let target_map = resolve_map_id(&conn.target_map)?;
            let dest_h = target_map.height() as u16 * 2;
            let new_y = dest_h - 1;
            let new_x = apply_offset(px, conn.offset);
            Some(ConnectionTransition {
                new_map: target_map,
                new_x,
                new_y,
            })
        }
        Direction::Down => {
            if py != current_h - 1 {
                return None;
            }
            let conn = conns.south.as_ref()?;
            let target_map = resolve_map_id(&conn.target_map)?;
            let new_y = 0;
            let new_x = apply_offset(px, conn.offset);
            Some(ConnectionTransition {
                new_map: target_map,
                new_x,
                new_y,
            })
        }
        Direction::Left => {
            if px != 0 {
                return None;
            }
            let conn = conns.west.as_ref()?;
            let target_map = resolve_map_id(&conn.target_map)?;
            let dest_w = target_map.width() as u16 * 2;
            let new_x = dest_w - 1;
            let new_y = apply_offset(py, conn.offset);
            Some(ConnectionTransition {
                new_map: target_map,
                new_x,
                new_y,
            })
        }
        Direction::Right => {
            if px != current_w - 1 {
                return None;
            }
            let conn = conns.east.as_ref()?;
            let target_map = resolve_map_id(&conn.target_map)?;
            let new_x = 0;
            let new_y = apply_offset(py, conn.offset);
            Some(ConnectionTransition {
                new_map: target_map,
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
/// Warp coordinates and player coordinates are both in step units
/// (2 per block, matching the original game's wXCoord/wYCoord).
pub fn check_warp_at(current_map: MapId, px: u8, py: u8) -> Option<WarpTransition> {
    let map_json = get_map_json(current_map)?;
    for warp in &map_json.warps {
        if (px as u16) == warp.x as u16 && (py as u16) == warp.y as u16 {
            let is_last_map = warp.dest_map.is_none();
            let new_map = warp
                .dest_map
                .as_ref()
                .and_then(|name| resolve_map_id(name))
                .unwrap_or(current_map);
            return Some(WarpTransition {
                new_map,
                dest_warp_id: warp.dest_warp_id,
                is_last_map,
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
pub fn resolve_warp_destination(dest_map: MapId, dest_warp_id: u8) -> Option<(u8, u8)> {
    let map_json = get_map_json(dest_map)?;
    let idx = dest_warp_id as usize;
    if idx < map_json.warps.len() {
        Some((map_json.warps[idx].x, map_json.warps[idx].y))
    } else {
        None
    }
}

/// Full warp execution: check if at a warp, resolve destination coordinates.
///
/// Combines `check_warp_at` and `resolve_warp_destination` into one call.
/// For "last map" warps, the caller must supply `last_map`.
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
