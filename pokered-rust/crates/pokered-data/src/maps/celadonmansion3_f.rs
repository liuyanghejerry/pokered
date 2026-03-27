use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 4] = [
NpcEntry {
        sprite_id: 0x15,
        x: 0,
        y: 4,
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
        sprite_id: 0x26,
        x: 3,
        y: 4,
        movement: NpcMovement(0),
        facing: NpcFacing(1),
        range: 0,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0C,
        x: 0,
        y: 7,
        movement: NpcMovement(0),
        facing: NpcFacing(1),
        range: 0,
        text_id: 3,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x2C,
        x: 2,
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

pub const SIGNS: [SignEntry; 4] = [
SignEntry {
        x: 1,
        y: 3,
        text_id: 1,
    },
    SignEntry {
        x: 4,
        y: 3,
        text_id: 2,
    },
    SignEntry {
        x: 1,
        y: 6,
        text_id: 3,
    },
    SignEntry {
        x: 4,
        y: 9,
        text_id: 4,
    },
];

pub const WARPS: [WarpData; 4] = [
WarpData {
        x: 6,
        y: 1,
        dest_map: Some(MapId::CeladonMansion2F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 7,
        y: 1,
        dest_map: Some(MapId::CeladonMansionRoof),
        dest_warp_id: 0,
    },
    WarpData {
        x: 2,
        y: 1,
        dest_map: Some(MapId::CeladonMansionRoof),
        dest_warp_id: 1,
    },
    WarpData {
        x: 4,
        y: 1,
        dest_map: Some(MapId::CeladonMansion2F),
        dest_warp_id: 3,
    },
];