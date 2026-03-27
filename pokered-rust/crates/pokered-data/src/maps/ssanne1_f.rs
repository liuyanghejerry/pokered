use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 2] = [
NpcEntry {
        sprite_id: 0x1A,
        x: 12,
        y: 6,
        movement: NpcMovement(1),
        facing: NpcFacing(2),
        range: 2,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x13,
        x: 27,
        y: 5,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
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

pub const WARPS: [WarpData; 11] = [
WarpData {
        x: 26,
        y: 0,
        dest_map: Some(MapId::VermilionDock),
        dest_warp_id: 1,
    },
    WarpData {
        x: 27,
        y: 0,
        dest_map: Some(MapId::VermilionDock),
        dest_warp_id: 1,
    },
    WarpData {
        x: 31,
        y: 8,
        dest_map: Some(MapId::SSAnne1FRooms),
        dest_warp_id: 0,
    },
    WarpData {
        x: 23,
        y: 8,
        dest_map: Some(MapId::SSAnne1FRooms),
        dest_warp_id: 1,
    },
    WarpData {
        x: 19,
        y: 8,
        dest_map: Some(MapId::SSAnne1FRooms),
        dest_warp_id: 2,
    },
    WarpData {
        x: 15,
        y: 8,
        dest_map: Some(MapId::SSAnne1FRooms),
        dest_warp_id: 3,
    },
    WarpData {
        x: 11,
        y: 8,
        dest_map: Some(MapId::SSAnne1FRooms),
        dest_warp_id: 4,
    },
    WarpData {
        x: 7,
        y: 8,
        dest_map: Some(MapId::SSAnne1FRooms),
        dest_warp_id: 5,
    },
    WarpData {
        x: 2,
        y: 6,
        dest_map: Some(MapId::SSAnne2F),
        dest_warp_id: 6,
    },
    WarpData {
        x: 37,
        y: 15,
        dest_map: Some(MapId::SSAnneB1F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 3,
        y: 16,
        dest_map: Some(MapId::SSAnneKitchen),
        dest_warp_id: 0,
    },
];