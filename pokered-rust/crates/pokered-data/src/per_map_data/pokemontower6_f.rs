use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 5] = [
NpcEntry {
        sprite_id: 0x19,
        x: 12,
        y: 10,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
        range: 0,
        text_id: 1,
        is_trainer: true,
        trainer_class: 45,
        trainer_set: 19,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x19,
        x: 9,
        y: 5,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 2,
        is_trainer: true,
        trainer_class: 45,
        trainer_set: 20,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x19,
        x: 16,
        y: 5,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 3,
        is_trainer: true,
        trainer_class: 45,
        trainer_set: 21,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 6,
        y: 8,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 4,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x28,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 14,
        y: 14,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 5,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x2E,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 2] = [
WarpData {
        x: 18,
        y: 9,
        dest_map: Some(MapId::PokemonTower5F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 9,
        y: 16,
        dest_map: Some(MapId::PokemonTower7F),
        dest_warp_id: 0,
    },
];