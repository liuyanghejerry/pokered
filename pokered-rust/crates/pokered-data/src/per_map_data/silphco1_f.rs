use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 1] = [
NpcEntry {
    sprite_id: 0x2A,
    x: 4,
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

pub const WARPS: [WarpData; 5] = [
WarpData {
        x: 10,
        y: 17,
        dest_map: None,
        dest_warp_id: 5,
    },
    WarpData {
        x: 11,
        y: 17,
        dest_map: None,
        dest_warp_id: 5,
    },
    WarpData {
        x: 26,
        y: 0,
        dest_map: Some(MapId::SilphCo2F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 20,
        y: 0,
        dest_map: Some(MapId::SilphCoElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 16,
        y: 10,
        dest_map: Some(MapId::SilphCo3F),
        dest_warp_id: 6,
    },
];