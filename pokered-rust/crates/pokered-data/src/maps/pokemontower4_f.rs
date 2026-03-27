use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 6] = [
NpcEntry {
        sprite_id: 0x19,
        x: 5,
        y: 10,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
        range: 0,
        text_id: 1,
        is_trainer: true,
        trainer_class: 45,
        trainer_set: 9,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x19,
        x: 15,
        y: 7,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 2,
        is_trainer: true,
        trainer_class: 45,
        trainer_set: 10,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x19,
        x: 14,
        y: 12,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 3,
        is_trainer: true,
        trainer_class: 45,
        trainer_set: 12,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 12,
        y: 10,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 4,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x52,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 9,
        y: 10,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 5,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x0E,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 12,
        y: 16,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 6,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x23,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 2] = [
WarpData {
        x: 3,
        y: 9,
        dest_map: Some(MapId::PokemonTower5F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 18,
        y: 9,
        dest_map: Some(MapId::PokemonTower3F),
        dest_warp_id: 1,
    },
];