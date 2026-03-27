use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 6] = [
NpcEntry {
        sprite_id: 0x0C,
        x: 10,
        y: 44,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 1,
        is_trainer: true,
        trainer_class: 7,
        trainer_set: 1,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0E,
        x: 3,
        y: 57,
        movement: NpcMovement(0),
        facing: NpcFacing(1),
        range: 0,
        text_id: 2,
        is_trainer: true,
        trainer_class: 9,
        trainer_set: 7,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0C,
        x: 14,
        y: 64,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 3,
        is_trainer: true,
        trainer_class: 7,
        trainer_set: 2,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x06,
        x: 7,
        y: 25,
        movement: NpcMovement(0),
        facing: NpcFacing(2),
        range: 0,
        text_id: 4,
        is_trainer: true,
        trainer_class: 6,
        trainer_set: 7,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0E,
        x: 3,
        y: 61,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 5,
        is_trainer: true,
        trainer_class: 9,
        trainer_set: 8,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x06,
        x: 7,
        y: 54,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 6,
        is_trainer: true,
        trainer_class: 6,
        trainer_set: 8,
        item_id: 0x00,
    },
];

pub const SIGNS: [SignEntry; 4] = [
SignEntry {
        x: 7,
        y: 19,
        text_id: 1,
    },
    SignEntry {
        x: 12,
        y: 19,
        text_id: 2,
    },
    SignEntry {
        x: 9,
        y: 55,
        text_id: 3,
    },
    SignEntry {
        x: 5,
        y: 41,
        text_id: 4,
    },
];

pub const WARPS: [WarpData; 4] = [
WarpData {
        x: 11,
        y: 19,
        dest_map: Some(MapId::RockTunnelPokecenter),
        dest_warp_id: 0,
    },
    WarpData {
        x: 8,
        y: 17,
        dest_map: Some(MapId::RockTunnel1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 8,
        y: 53,
        dest_map: Some(MapId::RockTunnel1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 6,
        y: 39,
        dest_map: Some(MapId::PowerPlant),
        dest_warp_id: 0,
    },
];