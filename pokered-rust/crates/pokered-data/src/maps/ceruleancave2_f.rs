use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 3] = [
NpcEntry {
        sprite_id: 0x3D,
        x: 29,
        y: 9,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x4F,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 4,
        y: 15,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x02,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 13,
        y: 6,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 3,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x10,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 6] = [
WarpData {
        x: 29,
        y: 1,
        dest_map: Some(MapId::CeruleanCave1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 22,
        y: 6,
        dest_map: Some(MapId::CeruleanCave1F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 19,
        y: 7,
        dest_map: Some(MapId::CeruleanCave1F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 9,
        y: 1,
        dest_map: Some(MapId::CeruleanCave1F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 1,
        y: 3,
        dest_map: Some(MapId::CeruleanCave1F),
        dest_warp_id: 6,
    },
    WarpData {
        x: 3,
        y: 11,
        dest_map: Some(MapId::CeruleanCave1F),
        dest_warp_id: 7,
    },
];