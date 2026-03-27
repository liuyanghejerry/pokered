use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 0] = [

];

pub const SIGNS: [SignEntry; 1] = [
SignEntry {
    x: 4,
    y: 9,
    text_id: 1,
}
];

pub const WARPS: [WarpData; 4] = [
WarpData {
        x: 6,
        y: 1,
        dest_map: Some(MapId::CeladonMansion3F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 7,
        y: 1,
        dest_map: Some(MapId::CeladonMansion1F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 2,
        y: 1,
        dest_map: Some(MapId::CeladonMansion1F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 4,
        y: 1,
        dest_map: Some(MapId::CeladonMansion3F),
        dest_warp_id: 3,
    },
];