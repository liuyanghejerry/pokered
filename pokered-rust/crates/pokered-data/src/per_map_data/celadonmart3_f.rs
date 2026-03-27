use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 5] = [
NpcEntry {
        sprite_id: 0x26,
        x: 16,
        y: 5,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x37,
        x: 11,
        y: 6,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
        range: 0,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x37,
        x: 7,
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
        sprite_id: 0x37,
        x: 8,
        y: 2,
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
        sprite_id: 0x35,
        x: 2,
        y: 5,
        movement: NpcMovement(0),
        facing: NpcFacing(1),
        range: 0,
        text_id: 5,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 12] = [
SignEntry {
        x: 2,
        y: 4,
        text_id: 1,
    },
    SignEntry {
        x: 3,
        y: 4,
        text_id: 2,
    },
    SignEntry {
        x: 5,
        y: 4,
        text_id: 3,
    },
    SignEntry {
        x: 6,
        y: 4,
        text_id: 4,
    },
    SignEntry {
        x: 2,
        y: 6,
        text_id: 5,
    },
    SignEntry {
        x: 3,
        y: 6,
        text_id: 6,
    },
    SignEntry {
        x: 5,
        y: 6,
        text_id: 7,
    },
    SignEntry {
        x: 6,
        y: 6,
        text_id: 8,
    },
    SignEntry {
        x: 14,
        y: 1,
        text_id: 9,
    },
    SignEntry {
        x: 4,
        y: 1,
        text_id: 10,
    },
    SignEntry {
        x: 6,
        y: 1,
        text_id: 11,
    },
    SignEntry {
        x: 10,
        y: 1,
        text_id: 12,
    },
];

pub const WARPS: [WarpData; 3] = [
WarpData {
        x: 12,
        y: 1,
        dest_map: Some(MapId::CeladonMart4F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 16,
        y: 1,
        dest_map: Some(MapId::CeladonMart2F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 1,
        y: 1,
        dest_map: Some(MapId::CeladonMartElevator),
        dest_warp_id: 0,
    },
];