use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 2] = [
NpcEntry {
        sprite_id: 0x3F,
        x: 18,
        y: 6,
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
        x: 23,
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

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 7] = [
WarpData {
        x: 5,
        y: 3,
        dest_map: Some(MapId::SeafoamIslandsB1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 13,
        dest_map: Some(MapId::SeafoamIslandsB3F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 13,
        y: 7,
        dest_map: Some(MapId::SeafoamIslandsB1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 19,
        y: 15,
        dest_map: Some(MapId::SeafoamIslandsB1F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 25,
        y: 3,
        dest_map: Some(MapId::SeafoamIslandsB3F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 25,
        y: 11,
        dest_map: Some(MapId::SeafoamIslandsB1F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 25,
        y: 14,
        dest_map: Some(MapId::SeafoamIslandsB3F),
        dest_warp_id: 4,
    },
];