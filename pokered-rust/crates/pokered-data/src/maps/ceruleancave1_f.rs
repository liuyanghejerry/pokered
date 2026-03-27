use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 3] = [
NpcEntry {
        sprite_id: 0x3D,
        x: 7,
        y: 13,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x10,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 19,
        y: 3,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x53,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 5,
        y: 0,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 3,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x31,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 9] = [
WarpData {
        x: 24,
        y: 17,
        dest_map: None,
        dest_warp_id: 6,
    },
    WarpData {
        x: 25,
        y: 17,
        dest_map: None,
        dest_warp_id: 6,
    },
    WarpData {
        x: 27,
        y: 1,
        dest_map: Some(MapId::CeruleanCave2F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 23,
        y: 7,
        dest_map: Some(MapId::CeruleanCave2F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 18,
        y: 9,
        dest_map: Some(MapId::CeruleanCave2F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 7,
        y: 1,
        dest_map: Some(MapId::CeruleanCave2F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 1,
        y: 3,
        dest_map: Some(MapId::CeruleanCave2F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 3,
        y: 11,
        dest_map: Some(MapId::CeruleanCave2F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 0,
        y: 6,
        dest_map: Some(MapId::CeruleanCaveB1F),
        dest_warp_id: 0,
    },
];