use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 4] = [
NpcEntry {
        sprite_id: 0x3D,
        x: 21,
        y: 10,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x10,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 3,
        y: 7,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x11,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 20,
        y: 13,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 3,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x26,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 15,
        y: 12,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 4,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0xED,
    },
];

pub const SIGNS: [SignEntry; 3] = [
SignEntry {
        x: 26,
        y: 10,
        text_id: 1,
    },
    SignEntry {
        x: 6,
        y: 4,
        text_id: 2,
    },
    SignEntry {
        x: 5,
        y: 23,
        text_id: 3,
    },
];

pub const WARPS: [WarpData; 5] = [
WarpData {
        x: 0,
        y: 4,
        dest_map: Some(MapId::SafariZoneNorth),
        dest_warp_id: 6,
    },
    WarpData {
        x: 0,
        y: 5,
        dest_map: Some(MapId::SafariZoneNorth),
        dest_warp_id: 7,
    },
    WarpData {
        x: 0,
        y: 22,
        dest_map: Some(MapId::SafariZoneCenter),
        dest_warp_id: 6,
    },
    WarpData {
        x: 0,
        y: 23,
        dest_map: Some(MapId::SafariZoneCenter),
        dest_warp_id: 6,
    },
    WarpData {
        x: 25,
        y: 9,
        dest_map: Some(MapId::SafariZoneEastRestHouse),
        dest_warp_id: 0,
    },
];