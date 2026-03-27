use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 3] = [
NpcEntry {
        sprite_id: 0x3F,
        x: 4,
        y: 15,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3F,
        x: 5,
        y: 15,
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
        sprite_id: 0x09,
        x: 6,
        y: 1,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 3,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 2] = [
SignEntry {
        x: 9,
        y: 15,
        text_id: 1,
    },
    SignEntry {
        x: 23,
        y: 1,
        text_id: 2,
    },
];

pub const WARPS: [WarpData; 4] = [
WarpData {
        x: 20,
        y: 17,
        dest_map: Some(MapId::SeafoamIslandsB3F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 21,
        y: 17,
        dest_map: Some(MapId::SeafoamIslandsB3F),
        dest_warp_id: 6,
    },
    WarpData {
        x: 11,
        y: 7,
        dest_map: Some(MapId::SeafoamIslandsB3F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 25,
        y: 4,
        dest_map: Some(MapId::SeafoamIslandsB3F),
        dest_warp_id: 2,
    },
];