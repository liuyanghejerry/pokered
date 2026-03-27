use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 0] = [

];

pub const SIGNS: [SignEntry; 1] = [
SignEntry {
    x: 3,
    y: 13,
    text_id: 1,
}
];

pub const WARPS: [WarpData; 5] = [
WarpData {
        x: 18,
        y: 9,
        dest_map: Some(MapId::Route7Gate),
        dest_warp_id: 2,
    },
    WarpData {
        x: 18,
        y: 10,
        dest_map: Some(MapId::Route7Gate),
        dest_warp_id: 3,
    },
    WarpData {
        x: 11,
        y: 9,
        dest_map: Some(MapId::Route7Gate),
        dest_warp_id: 0,
    },
    WarpData {
        x: 11,
        y: 10,
        dest_map: Some(MapId::Route7Gate),
        dest_warp_id: 1,
    },
    WarpData {
        x: 5,
        y: 13,
        dest_map: Some(MapId::UndergroundPathRoute7),
        dest_warp_id: 0,
    },
];