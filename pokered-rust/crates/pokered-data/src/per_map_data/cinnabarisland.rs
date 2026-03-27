use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 2] = [
NpcEntry {
        sprite_id: 0x0D,
        x: 12,
        y: 5,
        movement: NpcMovement(1),
        facing: NpcFacing(2),
        range: 2,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0B,
        x: 14,
        y: 6,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 5] = [
SignEntry {
        x: 9,
        y: 5,
        text_id: 1,
    },
    SignEntry {
        x: 16,
        y: 11,
        text_id: 2,
    },
    SignEntry {
        x: 12,
        y: 11,
        text_id: 3,
    },
    SignEntry {
        x: 9,
        y: 11,
        text_id: 4,
    },
    SignEntry {
        x: 13,
        y: 3,
        text_id: 5,
    },
];

pub const WARPS: [WarpData; 5] = [
WarpData {
        x: 6,
        y: 3,
        dest_map: Some(MapId::PokemonMansion1F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 18,
        y: 3,
        dest_map: Some(MapId::CinnabarGym),
        dest_warp_id: 0,
    },
    WarpData {
        x: 6,
        y: 9,
        dest_map: Some(MapId::CinnabarLab),
        dest_warp_id: 0,
    },
    WarpData {
        x: 11,
        y: 11,
        dest_map: Some(MapId::CinnabarPokecenter),
        dest_warp_id: 0,
    },
    WarpData {
        x: 15,
        y: 11,
        dest_map: Some(MapId::CinnabarMart),
        dest_warp_id: 0,
    },
];