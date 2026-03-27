use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 2] = [
NpcEntry {
        sprite_id: 0x02,
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
        sprite_id: 0x03,
        x: 3,
        y: 7,
        movement: NpcMovement(0),
        facing: NpcFacing(1),
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
        y: 7,
        dest_map: Some(MapId::LancesRoom),
        dest_warp_id: 1,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: Some(MapId::LancesRoom),
        dest_warp_id: 2,
    },
    WarpData {
        x: 3,
        y: 0,
        dest_map: Some(MapId::HallOfFame),
        dest_warp_id: 0,
    },
    WarpData {
        x: 4,
        y: 0,
        dest_map: Some(MapId::HallOfFame),
        dest_warp_id: 0,
    },
];