use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 4] = [
NpcEntry {
        sprite_id: 0x29,
        x: 3,
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
        sprite_id: 0x18,
        x: 2,
        y: 4,
        movement: NpcMovement(0),
        facing: NpcFacing(1),
        range: 0,
        text_id: 2,
        is_trainer: true,
        trainer_class: 30,
        trainer_set: 37,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x20,
        x: 21,
        y: 13,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 3,
        is_trainer: true,
        trainer_class: 28,
        trainer_set: 10,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x18,
        x: 13,
        y: 16,
        movement: NpcMovement(0),
        facing: NpcFacing(1),
        range: 0,
        text_id: 4,
        is_trainer: true,
        trainer_class: 30,
        trainer_set: 38,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 5] = [
WarpData {
        x: 14,
        y: 0,
        dest_map: Some(MapId::SilphCo10F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 16,
        y: 0,
        dest_map: Some(MapId::SilphCo8F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 18,
        y: 0,
        dest_map: Some(MapId::SilphCoElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 9,
        y: 3,
        dest_map: Some(MapId::SilphCo3F),
        dest_warp_id: 7,
    },
    WarpData {
        x: 17,
        y: 15,
        dest_map: Some(MapId::SilphCo5F),
        dest_warp_id: 4,
    },
];