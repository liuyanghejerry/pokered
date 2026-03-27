use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 0] = [

];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 1] = [
WarpData {
    x: 7,
    y: 1,
    dest_map: Some(MapId::RedsHouse1F),
    dest_warp_id: 2,
}
];