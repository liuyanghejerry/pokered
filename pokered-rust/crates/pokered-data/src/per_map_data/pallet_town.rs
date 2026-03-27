use crate::map_objects::WarpData;
use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;

pub const NPCS: [NpcEntry; 3] = [
    NpcEntry {
        sprite_id: 0x03,
        x: 8,
        y: 5,
        movement: NpcMovement::STATIONARY,
        facing: NpcFacing::DOWN,
        range: 0,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0,
    },
    NpcEntry {
        sprite_id: 0x0D,
        x: 3,
        y: 8,
        movement: NpcMovement::WANDER,
        facing: NpcFacing::DOWN,
        range: 0,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0,
    },
    NpcEntry {
        sprite_id: 0x2F,
        x: 11,
        y: 14,
        movement: NpcMovement::WANDER,
        facing: NpcFacing::DOWN,
        range: 0,
        text_id: 3,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0,
    },
];

pub const SIGNS: [SignEntry; 4] = [
    SignEntry {
        x: 13,
        y: 13,
        text_id: 1,
    },
    SignEntry {
        x: 7,
        y: 9,
        text_id: 2,
    },
    SignEntry {
        x: 3,
        y: 5,
        text_id: 3,
    },
    SignEntry {
        x: 11,
        y: 5,
        text_id: 4,
    },
];

pub const WARPS: [WarpData; 3] = [
    WarpData {
        x: 5,
        y: 5,
        dest_map: Some(MapId::RedsHouse1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 13,
        y: 5,
        dest_map: Some(MapId::BluesHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 12,
        y: 11,
        dest_map: Some(MapId::OaksLab),
        dest_warp_id: 1,
    },
];
