use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 2] = [
NpcEntry {
        sprite_id: 0x04,
        x: 5,
        y: 24,
        movement: NpcMovement(1),
        facing: NpcFacing(0),
        range: 1,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x04,
        x: 15,
        y: 13,
        movement: NpcMovement(1),
        facing: NpcFacing(2),
        range: 2,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 1] = [
SignEntry {
    x: 9,
    y: 27,
    text_id: 1,
}
];

pub const WARPS: [WarpData; 0] = [];