use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 5] = [
NpcEntry {
        sprite_id: 0x2B,
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
        sprite_id: 0x0F,
        x: 10,
        y: 5,
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
        sprite_id: 0x17,
        x: 6,
        y: 9,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 3,
        is_trainer: true,
        trainer_class: 29,
        trainer_set: 2,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x18,
        x: 3,
        y: 16,
        movement: NpcMovement(0),
        facing: NpcFacing(1),
        range: 0,
        text_id: 4,
        is_trainer: true,
        trainer_class: 30,
        trainer_set: 41,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x18,
        x: 15,
        y: 9,
        movement: NpcMovement(0),
        facing: NpcFacing(1),
        range: 0,
        text_id: 5,
        is_trainer: true,
        trainer_class: 30,
        trainer_set: 40,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 4] = [
WarpData {
        x: 9,
        y: 0,
        dest_map: Some(MapId::SilphCo10F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 13,
        y: 0,
        dest_map: Some(MapId::SilphCoElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 5,
        dest_map: None,
        dest_warp_id: 9,
    },
    WarpData {
        x: 3,
        y: 2,
        dest_map: Some(MapId::SilphCo7F),
        dest_warp_id: 3,
    },
];