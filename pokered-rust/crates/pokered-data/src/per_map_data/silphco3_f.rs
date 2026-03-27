use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 4] = [
NpcEntry {
        sprite_id: 0x2C,
        x: 24,
        y: 8,
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
        sprite_id: 0x18,
        x: 20,
        y: 7,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 2,
        is_trainer: true,
        trainer_class: 30,
        trainer_set: 25,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x20,
        x: 7,
        y: 9,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 3,
        is_trainer: true,
        trainer_class: 28,
        trainer_set: 4,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 8,
        y: 5,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 4,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x12,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 10] = [
WarpData {
        x: 26,
        y: 0,
        dest_map: Some(MapId::SilphCo2F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 24,
        y: 0,
        dest_map: Some(MapId::SilphCo4F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 20,
        y: 0,
        dest_map: Some(MapId::SilphCoElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 23,
        y: 11,
        dest_map: Some(MapId::SilphCo3F),
        dest_warp_id: 9,
    },
    WarpData {
        x: 3,
        y: 3,
        dest_map: Some(MapId::SilphCo5F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 3,
        y: 15,
        dest_map: Some(MapId::SilphCo5F),
        dest_warp_id: 6,
    },
    WarpData {
        x: 27,
        y: 3,
        dest_map: Some(MapId::SilphCo2F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 3,
        y: 11,
        dest_map: Some(MapId::SilphCo9F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 11,
        y: 11,
        dest_map: Some(MapId::SilphCo7F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 27,
        y: 15,
        dest_map: Some(MapId::SilphCo3F),
        dest_warp_id: 3,
    },
];