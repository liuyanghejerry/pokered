use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 5] = [
NpcEntry {
        sprite_id: 0x29,
        x: 7,
        y: 5,
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
        sprite_id: 0x24,
        x: 4,
        y: 9,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
        range: 0,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x06,
        x: 5,
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
    NpcEntry {
        sprite_id: 0x26,
        x: 0,
        y: 5,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
        range: 0,
        text_id: 4,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x2A,
        x: 13,
        y: 6,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 5,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 3] = [
WarpData {
        x: 7,
        y: 11,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 8,
        y: 11,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 8,
        y: 0,
        dest_map: Some(MapId::LoreleisRoom),
        dest_warp_id: 0,
    },
];