use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 4] = [
    NpcEntry {
        sprite_id: 0x05,
        x: 0,
        y: 5,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
        range: 0,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x28,
        x: 1,
        y: 5,
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
        sprite_id: 0x38,
        x: 1,
        y: 8,
        movement: NpcMovement(1),
        facing: NpcFacing(2),
        range: 2,
        text_id: 3,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x05,
        x: 4,
        y: 4,
        movement: NpcMovement(1),
        facing: NpcFacing(0),
        range: 1,
        text_id: 4,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 1] = [
SignEntry {
    x: 4,
    y: 9,
    text_id: 1,
}
];

pub const WARPS: [WarpData; 5] = [
WarpData {
        x: 4,
        y: 11,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 5,
        y: 11,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 4,
        y: 0,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 7,
        y: 1,
        dest_map: Some(MapId::CeladonMansion2F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 2,
        y: 1,
        dest_map: Some(MapId::CeladonMansion2F),
        dest_warp_id: 2,
    },
];
