use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 5] = [
NpcEntry {
        sprite_id: 0x0C,
        x: 5,
        y: 2,
        movement: NpcMovement(0),
        facing: NpcFacing(1),
        range: 0,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x13,
        x: 4,
        y: 9,
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
        sprite_id: 0x07,
        x: 7,
        y: 11,
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
        sprite_id: 0x13,
        x: 4,
        y: 4,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 4,
        is_trainer: true,
        trainer_class: 4,
        trainer_set: 1,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x13,
        x: 10,
        y: 8,
        movement: NpcMovement(0),
        facing: NpcFacing(1),
        range: 0,
        text_id: 5,
        is_trainer: true,
        trainer_class: 4,
        trainer_set: 2,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 2] = [
WarpData {
        x: 13,
        y: 6,
        dest_map: Some(MapId::SSAnne3F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 13,
        y: 7,
        dest_map: Some(MapId::SSAnne3F),
        dest_warp_id: 0,
    },
];