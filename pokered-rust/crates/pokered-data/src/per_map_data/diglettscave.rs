use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 0] = [

];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 2] = [
WarpData {
        x: 5,
        y: 5,
        dest_map: Some(MapId::DiglettsCaveRoute2),
        dest_warp_id: 2,
    },
    WarpData {
        x: 37,
        y: 31,
        dest_map: Some(MapId::DiglettsCaveRoute11),
        dest_warp_id: 2,
    },
];