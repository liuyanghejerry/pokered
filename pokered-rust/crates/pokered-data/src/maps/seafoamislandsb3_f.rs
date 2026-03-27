use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 6] = [
NpcEntry {
        sprite_id: 0x3F,
        x: 5,
        y: 14,
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
        x: 3,
        y: 15,
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
        sprite_id: 0x3F,
        x: 8,
        y: 14,
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
        sprite_id: 0x3F,
        x: 9,
        y: 14,
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
        sprite_id: 0x3F,
        x: 18,
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
    NpcEntry {
        sprite_id: 0x3F,
        x: 19,
        y: 6,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 6,
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
        y: 12,
        dest_map: Some(MapId::SeafoamIslandsB2F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 8,
        y: 6,
        dest_map: Some(MapId::SeafoamIslandsB4F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 25,
        y: 4,
        dest_map: Some(MapId::SeafoamIslandsB4F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 25,
        y: 3,
        dest_map: Some(MapId::SeafoamIslandsB2F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 25,
        y: 14,
        dest_map: Some(MapId::SeafoamIslandsB2F),
        dest_warp_id: 6,
    },
    WarpData {
        x: 20,
        y: 17,
        dest_map: Some(MapId::SeafoamIslandsB4F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 21,
        y: 17,
        dest_map: Some(MapId::SeafoamIslandsB4F),
        dest_warp_id: 1,
    },
];