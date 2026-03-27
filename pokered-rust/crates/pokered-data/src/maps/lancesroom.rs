use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 1] = [
NpcEntry {
    sprite_id: 0x1E,
    x: 6,
    y: 1,
    movement: NpcMovement(0),
    facing: NpcFacing(0),
    range: 0,
    text_id: 1,
    is_trainer: true,
    trainer_class: 47,
    trainer_set: 1,
    item_id: 0x00,
}
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 3] = [
WarpData {
        x: 24,
        y: 16,
        dest_map: Some(MapId::AgathasRoom),
        dest_warp_id: 2,
    },
    WarpData {
        x: 5,
        y: 0,
        dest_map: Some(MapId::ChampionsRoom),
        dest_warp_id: 0,
    },
    WarpData {
        x: 6,
        y: 0,
        dest_map: Some(MapId::ChampionsRoom),
        dest_warp_id: 0,
    },
];