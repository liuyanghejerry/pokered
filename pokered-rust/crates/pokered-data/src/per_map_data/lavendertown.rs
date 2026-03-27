use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 3] = [
NpcEntry {
        sprite_id: 0x08,
        x: 15,
        y: 9,
        movement: NpcMovement(1),
        facing: NpcFacing(0),
        range: 0,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x07,
        x: 9,
        y: 10,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0C,
        x: 8,
        y: 7,
        movement: NpcMovement(1),
        facing: NpcFacing(2),
        range: 2,
        text_id: 3,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 6] = [
SignEntry {
        x: 11,
        y: 9,
        text_id: 1,
    },
    SignEntry {
        x: 9,
        y: 3,
        text_id: 2,
    },
    SignEntry {
        x: 16,
        y: 13,
        text_id: 3,
    },
    SignEntry {
        x: 4,
        y: 5,
        text_id: 4,
    },
    SignEntry {
        x: 5,
        y: 9,
        text_id: 5,
    },
    SignEntry {
        x: 17,
        y: 7,
        text_id: 6,
    },
];

pub const WARPS: [WarpData; 6] = [
WarpData {
        x: 3,
        y: 5,
        dest_map: Some(MapId::LavenderPokecenter),
        dest_warp_id: 0,
    },
    WarpData {
        x: 14,
        y: 5,
        dest_map: Some(MapId::PokemonTower1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 7,
        y: 9,
        dest_map: Some(MapId::MrFujisHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 15,
        y: 13,
        dest_map: Some(MapId::LavenderMart),
        dest_warp_id: 0,
    },
    WarpData {
        x: 3,
        y: 13,
        dest_map: Some(MapId::LavenderCuboneHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 7,
        y: 13,
        dest_map: Some(MapId::NameRatersHouse),
        dest_warp_id: 0,
    },
];