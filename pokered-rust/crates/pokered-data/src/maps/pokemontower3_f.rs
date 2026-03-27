use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 4] = [
NpcEntry {
        sprite_id: 0x19,
        x: 12,
        y: 3,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 1,
        is_trainer: true,
        trainer_class: 45,
        trainer_set: 5,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x19,
        x: 9,
        y: 8,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 2,
        is_trainer: true,
        trainer_class: 45,
        trainer_set: 6,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x19,
        x: 10,
        y: 13,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 3,
        is_trainer: true,
        trainer_class: 45,
        trainer_set: 8,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 12,
        y: 1,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 4,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x1D,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 2] = [
WarpData {
        x: 3,
        y: 9,
        dest_map: Some(MapId::PokemonTower2F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 18,
        y: 9,
        dest_map: Some(MapId::PokemonTower4F),
        dest_warp_id: 1,
    },
];