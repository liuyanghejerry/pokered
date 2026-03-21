use crate::maps::MapId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleState {
    On,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToggleEntry {
    pub object_id: u8,
    pub state: ToggleState,
}

pub const NUM_TOGGLEABLE_OBJECTS: usize = 228;

pub const TOGGLE_PALLET_TOWN: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 1,
    state: ToggleState::Off,
}];

pub const TOGGLE_VIRIDIAN_CITY: [ToggleEntry; 2] = [
    ToggleEntry {
        object_id: 5,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 7,
        state: ToggleState::Off,
    },
];

pub const TOGGLE_PEWTER_CITY: [ToggleEntry; 2] = [
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 5,
        state: ToggleState::On,
    },
];

pub const TOGGLE_CERULEAN_CITY: [ToggleEntry; 5] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::Off,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 6,
        state: ToggleState::Off,
    },
    ToggleEntry {
        object_id: 10,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 11,
        state: ToggleState::On,
    },
];

pub const TOGGLE_SAFFRON_CITY: [ToggleEntry; 15] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 5,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 6,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 7,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 8,
        state: ToggleState::Off,
    },
    ToggleEntry {
        object_id: 9,
        state: ToggleState::Off,
    },
    ToggleEntry {
        object_id: 10,
        state: ToggleState::Off,
    },
    ToggleEntry {
        object_id: 11,
        state: ToggleState::Off,
    },
    ToggleEntry {
        object_id: 12,
        state: ToggleState::Off,
    },
    ToggleEntry {
        object_id: 13,
        state: ToggleState::Off,
    },
    ToggleEntry {
        object_id: 14,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 15,
        state: ToggleState::Off,
    },
];

pub const TOGGLE_ROUTE_2: [ToggleEntry; 2] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
];

pub const TOGGLE_ROUTE_4: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 3,
    state: ToggleState::On,
}];

pub const TOGGLE_ROUTE_9: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 10,
    state: ToggleState::On,
}];

pub const TOGGLE_ROUTE_12: [ToggleEntry; 3] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 9,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 10,
        state: ToggleState::On,
    },
];

pub const TOGGLE_ROUTE_15: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 11,
    state: ToggleState::On,
}];

pub const TOGGLE_ROUTE_16: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 7,
    state: ToggleState::On,
}];

pub const TOGGLE_ROUTE_22: [ToggleEntry; 2] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::Off,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::Off,
    },
];

pub const TOGGLE_ROUTE_24: [ToggleEntry; 2] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 8,
        state: ToggleState::On,
    },
];

pub const TOGGLE_ROUTE_25: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 10,
    state: ToggleState::On,
}];

pub const TOGGLE_BLUES_HOUSE: [ToggleEntry; 3] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::Off,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
];

pub const TOGGLE_OAKS_LAB: [ToggleEntry; 8] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 5,
        state: ToggleState::Off,
    },
    ToggleEntry {
        object_id: 6,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 7,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 8,
        state: ToggleState::Off,
    },
];

pub const TOGGLE_VIRIDIAN_GYM: [ToggleEntry; 2] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 11,
        state: ToggleState::On,
    },
];

pub const TOGGLE_MUSEUM_1F: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 5,
    state: ToggleState::On,
}];

pub const TOGGLE_CERULEAN_CAVE_1F: [ToggleEntry; 3] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
];

pub const TOGGLE_POKEMON_TOWER_2F: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 1,
    state: ToggleState::On,
}];

pub const TOGGLE_POKEMON_TOWER_3F: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 4,
    state: ToggleState::On,
}];

pub const TOGGLE_POKEMON_TOWER_4F: [ToggleEntry; 3] = [
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 5,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 6,
        state: ToggleState::On,
    },
];

pub const TOGGLE_POKEMON_TOWER_5F: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 6,
    state: ToggleState::On,
}];

pub const TOGGLE_POKEMON_TOWER_6F: [ToggleEntry; 2] = [
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 5,
        state: ToggleState::On,
    },
];

pub const TOGGLE_POKEMON_TOWER_7F: [ToggleEntry; 4] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
];

pub const TOGGLE_MR_FUJIS_HOUSE: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 5,
    state: ToggleState::Off,
}];

pub const TOGGLE_CELADON_MANSION_ROOF_HOUSE: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 2,
    state: ToggleState::On,
}];

pub const TOGGLE_GAME_CORNER: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 11,
    state: ToggleState::On,
}];

pub const TOGGLE_WARDENS_HOUSE: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 2,
    state: ToggleState::On,
}];

pub const TOGGLE_POKEMON_MANSION_1F: [ToggleEntry; 2] = [
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
];

pub const TOGGLE_FIGHTING_DOJO: [ToggleEntry; 2] = [
    ToggleEntry {
        object_id: 6,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 7,
        state: ToggleState::On,
    },
];

pub const TOGGLE_SILPH_CO_1F: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 1,
    state: ToggleState::Off,
}];

pub const TOGGLE_POWER_PLANT: [ToggleEntry; 14] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 5,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 6,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 7,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 8,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 9,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 10,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 11,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 12,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 13,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 14,
        state: ToggleState::On,
    },
];

pub const TOGGLE_VICTORY_ROAD_2F: [ToggleEntry; 6] = [
    ToggleEntry {
        object_id: 6,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 7,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 8,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 9,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 10,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 13,
        state: ToggleState::On,
    },
];

pub const TOGGLE_BILLS_HOUSE: [ToggleEntry; 3] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::Off,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::Off,
    },
];

pub const TOGGLE_VIRIDIAN_FOREST: [ToggleEntry; 3] = [
    ToggleEntry {
        object_id: 5,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 6,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 7,
        state: ToggleState::On,
    },
];

pub const TOGGLE_MT_MOON_1F: [ToggleEntry; 6] = [
    ToggleEntry {
        object_id: 8,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 9,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 10,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 11,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 12,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 13,
        state: ToggleState::On,
    },
];

pub const TOGGLE_MT_MOON_B2F: [ToggleEntry; 4] = [
    ToggleEntry {
        object_id: 6,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 7,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 8,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 9,
        state: ToggleState::On,
    },
];

pub const TOGGLE_SS_ANNE_2F: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 2,
    state: ToggleState::Off,
}];

pub const TOGGLE_SS_ANNE_1F_ROOMS: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 10,
    state: ToggleState::On,
}];

pub const TOGGLE_SS_ANNE_2F_ROOMS: [ToggleEntry; 2] = [
    ToggleEntry {
        object_id: 6,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 9,
        state: ToggleState::On,
    },
];

pub const TOGGLE_SS_ANNE_B1F_ROOMS: [ToggleEntry; 3] = [
    ToggleEntry {
        object_id: 9,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 10,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 11,
        state: ToggleState::On,
    },
];

pub const TOGGLE_VICTORY_ROAD_3F: [ToggleEntry; 3] = [
    ToggleEntry {
        object_id: 5,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 6,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 10,
        state: ToggleState::On,
    },
];

pub const TOGGLE_ROCKET_HIDEOUT_B1F: [ToggleEntry; 2] = [
    ToggleEntry {
        object_id: 6,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 7,
        state: ToggleState::On,
    },
];

pub const TOGGLE_ROCKET_HIDEOUT_B2F: [ToggleEntry; 4] = [
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 5,
        state: ToggleState::On,
    },
];

pub const TOGGLE_ROCKET_HIDEOUT_B3F: [ToggleEntry; 2] = [
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
];

pub const TOGGLE_ROCKET_HIDEOUT_B4F: [ToggleEntry; 6] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 5,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 6,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 7,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 8,
        state: ToggleState::Off,
    },
    ToggleEntry {
        object_id: 9,
        state: ToggleState::Off,
    },
];

pub const TOGGLE_SILPH_CO_2F: [ToggleEntry; 5] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 5,
        state: ToggleState::On,
    },
];

pub const TOGGLE_SILPH_CO_3F: [ToggleEntry; 3] = [
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
];

pub const TOGGLE_SILPH_CO_4F: [ToggleEntry; 6] = [
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 5,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 6,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 7,
        state: ToggleState::On,
    },
];

pub const TOGGLE_SILPH_CO_5F: [ToggleEntry; 7] = [
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 5,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 6,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 7,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 8,
        state: ToggleState::On,
    },
];

pub const TOGGLE_SILPH_CO_6F: [ToggleEntry; 5] = [
    ToggleEntry {
        object_id: 6,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 7,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 8,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 9,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 10,
        state: ToggleState::On,
    },
];

pub const TOGGLE_SILPH_CO_7F: [ToggleEntry; 8] = [
    ToggleEntry {
        object_id: 5,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 6,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 7,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 8,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 9,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 10,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 11,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 12,
        state: ToggleState::On,
    },
];

pub const TOGGLE_SILPH_CO_8F: [ToggleEntry; 3] = [
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
];

pub const TOGGLE_SILPH_CO_9F: [ToggleEntry; 3] = [
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
];

pub const TOGGLE_SILPH_CO_10F: [ToggleEntry; 6] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 5,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 6,
        state: ToggleState::On,
    },
];

pub const TOGGLE_SILPH_CO_11F: [ToggleEntry; 3] = [
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 5,
        state: ToggleState::On,
    },
];

pub const TOGGLE_UNUSED_MAP_F4: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 2,
    state: ToggleState::On,
}];

pub const TOGGLE_POKEMON_MANSION_2F: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 2,
    state: ToggleState::On,
}];

pub const TOGGLE_POKEMON_MANSION_3F: [ToggleEntry; 2] = [
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
];

pub const TOGGLE_POKEMON_MANSION_B1F: [ToggleEntry; 5] = [
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 5,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 6,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 8,
        state: ToggleState::On,
    },
];

pub const TOGGLE_SAFARI_ZONE_EAST: [ToggleEntry; 4] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
];

pub const TOGGLE_SAFARI_ZONE_NORTH: [ToggleEntry; 2] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
];

pub const TOGGLE_SAFARI_ZONE_WEST: [ToggleEntry; 4] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
];

pub const TOGGLE_SAFARI_ZONE_CENTER: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 1,
    state: ToggleState::On,
}];

pub const TOGGLE_CERULEAN_CAVE_2F: [ToggleEntry; 3] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
];

pub const TOGGLE_CERULEAN_CAVE_B1F: [ToggleEntry; 3] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
];

pub const TOGGLE_VICTORY_ROAD_1F: [ToggleEntry; 2] = [
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 4,
        state: ToggleState::On,
    },
];

pub const TOGGLE_CHAMPIONS_ROOM: [ToggleEntry; 1] = [ToggleEntry {
    object_id: 2,
    state: ToggleState::Off,
}];

pub const TOGGLE_SEAFOAM_ISLANDS_1F: [ToggleEntry; 2] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
];

pub const TOGGLE_SEAFOAM_ISLANDS_B1F: [ToggleEntry; 2] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::Off,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::Off,
    },
];

pub const TOGGLE_SEAFOAM_ISLANDS_B2F: [ToggleEntry; 2] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::Off,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::Off,
    },
];

pub const TOGGLE_SEAFOAM_ISLANDS_B3F: [ToggleEntry; 4] = [
    ToggleEntry {
        object_id: 2,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
    ToggleEntry {
        object_id: 5,
        state: ToggleState::Off,
    },
    ToggleEntry {
        object_id: 6,
        state: ToggleState::Off,
    },
];

pub const TOGGLE_SEAFOAM_ISLANDS_B4F: [ToggleEntry; 3] = [
    ToggleEntry {
        object_id: 1,
        state: ToggleState::Off,
    },
    ToggleEntry {
        object_id: 2,
        state: ToggleState::Off,
    },
    ToggleEntry {
        object_id: 3,
        state: ToggleState::On,
    },
];

pub fn toggleable_objects_for_map(map: MapId) -> &'static [ToggleEntry] {
    match map {
        MapId::PalletTown => &TOGGLE_PALLET_TOWN,
        MapId::ViridianCity => &TOGGLE_VIRIDIAN_CITY,
        MapId::PewterCity => &TOGGLE_PEWTER_CITY,
        MapId::CeruleanCity => &TOGGLE_CERULEAN_CITY,
        MapId::SaffronCity => &TOGGLE_SAFFRON_CITY,
        MapId::Route2 => &TOGGLE_ROUTE_2,
        MapId::Route4 => &TOGGLE_ROUTE_4,
        MapId::Route9 => &TOGGLE_ROUTE_9,
        MapId::Route12 => &TOGGLE_ROUTE_12,
        MapId::Route15 => &TOGGLE_ROUTE_15,
        MapId::Route16 => &TOGGLE_ROUTE_16,
        MapId::Route22 => &TOGGLE_ROUTE_22,
        MapId::Route24 => &TOGGLE_ROUTE_24,
        MapId::Route25 => &TOGGLE_ROUTE_25,
        MapId::BluesHouse => &TOGGLE_BLUES_HOUSE,
        MapId::OaksLab => &TOGGLE_OAKS_LAB,
        MapId::ViridianGym => &TOGGLE_VIRIDIAN_GYM,
        MapId::Museum1F => &TOGGLE_MUSEUM_1F,
        MapId::CeruleanCave1F => &TOGGLE_CERULEAN_CAVE_1F,
        MapId::PokemonTower2F => &TOGGLE_POKEMON_TOWER_2F,
        MapId::PokemonTower3F => &TOGGLE_POKEMON_TOWER_3F,
        MapId::PokemonTower4F => &TOGGLE_POKEMON_TOWER_4F,
        MapId::PokemonTower5F => &TOGGLE_POKEMON_TOWER_5F,
        MapId::PokemonTower6F => &TOGGLE_POKEMON_TOWER_6F,
        MapId::PokemonTower7F => &TOGGLE_POKEMON_TOWER_7F,
        MapId::MrFujisHouse => &TOGGLE_MR_FUJIS_HOUSE,
        MapId::CeladonMansionRoofHouse => &TOGGLE_CELADON_MANSION_ROOF_HOUSE,
        MapId::GameCorner => &TOGGLE_GAME_CORNER,
        MapId::WardensHouse => &TOGGLE_WARDENS_HOUSE,
        MapId::PokemonMansion1F => &TOGGLE_POKEMON_MANSION_1F,
        MapId::FightingDojo => &TOGGLE_FIGHTING_DOJO,
        MapId::SilphCo1F => &TOGGLE_SILPH_CO_1F,
        MapId::PowerPlant => &TOGGLE_POWER_PLANT,
        MapId::VictoryRoad2F => &TOGGLE_VICTORY_ROAD_2F,
        MapId::BillsHouse => &TOGGLE_BILLS_HOUSE,
        MapId::ViridianForest => &TOGGLE_VIRIDIAN_FOREST,
        MapId::MtMoon1F => &TOGGLE_MT_MOON_1F,
        MapId::MtMoonB2F => &TOGGLE_MT_MOON_B2F,
        MapId::SSAnne2F => &TOGGLE_SS_ANNE_2F,
        MapId::SSAnne1FRooms => &TOGGLE_SS_ANNE_1F_ROOMS,
        MapId::SSAnne2FRooms => &TOGGLE_SS_ANNE_2F_ROOMS,
        MapId::SSAnneB1FRooms => &TOGGLE_SS_ANNE_B1F_ROOMS,
        MapId::VictoryRoad3F => &TOGGLE_VICTORY_ROAD_3F,
        MapId::RocketHideoutB1F => &TOGGLE_ROCKET_HIDEOUT_B1F,
        MapId::RocketHideoutB2F => &TOGGLE_ROCKET_HIDEOUT_B2F,
        MapId::RocketHideoutB3F => &TOGGLE_ROCKET_HIDEOUT_B3F,
        MapId::RocketHideoutB4F => &TOGGLE_ROCKET_HIDEOUT_B4F,
        MapId::SilphCo2F => &TOGGLE_SILPH_CO_2F,
        MapId::SilphCo3F => &TOGGLE_SILPH_CO_3F,
        MapId::SilphCo4F => &TOGGLE_SILPH_CO_4F,
        MapId::SilphCo5F => &TOGGLE_SILPH_CO_5F,
        MapId::SilphCo6F => &TOGGLE_SILPH_CO_6F,
        MapId::SilphCo7F => &TOGGLE_SILPH_CO_7F,
        MapId::SilphCo8F => &TOGGLE_SILPH_CO_8F,
        MapId::SilphCo9F => &TOGGLE_SILPH_CO_9F,
        MapId::SilphCo10F => &TOGGLE_SILPH_CO_10F,
        MapId::SilphCo11F => &TOGGLE_SILPH_CO_11F,
        MapId::UnusedMapF4 => &TOGGLE_UNUSED_MAP_F4,
        MapId::PokemonMansion2F => &TOGGLE_POKEMON_MANSION_2F,
        MapId::PokemonMansion3F => &TOGGLE_POKEMON_MANSION_3F,
        MapId::PokemonMansionB1F => &TOGGLE_POKEMON_MANSION_B1F,
        MapId::SafariZoneEast => &TOGGLE_SAFARI_ZONE_EAST,
        MapId::SafariZoneNorth => &TOGGLE_SAFARI_ZONE_NORTH,
        MapId::SafariZoneWest => &TOGGLE_SAFARI_ZONE_WEST,
        MapId::SafariZoneCenter => &TOGGLE_SAFARI_ZONE_CENTER,
        MapId::CeruleanCave2F => &TOGGLE_CERULEAN_CAVE_2F,
        MapId::CeruleanCaveB1F => &TOGGLE_CERULEAN_CAVE_B1F,
        MapId::VictoryRoad1F => &TOGGLE_VICTORY_ROAD_1F,
        MapId::ChampionsRoom => &TOGGLE_CHAMPIONS_ROOM,
        MapId::SeafoamIslands1F => &TOGGLE_SEAFOAM_ISLANDS_1F,
        MapId::SeafoamIslandsB1F => &TOGGLE_SEAFOAM_ISLANDS_B1F,
        MapId::SeafoamIslandsB2F => &TOGGLE_SEAFOAM_ISLANDS_B2F,
        MapId::SeafoamIslandsB3F => &TOGGLE_SEAFOAM_ISLANDS_B3F,
        MapId::SeafoamIslandsB4F => &TOGGLE_SEAFOAM_ISLANDS_B4F,
        _ => &[],
    }
}
