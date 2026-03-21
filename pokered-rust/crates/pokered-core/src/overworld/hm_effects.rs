use pokered_data::maps::MapId;
use pokered_data::tileset_data::{
    cut_tree_replacement, is_outside_tileset, is_water_tileset, CUT_GRASS_TILE, CUT_TREE_TILE_GYM,
    CUT_TREE_TILE_OVERWORLD,
};
use pokered_data::tilesets::TilesetId;

use super::Direction;

// ── Badge Bit Positions ──────────────────────────────────────────────
// From constants/ram_constants.asm:
//   BIT_BOULDERBADGE = 0, BIT_CASCADEBADGE = 1, BIT_THUNDERBADGE = 2,
//   BIT_RAINBOWBADGE = 3, BIT_SOULBADGE = 4, ...

pub const BIT_BOULDERBADGE: u8 = 0;
pub const BIT_CASCADEBADGE: u8 = 1;
pub const BIT_THUNDERBADGE: u8 = 2;
pub const BIT_RAINBOWBADGE: u8 = 3;
pub const BIT_SOULBADGE: u8 = 4;

fn has_badge(obtained_badges: u8, bit: u8) -> bool {
    obtained_badges & (1 << bit) != 0
}

// ── HM Results ───────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CutResult {
    NoBadge,
    NothingToCut,
    CutTree { replacement_block: u8 },
    CutGrass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlyResult {
    NoBadge,
    CannotFlyHere,
    ChoseDestination { destination: MapId },
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SurfResult {
    NoBadge,
    AlreadySurfing,
    NotFacingWater,
    ForcedToRideBike,
    CurrentTooFast,
    StartedSurfing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StrengthResult {
    NoBadge,
    Activated,
    AlreadyActive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlashResult {
    NoBadge,
    AlreadyLit,
    LitUpCave,
}

// ── Fly Destinations ─────────────────────────────────────────────────
// From data/maps/special_warps.asm FlyWarpDataPtr

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlyDestination {
    pub map: MapId,
    pub x: u8,
    pub y: u8,
}

pub const FLY_DESTINATIONS: &[FlyDestination] = &[
    FlyDestination {
        map: MapId::PalletTown,
        x: 5,
        y: 6,
    },
    FlyDestination {
        map: MapId::ViridianCity,
        x: 23,
        y: 26,
    },
    FlyDestination {
        map: MapId::PewterCity,
        x: 13,
        y: 26,
    },
    FlyDestination {
        map: MapId::CeruleanCity,
        x: 19,
        y: 18,
    },
    FlyDestination {
        map: MapId::LavenderTown,
        x: 3,
        y: 6,
    },
    FlyDestination {
        map: MapId::VermilionCity,
        x: 11,
        y: 4,
    },
    FlyDestination {
        map: MapId::CeladonCity,
        x: 41,
        y: 10,
    },
    FlyDestination {
        map: MapId::FuchsiaCity,
        x: 19,
        y: 28,
    },
    FlyDestination {
        map: MapId::CinnabarIsland,
        x: 11,
        y: 12,
    },
    FlyDestination {
        map: MapId::IndigoPlateau,
        x: 9,
        y: 6,
    },
    FlyDestination {
        map: MapId::SaffronCity,
        x: 9,
        y: 30,
    },
    FlyDestination {
        map: MapId::Route4,
        x: 11,
        y: 6,
    },
    FlyDestination {
        map: MapId::Route10,
        x: 11,
        y: 20,
    },
];

pub const NUM_FLY_DESTINATIONS: usize = 13;

pub fn fly_destination_for_map(map: MapId) -> Option<&'static FlyDestination> {
    FLY_DESTINATIONS.iter().find(|d| d.map == map)
}

// ── Boulder Pushing ──────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoulderPushResult {
    StrengthNotActive,
    NoBoulderInFront,
    NotABoulder,
    BoulderBlocked,
    NotPushingCorrectDirection,
    NeedPushAgain,
    Pushed { direction: Direction },
}

// ── Seafoam Islands B4F Special Check ────────────────────────────────
// Surf is blocked on B4F until both boulders are in holes

pub const SEAFOAM_B4F_STAIRS_X: u8 = 7;
pub const SEAFOAM_B4F_STAIRS_Y: u8 = 11;

// ── HM Effect Functions ──────────────────────────────────────────────

pub fn use_cut(
    obtained_badges: u8,
    tileset: TilesetId,
    tile_in_front: u8,
    current_block: u8,
) -> CutResult {
    if !has_badge(obtained_badges, BIT_CASCADEBADGE) {
        return CutResult::NoBadge;
    }

    match tileset {
        TilesetId::Overworld => {
            if tile_in_front == CUT_TREE_TILE_OVERWORLD {
                if let Some(replacement) = cut_tree_replacement(current_block) {
                    return CutResult::CutTree {
                        replacement_block: replacement,
                    };
                }
            }
            if tile_in_front == CUT_GRASS_TILE {
                return CutResult::CutGrass;
            }
            CutResult::NothingToCut
        }
        TilesetId::Gym => {
            if tile_in_front == CUT_TREE_TILE_GYM {
                if let Some(replacement) = cut_tree_replacement(current_block) {
                    return CutResult::CutTree {
                        replacement_block: replacement,
                    };
                }
            }
            CutResult::NothingToCut
        }
        _ => CutResult::NothingToCut,
    }
}

pub fn use_fly(
    obtained_badges: u8,
    tileset: TilesetId,
    chosen_destination: Option<MapId>,
) -> FlyResult {
    if !has_badge(obtained_badges, BIT_THUNDERBADGE) {
        return FlyResult::NoBadge;
    }

    if !is_outside_tileset(tileset) {
        return FlyResult::CannotFlyHere;
    }

    match chosen_destination {
        Some(dest) => FlyResult::ChoseDestination { destination: dest },
        None => FlyResult::Cancelled,
    }
}

pub fn use_surf(
    obtained_badges: u8,
    tileset: TilesetId,
    is_facing_water: bool,
    already_surfing: bool,
    forced_bike: bool,
    current_map: MapId,
    seafoam_b4f_boulders_done: bool,
    player_x: u8,
    player_y: u8,
) -> SurfResult {
    if !has_badge(obtained_badges, BIT_SOULBADGE) {
        return SurfResult::NoBadge;
    }

    if already_surfing {
        return SurfResult::AlreadySurfing;
    }

    if forced_bike {
        return SurfResult::ForcedToRideBike;
    }

    if current_map == MapId::SeafoamIslandsB4F
        && !seafoam_b4f_boulders_done
        && player_x == SEAFOAM_B4F_STAIRS_X
        && player_y == SEAFOAM_B4F_STAIRS_Y
    {
        return SurfResult::CurrentTooFast;
    }

    if !is_facing_water {
        return SurfResult::NotFacingWater;
    }

    let _ = is_water_tileset(tileset);

    SurfResult::StartedSurfing
}

pub fn use_strength(obtained_badges: u8, strength_already_active: bool) -> StrengthResult {
    if !has_badge(obtained_badges, BIT_RAINBOWBADGE) {
        return StrengthResult::NoBadge;
    }

    if strength_already_active {
        return StrengthResult::AlreadyActive;
    }

    StrengthResult::Activated
}

pub fn use_flash(obtained_badges: u8, cave_is_dark: bool) -> FlashResult {
    if !has_badge(obtained_badges, BIT_BOULDERBADGE) {
        return FlashResult::NoBadge;
    }

    if !cave_is_dark {
        return FlashResult::AlreadyLit;
    }

    FlashResult::LitUpCave
}

pub fn try_push_boulder(
    strength_active: bool,
    boulder_dust_active: bool,
    sprite_in_front: Option<u8>,
    is_boulder: bool,
    already_tried_push: bool,
    pushing_correct_direction: bool,
    boulder_blocked: bool,
) -> BoulderPushResult {
    if !strength_active {
        return BoulderPushResult::StrengthNotActive;
    }

    if boulder_dust_active {
        return BoulderPushResult::StrengthNotActive;
    }

    if sprite_in_front.is_none() {
        return BoulderPushResult::NoBoulderInFront;
    }

    if !is_boulder {
        return BoulderPushResult::NotABoulder;
    }

    if !already_tried_push {
        return BoulderPushResult::NeedPushAgain;
    }

    if !pushing_correct_direction {
        return BoulderPushResult::NotPushingCorrectDirection;
    }

    if boulder_blocked {
        return BoulderPushResult::BoulderBlocked;
    }

    BoulderPushResult::Pushed {
        direction: Direction::Down,
    }
}

pub fn try_push_boulder_with_direction(
    strength_active: bool,
    boulder_dust_active: bool,
    sprite_in_front: Option<u8>,
    is_boulder: bool,
    already_tried_push: bool,
    player_facing: Direction,
    held_direction: Option<Direction>,
    boulder_blocked: bool,
) -> BoulderPushResult {
    if !strength_active {
        return BoulderPushResult::StrengthNotActive;
    }

    if boulder_dust_active {
        return BoulderPushResult::StrengthNotActive;
    }

    if sprite_in_front.is_none() {
        return BoulderPushResult::NoBoulderInFront;
    }

    if !is_boulder {
        return BoulderPushResult::NotABoulder;
    }

    if !already_tried_push {
        return BoulderPushResult::NeedPushAgain;
    }

    match held_direction {
        Some(dir) if dir == player_facing => {}
        _ => return BoulderPushResult::NotPushingCorrectDirection,
    }

    if boulder_blocked {
        return BoulderPushResult::BoulderBlocked;
    }

    BoulderPushResult::Pushed {
        direction: player_facing,
    }
}
