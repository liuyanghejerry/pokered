use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 4] = [
NpcEntry {
        sprite_id: 0x10,
        x: 14,
        y: 5,
        movement: NpcMovement(1),
        facing: NpcFacing(0),
        range: 1,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x13,
        x: 2,
        y: 6,
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
        sprite_id: 0x26,
        x: 5,
        y: 3,
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
        sprite_id: 0x26,
        x: 6,
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

pub const SIGNS: [SignEntry; 1] = [
SignEntry {
    x: 14,
    y: 1,
    text_id: 1,
}
];

pub const WARPS: [WarpData; 3] = [
WarpData {
        x: 12,
        y: 1,
        dest_map: Some(MapId::CeladonMartRoof),
        dest_warp_id: 0,
    },
    WarpData {
        x: 16,
        y: 1,
        dest_map: Some(MapId::CeladonMart4F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 1,
        y: 1,
        dest_map: Some(MapId::CeladonMartElevator),
        dest_warp_id: 0,
    },
];