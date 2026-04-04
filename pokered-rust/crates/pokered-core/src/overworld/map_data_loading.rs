//! Map data loading utilities - creates complete MapData objects from JSON map data.
//!
//! This module bridges the JSON-based map data (from pokered-data's map_data_loader)
//! to the runtime types used by pokered-core's overworld system.

use pokered_data::map_data_loader::{get_block_data, get_map_json, resolve_map_id};
use pokered_data::map_json::{ConnectionEntryJson, ConnectionsJson, NpcJson, SignJson, WarpJson};
use pokered_data::maps::MapId;
use pokered_data::music::MusicId;
use pokered_data::tilesets::TilesetId;

use super::{
    Direction, MapConnection, MapConnections, MapData, NpcDefinition, NpcMovementType, Sign,
    WarpPoint,
};

/// Load complete map data for the given MapId by combining JSON data and block data.
///
/// Returns fully converted runtime MapData with all fields populated from the
/// per-map `map.json` file and `map.blk` binary block data.
pub fn load_full_map_data(map_id: MapId) -> MapData {
    let map_json = get_map_json(map_id).unwrap_or_else(|| {
        panic!(
            "No map.json found for {:?} — ensure map data is generated",
            map_id
        )
    });

    let (width, height) = map_id.dimensions();

    // Load block data (terrain) from .blk file
    let blocks = get_block_data(map_id).to_vec();

    // Convert warps
    let warps: Vec<WarpPoint> = map_json
        .warps
        .iter()
        .map(|w| convert_warp(w, map_id))
        .collect();

    // Convert NPCs
    let npcs: Vec<NpcDefinition> = map_json.npcs.iter().map(convert_npc).collect();

    // Convert signs
    let signs: Vec<Sign> = map_json.signs.iter().map(convert_sign).collect();

    // Convert connections
    let connections = convert_connections(&map_json.connections);

    // Resolve tileset and music from header strings
    let tileset = TilesetId::from_name(&map_json.header.tileset).unwrap_or_else(|| {
        log::warn!(
            "Unknown tileset '{}' for map {:?}, defaulting to Overworld",
            map_json.header.tileset,
            map_id
        );
        TilesetId::Overworld
    });
    let music = MusicId::from_name(&map_json.header.music).unwrap_or_else(|| {
        log::warn!(
            "Unknown music '{}' for map {:?}, defaulting to PalletTown",
            map_json.header.music,
            map_id
        );
        MusicId::PalletTown
    });

    MapData {
        id: map_id,
        width,
        height,
        tileset,
        music,
        blocks,
        warps,
        npcs,
        signs,
        connections,
    }
}

// ── JSON → Runtime conversion helpers ──────────────────────────────

fn convert_npc(npc: &NpcJson) -> NpcDefinition {
    NpcDefinition {
        sprite_id: npc.sprite_id,
        x: npc.x,
        y: npc.y,
        movement: parse_movement_type(&npc.movement),
        facing: parse_direction(&npc.facing),
        range: npc.range,
        text_id: npc.text_id,
        is_trainer: npc.is_trainer,
        trainer_class: npc
            .trainer_class
            .as_ref()
            .map(|name| parse_trainer_class(name))
            .unwrap_or(0),
        trainer_set: npc.trainer_set.unwrap_or(0),
        item_id: npc.item_id.unwrap_or(0),
    }
}

fn convert_warp(warp: &WarpJson, current_map: MapId) -> WarpPoint {
    let target_map = warp
        .dest_map
        .as_ref()
        .and_then(|name| resolve_map_id(name))
        .unwrap_or(current_map);
    WarpPoint {
        x: warp.x,
        y: warp.y,
        target_map,
        target_warp_id: warp.dest_warp_id,
    }
}

fn convert_sign(sign: &SignJson) -> Sign {
    Sign {
        x: sign.x,
        y: sign.y,
        text_id: sign.text_id,
    }
}

fn convert_connections(conns: &ConnectionsJson) -> MapConnections {
    MapConnections {
        north: conns
            .north
            .as_ref()
            .and_then(|c| convert_connection_entry(c, Direction::Up)),
        south: conns
            .south
            .as_ref()
            .and_then(|c| convert_connection_entry(c, Direction::Down)),
        west: conns
            .west
            .as_ref()
            .and_then(|c| convert_connection_entry(c, Direction::Left)),
        east: conns
            .east
            .as_ref()
            .and_then(|c| convert_connection_entry(c, Direction::Right)),
    }
}

fn convert_connection_entry(
    entry: &ConnectionEntryJson,
    direction: Direction,
) -> Option<MapConnection> {
    let target_map = resolve_map_id(&entry.target_map)?;
    Some(MapConnection {
        direction,
        target_map,
        offset: entry.offset,
    })
}

// ── String → Enum parsers ──────────────────────────────────────────

fn parse_movement_type(s: &str) -> NpcMovementType {
    match s {
        "Stationary" => NpcMovementType::Stationary,
        "Wander" => NpcMovementType::Wander,
        "FixedPath" => NpcMovementType::FixedPath,
        "FacePlayer" => NpcMovementType::FacePlayer,
        _ => {
            log::warn!(
                "Unknown NPC movement type '{}', defaulting to Stationary",
                s
            );
            NpcMovementType::Stationary
        }
    }
}

fn parse_direction(s: &str) -> Direction {
    match s {
        "Down" => Direction::Down,
        "Up" => Direction::Up,
        "Left" => Direction::Left,
        "Right" => Direction::Right,
        _ => {
            log::warn!("Unknown direction '{}', defaulting to Down", s);
            Direction::Down
        }
    }
}

/// Parse trainer class name string to numeric ID.
///
/// Matches the names generated by `generate_map_json.rs::trainer_class_name()`.
fn parse_trainer_class(name: &str) -> u8 {
    match name {
        "Nobody" => 0,
        "Youngster" => 1,
        "BugCatcher" => 2,
        "Lass" => 3,
        "Sailor" => 4,
        "JrTrainerM" => 5,
        "JrTrainerF" => 6,
        "Pokemaniac" => 7,
        "SuperNerd" => 8,
        "Hiker" => 9,
        "Biker" => 10,
        "Burglar" => 11,
        "Engineer" => 12,
        "UnusedJuggler" => 13,
        "Fisher" => 14,
        "Swimmer" => 15,
        "CueBall" => 16,
        "Gambler" => 17,
        "Beauty" => 18,
        "Psychic" | "PsychicTr" => 19,
        "Rocker" => 20,
        "Juggler" => 21,
        "Tamer" => 22,
        "BirdKeeper" => 23,
        "Blackbelt" => 24,
        "Rival1" => 25,
        "ProfOak" => 26,
        "Chief" => 27,
        "Scientist" => 28,
        "Giovanni" => 29,
        "Rocket" => 30,
        "CooltrainerM" => 31,
        "CooltrainerF" => 32,
        "Bruno" => 33,
        "Brock" => 34,
        "Misty" => 35,
        "LtSurge" => 36,
        "Erika" => 37,
        "Koga" => 38,
        "Blaine" => 39,
        "Sabrina" => 40,
        "Gentleman" => 41,
        "Rival2" => 42,
        "Rival3" => 43,
        "Lorelei" => 44,
        "Channeler" => 45,
        "Agatha" => 46,
        "Lance" => 47,
        _ => {
            log::warn!("Unknown trainer class '{}', defaulting to 0", name);
            0
        }
    }
}
