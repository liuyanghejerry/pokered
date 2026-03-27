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
        x: 14,
        y: 0,
        dest_map: None,
        dest_warp_id: 5,
    },
    WarpData {
        x: 14,
        y: 2,
        dest_map: Some(MapId::SSAnne1F),
        dest_warp_id: 1,
    },
];