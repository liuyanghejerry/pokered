use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 2] = [
NpcEntry {
        sprite_id: 0x20,
        x: 1,
        y: 4,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x04,
        x: 5,
        y: 4,
        movement: NpcMovement(1),
        facing: NpcFacing(2),
        range: 2,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 4] = [
WarpData {
        x: 4,
        y: 0,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 5,
        y: 0,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 5,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
];