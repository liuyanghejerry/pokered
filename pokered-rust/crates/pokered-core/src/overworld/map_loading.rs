//! Map loading utilities — query static map data.
//!
//! Provides functions to look up map headers, dimensions, connections,
//! and other static data from the pokered-data tables.

use pokered_data::map_data::{
    MapHeaderData, CONN_EAST, CONN_NORTH, CONN_SOUTH, CONN_WEST, MAP_HEADER_DATA,
};
use pokered_data::map_flags;
use pokered_data::map_names::{self, MapNameId};
use pokered_data::maps::{MapId, NUM_MAPS};
use pokered_data::music::MusicId;
use pokered_data::tilesets::TilesetId;
use pokered_data::toggleable_objects::{self, ToggleEntry};

/// Get the static header data for a map (tileset, music, connection flags).
pub fn get_map_header(map: MapId) -> &'static MapHeaderData {
    &MAP_HEADER_DATA[map as usize]
}

/// Get the map dimensions in blocks (width, height).
pub fn get_map_dimensions(map: MapId) -> (u8, u8) {
    map.dimensions()
}

/// Get the map's tileset.
pub fn get_map_tileset(map: MapId) -> TilesetId {
    MAP_HEADER_DATA[map as usize].tileset
}

/// Get the map's music track.
pub fn get_map_music(map: MapId) -> MusicId {
    MAP_HEADER_DATA[map as usize].music
}

/// Check if a map has a connection in the given direction.
pub fn has_connection(map: MapId, direction: super::Direction) -> bool {
    let flags = MAP_HEADER_DATA[map as usize].connection_flags;
    match direction {
        super::Direction::Up => flags & CONN_NORTH != 0,
        super::Direction::Down => flags & CONN_SOUTH != 0,
        super::Direction::Left => flags & CONN_WEST != 0,
        super::Direction::Right => flags & CONN_EAST != 0,
    }
}

/// Get the number of connections for a map.
pub fn connection_count(map: MapId) -> u8 {
    MAP_HEADER_DATA[map as usize].connection_count()
}

/// Get the total number of map blocks (width * height).
pub fn get_map_block_count(map: MapId) -> u16 {
    let (w, h) = map.dimensions();
    w as u16 * h as u16
}

/// Check if a map ID value is valid.
pub fn is_valid_map_id(id: u8) -> bool {
    (id as usize) < NUM_MAPS
}

/// Get all maps that use a specific tileset.
pub fn maps_with_tileset(tileset: TilesetId) -> Vec<MapId> {
    (0..NUM_MAPS as u8)
        .filter_map(|i| {
            let map = MapId::from_u8(i)?;
            if MAP_HEADER_DATA[i as usize].tileset == tileset {
                Some(map)
            } else {
                None
            }
        })
        .collect()
}

/// Get all maps that play a specific music track.
pub fn maps_with_music(music: MusicId) -> Vec<MapId> {
    (0..NUM_MAPS as u8)
        .filter_map(|i| {
            let map = MapId::from_u8(i)?;
            if MAP_HEADER_DATA[i as usize].music == music {
                Some(map)
            } else {
                None
            }
        })
        .collect()
}

/// Get all outdoor maps (cities + routes) that have at least one connection.
pub fn connected_outdoor_maps() -> Vec<MapId> {
    (0..NUM_MAPS as u8)
        .filter_map(|i| {
            let map = MapId::from_u8(i)?;
            if MAP_HEADER_DATA[i as usize].connection_flags != 0 {
                Some(map)
            } else {
                None
            }
        })
        .collect()
}

// ── M4.10: Map names, flags, and toggleable objects ────────────────

/// Get the display name for a map (e.g. "PALLET TOWN").
pub fn get_map_name(map: MapId) -> &'static str {
    map_names::map_name_for_map(map)
}

/// Get the MapNameId for a map.
pub fn get_map_name_id(map: MapId) -> MapNameId {
    map_names::map_to_name_id(map)
}

/// Check if a map is a dungeon (Rock Tunnel, Seafoam, etc.)
/// Dungeon maps display the map name on entry.
pub fn is_dungeon_map(map: MapId) -> bool {
    map_flags::is_dungeon_map(map)
}

/// Check if a tile position forces biking or surfing (cycling road, etc.)
pub fn check_force_bike_surf(map: MapId, x: u8, y: u8) -> bool {
    map_flags::check_force_bike_surf(map, x, y)
}

/// Check if a map is a Safari Zone rest house.
pub fn is_safari_rest_house(map: MapId) -> bool {
    map_flags::is_safari_rest_house(map)
}

/// Get the badge bit for a gym map, if any.
pub fn badge_for_gym(map: MapId) -> Option<u8> {
    map_flags::badge_for_gym(map)
}

/// Get the toggleable objects for a map (NPCs/items that can be shown/hidden).
pub fn get_toggleable_objects(map: MapId) -> &'static [ToggleEntry] {
    toggleable_objects::toggleable_objects_for_map(map)
}
