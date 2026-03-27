use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 5] = [
NpcEntry {
        sprite_id: 0x1B,
        x: 10,
        y: 1,
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
        sprite_id: 0x20,
        x: 5,
        y: 12,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 2,
        is_trainer: true,
        trainer_class: 28,
        trainer_set: 2,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x20,
        x: 24,
        y: 13,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 3,
        is_trainer: true,
        trainer_class: 28,
        trainer_set: 3,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x18,
        x: 16,
        y: 11,
        movement: NpcMovement(0),
        facing: NpcFacing(1),
        range: 0,
        text_id: 4,
        is_trainer: true,
        trainer_class: 30,
        trainer_set: 23,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x18,
        x: 24,
        y: 7,
        movement: NpcMovement(0),
        facing: NpcFacing(1),
        range: 0,
        text_id: 5,
        is_trainer: true,
        trainer_class: 30,
        trainer_set: 24,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 7] = [
WarpData {
        x: 24,
        y: 0,
        dest_map: Some(MapId::SilphCo1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 26,
        y: 0,
        dest_map: Some(MapId::SilphCo3F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 20,
        y: 0,
        dest_map: Some(MapId::SilphCoElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 3,
        y: 3,
        dest_map: Some(MapId::SilphCo3F),
        dest_warp_id: 6,
    },
    WarpData {
        x: 13,
        y: 3,
        dest_map: Some(MapId::SilphCo8F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 27,
        y: 15,
        dest_map: Some(MapId::SilphCo8F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 9,
        y: 15,
        dest_map: Some(MapId::SilphCo6F),
        dest_warp_id: 4,
    },
];