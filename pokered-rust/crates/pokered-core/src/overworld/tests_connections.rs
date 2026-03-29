//! Tests for M4.3 — map connections, warp data, and transition logic.

use pokered_data::map_connections::{get_map_connections, MAP_CONNECTIONS};
use pokered_data::map_objects::{get_map_warps, WarpData};
use pokered_data::maps::MapId;

use crate::overworld::map_transitions::{
    calculate_connection_transition, check_warp_at, execute_warp, resolve_warp_destination,
    ConnectionTransition,
};
use crate::overworld::Direction;

// ── Connection Data Integrity Tests ─────────────────────────────────

#[test]
fn test_pallet_town_connections() {
    let conns = get_map_connections(MapId::PalletTown);
    assert!(
        conns.north.is_some(),
        "PalletTown should have north connection"
    );
    assert!(
        conns.south.is_some(),
        "PalletTown should have south connection"
    );
    assert!(
        conns.west.is_none(),
        "PalletTown should not have west connection"
    );
    assert!(
        conns.east.is_none(),
        "PalletTown should not have east connection"
    );

    let north = conns.north.unwrap();
    assert_eq!(north.target_map, MapId::Route1);
    assert_eq!(north.offset, 0);

    let south = conns.south.unwrap();
    assert_eq!(south.target_map, MapId::Route21);
    assert_eq!(south.offset, 0);
}

#[test]
fn test_viridian_city_connections() {
    let conns = get_map_connections(MapId::ViridianCity);
    assert!(conns.north.is_some());
    assert!(conns.south.is_some());
    assert!(conns.west.is_some());
    assert!(conns.east.is_none());

    assert_eq!(conns.north.unwrap().target_map, MapId::Route2);
    assert_eq!(conns.south.unwrap().target_map, MapId::Route1);
    assert_eq!(conns.west.unwrap().target_map, MapId::Route22);
}

#[test]
fn test_cerulean_city_all_four_connections() {
    let conns = get_map_connections(MapId::CeruleanCity);
    assert!(conns.north.is_some());
    assert!(conns.south.is_some());
    assert!(conns.west.is_some());
    assert!(conns.east.is_some());
    assert_eq!(conns.north.unwrap().target_map, MapId::Route24);
    assert_eq!(conns.south.unwrap().target_map, MapId::Route5);
    assert_eq!(conns.west.unwrap().target_map, MapId::Route4);
    assert_eq!(conns.east.unwrap().target_map, MapId::Route9);
}

#[test]
fn test_indoor_maps_have_no_connections() {
    // Indoor maps (RedsHouse1F, OaksLab, etc.) should have no connections
    let indoor_maps = [
        MapId::RedsHouse1F,
        MapId::RedsHouse2F,
        MapId::OaksLab,
        MapId::ViridianPokecenter,
        MapId::ViridianMart,
        MapId::PewterGym,
    ];
    for map in &indoor_maps {
        let conns = get_map_connections(*map);
        assert!(
            conns.north.is_none()
                && conns.south.is_none()
                && conns.west.is_none()
                && conns.east.is_none(),
            "{:?} should have no connections",
            map
        );
    }
}

#[test]
fn test_connection_symmetry_pallet_route1() {
    // PalletTown connects north to Route1
    let pallet = get_map_connections(MapId::PalletTown);
    assert_eq!(pallet.north.unwrap().target_map, MapId::Route1);

    // Route1 connects south to PalletTown
    let route1 = get_map_connections(MapId::Route1);
    assert_eq!(route1.south.unwrap().target_map, MapId::PalletTown);
}

#[test]
fn test_connection_count_across_all_maps() {
    // Count total connections
    let total: u8 = MAP_CONNECTIONS.iter().map(|c| c.connection_count()).sum();
    // We know from data extraction there are 78 total connections across 36 maps
    assert_eq!(total, 78, "Expected 78 total connections");
}

#[test]
fn test_maps_with_connections_count() {
    let maps_with_conns = MAP_CONNECTIONS
        .iter()
        .filter(|c| c.connection_count() > 0)
        .count();
    assert_eq!(maps_with_conns, 36, "Expected 36 maps with connections");
}

// ── Warp Data Integrity Tests ───────────────────────────────────────

#[test]
fn test_pallet_town_warps() {
    let warps = get_map_warps(MapId::PalletTown);
    assert_eq!(warps.len(), 3, "PalletTown has 3 warps");

    // Warp 0: Red's House
    assert_eq!(warps[0].x, 5);
    assert_eq!(warps[0].y, 5);
    assert_eq!(warps[0].dest_map, Some(MapId::RedsHouse1F));
    assert_eq!(warps[0].dest_warp_id, 0);

    // Warp 1: Blue's House
    assert_eq!(warps[1].x, 13);
    assert_eq!(warps[1].y, 5);
    assert_eq!(warps[1].dest_map, Some(MapId::BluesHouse));
    assert_eq!(warps[1].dest_warp_id, 0);

    // Warp 2: Oak's Lab
    assert_eq!(warps[2].x, 12);
    assert_eq!(warps[2].y, 11);
    assert_eq!(warps[2].dest_map, Some(MapId::OaksLab));
    assert_eq!(warps[2].dest_warp_id, 1);
}

#[test]
fn test_reds_house_1f_warps() {
    let warps = get_map_warps(MapId::RedsHouse1F);
    assert_eq!(warps.len(), 3);

    // Warps 0,1 go to LAST_MAP (None) — they are the exit doors
    assert_eq!(warps[0].dest_map, None); // LAST_MAP
    assert_eq!(warps[1].dest_map, None); // LAST_MAP

    // Warp 2 goes to RedsHouse2F (stairs up)
    assert_eq!(warps[2].dest_map, Some(MapId::RedsHouse2F));
    assert_eq!(warps[2].dest_warp_id, 0);
}

#[test]
fn test_reds_house_2f_warp_back() {
    let warps = get_map_warps(MapId::RedsHouse2F);
    assert_eq!(warps.len(), 1);
    assert_eq!(warps[0].dest_map, Some(MapId::RedsHouse1F));
    assert_eq!(warps[0].dest_warp_id, 2);
}

#[test]
fn test_maps_without_warps() {
    // Some maps have no warps (usually routes without buildings)
    let warps = get_map_warps(MapId::Route1);
    // Route1 has no warps in the warp table (no buildings directly on the route)
    // (Route1 has gate connections, but those are in different map IDs)
    assert_eq!(warps.len(), 0, "Route1 has no direct warps");
}

#[test]
fn test_viridian_city_warps() {
    let warps = get_map_warps(MapId::ViridianCity);
    assert_eq!(warps.len(), 5);
    // First warp leads to ViridianPokecenter
    assert_eq!(warps[0].dest_map, Some(MapId::ViridianPokecenter));
}

#[test]
fn test_silph_co_elevator_last_map_warps() {
    // SilphCo Elevator warps have UNUSED_MAP_ED → stored as None
    let warps = get_map_warps(MapId::SilphCoElevator);
    assert_eq!(warps.len(), 2);
    assert_eq!(warps[0].dest_map, None); // dynamically assigned
    assert_eq!(warps[1].dest_map, None);
}

#[test]
fn test_total_warp_count() {
    // Count total warps across all maps
    let total: usize = (0..248u8)
        .filter_map(MapId::from_u8)
        .map(|m| get_map_warps(m).len())
        .sum();
    // We know from extraction: 805 total warps across 212 maps
    assert_eq!(total, 805, "Expected 805 total warps");
}

// ── Connection Transition Tests ─────────────────────────────────────

#[test]
fn test_walk_north_from_pallet_to_route1() {
    // PalletTown: 10×9 blocks = 20×18 tiles
    // Walking north from Y=0 → Route1, offset=0
    // Route1: 10×18 blocks = 20×36 tiles → new Y = 35 (bottom)
    let result = calculate_connection_transition(
        MapId::PalletTown,
        10, // player X (mid-map)
        0,  // player Y (top edge)
        Direction::Up,
    );
    assert!(result.is_some());
    let t = result.unwrap();
    assert_eq!(t.new_map, MapId::Route1);
    assert_eq!(t.new_y, 35); // bottom of Route1 (18*2-1)
    assert_eq!(t.new_x, 10); // offset=0 → same X
}

#[test]
fn test_walk_south_from_pallet_to_route21() {
    // PalletTown: 20×18 tiles, walking south from Y=17
    // Route21: 10×45 blocks = 20×90 tiles → new Y = 0 (top)
    let result = calculate_connection_transition(
        MapId::PalletTown,
        10,
        17, // bottom edge (18-1)
        Direction::Down,
    );
    assert!(result.is_some());
    let t = result.unwrap();
    assert_eq!(t.new_map, MapId::Route21);
    assert_eq!(t.new_y, 0);
    assert_eq!(t.new_x, 10); // offset=0
}

#[test]
fn test_walk_south_from_route1_to_pallet() {
    // Route1: 10×18 blocks = 20×36 tiles
    // Route1 south → PalletTown, offset=0
    let result = calculate_connection_transition(
        MapId::Route1,
        10,
        35, // bottom edge
        Direction::Down,
    );
    assert!(result.is_some());
    let t = result.unwrap();
    assert_eq!(t.new_map, MapId::PalletTown);
    assert_eq!(t.new_y, 0);
}

#[test]
fn test_walk_north_from_route1_to_viridian() {
    // Route1 north → ViridianCity, offset=-5
    // Route1 top edge Y=0
    let result = calculate_connection_transition(MapId::Route1, 10, 0, Direction::Up);
    assert!(result.is_some());
    let t = result.unwrap();
    assert_eq!(t.new_map, MapId::ViridianCity);
    // ViridianCity: height is looked up from MapId, offset=-5 (blocks)
    // new_x = 10 + (-5)*2 = 0
    assert_eq!(t.new_x, 0);
}

#[test]
fn test_connection_with_positive_offset() {
    // ViridianCity north → Route2, offset=5
    // ViridianCity: Y=0 top edge
    let result = calculate_connection_transition(MapId::ViridianCity, 0, 0, Direction::Up);
    assert!(result.is_some());
    let t = result.unwrap();
    assert_eq!(t.new_map, MapId::Route2);
    // new_x = 0 + 5*2 = 10
    assert_eq!(t.new_x, 10);
}

#[test]
fn test_connection_west_viridian_to_route22() {
    // ViridianCity west → Route22, offset=4
    // ViridianCity: X=0, moving left
    let result = calculate_connection_transition(MapId::ViridianCity, 0, 10, Direction::Left);
    assert!(result.is_some());
    let t = result.unwrap();
    assert_eq!(t.new_map, MapId::Route22);
    // Route22 width: need to look up... dest_w*2-1
    let route22_w = MapId::Route22.width() as u16 * 2;
    assert_eq!(t.new_x, route22_w - 1);
    // new_y = 10 + 4*2 = 18
    assert_eq!(t.new_y, 18);
}

#[test]
fn test_no_connection_returns_none() {
    // PalletTown has no west connection
    let result = calculate_connection_transition(MapId::PalletTown, 0, 10, Direction::Left);
    assert!(result.is_none());
}

#[test]
fn test_not_at_edge_returns_none() {
    // Player is in the middle of the map, not at edge → no transition
    let result = calculate_connection_transition(MapId::PalletTown, 10, 10, Direction::Up);
    assert!(result.is_none());
}

#[test]
fn test_indoor_map_no_connections() {
    // Indoor map should never have connection transitions
    let result = calculate_connection_transition(MapId::RedsHouse1F, 0, 0, Direction::Up);
    assert!(result.is_none());
}

// ── Warp Transition Tests ───────────────────────────────────────────

#[test]
fn test_check_warp_pallet_town_reds_house() {
    let warp = check_warp_at(MapId::PalletTown, 5, 5);
    assert!(warp.is_some());
    let w = warp.unwrap();
    assert_eq!(w.new_map, MapId::RedsHouse1F);
    assert_eq!(w.dest_warp_id, 0);
    assert!(!w.is_last_map);
}

#[test]
fn test_check_warp_oaks_lab() {
    let warp = check_warp_at(MapId::PalletTown, 12, 11);
    assert!(warp.is_some());
    let w = warp.unwrap();
    assert_eq!(w.new_map, MapId::OaksLab);
    assert_eq!(w.dest_warp_id, 1);
}

#[test]
fn test_check_warp_no_warp() {
    // Position with no warp
    let warp = check_warp_at(MapId::PalletTown, 0, 0);
    assert!(warp.is_none());
}

#[test]
fn test_check_warp_last_map() {
    let warp = check_warp_at(MapId::RedsHouse1F, 2, 7);
    assert!(warp.is_some());
    let w = warp.unwrap();
    assert!(w.is_last_map);
}

#[test]
fn test_resolve_warp_destination() {
    let pos = resolve_warp_destination(MapId::RedsHouse1F, 0);
    assert_eq!(pos, Some((2, 7)));

    let pos = resolve_warp_destination(MapId::RedsHouse1F, 2);
    assert_eq!(pos, Some((7, 1)));
}

#[test]
fn test_resolve_warp_out_of_bounds() {
    let pos = resolve_warp_destination(MapId::RedsHouse1F, 99);
    assert!(pos.is_none());
}

#[test]
fn test_execute_warp_full() {
    let result = execute_warp(MapId::PalletTown, 5, 5, None);
    assert!(result.is_some());
    let (map, x, y) = result.unwrap();
    assert_eq!(map, MapId::RedsHouse1F);
    assert_eq!(x, 2);
    assert_eq!(y, 7);
}

#[test]
fn test_execute_warp_last_map() {
    let result = execute_warp(MapId::RedsHouse1F, 2, 7, Some(MapId::PalletTown));
    assert!(result.is_some());
    let (map, x, y) = result.unwrap();
    assert_eq!(map, MapId::PalletTown);
    assert_eq!(x, 5);
    assert_eq!(y, 5);
}

#[test]
fn test_execute_warp_last_map_none_returns_none() {
    // LAST_MAP warp with no last_map info → should return None
    let result = execute_warp(MapId::RedsHouse1F, 2, 7, None);
    assert!(result.is_none());
}

#[test]
fn test_execute_warp_stairs_reds_house() {
    let result = execute_warp(MapId::RedsHouse1F, 7, 1, None);
    assert!(result.is_some());
    let (map, x, y) = result.unwrap();
    assert_eq!(map, MapId::RedsHouse2F);
    assert_eq!(x, 7);
    assert_eq!(y, 1);
}

#[test]
fn test_execute_warp_stairs_down() {
    let result = execute_warp(MapId::RedsHouse2F, 7, 1, None);
    assert!(result.is_some());
    let (map, x, y) = result.unwrap();
    assert_eq!(map, MapId::RedsHouse1F);
    assert_eq!(x, 7);
    assert_eq!(y, 1);
}

#[test]
fn test_no_warp_returns_none() {
    let result = execute_warp(MapId::PalletTown, 0, 0, None);
    assert!(result.is_none());
}
