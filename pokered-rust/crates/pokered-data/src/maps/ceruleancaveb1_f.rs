use crate::maps::MapId;
use crate::npc_data::{NpcEntry, NpcFacing, NpcMovement};
use crate::sign_data::SignEntry;
use crate::map_objects::WarpData;

pub const NPCS: [NpcEntry; 3] = [
NpcEntry {
        sprite_id: 0x05,
        x: 27,
        y: 13,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 1,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 16,
        y: 9,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 2,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x02,
    },
    NpcEntry {
        sprite_id: 0x3D,
        x: 18,
        y: 1,
        movement: NpcMovement(0),
        facing: NpcFacing(0),
        range: 0,
        text_id: 3,
        is_trainer: false,
        trainer_class: 0,
        trainer_set: 0,
        item_id: 0x36,
    },
];

pub const SIGNS: [SignEntry; 0] = [

];

pub const WARPS: [WarpData; 1] = [
WarpData {
    x: 3,
    y: 6,
    dest_map: Some(MapId::CeruleanCave1F),
    dest_warp_id: 8,
}
];