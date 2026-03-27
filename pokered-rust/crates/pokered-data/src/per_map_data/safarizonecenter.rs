use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 1] = [
NpcEntry {
    sprite_id: 0x3D,
    x: 14,
    y: 10,
    movement: NpcMovement(0),
    facing: NpcFacing(0),
    range: 0,
    text_id: 1,
    is_trainer: false,
    trainer_class: 0,
    trainer_set: 0,
    item_id: 0x31,
}
];

pub const SIGNS: [SignEntry; 2] = [
SignEntry {
        x: 18,
        y: 20,
        text_id: 1,
    },
    SignEntry {
        x: 14,
        y: 22,
        text_id: 2,
    },
];

pub const WARPS: [WarpData; 9] = [
WarpData {
        x: 14,
        y: 25,
        dest_map: Some(MapId::SafariZoneGate),
        dest_warp_id: 2,
    },
    WarpData {
        x: 15,
        y: 25,
        dest_map: Some(MapId::SafariZoneGate),
        dest_warp_id: 3,
    },
    WarpData {
        x: 0,
        y: 10,
        dest_map: Some(MapId::SafariZoneWest),
        dest_warp_id: 4,
    },
    WarpData {
        x: 0,
        y: 11,
        dest_map: Some(MapId::SafariZoneWest),
        dest_warp_id: 5,
    },
    WarpData {
        x: 14,
        y: 0,
        dest_map: Some(MapId::SafariZoneNorth),
        dest_warp_id: 4,
    },
    WarpData {
        x: 15,
        y: 0,
        dest_map: Some(MapId::SafariZoneNorth),
        dest_warp_id: 5,
    },
    WarpData {
        x: 29,
        y: 10,
        dest_map: Some(MapId::SafariZoneEast),
        dest_warp_id: 2,
    },
    WarpData {
        x: 29,
        y: 11,
        dest_map: Some(MapId::SafariZoneEast),
        dest_warp_id: 3,
    },
    WarpData {
        x: 17,
        y: 19,
        dest_map: Some(MapId::SafariZoneCenterRestHouse),
        dest_warp_id: 0,
    },
];