use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 0] = [

];

pub const SIGNS: [SignEntry; 1] = [
SignEntry {
    x: 17,
    y: 29,
    text_id: 1,
}
];

pub const WARPS: [WarpData; 5] = [
WarpData {
        x: 10,
        y: 29,
        dest_map: Some(MapId::Route5Gate),
        dest_warp_id: 3,
    },
    WarpData {
        x: 9,
        y: 29,
        dest_map: Some(MapId::Route5Gate),
        dest_warp_id: 2,
    },
    WarpData {
        x: 10,
        y: 33,
        dest_map: Some(MapId::Route5Gate),
        dest_warp_id: 0,
    },
    WarpData {
        x: 17,
        y: 27,
        dest_map: Some(MapId::UndergroundPathRoute5),
        dest_warp_id: 0,
    },
    WarpData {
        x: 10,
        y: 21,
        dest_map: Some(MapId::Daycare),
        dest_warp_id: 0,
    },
];