use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 2] = [
NpcEntry {
        sprite_id: 0x0C,
        x: 10,
        y: 4,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x08,
        x: 5,
        y: 5,
        movement: NpcMovement(1),
        facing: NpcFacing(0),
        range: 0,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 4] = [
SignEntry {
        x: 10,
        y: 1,
        text_id: 1,
    },
    SignEntry {
        x: 11,
        y: 1,
        text_id: 2,
    },
    SignEntry {
        x: 12,
        y: 2,
        text_id: 3,
    },
    SignEntry {
        x: 13,
        y: 2,
        text_id: 4,
    },
];

pub const WARPS: [WarpData; 1] = [
WarpData {
    x: 15,
    y: 2,
    dest_map: Some(MapId::CeladonMart5F),
    dest_warp_id: 0,
}
];