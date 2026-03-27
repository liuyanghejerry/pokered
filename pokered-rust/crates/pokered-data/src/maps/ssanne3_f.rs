use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 1] = [
NpcEntry {
    sprite_id: 0x13,
    x: 9,
    y: 3,
    movement: NpcMovement(1),
    facing: NpcFacing(2),
    range: 2,
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
        x: 0,
        y: 3,
        dest_map: Some(MapId::SSAnneBow),
        dest_warp_id: 0,
    },
    WarpData {
        x: 19,
        y: 3,
        dest_map: Some(MapId::SSAnne2F),
        dest_warp_id: 7,
    },
];