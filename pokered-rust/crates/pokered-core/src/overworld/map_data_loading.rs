//! Map data loading utilities - creates complete MapData objects with all game data combined

use pokered_data::map_data::MAP_HEADER_DATA;
use pokered_data::map_objects::get_map_warps;
use pokered_data::maps::MapId;
use pokered_data::toggleable_objects;

use super::{
    Direction, MapConnection, MapConnections, MapData, NpcDefinition, NpcMovementType, Sign,
    WarpPoint,
};

/// Load complete map data for the given MapId by combining:
/// - Block data (terrain layout)
/// - Warp data (stairs, doors, etc.)
/// - NPC definitions
/// - Sign data
/// - Connection information
pub fn load_full_map_data(map_id: MapId) -> MapData {
    let header = &MAP_HEADER_DATA[map_id as usize];
    let (width, height) = map_id.dimensions();

    // Load block data (terrain)
    let blocks = pokered_data::map_blocks::block_data_for_map(map_id).to_vec();

    // Load warp data
    let warp_data = get_map_warps(map_id);
    let warps: Vec<WarpPoint> = warp_data
        .iter()
        .map(|warp| WarpPoint {
            x: warp.x,
            y: warp.y,
            target_map: warp.dest_map.unwrap_or(map_id),
            target_warp_id: warp.dest_warp_id,
        })
        .collect();

    // Load NPCs using toggleable objects
    let toggle_entries = toggleable_objects::toggleable_objects_for_map(map_id);
    let mut npcs = Vec::new();

    // For now, using placeholder NPCs data from the per-map data if available
    // For real implementation, should load from per_map_data for each specific map
    // Let's load placeholder based on what's available in the data
    let per_map_npcs = load_per_map_npcs(map_id);
    npcs.extend(per_map_npcs);

    // Load signs - in original data this may come from per-map data
    let mut signs = Vec::new();
    // Add placeholder signs for now
    let per_map_signs = load_per_map_signs(map_id);
    signs.extend(per_map_signs);

    // Build connections based on header flags
    let connections = build_connections(map_id);

    MapData {
        id: map_id,
        width,
        height,
        tileset: header.tileset,
        music: header.music,
        blocks,
        warps,
        npcs,
        signs,
        connections,
    }
}

fn load_per_map_npcs(map_id: MapId) -> Vec<NpcDefinition> {
    match map_id {
        MapId::RedsHouse1F => load_reds_house_1f_npcs(),
        MapId::RedsHouse2F => Vec::new(),
        _ => Vec::new(),
    }
}

fn load_reds_house_1f_npcs() -> Vec<NpcDefinition> {
    vec![NpcDefinition {
        sprite_id: 0x33,
        x: 5,
        y: 4,
        movement: NpcMovementType::Stationary,
        facing: Direction::Up,
        range: 0,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    }]
}

fn load_per_map_signs(map_id: MapId) -> Vec<Sign> {
    match map_id {
        MapId::RedsHouse1F => vec![Sign {
            x: 3,
            y: 1,
            text_id: 1,
        }],
        _ => Vec::new(),
    }
}

fn build_connections(map_id: MapId) -> MapConnections {
    let header = &MAP_HEADER_DATA[map_id as usize];

    let mut connections = MapConnections::default();

    use pokered_data::map_data::{CONN_EAST, CONN_NORTH, CONN_SOUTH, CONN_WEST};

    if header.connection_flags & CONN_NORTH != 0 {
        connections.north = get_connection_for_direction(map_id, Direction::Up);
    }

    if header.connection_flags & CONN_SOUTH != 0 {
        connections.south = get_connection_for_direction(map_id, Direction::Down);
    }

    if header.connection_flags & CONN_WEST != 0 {
        connections.west = get_connection_for_direction(map_id, Direction::Left);
    }

    if header.connection_flags & CONN_EAST != 0 {
        connections.east = get_connection_for_direction(map_id, Direction::Right);
    }

    connections
}

fn get_connection_for_direction(map_id: MapId, dir: Direction) -> Option<MapConnection> {
    let map_conns = pokered_data::map_connections::get_map_connections(map_id);

    match dir {
        Direction::Up => map_conns.north.map(|conn| MapConnection {
            direction: Direction::Up,
            target_map: conn.target_map,
            offset: conn.offset,
        }),
        Direction::Down => map_conns.south.map(|conn| MapConnection {
            direction: Direction::Down,
            target_map: conn.target_map,
            offset: conn.offset,
        }),
        Direction::Left => map_conns.west.map(|conn| MapConnection {
            direction: Direction::Left,
            target_map: conn.target_map,
            offset: conn.offset,
        }),
        Direction::Right => map_conns.east.map(|conn| MapConnection {
            direction: Direction::Right,
            target_map: conn.target_map,
            offset: conn.offset,
        }),
    }
}
