use pokered_data::maps::{MapId, FIRST_INDOOR_MAP};
use pokered_data::tileset_data::get_tileset_header;
use pokered_data::tilesets::TilesetId;
use pokered_data::wild_data::{wild_data_for_map, GameVersion, MapWildData, WildEncounterTable};

use crate::battle::wild::{
    try_wild_encounter, EncounterContext, WildEncounterRandoms, WildEncounterResult,
};

/// Hardcoded water tile ID from wild_encounters.asm: `ld a, $14`
pub const WATER_TILE: u8 = 0x14;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileEncounterType {
    Grass,
    Water,
    /// Indoor/cave (>= FIRST_INDOOR_MAP, tileset != Forest) — uses grass table
    IndoorCave,
    None,
}

pub fn determine_encounter_type(
    standing_tile: u8,
    tileset: TilesetId,
    map_id: MapId,
) -> TileEncounterType {
    let header = get_tileset_header(tileset);

    if header.is_grass_tile(standing_tile) {
        return TileEncounterType::Grass;
    }

    if standing_tile == WATER_TILE {
        return TileEncounterType::Water;
    }

    // Indoor map exception: maps >= FIRST_INDOOR_MAP with tileset != Forest
    // use grass encounter rate even without standing on grass/water.
    // This enables cave encounters (Mt Moon, Rock Tunnel, etc.)
    if (map_id as u8) >= FIRST_INDOOR_MAP && tileset != TilesetId::Forest {
        return TileEncounterType::IndoorCave;
    }

    TileEncounterType::None
}

pub fn select_encounter_table(
    encounter_type: TileEncounterType,
    wild_data: &MapWildData,
) -> Option<&WildEncounterTable> {
    match encounter_type {
        TileEncounterType::Grass | TileEncounterType::IndoorCave => Some(&wild_data.grass),
        TileEncounterType::Water => Some(&wild_data.water),
        TileEncounterType::None => Option::None,
    }
}

pub fn should_check_encounter(
    on_warp_tile: bool,
    npc_script_active: bool,
    encounter_cooldown: u8,
) -> bool {
    !on_warp_tile && !npc_script_active && encounter_cooldown == 0
}

pub fn check_wild_encounter(
    map_id: MapId,
    tileset: TilesetId,
    standing_tile: u8,
    version: GameVersion,
    randoms: &WildEncounterRandoms,
    context: &EncounterContext,
    on_warp_tile: bool,
    npc_script_active: bool,
    encounter_cooldown: u8,
) -> WildEncounterResult {
    if !should_check_encounter(on_warp_tile, npc_script_active, encounter_cooldown) {
        return WildEncounterResult::NoEncounter;
    }

    let wild_data = match wild_data_for_map(map_id, version) {
        Some(data) => data,
        Option::None => return WildEncounterResult::NoEncounter,
    };

    let encounter_type = determine_encounter_type(standing_tile, tileset, map_id);
    let table = select_encounter_table(encounter_type, &wild_data);

    try_wild_encounter(table, randoms, context)
}
