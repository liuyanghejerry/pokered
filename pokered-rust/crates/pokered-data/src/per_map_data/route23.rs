use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 7] = [
NpcEntry {
        sprite_id: 0x31,
        x: 4,
        y: 35,
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
        sprite_id: 0x31,
        x: 10,
        y: 56,
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
        sprite_id: 0x22,
        x: 8,
        y: 85,
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
        sprite_id: 0x22,
        x: 11,
        y: 96,
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
        sprite_id: 0x31,
        x: 12,
        y: 105,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 5,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x31,
        x: 8,
        y: 119,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 6,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x31,
        x: 8,
        y: 136,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 7,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 1] = [
SignEntry {
    x: 3,
    y: 33,
    text_id: 1,
}
];

pub const WARPS: [WarpData; 4] = [
WarpData {
        x: 7,
        y: 139,
        dest_map: Some(MapId::Route22Gate),
        dest_warp_id: 2,
    },
    WarpData {
        x: 8,
        y: 139,
        dest_map: Some(MapId::Route22Gate),
        dest_warp_id: 3,
    },
    WarpData {
        x: 4,
        y: 31,
        dest_map: Some(MapId::VictoryRoad1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 14,
        y: 31,
        dest_map: Some(MapId::VictoryRoad2F),
        dest_warp_id: 1,
    },
];