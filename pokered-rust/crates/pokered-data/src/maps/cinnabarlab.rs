use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 1] = [
NpcEntry {
    sprite_id: 0x27,
    x: 1,
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

pub const SIGNS: [SignEntry; 4] = [
SignEntry {
        x: 3,
        y: 2,
        text_id: 1,
    },
    SignEntry {
        x: 9,
        y: 4,
        text_id: 2,
    },
    SignEntry {
        x: 13,
        y: 4,
        text_id: 3,
    },
    SignEntry {
        x: 17,
        y: 4,
        text_id: 4,
    },
];

pub const WARPS: [WarpData; 5] = [
WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 8,
        y: 4,
        dest_map: Some(MapId::CinnabarLabTradeRoom),
        dest_warp_id: 0,
    },
    WarpData {
        x: 12,
        y: 4,
        dest_map: Some(MapId::CinnabarLabMetronomeRoom),
        dest_warp_id: 0,
    },
    WarpData {
        x: 16,
        y: 4,
        dest_map: Some(MapId::CinnabarLabFossilRoom),
        dest_warp_id: 0,
    },
];