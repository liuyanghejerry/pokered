use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 4] = [
NpcEntry {
        sprite_id: 0x18,
        x: 9,
        y: 11,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
        range: 0,
        text_id: 1,
        is_trainer: true,
        trainer_class: 30,
        trainer_set: 19,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x18,
        x: 12,
        y: 9,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 2,
        is_trainer: true,
        trainer_class: 30,
        trainer_set: 20,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x18,
        x: 9,
        y: 7,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
        range: 0,
        text_id: 3,
        is_trainer: true,
        trainer_class: 30,
        trainer_set: 21,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x16,
        x: 10,
        y: 3,
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

pub const WARPS: [WarpData; 1] = [
WarpData {
    x: 9,
    y: 16,
    dest_map: Some(MapId::PokemonTower6F),
    dest_warp_id: 1,
}
];