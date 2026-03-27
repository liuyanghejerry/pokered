use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 1] = [
NpcEntry {
    sprite_id: 0x3A,
    x: 5,
    y: 2,
    movement: NpcMovement(0),
    facing: NpcFacing(0),
    range: 0,
    text_id: 1,
    is_trainer: true,
    trainer_class: 33,
    trainer_set: 1,
    item_id: 0x00,
}
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 4] = [
WarpData {
        x: 4,
        y: 11,
        dest_map: Some(MapId::LoreleisRoom),
        dest_warp_id: 2,
    },
    WarpData {
        x: 5,
        y: 11,
        dest_map: Some(MapId::LoreleisRoom),
        dest_warp_id: 3,
    },
    WarpData {
        x: 4,
        y: 0,
        dest_map: Some(MapId::AgathasRoom),
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 0,
        dest_map: Some(MapId::AgathasRoom),
        dest_warp_id: 1,
    },
];