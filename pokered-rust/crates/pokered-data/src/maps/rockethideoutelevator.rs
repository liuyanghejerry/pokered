use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 0] = [

];

pub const SIGNS: [SignEntry; 1] = [
SignEntry {
    x: 1,
    y: 1,
    text_id: 1,
}
];

pub const WARPS: [WarpData; 2] = [
WarpData {
        x: 2,
        y: 1,
        dest_map: Some(MapId::RocketHideoutB1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 3,
        y: 1,
        dest_map: Some(MapId::RocketHideoutB1F),
        dest_warp_id: 4,
    },
];