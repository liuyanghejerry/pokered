use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 5] = [
NpcEntry {
        sprite_id: 0x0C,
        x: 5,
        y: 11,
        movement: NpcMovement(1),
        facing: NpcFacing(2),
        range: 2,
        text_id: 1,
        is_trainer: true,
        trainer_class: 11,
        trainer_set: 8,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x20,
        x: 20,
        y: 11,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 2,
        is_trainer: true,
        trainer_class: 28,
        trainer_set: 12,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 1,
        y: 16,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 3,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x11,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 25,
        y: 5,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 4,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x25,
    },
    NpcEntry {
        sprite_id: 0x41,
        x: 6,
        y: 12,
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

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 3] = [
WarpData {
        x: 7,
        y: 10,
        dest_map: Some(MapId::PokemonMansion2F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 6,
        y: 1,
        dest_map: Some(MapId::PokemonMansion2F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 25,
        y: 14,
        dest_map: Some(MapId::PokemonMansion2F),
        dest_warp_id: 2,
    },
];