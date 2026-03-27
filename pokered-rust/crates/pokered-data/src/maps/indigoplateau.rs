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
        x: 9,
        y: 5,
        dest_map: Some(MapId::IndigoPlateauLobby),
        dest_warp_id: 0,
    },
    WarpData {
        x: 10,
        y: 5,
        dest_map: Some(MapId::IndigoPlateauLobby),
        dest_warp_id: 0,
    },
];