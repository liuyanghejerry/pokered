//! Map display names — 53 unique location names from data/maps/names.asm
//!
//! Not all 248 maps have unique names — indoor maps share their parent location name.

use crate::maps::MapId;

/// Map name index — references into MAP_NAME_STRINGS
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MapNameId {
    PalletTown = 0,
    ViridianCity,
    PewterCity,
    CeruleanCity,
    LavenderTown,
    VermilionCity,
    CeladonCity,
    FuchsiaCity,
    CinnabarIsland,
    IndigoPlateau,
    SaffronCity,
    Route1,
    Route2,
    Route3,
    Route4,
    Route5,
    Route6,
    Route7,
    Route8,
    Route9,
    Route10,
    Route11,
    Route12,
    Route13,
    Route14,
    Route15,
    Route16,
    Route17,
    Route18,
    SeaRoute19,
    SeaRoute20,
    SeaRoute21,
    Route22,
    Route23,
    Route24,
    Route25,
    ViridianForest,
    MountMoon,
    RockTunnel,
    SeaCottage,
    SSAnne,
    PokemonLeague,
    UndergroundPath,
    PokemonTower,
    SeafoamIslands,
    VictoryRoad,
    DiglettsCave,
    RocketHQ,
    SilphCo,
    PokemonMansion,
    SafariZone,
    CeruleanCave,
    PowerPlant,
}

/// Number of unique map names
pub const NUM_MAP_NAMES: usize = 53;

/// All 53 map display names, indexed by MapNameId
pub const MAP_NAME_STRINGS: [&str; NUM_MAP_NAMES] = [
    "PALLET TOWN",
    "VIRIDIAN CITY",
    "PEWTER CITY",
    "CERULEAN CITY",
    "LAVENDER TOWN",
    "VERMILION CITY",
    "CELADON CITY",
    "FUCHSIA CITY",
    "CINNABAR ISLAND",
    "INDIGO PLATEAU",
    "SAFFRON CITY",
    "ROUTE 1",
    "ROUTE 2",
    "ROUTE 3",
    "ROUTE 4",
    "ROUTE 5",
    "ROUTE 6",
    "ROUTE 7",
    "ROUTE 8",
    "ROUTE 9",
    "ROUTE 10",
    "ROUTE 11",
    "ROUTE 12",
    "ROUTE 13",
    "ROUTE 14",
    "ROUTE 15",
    "ROUTE 16",
    "ROUTE 17",
    "ROUTE 18",
    "SEA ROUTE 19",
    "SEA ROUTE 20",
    "SEA ROUTE 21",
    "ROUTE 22",
    "ROUTE 23",
    "ROUTE 24",
    "ROUTE 25",
    "VIRIDIAN FOREST",
    "MT.MOON",
    "ROCK TUNNEL",
    "SEA COTTAGE",
    "S.S.ANNE",
    "POKéMON LEAGUE",
    "UNDERGROUND PATH",
    "POKéMON TOWER",
    "SEAFOAM ISLANDS",
    "VICTORY ROAD",
    "DIGLETT's CAVE",
    "ROCKET HQ",
    "SILPH CO.",
    "POKéMON MANSION",
    "SAFARI ZONE",
    "CERULEAN CAVE",
    "POWER PLANT",
];

/// Get the display name for a MapNameId
pub const fn map_name_str(id: MapNameId) -> &'static str {
    MAP_NAME_STRINGS[id as usize]
}

/// Get the display name for a map. Returns the location name string.
/// Indoor maps return their parent location name.
pub fn map_name_for_map(map: MapId) -> &'static str {
    let name_id = map_to_name_id(map);
    MAP_NAME_STRINGS[name_id as usize]
}

/// Map each MapId to its MapNameId (which display name to use).
/// This is derived from the town_map_entries — indoor maps use their
/// parent location's name.
pub fn map_to_name_id(map: MapId) -> MapNameId {
    use MapId::*;
    use MapNameId as N;
    match map {
        // Cities
        PalletTown => N::PalletTown,
        ViridianCity => N::ViridianCity,
        PewterCity => N::PewterCity,
        CeruleanCity => N::CeruleanCity,
        LavenderTown => N::LavenderTown,
        VermilionCity => N::VermilionCity,
        CeladonCity => N::CeladonCity,
        FuchsiaCity => N::FuchsiaCity,
        CinnabarIsland => N::CinnabarIsland,
        IndigoPlateau => N::IndigoPlateau,
        SaffronCity => N::SaffronCity,

        UnusedMap0B => N::PalletTown, // unused

        // Routes
        Route1 => N::Route1,
        Route2 => N::Route2,
        Route3 => N::Route3,
        Route4 => N::Route4,
        Route5 => N::Route5,
        Route6 => N::Route6,
        Route7 => N::Route7,
        Route8 => N::Route8,
        Route9 => N::Route9,
        Route10 => N::Route10,
        Route11 => N::Route11,
        Route12 => N::Route12,
        Route13 => N::Route13,
        Route14 => N::Route14,
        Route15 => N::Route15,
        Route16 => N::Route16,
        Route17 => N::Route17,
        Route18 => N::Route18,
        Route19 => N::SeaRoute19,
        Route20 => N::SeaRoute20,
        Route21 => N::SeaRoute21,
        Route22 => N::Route22,
        Route23 => N::Route23,
        Route24 => N::Route24,
        Route25 => N::Route25,

        // Pallet Town indoor group
        RedsHouse1F | RedsHouse2F | BluesHouse | OaksLab => N::PalletTown,

        // Viridian City indoor group
        ViridianPokecenter
        | ViridianMart
        | ViridianSchoolHouse
        | ViridianNicknameHouse
        | ViridianGym => N::ViridianCity,

        // Route 2 indoor group
        DiglettsCaveRoute2
        | ViridianForestNorthGate
        | Route2TradeHouse
        | Route2Gate
        | ViridianForestSouthGate => N::Route2,

        // Viridian Forest
        ViridianForest => N::ViridianForest,

        // Pewter City indoor group
        Museum1F | Museum2F | PewterGym | PewterNidoranHouse | PewterMart | PewterSpeechHouse
        | PewterPokecenter => N::PewterCity,

        // Mt. Moon
        MtMoon1F | MtMoonB1F | MtMoonB2F => N::MountMoon,

        // Cerulean City indoor group
        CeruleanTrashedHouse | CeruleanTradeHouse | CeruleanPokecenter | CeruleanGym | BikeShop
        | CeruleanMart => N::CeruleanCity,

        // Route 4
        MtMoonPokecenter => N::Route4,

        // Cerulean City 2
        CeruleanTrashedHouseCopy => N::CeruleanCity,

        // Route 5 indoor group
        Route5Gate | UndergroundPathRoute5 | Daycare => N::Route5,

        // Route 6 indoor group
        Route6Gate | UndergroundPathRoute6 | UndergroundPathRoute6Copy => N::Route6,

        // Route 7 indoor group
        Route7Gate | UndergroundPathRoute7 | UndergroundPathRoute7Copy => N::Route7,

        // Route 8 indoor group
        Route8Gate | UndergroundPathRoute8 => N::Route8,

        // Rock Tunnel
        RockTunnelPokecenter | RockTunnel1F => N::RockTunnel,

        // Power Plant
        PowerPlant => N::PowerPlant,

        // Route 11 indoor group
        Route11Gate1F | DiglettsCaveRoute11 | Route11Gate2F => N::Route11,

        // Route 12
        Route12Gate1F => N::Route12,

        // Sea Cottage
        BillsHouse => N::SeaCottage,

        // Vermilion City indoor group
        VermilionPokecenter | PokemonFanClub | VermilionMart | VermilionGym
        | VermilionPidgeyHouse | VermilionDock => N::VermilionCity,

        // SS Anne
        SSAnne1F | SSAnne2F | SSAnne3F | SSAnneB1F | SSAnneBow | SSAnneKitchen
        | SSAnneCaptainsRoom | SSAnne1FRooms | SSAnne2FRooms | SSAnneB1FRooms => N::SSAnne,

        // Victory Road (first group)
        UnusedMap69 | UnusedMap6A | UnusedMap6B | VictoryRoad1F => N::VictoryRoad,

        // Pokemon League
        UnusedMap6D | UnusedMap6E | UnusedMap6F | UnusedMap70 | LancesRoom | UnusedMap72
        | UnusedMap73 | UnusedMap74 | UnusedMap75 | HallOfFame => N::PokemonLeague,

        // Underground Path
        UndergroundPathNorthSouth => N::UndergroundPath,

        // Pokemon League 2
        ChampionsRoom => N::PokemonLeague,

        // Underground Path 2
        UndergroundPathWestEast => N::UndergroundPath,

        // Celadon City indoor group
        CeladonMart1F
        | CeladonMart2F
        | CeladonMart3F
        | CeladonMart4F
        | CeladonMartRoof
        | CeladonMartElevator
        | CeladonMansion1F
        | CeladonMansion2F
        | CeladonMansion3F
        | CeladonMansionRoof
        | CeladonMansionRoofHouse
        | CeladonPokecenter
        | CeladonGym
        | GameCorner
        | CeladonMart5F
        | GameCornerPrizeRoom
        | CeladonDiner
        | CeladonChiefHouse
        | CeladonHotel => N::CeladonCity,

        // Lavender Town
        LavenderPokecenter => N::LavenderTown,

        // Pokemon Tower
        PokemonTower1F | PokemonTower2F | PokemonTower3F | PokemonTower4F | PokemonTower5F
        | PokemonTower6F | PokemonTower7F => N::PokemonTower,

        // Lavender Town 2
        MrFujisHouse | LavenderMart | LavenderCuboneHouse => N::LavenderTown,

        // Fuchsia City indoor group
        FuchsiaMart | FuchsiaBillsGrandpasHouse | FuchsiaPokecenter | WardensHouse => {
            N::FuchsiaCity
        }

        // Safari Zone gate
        SafariZoneGate => N::SafariZone,

        // Fuchsia City 2
        FuchsiaGym | FuchsiaMeetingRoom => N::FuchsiaCity,

        // Seafoam Islands
        SeafoamIslandsB1F | SeafoamIslandsB2F | SeafoamIslandsB3F | SeafoamIslandsB4F => {
            N::SeafoamIslands
        }

        // Vermilion City 2
        VermilionOldRodHouse => N::VermilionCity,

        // Fuchsia City 3
        FuchsiaGoodRodHouse => N::FuchsiaCity,

        // Pokemon Mansion
        PokemonMansion1F => N::PokemonMansion,

        // Cinnabar Island indoor group
        CinnabarGym
        | CinnabarLab
        | CinnabarLabTradeRoom
        | CinnabarLabMetronomeRoom
        | CinnabarLabFossilRoom
        | CinnabarPokecenter
        | CinnabarMart
        | CinnabarMartCopy => N::CinnabarIsland,

        // Indigo Plateau
        IndigoPlateauLobby => N::IndigoPlateau,

        // Saffron City indoor group
        CopycatsHouse1F | CopycatsHouse2F | FightingDojo | SaffronGym | SaffronPidgeyHouse
        | SaffronMart | SilphCo1F | SaffronPokecenter | MrPsychicsHouse => N::SaffronCity,

        // Route 15
        Route15Gate1F | Route15Gate2F => N::Route15,

        // Route 16
        Route16Gate1F | Route16Gate2F | Route16FlyHouse => N::Route16,

        // Route 12 (2)
        Route12SuperRodHouse => N::Route12,

        // Route 18
        Route18Gate1F | Route18Gate2F => N::Route18,

        // Seafoam Islands 2
        SeafoamIslands1F => N::SeafoamIslands,

        // Route 22
        Route22Gate => N::Route22,

        // Victory Road 2
        VictoryRoad2F => N::VictoryRoad,

        // Route 12 (3)
        Route12Gate2F => N::Route12,

        // Vermilion City 3
        VermilionTradeHouse => N::VermilionCity,

        // Diglett's Cave
        DiglettsCave => N::DiglettsCave,

        // Victory Road 3
        VictoryRoad3F => N::VictoryRoad,

        // Rocket HQ
        RocketHideoutB1F
        | RocketHideoutB2F
        | RocketHideoutB3F
        | RocketHideoutB4F
        | RocketHideoutElevator
        | UnusedMapCC
        | UnusedMapCD
        | UnusedMapCE => N::RocketHQ,

        // Silph Co
        SilphCo2F | SilphCo3F | SilphCo4F | SilphCo5F | SilphCo6F | SilphCo7F | SilphCo8F => {
            N::SilphCo
        }

        // Pokemon Mansion 2
        PokemonMansion2F | PokemonMansion3F | PokemonMansionB1F => N::PokemonMansion,

        // Safari Zone 2
        SafariZoneEast
        | SafariZoneNorth
        | SafariZoneWest
        | SafariZoneCenter
        | SafariZoneCenterRestHouse
        | SafariZoneSecretHouse
        | SafariZoneWestRestHouse
        | SafariZoneEastRestHouse
        | SafariZoneNorthRestHouse => N::SafariZone,

        // Cerulean Cave
        CeruleanCave2F | CeruleanCaveB1F | CeruleanCave1F => N::CeruleanCave,

        // Lavender Town 3
        NameRatersHouse => N::LavenderTown,

        // Cerulean City 3
        CeruleanBadgeHouse => N::CeruleanCity,

        // Rock Tunnel 2
        UnusedMapE7 | RockTunnelB1F => N::RockTunnel,

        // Silph Co 2
        SilphCo9F | SilphCo10F | SilphCo11F | SilphCoElevator => N::SilphCo,

        // Misc / unused
        UnusedMapED | UnusedMapEE => N::SilphCo,
        TradeCenter | Colosseum => N::CeladonCity,
        UnusedMapF1 | UnusedMapF2 | UnusedMapF3 | UnusedMapF4 => N::SilphCo,

        // Pokemon League 3
        LoreleisRoom | BrunosRoom | AgathasRoom => N::PokemonLeague,
    }
}
