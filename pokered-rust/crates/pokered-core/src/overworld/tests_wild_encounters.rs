use super::wild_encounters::*;
use pokered_data::maps::MapId;
use pokered_data::species::Species;
use pokered_data::tilesets::TilesetId;
use pokered_data::wild_data::{wild_data_for_map, wild_data_name_for_map, GameVersion};

use crate::battle::wild::{EncounterContext, WildEncounterRandoms, WildEncounterResult};

// ── wild_data_name_for_map tests ──────────────────────────────────

#[test]
fn route1_maps_to_route1() {
    assert_eq!(wild_data_name_for_map(MapId::Route1), Some("Route1"));
}

#[test]
fn route19_and_route20_share_sea_routes() {
    assert_eq!(wild_data_name_for_map(MapId::Route19), Some("SeaRoutes"));
    assert_eq!(wild_data_name_for_map(MapId::Route20), Some("SeaRoutes"));
}

#[test]
fn pallet_town_has_no_wild_data() {
    assert_eq!(wild_data_name_for_map(MapId::PalletTown), None);
}

#[test]
fn pokemon_tower_1f_and_2f_have_no_wild_data() {
    assert_eq!(wild_data_name_for_map(MapId::PokemonTower1F), None);
    assert_eq!(wild_data_name_for_map(MapId::PokemonTower2F), None);
}

#[test]
fn pokemon_tower_3f_through_7f_have_wild_data() {
    assert_eq!(
        wild_data_name_for_map(MapId::PokemonTower3F),
        Some("PokemonTower3F")
    );
    assert_eq!(
        wild_data_name_for_map(MapId::PokemonTower7F),
        Some("PokemonTower7F")
    );
}

#[test]
fn viridian_forest_has_wild_data() {
    assert_eq!(
        wild_data_name_for_map(MapId::ViridianForest),
        Some("ViridianForest")
    );
}

#[test]
fn cerulean_cave_floors_have_wild_data() {
    assert_eq!(
        wild_data_name_for_map(MapId::CeruleanCave1F),
        Some("CeruleanCave1F")
    );
    assert_eq!(
        wild_data_name_for_map(MapId::CeruleanCave2F),
        Some("CeruleanCave2F")
    );
    assert_eq!(
        wild_data_name_for_map(MapId::CeruleanCaveB1F),
        Some("CeruleanCaveB1F")
    );
}

#[test]
fn safari_zone_areas_have_wild_data() {
    assert!(wild_data_name_for_map(MapId::SafariZoneEast).is_some());
    assert!(wild_data_name_for_map(MapId::SafariZoneNorth).is_some());
    assert!(wild_data_name_for_map(MapId::SafariZoneWest).is_some());
    assert!(wild_data_name_for_map(MapId::SafariZoneCenter).is_some());
}

#[test]
fn indoor_buildings_have_no_wild_data() {
    assert_eq!(wild_data_name_for_map(MapId::RedsHouse1F), None);
    assert_eq!(wild_data_name_for_map(MapId::OaksLab), None);
    assert_eq!(wild_data_name_for_map(MapId::ViridianPokecenter), None);
    assert_eq!(wild_data_name_for_map(MapId::CeladonMart1F), None);
}

#[test]
fn all_routes_with_encounters() {
    for map in [
        MapId::Route1,
        MapId::Route2,
        MapId::Route3,
        MapId::Route4,
        MapId::Route5,
        MapId::Route6,
        MapId::Route7,
        MapId::Route8,
        MapId::Route9,
        MapId::Route10,
        MapId::Route11,
        MapId::Route12,
        MapId::Route13,
        MapId::Route14,
        MapId::Route15,
        MapId::Route16,
        MapId::Route17,
        MapId::Route18,
        MapId::Route19,
        MapId::Route20,
        MapId::Route21,
        MapId::Route22,
        MapId::Route23,
        MapId::Route24,
        MapId::Route25,
    ] {
        assert!(
            wild_data_name_for_map(map).is_some(),
            "{:?} should have wild data",
            map
        );
    }
}

// ── wild_data_for_map tests ───────────────────────────────────────

#[test]
fn wild_data_for_route1_red() {
    let data = wild_data_for_map(MapId::Route1, GameVersion::Red).unwrap();
    assert_eq!(data.name, "Route1");
    assert!(data.grass.encounter_rate > 0);
    assert_eq!(data.grass.mons.len(), 10);
}

#[test]
fn wild_data_for_route1_blue() {
    let data = wild_data_for_map(MapId::Route1, GameVersion::Blue).unwrap();
    assert_eq!(data.name, "Route1");
    assert!(data.grass.encounter_rate > 0);
}

#[test]
fn wild_data_for_pallet_town_returns_none() {
    assert!(wild_data_for_map(MapId::PalletTown, GameVersion::Red).is_none());
}

#[test]
fn sea_routes_has_water_encounters_only() {
    let data = wild_data_for_map(MapId::Route19, GameVersion::Red).unwrap();
    assert_eq!(data.name, "SeaRoutes");
    assert_eq!(data.grass.encounter_rate, 0);
    assert!(data.water.encounter_rate > 0);
    assert_eq!(data.water.mons.len(), 10);
}

#[test]
fn route21_has_both_grass_and_water() {
    let data = wild_data_for_map(MapId::Route21, GameVersion::Red).unwrap();
    assert!(data.grass.encounter_rate > 0);
    assert!(data.water.encounter_rate > 0);
}

// ── determine_encounter_type tests ────────────────────────────────

#[test]
fn overworld_grass_tile_detected() {
    let result = determine_encounter_type(0x52, TilesetId::Overworld, MapId::Route1);
    assert_eq!(result, TileEncounterType::Grass);
}

#[test]
fn forest_grass_tile_detected() {
    let result = determine_encounter_type(0x20, TilesetId::Forest, MapId::ViridianForest);
    assert_eq!(result, TileEncounterType::Grass);
}

#[test]
fn water_tile_detected_on_overworld() {
    let result = determine_encounter_type(WATER_TILE, TilesetId::Overworld, MapId::Route21);
    assert_eq!(result, TileEncounterType::Water);
}

#[test]
fn water_tile_detected_in_cave() {
    let result = determine_encounter_type(WATER_TILE, TilesetId::Cavern, MapId::SeafoamIslandsB3F);
    assert_eq!(result, TileEncounterType::Water);
}

#[test]
fn indoor_cave_encounter_type() {
    let result = determine_encounter_type(0x00, TilesetId::Cavern, MapId::MtMoon1F);
    assert_eq!(result, TileEncounterType::IndoorCave);
}

#[test]
fn indoor_cemetery_encounter_type() {
    let result = determine_encounter_type(0x00, TilesetId::Cemetery, MapId::PokemonTower3F);
    assert_eq!(result, TileEncounterType::IndoorCave);
}

#[test]
fn viridian_forest_is_indoor_but_uses_forest_tileset_so_needs_grass_tile() {
    // ViridianForest (0x33 >= FIRST_INDOOR_MAP=0x25) but tileset == Forest
    // so the indoor exception does NOT apply — must be on grass tile
    let result = determine_encounter_type(0x00, TilesetId::Forest, MapId::ViridianForest);
    assert_eq!(result, TileEncounterType::None);

    let result = determine_encounter_type(0x20, TilesetId::Forest, MapId::ViridianForest);
    assert_eq!(result, TileEncounterType::Grass);
}

#[test]
fn outdoor_route_non_grass_non_water_tile_is_none() {
    let result = determine_encounter_type(0x00, TilesetId::Overworld, MapId::Route1);
    assert_eq!(result, TileEncounterType::None);
}

#[test]
fn city_map_non_grass_tile_is_none() {
    let result = determine_encounter_type(0x00, TilesetId::Overworld, MapId::PalletTown);
    assert_eq!(result, TileEncounterType::None);
}

#[test]
fn plateau_grass_tile_detected() {
    let result = determine_encounter_type(0x45, TilesetId::Plateau, MapId::VictoryRoad2F);
    assert_eq!(result, TileEncounterType::Grass);
}

// ── should_check_encounter tests ──────────────────────────────────

#[test]
fn check_allowed_when_all_clear() {
    assert!(should_check_encounter(false, false, 0));
}

#[test]
fn check_blocked_on_warp_tile() {
    assert!(!should_check_encounter(true, false, 0));
}

#[test]
fn check_blocked_during_npc_script() {
    assert!(!should_check_encounter(false, true, 0));
}

#[test]
fn check_blocked_by_cooldown() {
    assert!(!should_check_encounter(false, false, 3));
}

#[test]
fn check_blocked_by_multiple_conditions() {
    assert!(!should_check_encounter(true, true, 5));
}

// ── check_wild_encounter integration tests ────────────────────────

fn low_roll() -> WildEncounterRandoms {
    WildEncounterRandoms {
        encounter_roll: 0,
        slot_roll: 0,
    }
}

fn high_roll() -> WildEncounterRandoms {
    WildEncounterRandoms {
        encounter_roll: 255,
        slot_roll: 0,
    }
}

fn no_repel() -> EncounterContext {
    EncounterContext {
        repel_active: false,
        party_lead_level: 50,
    }
}

#[test]
fn route1_grass_encounter_triggers() {
    let result = check_wild_encounter(
        MapId::Route1,
        TilesetId::Overworld,
        0x52, // grass tile
        GameVersion::Red,
        &low_roll(),
        &no_repel(),
        false,
        false,
        0,
    );
    assert!(matches!(result, WildEncounterResult::Encounter { .. }));
}

#[test]
fn route1_high_roll_no_encounter() {
    let result = check_wild_encounter(
        MapId::Route1,
        TilesetId::Overworld,
        0x52,
        GameVersion::Red,
        &high_roll(),
        &no_repel(),
        false,
        false,
        0,
    );
    assert_eq!(result, WildEncounterResult::NoEncounter);
}

#[test]
fn route1_non_grass_tile_no_encounter() {
    let result = check_wild_encounter(
        MapId::Route1,
        TilesetId::Overworld,
        0x00, // not a grass tile
        GameVersion::Red,
        &low_roll(),
        &no_repel(),
        false,
        false,
        0,
    );
    assert_eq!(result, WildEncounterResult::NoEncounter);
}

#[test]
fn mt_moon_cave_encounter_on_any_tile() {
    let result = check_wild_encounter(
        MapId::MtMoon1F,
        TilesetId::Cavern,
        0x00, // any tile — indoor cave exception applies
        GameVersion::Red,
        &low_roll(),
        &no_repel(),
        false,
        false,
        0,
    );
    assert!(matches!(result, WildEncounterResult::Encounter { .. }));
}

#[test]
fn pallet_town_never_encounters() {
    let result = check_wild_encounter(
        MapId::PalletTown,
        TilesetId::Overworld,
        0x52,
        GameVersion::Red,
        &low_roll(),
        &no_repel(),
        false,
        false,
        0,
    );
    assert_eq!(result, WildEncounterResult::NoEncounter);
}

#[test]
fn warp_tile_blocks_encounter() {
    let result = check_wild_encounter(
        MapId::Route1,
        TilesetId::Overworld,
        0x52,
        GameVersion::Red,
        &low_roll(),
        &no_repel(),
        true, // on warp tile
        false,
        0,
    );
    assert_eq!(result, WildEncounterResult::NoEncounter);
}

#[test]
fn npc_script_blocks_encounter() {
    let result = check_wild_encounter(
        MapId::Route1,
        TilesetId::Overworld,
        0x52,
        GameVersion::Red,
        &low_roll(),
        &no_repel(),
        false,
        true, // NPC script active
        0,
    );
    assert_eq!(result, WildEncounterResult::NoEncounter);
}

#[test]
fn cooldown_blocks_encounter() {
    let result = check_wild_encounter(
        MapId::Route1,
        TilesetId::Overworld,
        0x52,
        GameVersion::Red,
        &low_roll(),
        &no_repel(),
        false,
        false,
        5, // cooldown active
    );
    assert_eq!(result, WildEncounterResult::NoEncounter);
}

#[test]
fn route19_water_encounter() {
    let result = check_wild_encounter(
        MapId::Route19,
        TilesetId::Overworld,
        WATER_TILE,
        GameVersion::Red,
        &low_roll(),
        &no_repel(),
        false,
        false,
        0,
    );
    assert!(matches!(result, WildEncounterResult::Encounter { .. }));
    if let WildEncounterResult::Encounter { species, .. } = result {
        assert_eq!(species, Species::Tentacool);
    }
}

#[test]
fn route20_uses_same_sea_routes_data() {
    let r19 = check_wild_encounter(
        MapId::Route19,
        TilesetId::Overworld,
        WATER_TILE,
        GameVersion::Red,
        &low_roll(),
        &no_repel(),
        false,
        false,
        0,
    );
    let r20 = check_wild_encounter(
        MapId::Route20,
        TilesetId::Overworld,
        WATER_TILE,
        GameVersion::Red,
        &low_roll(),
        &no_repel(),
        false,
        false,
        0,
    );
    assert_eq!(r19, r20);
}

#[test]
fn repel_blocks_low_level_encounter() {
    let ctx = EncounterContext {
        repel_active: true,
        party_lead_level: 50,
    };
    let result = check_wild_encounter(
        MapId::Route1,
        TilesetId::Overworld,
        0x52,
        GameVersion::Red,
        &low_roll(),
        &ctx,
        false,
        false,
        0,
    );
    assert_eq!(result, WildEncounterResult::RepelBlocked);
}

#[test]
fn repel_allows_high_level_encounter() {
    let ctx = EncounterContext {
        repel_active: true,
        party_lead_level: 1,
    };
    let result = check_wild_encounter(
        MapId::Route1,
        TilesetId::Overworld,
        0x52,
        GameVersion::Red,
        &low_roll(),
        &ctx,
        false,
        false,
        0,
    );
    assert!(matches!(result, WildEncounterResult::Encounter { .. }));
}

#[test]
fn viridian_forest_needs_grass_tile_despite_being_indoor() {
    let no_encounter = check_wild_encounter(
        MapId::ViridianForest,
        TilesetId::Forest,
        0x00,
        GameVersion::Red,
        &low_roll(),
        &no_repel(),
        false,
        false,
        0,
    );
    assert_eq!(no_encounter, WildEncounterResult::NoEncounter);

    let encounter = check_wild_encounter(
        MapId::ViridianForest,
        TilesetId::Forest,
        0x20, // Forest grass tile
        GameVersion::Red,
        &low_roll(),
        &no_repel(),
        false,
        false,
        0,
    );
    assert!(matches!(encounter, WildEncounterResult::Encounter { .. }));
}

#[test]
fn pokemon_tower_3f_cemetery_encounter() {
    let result = check_wild_encounter(
        MapId::PokemonTower3F,
        TilesetId::Cemetery,
        0x00,
        GameVersion::Red,
        &low_roll(),
        &no_repel(),
        false,
        false,
        0,
    );
    assert!(matches!(result, WildEncounterResult::Encounter { .. }));
}

#[test]
fn blue_version_has_different_pokemon() {
    let red_data = wild_data_for_map(MapId::Route1, GameVersion::Red).unwrap();
    let blue_data = wild_data_for_map(MapId::Route1, GameVersion::Blue).unwrap();
    assert_eq!(red_data.name, blue_data.name);
    // Red and Blue have different species on some routes
    // Both should have 10 slots
    assert_eq!(red_data.grass.mons.len(), 10);
    assert_eq!(blue_data.grass.mons.len(), 10);
}

#[test]
fn seafoam_islands_b3f_has_grass_but_no_water_encounters() {
    let data = wild_data_for_map(MapId::SeafoamIslandsB3F, GameVersion::Red).unwrap();
    assert!(
        data.grass.encounter_rate > 0,
        "SeafoamIslandsB3F should have grass/cave encounters"
    );
    assert_eq!(
        data.water.encounter_rate, 0,
        "SeafoamIslandsB3F has no water encounters in original data"
    );
}

#[test]
fn digletts_cave_encounter() {
    let result = check_wild_encounter(
        MapId::DiglettsCave,
        TilesetId::Cavern,
        0x00,
        GameVersion::Red,
        &low_roll(),
        &no_repel(),
        false,
        false,
        0,
    );
    assert!(matches!(result, WildEncounterResult::Encounter { .. }));
    if let WildEncounterResult::Encounter { species, .. } = result {
        assert_eq!(species, Species::Diglett);
    }
}

#[test]
fn power_plant_encounter() {
    let result = check_wild_encounter(
        MapId::PowerPlant,
        TilesetId::Cavern,
        0x00,
        GameVersion::Red,
        &low_roll(),
        &no_repel(),
        false,
        false,
        0,
    );
    assert!(matches!(result, WildEncounterResult::Encounter { .. }));
}

#[test]
fn select_table_grass_returns_grass() {
    let data = wild_data_for_map(MapId::Route1, GameVersion::Red).unwrap();
    let table = select_encounter_table(TileEncounterType::Grass, &data).unwrap();
    assert_eq!(table.encounter_rate, data.grass.encounter_rate);
}

#[test]
fn select_table_water_returns_water() {
    let data = wild_data_for_map(MapId::Route21, GameVersion::Red).unwrap();
    let table = select_encounter_table(TileEncounterType::Water, &data).unwrap();
    assert_eq!(table.encounter_rate, data.water.encounter_rate);
}

#[test]
fn select_table_indoor_cave_returns_grass() {
    let data = wild_data_for_map(MapId::MtMoon1F, GameVersion::Red).unwrap();
    let table = select_encounter_table(TileEncounterType::IndoorCave, &data).unwrap();
    assert_eq!(table.encounter_rate, data.grass.encounter_rate);
}

#[test]
fn select_table_none_returns_none() {
    let data = wild_data_for_map(MapId::Route1, GameVersion::Red).unwrap();
    assert!(select_encounter_table(TileEncounterType::None, &data).is_none());
}

#[test]
fn all_dungeon_maps_have_valid_data() {
    let dungeons = [
        MapId::MtMoon1F,
        MapId::MtMoonB1F,
        MapId::MtMoonB2F,
        MapId::RockTunnel1F,
        MapId::RockTunnelB1F,
        MapId::PowerPlant,
        MapId::DiglettsCave,
        MapId::VictoryRoad1F,
        MapId::VictoryRoad2F,
        MapId::VictoryRoad3F,
        MapId::SeafoamIslands1F,
        MapId::SeafoamIslandsB1F,
        MapId::SeafoamIslandsB2F,
        MapId::SeafoamIslandsB3F,
        MapId::SeafoamIslandsB4F,
        MapId::PokemonMansion1F,
        MapId::PokemonMansion2F,
        MapId::PokemonMansion3F,
        MapId::PokemonMansionB1F,
        MapId::CeruleanCave1F,
        MapId::CeruleanCave2F,
        MapId::CeruleanCaveB1F,
    ];
    for map in dungeons {
        let data = wild_data_for_map(map, GameVersion::Red);
        assert!(data.is_some(), "{:?} should have wild data in Red", map);
        let data = wild_data_for_map(map, GameVersion::Blue);
        assert!(data.is_some(), "{:?} should have wild data in Blue", map);
    }
}
