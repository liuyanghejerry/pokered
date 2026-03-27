use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 3] = [
NpcEntry {
        sprite_id: 0x06,
        x: 9,
        y: 8,
        movement: NpcMovement(1),
        facing: NpcFacing(0),
        range: 0,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x06,
        x: 63,
        y: 3,
        movement: NpcMovement(0),
        facing: NpcFacing(3),
        range: 0,
        text_id: 2,
        is_trainer: true,
        trainer_class: 3,
        trainer_set: 4,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 57,
        y: 3,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 3,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0xCC,
    },
];

pub const SIGNS: [SignEntry; 3] = [
SignEntry {
        x: 12,
        y: 5,
        text_id: 1,
    },
    SignEntry {
        x: 17,
        y: 7,
        text_id: 2,
    },
    SignEntry {
        x: 27,
        y: 7,
        text_id: 3,
    },
];

pub const WARPS: [WarpData; 3] = [
WarpData {
        x: 11,
        y: 5,
        dest_map: Some(MapId::MtMoonPokecenter),
        dest_warp_id: 0,
    },
    WarpData {
        x: 18,
        y: 5,
        dest_map: Some(MapId::MtMoon1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 24,
        y: 5,
        dest_map: Some(MapId::MtMoonB1F),
        dest_warp_id: 7,
    },
];