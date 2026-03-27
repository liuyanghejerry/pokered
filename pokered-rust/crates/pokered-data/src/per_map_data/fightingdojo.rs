use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 7] = [
NpcEntry {
        sprite_id: 0x0E,
        x: 5,
        y: 3,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 1,
        is_trainer: true,
        trainer_class: 24,
        trainer_set: 1,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0E,
        x: 3,
        y: 4,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
        range: 0,
        text_id: 2,
        is_trainer: true,
        trainer_class: 24,
        trainer_set: 2,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0E,
        x: 3,
        y: 6,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
        range: 0,
        text_id: 3,
        is_trainer: true,
        trainer_class: 24,
        trainer_set: 3,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0E,
        x: 5,
        y: 5,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 4,
        is_trainer: true,
        trainer_class: 24,
        trainer_set: 4,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0E,
        x: 5,
        y: 7,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 5,
        is_trainer: true,
        trainer_class: 24,
        trainer_set: 5,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 4,
        y: 1,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 6,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 5,
        y: 1,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 7,
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
        y: 11,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 5,
        y: 11,
        dest_map: None,
        dest_warp_id: 1,
    },
];