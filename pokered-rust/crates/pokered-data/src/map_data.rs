//! Map header data — tileset, music, and connection flags for all 248 maps.
//!
//! Generated from data/maps/headers/*.asm and data/maps/songs.asm

use crate::maps::NUM_MAPS;
use crate::music::MusicId as M;
use crate::tilesets::TilesetId as T;

/// Connection direction flags (bitfield)
pub const CONN_NORTH: u8 = 0x08;
pub const CONN_SOUTH: u8 = 0x04;
pub const CONN_WEST: u8 = 0x02;
pub const CONN_EAST: u8 = 0x01;

/// Static header data for a map: tileset, music track, and connection directions.
#[derive(Debug, Clone, Copy)]
pub struct MapHeaderData {
    pub tileset: T,
    pub music: M,
    pub connection_flags: u8,
}

impl MapHeaderData {
    pub const fn has_north(&self) -> bool {
        self.connection_flags & CONN_NORTH != 0
    }
    pub const fn has_south(&self) -> bool {
        self.connection_flags & CONN_SOUTH != 0
    }
    pub const fn has_west(&self) -> bool {
        self.connection_flags & CONN_WEST != 0
    }
    pub const fn has_east(&self) -> bool {
        self.connection_flags & CONN_EAST != 0
    }
    pub const fn connection_count(&self) -> u8 {
        let f = self.connection_flags;
        ((f >> 3) & 1) + ((f >> 2) & 1) + ((f >> 1) & 1) + (f & 1)
    }
}

pub const MAP_HEADER_DATA: [MapHeaderData; NUM_MAPS] = [
    /* 0x00 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::PalletTown,
        connection_flags: 12,
    }, // PalletTown
    /* 0x01 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Cities1,
        connection_flags: 14,
    }, // ViridianCity
    /* 0x02 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Cities1,
        connection_flags: 5,
    }, // PewterCity
    /* 0x03 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Cities2,
        connection_flags: 15,
    }, // CeruleanCity
    /* 0x04 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Lavender,
        connection_flags: 14,
    }, // LavenderTown
    /* 0x05 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Vermilion,
        connection_flags: 9,
    }, // VermilionCity
    /* 0x06 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Celadon,
        connection_flags: 3,
    }, // CeladonCity
    /* 0x07 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Cities2,
        connection_flags: 7,
    }, // FuchsiaCity
    /* 0x08 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Cinnabar,
        connection_flags: 9,
    }, // CinnabarIsland
    /* 0x09 */
    MapHeaderData {
        tileset: T::Plateau,
        music: M::IndigoPlateau,
        connection_flags: 4,
    }, // IndigoPlateau
    /* 0x0A */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Cities1,
        connection_flags: 15,
    }, // SaffronCity
    /* 0x0B */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Cities1,
        connection_flags: 0,
    }, // UnusedMap0B
    /* 0x0C */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes1,
        connection_flags: 12,
    }, // Route1
    /* 0x0D */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes1,
        connection_flags: 12,
    }, // Route2
    /* 0x0E */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes3,
        connection_flags: 10,
    }, // Route3
    /* 0x0F */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes3,
        connection_flags: 5,
    }, // Route4
    /* 0x10 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes3,
        connection_flags: 12,
    }, // Route5
    /* 0x11 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes3,
        connection_flags: 12,
    }, // Route6
    /* 0x12 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes3,
        connection_flags: 3,
    }, // Route7
    /* 0x13 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes3,
        connection_flags: 3,
    }, // Route8
    /* 0x14 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes3,
        connection_flags: 3,
    }, // Route9
    /* 0x15 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes3,
        connection_flags: 6,
    }, // Route10
    /* 0x16 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes4,
        connection_flags: 3,
    }, // Route11
    /* 0x17 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes4,
        connection_flags: 14,
    }, // Route12
    /* 0x18 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes4,
        connection_flags: 10,
    }, // Route13
    /* 0x19 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes4,
        connection_flags: 3,
    }, // Route14
    /* 0x1A */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes4,
        connection_flags: 3,
    }, // Route15
    /* 0x1B */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes3,
        connection_flags: 5,
    }, // Route16
    /* 0x1C */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes3,
        connection_flags: 12,
    }, // Route17
    /* 0x1D */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes3,
        connection_flags: 9,
    }, // Route18
    /* 0x1E */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes3,
        connection_flags: 10,
    }, // Route19
    /* 0x1F */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes3,
        connection_flags: 3,
    }, // Route20
    /* 0x20 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes3,
        connection_flags: 12,
    }, // Route21
    /* 0x21 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes3,
        connection_flags: 9,
    }, // Route22
    /* 0x22 */
    MapHeaderData {
        tileset: T::Plateau,
        music: M::IndigoPlateau,
        connection_flags: 12,
    }, // Route23
    /* 0x23 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes2,
        connection_flags: 5,
    }, // Route24
    /* 0x24 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Routes2,
        connection_flags: 2,
    }, // Route25
    /* 0x25 */
    MapHeaderData {
        tileset: T::RedsHouse1,
        music: M::PalletTown,
        connection_flags: 0,
    }, // RedsHouse1F
    /* 0x26 */
    MapHeaderData {
        tileset: T::RedsHouse2,
        music: M::PalletTown,
        connection_flags: 0,
    }, // RedsHouse2F
    /* 0x27 */
    MapHeaderData {
        tileset: T::House,
        music: M::PalletTown,
        connection_flags: 0,
    }, // BluesHouse
    /* 0x28 */
    MapHeaderData {
        tileset: T::Dojo,
        music: M::OaksLab,
        connection_flags: 0,
    }, // OaksLab
    /* 0x29 */
    MapHeaderData {
        tileset: T::Pokecenter,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // ViridianPokecenter
    /* 0x2A */
    MapHeaderData {
        tileset: T::Mart,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // ViridianMart
    /* 0x2B */
    MapHeaderData {
        tileset: T::House,
        music: M::Cities1,
        connection_flags: 0,
    }, // ViridianSchoolHouse
    /* 0x2C */
    MapHeaderData {
        tileset: T::House,
        music: M::Cities1,
        connection_flags: 0,
    }, // ViridianNicknameHouse
    /* 0x2D */
    MapHeaderData {
        tileset: T::Gym,
        music: M::Gym,
        connection_flags: 0,
    }, // ViridianGym
    /* 0x2E */
    MapHeaderData {
        tileset: T::Cavern,
        music: M::Dungeon2,
        connection_flags: 0,
    }, // DiglettsCaveRoute2
    /* 0x2F */
    MapHeaderData {
        tileset: T::ForestGate,
        music: M::Cities1,
        connection_flags: 0,
    }, // ViridianForestNorthGate
    /* 0x30 */
    MapHeaderData {
        tileset: T::House,
        music: M::Cities1,
        connection_flags: 0,
    }, // Route2TradeHouse
    /* 0x31 */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Cities1,
        connection_flags: 0,
    }, // Route2Gate
    /* 0x32 */
    MapHeaderData {
        tileset: T::ForestGate,
        music: M::Cities1,
        connection_flags: 0,
    }, // ViridianForestSouthGate
    /* 0x33 */
    MapHeaderData {
        tileset: T::Forest,
        music: M::Dungeon2,
        connection_flags: 0,
    }, // ViridianForest
    /* 0x34 */
    MapHeaderData {
        tileset: T::Museum,
        music: M::Cities1,
        connection_flags: 0,
    }, // Museum1F
    /* 0x35 */
    MapHeaderData {
        tileset: T::Museum,
        music: M::Cities1,
        connection_flags: 0,
    }, // Museum2F
    /* 0x36 */
    MapHeaderData {
        tileset: T::Gym,
        music: M::Gym,
        connection_flags: 0,
    }, // PewterGym
    /* 0x37 */
    MapHeaderData {
        tileset: T::House,
        music: M::Cities1,
        connection_flags: 0,
    }, // PewterNidoranHouse
    /* 0x38 */
    MapHeaderData {
        tileset: T::Mart,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // PewterMart
    /* 0x39 */
    MapHeaderData {
        tileset: T::House,
        music: M::Cities1,
        connection_flags: 0,
    }, // PewterSpeechHouse
    /* 0x3A */
    MapHeaderData {
        tileset: T::Pokecenter,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // PewterPokecenter
    /* 0x3B */
    MapHeaderData {
        tileset: T::Cavern,
        music: M::Dungeon3,
        connection_flags: 0,
    }, // MtMoon1F
    /* 0x3C */
    MapHeaderData {
        tileset: T::Cavern,
        music: M::Dungeon3,
        connection_flags: 0,
    }, // MtMoonB1F
    /* 0x3D */
    MapHeaderData {
        tileset: T::Cavern,
        music: M::Dungeon3,
        connection_flags: 0,
    }, // MtMoonB2F
    /* 0x3E */
    MapHeaderData {
        tileset: T::House,
        music: M::Cities2,
        connection_flags: 0,
    }, // CeruleanTrashedHouse
    /* 0x3F */
    MapHeaderData {
        tileset: T::House,
        music: M::Cities2,
        connection_flags: 0,
    }, // CeruleanTradeHouse
    /* 0x40 */
    MapHeaderData {
        tileset: T::Pokecenter,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // CeruleanPokecenter
    /* 0x41 */
    MapHeaderData {
        tileset: T::Gym,
        music: M::Gym,
        connection_flags: 0,
    }, // CeruleanGym
    /* 0x42 */
    MapHeaderData {
        tileset: T::Club,
        music: M::Cities2,
        connection_flags: 0,
    }, // BikeShop
    /* 0x43 */
    MapHeaderData {
        tileset: T::Mart,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // CeruleanMart
    /* 0x44 */
    MapHeaderData {
        tileset: T::Pokecenter,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // MtMoonPokecenter
    /* 0x45 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Dungeon3,
        connection_flags: 0,
    }, // CeruleanTrashedHouseCopy
    /* 0x46 */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Cities1,
        connection_flags: 0,
    }, // Route5Gate
    /* 0x47 */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Cities1,
        connection_flags: 0,
    }, // UndergroundPathRoute5
    /* 0x48 */
    MapHeaderData {
        tileset: T::House,
        music: M::Cities1,
        connection_flags: 0,
    }, // Daycare
    /* 0x49 */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Cities1,
        connection_flags: 0,
    }, // Route6Gate
    /* 0x4A */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Cities1,
        connection_flags: 0,
    }, // UndergroundPathRoute6
    /* 0x4B */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Vermilion,
        connection_flags: 0,
    }, // UndergroundPathRoute6Copy
    /* 0x4C */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Cities1,
        connection_flags: 0,
    }, // Route7Gate
    /* 0x4D */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Cities1,
        connection_flags: 0,
    }, // UndergroundPathRoute7
    /* 0x4E */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Celadon,
        connection_flags: 0,
    }, // UndergroundPathRoute7Copy
    /* 0x4F */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Cities1,
        connection_flags: 0,
    }, // Route8Gate
    /* 0x50 */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Cities1,
        connection_flags: 0,
    }, // UndergroundPathRoute8
    /* 0x51 */
    MapHeaderData {
        tileset: T::Pokecenter,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // RockTunnelPokecenter
    /* 0x52 */
    MapHeaderData {
        tileset: T::Cavern,
        music: M::Dungeon3,
        connection_flags: 0,
    }, // RockTunnel1F
    /* 0x53 */
    MapHeaderData {
        tileset: T::Facility,
        music: M::Dungeon1,
        connection_flags: 0,
    }, // PowerPlant
    /* 0x54 */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Vermilion,
        connection_flags: 0,
    }, // Route11Gate1F
    /* 0x55 */
    MapHeaderData {
        tileset: T::Cavern,
        music: M::Dungeon2,
        connection_flags: 0,
    }, // DiglettsCaveRoute11
    /* 0x56 */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Vermilion,
        connection_flags: 0,
    }, // Route11Gate2F
    /* 0x57 */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Cities1,
        connection_flags: 0,
    }, // Route12Gate1F
    /* 0x58 */
    MapHeaderData {
        tileset: T::Interior,
        music: M::Cities2,
        connection_flags: 0,
    }, // BillsHouse
    /* 0x59 */
    MapHeaderData {
        tileset: T::Pokecenter,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // VermilionPokecenter
    /* 0x5A */
    MapHeaderData {
        tileset: T::Interior,
        music: M::Vermilion,
        connection_flags: 0,
    }, // PokemonFanClub
    /* 0x5B */
    MapHeaderData {
        tileset: T::Mart,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // VermilionMart
    /* 0x5C */
    MapHeaderData {
        tileset: T::Gym,
        music: M::Gym,
        connection_flags: 0,
    }, // VermilionGym
    /* 0x5D */
    MapHeaderData {
        tileset: T::House,
        music: M::Vermilion,
        connection_flags: 0,
    }, // VermilionPidgeyHouse
    /* 0x5E */
    MapHeaderData {
        tileset: T::ShipPort,
        music: M::SSAnne,
        connection_flags: 0,
    }, // VermilionDock
    /* 0x5F */
    MapHeaderData {
        tileset: T::Ship,
        music: M::SSAnne,
        connection_flags: 0,
    }, // SSAnne1F
    /* 0x60 */
    MapHeaderData {
        tileset: T::Ship,
        music: M::SSAnne,
        connection_flags: 0,
    }, // SSAnne2F
    /* 0x61 */
    MapHeaderData {
        tileset: T::Ship,
        music: M::SSAnne,
        connection_flags: 0,
    }, // SSAnne3F
    /* 0x62 */
    MapHeaderData {
        tileset: T::Ship,
        music: M::SSAnne,
        connection_flags: 0,
    }, // SSAnneB1F
    /* 0x63 */
    MapHeaderData {
        tileset: T::Ship,
        music: M::SSAnne,
        connection_flags: 0,
    }, // SSAnneBow
    /* 0x64 */
    MapHeaderData {
        tileset: T::Ship,
        music: M::SSAnne,
        connection_flags: 0,
    }, // SSAnneKitchen
    /* 0x65 */
    MapHeaderData {
        tileset: T::Ship,
        music: M::SSAnne,
        connection_flags: 0,
    }, // SSAnneCaptainsRoom
    /* 0x66 */
    MapHeaderData {
        tileset: T::Ship,
        music: M::SSAnne,
        connection_flags: 0,
    }, // SSAnne1FRooms
    /* 0x67 */
    MapHeaderData {
        tileset: T::Ship,
        music: M::SSAnne,
        connection_flags: 0,
    }, // SSAnne2FRooms
    /* 0x68 */
    MapHeaderData {
        tileset: T::Ship,
        music: M::SSAnne,
        connection_flags: 0,
    }, // SSAnneB1FRooms
    /* 0x69 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Dungeon2,
        connection_flags: 0,
    }, // UnusedMap69
    /* 0x6A */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Dungeon2,
        connection_flags: 0,
    }, // UnusedMap6A
    /* 0x6B */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::SSAnne,
        connection_flags: 0,
    }, // UnusedMap6B
    /* 0x6C */
    MapHeaderData {
        tileset: T::Cavern,
        music: M::Dungeon3,
        connection_flags: 0,
    }, // VictoryRoad1F
    /* 0x6D */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::PokemonTower,
        connection_flags: 0,
    }, // UnusedMap6D
    /* 0x6E */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Dungeon1,
        connection_flags: 0,
    }, // UnusedMap6E
    /* 0x6F */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::SilphCo,
        connection_flags: 0,
    }, // UnusedMap6F
    /* 0x70 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::SilphCo,
        connection_flags: 0,
    }, // UnusedMap70
    /* 0x71 */
    MapHeaderData {
        tileset: T::Dojo,
        music: M::IndigoPlateau,
        connection_flags: 0,
    }, // LancesRoom
    /* 0x72 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::SSAnne,
        connection_flags: 0,
    }, // UnusedMap72
    /* 0x73 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::SSAnne,
        connection_flags: 0,
    }, // UnusedMap73
    /* 0x74 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::SSAnne,
        connection_flags: 0,
    }, // UnusedMap74
    /* 0x75 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::SSAnne,
        connection_flags: 0,
    }, // UnusedMap75
    /* 0x76 */
    MapHeaderData {
        tileset: T::Gym,
        music: M::PalletTown,
        connection_flags: 0,
    }, // HallOfFame
    /* 0x77 */
    MapHeaderData {
        tileset: T::Underground,
        music: M::Routes1,
        connection_flags: 0,
    }, // UndergroundPathNorthSouth
    /* 0x78 */
    MapHeaderData {
        tileset: T::Gym,
        music: M::IndigoPlateau,
        connection_flags: 0,
    }, // ChampionsRoom
    /* 0x79 */
    MapHeaderData {
        tileset: T::Underground,
        music: M::Routes1,
        connection_flags: 0,
    }, // UndergroundPathWestEast
    /* 0x7A */
    MapHeaderData {
        tileset: T::Lobby,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // CeladonMart1F
    /* 0x7B */
    MapHeaderData {
        tileset: T::Lobby,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // CeladonMart2F
    /* 0x7C */
    MapHeaderData {
        tileset: T::Lobby,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // CeladonMart3F
    /* 0x7D */
    MapHeaderData {
        tileset: T::Lobby,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // CeladonMart4F
    /* 0x7E */
    MapHeaderData {
        tileset: T::Lobby,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // CeladonMartRoof
    /* 0x7F */
    MapHeaderData {
        tileset: T::Lobby,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // CeladonMartElevator
    /* 0x80 */
    MapHeaderData {
        tileset: T::Mansion,
        music: M::Celadon,
        connection_flags: 0,
    }, // CeladonMansion1F
    /* 0x81 */
    MapHeaderData {
        tileset: T::Mansion,
        music: M::Celadon,
        connection_flags: 0,
    }, // CeladonMansion2F
    /* 0x82 */
    MapHeaderData {
        tileset: T::Mansion,
        music: M::Celadon,
        connection_flags: 0,
    }, // CeladonMansion3F
    /* 0x83 */
    MapHeaderData {
        tileset: T::Mansion,
        music: M::Celadon,
        connection_flags: 0,
    }, // CeladonMansionRoof
    /* 0x84 */
    MapHeaderData {
        tileset: T::House,
        music: M::Celadon,
        connection_flags: 0,
    }, // CeladonMansionRoofHouse
    /* 0x85 */
    MapHeaderData {
        tileset: T::Pokecenter,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // CeladonPokecenter
    /* 0x86 */
    MapHeaderData {
        tileset: T::Gym,
        music: M::Gym,
        connection_flags: 0,
    }, // CeladonGym
    /* 0x87 */
    MapHeaderData {
        tileset: T::Lobby,
        music: M::GameCorner,
        connection_flags: 0,
    }, // GameCorner
    /* 0x88 */
    MapHeaderData {
        tileset: T::Lobby,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // CeladonMart5F
    /* 0x89 */
    MapHeaderData {
        tileset: T::Lobby,
        music: M::Celadon,
        connection_flags: 0,
    }, // GameCornerPrizeRoom
    /* 0x8A */
    MapHeaderData {
        tileset: T::Lobby,
        music: M::Celadon,
        connection_flags: 0,
    }, // CeladonDiner
    /* 0x8B */
    MapHeaderData {
        tileset: T::Mansion,
        music: M::Celadon,
        connection_flags: 0,
    }, // CeladonChiefHouse
    /* 0x8C */
    MapHeaderData {
        tileset: T::Pokecenter,
        music: M::Celadon,
        connection_flags: 0,
    }, // CeladonHotel
    /* 0x8D */
    MapHeaderData {
        tileset: T::Pokecenter,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // LavenderPokecenter
    /* 0x8E */
    MapHeaderData {
        tileset: T::Cemetery,
        music: M::PokemonTower,
        connection_flags: 0,
    }, // PokemonTower1F
    /* 0x8F */
    MapHeaderData {
        tileset: T::Cemetery,
        music: M::PokemonTower,
        connection_flags: 0,
    }, // PokemonTower2F
    /* 0x90 */
    MapHeaderData {
        tileset: T::Cemetery,
        music: M::PokemonTower,
        connection_flags: 0,
    }, // PokemonTower3F
    /* 0x91 */
    MapHeaderData {
        tileset: T::Cemetery,
        music: M::PokemonTower,
        connection_flags: 0,
    }, // PokemonTower4F
    /* 0x92 */
    MapHeaderData {
        tileset: T::Cemetery,
        music: M::PokemonTower,
        connection_flags: 0,
    }, // PokemonTower5F
    /* 0x93 */
    MapHeaderData {
        tileset: T::Cemetery,
        music: M::PokemonTower,
        connection_flags: 0,
    }, // PokemonTower6F
    /* 0x94 */
    MapHeaderData {
        tileset: T::Cemetery,
        music: M::PokemonTower,
        connection_flags: 0,
    }, // PokemonTower7F
    /* 0x95 */
    MapHeaderData {
        tileset: T::House,
        music: M::Lavender,
        connection_flags: 0,
    }, // MrFujisHouse
    /* 0x96 */
    MapHeaderData {
        tileset: T::Mart,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // LavenderMart
    /* 0x97 */
    MapHeaderData {
        tileset: T::House,
        music: M::Lavender,
        connection_flags: 0,
    }, // LavenderCuboneHouse
    /* 0x98 */
    MapHeaderData {
        tileset: T::Mart,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // FuchsiaMart
    /* 0x99 */
    MapHeaderData {
        tileset: T::House,
        music: M::Cities2,
        connection_flags: 0,
    }, // FuchsiaBillsGrandpasHouse
    /* 0x9A */
    MapHeaderData {
        tileset: T::Pokecenter,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // FuchsiaPokecenter
    /* 0x9B */
    MapHeaderData {
        tileset: T::Lab,
        music: M::Cities2,
        connection_flags: 0,
    }, // WardensHouse
    /* 0x9C */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Cities2,
        connection_flags: 0,
    }, // SafariZoneGate
    /* 0x9D */
    MapHeaderData {
        tileset: T::Gym,
        music: M::Gym,
        connection_flags: 0,
    }, // FuchsiaGym
    /* 0x9E */
    MapHeaderData {
        tileset: T::Lab,
        music: M::Cities2,
        connection_flags: 0,
    }, // FuchsiaMeetingRoom
    /* 0x9F */
    MapHeaderData {
        tileset: T::Cavern,
        music: M::Dungeon2,
        connection_flags: 0,
    }, // SeafoamIslandsB1F
    /* 0xA0 */
    MapHeaderData {
        tileset: T::Cavern,
        music: M::Dungeon2,
        connection_flags: 0,
    }, // SeafoamIslandsB2F
    /* 0xA1 */
    MapHeaderData {
        tileset: T::Cavern,
        music: M::Dungeon2,
        connection_flags: 0,
    }, // SeafoamIslandsB3F
    /* 0xA2 */
    MapHeaderData {
        tileset: T::Cavern,
        music: M::Dungeon2,
        connection_flags: 0,
    }, // SeafoamIslandsB4F
    /* 0xA3 */
    MapHeaderData {
        tileset: T::House,
        music: M::Cities2,
        connection_flags: 0,
    }, // VermilionOldRodHouse
    /* 0xA4 */
    MapHeaderData {
        tileset: T::Ship,
        music: M::Cities2,
        connection_flags: 0,
    }, // FuchsiaGoodRodHouse
    /* 0xA5 */
    MapHeaderData {
        tileset: T::Facility,
        music: M::CinnabarMansion,
        connection_flags: 0,
    }, // PokemonMansion1F
    /* 0xA6 */
    MapHeaderData {
        tileset: T::Facility,
        music: M::Gym,
        connection_flags: 0,
    }, // CinnabarGym
    /* 0xA7 */
    MapHeaderData {
        tileset: T::Lab,
        music: M::Cinnabar,
        connection_flags: 0,
    }, // CinnabarLab
    /* 0xA8 */
    MapHeaderData {
        tileset: T::Lab,
        music: M::Cinnabar,
        connection_flags: 0,
    }, // CinnabarLabTradeRoom
    /* 0xA9 */
    MapHeaderData {
        tileset: T::Lab,
        music: M::Cinnabar,
        connection_flags: 0,
    }, // CinnabarLabMetronomeRoom
    /* 0xAA */
    MapHeaderData {
        tileset: T::Lab,
        music: M::Cinnabar,
        connection_flags: 0,
    }, // CinnabarLabFossilRoom
    /* 0xAB */
    MapHeaderData {
        tileset: T::Pokecenter,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // CinnabarPokecenter
    /* 0xAC */
    MapHeaderData {
        tileset: T::Mart,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // CinnabarMart
    /* 0xAD */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Cinnabar,
        connection_flags: 0,
    }, // CinnabarMartCopy
    /* 0xAE */
    MapHeaderData {
        tileset: T::Mart,
        music: M::IndigoPlateau,
        connection_flags: 0,
    }, // IndigoPlateauLobby
    /* 0xAF */
    MapHeaderData {
        tileset: T::RedsHouse1,
        music: M::Cities1,
        connection_flags: 0,
    }, // CopycatsHouse1F
    /* 0xB0 */
    MapHeaderData {
        tileset: T::RedsHouse2,
        music: M::Cities1,
        connection_flags: 0,
    }, // CopycatsHouse2F
    /* 0xB1 */
    MapHeaderData {
        tileset: T::Dojo,
        music: M::Cities1,
        connection_flags: 0,
    }, // FightingDojo
    /* 0xB2 */
    MapHeaderData {
        tileset: T::Facility,
        music: M::Gym,
        connection_flags: 0,
    }, // SaffronGym
    /* 0xB3 */
    MapHeaderData {
        tileset: T::House,
        music: M::Cities1,
        connection_flags: 0,
    }, // SaffronPidgeyHouse
    /* 0xB4 */
    MapHeaderData {
        tileset: T::Mart,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // SaffronMart
    /* 0xB5 */
    MapHeaderData {
        tileset: T::Facility,
        music: M::SilphCo,
        connection_flags: 0,
    }, // SilphCo1F
    /* 0xB6 */
    MapHeaderData {
        tileset: T::Pokecenter,
        music: M::Pokecenter,
        connection_flags: 0,
    }, // SaffronPokecenter
    /* 0xB7 */
    MapHeaderData {
        tileset: T::House,
        music: M::Cities1,
        connection_flags: 0,
    }, // MrPsychicsHouse
    /* 0xB8 */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Cities1,
        connection_flags: 0,
    }, // Route15Gate1F
    /* 0xB9 */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Cities1,
        connection_flags: 0,
    }, // Route15Gate2F
    /* 0xBA */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Cities1,
        connection_flags: 0,
    }, // Route16Gate1F
    /* 0xBB */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Cities1,
        connection_flags: 0,
    }, // Route16Gate2F
    /* 0xBC */
    MapHeaderData {
        tileset: T::House,
        music: M::Celadon,
        connection_flags: 0,
    }, // Route16FlyHouse
    /* 0xBD */
    MapHeaderData {
        tileset: T::House,
        music: M::Celadon,
        connection_flags: 0,
    }, // Route12SuperRodHouse
    /* 0xBE */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Cities1,
        connection_flags: 0,
    }, // Route18Gate1F
    /* 0xBF */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Cities1,
        connection_flags: 0,
    }, // Route18Gate2F
    /* 0xC0 */
    MapHeaderData {
        tileset: T::Cavern,
        music: M::Dungeon2,
        connection_flags: 0,
    }, // SeafoamIslands1F
    /* 0xC1 */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Dungeon2,
        connection_flags: 0,
    }, // Route22Gate
    /* 0xC2 */
    MapHeaderData {
        tileset: T::Cavern,
        music: M::Dungeon3,
        connection_flags: 0,
    }, // VictoryRoad2F
    /* 0xC3 */
    MapHeaderData {
        tileset: T::Gate,
        music: M::Cities1,
        connection_flags: 0,
    }, // Route12Gate2F
    /* 0xC4 */
    MapHeaderData {
        tileset: T::House,
        music: M::Vermilion,
        connection_flags: 0,
    }, // VermilionTradeHouse
    /* 0xC5 */
    MapHeaderData {
        tileset: T::Cavern,
        music: M::Dungeon2,
        connection_flags: 0,
    }, // DiglettsCave
    /* 0xC6 */
    MapHeaderData {
        tileset: T::Cavern,
        music: M::Dungeon3,
        connection_flags: 0,
    }, // VictoryRoad3F
    /* 0xC7 */
    MapHeaderData {
        tileset: T::Facility,
        music: M::Dungeon1,
        connection_flags: 0,
    }, // RocketHideoutB1F
    /* 0xC8 */
    MapHeaderData {
        tileset: T::Facility,
        music: M::Dungeon1,
        connection_flags: 0,
    }, // RocketHideoutB2F
    /* 0xC9 */
    MapHeaderData {
        tileset: T::Facility,
        music: M::Dungeon1,
        connection_flags: 0,
    }, // RocketHideoutB3F
    /* 0xCA */
    MapHeaderData {
        tileset: T::Facility,
        music: M::Dungeon1,
        connection_flags: 0,
    }, // RocketHideoutB4F
    /* 0xCB */
    MapHeaderData {
        tileset: T::Lobby,
        music: M::Dungeon1,
        connection_flags: 0,
    }, // RocketHideoutElevator
    /* 0xCC */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Dungeon1,
        connection_flags: 0,
    }, // UnusedMapCC
    /* 0xCD */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Dungeon1,
        connection_flags: 0,
    }, // UnusedMapCD
    /* 0xCE */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Dungeon1,
        connection_flags: 0,
    }, // UnusedMapCE
    /* 0xCF */
    MapHeaderData {
        tileset: T::Facility,
        music: M::SilphCo,
        connection_flags: 0,
    }, // SilphCo2F
    /* 0xD0 */
    MapHeaderData {
        tileset: T::Facility,
        music: M::SilphCo,
        connection_flags: 0,
    }, // SilphCo3F
    /* 0xD1 */
    MapHeaderData {
        tileset: T::Facility,
        music: M::SilphCo,
        connection_flags: 0,
    }, // SilphCo4F
    /* 0xD2 */
    MapHeaderData {
        tileset: T::Facility,
        music: M::SilphCo,
        connection_flags: 0,
    }, // SilphCo5F
    /* 0xD3 */
    MapHeaderData {
        tileset: T::Facility,
        music: M::SilphCo,
        connection_flags: 0,
    }, // SilphCo6F
    /* 0xD4 */
    MapHeaderData {
        tileset: T::Facility,
        music: M::SilphCo,
        connection_flags: 0,
    }, // SilphCo7F
    /* 0xD5 */
    MapHeaderData {
        tileset: T::Facility,
        music: M::SilphCo,
        connection_flags: 0,
    }, // SilphCo8F
    /* 0xD6 */
    MapHeaderData {
        tileset: T::Facility,
        music: M::CinnabarMansion,
        connection_flags: 0,
    }, // PokemonMansion2F
    /* 0xD7 */
    MapHeaderData {
        tileset: T::Facility,
        music: M::CinnabarMansion,
        connection_flags: 0,
    }, // PokemonMansion3F
    /* 0xD8 */
    MapHeaderData {
        tileset: T::Facility,
        music: M::CinnabarMansion,
        connection_flags: 0,
    }, // PokemonMansionB1F
    /* 0xD9 */
    MapHeaderData {
        tileset: T::Forest,
        music: M::SafariZone,
        connection_flags: 0,
    }, // SafariZoneEast
    /* 0xDA */
    MapHeaderData {
        tileset: T::Forest,
        music: M::SafariZone,
        connection_flags: 0,
    }, // SafariZoneNorth
    /* 0xDB */
    MapHeaderData {
        tileset: T::Forest,
        music: M::SafariZone,
        connection_flags: 0,
    }, // SafariZoneWest
    /* 0xDC */
    MapHeaderData {
        tileset: T::Forest,
        music: M::SafariZone,
        connection_flags: 0,
    }, // SafariZoneCenter
    /* 0xDD */
    MapHeaderData {
        tileset: T::Gate,
        music: M::SafariZone,
        connection_flags: 0,
    }, // SafariZoneCenterRestHouse
    /* 0xDE */
    MapHeaderData {
        tileset: T::Lab,
        music: M::SafariZone,
        connection_flags: 0,
    }, // SafariZoneSecretHouse
    /* 0xDF */
    MapHeaderData {
        tileset: T::Gate,
        music: M::SafariZone,
        connection_flags: 0,
    }, // SafariZoneWestRestHouse
    /* 0xE0 */
    MapHeaderData {
        tileset: T::Gate,
        music: M::SafariZone,
        connection_flags: 0,
    }, // SafariZoneEastRestHouse
    /* 0xE1 */
    MapHeaderData {
        tileset: T::Gate,
        music: M::SafariZone,
        connection_flags: 0,
    }, // SafariZoneNorthRestHouse
    /* 0xE2 */
    MapHeaderData {
        tileset: T::Cavern,
        music: M::Dungeon1,
        connection_flags: 0,
    }, // CeruleanCave2F
    /* 0xE3 */
    MapHeaderData {
        tileset: T::Cavern,
        music: M::Dungeon1,
        connection_flags: 0,
    }, // CeruleanCaveB1F
    /* 0xE4 */
    MapHeaderData {
        tileset: T::Cavern,
        music: M::Dungeon1,
        connection_flags: 0,
    }, // CeruleanCave1F
    /* 0xE5 */
    MapHeaderData {
        tileset: T::House,
        music: M::Cities2,
        connection_flags: 0,
    }, // NameRatersHouse
    /* 0xE6 */
    MapHeaderData {
        tileset: T::Ship,
        music: M::Cities1,
        connection_flags: 0,
    }, // CeruleanBadgeHouse
    /* 0xE7 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::Cinnabar,
        connection_flags: 0,
    }, // UnusedMapE7
    /* 0xE8 */
    MapHeaderData {
        tileset: T::Cavern,
        music: M::Dungeon3,
        connection_flags: 0,
    }, // RockTunnelB1F
    /* 0xE9 */
    MapHeaderData {
        tileset: T::Facility,
        music: M::SilphCo,
        connection_flags: 0,
    }, // SilphCo9F
    /* 0xEA */
    MapHeaderData {
        tileset: T::Facility,
        music: M::SilphCo,
        connection_flags: 0,
    }, // SilphCo10F
    /* 0xEB */
    MapHeaderData {
        tileset: T::Interior,
        music: M::SilphCo,
        connection_flags: 0,
    }, // SilphCo11F
    /* 0xEC */
    MapHeaderData {
        tileset: T::Lobby,
        music: M::SilphCo,
        connection_flags: 0,
    }, // SilphCoElevator
    /* 0xED */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::SilphCo,
        connection_flags: 0,
    }, // UnusedMapED
    /* 0xEE */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::SilphCo,
        connection_flags: 0,
    }, // UnusedMapEE
    /* 0xEF */
    MapHeaderData {
        tileset: T::Club,
        music: M::Celadon,
        connection_flags: 0,
    }, // TradeCenter
    /* 0xF0 */
    MapHeaderData {
        tileset: T::Club,
        music: M::Celadon,
        connection_flags: 0,
    }, // Colosseum
    /* 0xF1 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::SilphCo,
        connection_flags: 0,
    }, // UnusedMapF1
    /* 0xF2 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::SilphCo,
        connection_flags: 0,
    }, // UnusedMapF2
    /* 0xF3 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::SilphCo,
        connection_flags: 0,
    }, // UnusedMapF3
    /* 0xF4 */
    MapHeaderData {
        tileset: T::Overworld,
        music: M::SilphCo,
        connection_flags: 0,
    }, // UnusedMapF4
    /* 0xF5 */
    MapHeaderData {
        tileset: T::Gym,
        music: M::Gym,
        connection_flags: 0,
    }, // LoreleisRoom
    /* 0xF6 */
    MapHeaderData {
        tileset: T::Gym,
        music: M::Dungeon1,
        connection_flags: 0,
    }, // BrunosRoom
    /* 0xF7 */
    MapHeaderData {
        tileset: T::Cemetery,
        music: M::PokemonTower,
        connection_flags: 0,
    }, // AgathasRoom
];
