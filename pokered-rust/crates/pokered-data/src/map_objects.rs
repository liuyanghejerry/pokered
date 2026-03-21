//! Map warp data — warp points for all 248 maps.
//!
//! Ported from data/maps/objects/*.asm warp_event macros.
//! Each map can have zero or more warp points (doors, stairs, etc.).
//!
//! `dest_map: None` means LAST_MAP (return to previous map) or
//! dynamically-assigned destination (e.g., elevator warps).

use crate::maps::MapId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WarpData {
    pub x: u8,
    pub y: u8,
    pub dest_map: Option<MapId>,
    pub dest_warp_id: u8,
}

pub struct MapWarpEntry {
    pub warps: &'static [WarpData],
}

static WARPS_AGATHASROOM: [WarpData; 4] = [
    WarpData {
        x: 4,
        y: 11,
        dest_map: Some(MapId::BrunosRoom),
        dest_warp_id: 2,
    },
    WarpData {
        x: 5,
        y: 11,
        dest_map: Some(MapId::BrunosRoom),
        dest_warp_id: 3,
    },
    WarpData {
        x: 4,
        y: 0,
        dest_map: Some(MapId::LancesRoom),
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 0,
        dest_map: Some(MapId::LancesRoom),
        dest_warp_id: 0,
    },
];

static WARPS_BIKESHOP: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
];

static WARPS_BILLSHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
];

static WARPS_BLUESHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 1,
    },
];

static WARPS_BRUNOSROOM: [WarpData; 4] = [
    WarpData {
        x: 4,
        y: 11,
        dest_map: Some(MapId::LoreleisRoom),
        dest_warp_id: 2,
    },
    WarpData {
        x: 5,
        y: 11,
        dest_map: Some(MapId::LoreleisRoom),
        dest_warp_id: 3,
    },
    WarpData {
        x: 4,
        y: 0,
        dest_map: Some(MapId::AgathasRoom),
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 0,
        dest_map: Some(MapId::AgathasRoom),
        dest_warp_id: 1,
    },
];

static WARPS_CELADONCHIEFHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 11,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 11,
    },
];

static WARPS_CELADONCITY: [WarpData; 13] = [
    WarpData {
        x: 8,
        y: 13,
        dest_map: Some(MapId::CeladonMart1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 10,
        y: 13,
        dest_map: Some(MapId::CeladonMart1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 24,
        y: 9,
        dest_map: Some(MapId::CeladonMansion1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 24,
        y: 3,
        dest_map: Some(MapId::CeladonMansion1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 25,
        y: 3,
        dest_map: Some(MapId::CeladonMansion1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 41,
        y: 9,
        dest_map: Some(MapId::CeladonPokecenter),
        dest_warp_id: 0,
    },
    WarpData {
        x: 12,
        y: 27,
        dest_map: Some(MapId::CeladonGym),
        dest_warp_id: 0,
    },
    WarpData {
        x: 28,
        y: 19,
        dest_map: Some(MapId::GameCorner),
        dest_warp_id: 0,
    },
    WarpData {
        x: 39,
        y: 19,
        dest_map: Some(MapId::CeladonMart5F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 33,
        y: 19,
        dest_map: Some(MapId::GameCornerPrizeRoom),
        dest_warp_id: 0,
    },
    WarpData {
        x: 31,
        y: 27,
        dest_map: Some(MapId::CeladonDiner),
        dest_warp_id: 0,
    },
    WarpData {
        x: 35,
        y: 27,
        dest_map: Some(MapId::CeladonChiefHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 43,
        y: 27,
        dest_map: Some(MapId::CeladonHotel),
        dest_warp_id: 0,
    },
];

static WARPS_CELADONDINER: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 10,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 10,
    },
];

static WARPS_CELADONGYM: [WarpData; 2] = [
    WarpData {
        x: 4,
        y: 17,
        dest_map: None,
        dest_warp_id: 6,
    },
    WarpData {
        x: 5,
        y: 17,
        dest_map: None,
        dest_warp_id: 6,
    },
];

static WARPS_CELADONHOTEL: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 12,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 12,
    },
];

static WARPS_CELADONMANSION1F: [WarpData; 5] = [
    WarpData {
        x: 4,
        y: 11,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 5,
        y: 11,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 4,
        y: 0,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 7,
        y: 1,
        dest_map: Some(MapId::CeladonMansion2F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 2,
        y: 1,
        dest_map: Some(MapId::CeladonMansion2F),
        dest_warp_id: 2,
    },
];

static WARPS_CELADONMANSION2F: [WarpData; 4] = [
    WarpData {
        x: 6,
        y: 1,
        dest_map: Some(MapId::CeladonMansion3F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 7,
        y: 1,
        dest_map: Some(MapId::CeladonMansion1F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 2,
        y: 1,
        dest_map: Some(MapId::CeladonMansion1F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 4,
        y: 1,
        dest_map: Some(MapId::CeladonMansion3F),
        dest_warp_id: 3,
    },
];

static WARPS_CELADONMANSION3F: [WarpData; 4] = [
    WarpData {
        x: 6,
        y: 1,
        dest_map: Some(MapId::CeladonMansion2F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 7,
        y: 1,
        dest_map: Some(MapId::CeladonMansionRoof),
        dest_warp_id: 0,
    },
    WarpData {
        x: 2,
        y: 1,
        dest_map: Some(MapId::CeladonMansionRoof),
        dest_warp_id: 1,
    },
    WarpData {
        x: 4,
        y: 1,
        dest_map: Some(MapId::CeladonMansion2F),
        dest_warp_id: 3,
    },
];

static WARPS_CELADONMANSIONROOF: [WarpData; 3] = [
    WarpData {
        x: 6,
        y: 1,
        dest_map: Some(MapId::CeladonMansion3F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 2,
        y: 1,
        dest_map: Some(MapId::CeladonMansion3F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 2,
        y: 7,
        dest_map: Some(MapId::CeladonMansionRoofHouse),
        dest_warp_id: 0,
    },
];

static WARPS_CELADONMANSIONROOFHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: Some(MapId::CeladonMansionRoof),
        dest_warp_id: 2,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: Some(MapId::CeladonMansionRoof),
        dest_warp_id: 2,
    },
];

static WARPS_CELADONMART1F: [WarpData; 6] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 16,
        y: 7,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 17,
        y: 7,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 12,
        y: 1,
        dest_map: Some(MapId::CeladonMart2F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 1,
        y: 1,
        dest_map: Some(MapId::CeladonMartElevator),
        dest_warp_id: 0,
    },
];

static WARPS_CELADONMART2F: [WarpData; 3] = [
    WarpData {
        x: 12,
        y: 1,
        dest_map: Some(MapId::CeladonMart1F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 16,
        y: 1,
        dest_map: Some(MapId::CeladonMart3F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 1,
        y: 1,
        dest_map: Some(MapId::CeladonMartElevator),
        dest_warp_id: 0,
    },
];

static WARPS_CELADONMART3F: [WarpData; 3] = [
    WarpData {
        x: 12,
        y: 1,
        dest_map: Some(MapId::CeladonMart4F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 16,
        y: 1,
        dest_map: Some(MapId::CeladonMart2F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 1,
        y: 1,
        dest_map: Some(MapId::CeladonMartElevator),
        dest_warp_id: 0,
    },
];

static WARPS_CELADONMART4F: [WarpData; 3] = [
    WarpData {
        x: 12,
        y: 1,
        dest_map: Some(MapId::CeladonMart3F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 16,
        y: 1,
        dest_map: Some(MapId::CeladonMart5F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 1,
        y: 1,
        dest_map: Some(MapId::CeladonMartElevator),
        dest_warp_id: 0,
    },
];

static WARPS_CELADONMART5F: [WarpData; 3] = [
    WarpData {
        x: 12,
        y: 1,
        dest_map: Some(MapId::CeladonMartRoof),
        dest_warp_id: 0,
    },
    WarpData {
        x: 16,
        y: 1,
        dest_map: Some(MapId::CeladonMart4F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 1,
        y: 1,
        dest_map: Some(MapId::CeladonMartElevator),
        dest_warp_id: 0,
    },
];

static WARPS_CELADONMARTELEVATOR: [WarpData; 2] = [
    WarpData {
        x: 1,
        y: 3,
        dest_map: Some(MapId::CeladonMart1F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 2,
        y: 3,
        dest_map: Some(MapId::CeladonMart1F),
        dest_warp_id: 5,
    },
];

static WARPS_CELADONMARTROOF: [WarpData; 1] = [WarpData {
    x: 15,
    y: 2,
    dest_map: Some(MapId::CeladonMart5F),
    dest_warp_id: 0,
}];

static WARPS_CELADONPOKECENTER: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 5,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 5,
    },
];

static WARPS_CERULEANBADGEHOUSE: [WarpData; 3] = [
    WarpData {
        x: 2,
        y: 0,
        dest_map: None,
        dest_warp_id: 9,
    },
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 8,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 8,
    },
];

static WARPS_CERULEANCAVE1F: [WarpData; 9] = [
    WarpData {
        x: 24,
        y: 17,
        dest_map: None,
        dest_warp_id: 6,
    },
    WarpData {
        x: 25,
        y: 17,
        dest_map: None,
        dest_warp_id: 6,
    },
    WarpData {
        x: 27,
        y: 1,
        dest_map: Some(MapId::CeruleanCave2F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 23,
        y: 7,
        dest_map: Some(MapId::CeruleanCave2F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 18,
        y: 9,
        dest_map: Some(MapId::CeruleanCave2F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 7,
        y: 1,
        dest_map: Some(MapId::CeruleanCave2F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 1,
        y: 3,
        dest_map: Some(MapId::CeruleanCave2F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 3,
        y: 11,
        dest_map: Some(MapId::CeruleanCave2F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 0,
        y: 6,
        dest_map: Some(MapId::CeruleanCaveB1F),
        dest_warp_id: 0,
    },
];

static WARPS_CERULEANCAVE2F: [WarpData; 6] = [
    WarpData {
        x: 29,
        y: 1,
        dest_map: Some(MapId::CeruleanCave1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 22,
        y: 6,
        dest_map: Some(MapId::CeruleanCave1F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 19,
        y: 7,
        dest_map: Some(MapId::CeruleanCave1F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 9,
        y: 1,
        dest_map: Some(MapId::CeruleanCave1F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 1,
        y: 3,
        dest_map: Some(MapId::CeruleanCave1F),
        dest_warp_id: 6,
    },
    WarpData {
        x: 3,
        y: 11,
        dest_map: Some(MapId::CeruleanCave1F),
        dest_warp_id: 7,
    },
];

static WARPS_CERULEANCAVEB1F: [WarpData; 1] = [WarpData {
    x: 3,
    y: 6,
    dest_map: Some(MapId::CeruleanCave1F),
    dest_warp_id: 8,
}];

static WARPS_CERULEANCITY: [WarpData; 10] = [
    WarpData {
        x: 27,
        y: 11,
        dest_map: Some(MapId::CeruleanTrashedHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 13,
        y: 15,
        dest_map: Some(MapId::CeruleanTradeHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 19,
        y: 17,
        dest_map: Some(MapId::CeruleanPokecenter),
        dest_warp_id: 0,
    },
    WarpData {
        x: 30,
        y: 19,
        dest_map: Some(MapId::CeruleanGym),
        dest_warp_id: 0,
    },
    WarpData {
        x: 13,
        y: 25,
        dest_map: Some(MapId::BikeShop),
        dest_warp_id: 0,
    },
    WarpData {
        x: 25,
        y: 25,
        dest_map: Some(MapId::CeruleanMart),
        dest_warp_id: 0,
    },
    WarpData {
        x: 4,
        y: 11,
        dest_map: Some(MapId::CeruleanCave1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 27,
        y: 9,
        dest_map: Some(MapId::CeruleanTrashedHouse),
        dest_warp_id: 2,
    },
    WarpData {
        x: 9,
        y: 11,
        dest_map: Some(MapId::CeruleanBadgeHouse),
        dest_warp_id: 1,
    },
    WarpData {
        x: 9,
        y: 9,
        dest_map: Some(MapId::CeruleanBadgeHouse),
        dest_warp_id: 0,
    },
];

static WARPS_CERULEANGYM: [WarpData; 2] = [
    WarpData {
        x: 4,
        y: 13,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 5,
        y: 13,
        dest_map: None,
        dest_warp_id: 3,
    },
];

static WARPS_CERULEANMART: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 5,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 5,
    },
];

static WARPS_CERULEANPOKECENTER: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 2,
    },
];

static WARPS_CERULEANTRADEHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 1,
    },
];

static WARPS_CERULEANTRASHEDHOUSE: [WarpData; 3] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 3,
        y: 0,
        dest_map: None,
        dest_warp_id: 7,
    },
];

static WARPS_CHAMPIONSROOM: [WarpData; 4] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: Some(MapId::LancesRoom),
        dest_warp_id: 1,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: Some(MapId::LancesRoom),
        dest_warp_id: 2,
    },
    WarpData {
        x: 3,
        y: 0,
        dest_map: Some(MapId::HallOfFame),
        dest_warp_id: 0,
    },
    WarpData {
        x: 4,
        y: 0,
        dest_map: Some(MapId::HallOfFame),
        dest_warp_id: 0,
    },
];

static WARPS_CINNABARGYM: [WarpData; 2] = [
    WarpData {
        x: 16,
        y: 17,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 17,
        y: 17,
        dest_map: None,
        dest_warp_id: 1,
    },
];

static WARPS_CINNABARISLAND: [WarpData; 5] = [
    WarpData {
        x: 6,
        y: 3,
        dest_map: Some(MapId::PokemonMansion1F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 18,
        y: 3,
        dest_map: Some(MapId::CinnabarGym),
        dest_warp_id: 0,
    },
    WarpData {
        x: 6,
        y: 9,
        dest_map: Some(MapId::CinnabarLab),
        dest_warp_id: 0,
    },
    WarpData {
        x: 11,
        y: 11,
        dest_map: Some(MapId::CinnabarPokecenter),
        dest_warp_id: 0,
    },
    WarpData {
        x: 15,
        y: 11,
        dest_map: Some(MapId::CinnabarMart),
        dest_warp_id: 0,
    },
];

static WARPS_CINNABARLAB: [WarpData; 5] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 8,
        y: 4,
        dest_map: Some(MapId::CinnabarLabTradeRoom),
        dest_warp_id: 0,
    },
    WarpData {
        x: 12,
        y: 4,
        dest_map: Some(MapId::CinnabarLabMetronomeRoom),
        dest_warp_id: 0,
    },
    WarpData {
        x: 16,
        y: 4,
        dest_map: Some(MapId::CinnabarLabFossilRoom),
        dest_warp_id: 0,
    },
];

static WARPS_CINNABARLABFOSSILROOM: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: Some(MapId::CinnabarLab),
        dest_warp_id: 4,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: Some(MapId::CinnabarLab),
        dest_warp_id: 4,
    },
];

static WARPS_CINNABARLABMETRONOMEROOM: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: Some(MapId::CinnabarLab),
        dest_warp_id: 3,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: Some(MapId::CinnabarLab),
        dest_warp_id: 3,
    },
];

static WARPS_CINNABARLABTRADEROOM: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: Some(MapId::CinnabarLab),
        dest_warp_id: 2,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: Some(MapId::CinnabarLab),
        dest_warp_id: 2,
    },
];

static WARPS_CINNABARMART: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
];

static WARPS_CINNABARPOKECENTER: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 3,
    },
];

static WARPS_COPYCATSHOUSE1F: [WarpData; 3] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 7,
        y: 1,
        dest_map: Some(MapId::CopycatsHouse2F),
        dest_warp_id: 0,
    },
];

static WARPS_COPYCATSHOUSE2F: [WarpData; 1] = [WarpData {
    x: 7,
    y: 1,
    dest_map: Some(MapId::CopycatsHouse1F),
    dest_warp_id: 2,
}];

static WARPS_DAYCARE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
];

static WARPS_DIGLETTSCAVE: [WarpData; 2] = [
    WarpData {
        x: 5,
        y: 5,
        dest_map: Some(MapId::DiglettsCaveRoute2),
        dest_warp_id: 2,
    },
    WarpData {
        x: 37,
        y: 31,
        dest_map: Some(MapId::DiglettsCaveRoute11),
        dest_warp_id: 2,
    },
];

static WARPS_DIGLETTSCAVEROUTE11: [WarpData; 3] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 4,
        y: 4,
        dest_map: Some(MapId::DiglettsCave),
        dest_warp_id: 1,
    },
];

static WARPS_DIGLETTSCAVEROUTE2: [WarpData; 3] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 4,
        y: 4,
        dest_map: Some(MapId::DiglettsCave),
        dest_warp_id: 0,
    },
];

static WARPS_FIGHTINGDOJO: [WarpData; 2] = [
    WarpData {
        x: 4,
        y: 11,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 5,
        y: 11,
        dest_map: None,
        dest_warp_id: 1,
    },
];

static WARPS_FUCHSIABILLSGRANDPASHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 1,
    },
];

static WARPS_FUCHSIACITY: [WarpData; 9] = [
    WarpData {
        x: 5,
        y: 13,
        dest_map: Some(MapId::FuchsiaMart),
        dest_warp_id: 0,
    },
    WarpData {
        x: 11,
        y: 27,
        dest_map: Some(MapId::FuchsiaBillsGrandpasHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 19,
        y: 27,
        dest_map: Some(MapId::FuchsiaPokecenter),
        dest_warp_id: 0,
    },
    WarpData {
        x: 27,
        y: 27,
        dest_map: Some(MapId::WardensHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 18,
        y: 3,
        dest_map: Some(MapId::SafariZoneGate),
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 27,
        dest_map: Some(MapId::FuchsiaGym),
        dest_warp_id: 0,
    },
    WarpData {
        x: 22,
        y: 13,
        dest_map: Some(MapId::FuchsiaMeetingRoom),
        dest_warp_id: 0,
    },
    WarpData {
        x: 31,
        y: 27,
        dest_map: Some(MapId::FuchsiaGoodRodHouse),
        dest_warp_id: 1,
    },
    WarpData {
        x: 31,
        y: 24,
        dest_map: Some(MapId::FuchsiaGoodRodHouse),
        dest_warp_id: 0,
    },
];

static WARPS_FUCHSIAGOODRODHOUSE: [WarpData; 3] = [
    WarpData {
        x: 2,
        y: 0,
        dest_map: None,
        dest_warp_id: 8,
    },
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 7,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 7,
    },
];

static WARPS_FUCHSIAGYM: [WarpData; 2] = [
    WarpData {
        x: 4,
        y: 17,
        dest_map: None,
        dest_warp_id: 5,
    },
    WarpData {
        x: 5,
        y: 17,
        dest_map: None,
        dest_warp_id: 5,
    },
];

static WARPS_FUCHSIAMART: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
];

static WARPS_FUCHSIAMEETINGROOM: [WarpData; 2] = [
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 6,
    },
    WarpData {
        x: 5,
        y: 7,
        dest_map: None,
        dest_warp_id: 6,
    },
];

static WARPS_FUCHSIAPOKECENTER: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 2,
    },
];

static WARPS_GAMECORNER: [WarpData; 3] = [
    WarpData {
        x: 15,
        y: 17,
        dest_map: None,
        dest_warp_id: 7,
    },
    WarpData {
        x: 16,
        y: 17,
        dest_map: None,
        dest_warp_id: 7,
    },
    WarpData {
        x: 17,
        y: 4,
        dest_map: Some(MapId::RocketHideoutB1F),
        dest_warp_id: 1,
    },
];

static WARPS_GAMECORNERPRIZEROOM: [WarpData; 2] = [
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 9,
    },
    WarpData {
        x: 5,
        y: 7,
        dest_map: None,
        dest_warp_id: 9,
    },
];

static WARPS_HALLOFFAME: [WarpData; 2] = [
    WarpData {
        x: 4,
        y: 7,
        dest_map: Some(MapId::ChampionsRoom),
        dest_warp_id: 2,
    },
    WarpData {
        x: 5,
        y: 7,
        dest_map: Some(MapId::ChampionsRoom),
        dest_warp_id: 3,
    },
];

static WARPS_INDIGOPLATEAU: [WarpData; 2] = [
    WarpData {
        x: 9,
        y: 5,
        dest_map: Some(MapId::IndigoPlateauLobby),
        dest_warp_id: 0,
    },
    WarpData {
        x: 10,
        y: 5,
        dest_map: Some(MapId::IndigoPlateauLobby),
        dest_warp_id: 0,
    },
];

static WARPS_INDIGOPLATEAULOBBY: [WarpData; 3] = [
    WarpData {
        x: 7,
        y: 11,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 8,
        y: 11,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 8,
        y: 0,
        dest_map: Some(MapId::LoreleisRoom),
        dest_warp_id: 0,
    },
];

static WARPS_LANCESROOM: [WarpData; 3] = [
    WarpData {
        x: 24,
        y: 16,
        dest_map: Some(MapId::AgathasRoom),
        dest_warp_id: 2,
    },
    WarpData {
        x: 5,
        y: 0,
        dest_map: Some(MapId::ChampionsRoom),
        dest_warp_id: 0,
    },
    WarpData {
        x: 6,
        y: 0,
        dest_map: Some(MapId::ChampionsRoom),
        dest_warp_id: 0,
    },
];

static WARPS_LAVENDERCUBONEHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
];

static WARPS_LAVENDERMART: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 3,
    },
];

static WARPS_LAVENDERPOKECENTER: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
];

static WARPS_LAVENDERTOWN: [WarpData; 6] = [
    WarpData {
        x: 3,
        y: 5,
        dest_map: Some(MapId::LavenderPokecenter),
        dest_warp_id: 0,
    },
    WarpData {
        x: 14,
        y: 5,
        dest_map: Some(MapId::PokemonTower1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 7,
        y: 9,
        dest_map: Some(MapId::MrFujisHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 15,
        y: 13,
        dest_map: Some(MapId::LavenderMart),
        dest_warp_id: 0,
    },
    WarpData {
        x: 3,
        y: 13,
        dest_map: Some(MapId::LavenderCuboneHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 7,
        y: 13,
        dest_map: Some(MapId::NameRatersHouse),
        dest_warp_id: 0,
    },
];

static WARPS_LORELEISROOM: [WarpData; 4] = [
    WarpData {
        x: 4,
        y: 11,
        dest_map: Some(MapId::IndigoPlateauLobby),
        dest_warp_id: 2,
    },
    WarpData {
        x: 5,
        y: 11,
        dest_map: Some(MapId::IndigoPlateauLobby),
        dest_warp_id: 2,
    },
    WarpData {
        x: 4,
        y: 0,
        dest_map: Some(MapId::BrunosRoom),
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 0,
        dest_map: Some(MapId::BrunosRoom),
        dest_warp_id: 1,
    },
];

static WARPS_MRFUJISHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 2,
    },
];

static WARPS_MRPSYCHICSHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 7,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 7,
    },
];

static WARPS_MTMOON1F: [WarpData; 5] = [
    WarpData {
        x: 14,
        y: 35,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 15,
        y: 35,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 5,
        y: 5,
        dest_map: Some(MapId::MtMoonB1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 17,
        y: 11,
        dest_map: Some(MapId::MtMoonB1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 25,
        y: 15,
        dest_map: Some(MapId::MtMoonB1F),
        dest_warp_id: 3,
    },
];

static WARPS_MTMOONB1F: [WarpData; 8] = [
    WarpData {
        x: 5,
        y: 5,
        dest_map: Some(MapId::MtMoon1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 17,
        y: 11,
        dest_map: Some(MapId::MtMoonB2F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 25,
        y: 9,
        dest_map: Some(MapId::MtMoon1F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 25,
        y: 15,
        dest_map: Some(MapId::MtMoon1F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 21,
        y: 17,
        dest_map: Some(MapId::MtMoonB2F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 13,
        y: 27,
        dest_map: Some(MapId::MtMoonB2F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 23,
        y: 3,
        dest_map: Some(MapId::MtMoonB2F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 27,
        y: 3,
        dest_map: None,
        dest_warp_id: 2,
    },
];

static WARPS_MTMOONB2F: [WarpData; 4] = [
    WarpData {
        x: 25,
        y: 9,
        dest_map: Some(MapId::MtMoonB1F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 21,
        y: 17,
        dest_map: Some(MapId::MtMoonB1F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 15,
        y: 27,
        dest_map: Some(MapId::MtMoonB1F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 5,
        y: 7,
        dest_map: Some(MapId::MtMoonB1F),
        dest_warp_id: 6,
    },
];

static WARPS_MTMOONPOKECENTER: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
];

static WARPS_MUSEUM1F: [WarpData; 5] = [
    WarpData {
        x: 10,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 11,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 16,
        y: 7,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 17,
        y: 7,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 7,
        y: 7,
        dest_map: Some(MapId::Museum2F),
        dest_warp_id: 0,
    },
];

static WARPS_MUSEUM2F: [WarpData; 1] = [WarpData {
    x: 7,
    y: 7,
    dest_map: Some(MapId::Museum1F),
    dest_warp_id: 4,
}];

static WARPS_NAMERATERSHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 5,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 5,
    },
];

static WARPS_OAKSLAB: [WarpData; 2] = [
    WarpData {
        x: 4,
        y: 11,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 5,
        y: 11,
        dest_map: None,
        dest_warp_id: 2,
    },
];

static WARPS_PALLETTOWN: [WarpData; 3] = [
    WarpData {
        x: 5,
        y: 5,
        dest_map: Some(MapId::RedsHouse1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 13,
        y: 5,
        dest_map: Some(MapId::BluesHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 12,
        y: 11,
        dest_map: Some(MapId::OaksLab),
        dest_warp_id: 1,
    },
];

static WARPS_PEWTERCITY: [WarpData; 7] = [
    WarpData {
        x: 14,
        y: 7,
        dest_map: Some(MapId::Museum1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 19,
        y: 5,
        dest_map: Some(MapId::Museum1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 16,
        y: 17,
        dest_map: Some(MapId::PewterGym),
        dest_warp_id: 0,
    },
    WarpData {
        x: 29,
        y: 13,
        dest_map: Some(MapId::PewterNidoranHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 23,
        y: 17,
        dest_map: Some(MapId::PewterMart),
        dest_warp_id: 0,
    },
    WarpData {
        x: 7,
        y: 29,
        dest_map: Some(MapId::PewterSpeechHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 13,
        y: 25,
        dest_map: Some(MapId::PewterPokecenter),
        dest_warp_id: 0,
    },
];

static WARPS_PEWTERGYM: [WarpData; 2] = [
    WarpData {
        x: 4,
        y: 13,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 5,
        y: 13,
        dest_map: None,
        dest_warp_id: 2,
    },
];

static WARPS_PEWTERMART: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
];

static WARPS_PEWTERNIDORANHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 3,
    },
];

static WARPS_PEWTERPOKECENTER: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 6,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 6,
    },
];

static WARPS_PEWTERSPEECHHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 5,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 5,
    },
];

static WARPS_POKEMONFANCLUB: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 1,
    },
];

static WARPS_POKEMONMANSION1F: [WarpData; 8] = [
    WarpData {
        x: 4,
        y: 27,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 27,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 6,
        y: 27,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 7,
        y: 27,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 10,
        dest_map: Some(MapId::PokemonMansion2F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 21,
        y: 23,
        dest_map: Some(MapId::PokemonMansionB1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 26,
        y: 27,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 27,
        y: 27,
        dest_map: None,
        dest_warp_id: 0,
    },
];

static WARPS_POKEMONMANSION2F: [WarpData; 4] = [
    WarpData {
        x: 5,
        y: 10,
        dest_map: Some(MapId::PokemonMansion1F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 7,
        y: 10,
        dest_map: Some(MapId::PokemonMansion3F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 25,
        y: 14,
        dest_map: Some(MapId::PokemonMansion3F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 6,
        y: 1,
        dest_map: Some(MapId::PokemonMansion3F),
        dest_warp_id: 1,
    },
];

static WARPS_POKEMONMANSION3F: [WarpData; 3] = [
    WarpData {
        x: 7,
        y: 10,
        dest_map: Some(MapId::PokemonMansion2F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 6,
        y: 1,
        dest_map: Some(MapId::PokemonMansion2F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 25,
        y: 14,
        dest_map: Some(MapId::PokemonMansion2F),
        dest_warp_id: 2,
    },
];

static WARPS_POKEMONMANSIONB1F: [WarpData; 1] = [WarpData {
    x: 23,
    y: 22,
    dest_map: Some(MapId::PokemonMansion1F),
    dest_warp_id: 5,
}];

static WARPS_POKEMONTOWER1F: [WarpData; 3] = [
    WarpData {
        x: 10,
        y: 17,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 11,
        y: 17,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 18,
        y: 9,
        dest_map: Some(MapId::PokemonTower2F),
        dest_warp_id: 1,
    },
];

static WARPS_POKEMONTOWER2F: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 9,
        dest_map: Some(MapId::PokemonTower3F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 18,
        y: 9,
        dest_map: Some(MapId::PokemonTower1F),
        dest_warp_id: 2,
    },
];

static WARPS_POKEMONTOWER3F: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 9,
        dest_map: Some(MapId::PokemonTower2F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 18,
        y: 9,
        dest_map: Some(MapId::PokemonTower4F),
        dest_warp_id: 1,
    },
];

static WARPS_POKEMONTOWER4F: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 9,
        dest_map: Some(MapId::PokemonTower5F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 18,
        y: 9,
        dest_map: Some(MapId::PokemonTower3F),
        dest_warp_id: 1,
    },
];

static WARPS_POKEMONTOWER5F: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 9,
        dest_map: Some(MapId::PokemonTower4F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 18,
        y: 9,
        dest_map: Some(MapId::PokemonTower6F),
        dest_warp_id: 0,
    },
];

static WARPS_POKEMONTOWER6F: [WarpData; 2] = [
    WarpData {
        x: 18,
        y: 9,
        dest_map: Some(MapId::PokemonTower5F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 9,
        y: 16,
        dest_map: Some(MapId::PokemonTower7F),
        dest_warp_id: 0,
    },
];

static WARPS_POKEMONTOWER7F: [WarpData; 1] = [WarpData {
    x: 9,
    y: 16,
    dest_map: Some(MapId::PokemonTower6F),
    dest_warp_id: 1,
}];

static WARPS_POWERPLANT: [WarpData; 3] = [
    WarpData {
        x: 4,
        y: 35,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 5,
        y: 35,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 0,
        y: 11,
        dest_map: None,
        dest_warp_id: 3,
    },
];

static WARPS_REDSHOUSE1F: [WarpData; 3] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 7,
        y: 1,
        dest_map: Some(MapId::RedsHouse2F),
        dest_warp_id: 0,
    },
];

static WARPS_REDSHOUSE2F: [WarpData; 1] = [WarpData {
    x: 7,
    y: 1,
    dest_map: Some(MapId::RedsHouse1F),
    dest_warp_id: 2,
}];

static WARPS_ROCKTUNNEL1F: [WarpData; 8] = [
    WarpData {
        x: 15,
        y: 3,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 15,
        y: 0,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 15,
        y: 33,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 15,
        y: 35,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 37,
        y: 3,
        dest_map: Some(MapId::RockTunnelB1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 3,
        dest_map: Some(MapId::RockTunnelB1F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 17,
        y: 11,
        dest_map: Some(MapId::RockTunnelB1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 37,
        y: 17,
        dest_map: Some(MapId::RockTunnelB1F),
        dest_warp_id: 3,
    },
];

static WARPS_ROCKTUNNELB1F: [WarpData; 4] = [
    WarpData {
        x: 33,
        y: 25,
        dest_map: Some(MapId::RockTunnel1F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 27,
        y: 3,
        dest_map: Some(MapId::RockTunnel1F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 23,
        y: 11,
        dest_map: Some(MapId::RockTunnel1F),
        dest_warp_id: 6,
    },
    WarpData {
        x: 3,
        y: 3,
        dest_map: Some(MapId::RockTunnel1F),
        dest_warp_id: 7,
    },
];

static WARPS_ROCKTUNNELPOKECENTER: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
];

static WARPS_ROCKETHIDEOUTB1F: [WarpData; 5] = [
    WarpData {
        x: 23,
        y: 2,
        dest_map: Some(MapId::RocketHideoutB2F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 21,
        y: 2,
        dest_map: Some(MapId::GameCorner),
        dest_warp_id: 2,
    },
    WarpData {
        x: 24,
        y: 19,
        dest_map: Some(MapId::RocketHideoutElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 21,
        y: 24,
        dest_map: Some(MapId::RocketHideoutB2F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 25,
        y: 19,
        dest_map: Some(MapId::RocketHideoutElevator),
        dest_warp_id: 1,
    },
];

static WARPS_ROCKETHIDEOUTB2F: [WarpData; 5] = [
    WarpData {
        x: 27,
        y: 8,
        dest_map: Some(MapId::RocketHideoutB1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 21,
        y: 8,
        dest_map: Some(MapId::RocketHideoutB3F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 24,
        y: 19,
        dest_map: Some(MapId::RocketHideoutElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 21,
        y: 22,
        dest_map: Some(MapId::RocketHideoutB1F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 25,
        y: 19,
        dest_map: Some(MapId::RocketHideoutElevator),
        dest_warp_id: 1,
    },
];

static WARPS_ROCKETHIDEOUTB3F: [WarpData; 2] = [
    WarpData {
        x: 25,
        y: 6,
        dest_map: Some(MapId::RocketHideoutB2F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 19,
        y: 18,
        dest_map: Some(MapId::RocketHideoutB4F),
        dest_warp_id: 0,
    },
];

static WARPS_ROCKETHIDEOUTB4F: [WarpData; 3] = [
    WarpData {
        x: 19,
        y: 10,
        dest_map: Some(MapId::RocketHideoutB3F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 24,
        y: 15,
        dest_map: Some(MapId::RocketHideoutElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 25,
        y: 15,
        dest_map: Some(MapId::RocketHideoutElevator),
        dest_warp_id: 1,
    },
];

static WARPS_ROCKETHIDEOUTELEVATOR: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 1,
        dest_map: Some(MapId::RocketHideoutB1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 3,
        y: 1,
        dest_map: Some(MapId::RocketHideoutB1F),
        dest_warp_id: 4,
    },
];

static WARPS_ROUTE10: [WarpData; 4] = [
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

static WARPS_ROUTE11: [WarpData; 5] = [
    WarpData {
        x: 49,
        y: 8,
        dest_map: Some(MapId::Route11Gate1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 49,
        y: 9,
        dest_map: Some(MapId::Route11Gate1F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 58,
        y: 8,
        dest_map: Some(MapId::Route11Gate1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 58,
        y: 9,
        dest_map: Some(MapId::Route11Gate1F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 4,
        y: 5,
        dest_map: Some(MapId::DiglettsCaveRoute11),
        dest_warp_id: 0,
    },
];

static WARPS_ROUTE11GATE1F: [WarpData; 5] = [
    WarpData {
        x: 0,
        y: 4,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 0,
        y: 5,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 7,
        y: 4,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 7,
        y: 5,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 6,
        y: 8,
        dest_map: Some(MapId::Route11Gate2F),
        dest_warp_id: 0,
    },
];

static WARPS_ROUTE11GATE2F: [WarpData; 1] = [WarpData {
    x: 7,
    y: 7,
    dest_map: Some(MapId::Route11Gate1F),
    dest_warp_id: 4,
}];

static WARPS_ROUTE12: [WarpData; 4] = [
    WarpData {
        x: 10,
        y: 15,
        dest_map: Some(MapId::Route12Gate1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 11,
        y: 15,
        dest_map: Some(MapId::Route12Gate1F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 10,
        y: 21,
        dest_map: Some(MapId::Route12Gate1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 11,
        y: 77,
        dest_map: Some(MapId::Route12SuperRodHouse),
        dest_warp_id: 0,
    },
];

static WARPS_ROUTE12GATE1F: [WarpData; 5] = [
    WarpData {
        x: 4,
        y: 0,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 0,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 5,
        y: 7,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 8,
        y: 6,
        dest_map: Some(MapId::Route12Gate2F),
        dest_warp_id: 0,
    },
];

static WARPS_ROUTE12GATE2F: [WarpData; 1] = [WarpData {
    x: 7,
    y: 7,
    dest_map: Some(MapId::Route12Gate1F),
    dest_warp_id: 4,
}];

static WARPS_ROUTE12SUPERRODHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 3,
    },
];

static WARPS_ROUTE15: [WarpData; 4] = [
    WarpData {
        x: 7,
        y: 8,
        dest_map: Some(MapId::Route15Gate1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 7,
        y: 9,
        dest_map: Some(MapId::Route15Gate1F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 14,
        y: 8,
        dest_map: Some(MapId::Route15Gate1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 14,
        y: 9,
        dest_map: Some(MapId::Route15Gate1F),
        dest_warp_id: 3,
    },
];

static WARPS_ROUTE15GATE1F: [WarpData; 5] = [
    WarpData {
        x: 0,
        y: 4,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 0,
        y: 5,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 7,
        y: 4,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 7,
        y: 5,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 6,
        y: 8,
        dest_map: Some(MapId::Route15Gate2F),
        dest_warp_id: 0,
    },
];

static WARPS_ROUTE15GATE2F: [WarpData; 1] = [WarpData {
    x: 7,
    y: 7,
    dest_map: Some(MapId::Route15Gate1F),
    dest_warp_id: 4,
}];

static WARPS_ROUTE16: [WarpData; 9] = [
    WarpData {
        x: 17,
        y: 10,
        dest_map: Some(MapId::Route16Gate1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 17,
        y: 11,
        dest_map: Some(MapId::Route16Gate1F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 24,
        y: 10,
        dest_map: Some(MapId::Route16Gate1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 24,
        y: 11,
        dest_map: Some(MapId::Route16Gate1F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 17,
        y: 4,
        dest_map: Some(MapId::Route16Gate1F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 17,
        y: 5,
        dest_map: Some(MapId::Route16Gate1F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 24,
        y: 4,
        dest_map: Some(MapId::Route16Gate1F),
        dest_warp_id: 6,
    },
    WarpData {
        x: 24,
        y: 5,
        dest_map: Some(MapId::Route16Gate1F),
        dest_warp_id: 7,
    },
    WarpData {
        x: 7,
        y: 5,
        dest_map: Some(MapId::Route16FlyHouse),
        dest_warp_id: 0,
    },
];

static WARPS_ROUTE16FLYHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 8,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 8,
    },
];

static WARPS_ROUTE16GATE1F: [WarpData; 9] = [
    WarpData {
        x: 0,
        y: 8,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 0,
        y: 9,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 7,
        y: 8,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 7,
        y: 9,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 0,
        y: 2,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 0,
        y: 3,
        dest_map: None,
        dest_warp_id: 5,
    },
    WarpData {
        x: 7,
        y: 2,
        dest_map: None,
        dest_warp_id: 6,
    },
    WarpData {
        x: 7,
        y: 3,
        dest_map: None,
        dest_warp_id: 7,
    },
    WarpData {
        x: 6,
        y: 12,
        dest_map: Some(MapId::Route16Gate2F),
        dest_warp_id: 0,
    },
];

static WARPS_ROUTE16GATE2F: [WarpData; 1] = [WarpData {
    x: 7,
    y: 7,
    dest_map: Some(MapId::Route16Gate1F),
    dest_warp_id: 8,
}];

static WARPS_ROUTE18: [WarpData; 4] = [
    WarpData {
        x: 33,
        y: 8,
        dest_map: Some(MapId::Route18Gate1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 33,
        y: 9,
        dest_map: Some(MapId::Route18Gate1F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 40,
        y: 8,
        dest_map: Some(MapId::Route18Gate1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 40,
        y: 9,
        dest_map: Some(MapId::Route18Gate1F),
        dest_warp_id: 3,
    },
];

static WARPS_ROUTE18GATE1F: [WarpData; 5] = [
    WarpData {
        x: 0,
        y: 4,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 0,
        y: 5,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 7,
        y: 4,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 7,
        y: 5,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 6,
        y: 8,
        dest_map: Some(MapId::Route18Gate2F),
        dest_warp_id: 0,
    },
];

static WARPS_ROUTE18GATE2F: [WarpData; 1] = [WarpData {
    x: 7,
    y: 7,
    dest_map: Some(MapId::Route18Gate1F),
    dest_warp_id: 4,
}];

static WARPS_ROUTE2: [WarpData; 6] = [
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

static WARPS_ROUTE20: [WarpData; 2] = [
    WarpData {
        x: 48,
        y: 5,
        dest_map: Some(MapId::SeafoamIslands1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 58,
        y: 9,
        dest_map: Some(MapId::SeafoamIslands1F),
        dest_warp_id: 2,
    },
];

static WARPS_ROUTE22: [WarpData; 1] = [WarpData {
    x: 8,
    y: 5,
    dest_map: Some(MapId::Route22Gate),
    dest_warp_id: 0,
}];

static WARPS_ROUTE22GATE: [WarpData; 4] = [
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 4,
        y: 0,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 0,
        dest_map: None,
        dest_warp_id: 1,
    },
];

static WARPS_ROUTE23: [WarpData; 4] = [
    WarpData {
        x: 7,
        y: 139,
        dest_map: Some(MapId::Route22Gate),
        dest_warp_id: 2,
    },
    WarpData {
        x: 8,
        y: 139,
        dest_map: Some(MapId::Route22Gate),
        dest_warp_id: 3,
    },
    WarpData {
        x: 4,
        y: 31,
        dest_map: Some(MapId::VictoryRoad1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 14,
        y: 31,
        dest_map: Some(MapId::VictoryRoad2F),
        dest_warp_id: 1,
    },
];

static WARPS_ROUTE25: [WarpData; 1] = [WarpData {
    x: 45,
    y: 3,
    dest_map: Some(MapId::BillsHouse),
    dest_warp_id: 0,
}];

static WARPS_ROUTE2GATE: [WarpData; 4] = [
    WarpData {
        x: 4,
        y: 0,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 5,
        y: 0,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 5,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
];

static WARPS_ROUTE2TRADEHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 2,
    },
];

static WARPS_ROUTE4: [WarpData; 3] = [
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

static WARPS_ROUTE5: [WarpData; 5] = [
    WarpData {
        x: 10,
        y: 29,
        dest_map: Some(MapId::Route5Gate),
        dest_warp_id: 3,
    },
    WarpData {
        x: 9,
        y: 29,
        dest_map: Some(MapId::Route5Gate),
        dest_warp_id: 2,
    },
    WarpData {
        x: 10,
        y: 33,
        dest_map: Some(MapId::Route5Gate),
        dest_warp_id: 0,
    },
    WarpData {
        x: 17,
        y: 27,
        dest_map: Some(MapId::UndergroundPathRoute5),
        dest_warp_id: 0,
    },
    WarpData {
        x: 10,
        y: 21,
        dest_map: Some(MapId::Daycare),
        dest_warp_id: 0,
    },
];

static WARPS_ROUTE5GATE: [WarpData; 4] = [
    WarpData {
        x: 3,
        y: 5,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 4,
        y: 5,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 3,
        y: 0,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 4,
        y: 0,
        dest_map: None,
        dest_warp_id: 0,
    },
];

static WARPS_ROUTE6: [WarpData; 4] = [
    WarpData {
        x: 9,
        y: 1,
        dest_map: Some(MapId::Route6Gate),
        dest_warp_id: 2,
    },
    WarpData {
        x: 10,
        y: 1,
        dest_map: Some(MapId::Route6Gate),
        dest_warp_id: 2,
    },
    WarpData {
        x: 10,
        y: 7,
        dest_map: Some(MapId::Route6Gate),
        dest_warp_id: 0,
    },
    WarpData {
        x: 17,
        y: 13,
        dest_map: Some(MapId::UndergroundPathRoute6),
        dest_warp_id: 0,
    },
];

static WARPS_ROUTE6GATE: [WarpData; 4] = [
    WarpData {
        x: 3,
        y: 5,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 4,
        y: 5,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 3,
        y: 0,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 4,
        y: 0,
        dest_map: None,
        dest_warp_id: 1,
    },
];

static WARPS_ROUTE7: [WarpData; 5] = [
    WarpData {
        x: 18,
        y: 9,
        dest_map: Some(MapId::Route7Gate),
        dest_warp_id: 2,
    },
    WarpData {
        x: 18,
        y: 10,
        dest_map: Some(MapId::Route7Gate),
        dest_warp_id: 3,
    },
    WarpData {
        x: 11,
        y: 9,
        dest_map: Some(MapId::Route7Gate),
        dest_warp_id: 0,
    },
    WarpData {
        x: 11,
        y: 10,
        dest_map: Some(MapId::Route7Gate),
        dest_warp_id: 1,
    },
    WarpData {
        x: 5,
        y: 13,
        dest_map: Some(MapId::UndergroundPathRoute7),
        dest_warp_id: 0,
    },
];

static WARPS_ROUTE7GATE: [WarpData; 4] = [
    WarpData {
        x: 0,
        y: 3,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 0,
        y: 4,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 5,
        y: 3,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 4,
        dest_map: None,
        dest_warp_id: 1,
    },
];

static WARPS_ROUTE8: [WarpData; 5] = [
    WarpData {
        x: 1,
        y: 9,
        dest_map: Some(MapId::Route8Gate),
        dest_warp_id: 0,
    },
    WarpData {
        x: 1,
        y: 10,
        dest_map: Some(MapId::Route8Gate),
        dest_warp_id: 1,
    },
    WarpData {
        x: 8,
        y: 9,
        dest_map: Some(MapId::Route8Gate),
        dest_warp_id: 2,
    },
    WarpData {
        x: 8,
        y: 10,
        dest_map: Some(MapId::Route8Gate),
        dest_warp_id: 3,
    },
    WarpData {
        x: 13,
        y: 3,
        dest_map: Some(MapId::UndergroundPathRoute8),
        dest_warp_id: 0,
    },
];

static WARPS_ROUTE8GATE: [WarpData; 4] = [
    WarpData {
        x: 0,
        y: 3,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 0,
        y: 4,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 5,
        y: 3,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 5,
        y: 4,
        dest_map: None,
        dest_warp_id: 3,
    },
];

static WARPS_SSANNE1F: [WarpData; 11] = [
    WarpData {
        x: 26,
        y: 0,
        dest_map: Some(MapId::VermilionDock),
        dest_warp_id: 1,
    },
    WarpData {
        x: 27,
        y: 0,
        dest_map: Some(MapId::VermilionDock),
        dest_warp_id: 1,
    },
    WarpData {
        x: 31,
        y: 8,
        dest_map: Some(MapId::SSAnne1FRooms),
        dest_warp_id: 0,
    },
    WarpData {
        x: 23,
        y: 8,
        dest_map: Some(MapId::SSAnne1FRooms),
        dest_warp_id: 1,
    },
    WarpData {
        x: 19,
        y: 8,
        dest_map: Some(MapId::SSAnne1FRooms),
        dest_warp_id: 2,
    },
    WarpData {
        x: 15,
        y: 8,
        dest_map: Some(MapId::SSAnne1FRooms),
        dest_warp_id: 3,
    },
    WarpData {
        x: 11,
        y: 8,
        dest_map: Some(MapId::SSAnne1FRooms),
        dest_warp_id: 4,
    },
    WarpData {
        x: 7,
        y: 8,
        dest_map: Some(MapId::SSAnne1FRooms),
        dest_warp_id: 5,
    },
    WarpData {
        x: 2,
        y: 6,
        dest_map: Some(MapId::SSAnne2F),
        dest_warp_id: 6,
    },
    WarpData {
        x: 37,
        y: 15,
        dest_map: Some(MapId::SSAnneB1F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 3,
        y: 16,
        dest_map: Some(MapId::SSAnneKitchen),
        dest_warp_id: 0,
    },
];

static WARPS_SSANNE1FROOMS: [WarpData; 6] = [
    WarpData {
        x: 0,
        y: 0,
        dest_map: Some(MapId::SSAnne1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 10,
        y: 0,
        dest_map: Some(MapId::SSAnne1F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 20,
        y: 0,
        dest_map: Some(MapId::SSAnne1F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 0,
        y: 10,
        dest_map: Some(MapId::SSAnne1F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 10,
        y: 10,
        dest_map: Some(MapId::SSAnne1F),
        dest_warp_id: 6,
    },
    WarpData {
        x: 20,
        y: 10,
        dest_map: Some(MapId::SSAnne1F),
        dest_warp_id: 7,
    },
];

static WARPS_SSANNE2F: [WarpData; 9] = [
    WarpData {
        x: 9,
        y: 11,
        dest_map: Some(MapId::SSAnne2FRooms),
        dest_warp_id: 0,
    },
    WarpData {
        x: 13,
        y: 11,
        dest_map: Some(MapId::SSAnne2FRooms),
        dest_warp_id: 2,
    },
    WarpData {
        x: 17,
        y: 11,
        dest_map: Some(MapId::SSAnne2FRooms),
        dest_warp_id: 4,
    },
    WarpData {
        x: 21,
        y: 11,
        dest_map: Some(MapId::SSAnne2FRooms),
        dest_warp_id: 6,
    },
    WarpData {
        x: 25,
        y: 11,
        dest_map: Some(MapId::SSAnne2FRooms),
        dest_warp_id: 8,
    },
    WarpData {
        x: 29,
        y: 11,
        dest_map: Some(MapId::SSAnne2FRooms),
        dest_warp_id: 10,
    },
    WarpData {
        x: 2,
        y: 4,
        dest_map: Some(MapId::SSAnne1F),
        dest_warp_id: 8,
    },
    WarpData {
        x: 2,
        y: 12,
        dest_map: Some(MapId::SSAnne3F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 36,
        y: 4,
        dest_map: Some(MapId::SSAnneCaptainsRoom),
        dest_warp_id: 0,
    },
];

static WARPS_SSANNE2FROOMS: [WarpData; 12] = [
    WarpData {
        x: 2,
        y: 5,
        dest_map: Some(MapId::SSAnne2F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 3,
        y: 5,
        dest_map: Some(MapId::SSAnne2F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 12,
        y: 5,
        dest_map: Some(MapId::SSAnne2F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 13,
        y: 5,
        dest_map: Some(MapId::SSAnne2F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 22,
        y: 5,
        dest_map: Some(MapId::SSAnne2F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 23,
        y: 5,
        dest_map: Some(MapId::SSAnne2F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 2,
        y: 15,
        dest_map: Some(MapId::SSAnne2F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 3,
        y: 15,
        dest_map: Some(MapId::SSAnne2F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 12,
        y: 15,
        dest_map: Some(MapId::SSAnne2F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 13,
        y: 15,
        dest_map: Some(MapId::SSAnne2F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 22,
        y: 15,
        dest_map: Some(MapId::SSAnne2F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 23,
        y: 15,
        dest_map: Some(MapId::SSAnne2F),
        dest_warp_id: 5,
    },
];

static WARPS_SSANNE3F: [WarpData; 2] = [
    WarpData {
        x: 0,
        y: 3,
        dest_map: Some(MapId::SSAnneBow),
        dest_warp_id: 0,
    },
    WarpData {
        x: 19,
        y: 3,
        dest_map: Some(MapId::SSAnne2F),
        dest_warp_id: 7,
    },
];

static WARPS_SSANNEB1F: [WarpData; 6] = [
    WarpData {
        x: 23,
        y: 3,
        dest_map: Some(MapId::SSAnneB1FRooms),
        dest_warp_id: 8,
    },
    WarpData {
        x: 19,
        y: 3,
        dest_map: Some(MapId::SSAnneB1FRooms),
        dest_warp_id: 6,
    },
    WarpData {
        x: 15,
        y: 3,
        dest_map: Some(MapId::SSAnneB1FRooms),
        dest_warp_id: 4,
    },
    WarpData {
        x: 11,
        y: 3,
        dest_map: Some(MapId::SSAnneB1FRooms),
        dest_warp_id: 2,
    },
    WarpData {
        x: 7,
        y: 3,
        dest_map: Some(MapId::SSAnneB1FRooms),
        dest_warp_id: 0,
    },
    WarpData {
        x: 27,
        y: 5,
        dest_map: Some(MapId::SSAnne1F),
        dest_warp_id: 9,
    },
];

static WARPS_SSANNEB1FROOMS: [WarpData; 10] = [
    WarpData {
        x: 2,
        y: 5,
        dest_map: Some(MapId::SSAnneB1F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 3,
        y: 5,
        dest_map: Some(MapId::SSAnneB1F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 12,
        y: 5,
        dest_map: Some(MapId::SSAnneB1F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 13,
        y: 5,
        dest_map: Some(MapId::SSAnneB1F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 22,
        y: 5,
        dest_map: Some(MapId::SSAnneB1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 23,
        y: 5,
        dest_map: Some(MapId::SSAnneB1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 2,
        y: 15,
        dest_map: Some(MapId::SSAnneB1F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 3,
        y: 15,
        dest_map: Some(MapId::SSAnneB1F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 12,
        y: 15,
        dest_map: Some(MapId::SSAnneB1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 13,
        y: 15,
        dest_map: Some(MapId::SSAnneB1F),
        dest_warp_id: 0,
    },
];

static WARPS_SSANNEBOW: [WarpData; 2] = [
    WarpData {
        x: 13,
        y: 6,
        dest_map: Some(MapId::SSAnne3F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 13,
        y: 7,
        dest_map: Some(MapId::SSAnne3F),
        dest_warp_id: 0,
    },
];

static WARPS_SSANNECAPTAINSROOM: [WarpData; 1] = [WarpData {
    x: 0,
    y: 7,
    dest_map: Some(MapId::SSAnne2F),
    dest_warp_id: 8,
}];

static WARPS_SSANNEKITCHEN: [WarpData; 1] = [WarpData {
    x: 6,
    y: 0,
    dest_map: Some(MapId::SSAnne1F),
    dest_warp_id: 10,
}];

static WARPS_SAFARIZONECENTER: [WarpData; 9] = [
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

static WARPS_SAFARIZONECENTERRESTHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: Some(MapId::SafariZoneCenter),
        dest_warp_id: 8,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: Some(MapId::SafariZoneCenter),
        dest_warp_id: 8,
    },
];

static WARPS_SAFARIZONEEAST: [WarpData; 5] = [
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

static WARPS_SAFARIZONEEASTRESTHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: Some(MapId::SafariZoneEast),
        dest_warp_id: 4,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: Some(MapId::SafariZoneEast),
        dest_warp_id: 4,
    },
];

static WARPS_SAFARIZONEGATE: [WarpData; 4] = [
    WarpData {
        x: 3,
        y: 5,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 4,
        y: 5,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 3,
        y: 0,
        dest_map: Some(MapId::SafariZoneCenter),
        dest_warp_id: 0,
    },
    WarpData {
        x: 4,
        y: 0,
        dest_map: Some(MapId::SafariZoneCenter),
        dest_warp_id: 1,
    },
];

static WARPS_SAFARIZONENORTH: [WarpData; 9] = [
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

static WARPS_SAFARIZONENORTHRESTHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: Some(MapId::SafariZoneNorth),
        dest_warp_id: 8,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: Some(MapId::SafariZoneNorth),
        dest_warp_id: 8,
    },
];

static WARPS_SAFARIZONESECRETHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: Some(MapId::SafariZoneWest),
        dest_warp_id: 6,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: Some(MapId::SafariZoneWest),
        dest_warp_id: 6,
    },
];

static WARPS_SAFARIZONEWEST: [WarpData; 8] = [
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

static WARPS_SAFARIZONEWESTRESTHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: Some(MapId::SafariZoneWest),
        dest_warp_id: 7,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: Some(MapId::SafariZoneWest),
        dest_warp_id: 7,
    },
];

static WARPS_SAFFRONCITY: [WarpData; 8] = [
    WarpData {
        x: 7,
        y: 5,
        dest_map: Some(MapId::CopycatsHouse1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 26,
        y: 3,
        dest_map: Some(MapId::FightingDojo),
        dest_warp_id: 0,
    },
    WarpData {
        x: 34,
        y: 3,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 0,
    },
    WarpData {
        x: 13,
        y: 11,
        dest_map: Some(MapId::SaffronPidgeyHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 25,
        y: 11,
        dest_map: Some(MapId::SaffronMart),
        dest_warp_id: 0,
    },
    WarpData {
        x: 18,
        y: 21,
        dest_map: Some(MapId::SilphCo1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 9,
        y: 29,
        dest_map: Some(MapId::SaffronPokecenter),
        dest_warp_id: 0,
    },
    WarpData {
        x: 29,
        y: 29,
        dest_map: Some(MapId::MrPsychicsHouse),
        dest_warp_id: 0,
    },
];

static WARPS_SAFFRONGYM: [WarpData; 32] = [
    WarpData {
        x: 8,
        y: 17,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 9,
        y: 17,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 1,
        y: 3,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 22,
    },
    WarpData {
        x: 5,
        y: 3,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 15,
    },
    WarpData {
        x: 1,
        y: 5,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 18,
    },
    WarpData {
        x: 5,
        y: 5,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 8,
    },
    WarpData {
        x: 1,
        y: 9,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 27,
    },
    WarpData {
        x: 5,
        y: 9,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 16,
    },
    WarpData {
        x: 1,
        y: 11,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 5,
    },
    WarpData {
        x: 5,
        y: 11,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 13,
    },
    WarpData {
        x: 1,
        y: 15,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 23,
    },
    WarpData {
        x: 5,
        y: 15,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 30,
    },
    WarpData {
        x: 1,
        y: 17,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 17,
    },
    WarpData {
        x: 5,
        y: 17,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 9,
    },
    WarpData {
        x: 9,
        y: 3,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 26,
    },
    WarpData {
        x: 11,
        y: 3,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 3,
    },
    WarpData {
        x: 9,
        y: 5,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 7,
    },
    WarpData {
        x: 11,
        y: 5,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 12,
    },
    WarpData {
        x: 11,
        y: 11,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 4,
    },
    WarpData {
        x: 11,
        y: 15,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 31,
    },
    WarpData {
        x: 15,
        y: 3,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 24,
    },
    WarpData {
        x: 19,
        y: 3,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 28,
    },
    WarpData {
        x: 15,
        y: 5,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 2,
    },
    WarpData {
        x: 19,
        y: 5,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 10,
    },
    WarpData {
        x: 15,
        y: 9,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 20,
    },
    WarpData {
        x: 19,
        y: 9,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 29,
    },
    WarpData {
        x: 15,
        y: 11,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 14,
    },
    WarpData {
        x: 19,
        y: 11,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 6,
    },
    WarpData {
        x: 15,
        y: 15,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 21,
    },
    WarpData {
        x: 19,
        y: 15,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 25,
    },
    WarpData {
        x: 15,
        y: 17,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 11,
    },
    WarpData {
        x: 19,
        y: 17,
        dest_map: Some(MapId::SaffronGym),
        dest_warp_id: 19,
    },
];

static WARPS_SAFFRONMART: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
];

static WARPS_SAFFRONPIDGEYHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 3,
    },
];

static WARPS_SAFFRONPOKECENTER: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 6,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 6,
    },
];

static WARPS_SEAFOAMISLANDS1F: [WarpData; 7] = [
    WarpData {
        x: 4,
        y: 17,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 17,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 26,
        y: 17,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 27,
        y: 17,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 7,
        y: 5,
        dest_map: Some(MapId::SeafoamIslandsB1F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 25,
        y: 3,
        dest_map: Some(MapId::SeafoamIslandsB1F),
        dest_warp_id: 6,
    },
    WarpData {
        x: 23,
        y: 15,
        dest_map: Some(MapId::SeafoamIslandsB1F),
        dest_warp_id: 4,
    },
];

static WARPS_SEAFOAMISLANDSB1F: [WarpData; 7] = [
    WarpData {
        x: 4,
        y: 2,
        dest_map: Some(MapId::SeafoamIslandsB2F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 7,
        y: 5,
        dest_map: Some(MapId::SeafoamIslands1F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 13,
        y: 7,
        dest_map: Some(MapId::SeafoamIslandsB2F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 19,
        y: 15,
        dest_map: Some(MapId::SeafoamIslandsB2F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 23,
        y: 15,
        dest_map: Some(MapId::SeafoamIslands1F),
        dest_warp_id: 6,
    },
    WarpData {
        x: 25,
        y: 11,
        dest_map: Some(MapId::SeafoamIslandsB2F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 25,
        y: 3,
        dest_map: Some(MapId::SeafoamIslands1F),
        dest_warp_id: 5,
    },
];

static WARPS_SEAFOAMISLANDSB2F: [WarpData; 7] = [
    WarpData {
        x: 5,
        y: 3,
        dest_map: Some(MapId::SeafoamIslandsB1F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 13,
        dest_map: Some(MapId::SeafoamIslandsB3F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 13,
        y: 7,
        dest_map: Some(MapId::SeafoamIslandsB1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 19,
        y: 15,
        dest_map: Some(MapId::SeafoamIslandsB1F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 25,
        y: 3,
        dest_map: Some(MapId::SeafoamIslandsB3F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 25,
        y: 11,
        dest_map: Some(MapId::SeafoamIslandsB1F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 25,
        y: 14,
        dest_map: Some(MapId::SeafoamIslandsB3F),
        dest_warp_id: 4,
    },
];

static WARPS_SEAFOAMISLANDSB3F: [WarpData; 7] = [
    WarpData {
        x: 5,
        y: 12,
        dest_map: Some(MapId::SeafoamIslandsB2F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 8,
        y: 6,
        dest_map: Some(MapId::SeafoamIslandsB4F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 25,
        y: 4,
        dest_map: Some(MapId::SeafoamIslandsB4F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 25,
        y: 3,
        dest_map: Some(MapId::SeafoamIslandsB2F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 25,
        y: 14,
        dest_map: Some(MapId::SeafoamIslandsB2F),
        dest_warp_id: 6,
    },
    WarpData {
        x: 20,
        y: 17,
        dest_map: Some(MapId::SeafoamIslandsB4F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 21,
        y: 17,
        dest_map: Some(MapId::SeafoamIslandsB4F),
        dest_warp_id: 1,
    },
];

static WARPS_SEAFOAMISLANDSB4F: [WarpData; 4] = [
    WarpData {
        x: 20,
        y: 17,
        dest_map: Some(MapId::SeafoamIslandsB3F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 21,
        y: 17,
        dest_map: Some(MapId::SeafoamIslandsB3F),
        dest_warp_id: 6,
    },
    WarpData {
        x: 11,
        y: 7,
        dest_map: Some(MapId::SeafoamIslandsB3F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 25,
        y: 4,
        dest_map: Some(MapId::SeafoamIslandsB3F),
        dest_warp_id: 2,
    },
];

static WARPS_SILPHCO10F: [WarpData; 6] = [
    WarpData {
        x: 8,
        y: 0,
        dest_map: Some(MapId::SilphCo9F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 10,
        y: 0,
        dest_map: Some(MapId::SilphCo11F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 12,
        y: 0,
        dest_map: Some(MapId::SilphCoElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 9,
        y: 11,
        dest_map: Some(MapId::SilphCo4F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 13,
        y: 15,
        dest_map: Some(MapId::SilphCo4F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 13,
        y: 7,
        dest_map: Some(MapId::SilphCo4F),
        dest_warp_id: 6,
    },
];

static WARPS_SILPHCO11F: [WarpData; 4] = [
    WarpData {
        x: 9,
        y: 0,
        dest_map: Some(MapId::SilphCo10F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 13,
        y: 0,
        dest_map: Some(MapId::SilphCoElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 5,
        dest_map: None,
        dest_warp_id: 9,
    },
    WarpData {
        x: 3,
        y: 2,
        dest_map: Some(MapId::SilphCo7F),
        dest_warp_id: 3,
    },
];

static WARPS_SILPHCO1F: [WarpData; 5] = [
    WarpData {
        x: 10,
        y: 17,
        dest_map: None,
        dest_warp_id: 5,
    },
    WarpData {
        x: 11,
        y: 17,
        dest_map: None,
        dest_warp_id: 5,
    },
    WarpData {
        x: 26,
        y: 0,
        dest_map: Some(MapId::SilphCo2F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 20,
        y: 0,
        dest_map: Some(MapId::SilphCoElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 16,
        y: 10,
        dest_map: Some(MapId::SilphCo3F),
        dest_warp_id: 6,
    },
];

static WARPS_SILPHCO2F: [WarpData; 7] = [
    WarpData {
        x: 24,
        y: 0,
        dest_map: Some(MapId::SilphCo1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 26,
        y: 0,
        dest_map: Some(MapId::SilphCo3F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 20,
        y: 0,
        dest_map: Some(MapId::SilphCoElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 3,
        y: 3,
        dest_map: Some(MapId::SilphCo3F),
        dest_warp_id: 6,
    },
    WarpData {
        x: 13,
        y: 3,
        dest_map: Some(MapId::SilphCo8F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 27,
        y: 15,
        dest_map: Some(MapId::SilphCo8F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 9,
        y: 15,
        dest_map: Some(MapId::SilphCo6F),
        dest_warp_id: 4,
    },
];

static WARPS_SILPHCO3F: [WarpData; 10] = [
    WarpData {
        x: 26,
        y: 0,
        dest_map: Some(MapId::SilphCo2F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 24,
        y: 0,
        dest_map: Some(MapId::SilphCo4F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 20,
        y: 0,
        dest_map: Some(MapId::SilphCoElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 23,
        y: 11,
        dest_map: Some(MapId::SilphCo3F),
        dest_warp_id: 9,
    },
    WarpData {
        x: 3,
        y: 3,
        dest_map: Some(MapId::SilphCo5F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 3,
        y: 15,
        dest_map: Some(MapId::SilphCo5F),
        dest_warp_id: 6,
    },
    WarpData {
        x: 27,
        y: 3,
        dest_map: Some(MapId::SilphCo2F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 3,
        y: 11,
        dest_map: Some(MapId::SilphCo9F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 11,
        y: 11,
        dest_map: Some(MapId::SilphCo7F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 27,
        y: 15,
        dest_map: Some(MapId::SilphCo3F),
        dest_warp_id: 3,
    },
];

static WARPS_SILPHCO4F: [WarpData; 7] = [
    WarpData {
        x: 24,
        y: 0,
        dest_map: Some(MapId::SilphCo3F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 26,
        y: 0,
        dest_map: Some(MapId::SilphCo5F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 20,
        y: 0,
        dest_map: Some(MapId::SilphCoElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 11,
        y: 7,
        dest_map: Some(MapId::SilphCo10F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 17,
        y: 3,
        dest_map: Some(MapId::SilphCo6F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 3,
        y: 15,
        dest_map: Some(MapId::SilphCo10F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 17,
        y: 11,
        dest_map: Some(MapId::SilphCo10F),
        dest_warp_id: 5,
    },
];

static WARPS_SILPHCO5F: [WarpData; 7] = [
    WarpData {
        x: 24,
        y: 0,
        dest_map: Some(MapId::SilphCo6F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 26,
        y: 0,
        dest_map: Some(MapId::SilphCo4F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 20,
        y: 0,
        dest_map: Some(MapId::SilphCoElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 27,
        y: 3,
        dest_map: Some(MapId::SilphCo7F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 9,
        y: 15,
        dest_map: Some(MapId::SilphCo9F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 11,
        y: 5,
        dest_map: Some(MapId::SilphCo3F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 3,
        y: 15,
        dest_map: Some(MapId::SilphCo3F),
        dest_warp_id: 5,
    },
];

static WARPS_SILPHCO6F: [WarpData; 5] = [
    WarpData {
        x: 16,
        y: 0,
        dest_map: Some(MapId::SilphCo7F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 14,
        y: 0,
        dest_map: Some(MapId::SilphCo5F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 18,
        y: 0,
        dest_map: Some(MapId::SilphCoElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 3,
        y: 3,
        dest_map: Some(MapId::SilphCo4F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 23,
        y: 3,
        dest_map: Some(MapId::SilphCo2F),
        dest_warp_id: 6,
    },
];

static WARPS_SILPHCO7F: [WarpData; 6] = [
    WarpData {
        x: 16,
        y: 0,
        dest_map: Some(MapId::SilphCo8F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 22,
        y: 0,
        dest_map: Some(MapId::SilphCo6F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 18,
        y: 0,
        dest_map: Some(MapId::SilphCoElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 7,
        dest_map: Some(MapId::SilphCo11F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 5,
        y: 3,
        dest_map: Some(MapId::SilphCo3F),
        dest_warp_id: 8,
    },
    WarpData {
        x: 21,
        y: 15,
        dest_map: Some(MapId::SilphCo5F),
        dest_warp_id: 3,
    },
];

static WARPS_SILPHCO8F: [WarpData; 7] = [
    WarpData {
        x: 16,
        y: 0,
        dest_map: Some(MapId::SilphCo9F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 14,
        y: 0,
        dest_map: Some(MapId::SilphCo7F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 18,
        y: 0,
        dest_map: Some(MapId::SilphCoElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 3,
        y: 11,
        dest_map: Some(MapId::SilphCo8F),
        dest_warp_id: 6,
    },
    WarpData {
        x: 3,
        y: 15,
        dest_map: Some(MapId::SilphCo2F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 11,
        y: 5,
        dest_map: Some(MapId::SilphCo2F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 11,
        y: 9,
        dest_map: Some(MapId::SilphCo8F),
        dest_warp_id: 3,
    },
];

static WARPS_SILPHCO9F: [WarpData; 5] = [
    WarpData {
        x: 14,
        y: 0,
        dest_map: Some(MapId::SilphCo10F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 16,
        y: 0,
        dest_map: Some(MapId::SilphCo8F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 18,
        y: 0,
        dest_map: Some(MapId::SilphCoElevator),
        dest_warp_id: 0,
    },
    WarpData {
        x: 9,
        y: 3,
        dest_map: Some(MapId::SilphCo3F),
        dest_warp_id: 7,
    },
    WarpData {
        x: 17,
        y: 15,
        dest_map: Some(MapId::SilphCo5F),
        dest_warp_id: 4,
    },
];

static WARPS_SILPHCOELEVATOR: [WarpData; 2] = [
    WarpData {
        x: 1,
        y: 3,
        dest_map: None, /* UNUSED_MAP_ED */
        dest_warp_id: 0,
    },
    WarpData {
        x: 2,
        y: 3,
        dest_map: None, /* UNUSED_MAP_ED */
        dest_warp_id: 0,
    },
];

static WARPS_UNDERGROUNDPATHNORTHSOUTH: [WarpData; 2] = [
    WarpData {
        x: 5,
        y: 4,
        dest_map: Some(MapId::UndergroundPathRoute5),
        dest_warp_id: 2,
    },
    WarpData {
        x: 2,
        y: 41,
        dest_map: Some(MapId::UndergroundPathRoute6),
        dest_warp_id: 2,
    },
];

static WARPS_UNDERGROUNDPATHROUTE5: [WarpData; 3] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 4,
        y: 4,
        dest_map: Some(MapId::UndergroundPathNorthSouth),
        dest_warp_id: 0,
    },
];

static WARPS_UNDERGROUNDPATHROUTE6: [WarpData; 3] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 4,
        y: 4,
        dest_map: Some(MapId::UndergroundPathNorthSouth),
        dest_warp_id: 1,
    },
];

static WARPS_UNDERGROUNDPATHROUTE7: [WarpData; 3] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 4,
        y: 4,
        dest_map: Some(MapId::UndergroundPathWestEast),
        dest_warp_id: 0,
    },
];

static WARPS_UNDERGROUNDPATHROUTE7COPY: [WarpData; 3] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 5,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 5,
    },
    WarpData {
        x: 4,
        y: 4,
        dest_map: Some(MapId::UndergroundPathWestEast),
        dest_warp_id: 0,
    },
];

static WARPS_UNDERGROUNDPATHROUTE8: [WarpData; 3] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 4,
        y: 4,
        dest_map: Some(MapId::UndergroundPathWestEast),
        dest_warp_id: 1,
    },
];

static WARPS_UNDERGROUNDPATHWESTEAST: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 5,
        dest_map: Some(MapId::UndergroundPathRoute7),
        dest_warp_id: 2,
    },
    WarpData {
        x: 47,
        y: 2,
        dest_map: Some(MapId::UndergroundPathRoute8),
        dest_warp_id: 2,
    },
];

static WARPS_VERMILIONCITY: [WarpData; 9] = [
    WarpData {
        x: 11,
        y: 3,
        dest_map: Some(MapId::VermilionPokecenter),
        dest_warp_id: 0,
    },
    WarpData {
        x: 9,
        y: 13,
        dest_map: Some(MapId::PokemonFanClub),
        dest_warp_id: 0,
    },
    WarpData {
        x: 23,
        y: 13,
        dest_map: Some(MapId::VermilionMart),
        dest_warp_id: 0,
    },
    WarpData {
        x: 12,
        y: 19,
        dest_map: Some(MapId::VermilionGym),
        dest_warp_id: 0,
    },
    WarpData {
        x: 23,
        y: 19,
        dest_map: Some(MapId::VermilionPidgeyHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 18,
        y: 31,
        dest_map: Some(MapId::VermilionDock),
        dest_warp_id: 0,
    },
    WarpData {
        x: 19,
        y: 31,
        dest_map: Some(MapId::VermilionDock),
        dest_warp_id: 0,
    },
    WarpData {
        x: 15,
        y: 13,
        dest_map: Some(MapId::VermilionTradeHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 7,
        y: 3,
        dest_map: Some(MapId::VermilionOldRodHouse),
        dest_warp_id: 0,
    },
];

static WARPS_VERMILIONDOCK: [WarpData; 2] = [
    WarpData {
        x: 14,
        y: 0,
        dest_map: None,
        dest_warp_id: 5,
    },
    WarpData {
        x: 14,
        y: 2,
        dest_map: Some(MapId::SSAnne1F),
        dest_warp_id: 1,
    },
];

static WARPS_VERMILIONGYM: [WarpData; 2] = [
    WarpData {
        x: 4,
        y: 17,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 5,
        y: 17,
        dest_map: None,
        dest_warp_id: 3,
    },
];

static WARPS_VERMILIONMART: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 2,
    },
];

static WARPS_VERMILIONOLDRODHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 8,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 8,
    },
];

static WARPS_VERMILIONPIDGEYHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 4,
    },
];

static WARPS_VERMILIONPOKECENTER: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
];

static WARPS_VERMILIONTRADEHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 7,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 7,
    },
];

static WARPS_VICTORYROAD1F: [WarpData; 3] = [
    WarpData {
        x: 8,
        y: 17,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 9,
        y: 17,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 1,
        y: 1,
        dest_map: Some(MapId::VictoryRoad2F),
        dest_warp_id: 0,
    },
];

static WARPS_VICTORYROAD2F: [WarpData; 7] = [
    WarpData {
        x: 0,
        y: 8,
        dest_map: Some(MapId::VictoryRoad1F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 29,
        y: 7,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 29,
        y: 8,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 23,
        y: 7,
        dest_map: Some(MapId::VictoryRoad3F),
        dest_warp_id: 0,
    },
    WarpData {
        x: 25,
        y: 14,
        dest_map: Some(MapId::VictoryRoad3F),
        dest_warp_id: 2,
    },
    WarpData {
        x: 27,
        y: 7,
        dest_map: Some(MapId::VictoryRoad3F),
        dest_warp_id: 1,
    },
    WarpData {
        x: 1,
        y: 1,
        dest_map: Some(MapId::VictoryRoad3F),
        dest_warp_id: 3,
    },
];

static WARPS_VICTORYROAD3F: [WarpData; 4] = [
    WarpData {
        x: 23,
        y: 7,
        dest_map: Some(MapId::VictoryRoad2F),
        dest_warp_id: 3,
    },
    WarpData {
        x: 26,
        y: 8,
        dest_map: Some(MapId::VictoryRoad2F),
        dest_warp_id: 5,
    },
    WarpData {
        x: 27,
        y: 15,
        dest_map: Some(MapId::VictoryRoad2F),
        dest_warp_id: 4,
    },
    WarpData {
        x: 2,
        y: 0,
        dest_map: Some(MapId::VictoryRoad2F),
        dest_warp_id: 6,
    },
];

static WARPS_VIRIDIANCITY: [WarpData; 5] = [
    WarpData {
        x: 23,
        y: 25,
        dest_map: Some(MapId::ViridianPokecenter),
        dest_warp_id: 0,
    },
    WarpData {
        x: 29,
        y: 19,
        dest_map: Some(MapId::ViridianMart),
        dest_warp_id: 0,
    },
    WarpData {
        x: 21,
        y: 15,
        dest_map: Some(MapId::ViridianSchoolHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 21,
        y: 9,
        dest_map: Some(MapId::ViridianNicknameHouse),
        dest_warp_id: 0,
    },
    WarpData {
        x: 32,
        y: 7,
        dest_map: Some(MapId::ViridianGym),
        dest_warp_id: 0,
    },
];

static WARPS_VIRIDIANFOREST: [WarpData; 6] = [
    WarpData {
        x: 1,
        y: 0,
        dest_map: Some(MapId::ViridianForestNorthGate),
        dest_warp_id: 2,
    },
    WarpData {
        x: 2,
        y: 0,
        dest_map: Some(MapId::ViridianForestNorthGate),
        dest_warp_id: 3,
    },
    WarpData {
        x: 15,
        y: 47,
        dest_map: Some(MapId::ViridianForestSouthGate),
        dest_warp_id: 1,
    },
    WarpData {
        x: 16,
        y: 47,
        dest_map: Some(MapId::ViridianForestSouthGate),
        dest_warp_id: 1,
    },
    WarpData {
        x: 17,
        y: 47,
        dest_map: Some(MapId::ViridianForestSouthGate),
        dest_warp_id: 1,
    },
    WarpData {
        x: 18,
        y: 47,
        dest_map: Some(MapId::ViridianForestSouthGate),
        dest_warp_id: 1,
    },
];

static WARPS_VIRIDIANFORESTNORTHGATE: [WarpData; 4] = [
    WarpData {
        x: 4,
        y: 0,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 5,
        y: 0,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: Some(MapId::ViridianForest),
        dest_warp_id: 0,
    },
    WarpData {
        x: 5,
        y: 7,
        dest_map: Some(MapId::ViridianForest),
        dest_warp_id: 0,
    },
];

static WARPS_VIRIDIANFORESTSOUTHGATE: [WarpData; 4] = [
    WarpData {
        x: 4,
        y: 0,
        dest_map: Some(MapId::ViridianForest),
        dest_warp_id: 3,
    },
    WarpData {
        x: 5,
        y: 0,
        dest_map: Some(MapId::ViridianForest),
        dest_warp_id: 4,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 5,
    },
    WarpData {
        x: 5,
        y: 7,
        dest_map: None,
        dest_warp_id: 5,
    },
];

static WARPS_VIRIDIANGYM: [WarpData; 2] = [
    WarpData {
        x: 16,
        y: 17,
        dest_map: None,
        dest_warp_id: 4,
    },
    WarpData {
        x: 17,
        y: 17,
        dest_map: None,
        dest_warp_id: 4,
    },
];

static WARPS_VIRIDIANMART: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 1,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 1,
    },
];

static WARPS_VIRIDIANNICKNAMEHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 3,
    },
];

static WARPS_VIRIDIANPOKECENTER: [WarpData; 2] = [
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 0,
    },
];

static WARPS_VIRIDIANSCHOOLHOUSE: [WarpData; 2] = [
    WarpData {
        x: 2,
        y: 7,
        dest_map: None,
        dest_warp_id: 2,
    },
    WarpData {
        x: 3,
        y: 7,
        dest_map: None,
        dest_warp_id: 2,
    },
];

static WARPS_WARDENSHOUSE: [WarpData; 2] = [
    WarpData {
        x: 4,
        y: 7,
        dest_map: None,
        dest_warp_id: 3,
    },
    WarpData {
        x: 5,
        y: 7,
        dest_map: None,
        dest_warp_id: 3,
    },
];

static EMPTY_WARPS: [WarpData; 0] = [];

pub fn get_map_warps(map: MapId) -> &'static [WarpData] {
    match map {
        MapId::AgathasRoom => &WARPS_AGATHASROOM,
        MapId::BikeShop => &WARPS_BIKESHOP,
        MapId::BillsHouse => &WARPS_BILLSHOUSE,
        MapId::BluesHouse => &WARPS_BLUESHOUSE,
        MapId::BrunosRoom => &WARPS_BRUNOSROOM,
        MapId::CeladonChiefHouse => &WARPS_CELADONCHIEFHOUSE,
        MapId::CeladonCity => &WARPS_CELADONCITY,
        MapId::CeladonDiner => &WARPS_CELADONDINER,
        MapId::CeladonGym => &WARPS_CELADONGYM,
        MapId::CeladonHotel => &WARPS_CELADONHOTEL,
        MapId::CeladonMansion1F => &WARPS_CELADONMANSION1F,
        MapId::CeladonMansion2F => &WARPS_CELADONMANSION2F,
        MapId::CeladonMansion3F => &WARPS_CELADONMANSION3F,
        MapId::CeladonMansionRoof => &WARPS_CELADONMANSIONROOF,
        MapId::CeladonMansionRoofHouse => &WARPS_CELADONMANSIONROOFHOUSE,
        MapId::CeladonMart1F => &WARPS_CELADONMART1F,
        MapId::CeladonMart2F => &WARPS_CELADONMART2F,
        MapId::CeladonMart3F => &WARPS_CELADONMART3F,
        MapId::CeladonMart4F => &WARPS_CELADONMART4F,
        MapId::CeladonMart5F => &WARPS_CELADONMART5F,
        MapId::CeladonMartElevator => &WARPS_CELADONMARTELEVATOR,
        MapId::CeladonMartRoof => &WARPS_CELADONMARTROOF,
        MapId::CeladonPokecenter => &WARPS_CELADONPOKECENTER,
        MapId::CeruleanBadgeHouse => &WARPS_CERULEANBADGEHOUSE,
        MapId::CeruleanCave1F => &WARPS_CERULEANCAVE1F,
        MapId::CeruleanCave2F => &WARPS_CERULEANCAVE2F,
        MapId::CeruleanCaveB1F => &WARPS_CERULEANCAVEB1F,
        MapId::CeruleanCity => &WARPS_CERULEANCITY,
        MapId::CeruleanGym => &WARPS_CERULEANGYM,
        MapId::CeruleanMart => &WARPS_CERULEANMART,
        MapId::CeruleanPokecenter => &WARPS_CERULEANPOKECENTER,
        MapId::CeruleanTradeHouse => &WARPS_CERULEANTRADEHOUSE,
        MapId::CeruleanTrashedHouse => &WARPS_CERULEANTRASHEDHOUSE,
        MapId::ChampionsRoom => &WARPS_CHAMPIONSROOM,
        MapId::CinnabarGym => &WARPS_CINNABARGYM,
        MapId::CinnabarIsland => &WARPS_CINNABARISLAND,
        MapId::CinnabarLab => &WARPS_CINNABARLAB,
        MapId::CinnabarLabFossilRoom => &WARPS_CINNABARLABFOSSILROOM,
        MapId::CinnabarLabMetronomeRoom => &WARPS_CINNABARLABMETRONOMEROOM,
        MapId::CinnabarLabTradeRoom => &WARPS_CINNABARLABTRADEROOM,
        MapId::CinnabarMart => &WARPS_CINNABARMART,
        MapId::CinnabarPokecenter => &WARPS_CINNABARPOKECENTER,
        MapId::CopycatsHouse1F => &WARPS_COPYCATSHOUSE1F,
        MapId::CopycatsHouse2F => &WARPS_COPYCATSHOUSE2F,
        MapId::Daycare => &WARPS_DAYCARE,
        MapId::DiglettsCave => &WARPS_DIGLETTSCAVE,
        MapId::DiglettsCaveRoute11 => &WARPS_DIGLETTSCAVEROUTE11,
        MapId::DiglettsCaveRoute2 => &WARPS_DIGLETTSCAVEROUTE2,
        MapId::FightingDojo => &WARPS_FIGHTINGDOJO,
        MapId::FuchsiaBillsGrandpasHouse => &WARPS_FUCHSIABILLSGRANDPASHOUSE,
        MapId::FuchsiaCity => &WARPS_FUCHSIACITY,
        MapId::FuchsiaGoodRodHouse => &WARPS_FUCHSIAGOODRODHOUSE,
        MapId::FuchsiaGym => &WARPS_FUCHSIAGYM,
        MapId::FuchsiaMart => &WARPS_FUCHSIAMART,
        MapId::FuchsiaMeetingRoom => &WARPS_FUCHSIAMEETINGROOM,
        MapId::FuchsiaPokecenter => &WARPS_FUCHSIAPOKECENTER,
        MapId::GameCorner => &WARPS_GAMECORNER,
        MapId::GameCornerPrizeRoom => &WARPS_GAMECORNERPRIZEROOM,
        MapId::HallOfFame => &WARPS_HALLOFFAME,
        MapId::IndigoPlateau => &WARPS_INDIGOPLATEAU,
        MapId::IndigoPlateauLobby => &WARPS_INDIGOPLATEAULOBBY,
        MapId::LancesRoom => &WARPS_LANCESROOM,
        MapId::LavenderCuboneHouse => &WARPS_LAVENDERCUBONEHOUSE,
        MapId::LavenderMart => &WARPS_LAVENDERMART,
        MapId::LavenderPokecenter => &WARPS_LAVENDERPOKECENTER,
        MapId::LavenderTown => &WARPS_LAVENDERTOWN,
        MapId::LoreleisRoom => &WARPS_LORELEISROOM,
        MapId::MrFujisHouse => &WARPS_MRFUJISHOUSE,
        MapId::MrPsychicsHouse => &WARPS_MRPSYCHICSHOUSE,
        MapId::MtMoon1F => &WARPS_MTMOON1F,
        MapId::MtMoonB1F => &WARPS_MTMOONB1F,
        MapId::MtMoonB2F => &WARPS_MTMOONB2F,
        MapId::MtMoonPokecenter => &WARPS_MTMOONPOKECENTER,
        MapId::Museum1F => &WARPS_MUSEUM1F,
        MapId::Museum2F => &WARPS_MUSEUM2F,
        MapId::NameRatersHouse => &WARPS_NAMERATERSHOUSE,
        MapId::OaksLab => &WARPS_OAKSLAB,
        MapId::PalletTown => &WARPS_PALLETTOWN,
        MapId::PewterCity => &WARPS_PEWTERCITY,
        MapId::PewterGym => &WARPS_PEWTERGYM,
        MapId::PewterMart => &WARPS_PEWTERMART,
        MapId::PewterNidoranHouse => &WARPS_PEWTERNIDORANHOUSE,
        MapId::PewterPokecenter => &WARPS_PEWTERPOKECENTER,
        MapId::PewterSpeechHouse => &WARPS_PEWTERSPEECHHOUSE,
        MapId::PokemonFanClub => &WARPS_POKEMONFANCLUB,
        MapId::PokemonMansion1F => &WARPS_POKEMONMANSION1F,
        MapId::PokemonMansion2F => &WARPS_POKEMONMANSION2F,
        MapId::PokemonMansion3F => &WARPS_POKEMONMANSION3F,
        MapId::PokemonMansionB1F => &WARPS_POKEMONMANSIONB1F,
        MapId::PokemonTower1F => &WARPS_POKEMONTOWER1F,
        MapId::PokemonTower2F => &WARPS_POKEMONTOWER2F,
        MapId::PokemonTower3F => &WARPS_POKEMONTOWER3F,
        MapId::PokemonTower4F => &WARPS_POKEMONTOWER4F,
        MapId::PokemonTower5F => &WARPS_POKEMONTOWER5F,
        MapId::PokemonTower6F => &WARPS_POKEMONTOWER6F,
        MapId::PokemonTower7F => &WARPS_POKEMONTOWER7F,
        MapId::PowerPlant => &WARPS_POWERPLANT,
        MapId::RedsHouse1F => &WARPS_REDSHOUSE1F,
        MapId::RedsHouse2F => &WARPS_REDSHOUSE2F,
        MapId::RockTunnel1F => &WARPS_ROCKTUNNEL1F,
        MapId::RockTunnelB1F => &WARPS_ROCKTUNNELB1F,
        MapId::RockTunnelPokecenter => &WARPS_ROCKTUNNELPOKECENTER,
        MapId::RocketHideoutB1F => &WARPS_ROCKETHIDEOUTB1F,
        MapId::RocketHideoutB2F => &WARPS_ROCKETHIDEOUTB2F,
        MapId::RocketHideoutB3F => &WARPS_ROCKETHIDEOUTB3F,
        MapId::RocketHideoutB4F => &WARPS_ROCKETHIDEOUTB4F,
        MapId::RocketHideoutElevator => &WARPS_ROCKETHIDEOUTELEVATOR,
        MapId::Route10 => &WARPS_ROUTE10,
        MapId::Route11 => &WARPS_ROUTE11,
        MapId::Route11Gate1F => &WARPS_ROUTE11GATE1F,
        MapId::Route11Gate2F => &WARPS_ROUTE11GATE2F,
        MapId::Route12 => &WARPS_ROUTE12,
        MapId::Route12Gate1F => &WARPS_ROUTE12GATE1F,
        MapId::Route12Gate2F => &WARPS_ROUTE12GATE2F,
        MapId::Route12SuperRodHouse => &WARPS_ROUTE12SUPERRODHOUSE,
        MapId::Route15 => &WARPS_ROUTE15,
        MapId::Route15Gate1F => &WARPS_ROUTE15GATE1F,
        MapId::Route15Gate2F => &WARPS_ROUTE15GATE2F,
        MapId::Route16 => &WARPS_ROUTE16,
        MapId::Route16FlyHouse => &WARPS_ROUTE16FLYHOUSE,
        MapId::Route16Gate1F => &WARPS_ROUTE16GATE1F,
        MapId::Route16Gate2F => &WARPS_ROUTE16GATE2F,
        MapId::Route18 => &WARPS_ROUTE18,
        MapId::Route18Gate1F => &WARPS_ROUTE18GATE1F,
        MapId::Route18Gate2F => &WARPS_ROUTE18GATE2F,
        MapId::Route2 => &WARPS_ROUTE2,
        MapId::Route20 => &WARPS_ROUTE20,
        MapId::Route22 => &WARPS_ROUTE22,
        MapId::Route22Gate => &WARPS_ROUTE22GATE,
        MapId::Route23 => &WARPS_ROUTE23,
        MapId::Route25 => &WARPS_ROUTE25,
        MapId::Route2Gate => &WARPS_ROUTE2GATE,
        MapId::Route2TradeHouse => &WARPS_ROUTE2TRADEHOUSE,
        MapId::Route4 => &WARPS_ROUTE4,
        MapId::Route5 => &WARPS_ROUTE5,
        MapId::Route5Gate => &WARPS_ROUTE5GATE,
        MapId::Route6 => &WARPS_ROUTE6,
        MapId::Route6Gate => &WARPS_ROUTE6GATE,
        MapId::Route7 => &WARPS_ROUTE7,
        MapId::Route7Gate => &WARPS_ROUTE7GATE,
        MapId::Route8 => &WARPS_ROUTE8,
        MapId::Route8Gate => &WARPS_ROUTE8GATE,
        MapId::SSAnne1F => &WARPS_SSANNE1F,
        MapId::SSAnne1FRooms => &WARPS_SSANNE1FROOMS,
        MapId::SSAnne2F => &WARPS_SSANNE2F,
        MapId::SSAnne2FRooms => &WARPS_SSANNE2FROOMS,
        MapId::SSAnne3F => &WARPS_SSANNE3F,
        MapId::SSAnneB1F => &WARPS_SSANNEB1F,
        MapId::SSAnneB1FRooms => &WARPS_SSANNEB1FROOMS,
        MapId::SSAnneBow => &WARPS_SSANNEBOW,
        MapId::SSAnneCaptainsRoom => &WARPS_SSANNECAPTAINSROOM,
        MapId::SSAnneKitchen => &WARPS_SSANNEKITCHEN,
        MapId::SafariZoneCenter => &WARPS_SAFARIZONECENTER,
        MapId::SafariZoneCenterRestHouse => &WARPS_SAFARIZONECENTERRESTHOUSE,
        MapId::SafariZoneEast => &WARPS_SAFARIZONEEAST,
        MapId::SafariZoneEastRestHouse => &WARPS_SAFARIZONEEASTRESTHOUSE,
        MapId::SafariZoneGate => &WARPS_SAFARIZONEGATE,
        MapId::SafariZoneNorth => &WARPS_SAFARIZONENORTH,
        MapId::SafariZoneNorthRestHouse => &WARPS_SAFARIZONENORTHRESTHOUSE,
        MapId::SafariZoneSecretHouse => &WARPS_SAFARIZONESECRETHOUSE,
        MapId::SafariZoneWest => &WARPS_SAFARIZONEWEST,
        MapId::SafariZoneWestRestHouse => &WARPS_SAFARIZONEWESTRESTHOUSE,
        MapId::SaffronCity => &WARPS_SAFFRONCITY,
        MapId::SaffronGym => &WARPS_SAFFRONGYM,
        MapId::SaffronMart => &WARPS_SAFFRONMART,
        MapId::SaffronPidgeyHouse => &WARPS_SAFFRONPIDGEYHOUSE,
        MapId::SaffronPokecenter => &WARPS_SAFFRONPOKECENTER,
        MapId::SeafoamIslands1F => &WARPS_SEAFOAMISLANDS1F,
        MapId::SeafoamIslandsB1F => &WARPS_SEAFOAMISLANDSB1F,
        MapId::SeafoamIslandsB2F => &WARPS_SEAFOAMISLANDSB2F,
        MapId::SeafoamIslandsB3F => &WARPS_SEAFOAMISLANDSB3F,
        MapId::SeafoamIslandsB4F => &WARPS_SEAFOAMISLANDSB4F,
        MapId::SilphCo10F => &WARPS_SILPHCO10F,
        MapId::SilphCo11F => &WARPS_SILPHCO11F,
        MapId::SilphCo1F => &WARPS_SILPHCO1F,
        MapId::SilphCo2F => &WARPS_SILPHCO2F,
        MapId::SilphCo3F => &WARPS_SILPHCO3F,
        MapId::SilphCo4F => &WARPS_SILPHCO4F,
        MapId::SilphCo5F => &WARPS_SILPHCO5F,
        MapId::SilphCo6F => &WARPS_SILPHCO6F,
        MapId::SilphCo7F => &WARPS_SILPHCO7F,
        MapId::SilphCo8F => &WARPS_SILPHCO8F,
        MapId::SilphCo9F => &WARPS_SILPHCO9F,
        MapId::SilphCoElevator => &WARPS_SILPHCOELEVATOR,
        MapId::UndergroundPathNorthSouth => &WARPS_UNDERGROUNDPATHNORTHSOUTH,
        MapId::UndergroundPathRoute5 => &WARPS_UNDERGROUNDPATHROUTE5,
        MapId::UndergroundPathRoute6 => &WARPS_UNDERGROUNDPATHROUTE6,
        MapId::UndergroundPathRoute7 => &WARPS_UNDERGROUNDPATHROUTE7,
        MapId::UndergroundPathRoute7Copy => &WARPS_UNDERGROUNDPATHROUTE7COPY,
        MapId::UndergroundPathRoute8 => &WARPS_UNDERGROUNDPATHROUTE8,
        MapId::UndergroundPathWestEast => &WARPS_UNDERGROUNDPATHWESTEAST,
        MapId::VermilionCity => &WARPS_VERMILIONCITY,
        MapId::VermilionDock => &WARPS_VERMILIONDOCK,
        MapId::VermilionGym => &WARPS_VERMILIONGYM,
        MapId::VermilionMart => &WARPS_VERMILIONMART,
        MapId::VermilionOldRodHouse => &WARPS_VERMILIONOLDRODHOUSE,
        MapId::VermilionPidgeyHouse => &WARPS_VERMILIONPIDGEYHOUSE,
        MapId::VermilionPokecenter => &WARPS_VERMILIONPOKECENTER,
        MapId::VermilionTradeHouse => &WARPS_VERMILIONTRADEHOUSE,
        MapId::VictoryRoad1F => &WARPS_VICTORYROAD1F,
        MapId::VictoryRoad2F => &WARPS_VICTORYROAD2F,
        MapId::VictoryRoad3F => &WARPS_VICTORYROAD3F,
        MapId::ViridianCity => &WARPS_VIRIDIANCITY,
        MapId::ViridianForest => &WARPS_VIRIDIANFOREST,
        MapId::ViridianForestNorthGate => &WARPS_VIRIDIANFORESTNORTHGATE,
        MapId::ViridianForestSouthGate => &WARPS_VIRIDIANFORESTSOUTHGATE,
        MapId::ViridianGym => &WARPS_VIRIDIANGYM,
        MapId::ViridianMart => &WARPS_VIRIDIANMART,
        MapId::ViridianNicknameHouse => &WARPS_VIRIDIANNICKNAMEHOUSE,
        MapId::ViridianPokecenter => &WARPS_VIRIDIANPOKECENTER,
        MapId::ViridianSchoolHouse => &WARPS_VIRIDIANSCHOOLHOUSE,
        MapId::WardensHouse => &WARPS_WARDENSHOUSE,
        _ => &EMPTY_WARPS,
    }
}
