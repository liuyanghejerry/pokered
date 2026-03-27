use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 0] = [

];

pub const SIGNS: [SignEntry; 1] = [
SignEntry {
    x: 3,
    y: 7,
    text_id: 1,
}
];

pub const WARPS: [WarpData; 3] = [
WarpData {
        x: 6,
        y: 1,
        dest_map: Some(MapId::CeladonMansion3F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 2,
        y: 1,
        dest_map: Some(MapId::CeladonMansion3F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 2,
        y: 7,
        dest_map: Some(MapId::CeladonMansionRoofHouse),
        dest_warp_id: 0,
    },
];