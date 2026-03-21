//! Tests for the overworld module — map loading, dimensions, connections, header data.

use super::map_loading::*;
use super::*;
use pokered_data::maps::MapId;
use pokered_data::music::MusicId;
use pokered_data::tilesets::TilesetId;

// ── Map Header Data Tests ──────────────────────────────────────────

#[test]
fn test_pallet_town_header() {
    let h = get_map_header(MapId::PalletTown);
    assert_eq!(h.tileset, TilesetId::Overworld);
    assert_eq!(h.music, MusicId::PalletTown);
    assert!(h.has_north());
    assert!(h.has_south());
    assert!(!h.has_west());
    assert!(!h.has_east());
    assert_eq!(h.connection_count(), 2);
}

#[test]
fn test_viridian_city_header() {
    let h = get_map_header(MapId::ViridianCity);
    assert_eq!(h.tileset, TilesetId::Overworld);
    assert_eq!(h.music, MusicId::Cities1);
    assert!(h.has_north());
    assert!(h.has_south());
    assert!(h.has_west());
    assert!(!h.has_east());
    assert_eq!(h.connection_count(), 3);
}

#[test]
fn test_cerulean_city_all_connections() {
    let h = get_map_header(MapId::CeruleanCity);
    assert_eq!(h.connection_flags, 0x0F); // all four directions
    assert_eq!(h.connection_count(), 4);
}

#[test]
fn test_saffron_city_all_connections() {
    let h = get_map_header(MapId::SaffronCity);
    assert_eq!(h.connection_flags, 0x0F);
    assert_eq!(h.connection_count(), 4);
}

#[test]
fn test_indigo_plateau_south_only() {
    let h = get_map_header(MapId::IndigoPlateau);
    assert_eq!(h.tileset, TilesetId::Plateau);
    assert_eq!(h.music, MusicId::IndigoPlateau);
    assert!(!h.has_north());
    assert!(h.has_south());
    assert!(!h.has_west());
    assert!(!h.has_east());
    assert_eq!(h.connection_count(), 1);
}

#[test]
fn test_indoor_map_no_connections() {
    let h = get_map_header(MapId::RedsHouse1F);
    assert_eq!(h.tileset, TilesetId::RedsHouse1);
    assert_eq!(h.music, MusicId::PalletTown);
    assert_eq!(h.connection_flags, 0);
    assert_eq!(h.connection_count(), 0);
}

#[test]
fn test_oaks_lab_header() {
    let h = get_map_header(MapId::OaksLab);
    assert_eq!(h.tileset, TilesetId::Dojo);
    assert_eq!(h.music, MusicId::OaksLab);
    assert_eq!(h.connection_flags, 0);
}

#[test]
fn test_viridian_forest_header() {
    let h = get_map_header(MapId::ViridianForest);
    assert_eq!(h.tileset, TilesetId::Forest);
    assert_eq!(h.music, MusicId::Dungeon2);
    assert_eq!(h.connection_flags, 0);
}

#[test]
fn test_mt_moon_cavern() {
    let h = get_map_header(MapId::MtMoon1F);
    assert_eq!(h.tileset, TilesetId::Cavern);
    assert_eq!(h.music, MusicId::Dungeon3);
}

#[test]
fn test_game_corner_header() {
    let h = get_map_header(MapId::GameCorner);
    assert_eq!(h.tileset, TilesetId::Lobby);
    assert_eq!(h.music, MusicId::GameCorner);
}

#[test]
fn test_pokemon_tower_header() {
    let h = get_map_header(MapId::PokemonTower1F);
    assert_eq!(h.tileset, TilesetId::Cemetery);
    assert_eq!(h.music, MusicId::PokemonTower);
}

#[test]
fn test_agathas_room_header() {
    let h = get_map_header(MapId::AgathasRoom);
    assert_eq!(h.tileset, TilesetId::Cemetery);
    assert_eq!(h.music, MusicId::PokemonTower);
}

// ── Dimension Tests ────────────────────────────────────────────────

#[test]
fn test_pallet_town_dimensions() {
    assert_eq!(get_map_dimensions(MapId::PalletTown), (10, 9));
}

#[test]
fn test_celadon_city_dimensions() {
    assert_eq!(get_map_dimensions(MapId::CeladonCity), (25, 18));
}

#[test]
fn test_route17_dimensions() {
    // Route 17 (cycling road) is the tallest route
    assert_eq!(get_map_dimensions(MapId::Route17), (10, 72));
}

#[test]
fn test_reds_house_1f_dimensions() {
    assert_eq!(get_map_dimensions(MapId::RedsHouse1F), (4, 4));
}

#[test]
fn test_map_block_count() {
    assert_eq!(get_map_block_count(MapId::PalletTown), 90); // 10 * 9
    assert_eq!(get_map_block_count(MapId::CeladonCity), 450); // 25 * 18
    assert_eq!(get_map_block_count(MapId::RedsHouse1F), 16); // 4 * 4
}

// ── Map Loading Functions Tests ────────────────────────────────────

#[test]
fn test_get_map_tileset() {
    assert_eq!(get_map_tileset(MapId::PalletTown), TilesetId::Overworld);
    assert_eq!(get_map_tileset(MapId::PewterGym), TilesetId::Gym);
    assert_eq!(get_map_tileset(MapId::VermilionDock), TilesetId::ShipPort);
    assert_eq!(get_map_tileset(MapId::SilphCo1F), TilesetId::Facility);
}

#[test]
fn test_get_map_music() {
    assert_eq!(get_map_music(MapId::PalletTown), MusicId::PalletTown);
    assert_eq!(get_map_music(MapId::LavenderTown), MusicId::Lavender);
    assert_eq!(get_map_music(MapId::VermilionGym), MusicId::Gym);
    assert_eq!(get_map_music(MapId::SSAnne1F), MusicId::SSAnne);
}

#[test]
fn test_has_connection() {
    assert!(has_connection(MapId::PalletTown, Direction::Up));
    assert!(has_connection(MapId::PalletTown, Direction::Down));
    assert!(!has_connection(MapId::PalletTown, Direction::Left));
    assert!(!has_connection(MapId::PalletTown, Direction::Right));

    assert!(!has_connection(MapId::RedsHouse1F, Direction::Up));
    assert!(!has_connection(MapId::RedsHouse1F, Direction::Down));
}

#[test]
fn test_connection_count() {
    assert_eq!(connection_count(MapId::PalletTown), 2);
    assert_eq!(connection_count(MapId::CeruleanCity), 4);
    assert_eq!(connection_count(MapId::IndigoPlateau), 1);
    assert_eq!(connection_count(MapId::RedsHouse1F), 0);
}

#[test]
fn test_is_valid_map_id() {
    assert!(is_valid_map_id(0x00)); // PalletTown
    assert!(is_valid_map_id(0xF7)); // AgathasRoom (last)
    assert!(!is_valid_map_id(0xF8)); // Out of range
    assert!(!is_valid_map_id(0xFF));
}

// ── Query Tests ────────────────────────────────────────────────────

#[test]
fn test_maps_with_tileset_overworld() {
    let maps = maps_with_tileset(TilesetId::Overworld);
    // All cities and routes use Overworld, plus some unused/copy maps
    assert!(maps.contains(&MapId::PalletTown));
    assert!(maps.contains(&MapId::Route1));
    assert!(maps.contains(&MapId::SaffronCity));
    // Indoor maps should NOT be here (except unused/copy maps that default to Overworld)
    assert!(!maps.contains(&MapId::OaksLab)); // Dojo tileset
    assert!(!maps.contains(&MapId::PewterGym)); // Gym tileset
}

#[test]
fn test_maps_with_tileset_cemetery() {
    let maps = maps_with_tileset(TilesetId::Cemetery);
    // Pokemon Tower floors + Agatha's Room
    assert!(maps.contains(&MapId::PokemonTower1F));
    assert!(maps.contains(&MapId::PokemonTower7F));
    assert!(maps.contains(&MapId::AgathasRoom));
    assert_eq!(maps.len(), 8); // 7 tower floors + Agatha
}

#[test]
fn test_maps_with_music_pokecenter() {
    let maps = maps_with_music(MusicId::Pokecenter);
    // All pokecenters + marts use Pokecenter music
    assert!(maps.contains(&MapId::ViridianPokecenter));
    assert!(maps.contains(&MapId::PewterPokecenter));
    assert!(maps.contains(&MapId::ViridianMart));
    assert!(maps.len() > 10); // Many maps use pokecenter music
}

#[test]
fn test_connected_outdoor_maps() {
    let maps = connected_outdoor_maps();
    // All cities and routes should have connections
    assert!(maps.contains(&MapId::PalletTown));
    assert!(maps.contains(&MapId::Route1));
    // Indoor maps should not be here
    assert!(!maps.contains(&MapId::RedsHouse1F));
    assert!(!maps.contains(&MapId::OaksLab));
}

// ── State/Type Tests ───────────────────────────────────────────────

#[test]
fn test_overworld_state_new() {
    let state = OverworldState::new(MapId::PalletTown);
    assert_eq!(state.current_map, MapId::PalletTown);
    assert_eq!(state.player.facing, Direction::Down);
    assert_eq!(state.player.transport, TransportMode::Walking);
    assert_eq!(state.player.movement_state, MovementState::Idle);
    assert_eq!(state.walk_counter, 0);
    assert_eq!(state.encounter_cooldown, 0);
    assert_eq!(state.repel_steps, 0);
}

#[test]
fn test_player_state_default() {
    let player = PlayerState::default();
    assert_eq!(player.x, 0);
    assert_eq!(player.y, 0);
    assert_eq!(player.facing, Direction::Down);
    assert_eq!(player.movement_state, MovementState::Idle);
    assert_eq!(player.transport, TransportMode::Walking);
}

#[test]
fn test_map_connections_empty() {
    let conns = MapConnections::default();
    assert_eq!(conns.count(), 0);
    assert!(conns.get(Direction::Up).is_none());
    assert!(conns.get(Direction::Down).is_none());
    assert!(conns.get(Direction::Left).is_none());
    assert!(conns.get(Direction::Right).is_none());
}

#[test]
fn test_map_connections_partial() {
    let conns = MapConnections {
        north: Some(MapConnection {
            direction: Direction::Up,
            target_map: MapId::Route1,
            offset: 0,
        }),
        south: Some(MapConnection {
            direction: Direction::Down,
            target_map: MapId::Route21,
            offset: 0,
        }),
        west: None,
        east: None,
    };
    assert_eq!(conns.count(), 2);
    assert!(conns.get(Direction::Up).is_some());
    assert!(conns.get(Direction::Down).is_some());
    assert!(conns.get(Direction::Left).is_none());
    assert!(conns.get(Direction::Right).is_none());
    assert_eq!(conns.get(Direction::Up).unwrap().target_map, MapId::Route1);
}

#[test]
fn test_route23_plateau_tileset() {
    // Route 23 uniquely uses Plateau tileset instead of Overworld
    let h = get_map_header(MapId::Route23);
    assert_eq!(h.tileset, TilesetId::Plateau);
    assert_eq!(h.music, MusicId::IndigoPlateau);
    assert!(h.has_north());
    assert!(h.has_south());
    assert_eq!(h.connection_count(), 2);
}

#[test]
fn test_ss_anne_ship_tileset() {
    // All SS Anne interior maps use Ship tileset
    let ship_maps = [
        MapId::SSAnne1F,
        MapId::SSAnne2F,
        MapId::SSAnne3F,
        MapId::SSAnneB1F,
        MapId::SSAnneBow,
        MapId::SSAnneKitchen,
        MapId::SSAnneCaptainsRoom,
        MapId::SSAnne1FRooms,
        MapId::SSAnne2FRooms,
        MapId::SSAnneB1FRooms,
    ];
    for map in &ship_maps {
        assert_eq!(
            get_map_tileset(*map),
            TilesetId::Ship,
            "Expected Ship tileset for {:?}",
            map
        );
        assert_eq!(
            get_map_music(*map),
            MusicId::SSAnne,
            "Expected SSAnne music for {:?}",
            map
        );
    }
}

#[test]
fn test_rocket_hideout_facility_tileset() {
    let rocket_maps = [
        MapId::RocketHideoutB1F,
        MapId::RocketHideoutB2F,
        MapId::RocketHideoutB3F,
        MapId::RocketHideoutB4F,
    ];
    for map in &rocket_maps {
        assert_eq!(
            get_map_tileset(*map),
            TilesetId::Facility,
            "Expected Facility tileset for {:?}",
            map
        );
        assert_eq!(
            get_map_music(*map),
            MusicId::Dungeon1,
            "Expected Dungeon1 music for {:?}",
            map
        );
    }
}

#[test]
fn test_silph_co_facility_tileset() {
    let silph_maps = [
        MapId::SilphCo1F,
        MapId::SilphCo2F,
        MapId::SilphCo3F,
        MapId::SilphCo4F,
        MapId::SilphCo5F,
        MapId::SilphCo6F,
        MapId::SilphCo7F,
        MapId::SilphCo8F,
        MapId::SilphCo9F,
        MapId::SilphCo10F,
    ];
    for map in &silph_maps {
        assert_eq!(
            get_map_tileset(*map),
            TilesetId::Facility,
            "Expected Facility tileset for {:?}",
            map
        );
        assert_eq!(
            get_map_music(*map),
            MusicId::SilphCo,
            "Expected SilphCo music for {:?}",
            map
        );
    }
}

#[test]
fn test_all_248_maps_have_valid_header() {
    // Verify all 248 maps can be looked up without panicking
    for i in 0..248u8 {
        let map = MapId::from_u8(i).expect("valid map id");
        let h = get_map_header(map);
        let (w, _h) = get_map_dimensions(map);
        // Unused maps have 0x0 dimensions but still have valid headers
        let _ = h.tileset;
        let _ = h.music;
        let _ = h.connection_count();
        let _ = w;
    }
}

#[test]
fn test_map_data_struct_creation() {
    let map_data = MapData {
        id: MapId::PalletTown,
        width: 10,
        height: 9,
        tileset: TilesetId::Overworld,
        music: MusicId::PalletTown,
        blocks: vec![0; 90],
        warps: vec![],
        npcs: vec![],
        signs: vec![],
        connections: MapConnections::default(),
    };
    assert_eq!(map_data.blocks.len(), 90);
    assert_eq!(map_data.id, MapId::PalletTown);
}

#[test]
fn test_warp_point_creation() {
    let warp = WarpPoint {
        x: 5,
        y: 3,
        target_map: MapId::RedsHouse1F,
        target_warp_id: 0,
    };
    assert_eq!(warp.target_map, MapId::RedsHouse1F);
}

#[test]
fn test_sign_creation() {
    let sign = Sign {
        x: 7,
        y: 4,
        text_id: 1,
    };
    assert_eq!(sign.text_id, 1);
}

#[test]
fn test_npc_definition_creation() {
    let npc = NpcDefinition {
        sprite_id: 1,
        x: 3,
        y: 5,
        movement: NpcMovementType::Wander,
        facing: Direction::Down,
        range: 2,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0,
    };
    assert_eq!(npc.movement, NpcMovementType::Wander);
    assert!(!npc.is_trainer);
}
