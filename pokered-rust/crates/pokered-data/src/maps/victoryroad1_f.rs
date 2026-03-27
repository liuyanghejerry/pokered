use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 7] = [
NpcEntry {
        sprite_id: 0x06,
        x: 7,
        y: 5,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
        range: 0,
        text_id: 1,
        is_trainer: true,
        trainer_class: 32,
        trainer_set: 5,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x07,
        x: 3,
        y: 2,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 2,
        is_trainer: true,
        trainer_class: 31,
        trainer_set: 5,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 11,
        y: 0,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 3,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0xF3,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 9,
        y: 2,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 4,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x28,
    },
    NpcEntry {
        sprite_id: 0x3F,
        x: 5,
        y: 15,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 5,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3F,
        x: 14,
        y: 2,
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
        sprite_id: 0x3F,
        x: 2,
        y: 10,
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

pub const WARPS: [WarpData; 3] = [
WarpData {
        x: 8,
        y: 17,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 9,
        y: 17,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 1,
        y: 1,
        dest_map: Some(MapId::VictoryRoad2F),
        dest_warp_id: 0,
    },
];