use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 3] = [
NpcEntry {
        sprite_id: 0x1C,
        x: 2,
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
        sprite_id: 0x0A,
        x: 5,
        y: 4,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x38,
        x: 1,
        y: 4,
        movement: NpcMovement(1),
        facing: NpcFacing(0),
        range: 1,
        text_id: 3,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 3] = [
WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 7,
        y: 1,
        dest_map: Some(MapId::CopycatsHouse2F),
        dest_warp_id: 0,
    },
];