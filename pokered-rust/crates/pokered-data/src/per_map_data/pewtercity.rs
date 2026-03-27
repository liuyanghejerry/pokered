use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 5] = [
NpcEntry {
        sprite_id: 0x06,
        x: 8,
        y: 15,
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
        sprite_id: 0x07,
        x: 17,
        y: 25,
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
        sprite_id: 0x0C,
        x: 27,
        y: 17,
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
        sprite_id: 0x0C,
        x: 26,
        y: 25,
        movement: NpcMovement(1),
        facing: NpcFacing(2),
        range: 2,
        text_id: 4,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x04,
        x: 35,
        y: 16,
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

pub const SIGNS: [SignEntry; 7] = [
SignEntry {
        x: 19,
        y: 29,
        text_id: 1,
    },
    SignEntry {
        x: 33,
        y: 19,
        text_id: 2,
    },
    SignEntry {
        x: 24,
        y: 17,
        text_id: 3,
    },
    SignEntry {
        x: 14,
        y: 25,
        text_id: 4,
    },
    SignEntry {
        x: 15,
        y: 9,
        text_id: 5,
    },
    SignEntry {
        x: 11,
        y: 17,
        text_id: 6,
    },
    SignEntry {
        x: 25,
        y: 23,
        text_id: 7,
    },
];

pub const WARPS: [WarpData; 7] = [
WarpData {
        x: 14,
        y: 7,
        dest_map: Some(MapId::Museum1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 19,
        y: 5,
        dest_map: Some(MapId::Museum1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 16,
        y: 17,
        dest_map: Some(MapId::PewterGym),
        dest_warp_id: 0,
    },
    WarpData {
        x: 29,
        y: 13,
        dest_map: Some(MapId::PewterNidoranHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 23,
        y: 17,
        dest_map: Some(MapId::PewterMart),
        dest_warp_id: 0,
    },
    WarpData {
        x: 7,
        y: 29,
        dest_map: Some(MapId::PewterSpeechHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 13,
        y: 25,
        dest_map: Some(MapId::PewterPokecenter),
        dest_warp_id: 0,
    },
];