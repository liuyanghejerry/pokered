use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 0] = [

];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 6] = [
WarpData {
        x: 23,
        y: 3,
        dest_map: Some(MapId::SSAnneB1FRooms),
        dest_warp_id: 8,
    },
    WarpData {
        x: 19,
        y: 3,
        dest_map: Some(MapId::SSAnneB1FRooms),
        dest_warp_id: 6,
    },
    WarpData {
        x: 15,
        y: 3,
        dest_map: Some(MapId::SSAnneB1FRooms),
        dest_warp_id: 4,
    },
    WarpData {
        x: 11,
        y: 3,
        dest_map: Some(MapId::SSAnneB1FRooms),
        dest_warp_id: 2,
    },
    WarpData {
        x: 7,
        y: 3,
        dest_map: Some(MapId::SSAnneB1FRooms),
        dest_warp_id: 0,
    },
    WarpData {
        x: 27,
        y: 5,
        dest_map: Some(MapId::SSAnne1F),
        dest_warp_id: 9,
    },
];