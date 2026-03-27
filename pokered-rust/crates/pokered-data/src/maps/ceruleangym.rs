use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 4] = [
NpcEntry {
        sprite_id: 0x1D,
        x: 4,
        y: 2,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 1,
        is_trainer: true,
        trainer_class: 35,
        trainer_set: 1,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x06,
        x: 2,
        y: 3,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
        range: 0,
        text_id: 2,
        is_trainer: true,
        trainer_class: 6,
        trainer_set: 1,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x22,
        x: 8,
        y: 7,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 3,
        is_trainer: true,
        trainer_class: 15,
        trainer_set: 1,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x24,
        x: 7,
        y: 10,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 4,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 2] = [
WarpData {
        x: 4,
        y: 13,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 5,
        y: 13,
        dest_map: None,
        dest_warp_id: 3,
    },
];