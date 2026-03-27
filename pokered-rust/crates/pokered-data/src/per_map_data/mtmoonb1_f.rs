use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 0] = [

];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 8] = [
WarpData {
        x: 5,
        y: 5,
        dest_map: Some(MapId::MtMoon1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 17,
        y: 11,
        dest_map: Some(MapId::MtMoonB2F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 25,
        y: 9,
        dest_map: Some(MapId::MtMoon1F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 25,
        y: 15,
        dest_map: Some(MapId::MtMoon1F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 21,
        y: 17,
        dest_map: Some(MapId::MtMoonB2F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 13,
        y: 27,
        dest_map: Some(MapId::MtMoonB2F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 23,
        y: 3,
        dest_map: Some(MapId::MtMoonB2F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 27,
        y: 3,
        dest_map: None,
        dest_warp_id: 2,
    },
];