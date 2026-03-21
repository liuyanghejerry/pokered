#[path = "sign_data_part1.rs"]
mod sign_data_part1;
#[path = "sign_data_part2.rs"]
mod sign_data_part2;
#[path = "sign_data_part3.rs"]
mod sign_data_part3;
#[path = "sign_data_part4.rs"]
mod sign_data_part4;
#[path = "sign_data_part5.rs"]
mod sign_data_part5;

use crate::maps::MapId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignEntry {
    pub x: u8,
    pub y: u8,
    pub text_id: u8,
}

pub fn get_map_signs(map: MapId) -> &'static [SignEntry] {
    match map {
        MapId::CeladonCity => &sign_data_part1::SIGNS_CELADONCITY,
        MapId::CeladonMansion1F => &sign_data_part1::SIGNS_CELADONMANSION1F,
        MapId::CeladonMansion2F => &sign_data_part1::SIGNS_CELADONMANSION2F,
        MapId::CeladonMansion3F => &sign_data_part1::SIGNS_CELADONMANSION3F,
        MapId::CeladonMansionRoof => &sign_data_part1::SIGNS_CELADONMANSIONROOF,
        MapId::CeladonMart1F => &sign_data_part1::SIGNS_CELADONMART1F,
        MapId::CeladonMart2F => &sign_data_part1::SIGNS_CELADONMART2F,
        MapId::CeladonMart3F => &sign_data_part1::SIGNS_CELADONMART3F,
        MapId::CeladonMart4F => &sign_data_part1::SIGNS_CELADONMART4F,
        MapId::CeladonMart5F => &sign_data_part1::SIGNS_CELADONMART5F,
        MapId::CeladonMartElevator => &sign_data_part1::SIGNS_CELADONMARTELEVATOR,
        MapId::CeladonMartRoof => &sign_data_part1::SIGNS_CELADONMARTROOF,
        MapId::CeruleanCity => &sign_data_part1::SIGNS_CERULEANCITY,
        MapId::CeruleanTrashedHouse => &sign_data_part1::SIGNS_CERULEANTRASHEDHOUSE,
        MapId::CinnabarIsland => &sign_data_part1::SIGNS_CINNABARISLAND,
        MapId::CinnabarLab => &sign_data_part1::SIGNS_CINNABARLAB,
        MapId::CinnabarLabMetronomeRoom => &sign_data_part1::SIGNS_CINNABARLABMETRONOMEROOM,
        MapId::CopycatsHouse2F => &sign_data_part1::SIGNS_COPYCATSHOUSE2F,
        MapId::FuchsiaCity => &sign_data_part2::SIGNS_FUCHSIACITY,
        MapId::GameCorner => &sign_data_part2::SIGNS_GAMECORNER,
        MapId::GameCornerPrizeRoom => &sign_data_part2::SIGNS_GAMECORNERPRIZEROOM,
        MapId::LavenderTown => &sign_data_part2::SIGNS_LAVENDERTOWN,
        MapId::MtMoon1F => &sign_data_part2::SIGNS_MTMOON1F,
        MapId::Museum2F => &sign_data_part2::SIGNS_MUSEUM2F,
        MapId::PalletTown => &sign_data_part2::SIGNS_PALLETTOWN,
        MapId::PewterCity => &sign_data_part2::SIGNS_PEWTERCITY,
        MapId::PokemonFanClub => &sign_data_part2::SIGNS_POKEMONFANCLUB,
        MapId::RedsHouse1F => &sign_data_part2::SIGNS_REDSHOUSE1F,
        MapId::RockTunnel1F => &sign_data_part3::SIGNS_ROCKTUNNEL1F,
        MapId::RocketHideoutElevator => &sign_data_part3::SIGNS_ROCKETHIDEOUTELEVATOR,
        MapId::Route1 => &sign_data_part3::SIGNS_ROUTE1,
        MapId::Route10 => &sign_data_part3::SIGNS_ROUTE10,
        MapId::Route11 => &sign_data_part3::SIGNS_ROUTE11,
        MapId::Route11Gate2F => &sign_data_part3::SIGNS_ROUTE11GATE2F,
        MapId::Route12 => &sign_data_part3::SIGNS_ROUTE12,
        MapId::Route12Gate2F => &sign_data_part3::SIGNS_ROUTE12GATE2F,
        MapId::Route13 => &sign_data_part3::SIGNS_ROUTE13,
        MapId::Route14 => &sign_data_part3::SIGNS_ROUTE14,
        MapId::Route15 => &sign_data_part3::SIGNS_ROUTE15,
        MapId::Route15Gate2F => &sign_data_part3::SIGNS_ROUTE15GATE2F,
        MapId::Route16 => &sign_data_part3::SIGNS_ROUTE16,
        MapId::Route16Gate2F => &sign_data_part3::SIGNS_ROUTE16GATE2F,
        MapId::Route17 => &sign_data_part3::SIGNS_ROUTE17,
        MapId::Route18 => &sign_data_part3::SIGNS_ROUTE18,
        MapId::Route18Gate2F => &sign_data_part3::SIGNS_ROUTE18GATE2F,
        MapId::Route19 => &sign_data_part3::SIGNS_ROUTE19,
        MapId::Route2 => &sign_data_part3::SIGNS_ROUTE2,
        MapId::Route20 => &sign_data_part3::SIGNS_ROUTE20,
        MapId::Route22 => &sign_data_part3::SIGNS_ROUTE22,
        MapId::Route23 => &sign_data_part3::SIGNS_ROUTE23,
        MapId::Route25 => &sign_data_part3::SIGNS_ROUTE25,
        MapId::Route3 => &sign_data_part3::SIGNS_ROUTE3,
        MapId::Route4 => &sign_data_part3::SIGNS_ROUTE4,
        MapId::Route5 => &sign_data_part3::SIGNS_ROUTE5,
        MapId::Route6 => &sign_data_part3::SIGNS_ROUTE6,
        MapId::Route7 => &sign_data_part3::SIGNS_ROUTE7,
        MapId::Route8 => &sign_data_part3::SIGNS_ROUTE8,
        MapId::Route9 => &sign_data_part4::SIGNS_ROUTE9,
        MapId::SSAnneCaptainsRoom => &sign_data_part4::SIGNS_SSANNECAPTAINSROOM,
        MapId::SafariZoneCenter => &sign_data_part4::SIGNS_SAFARIZONECENTER,
        MapId::SafariZoneEast => &sign_data_part4::SIGNS_SAFARIZONEEAST,
        MapId::SafariZoneNorth => &sign_data_part4::SIGNS_SAFARIZONENORTH,
        MapId::SafariZoneWest => &sign_data_part4::SIGNS_SAFARIZONEWEST,
        MapId::SaffronCity => &sign_data_part4::SIGNS_SAFFRONCITY,
        MapId::SeafoamIslandsB4F => &sign_data_part4::SIGNS_SEAFOAMISLANDSB4F,
        MapId::SilphCoElevator => &sign_data_part4::SIGNS_SILPHCOELEVATOR,
        MapId::VermilionCity => &sign_data_part5::SIGNS_VERMILIONCITY,
        MapId::ViridianCity => &sign_data_part5::SIGNS_VIRIDIANCITY,
        MapId::ViridianForest => &sign_data_part5::SIGNS_VIRIDIANFOREST,
        MapId::WardensHouse => &sign_data_part5::SIGNS_WARDENSHOUSE,
        _ => &[],
    }
}
