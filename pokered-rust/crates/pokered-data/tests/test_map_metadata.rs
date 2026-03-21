use pokered_data::map_flags::*;
use pokered_data::map_names::*;
use pokered_data::maps::MapId;
use pokered_data::sprite_set_data::*;
use pokered_data::toggleable_objects::*;
use pokered_data::town_map_data::*;

// ── map_names ──────────────────────────────────────────────────────

#[test]
fn all_53_map_names_present() {
    assert_eq!(MAP_NAME_STRINGS.len(), NUM_MAP_NAMES);
    assert_eq!(NUM_MAP_NAMES, 53);
}

#[test]
fn pallet_town_name() {
    assert_eq!(map_name_for_map(MapId::PalletTown), "PALLET TOWN");
}

#[test]
fn indoor_map_inherits_parent_name() {
    assert_eq!(map_name_for_map(MapId::OaksLab), "PALLET TOWN");
    assert_eq!(map_name_for_map(MapId::ViridianGym), "VIRIDIAN CITY");
    assert_eq!(map_name_for_map(MapId::PokemonTower1F), "POKéMON TOWER");
}

#[test]
fn route_names() {
    assert_eq!(map_name_for_map(MapId::Route1), "ROUTE 1");
    assert_eq!(map_name_for_map(MapId::Route25), "ROUTE 25");
}

#[test]
fn name_id_round_trip() {
    let id = map_to_name_id(MapId::CeruleanCity);
    assert_eq!(id, MapNameId::CeruleanCity);
    assert_eq!(map_name_str(id), "CERULEAN CITY");
}

// ── town_map_data ──────────────────────────────────────────────────

#[test]
fn outdoor_entries_count() {
    assert_eq!(OUTDOOR_TOWN_MAP_ENTRIES.len(), NUM_OUTDOOR_ENTRIES);
    assert_eq!(NUM_OUTDOOR_ENTRIES, 37);
}

#[test]
fn indoor_entries_count() {
    assert_eq!(INDOOR_TOWN_MAP_ENTRIES.len(), NUM_INDOOR_ENTRIES);
    assert_eq!(NUM_INDOOR_ENTRIES, 60);
}

#[test]
fn town_map_order_count() {
    assert_eq!(TOWN_MAP_ORDER.len(), NUM_TOWN_MAP_ORDER);
    assert_eq!(NUM_TOWN_MAP_ORDER, 47);
}

#[test]
fn pallet_town_in_outdoor_entries() {
    let entry = OUTDOOR_TOWN_MAP_ENTRIES
        .iter()
        .find(|e| e.name == MapNameId::PalletTown);
    assert!(entry.is_some(), "Pallet Town must be in outdoor entries");
}

// ── sprite_set_data ────────────────────────────────────────────────

#[test]
fn sprite_sets_count() {
    assert_eq!(SPRITE_SETS.len(), NUM_SPRITE_SETS);
    assert_eq!(NUM_SPRITE_SETS, 10);
}

#[test]
fn each_sprite_set_has_11_entries() {
    for (i, set) in SPRITE_SETS.iter().enumerate() {
        assert_eq!(set.len(), SPRITE_SET_LENGTH, "set {i} wrong length");
    }
}

#[test]
fn map_sprite_sets_count() {
    assert_eq!(MAP_SPRITE_SETS.len(), NUM_OUTDOOR_MAPS);
    assert_eq!(NUM_OUTDOOR_MAPS, 37);
}

#[test]
fn split_map_sprite_sets_count() {
    assert_eq!(SPLIT_MAP_SPRITE_SETS.len(), NUM_SPLIT_SETS);
    assert_eq!(NUM_SPLIT_SETS, 12);
}

// ── map_flags ──────────────────────────────────────────────────────

#[test]
fn viridian_forest_is_dungeon() {
    assert!(is_dungeon_map(MapId::ViridianForest));
}

#[test]
fn pallet_town_not_dungeon() {
    assert!(!is_dungeon_map(MapId::PalletTown));
}

#[test]
fn rock_tunnel_is_dungeon() {
    assert!(is_dungeon_map(MapId::RockTunnel1F));
    assert!(is_dungeon_map(MapId::RockTunnelB1F));
}

#[test]
fn mt_moon_is_dungeon() {
    assert!(is_dungeon_map(MapId::MtMoon1F));
    assert!(is_dungeon_map(MapId::MtMoonB2F));
}

#[test]
fn pewter_gym_badge() {
    assert_eq!(badge_for_gym(MapId::PewterGym), Some(1 << BIT_BOULDERBADGE));
}

#[test]
fn non_gym_no_badge() {
    assert_eq!(badge_for_gym(MapId::PalletTown), None);
}

#[test]
fn all_8_badges_mapped() {
    assert_eq!(BADGE_MAP_FLAGS.len(), 8);
    let bits: Vec<u8> = BADGE_MAP_FLAGS.iter().map(|e| e.badge_bit).collect();
    for i in 0..8u8 {
        assert!(bits.contains(&i), "missing badge bit {i}");
    }
}

#[test]
fn safari_rest_houses() {
    assert!(is_safari_rest_house(MapId::SafariZoneWestRestHouse));
    assert!(is_safari_rest_house(MapId::SafariZoneEastRestHouse));
    assert!(is_safari_rest_house(MapId::SafariZoneNorthRestHouse));
    assert!(!is_safari_rest_house(MapId::PalletTown));
}

// ── toggleable_objects ─────────────────────────────────────────────

#[test]
fn total_228_toggleable_objects() {
    assert_eq!(NUM_TOGGLEABLE_OBJECTS, 228);
}

#[test]
fn pallet_town_toggle_oak() {
    let t = toggleable_objects_for_map(MapId::PalletTown);
    assert_eq!(t.len(), 1);
    assert_eq!(t[0].object_id, 1);
    assert_eq!(t[0].state, ToggleState::Off);
}

#[test]
fn viridian_city_toggle_old_man() {
    let t = toggleable_objects_for_map(MapId::ViridianCity);
    assert_eq!(t.len(), 2);
    assert_eq!(t[0].object_id, 5);
    assert_eq!(t[0].state, ToggleState::On);
    assert_eq!(t[1].object_id, 7);
    assert_eq!(t[1].state, ToggleState::Off);
}

#[test]
fn cerulean_city_toggle_5_entries() {
    let t = toggleable_objects_for_map(MapId::CeruleanCity);
    assert_eq!(t.len(), 5);
    assert_eq!(t[0].object_id, 1);
    assert_eq!(t[2].object_id, 6);
    assert_eq!(t[3].object_id, 10);
    assert_eq!(t[4].object_id, 11);
}

#[test]
fn saffron_city_15_toggles() {
    let t = toggleable_objects_for_map(MapId::SaffronCity);
    assert_eq!(t.len(), 15);
    assert_eq!(t[0].object_id, 1);
    assert_eq!(t[14].object_id, 15);
}

#[test]
fn power_plant_14_toggles() {
    let t = toggleable_objects_for_map(MapId::PowerPlant);
    assert_eq!(t.len(), 14);
    for (i, entry) in t.iter().enumerate() {
        assert_eq!(entry.object_id, (i + 1) as u8);
        assert_eq!(entry.state, ToggleState::On);
    }
}

#[test]
fn victory_road_2f_non_contiguous_ids() {
    let t = toggleable_objects_for_map(MapId::VictoryRoad2F);
    assert_eq!(t.len(), 6);
    let ids: Vec<u8> = t.iter().map(|e| e.object_id).collect();
    assert_eq!(ids, vec![6, 7, 8, 9, 10, 13]);
}

#[test]
fn seafoam_b3f_mixed_states() {
    let t = toggleable_objects_for_map(MapId::SeafoamIslandsB3F);
    assert_eq!(t.len(), 4);
    assert_eq!(t[0].object_id, 2);
    assert_eq!(t[0].state, ToggleState::On);
    assert_eq!(t[2].object_id, 5);
    assert_eq!(t[2].state, ToggleState::Off);
}

#[test]
fn map_without_toggles_returns_empty() {
    let t = toggleable_objects_for_map(MapId::Route1);
    assert!(t.is_empty());
}

#[test]
fn oaks_lab_8_toggles() {
    let t = toggleable_objects_for_map(MapId::OaksLab);
    assert_eq!(t.len(), 8);
    assert_eq!(t[4].object_id, 5);
    assert_eq!(t[4].state, ToggleState::Off);
    assert_eq!(t[7].object_id, 8);
    assert_eq!(t[7].state, ToggleState::Off);
}

#[test]
fn ss_anne_rooms_high_object_ids() {
    let t = toggleable_objects_for_map(MapId::SSAnne1FRooms);
    assert_eq!(t.len(), 1);
    assert_eq!(t[0].object_id, 10);

    let t2 = toggleable_objects_for_map(MapId::SSAnneB1FRooms);
    assert_eq!(t2.len(), 3);
    assert_eq!(t2[0].object_id, 9);
    assert_eq!(t2[1].object_id, 10);
    assert_eq!(t2[2].object_id, 11);
}

#[test]
fn sum_all_toggle_entries_equals_228() {
    use pokered_data::maps::NUM_MAPS;
    let mut total = 0usize;
    for i in 0..NUM_MAPS {
        if let Some(map) = MapId::from_u8(i as u8) {
            total += toggleable_objects_for_map(map).len();
        }
    }
    assert_eq!(total, NUM_TOGGLEABLE_OBJECTS);
}

#[test]
fn unused_map_f4_has_toggle() {
    let t = toggleable_objects_for_map(MapId::UnusedMapF4);
    assert_eq!(t.len(), 1);
    assert_eq!(t[0].object_id, 2);
    assert_eq!(t[0].state, ToggleState::On);
}

#[test]
fn pokemon_mansion_b1f_gap_in_ids() {
    let t = toggleable_objects_for_map(MapId::PokemonMansionB1F);
    assert_eq!(t.len(), 5);
    let ids: Vec<u8> = t.iter().map(|e| e.object_id).collect();
    assert_eq!(ids, vec![3, 4, 5, 6, 8]);
}
