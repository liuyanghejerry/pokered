use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 2] = [
NpcEntry {
        sprite_id: 0x3F,
        x: 18,
        y: 10,
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
        x: 26,
        y: 7,
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

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 7] = [
WarpData {
        x: 4,
        y: 17,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 17,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 26,
        y: 17,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 27,
        y: 17,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 7,
        y: 5,
        dest_map: Some(MapId::SeafoamIslandsB1F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 25,
        y: 3,
        dest_map: Some(MapId::SeafoamIslandsB1F),
        dest_warp_id: 6,
    },
    WarpData {
        x: 23,
        y: 15,
        dest_map: Some(MapId::SeafoamIslandsB1F),
        dest_warp_id: 4,
    },
];