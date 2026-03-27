use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 2] = [
NpcEntry {
        sprite_id: 0x3D,
        x: 25,
        y: 1,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x24,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 19,
        y: 7,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0xF0,
    },
];

pub const SIGNS: [SignEntry; 5] = [
SignEntry {
        x: 36,
        y: 4,
        text_id: 1,
    },
    SignEntry {
        x: 4,
        y: 25,
        text_id: 2,
    },
    SignEntry {
        x: 13,
        y: 31,
        text_id: 3,
    },
    SignEntry {
        x: 19,
        y: 33,
        text_id: 4,
    },
    SignEntry {
        x: 26,
        y: 28,
        text_id: 5,
    },
];

pub const WARPS: [WarpData; 9] = [
WarpData {
        x: 2,
        y: 35,
        dest_map: Some(MapId::SafariZoneWest),
        dest_warp_id: 0,
    },
    WarpData {
        x: 3,
        y: 35,
        dest_map: Some(MapId::SafariZoneWest),
        dest_warp_id: 1,
    },
    WarpData {
        x: 8,
        y: 35,
        dest_map: Some(MapId::SafariZoneWest),
        dest_warp_id: 2,
    },
    WarpData {
        x: 9,
        y: 35,
        dest_map: Some(MapId::SafariZoneWest),
        dest_warp_id: 3,
    },
    WarpData {
        x: 20,
        y: 35,
        dest_map: Some(MapId::SafariZoneCenter),
        dest_warp_id: 4,
    },
    WarpData {
        x: 21,
        y: 35,
        dest_map: Some(MapId::SafariZoneCenter),
        dest_warp_id: 5,
    },
    WarpData {
        x: 39,
        y: 30,
        dest_map: Some(MapId::SafariZoneEast),
        dest_warp_id: 0,
    },
    WarpData {
        x: 39,
        y: 31,
        dest_map: Some(MapId::SafariZoneEast),
        dest_warp_id: 1,
    },
    WarpData {
        x: 35,
        y: 3,
        dest_map: Some(MapId::SafariZoneNorthRestHouse),
        dest_warp_id: 0,
    },
];