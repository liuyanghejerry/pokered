//! Map connection data — which maps connect to each other and with what offset.
//!
//! Ported from data/maps/headers/*.asm connection macros.
//! Each outdoor map can have up to 4 connections (N/S/W/E).

use crate::maps::MapId;

/// A single map connection entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConnectionData {
    pub target_map: MapId,
    pub offset: i8,
}

/// All connections for a single map (up to one per cardinal direction).
#[derive(Debug, Clone, Copy)]
pub struct MapConnectionEntry {
    pub north: Option<ConnectionData>,
    pub south: Option<ConnectionData>,
    pub west: Option<ConnectionData>,
    pub east: Option<ConnectionData>,
}

impl MapConnectionEntry {
    pub const NONE: Self = Self {
        north: None,
        south: None,
        west: None,
        east: None,
    };

    pub const fn connection_count(&self) -> u8 {
        let mut c = 0;
        if self.north.is_some() {
            c += 1;
        }
        if self.south.is_some() {
            c += 1;
        }
        if self.west.is_some() {
            c += 1;
        }
        if self.east.is_some() {
            c += 1;
        }
        c
    }
}

const fn n(target: MapId, offset: i8) -> Option<ConnectionData> {
    Some(ConnectionData {
        target_map: target,
        offset,
    })
}

use MapId::*;

/// Connection data for all 248 maps.
/// Outdoor maps have connections; indoor maps have NONE.
pub static MAP_CONNECTIONS: [MapConnectionEntry; 248] = {
    let e = MapConnectionEntry::NONE;
    let mut t = [e; 248];

    // PalletTown (0x00): NORTH | SOUTH
    t[PalletTown as usize] = MapConnectionEntry {
        north: n(Route1, 0),
        south: n(Route21, 0),
        west: None,
        east: None,
    };
    // ViridianCity (0x01): NORTH | SOUTH | WEST
    t[ViridianCity as usize] = MapConnectionEntry {
        north: n(Route2, 5),
        south: n(Route1, 5),
        west: n(Route22, 4),
        east: None,
    };
    // PewterCity (0x02): SOUTH | EAST
    t[PewterCity as usize] = MapConnectionEntry {
        north: None,
        south: n(Route2, 5),
        west: None,
        east: n(Route3, 4),
    };
    // CeruleanCity (0x03): NORTH | SOUTH | WEST | EAST
    t[CeruleanCity as usize] = MapConnectionEntry {
        north: n(Route24, 5),
        south: n(Route5, 5),
        west: n(Route4, 4),
        east: n(Route9, 4),
    };
    // LavenderTown (0x04): NORTH | SOUTH | WEST
    t[LavenderTown as usize] = MapConnectionEntry {
        north: n(Route10, 0),
        south: n(Route12, 0),
        west: n(Route8, 0),
        east: None,
    };
    // VermilionCity (0x05): NORTH | EAST
    t[VermilionCity as usize] = MapConnectionEntry {
        north: n(Route6, 5),
        south: None,
        west: None,
        east: n(Route11, 4),
    };
    // CeladonCity (0x06): WEST | EAST
    t[CeladonCity as usize] = MapConnectionEntry {
        north: None,
        south: None,
        west: n(Route16, 4),
        east: n(Route7, 4),
    };
    // FuchsiaCity (0x07): SOUTH | WEST | EAST
    t[FuchsiaCity as usize] = MapConnectionEntry {
        north: None,
        south: n(Route19, 5),
        west: n(Route18, 4),
        east: n(Route15, 4),
    };
    // CinnabarIsland (0x08): NORTH | EAST
    t[CinnabarIsland as usize] = MapConnectionEntry {
        north: n(Route21, 0),
        south: None,
        west: None,
        east: n(Route20, 0),
    };
    // IndigoPlateau (0x09): SOUTH
    t[IndigoPlateau as usize] = MapConnectionEntry {
        north: None,
        south: n(Route23, 0),
        west: None,
        east: None,
    };
    // Route1 (0x0E = 14): NORTH | SOUTH
    t[Route1 as usize] = MapConnectionEntry {
        north: n(ViridianCity, -5),
        south: n(PalletTown, 0),
        west: None,
        east: None,
    };
    // Route2 (0x0F = 15): NORTH | SOUTH
    t[Route2 as usize] = MapConnectionEntry {
        north: n(PewterCity, -5),
        south: n(ViridianCity, -5),
        west: None,
        east: None,
    };
    // Route3 (0x10 = 16): NORTH | WEST
    t[Route3 as usize] = MapConnectionEntry {
        north: n(Route4, 25),
        south: None,
        west: n(PewterCity, -4),
        east: None,
    };
    // Route4 (0x11 = 17): SOUTH | EAST
    t[Route4 as usize] = MapConnectionEntry {
        north: None,
        south: n(Route3, -25),
        west: None,
        east: n(CeruleanCity, -4),
    };
    // Route5 (0x12 = 18): NORTH | SOUTH
    t[Route5 as usize] = MapConnectionEntry {
        north: n(CeruleanCity, -5),
        south: n(SaffronCity, -5),
        west: None,
        east: None,
    };
    // Route6 (0x13 = 19): NORTH | SOUTH
    t[Route6 as usize] = MapConnectionEntry {
        north: n(SaffronCity, -5),
        south: n(VermilionCity, -5),
        west: None,
        east: None,
    };
    // Route7 (0x14 = 20): WEST | EAST
    t[Route7 as usize] = MapConnectionEntry {
        north: None,
        south: None,
        west: n(CeladonCity, -4),
        east: n(SaffronCity, -4),
    };
    // Route8 (0x15 = 21): WEST | EAST
    t[Route8 as usize] = MapConnectionEntry {
        north: None,
        south: None,
        west: n(SaffronCity, -4),
        east: n(LavenderTown, 0),
    };
    // Route9 (0x16 = 22): WEST | EAST
    t[Route9 as usize] = MapConnectionEntry {
        north: None,
        south: None,
        west: n(CeruleanCity, -4),
        east: n(Route10, 0),
    };
    // Route10 (0x17 = 23): SOUTH | WEST
    t[Route10 as usize] = MapConnectionEntry {
        north: None,
        south: n(LavenderTown, 0),
        west: n(Route9, 0),
        east: None,
    };
    // Route11 (0x18 = 24): WEST | EAST
    t[Route11 as usize] = MapConnectionEntry {
        north: None,
        south: None,
        west: n(VermilionCity, -4),
        east: n(Route12, -27),
    };
    // Route12 (0x19 = 25): NORTH | SOUTH | WEST
    t[Route12 as usize] = MapConnectionEntry {
        north: n(LavenderTown, 0),
        south: n(Route13, -20),
        west: n(Route11, 27),
        east: None,
    };
    // Route13 (0x1A = 26): NORTH | WEST
    t[Route13 as usize] = MapConnectionEntry {
        north: n(Route12, 20),
        south: None,
        west: n(Route14, 0),
        east: None,
    };
    // Route14 (0x1B = 27): WEST | EAST
    t[Route14 as usize] = MapConnectionEntry {
        north: None,
        south: None,
        west: n(Route15, 18),
        east: n(Route13, 0),
    };
    // Route15 (0x1C = 28): WEST | EAST
    t[Route15 as usize] = MapConnectionEntry {
        north: None,
        south: None,
        west: n(FuchsiaCity, -4),
        east: n(Route14, -18),
    };
    // Route16 (0x1D = 29): SOUTH | EAST
    t[Route16 as usize] = MapConnectionEntry {
        north: None,
        south: n(Route17, 0),
        west: None,
        east: n(CeladonCity, -4),
    };
    // Route17 (0x1E = 30): NORTH | SOUTH
    t[Route17 as usize] = MapConnectionEntry {
        north: n(Route16, 0),
        south: n(Route18, 0),
        west: None,
        east: None,
    };
    // Route18 (0x1F = 31): NORTH | EAST
    t[Route18 as usize] = MapConnectionEntry {
        north: n(Route17, 0),
        south: None,
        west: None,
        east: n(FuchsiaCity, -4),
    };
    // Route19 (0x20 = 32): NORTH | WEST
    t[Route19 as usize] = MapConnectionEntry {
        north: n(FuchsiaCity, -5),
        south: None,
        west: n(Route20, 18),
        east: None,
    };
    // Route20 (0x21 = 33): WEST | EAST
    t[Route20 as usize] = MapConnectionEntry {
        north: None,
        south: None,
        west: n(CinnabarIsland, 0),
        east: n(Route19, -18),
    };
    // Route21 (0x22 = 34): NORTH | SOUTH
    t[Route21 as usize] = MapConnectionEntry {
        north: n(PalletTown, 0),
        south: n(CinnabarIsland, 0),
        west: None,
        east: None,
    };
    // Route22 (0x23 = 35): NORTH | EAST
    t[Route22 as usize] = MapConnectionEntry {
        north: n(Route23, 0),
        south: None,
        west: None,
        east: n(ViridianCity, -4),
    };
    // Route23 (0x24 = 36): NORTH | SOUTH
    t[Route23 as usize] = MapConnectionEntry {
        north: n(IndigoPlateau, 0),
        south: n(Route22, 0),
        west: None,
        east: None,
    };
    // Route24 (0x25 = 37): SOUTH | EAST
    t[Route24 as usize] = MapConnectionEntry {
        north: None,
        south: n(CeruleanCity, -5),
        west: None,
        east: n(Route25, 0),
    };
    // Route25 (0x26 = 38): WEST
    t[Route25 as usize] = MapConnectionEntry {
        north: None,
        south: None,
        west: n(Route24, 0),
        east: None,
    };
    // SaffronCity: NORTH | SOUTH | WEST | EAST
    t[SaffronCity as usize] = MapConnectionEntry {
        north: n(Route5, 5),
        south: n(Route6, 5),
        west: n(Route7, 4),
        east: n(Route8, 4),
    };

    t
};

/// Get the connections for a map.
pub fn get_map_connections(map: MapId) -> &'static MapConnectionEntry {
    &MAP_CONNECTIONS[map as usize]
}
