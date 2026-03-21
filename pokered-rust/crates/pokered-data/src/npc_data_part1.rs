use crate::npc_data::{NpcEntry, NpcMovement, NpcFacing};

pub static NPCS_AGATHASROOM: [NpcEntry; 1] = [
    NpcEntry {
        sprite_id: 0x39, x: 5, y: 2,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: true,
        trainer_class: 46, trainer_set: 1,
        item_id: 0x00,
    },
];

pub static NPCS_BIKESHOP: [NpcEntry; 3] = [
    NpcEntry {
        sprite_id: 0x15, x: 6, y: 2,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x1C, x: 5, y: 6,
        movement: NpcMovement(1), facing: NpcFacing(0),
        range: 1, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x04, x: 1, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(1),
        range: 0, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_BILLSHOUSE: [NpcEntry; 3] = [
    NpcEntry {
        sprite_id: 0x05, x: 6, y: 5,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0C, x: 4, y: 4,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0C, x: 6, y: 5,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_BLUESHOUSE: [NpcEntry; 3] = [
    NpcEntry {
        sprite_id: 0x11, x: 2, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(3),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x11, x: 6, y: 4,
        movement: NpcMovement(1), facing: NpcFacing(0),
        range: 1, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x41, x: 3, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_BRUNOSROOM: [NpcEntry; 1] = [
    NpcEntry {
        sprite_id: 0x3A, x: 5, y: 2,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: true,
        trainer_class: 33, trainer_set: 1,
        item_id: 0x00,
    },
];

pub static NPCS_CELADONCHIEFHOUSE: [NpcEntry; 3] = [
    NpcEntry {
        sprite_id: 0x25, x: 4, y: 2,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x18, x: 1, y: 4,
        movement: NpcMovement(1), facing: NpcFacing(0),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x13, x: 5, y: 6,
        movement: NpcMovement(0), facing: NpcFacing(2),
        range: 0, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CELADONCITY: [NpcEntry; 9] = [
    NpcEntry {
        sprite_id: 0x08, x: 8, y: 17,
        movement: NpcMovement(1), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x25, x: 11, y: 28,
        movement: NpcMovement(0), facing: NpcFacing(1),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0D, x: 14, y: 19,
        movement: NpcMovement(1), facing: NpcFacing(0),
        range: 1, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x25, x: 25, y: 22,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 4,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x25, x: 22, y: 16,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 5,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x2F, x: 32, y: 12,
        movement: NpcMovement(0), facing: NpcFacing(2),
        range: 0, text_id: 6,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x05, x: 30, y: 12,
        movement: NpcMovement(0), facing: NpcFacing(3),
        range: 0, text_id: 7,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x18, x: 32, y: 29,
        movement: NpcMovement(1), facing: NpcFacing(2),
        range: 2, text_id: 8,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x18, x: 42, y: 14,
        movement: NpcMovement(1), facing: NpcFacing(2),
        range: 2, text_id: 9,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CELADONDINER: [NpcEntry; 5] = [
    NpcEntry {
        sprite_id: 0x14, x: 8, y: 5,
        movement: NpcMovement(1), facing: NpcFacing(2),
        range: 2, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x1C, x: 7, y: 2,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0A, x: 1, y: 4,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x2F, x: 5, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(3),
        range: 0, text_id: 4,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x24, x: 0, y: 1,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 5,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CELADONGYM: [NpcEntry; 8] = [
    NpcEntry {
        sprite_id: 0x1B, x: 4, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: true,
        trainer_class: 37, trainer_set: 1,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x06, x: 2, y: 11,
        movement: NpcMovement(0), facing: NpcFacing(3),
        range: 0, text_id: 2,
        is_trainer: true,
        trainer_class: 3, trainer_set: 17,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0F, x: 7, y: 10,
        movement: NpcMovement(0), facing: NpcFacing(2),
        range: 0, text_id: 3,
        is_trainer: true,
        trainer_class: 18, trainer_set: 1,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x06, x: 9, y: 5,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 4,
        is_trainer: true,
        trainer_class: 6, trainer_set: 11,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0F, x: 1, y: 5,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 5,
        is_trainer: true,
        trainer_class: 18, trainer_set: 2,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x06, x: 6, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 6,
        is_trainer: true,
        trainer_class: 3, trainer_set: 18,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0F, x: 3, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 7,
        is_trainer: true,
        trainer_class: 18, trainer_set: 3,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x06, x: 5, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 8,
        is_trainer: true,
        trainer_class: 32, trainer_set: 1,
        item_id: 0x00,
    },
];

pub static NPCS_CELADONHOTEL: [NpcEntry; 3] = [
    NpcEntry {
        sprite_id: 0x28, x: 3, y: 1,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0F, x: 2, y: 4,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0C, x: 8, y: 4,
        movement: NpcMovement(1), facing: NpcFacing(2),
        range: 2, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CELADONMANSION1F: [NpcEntry; 4] = [
    NpcEntry {
        sprite_id: 0x05, x: 0, y: 5,
        movement: NpcMovement(0), facing: NpcFacing(3),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x28, x: 1, y: 5,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x38, x: 1, y: 8,
        movement: NpcMovement(1), facing: NpcFacing(2),
        range: 2, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x05, x: 4, y: 4,
        movement: NpcMovement(1), facing: NpcFacing(0),
        range: 1, text_id: 4,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CELADONMANSION2F: [NpcEntry; 0] = [];

pub static NPCS_CELADONMANSION3F: [NpcEntry; 4] = [
    NpcEntry {
        sprite_id: 0x15, x: 0, y: 4,
        movement: NpcMovement(0), facing: NpcFacing(1),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x26, x: 3, y: 4,
        movement: NpcMovement(0), facing: NpcFacing(1),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0C, x: 0, y: 7,
        movement: NpcMovement(0), facing: NpcFacing(1),
        range: 0, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x2C, x: 2, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 4,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CELADONMANSIONROOF: [NpcEntry; 0] = [];

pub static NPCS_CELADONMANSIONROOFHOUSE: [NpcEntry; 2] = [
    NpcEntry {
        sprite_id: 0x0E, x: 2, y: 2,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3D, x: 4, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CELADONMART1F: [NpcEntry; 1] = [
    NpcEntry {
        sprite_id: 0x2A, x: 8, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CELADONMART2F: [NpcEntry; 4] = [
    NpcEntry {
        sprite_id: 0x26, x: 5, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x26, x: 6, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0A, x: 19, y: 5,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0D, x: 14, y: 4,
        movement: NpcMovement(1), facing: NpcFacing(0),
        range: 1, text_id: 4,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CELADONMART3F: [NpcEntry; 5] = [
    NpcEntry {
        sprite_id: 0x26, x: 16, y: 5,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x37, x: 11, y: 6,
        movement: NpcMovement(0), facing: NpcFacing(3),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x37, x: 7, y: 2,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x37, x: 8, y: 2,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 4,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x35, x: 2, y: 5,
        movement: NpcMovement(0), facing: NpcFacing(1),
        range: 0, text_id: 5,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CELADONMART4F: [NpcEntry; 3] = [
    NpcEntry {
        sprite_id: 0x26, x: 5, y: 7,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0C, x: 15, y: 5,
        movement: NpcMovement(1), facing: NpcFacing(2),
        range: 2, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x04, x: 5, y: 2,
        movement: NpcMovement(1), facing: NpcFacing(2),
        range: 2, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CELADONMART5F: [NpcEntry; 4] = [
    NpcEntry {
        sprite_id: 0x10, x: 14, y: 5,
        movement: NpcMovement(1), facing: NpcFacing(0),
        range: 1, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x13, x: 2, y: 6,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x26, x: 5, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x26, x: 6, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 4,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CELADONMARTELEVATOR: [NpcEntry; 0] = [];

pub static NPCS_CELADONMARTROOF: [NpcEntry; 2] = [
    NpcEntry {
        sprite_id: 0x0C, x: 10, y: 4,
        movement: NpcMovement(0), facing: NpcFacing(2),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x08, x: 5, y: 5,
        movement: NpcMovement(1), facing: NpcFacing(0),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CELADONPOKECENTER: [NpcEntry; 4] = [
    NpcEntry {
        sprite_id: 0x29, x: 3, y: 1,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x10, x: 7, y: 3,
        movement: NpcMovement(1), facing: NpcFacing(2),
        range: 2, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0F, x: 10, y: 5,
        movement: NpcMovement(1), facing: NpcFacing(0),
        range: 0, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x2A, x: 11, y: 2,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 4,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CERULEANBADGEHOUSE: [NpcEntry; 1] = [
    NpcEntry {
        sprite_id: 0x0A, x: 5, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(3),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CERULEANCAVE1F: [NpcEntry; 3] = [
    NpcEntry {
        sprite_id: 0x3D, x: 7, y: 13,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x10,
    },
    NpcEntry {
        sprite_id: 0x3D, x: 19, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x53,
    },
    NpcEntry {
        sprite_id: 0x3D, x: 5, y: 0,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x31,
    },
];

pub static NPCS_CERULEANCAVE2F: [NpcEntry; 3] = [
    NpcEntry {
        sprite_id: 0x3D, x: 29, y: 9,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x4F,
    },
    NpcEntry {
        sprite_id: 0x3D, x: 4, y: 15,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x02,
    },
    NpcEntry {
        sprite_id: 0x3D, x: 13, y: 6,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x10,
    },
];

pub static NPCS_CERULEANCAVEB1F: [NpcEntry; 3] = [
    NpcEntry {
        sprite_id: 0x05, x: 27, y: 13,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3D, x: 16, y: 9,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x02,
    },
    NpcEntry {
        sprite_id: 0x3D, x: 18, y: 1,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x36,
    },
];

pub static NPCS_CERULEANCITY: [NpcEntry; 11] = [
    NpcEntry {
        sprite_id: 0x02, x: 20, y: 2,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x18, x: 30, y: 8,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 2,
        is_trainer: true,
        trainer_class: 30, trainer_set: 5,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x07, x: 31, y: 20,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0C, x: 15, y: 18,
        movement: NpcMovement(1), facing: NpcFacing(0),
        range: 1, text_id: 4,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0C, x: 9, y: 21,
        movement: NpcMovement(1), facing: NpcFacing(2),
        range: 2, text_id: 5,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x31, x: 28, y: 12,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 6,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x06, x: 29, y: 26,
        movement: NpcMovement(0), facing: NpcFacing(2),
        range: 0, text_id: 7,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x05, x: 28, y: 26,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 8,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x06, x: 9, y: 27,
        movement: NpcMovement(1), facing: NpcFacing(2),
        range: 2, text_id: 9,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0C, x: 4, y: 12,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 10,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x31, x: 27, y: 12,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 11,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CERULEANGYM: [NpcEntry; 4] = [
    NpcEntry {
        sprite_id: 0x1D, x: 4, y: 2,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: true,
        trainer_class: 35, trainer_set: 1,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x06, x: 2, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(3),
        range: 0, text_id: 2,
        is_trainer: true,
        trainer_class: 6, trainer_set: 1,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x22, x: 8, y: 7,
        movement: NpcMovement(0), facing: NpcFacing(2),
        range: 0, text_id: 3,
        is_trainer: true,
        trainer_class: 15, trainer_set: 1,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x24, x: 7, y: 10,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 4,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CERULEANMART: [NpcEntry; 3] = [
    NpcEntry {
        sprite_id: 0x26, x: 0, y: 5,
        movement: NpcMovement(0), facing: NpcFacing(3),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x07, x: 3, y: 4,
        movement: NpcMovement(1), facing: NpcFacing(0),
        range: 1, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x06, x: 6, y: 2,
        movement: NpcMovement(1), facing: NpcFacing(2),
        range: 2, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CERULEANPOKECENTER: [NpcEntry; 4] = [
    NpcEntry {
        sprite_id: 0x29, x: 3, y: 1,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0C, x: 10, y: 5,
        movement: NpcMovement(1), facing: NpcFacing(0),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x10, x: 4, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x2A, x: 11, y: 2,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 4,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CERULEANTRADEHOUSE: [NpcEntry; 2] = [
    NpcEntry {
        sprite_id: 0x28, x: 5, y: 4,
        movement: NpcMovement(0), facing: NpcFacing(2),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0B, x: 1, y: 2,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CERULEANTRASHEDHOUSE: [NpcEntry; 2] = [
    NpcEntry {
        sprite_id: 0x27, x: 2, y: 1,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0D, x: 5, y: 6,
        movement: NpcMovement(1), facing: NpcFacing(2),
        range: 2, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CHAMPIONSROOM: [NpcEntry; 2] = [
    NpcEntry {
        sprite_id: 0x02, x: 4, y: 2,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x03, x: 3, y: 7,
        movement: NpcMovement(0), facing: NpcFacing(1),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CINNABARGYM: [NpcEntry; 9] = [
    NpcEntry {
        sprite_id: 0x0A, x: 3, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: true,
        trainer_class: 39, trainer_set: 1,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0C, x: 17, y: 2,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 2,
        is_trainer: true,
        trainer_class: 8, trainer_set: 9,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0C, x: 17, y: 8,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 3,
        is_trainer: true,
        trainer_class: 11, trainer_set: 4,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0C, x: 11, y: 4,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 4,
        is_trainer: true,
        trainer_class: 8, trainer_set: 10,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0C, x: 11, y: 8,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 5,
        is_trainer: true,
        trainer_class: 11, trainer_set: 5,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0C, x: 11, y: 14,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 6,
        is_trainer: true,
        trainer_class: 8, trainer_set: 11,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0C, x: 3, y: 14,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 7,
        is_trainer: true,
        trainer_class: 11, trainer_set: 6,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0C, x: 3, y: 8,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 8,
        is_trainer: true,
        trainer_class: 8, trainer_set: 12,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x24, x: 16, y: 13,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 9,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CINNABARISLAND: [NpcEntry; 2] = [
    NpcEntry {
        sprite_id: 0x0D, x: 12, y: 5,
        movement: NpcMovement(1), facing: NpcFacing(2),
        range: 2, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0B, x: 14, y: 6,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CINNABARLAB: [NpcEntry; 1] = [
    NpcEntry {
        sprite_id: 0x27, x: 1, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CINNABARLABFOSSILROOM: [NpcEntry; 2] = [
    NpcEntry {
        sprite_id: 0x20, x: 5, y: 2,
        movement: NpcMovement(1), facing: NpcFacing(2),
        range: 2, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x20, x: 7, y: 6,
        movement: NpcMovement(0), facing: NpcFacing(1),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CINNABARLABMETRONOMEROOM: [NpcEntry; 2] = [
    NpcEntry {
        sprite_id: 0x20, x: 7, y: 2,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x20, x: 2, y: 3,
        movement: NpcMovement(1), facing: NpcFacing(2),
        range: 2, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CINNABARLABTRADEROOM: [NpcEntry; 3] = [
    NpcEntry {
        sprite_id: 0x0C, x: 3, y: 2,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x25, x: 1, y: 4,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0F, x: 5, y: 5,
        movement: NpcMovement(0), facing: NpcFacing(1),
        range: 0, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CINNABARMART: [NpcEntry; 3] = [
    NpcEntry {
        sprite_id: 0x26, x: 0, y: 5,
        movement: NpcMovement(0), facing: NpcFacing(3),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x1B, x: 6, y: 2,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x20, x: 3, y: 4,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_CINNABARPOKECENTER: [NpcEntry; 4] = [
    NpcEntry {
        sprite_id: 0x29, x: 3, y: 1,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x06, x: 9, y: 4,
        movement: NpcMovement(1), facing: NpcFacing(0),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x10, x: 2, y: 6,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x2A, x: 11, y: 2,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 4,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_COLOSSEUM: [NpcEntry; 1] = [
    NpcEntry {
        sprite_id: 0x01, x: 2, y: 2,
        movement: NpcMovement(1), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_COPYCATSHOUSE1F: [NpcEntry; 3] = [
    NpcEntry {
        sprite_id: 0x1C, x: 2, y: 2,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0A, x: 5, y: 4,
        movement: NpcMovement(0), facing: NpcFacing(2),
        range: 0, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x38, x: 1, y: 4,
        movement: NpcMovement(1), facing: NpcFacing(0),
        range: 1, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_COPYCATSHOUSE2F: [NpcEntry; 5] = [
    NpcEntry {
        sprite_id: 0x1D, x: 4, y: 3,
        movement: NpcMovement(1), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x09, x: 4, y: 6,
        movement: NpcMovement(1), facing: NpcFacing(2),
        range: 2, text_id: 2,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x05, x: 5, y: 1,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 3,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x09, x: 2, y: 0,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 4,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x38, x: 1, y: 6,
        movement: NpcMovement(0), facing: NpcFacing(3),
        range: 0, text_id: 5,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_DAYCARE: [NpcEntry; 1] = [
    NpcEntry {
        sprite_id: 0x10, x: 2, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(3),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_DIGLETTSCAVE: [NpcEntry; 0] = [];

pub static NPCS_DIGLETTSCAVEROUTE11: [NpcEntry; 1] = [
    NpcEntry {
        sprite_id: 0x0B, x: 2, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_DIGLETTSCAVEROUTE2: [NpcEntry; 1] = [
    NpcEntry {
        sprite_id: 0x27, x: 3, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

pub static NPCS_FIGHTINGDOJO: [NpcEntry; 7] = [
    NpcEntry {
        sprite_id: 0x0E, x: 5, y: 3,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 1,
        is_trainer: true,
        trainer_class: 24, trainer_set: 1,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0E, x: 3, y: 4,
        movement: NpcMovement(0), facing: NpcFacing(3),
        range: 0, text_id: 2,
        is_trainer: true,
        trainer_class: 24, trainer_set: 2,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0E, x: 3, y: 6,
        movement: NpcMovement(0), facing: NpcFacing(3),
        range: 0, text_id: 3,
        is_trainer: true,
        trainer_class: 24, trainer_set: 3,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0E, x: 5, y: 5,
        movement: NpcMovement(0), facing: NpcFacing(2),
        range: 0, text_id: 4,
        is_trainer: true,
        trainer_class: 24, trainer_set: 4,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x0E, x: 5, y: 7,
        movement: NpcMovement(0), facing: NpcFacing(2),
        range: 0, text_id: 5,
        is_trainer: true,
        trainer_class: 24, trainer_set: 5,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3D, x: 4, y: 1,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 6,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
    NpcEntry {
        sprite_id: 0x3D, x: 5, y: 1,
        movement: NpcMovement(0), facing: NpcFacing(0),
        range: 0, text_id: 7,
        is_trainer: false,
        trainer_class: 0, trainer_set: 0,
        item_id: 0x00,
    },
];

