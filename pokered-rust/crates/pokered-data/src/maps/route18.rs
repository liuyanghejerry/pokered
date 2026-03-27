use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 3] = [
NpcEntry {
        sprite_id: 0x07,
        x: 36,
        y: 11,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
        range: 0,
        text_id: 1,
        is_trainer: true,
        trainer_class: 23,
        trainer_set: 8,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x07,
        x: 40,
        y: 15,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 2,
        is_trainer: true,
        trainer_class: 23,
        trainer_set: 9,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x07,
        x: 42,
        y: 13,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 3,
        is_trainer: true,
        trainer_class: 23,
        trainer_set: 10,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 2] = [
SignEntry {
        x: 43,
        y: 7,
        text_id: 1,
    },
    SignEntry {
        x: 33,
        y: 5,
        text_id: 2,
    },
];

pub const WARPS: [WarpData; 4] = [
WarpData {
        x: 33,
        y: 8,
        dest_map: Some(MapId::Route18Gate1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 33,
        y: 9,
        dest_map: Some(MapId::Route18Gate1F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 40,
        y: 8,
        dest_map: Some(MapId::Route18Gate1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 40,
        y: 9,
        dest_map: Some(MapId::Route18Gate1F),
        dest_warp_id: 3,
    },
];