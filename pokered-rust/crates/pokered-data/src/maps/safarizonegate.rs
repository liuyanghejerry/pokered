use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 2] = [
NpcEntry {
        sprite_id: 0x23,
        x: 6,
        y: 2,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x23,
        x: 1,
        y: 4,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
        range: 0,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 4] = [
WarpData {
        x: 3,
        y: 5,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 4,
        y: 5,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 3,
        y: 0,
        dest_map: Some(MapId::SafariZoneCenter),
        dest_warp_id: 0,
    },
    WarpData {
        x: 4,
        y: 0,
        dest_map: Some(MapId::SafariZoneCenter),
        dest_warp_id: 1,
    },
];