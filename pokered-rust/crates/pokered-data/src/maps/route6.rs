use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 6] = [
NpcEntry {
        sprite_id: 0x07,
        x: 10,
        y: 21,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
        range: 0,
        text_id: 1,
        is_trainer: true,
        trainer_class: 5,
        trainer_set: 4,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x06,
        x: 11,
        y: 21,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 2,
        is_trainer: true,
        trainer_class: 6,
        trainer_set: 2,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x04,
        x: 0,
        y: 15,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
        range: 0,
        text_id: 3,
        is_trainer: true,
        trainer_class: 2,
        trainer_set: 10,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x07,
        x: 11,
        y: 31,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 4,
        is_trainer: true,
        trainer_class: 5,
        trainer_set: 5,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x06,
        x: 11,
        y: 30,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 5,
        is_trainer: true,
        trainer_class: 6,
        trainer_set: 3,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x04,
        x: 19,
        y: 26,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 6,
        is_trainer: true,
        trainer_class: 2,
        trainer_set: 11,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 1] = [
SignEntry {
    x: 19,
    y: 15,
    text_id: 1,
}
];

pub const WARPS: [WarpData; 4] = [
WarpData {
        x: 9,
        y: 1,
        dest_map: Some(MapId::Route6Gate),
        dest_warp_id: 2,
    },
    WarpData {
        x: 10,
        y: 1,
        dest_map: Some(MapId::Route6Gate),
        dest_warp_id: 2,
    },
    WarpData {
        x: 10,
        y: 7,
        dest_map: Some(MapId::Route6Gate),
        dest_warp_id: 0,
    },
    WarpData {
        x: 17,
        y: 13,
        dest_map: Some(MapId::UndergroundPathRoute6),
        dest_warp_id: 0,
    },
];