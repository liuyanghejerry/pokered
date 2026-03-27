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
        x: 2,
        y: 5,
        dest_map: Some(MapId::UndergroundPathRoute7),
        dest_warp_id: 2,
    },
    WarpData {
        x: 47,
        y: 2,
        dest_map: Some(MapId::UndergroundPathRoute8),
        dest_warp_id: 2,
    },
];