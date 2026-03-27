use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 4] = [
NpcEntry {
        sprite_id: 0x0C,
        x: 3,
        y: 17,
        movement: NpcMovement(1),
        facing: NpcFacing(2),
        range: 2,
        text_id: 1,
        is_trainer: true,
        trainer_class: 11,
        trainer_set: 7,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 28,
        y: 7,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x27,
    },
    NpcEntry {
        sprite_id: 0x41,
        x: 18,
        y: 2,
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
        sprite_id: 0x41,
        x: 3,
        y: 22,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 4,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 4] = [
WarpData {
        x: 5,
        y: 10,
        dest_map: Some(MapId::PokemonMansion1F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 7,
        y: 10,
        dest_map: Some(MapId::PokemonMansion3F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 25,
        y: 14,
        dest_map: Some(MapId::PokemonMansion3F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 6,
        y: 1,
        dest_map: Some(MapId::PokemonMansion3F),
        dest_warp_id: 1,
    },
];