use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 3] = [
NpcEntry {
        sprite_id: 0x20,
        x: 17,
        y: 17,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 1,
        is_trainer: true,
        trainer_class: 28,
        trainer_set: 4,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 14,
        y: 3,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x1D,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 18,
        y: 21,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 3,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x26,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 8] = [
WarpData {
        x: 4,
        y: 27,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 27,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 6,
        y: 27,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 7,
        y: 27,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 10,
        dest_map: Some(MapId::PokemonMansion2F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 21,
        y: 23,
        dest_map: Some(MapId::PokemonMansionB1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 26,
        y: 27,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 27,
        y: 27,
        dest_map: None,
        dest_warp_id: 0,
    },
];