use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 4] = [
NpcEntry {
        sprite_id: 0x18,
        x: 10,
        y: 22,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
        range: 0,
        text_id: 1,
        is_trainer: true,
        trainer_class: 30,
        trainer_set: 14,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x18,
        x: 26,
        y: 12,
        movement: NpcMovement(0),
        facing: NpcFacing(1),
        range: 0,
        text_id: 2,
        is_trainer: true,
        trainer_class: 30,
        trainer_set: 15,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 26,
        y: 17,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 3,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0xD2,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 20,
        y: 14,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 4,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x28,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 2] = [
WarpData {
        x: 25,
        y: 6,
        dest_map: Some(MapId::RocketHideoutB2F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 19,
        y: 18,
        dest_map: Some(MapId::RocketHideoutB4F),
        dest_warp_id: 0,
    },
];