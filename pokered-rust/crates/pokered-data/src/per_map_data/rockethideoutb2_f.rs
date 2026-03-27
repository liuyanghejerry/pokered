use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 5] = [
NpcEntry {
        sprite_id: 0x18,
        x: 20,
        y: 12,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 1,
        is_trainer: true,
        trainer_class: 30,
        trainer_set: 13,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 1,
        y: 11,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x0A,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 16,
        y: 8,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 3,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x31,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 6,
        y: 12,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 4,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0xCF,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 3,
        y: 21,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 5,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x13,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 5] = [
WarpData {
        x: 27,
        y: 8,
        dest_map: Some(MapId::RocketHideoutB1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 21,
        y: 8,
        dest_map: Some(MapId::RocketHideoutB3F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 24,
        y: 19,
        dest_map: Some(MapId::RocketHideoutElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 21,
        y: 22,
        dest_map: Some(MapId::RocketHideoutB1F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 25,
        y: 19,
        dest_map: Some(MapId::RocketHideoutElevator),
        dest_warp_id: 1,
    },
];