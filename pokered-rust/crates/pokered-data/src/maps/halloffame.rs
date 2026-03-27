use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 1] = [
NpcEntry {
    sprite_id: 0x03,
    x: 5,
    y: 2,
    movement: NpcMovement(0),
    facing: NpcFacing(0),
    range: 0,
    text_id: 1,
    is_trainer: false,
    trainer_class: 0,
    trainer_set: 0,
    item_id: 0x00,
}
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 2] = [
WarpData {
        x: 4,
        y: 7,
        dest_map: Some(MapId::ChampionsRoom),
        dest_warp_id: 2,
    },
    WarpData {
        x: 5,
        y: 7,
        dest_map: Some(MapId::ChampionsRoom),
        dest_warp_id: 3,
    },
];