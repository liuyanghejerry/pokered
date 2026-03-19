//! Wild encounter data for all maps.
//! Auto-generated from data/wild/*.asm — do not edit manually.

use crate::species::Species;
use serde::{Deserialize, Serialize};

/// Encounter slot probabilities (out of 256)
/// 10 slots with decreasing probability
pub const ENCOUNTER_SLOT_CHANCES: [u8; 10] = [51, 51, 39, 25, 25, 25, 13, 13, 11, 3];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WildMon {
    pub level: u8,
    pub species: Species,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WildEncounterTable {
    pub encounter_rate: u8,
    pub mons: Vec<WildMon>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapWildData {
    pub name: &'static str,
    pub grass: WildEncounterTable,
    pub water: WildEncounterTable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameVersion {
    Red,
    Blue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FishingGroup {
    pub mons: Vec<WildMon>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuperRodMapEntry {
    pub map_name: &'static str,
    pub group_index: usize,
}

/// Get all wild encounter data for the given game version.
/// Each entry corresponds to a unique map location that has wild encounters.
pub fn wild_data(version: GameVersion) -> Vec<MapWildData> {
    match version {
        GameVersion::Red => vec![
            MapWildData {
                name: "CeruleanCave1F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 46,
                            species: Species::Golbat,
                        },
                        WildMon {
                            level: 46,
                            species: Species::Hypno,
                        },
                        WildMon {
                            level: 46,
                            species: Species::Magneton,
                        },
                        WildMon {
                            level: 49,
                            species: Species::Dodrio,
                        },
                        WildMon {
                            level: 49,
                            species: Species::Venomoth,
                        },
                        WildMon {
                            level: 52,
                            species: Species::Arbok,
                        },
                        WildMon {
                            level: 49,
                            species: Species::Kadabra,
                        },
                        WildMon {
                            level: 52,
                            species: Species::Parasect,
                        },
                        WildMon {
                            level: 53,
                            species: Species::Raichu,
                        },
                        WildMon {
                            level: 53,
                            species: Species::Ditto,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "CeruleanCave2F",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 51,
                            species: Species::Dodrio,
                        },
                        WildMon {
                            level: 51,
                            species: Species::Venomoth,
                        },
                        WildMon {
                            level: 51,
                            species: Species::Kadabra,
                        },
                        WildMon {
                            level: 52,
                            species: Species::Rhydon,
                        },
                        WildMon {
                            level: 52,
                            species: Species::Marowak,
                        },
                        WildMon {
                            level: 52,
                            species: Species::Electrode,
                        },
                        WildMon {
                            level: 56,
                            species: Species::Chansey,
                        },
                        WildMon {
                            level: 54,
                            species: Species::Wigglytuff,
                        },
                        WildMon {
                            level: 55,
                            species: Species::Ditto,
                        },
                        WildMon {
                            level: 60,
                            species: Species::Ditto,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "CeruleanCaveB1F",
                grass: WildEncounterTable {
                    encounter_rate: 25,
                    mons: vec![
                        WildMon {
                            level: 55,
                            species: Species::Rhydon,
                        },
                        WildMon {
                            level: 55,
                            species: Species::Marowak,
                        },
                        WildMon {
                            level: 55,
                            species: Species::Electrode,
                        },
                        WildMon {
                            level: 64,
                            species: Species::Chansey,
                        },
                        WildMon {
                            level: 64,
                            species: Species::Parasect,
                        },
                        WildMon {
                            level: 64,
                            species: Species::Raichu,
                        },
                        WildMon {
                            level: 57,
                            species: Species::Arbok,
                        },
                        WildMon {
                            level: 65,
                            species: Species::Ditto,
                        },
                        WildMon {
                            level: 63,
                            species: Species::Ditto,
                        },
                        WildMon {
                            level: 67,
                            species: Species::Ditto,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "DiglettsCave",
                grass: WildEncounterTable {
                    encounter_rate: 20,
                    mons: vec![
                        WildMon {
                            level: 18,
                            species: Species::Diglett,
                        },
                        WildMon {
                            level: 19,
                            species: Species::Diglett,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Diglett,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Diglett,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Diglett,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Diglett,
                        },
                        WildMon {
                            level: 21,
                            species: Species::Diglett,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Diglett,
                        },
                        WildMon {
                            level: 29,
                            species: Species::Dugtrio,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Dugtrio,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "MtMoon1F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 8,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 7,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 9,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 6,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Paras,
                        },
                        WildMon {
                            level: 11,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Clefairy,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "MtMoonB1F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 8,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 7,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 7,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 9,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Paras,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 11,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 9,
                            species: Species::Clefairy,
                        },
                        WildMon {
                            level: 9,
                            species: Species::Geodude,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "MtMoonB2F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 9,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 9,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 11,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Paras,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Paras,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Clefairy,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Clefairy,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "PokemonMansion1F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 32,
                            species: Species::Koffing,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Koffing,
                        },
                        WildMon {
                            level: 34,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 34,
                            species: Species::Growlithe,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Grimer,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 37,
                            species: Species::Weezing,
                        },
                        WildMon {
                            level: 39,
                            species: Species::Muk,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "PokemonMansion2F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 32,
                            species: Species::Growlithe,
                        },
                        WildMon {
                            level: 34,
                            species: Species::Koffing,
                        },
                        WildMon {
                            level: 34,
                            species: Species::Koffing,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Koffing,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Grimer,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 39,
                            species: Species::Weezing,
                        },
                        WildMon {
                            level: 37,
                            species: Species::Muk,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "PokemonMansion3F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 31,
                            species: Species::Koffing,
                        },
                        WildMon {
                            level: 33,
                            species: Species::Growlithe,
                        },
                        WildMon {
                            level: 35,
                            species: Species::Koffing,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 34,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 40,
                            species: Species::Weezing,
                        },
                        WildMon {
                            level: 34,
                            species: Species::Grimer,
                        },
                        WildMon {
                            level: 38,
                            species: Species::Weezing,
                        },
                        WildMon {
                            level: 36,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 42,
                            species: Species::Muk,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "PokemonMansionB1F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 33,
                            species: Species::Koffing,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Koffing,
                        },
                        WildMon {
                            level: 35,
                            species: Species::Growlithe,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Koffing,
                        },
                        WildMon {
                            level: 40,
                            species: Species::Weezing,
                        },
                        WildMon {
                            level: 34,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 35,
                            species: Species::Grimer,
                        },
                        WildMon {
                            level: 42,
                            species: Species::Weezing,
                        },
                        WildMon {
                            level: 42,
                            species: Species::Muk,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "PokemonTower3F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 20,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 21,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 19,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Cubone,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Cubone,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Haunter,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "PokemonTower4F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 20,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 21,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 19,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Haunter,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Cubone,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Cubone,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Gastly,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "PokemonTower5F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 20,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 21,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 19,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Haunter,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Cubone,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Cubone,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Gastly,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "PokemonTower6F",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 21,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 19,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Haunter,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Cubone,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Cubone,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Haunter,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "PokemonTower7F",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 21,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Haunter,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Cubone,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Cubone,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Haunter,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Haunter,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "PowerPlant",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 21,
                            species: Species::Voltorb,
                        },
                        WildMon {
                            level: 21,
                            species: Species::Magnemite,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Pikachu,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Pikachu,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Magnemite,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Voltorb,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Magneton,
                        },
                        WildMon {
                            level: 35,
                            species: Species::Magneton,
                        },
                        WildMon {
                            level: 33,
                            species: Species::Electabuzz,
                        },
                        WildMon {
                            level: 36,
                            species: Species::Electabuzz,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "RockTunnel1F",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 16,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Machop,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Machop,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Onix,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "RockTunnelB1F",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 16,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Machop,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Machop,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Geodude,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route1",
                grass: WildEncounterTable {
                    encounter_rate: 25,
                    mons: vec![
                        WildMon {
                            level: 3,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 2,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 2,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 4,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 4,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Pidgey,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route10",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 16,
                            species: Species::Voltorb,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 14,
                            species: Species::Voltorb,
                        },
                        WildMon {
                            level: 11,
                            species: Species::Ekans,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Ekans,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Voltorb,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Ekans,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Ekans,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route11",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 14,
                            species: Species::Ekans,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Ekans,
                        },
                        WildMon {
                            level: 9,
                            species: Species::Drowzee,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Drowzee,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Ekans,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 11,
                            species: Species::Drowzee,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Drowzee,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route12",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 24,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Venonat,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Venonat,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 27,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Gloom,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Gloom,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route13",
                grass: WildEncounterTable {
                    encounter_rate: 20,
                    mons: vec![
                        WildMon {
                            level: 24,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 27,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Venonat,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Venonat,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Ditto,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Gloom,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Gloom,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route14",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 24,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Ditto,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Venonat,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Venonat,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Gloom,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Pidgeotto,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Pidgeotto,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route15",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 24,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Ditto,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Venonat,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Venonat,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Gloom,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Pidgeotto,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Pidgeotto,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route16",
                grass: WildEncounterTable {
                    encounter_rate: 25,
                    mons: vec![
                        WildMon {
                            level: 20,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Raticate,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Raticate,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route17",
                grass: WildEncounterTable {
                    encounter_rate: 25,
                    mons: vec![
                        WildMon {
                            level: 20,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Raticate,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 27,
                            species: Species::Raticate,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 29,
                            species: Species::Raticate,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Fearow,
                        },
                        WildMon {
                            level: 27,
                            species: Species::Fearow,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route18",
                grass: WildEncounterTable {
                    encounter_rate: 25,
                    mons: vec![
                        WildMon {
                            level: 20,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Raticate,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Fearow,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 29,
                            species: Species::Raticate,
                        },
                        WildMon {
                            level: 27,
                            species: Species::Fearow,
                        },
                        WildMon {
                            level: 29,
                            species: Species::Fearow,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route2",
                grass: WildEncounterTable {
                    encounter_rate: 25,
                    mons: vec![
                        WildMon {
                            level: 3,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 4,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 4,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Weedle,
                        },
                        WildMon {
                            level: 2,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 4,
                            species: Species::Weedle,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Weedle,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route21",
                grass: WildEncounterTable {
                    encounter_rate: 25,
                    mons: vec![
                        WildMon {
                            level: 21,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Raticate,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 21,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Pidgeotto,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Pidgeotto,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Tangela,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Tangela,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Tangela,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 5,
                    mons: vec![
                        WildMon {
                            level: 5,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 35,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 40,
                            species: Species::Tentacool,
                        },
                    ],
                },
            },
            MapWildData {
                name: "Route22",
                grass: WildEncounterTable {
                    encounter_rate: 25,
                    mons: vec![
                        WildMon {
                            level: 3,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 3,
                            species: Species::NidoranM,
                        },
                        WildMon {
                            level: 4,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 4,
                            species: Species::NidoranM,
                        },
                        WildMon {
                            level: 2,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 2,
                            species: Species::NidoranM,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 3,
                            species: Species::NidoranF,
                        },
                        WildMon {
                            level: 4,
                            species: Species::NidoranF,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route23",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 26,
                            species: Species::Ekans,
                        },
                        WildMon {
                            level: 33,
                            species: Species::Ditto,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 38,
                            species: Species::Fearow,
                        },
                        WildMon {
                            level: 38,
                            species: Species::Ditto,
                        },
                        WildMon {
                            level: 38,
                            species: Species::Fearow,
                        },
                        WildMon {
                            level: 41,
                            species: Species::Arbok,
                        },
                        WildMon {
                            level: 43,
                            species: Species::Ditto,
                        },
                        WildMon {
                            level: 41,
                            species: Species::Fearow,
                        },
                        WildMon {
                            level: 43,
                            species: Species::Fearow,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route24",
                grass: WildEncounterTable {
                    encounter_rate: 25,
                    mons: vec![
                        WildMon {
                            level: 7,
                            species: Species::Weedle,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Kakuna,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Abra,
                        },
                        WildMon {
                            level: 14,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Abra,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Abra,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route25",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 8,
                            species: Species::Weedle,
                        },
                        WildMon {
                            level: 9,
                            species: Species::Kakuna,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Abra,
                        },
                        WildMon {
                            level: 14,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Abra,
                        },
                        WildMon {
                            level: 7,
                            species: Species::Metapod,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Caterpie,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route3",
                grass: WildEncounterTable {
                    encounter_rate: 20,
                    mons: vec![
                        WildMon {
                            level: 6,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 7,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 6,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 7,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Jigglypuff,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Jigglypuff,
                        },
                        WildMon {
                            level: 7,
                            species: Species::Jigglypuff,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route4",
                grass: WildEncounterTable {
                    encounter_rate: 20,
                    mons: vec![
                        WildMon {
                            level: 10,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 6,
                            species: Species::Ekans,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Ekans,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Ekans,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Ekans,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route5",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 13,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Mankey,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Mankey,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 14,
                            species: Species::Mankey,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Mankey,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route6",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 13,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Mankey,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Mankey,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 14,
                            species: Species::Mankey,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Mankey,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route7",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 19,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 19,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Mankey,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Oddish,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Mankey,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Growlithe,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Growlithe,
                        },
                        WildMon {
                            level: 19,
                            species: Species::Mankey,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Mankey,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route8",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 18,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Mankey,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Ekans,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Growlithe,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Mankey,
                        },
                        WildMon {
                            level: 19,
                            species: Species::Ekans,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Growlithe,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Growlithe,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Growlithe,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route9",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 16,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 14,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 11,
                            species: Species::Ekans,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Ekans,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Ekans,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Ekans,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "SafariZoneCenter",
                grass: WildEncounterTable {
                    encounter_rate: 30,
                    mons: vec![
                        WildMon {
                            level: 22,
                            species: Species::NidoranM,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Rhyhorn,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Venonat,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Exeggcute,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Nidorino,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Exeggcute,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Nidorina,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Parasect,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Scyther,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Chansey,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "SafariZoneEast",
                grass: WildEncounterTable {
                    encounter_rate: 30,
                    mons: vec![
                        WildMon {
                            level: 24,
                            species: Species::NidoranM,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Paras,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Exeggcute,
                        },
                        WildMon {
                            level: 33,
                            species: Species::Nidorino,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Exeggcute,
                        },
                        WildMon {
                            level: 24,
                            species: Species::NidoranF,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Parasect,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Kangaskhan,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Scyther,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "SafariZoneNorth",
                grass: WildEncounterTable {
                    encounter_rate: 30,
                    mons: vec![
                        WildMon {
                            level: 22,
                            species: Species::NidoranM,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Rhyhorn,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Paras,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Exeggcute,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Nidorino,
                        },
                        WildMon {
                            level: 27,
                            species: Species::Exeggcute,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Nidorina,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Venomoth,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Chansey,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Tauros,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "SafariZoneWest",
                grass: WildEncounterTable {
                    encounter_rate: 30,
                    mons: vec![
                        WildMon {
                            level: 25,
                            species: Species::NidoranM,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Venonat,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Exeggcute,
                        },
                        WildMon {
                            level: 33,
                            species: Species::Nidorino,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Exeggcute,
                        },
                        WildMon {
                            level: 25,
                            species: Species::NidoranF,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Venomoth,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Tauros,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Kangaskhan,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "SeaRoutes",
                grass: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
                water: WildEncounterTable {
                    encounter_rate: 5,
                    mons: vec![
                        WildMon {
                            level: 5,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 35,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 40,
                            species: Species::Tentacool,
                        },
                    ],
                },
            },
            MapWildData {
                name: "SeafoamIslands1F",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 30,
                            species: Species::Seel,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Slowpoke,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Shellder,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Horsea,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Horsea,
                        },
                        WildMon {
                            level: 21,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 29,
                            species: Species::Golbat,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Psyduck,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Shellder,
                        },
                        WildMon {
                            level: 38,
                            species: Species::Golduck,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "SeafoamIslandsB1F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 30,
                            species: Species::Staryu,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Horsea,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Shellder,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Horsea,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Slowpoke,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Seel,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Slowpoke,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Seel,
                        },
                        WildMon {
                            level: 38,
                            species: Species::Dewgong,
                        },
                        WildMon {
                            level: 37,
                            species: Species::Seadra,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "SeafoamIslandsB2F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 30,
                            species: Species::Seel,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Slowpoke,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Seel,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Slowpoke,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Horsea,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Staryu,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Horsea,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Shellder,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Golbat,
                        },
                        WildMon {
                            level: 37,
                            species: Species::Slowbro,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "SeafoamIslandsB3F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 31,
                            species: Species::Slowpoke,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Seel,
                        },
                        WildMon {
                            level: 33,
                            species: Species::Slowpoke,
                        },
                        WildMon {
                            level: 33,
                            species: Species::Seel,
                        },
                        WildMon {
                            level: 29,
                            species: Species::Horsea,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Shellder,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Horsea,
                        },
                        WildMon {
                            level: 29,
                            species: Species::Shellder,
                        },
                        WildMon {
                            level: 39,
                            species: Species::Seadra,
                        },
                        WildMon {
                            level: 37,
                            species: Species::Dewgong,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "SeafoamIslandsB4F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 31,
                            species: Species::Horsea,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Shellder,
                        },
                        WildMon {
                            level: 33,
                            species: Species::Horsea,
                        },
                        WildMon {
                            level: 33,
                            species: Species::Shellder,
                        },
                        WildMon {
                            level: 29,
                            species: Species::Slowpoke,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Seel,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Slowpoke,
                        },
                        WildMon {
                            level: 29,
                            species: Species::Seel,
                        },
                        WildMon {
                            level: 39,
                            species: Species::Slowbro,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Golbat,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "VictoryRoad1F",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 24,
                            species: Species::Machop,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 36,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 39,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 42,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 41,
                            species: Species::Graveler,
                        },
                        WildMon {
                            level: 41,
                            species: Species::Golbat,
                        },
                        WildMon {
                            level: 42,
                            species: Species::Machoke,
                        },
                        WildMon {
                            level: 43,
                            species: Species::Marowak,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "VictoryRoad2F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 22,
                            species: Species::Machop,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 36,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 39,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 42,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 41,
                            species: Species::Machoke,
                        },
                        WildMon {
                            level: 40,
                            species: Species::Golbat,
                        },
                        WildMon {
                            level: 40,
                            species: Species::Marowak,
                        },
                        WildMon {
                            level: 43,
                            species: Species::Graveler,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "VictoryRoad3F",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 24,
                            species: Species::Machop,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 42,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 40,
                            species: Species::Venomoth,
                        },
                        WildMon {
                            level: 45,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 43,
                            species: Species::Graveler,
                        },
                        WildMon {
                            level: 41,
                            species: Species::Golbat,
                        },
                        WildMon {
                            level: 42,
                            species: Species::Machoke,
                        },
                        WildMon {
                            level: 45,
                            species: Species::Machoke,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "ViridianForest",
                grass: WildEncounterTable {
                    encounter_rate: 8,
                    mons: vec![
                        WildMon {
                            level: 4,
                            species: Species::Weedle,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Kakuna,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Weedle,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Weedle,
                        },
                        WildMon {
                            level: 4,
                            species: Species::Kakuna,
                        },
                        WildMon {
                            level: 6,
                            species: Species::Kakuna,
                        },
                        WildMon {
                            level: 4,
                            species: Species::Metapod,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Caterpie,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Pikachu,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Pikachu,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
        ],
        GameVersion::Blue => vec![
            MapWildData {
                name: "CeruleanCave1F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 46,
                            species: Species::Golbat,
                        },
                        WildMon {
                            level: 46,
                            species: Species::Hypno,
                        },
                        WildMon {
                            level: 46,
                            species: Species::Magneton,
                        },
                        WildMon {
                            level: 49,
                            species: Species::Dodrio,
                        },
                        WildMon {
                            level: 49,
                            species: Species::Venomoth,
                        },
                        WildMon {
                            level: 52,
                            species: Species::Sandslash,
                        },
                        WildMon {
                            level: 49,
                            species: Species::Kadabra,
                        },
                        WildMon {
                            level: 52,
                            species: Species::Parasect,
                        },
                        WildMon {
                            level: 53,
                            species: Species::Raichu,
                        },
                        WildMon {
                            level: 53,
                            species: Species::Ditto,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "CeruleanCave2F",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 51,
                            species: Species::Dodrio,
                        },
                        WildMon {
                            level: 51,
                            species: Species::Venomoth,
                        },
                        WildMon {
                            level: 51,
                            species: Species::Kadabra,
                        },
                        WildMon {
                            level: 52,
                            species: Species::Rhydon,
                        },
                        WildMon {
                            level: 52,
                            species: Species::Marowak,
                        },
                        WildMon {
                            level: 52,
                            species: Species::Electrode,
                        },
                        WildMon {
                            level: 56,
                            species: Species::Chansey,
                        },
                        WildMon {
                            level: 54,
                            species: Species::Wigglytuff,
                        },
                        WildMon {
                            level: 55,
                            species: Species::Ditto,
                        },
                        WildMon {
                            level: 60,
                            species: Species::Ditto,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "CeruleanCaveB1F",
                grass: WildEncounterTable {
                    encounter_rate: 25,
                    mons: vec![
                        WildMon {
                            level: 55,
                            species: Species::Rhydon,
                        },
                        WildMon {
                            level: 55,
                            species: Species::Marowak,
                        },
                        WildMon {
                            level: 55,
                            species: Species::Electrode,
                        },
                        WildMon {
                            level: 64,
                            species: Species::Chansey,
                        },
                        WildMon {
                            level: 64,
                            species: Species::Parasect,
                        },
                        WildMon {
                            level: 64,
                            species: Species::Raichu,
                        },
                        WildMon {
                            level: 57,
                            species: Species::Sandslash,
                        },
                        WildMon {
                            level: 65,
                            species: Species::Ditto,
                        },
                        WildMon {
                            level: 63,
                            species: Species::Ditto,
                        },
                        WildMon {
                            level: 67,
                            species: Species::Ditto,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "DiglettsCave",
                grass: WildEncounterTable {
                    encounter_rate: 20,
                    mons: vec![
                        WildMon {
                            level: 18,
                            species: Species::Diglett,
                        },
                        WildMon {
                            level: 19,
                            species: Species::Diglett,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Diglett,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Diglett,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Diglett,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Diglett,
                        },
                        WildMon {
                            level: 21,
                            species: Species::Diglett,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Diglett,
                        },
                        WildMon {
                            level: 29,
                            species: Species::Dugtrio,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Dugtrio,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "MtMoon1F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 8,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 7,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 9,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 6,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Paras,
                        },
                        WildMon {
                            level: 11,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Clefairy,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "MtMoonB1F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 8,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 7,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 7,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 9,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Paras,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 11,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 9,
                            species: Species::Clefairy,
                        },
                        WildMon {
                            level: 9,
                            species: Species::Geodude,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "MtMoonB2F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 9,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 9,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 11,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Paras,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Paras,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Clefairy,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Clefairy,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "PokemonMansion1F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 32,
                            species: Species::Grimer,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Grimer,
                        },
                        WildMon {
                            level: 34,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 34,
                            species: Species::Vulpix,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Koffing,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 37,
                            species: Species::Muk,
                        },
                        WildMon {
                            level: 39,
                            species: Species::Weezing,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "PokemonMansion2F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 32,
                            species: Species::Vulpix,
                        },
                        WildMon {
                            level: 34,
                            species: Species::Grimer,
                        },
                        WildMon {
                            level: 34,
                            species: Species::Grimer,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Grimer,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Koffing,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 39,
                            species: Species::Muk,
                        },
                        WildMon {
                            level: 37,
                            species: Species::Weezing,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "PokemonMansion3F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 31,
                            species: Species::Grimer,
                        },
                        WildMon {
                            level: 33,
                            species: Species::Vulpix,
                        },
                        WildMon {
                            level: 35,
                            species: Species::Grimer,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 34,
                            species: Species::Magmar,
                        },
                        WildMon {
                            level: 40,
                            species: Species::Muk,
                        },
                        WildMon {
                            level: 34,
                            species: Species::Koffing,
                        },
                        WildMon {
                            level: 38,
                            species: Species::Muk,
                        },
                        WildMon {
                            level: 36,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 42,
                            species: Species::Weezing,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "PokemonMansionB1F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 33,
                            species: Species::Grimer,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Grimer,
                        },
                        WildMon {
                            level: 35,
                            species: Species::Vulpix,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Grimer,
                        },
                        WildMon {
                            level: 40,
                            species: Species::Muk,
                        },
                        WildMon {
                            level: 34,
                            species: Species::Ponyta,
                        },
                        WildMon {
                            level: 35,
                            species: Species::Koffing,
                        },
                        WildMon {
                            level: 38,
                            species: Species::Magmar,
                        },
                        WildMon {
                            level: 42,
                            species: Species::Weezing,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "PokemonTower3F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 20,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 21,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 19,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Cubone,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Cubone,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Haunter,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "PokemonTower4F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 20,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 21,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 19,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Haunter,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Cubone,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Cubone,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Gastly,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "PokemonTower5F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 20,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 21,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 19,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Haunter,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Cubone,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Cubone,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Gastly,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "PokemonTower6F",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 21,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 19,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Haunter,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Cubone,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Cubone,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Haunter,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "PokemonTower7F",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 21,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Gastly,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Haunter,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Cubone,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Cubone,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Haunter,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Haunter,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "PowerPlant",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 21,
                            species: Species::Voltorb,
                        },
                        WildMon {
                            level: 21,
                            species: Species::Magnemite,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Pikachu,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Pikachu,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Magnemite,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Voltorb,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Magneton,
                        },
                        WildMon {
                            level: 35,
                            species: Species::Magneton,
                        },
                        WildMon {
                            level: 33,
                            species: Species::Raichu,
                        },
                        WildMon {
                            level: 36,
                            species: Species::Raichu,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "RockTunnel1F",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 16,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Machop,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Machop,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Onix,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "RockTunnelB1F",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 16,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Machop,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Machop,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Geodude,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route1",
                grass: WildEncounterTable {
                    encounter_rate: 25,
                    mons: vec![
                        WildMon {
                            level: 3,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 2,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 2,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 4,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 4,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Pidgey,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route10",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 16,
                            species: Species::Voltorb,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 14,
                            species: Species::Voltorb,
                        },
                        WildMon {
                            level: 11,
                            species: Species::Sandshrew,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Sandshrew,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Voltorb,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Sandshrew,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Sandshrew,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route11",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 14,
                            species: Species::Sandshrew,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Sandshrew,
                        },
                        WildMon {
                            level: 9,
                            species: Species::Drowzee,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Drowzee,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Sandshrew,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 11,
                            species: Species::Drowzee,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Drowzee,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route12",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 24,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Venonat,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Venonat,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 27,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Weepinbell,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Weepinbell,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route13",
                grass: WildEncounterTable {
                    encounter_rate: 20,
                    mons: vec![
                        WildMon {
                            level: 24,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 27,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Venonat,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Venonat,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Ditto,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Weepinbell,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Weepinbell,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route14",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 24,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Ditto,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Venonat,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Venonat,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Weepinbell,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Pidgeotto,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Pidgeotto,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route15",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 24,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Ditto,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Venonat,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Venonat,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Weepinbell,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Pidgeotto,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Pidgeotto,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route16",
                grass: WildEncounterTable {
                    encounter_rate: 25,
                    mons: vec![
                        WildMon {
                            level: 20,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Raticate,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Raticate,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route17",
                grass: WildEncounterTable {
                    encounter_rate: 25,
                    mons: vec![
                        WildMon {
                            level: 20,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Raticate,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 27,
                            species: Species::Raticate,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 29,
                            species: Species::Raticate,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Fearow,
                        },
                        WildMon {
                            level: 27,
                            species: Species::Fearow,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route18",
                grass: WildEncounterTable {
                    encounter_rate: 25,
                    mons: vec![
                        WildMon {
                            level: 20,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Raticate,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Fearow,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 29,
                            species: Species::Raticate,
                        },
                        WildMon {
                            level: 27,
                            species: Species::Fearow,
                        },
                        WildMon {
                            level: 29,
                            species: Species::Fearow,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route2",
                grass: WildEncounterTable {
                    encounter_rate: 25,
                    mons: vec![
                        WildMon {
                            level: 3,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 4,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 4,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Caterpie,
                        },
                        WildMon {
                            level: 2,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 4,
                            species: Species::Caterpie,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Caterpie,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route21",
                grass: WildEncounterTable {
                    encounter_rate: 25,
                    mons: vec![
                        WildMon {
                            level: 21,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Raticate,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 21,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Pidgeotto,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Pidgeotto,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Tangela,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Tangela,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Tangela,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 5,
                    mons: vec![
                        WildMon {
                            level: 5,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 35,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 40,
                            species: Species::Tentacool,
                        },
                    ],
                },
            },
            MapWildData {
                name: "Route22",
                grass: WildEncounterTable {
                    encounter_rate: 25,
                    mons: vec![
                        WildMon {
                            level: 3,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 3,
                            species: Species::NidoranF,
                        },
                        WildMon {
                            level: 4,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 4,
                            species: Species::NidoranF,
                        },
                        WildMon {
                            level: 2,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 2,
                            species: Species::NidoranF,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 3,
                            species: Species::NidoranM,
                        },
                        WildMon {
                            level: 4,
                            species: Species::NidoranM,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route23",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 26,
                            species: Species::Sandshrew,
                        },
                        WildMon {
                            level: 33,
                            species: Species::Ditto,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 38,
                            species: Species::Fearow,
                        },
                        WildMon {
                            level: 38,
                            species: Species::Ditto,
                        },
                        WildMon {
                            level: 38,
                            species: Species::Fearow,
                        },
                        WildMon {
                            level: 41,
                            species: Species::Sandslash,
                        },
                        WildMon {
                            level: 43,
                            species: Species::Ditto,
                        },
                        WildMon {
                            level: 41,
                            species: Species::Fearow,
                        },
                        WildMon {
                            level: 43,
                            species: Species::Fearow,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route24",
                grass: WildEncounterTable {
                    encounter_rate: 25,
                    mons: vec![
                        WildMon {
                            level: 7,
                            species: Species::Caterpie,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Metapod,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Abra,
                        },
                        WildMon {
                            level: 14,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Abra,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Abra,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route25",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 8,
                            species: Species::Caterpie,
                        },
                        WildMon {
                            level: 9,
                            species: Species::Metapod,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Abra,
                        },
                        WildMon {
                            level: 14,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Abra,
                        },
                        WildMon {
                            level: 7,
                            species: Species::Kakuna,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Weedle,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route3",
                grass: WildEncounterTable {
                    encounter_rate: 20,
                    mons: vec![
                        WildMon {
                            level: 6,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 7,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 6,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 7,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Jigglypuff,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Jigglypuff,
                        },
                        WildMon {
                            level: 7,
                            species: Species::Jigglypuff,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route4",
                grass: WildEncounterTable {
                    encounter_rate: 20,
                    mons: vec![
                        WildMon {
                            level: 10,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 6,
                            species: Species::Sandshrew,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Sandshrew,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 8,
                            species: Species::Sandshrew,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Sandshrew,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route5",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 13,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Meowth,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Meowth,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 14,
                            species: Species::Meowth,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Meowth,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route6",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 13,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Meowth,
                        },
                        WildMon {
                            level: 12,
                            species: Species::Meowth,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 14,
                            species: Species::Meowth,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Meowth,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route7",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 19,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 19,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Meowth,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Bellsprout,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Meowth,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Vulpix,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Vulpix,
                        },
                        WildMon {
                            level: 19,
                            species: Species::Meowth,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Meowth,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route8",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 18,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Meowth,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Sandshrew,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Vulpix,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Pidgey,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Meowth,
                        },
                        WildMon {
                            level: 19,
                            species: Species::Sandshrew,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Vulpix,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Vulpix,
                        },
                        WildMon {
                            level: 18,
                            species: Species::Vulpix,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "Route9",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 16,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 16,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 14,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 11,
                            species: Species::Sandshrew,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Sandshrew,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Rattata,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Spearow,
                        },
                        WildMon {
                            level: 13,
                            species: Species::Sandshrew,
                        },
                        WildMon {
                            level: 17,
                            species: Species::Sandshrew,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "SafariZoneCenter",
                grass: WildEncounterTable {
                    encounter_rate: 30,
                    mons: vec![
                        WildMon {
                            level: 22,
                            species: Species::NidoranF,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Rhyhorn,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Venonat,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Exeggcute,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Nidorina,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Exeggcute,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Nidorino,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Parasect,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Pinsir,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Chansey,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "SafariZoneEast",
                grass: WildEncounterTable {
                    encounter_rate: 30,
                    mons: vec![
                        WildMon {
                            level: 24,
                            species: Species::NidoranF,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Paras,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Exeggcute,
                        },
                        WildMon {
                            level: 33,
                            species: Species::Nidorina,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Exeggcute,
                        },
                        WildMon {
                            level: 24,
                            species: Species::NidoranM,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Parasect,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Kangaskhan,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Pinsir,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "SafariZoneNorth",
                grass: WildEncounterTable {
                    encounter_rate: 30,
                    mons: vec![
                        WildMon {
                            level: 22,
                            species: Species::NidoranF,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Rhyhorn,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Paras,
                        },
                        WildMon {
                            level: 25,
                            species: Species::Exeggcute,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Nidorina,
                        },
                        WildMon {
                            level: 27,
                            species: Species::Exeggcute,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Nidorino,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Venomoth,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Chansey,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Tauros,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "SafariZoneWest",
                grass: WildEncounterTable {
                    encounter_rate: 30,
                    mons: vec![
                        WildMon {
                            level: 25,
                            species: Species::NidoranF,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Doduo,
                        },
                        WildMon {
                            level: 23,
                            species: Species::Venonat,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Exeggcute,
                        },
                        WildMon {
                            level: 33,
                            species: Species::Nidorina,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Exeggcute,
                        },
                        WildMon {
                            level: 25,
                            species: Species::NidoranM,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Venomoth,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Tauros,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Kangaskhan,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "SeaRoutes",
                grass: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
                water: WildEncounterTable {
                    encounter_rate: 5,
                    mons: vec![
                        WildMon {
                            level: 5,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 10,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 15,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 20,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 35,
                            species: Species::Tentacool,
                        },
                        WildMon {
                            level: 40,
                            species: Species::Tentacool,
                        },
                    ],
                },
            },
            MapWildData {
                name: "SeafoamIslands1F",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 30,
                            species: Species::Seel,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Psyduck,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Staryu,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Krabby,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Krabby,
                        },
                        WildMon {
                            level: 21,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 29,
                            species: Species::Golbat,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Slowpoke,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Staryu,
                        },
                        WildMon {
                            level: 38,
                            species: Species::Slowbro,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "SeafoamIslandsB1F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 30,
                            species: Species::Shellder,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Krabby,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Staryu,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Krabby,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Psyduck,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Seel,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Psyduck,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Seel,
                        },
                        WildMon {
                            level: 38,
                            species: Species::Dewgong,
                        },
                        WildMon {
                            level: 37,
                            species: Species::Kingler,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "SeafoamIslandsB2F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 30,
                            species: Species::Seel,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Psyduck,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Seel,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Psyduck,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Krabby,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Shellder,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Krabby,
                        },
                        WildMon {
                            level: 28,
                            species: Species::Staryu,
                        },
                        WildMon {
                            level: 30,
                            species: Species::Golbat,
                        },
                        WildMon {
                            level: 37,
                            species: Species::Golduck,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "SeafoamIslandsB3F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 31,
                            species: Species::Psyduck,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Seel,
                        },
                        WildMon {
                            level: 33,
                            species: Species::Psyduck,
                        },
                        WildMon {
                            level: 33,
                            species: Species::Seel,
                        },
                        WildMon {
                            level: 29,
                            species: Species::Krabby,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Staryu,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Krabby,
                        },
                        WildMon {
                            level: 29,
                            species: Species::Staryu,
                        },
                        WildMon {
                            level: 39,
                            species: Species::Kingler,
                        },
                        WildMon {
                            level: 37,
                            species: Species::Dewgong,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "SeafoamIslandsB4F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 31,
                            species: Species::Krabby,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Staryu,
                        },
                        WildMon {
                            level: 33,
                            species: Species::Krabby,
                        },
                        WildMon {
                            level: 33,
                            species: Species::Staryu,
                        },
                        WildMon {
                            level: 29,
                            species: Species::Psyduck,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Seel,
                        },
                        WildMon {
                            level: 31,
                            species: Species::Psyduck,
                        },
                        WildMon {
                            level: 29,
                            species: Species::Seel,
                        },
                        WildMon {
                            level: 39,
                            species: Species::Golduck,
                        },
                        WildMon {
                            level: 32,
                            species: Species::Golbat,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "VictoryRoad1F",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 24,
                            species: Species::Machop,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 36,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 39,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 42,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 41,
                            species: Species::Graveler,
                        },
                        WildMon {
                            level: 41,
                            species: Species::Golbat,
                        },
                        WildMon {
                            level: 42,
                            species: Species::Machoke,
                        },
                        WildMon {
                            level: 43,
                            species: Species::Marowak,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "VictoryRoad2F",
                grass: WildEncounterTable {
                    encounter_rate: 10,
                    mons: vec![
                        WildMon {
                            level: 22,
                            species: Species::Machop,
                        },
                        WildMon {
                            level: 24,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 36,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 39,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 42,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 41,
                            species: Species::Machoke,
                        },
                        WildMon {
                            level: 40,
                            species: Species::Golbat,
                        },
                        WildMon {
                            level: 40,
                            species: Species::Marowak,
                        },
                        WildMon {
                            level: 43,
                            species: Species::Graveler,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "VictoryRoad3F",
                grass: WildEncounterTable {
                    encounter_rate: 15,
                    mons: vec![
                        WildMon {
                            level: 24,
                            species: Species::Machop,
                        },
                        WildMon {
                            level: 26,
                            species: Species::Geodude,
                        },
                        WildMon {
                            level: 22,
                            species: Species::Zubat,
                        },
                        WildMon {
                            level: 42,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 40,
                            species: Species::Venomoth,
                        },
                        WildMon {
                            level: 45,
                            species: Species::Onix,
                        },
                        WildMon {
                            level: 43,
                            species: Species::Graveler,
                        },
                        WildMon {
                            level: 41,
                            species: Species::Golbat,
                        },
                        WildMon {
                            level: 42,
                            species: Species::Machoke,
                        },
                        WildMon {
                            level: 45,
                            species: Species::Machoke,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
            MapWildData {
                name: "ViridianForest",
                grass: WildEncounterTable {
                    encounter_rate: 8,
                    mons: vec![
                        WildMon {
                            level: 4,
                            species: Species::Caterpie,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Metapod,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Caterpie,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Caterpie,
                        },
                        WildMon {
                            level: 4,
                            species: Species::Metapod,
                        },
                        WildMon {
                            level: 6,
                            species: Species::Metapod,
                        },
                        WildMon {
                            level: 4,
                            species: Species::Kakuna,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Weedle,
                        },
                        WildMon {
                            level: 3,
                            species: Species::Pikachu,
                        },
                        WildMon {
                            level: 5,
                            species: Species::Pikachu,
                        },
                    ],
                },
                water: WildEncounterTable {
                    encounter_rate: 0,
                    mons: vec![],
                },
            },
        ],
    }
}

/// Good Rod encounters (same for all maps, random choice of these)
pub fn good_rod_data() -> Vec<WildMon> {
    vec![
        WildMon {
            level: 10,
            species: Species::Goldeen,
        },
        WildMon {
            level: 10,
            species: Species::Poliwag,
        },
    ]
}

/// Super Rod fishing groups
pub fn super_rod_groups() -> Vec<FishingGroup> {
    vec![
        // Group1
        FishingGroup {
            mons: vec![
                WildMon {
                    level: 15,
                    species: Species::Tentacool,
                },
                WildMon {
                    level: 15,
                    species: Species::Poliwag,
                },
            ],
        },
        // Group2
        FishingGroup {
            mons: vec![
                WildMon {
                    level: 15,
                    species: Species::Goldeen,
                },
                WildMon {
                    level: 15,
                    species: Species::Poliwag,
                },
            ],
        },
        // Group3
        FishingGroup {
            mons: vec![
                WildMon {
                    level: 15,
                    species: Species::Psyduck,
                },
                WildMon {
                    level: 15,
                    species: Species::Goldeen,
                },
                WildMon {
                    level: 15,
                    species: Species::Krabby,
                },
            ],
        },
        // Group4
        FishingGroup {
            mons: vec![
                WildMon {
                    level: 15,
                    species: Species::Krabby,
                },
                WildMon {
                    level: 15,
                    species: Species::Shellder,
                },
            ],
        },
        // Group5
        FishingGroup {
            mons: vec![
                WildMon {
                    level: 23,
                    species: Species::Poliwhirl,
                },
                WildMon {
                    level: 15,
                    species: Species::Slowpoke,
                },
            ],
        },
        // Group6
        FishingGroup {
            mons: vec![
                WildMon {
                    level: 15,
                    species: Species::Dratini,
                },
                WildMon {
                    level: 15,
                    species: Species::Krabby,
                },
                WildMon {
                    level: 15,
                    species: Species::Psyduck,
                },
                WildMon {
                    level: 15,
                    species: Species::Slowpoke,
                },
            ],
        },
        // Group7
        FishingGroup {
            mons: vec![
                WildMon {
                    level: 5,
                    species: Species::Tentacool,
                },
                WildMon {
                    level: 15,
                    species: Species::Krabby,
                },
                WildMon {
                    level: 15,
                    species: Species::Goldeen,
                },
                WildMon {
                    level: 15,
                    species: Species::Magikarp,
                },
            ],
        },
        // Group8
        FishingGroup {
            mons: vec![
                WildMon {
                    level: 15,
                    species: Species::Staryu,
                },
                WildMon {
                    level: 15,
                    species: Species::Horsea,
                },
                WildMon {
                    level: 15,
                    species: Species::Shellder,
                },
                WildMon {
                    level: 15,
                    species: Species::Goldeen,
                },
            ],
        },
        // Group9
        FishingGroup {
            mons: vec![
                WildMon {
                    level: 23,
                    species: Species::Slowbro,
                },
                WildMon {
                    level: 23,
                    species: Species::Seaking,
                },
                WildMon {
                    level: 23,
                    species: Species::Kingler,
                },
                WildMon {
                    level: 23,
                    species: Species::Seadra,
                },
            ],
        },
        // Group10
        FishingGroup {
            mons: vec![
                WildMon {
                    level: 23,
                    species: Species::Seaking,
                },
                WildMon {
                    level: 15,
                    species: Species::Krabby,
                },
                WildMon {
                    level: 15,
                    species: Species::Goldeen,
                },
                WildMon {
                    level: 15,
                    species: Species::Magikarp,
                },
            ],
        },
    ]
}

/// Super Rod map-to-group mappings
pub fn super_rod_map_entries() -> Vec<SuperRodMapEntry> {
    vec![
        SuperRodMapEntry {
            map_name: "PALLET_TOWN",
            group_index: 0,
        },
        SuperRodMapEntry {
            map_name: "VIRIDIAN_CITY",
            group_index: 0,
        },
        SuperRodMapEntry {
            map_name: "CERULEAN_CITY",
            group_index: 2,
        },
        SuperRodMapEntry {
            map_name: "VERMILION_CITY",
            group_index: 3,
        },
        SuperRodMapEntry {
            map_name: "CELADON_CITY",
            group_index: 4,
        },
        SuperRodMapEntry {
            map_name: "FUCHSIA_CITY",
            group_index: 9,
        },
        SuperRodMapEntry {
            map_name: "CINNABAR_ISLAND",
            group_index: 7,
        },
        SuperRodMapEntry {
            map_name: "ROUTE_4",
            group_index: 2,
        },
        SuperRodMapEntry {
            map_name: "ROUTE_6",
            group_index: 3,
        },
        SuperRodMapEntry {
            map_name: "ROUTE_10",
            group_index: 4,
        },
        SuperRodMapEntry {
            map_name: "ROUTE_11",
            group_index: 3,
        },
        SuperRodMapEntry {
            map_name: "ROUTE_12",
            group_index: 6,
        },
        SuperRodMapEntry {
            map_name: "ROUTE_13",
            group_index: 6,
        },
        SuperRodMapEntry {
            map_name: "ROUTE_17",
            group_index: 6,
        },
        SuperRodMapEntry {
            map_name: "ROUTE_18",
            group_index: 6,
        },
        SuperRodMapEntry {
            map_name: "ROUTE_19",
            group_index: 7,
        },
        SuperRodMapEntry {
            map_name: "ROUTE_20",
            group_index: 7,
        },
        SuperRodMapEntry {
            map_name: "ROUTE_21",
            group_index: 7,
        },
        SuperRodMapEntry {
            map_name: "ROUTE_22",
            group_index: 1,
        },
        SuperRodMapEntry {
            map_name: "ROUTE_23",
            group_index: 8,
        },
        SuperRodMapEntry {
            map_name: "ROUTE_24",
            group_index: 2,
        },
        SuperRodMapEntry {
            map_name: "ROUTE_25",
            group_index: 2,
        },
        SuperRodMapEntry {
            map_name: "CERULEAN_GYM",
            group_index: 2,
        },
        SuperRodMapEntry {
            map_name: "VERMILION_DOCK",
            group_index: 3,
        },
        SuperRodMapEntry {
            map_name: "SEAFOAM_ISLANDS_B3F",
            group_index: 7,
        },
        SuperRodMapEntry {
            map_name: "SEAFOAM_ISLANDS_B4F",
            group_index: 7,
        },
        SuperRodMapEntry {
            map_name: "SAFARI_ZONE_EAST",
            group_index: 5,
        },
        SuperRodMapEntry {
            map_name: "SAFARI_ZONE_NORTH",
            group_index: 5,
        },
        SuperRodMapEntry {
            map_name: "SAFARI_ZONE_WEST",
            group_index: 5,
        },
        SuperRodMapEntry {
            map_name: "SAFARI_ZONE_CENTER",
            group_index: 5,
        },
        SuperRodMapEntry {
            map_name: "CERULEAN_CAVE_2F",
            group_index: 8,
        },
        SuperRodMapEntry {
            map_name: "CERULEAN_CAVE_B1F",
            group_index: 8,
        },
        SuperRodMapEntry {
            map_name: "CERULEAN_CAVE_1F",
            group_index: 8,
        },
    ]
}
