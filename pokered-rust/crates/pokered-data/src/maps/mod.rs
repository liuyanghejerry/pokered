mod pallet_town;

use crate::map_objects::WarpData;
use crate::maps::MapId;
use crate::npc_data::NpcEntry;
use crate::sign_data::SignEntry;

pub fn get_npcs(map: MapId) -> &'static [NpcEntry] {
    match map {
        MapId::PalletTown => &pallet_town::NPCS,
        _ => &[],
    }
}

pub fn get_signs(map: MapId) -> &'static [SignEntry] {
    match map {
        MapId::PalletTown => &pallet_town::SIGNS,
        _ => &[],
    }
}

pub fn get_warps(map: MapId) -> &'static [WarpData] {
    match map {
        MapId::PalletTown => &pallet_town::WARPS,
        _ => &[],
    }
}
