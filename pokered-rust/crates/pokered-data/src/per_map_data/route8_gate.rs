use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 1] = [
NpcEntry {
    sprite_id: 0x31,
    x: 2,
    y: 1,
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

pub const WARPS: [WarpData; 4] = [
WarpData {
        x: 0,
        y: 3,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 0,
        y: 4,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 5,
        y: 3,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 5,
        y: 4,
        dest_map: None,
        dest_warp_id: 3,
    },
];