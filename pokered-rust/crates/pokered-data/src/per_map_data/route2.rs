use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 2] = [
NpcEntry {
        sprite_id: 0x3D,
        x: 13,
        y: 54,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x0A,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 13,
        y: 45,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x23,
    },
];

pub const SIGNS: [SignEntry; 2] = [
SignEntry {
        x: 5,
        y: 65,
        text_id: 1,
    },
    SignEntry {
        x: 11,
        y: 11,
        text_id: 2,
    },
];

pub const WARPS: [WarpData; 6] = [
WarpData {
        x: 12,
        y: 9,
        dest_map: Some(MapId::DiglettsCaveRoute2),
        dest_warp_id: 0,
    },
    WarpData {
        x: 3,
        y: 11,
        dest_map: Some(MapId::ViridianForestNorthGate),
        dest_warp_id: 1,
    },
    WarpData {
        x: 15,
        y: 19,
        dest_map: Some(MapId::Route2TradeHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 16,
        y: 35,
        dest_map: Some(MapId::Route2Gate),
        dest_warp_id: 1,
    },
    WarpData {
        x: 15,
        y: 39,
        dest_map: Some(MapId::Route2Gate),
        dest_warp_id: 2,
    },
    WarpData {
        x: 3,
        y: 43,
        dest_map: Some(MapId::ViridianForestSouthGate),
        dest_warp_id: 2,
    },
];