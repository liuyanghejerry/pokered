use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 0] = [

];

pub const SIGNS: [SignEntry; 1] = [
SignEntry {
    x: 3,
    y: 0,
    text_id: 1,
}
];

pub const WARPS: [WarpData; 2] = [
WarpData {
        x: 1,
        y: 3,
        dest_map: None, /* UNUSED_MAP_ED */
        dest_warp_id: 0,
    },
    WarpData {
        x: 2,
        y: 3,
        dest_map: None, /* UNUSED_MAP_ED */
        dest_warp_id: 0,
    },
];