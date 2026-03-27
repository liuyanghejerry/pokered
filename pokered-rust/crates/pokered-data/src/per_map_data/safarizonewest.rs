use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 4] = [
NpcEntry {
        sprite_id: 0x3D,
        x: 8,
        y: 20,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x11,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 9,
        y: 7,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0xE8,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 18,
        y: 18,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 3,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x36,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 19,
        y: 7,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 4,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x40,
    },
];

pub const SIGNS: [SignEntry; 4] = [
SignEntry {
        x: 12,
        y: 12,
        text_id: 1,
    },
    SignEntry {
        x: 17,
        y: 3,
        text_id: 2,
    },
    SignEntry {
        x: 26,
        y: 4,
        text_id: 3,
    },
    SignEntry {
        x: 24,
        y: 22,
        text_id: 4,
    },
];

pub const WARPS: [WarpData; 8] = [
WarpData {
        x: 20,
        y: 0,
        dest_map: Some(MapId::SafariZoneNorth),
        dest_warp_id: 0,
    },
    WarpData {
        x: 21,
        y: 0,
        dest_map: Some(MapId::SafariZoneNorth),
        dest_warp_id: 1,
    },
    WarpData {
        x: 26,
        y: 0,
        dest_map: Some(MapId::SafariZoneNorth),
        dest_warp_id: 2,
    },
    WarpData {
        x: 27,
        y: 0,
        dest_map: Some(MapId::SafariZoneNorth),
        dest_warp_id: 3,
    },
    WarpData {
        x: 29,
        y: 22,
        dest_map: Some(MapId::SafariZoneCenter),
        dest_warp_id: 2,
    },
    WarpData {
        x: 29,
        y: 23,
        dest_map: Some(MapId::SafariZoneCenter),
        dest_warp_id: 3,
    },
    WarpData {
        x: 3,
        y: 3,
        dest_map: Some(MapId::SafariZoneSecretHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 11,
        y: 11,
        dest_map: Some(MapId::SafariZoneWestRestHouse),
        dest_warp_id: 0,
    },
];