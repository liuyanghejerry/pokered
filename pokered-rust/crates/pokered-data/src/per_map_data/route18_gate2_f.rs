use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 1] = [
NpcEntry {
    sprite_id: 0x04,
    x: 4,
    y: 2,
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

pub const SIGNS: [SignEntry; 2] = [
SignEntry {
        x: 1,
        y: 2,
        text_id: 1,
    },
    SignEntry {
        x: 6,
        y: 2,
        text_id: 2,
    },
];

pub const WARPS: [WarpData; 1] = [
WarpData {
    x: 7,
    y: 7,
    dest_map: Some(MapId::Route18Gate1F),
    dest_warp_id: 4,
}
];