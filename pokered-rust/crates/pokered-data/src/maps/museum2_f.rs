use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 5] = [
NpcEntry {
        sprite_id: 0x04,
        x: 1,
        y: 7,
        movement: NpcMovement(1),
        facing: NpcFacing(2),
        range: 2,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x25,
        x: 0,
        y: 5,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x20,
        x: 7,
        y: 5,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 3,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x1D,
        x: 11,
        y: 5,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 4,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0E,
        x: 12,
        y: 5,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 5,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 2] = [
SignEntry {
        x: 11,
        y: 2,
        text_id: 1,
    },
    SignEntry {
        x: 2,
        y: 5,
        text_id: 2,
    },
];

pub const WARPS: [WarpData; 1] = [
WarpData {
    x: 7,
    y: 7,
    dest_map: Some(MapId::Museum1F),
    dest_warp_id: 4,
}
];