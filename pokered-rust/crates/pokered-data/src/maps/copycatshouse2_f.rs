use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 5] = [
NpcEntry {
        sprite_id: 0x1D,
        x: 4,
        y: 3,
        movement: NpcMovement(1),
        facing: NpcFacing(0),
        range: 0,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x09,
        x: 4,
        y: 6,
        movement: NpcMovement(1),
        facing: NpcFacing(2),
        range: 2,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x05,
        x: 5,
        y: 1,
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
        sprite_id: 0x09,
        x: 2,
        y: 0,
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
        sprite_id: 0x38,
        x: 1,
        y: 6,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
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
        x: 3,
        y: 5,
        text_id: 1,
    },
    SignEntry {
        x: 0,
        y: 1,
        text_id: 2,
    },
];

pub const WARPS: [WarpData; 1] = [
WarpData {
    x: 7,
    y: 1,
    dest_map: Some(MapId::CopycatsHouse1F),
    dest_warp_id: 2,
}
];