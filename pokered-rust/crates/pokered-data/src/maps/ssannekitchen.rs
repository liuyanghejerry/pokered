use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 7] = [
NpcEntry {
        sprite_id: 0x14,
        x: 1,
        y: 8,
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
        sprite_id: 0x14,
        x: 5,
        y: 8,
        movement: NpcMovement(1),
        facing: NpcFacing(0),
        range: 1,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x14,
        x: 9,
        y: 7,
        movement: NpcMovement(1),
        facing: NpcFacing(0),
        range: 1,
        text_id: 3,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x14,
        x: 13,
        y: 6,
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
        sprite_id: 0x14,
        x: 13,
        y: 8,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 5,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x14,
        x: 13,
        y: 10,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 6,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x14,
        x: 11,
        y: 13,
        movement: NpcMovement(0),
        facing: NpcFacing(1),
        range: 0,
        text_id: 7,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 1] = [
WarpData {
    x: 6,
    y: 0,
    dest_map: Some(MapId::SSAnne1F),
    dest_warp_id: 10,
}
];