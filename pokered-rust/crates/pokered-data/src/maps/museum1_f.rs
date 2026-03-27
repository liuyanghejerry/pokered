use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 5] = [
NpcEntry {
        sprite_id: 0x20,
        x: 12,
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
        sprite_id: 0x0B,
        x: 1,
        y: 4,
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
        sprite_id: 0x20,
        x: 15,
        y: 2,
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
        sprite_id: 0x20,
        x: 17,
        y: 4,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 4,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x45,
        x: 16,
        y: 2,
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

pub const WARPS: [WarpData; 5] = [
WarpData {
        x: 10,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 11,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 16,
        y: 7,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 17,
        y: 7,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 7,
        y: 7,
        dest_map: Some(MapId::Museum2F),
        dest_warp_id: 0,
    },
];