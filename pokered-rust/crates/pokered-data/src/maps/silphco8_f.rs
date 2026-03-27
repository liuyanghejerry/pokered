use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 4] = [
NpcEntry {
        sprite_id: 0x2C,
        x: 4,
        y: 2,
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
        x: 19,
        y: 2,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 2,
        is_trainer: true,
        trainer_class: 30,
        trainer_set: 35,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x20,
        x: 10,
        y: 2,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 3,
        is_trainer: true,
        trainer_class: 28,
        trainer_set: 9,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x18,
        x: 12,
        y: 15,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
        range: 0,
        text_id: 4,
        is_trainer: true,
        trainer_class: 30,
        trainer_set: 36,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 7] = [
WarpData {
        x: 16,
        y: 0,
        dest_map: Some(MapId::SilphCo9F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 14,
        y: 0,
        dest_map: Some(MapId::SilphCo7F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 18,
        y: 0,
        dest_map: Some(MapId::SilphCoElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 3,
        y: 11,
        dest_map: Some(MapId::SilphCo8F),
        dest_warp_id: 6,
    },
    WarpData {
        x: 3,
        y: 15,
        dest_map: Some(MapId::SilphCo2F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 11,
        y: 5,
        dest_map: Some(MapId::SilphCo2F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 11,
        y: 9,
        dest_map: Some(MapId::SilphCo8F),
        dest_warp_id: 3,
    },
];