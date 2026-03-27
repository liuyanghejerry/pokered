use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 6] = [
NpcEntry {
        sprite_id: 0x18,
        x: 1,
        y: 9,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
        range: 0,
        text_id: 1,
        is_trainer: true,
        trainer_class: 30,
        trainer_set: 39,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x20,
        x: 10,
        y: 2,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 2,
        is_trainer: true,
        trainer_class: 28,
        trainer_set: 11,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x1B,
        x: 9,
        y: 15,
        movement: NpcMovement(1),
        facing: NpcFacing(0),
        range: 0,
        text_id: 3,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 2,
        y: 12,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 4,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0xE2,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 4,
        y: 14,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 5,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x28,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 5,
        y: 11,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 6,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x26,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 6] = [
WarpData {
        x: 8,
        y: 0,
        dest_map: Some(MapId::SilphCo9F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 10,
        y: 0,
        dest_map: Some(MapId::SilphCo11F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 12,
        y: 0,
        dest_map: Some(MapId::SilphCoElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 9,
        y: 11,
        dest_map: Some(MapId::SilphCo4F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 13,
        y: 15,
        dest_map: Some(MapId::SilphCo4F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 13,
        y: 7,
        dest_map: Some(MapId::SilphCo4F),
        dest_warp_id: 6,
    },
];