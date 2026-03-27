use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 5] = [
NpcEntry {
        sprite_id: 0x2A,
        x: 15,
        y: 13,
        movement: NpcMovement(0),
        facing: NpcFacing(1),
        range: 0,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x1C,
        x: 6,
        y: 8,
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
        sprite_id: 0x34,
        x: 8,
        y: 12,
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
        sprite_id: 0x0D,
        x: 13,
        y: 7,
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
        sprite_id: 0x19,
        x: 17,
        y: 7,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 5,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 3] = [
WarpData {
        x: 10,
        y: 17,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 11,
        y: 17,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 18,
        y: 9,
        dest_map: Some(MapId::PokemonTower2F),
        dest_warp_id: 1,
    },
];