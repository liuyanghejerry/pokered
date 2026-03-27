use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 1] = [
NpcEntry {
    sprite_id: 0x2A,
    x: 8,
    y: 3,
    movement: NpcMovement(0),
    facing: NpcFacing(0),
    range: 0,
    text_id: 1,
    is_trainer: false,
    trainer_class: 0,
    trainer_set: 0,
    item_id: 0x00,
}
];

pub const SIGNS: [SignEntry; 2] = [
SignEntry {
        x: 11,
        y: 4,
        text_id: 1,
    },
    SignEntry {
        x: 14,
        y: 1,
        text_id: 2,
    },
];

pub const WARPS: [WarpData; 6] = [
WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 16,
        y: 7,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 17,
        y: 7,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 12,
        y: 1,
        dest_map: Some(MapId::CeladonMart2F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 1,
        y: 1,
        dest_map: Some(MapId::CeladonMartElevator),
        dest_warp_id: 0,
    },
];