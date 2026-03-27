use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 2] = [
NpcEntry {
        sprite_id: 0x1A,
        x: 3,
        y: 7,
        movement: NpcMovement(1),
        facing: NpcFacing(0),
        range: 1,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x02,
        x: 36,
        y: 4,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 2,
        is_trainer: true,
        trainer_class: 25,
        trainer_set: 1,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 9] = [
WarpData {
        x: 9,
        y: 11,
        dest_map: Some(MapId::SSAnne2FRooms),
        dest_warp_id: 0,
    },
    WarpData {
        x: 13,
        y: 11,
        dest_map: Some(MapId::SSAnne2FRooms),
        dest_warp_id: 2,
    },
    WarpData {
        x: 17,
        y: 11,
        dest_map: Some(MapId::SSAnne2FRooms),
        dest_warp_id: 4,
    },
    WarpData {
        x: 21,
        y: 11,
        dest_map: Some(MapId::SSAnne2FRooms),
        dest_warp_id: 6,
    },
    WarpData {
        x: 25,
        y: 11,
        dest_map: Some(MapId::SSAnne2FRooms),
        dest_warp_id: 8,
    },
    WarpData {
        x: 29,
        y: 11,
        dest_map: Some(MapId::SSAnne2FRooms),
        dest_warp_id: 10,
    },
    WarpData {
        x: 2,
        y: 4,
        dest_map: Some(MapId::SSAnne1F),
        dest_warp_id: 8,
    },
    WarpData {
        x: 2,
        y: 12,
        dest_map: Some(MapId::SSAnne3F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 36,
        y: 4,
        dest_map: Some(MapId::SSAnneCaptainsRoom),
        dest_warp_id: 0,
    },
];