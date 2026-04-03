use std::collections::HashMap;
use std::sync::OnceLock;

use crate::map_json::MapJson;
use crate::maps::{MapId, NUM_MAPS};

struct MapDataStore {
    maps: HashMap<String, MapJson>,
    blocks: HashMap<String, Vec<u8>>,
}

static MAP_DATA: OnceLock<MapDataStore> = OnceLock::new();

fn get_store() -> &'static MapDataStore {
    MAP_DATA.get_or_init(|| init_map_data())
}

fn build_name_to_id() -> HashMap<String, MapId> {
    let mut map = HashMap::new();
    for i in 0..NUM_MAPS {
        if let Some(id) = MapId::from_u8(i as u8) {
            map.insert(format!("{:?}", id), id);
        }
    }
    map
}

pub fn get_map_json(map_id: MapId) -> Option<&'static MapJson> {
    let name = format!("{:?}", map_id);
    get_store().maps.get(&name)
}

pub fn get_block_data(map_id: MapId) -> &'static [u8] {
    let name = format!("{:?}", map_id);
    get_store()
        .blocks
        .get(&name)
        .map(|v| v.as_slice())
        .unwrap_or(&[])
}

pub fn all_map_names() -> Vec<&'static str> {
    get_store().maps.keys().map(|s| s.as_str()).collect()
}

pub fn name_to_map_id() -> &'static HashMap<String, MapId> {
    static NAME_MAP: OnceLock<HashMap<String, MapId>> = OnceLock::new();
    NAME_MAP.get_or_init(build_name_to_id)
}

pub fn resolve_map_id(name: &str) -> Option<MapId> {
    name_to_map_id().get(name).copied()
}

// ── Embedded mode ──────────────────────────────────────────────────────────

#[cfg(feature = "embedded-map-data")]
fn embedded_json_sources() -> &'static [(&'static str, &'static str)] {
    &[
        ("AgathasRoom", include_str!("../maps/AgathasRoom/map.json")),
        ("BikeShop", include_str!("../maps/BikeShop/map.json")),
        ("BillsHouse", include_str!("../maps/BillsHouse/map.json")),
        ("BluesHouse", include_str!("../maps/BluesHouse/map.json")),
        ("BrunosRoom", include_str!("../maps/BrunosRoom/map.json")),
        (
            "CeladonChiefHouse",
            include_str!("../maps/CeladonChiefHouse/map.json"),
        ),
        ("CeladonCity", include_str!("../maps/CeladonCity/map.json")),
        (
            "CeladonDiner",
            include_str!("../maps/CeladonDiner/map.json"),
        ),
        ("CeladonGym", include_str!("../maps/CeladonGym/map.json")),
        (
            "CeladonHotel",
            include_str!("../maps/CeladonHotel/map.json"),
        ),
        (
            "CeladonMansion1F",
            include_str!("../maps/CeladonMansion1F/map.json"),
        ),
        (
            "CeladonMansion2F",
            include_str!("../maps/CeladonMansion2F/map.json"),
        ),
        (
            "CeladonMansion3F",
            include_str!("../maps/CeladonMansion3F/map.json"),
        ),
        (
            "CeladonMansionRoof",
            include_str!("../maps/CeladonMansionRoof/map.json"),
        ),
        (
            "CeladonMansionRoofHouse",
            include_str!("../maps/CeladonMansionRoofHouse/map.json"),
        ),
        (
            "CeladonMart1F",
            include_str!("../maps/CeladonMart1F/map.json"),
        ),
        (
            "CeladonMart2F",
            include_str!("../maps/CeladonMart2F/map.json"),
        ),
        (
            "CeladonMart3F",
            include_str!("../maps/CeladonMart3F/map.json"),
        ),
        (
            "CeladonMart4F",
            include_str!("../maps/CeladonMart4F/map.json"),
        ),
        (
            "CeladonMart5F",
            include_str!("../maps/CeladonMart5F/map.json"),
        ),
        (
            "CeladonMartElevator",
            include_str!("../maps/CeladonMartElevator/map.json"),
        ),
        (
            "CeladonMartRoof",
            include_str!("../maps/CeladonMartRoof/map.json"),
        ),
        (
            "CeladonPokecenter",
            include_str!("../maps/CeladonPokecenter/map.json"),
        ),
        (
            "CeruleanBadgeHouse",
            include_str!("../maps/CeruleanBadgeHouse/map.json"),
        ),
        (
            "CeruleanCave1F",
            include_str!("../maps/CeruleanCave1F/map.json"),
        ),
        (
            "CeruleanCave2F",
            include_str!("../maps/CeruleanCave2F/map.json"),
        ),
        (
            "CeruleanCaveB1F",
            include_str!("../maps/CeruleanCaveB1F/map.json"),
        ),
        (
            "CeruleanCity",
            include_str!("../maps/CeruleanCity/map.json"),
        ),
        ("CeruleanGym", include_str!("../maps/CeruleanGym/map.json")),
        (
            "CeruleanMart",
            include_str!("../maps/CeruleanMart/map.json"),
        ),
        (
            "CeruleanPokecenter",
            include_str!("../maps/CeruleanPokecenter/map.json"),
        ),
        (
            "CeruleanTradeHouse",
            include_str!("../maps/CeruleanTradeHouse/map.json"),
        ),
        (
            "CeruleanTrashedHouse",
            include_str!("../maps/CeruleanTrashedHouse/map.json"),
        ),
        (
            "CeruleanTrashedHouseCopy",
            include_str!("../maps/CeruleanTrashedHouseCopy/map.json"),
        ),
        (
            "ChampionsRoom",
            include_str!("../maps/ChampionsRoom/map.json"),
        ),
        ("CinnabarGym", include_str!("../maps/CinnabarGym/map.json")),
        (
            "CinnabarIsland",
            include_str!("../maps/CinnabarIsland/map.json"),
        ),
        ("CinnabarLab", include_str!("../maps/CinnabarLab/map.json")),
        (
            "CinnabarLabFossilRoom",
            include_str!("../maps/CinnabarLabFossilRoom/map.json"),
        ),
        (
            "CinnabarLabMetronomeRoom",
            include_str!("../maps/CinnabarLabMetronomeRoom/map.json"),
        ),
        (
            "CinnabarLabTradeRoom",
            include_str!("../maps/CinnabarLabTradeRoom/map.json"),
        ),
        (
            "CinnabarMart",
            include_str!("../maps/CinnabarMart/map.json"),
        ),
        (
            "CinnabarMartCopy",
            include_str!("../maps/CinnabarMartCopy/map.json"),
        ),
        (
            "CinnabarPokecenter",
            include_str!("../maps/CinnabarPokecenter/map.json"),
        ),
        ("Colosseum", include_str!("../maps/Colosseum/map.json")),
        (
            "CopycatsHouse1F",
            include_str!("../maps/CopycatsHouse1F/map.json"),
        ),
        (
            "CopycatsHouse2F",
            include_str!("../maps/CopycatsHouse2F/map.json"),
        ),
        ("Daycare", include_str!("../maps/Daycare/map.json")),
        (
            "DiglettsCave",
            include_str!("../maps/DiglettsCave/map.json"),
        ),
        (
            "DiglettsCaveRoute11",
            include_str!("../maps/DiglettsCaveRoute11/map.json"),
        ),
        (
            "DiglettsCaveRoute2",
            include_str!("../maps/DiglettsCaveRoute2/map.json"),
        ),
        (
            "FightingDojo",
            include_str!("../maps/FightingDojo/map.json"),
        ),
        (
            "FuchsiaBillsGrandpasHouse",
            include_str!("../maps/FuchsiaBillsGrandpasHouse/map.json"),
        ),
        ("FuchsiaCity", include_str!("../maps/FuchsiaCity/map.json")),
        (
            "FuchsiaGoodRodHouse",
            include_str!("../maps/FuchsiaGoodRodHouse/map.json"),
        ),
        ("FuchsiaGym", include_str!("../maps/FuchsiaGym/map.json")),
        ("FuchsiaMart", include_str!("../maps/FuchsiaMart/map.json")),
        (
            "FuchsiaMeetingRoom",
            include_str!("../maps/FuchsiaMeetingRoom/map.json"),
        ),
        (
            "FuchsiaPokecenter",
            include_str!("../maps/FuchsiaPokecenter/map.json"),
        ),
        ("GameCorner", include_str!("../maps/GameCorner/map.json")),
        (
            "GameCornerPrizeRoom",
            include_str!("../maps/GameCornerPrizeRoom/map.json"),
        ),
        ("HallOfFame", include_str!("../maps/HallOfFame/map.json")),
        (
            "IndigoPlateau",
            include_str!("../maps/IndigoPlateau/map.json"),
        ),
        (
            "IndigoPlateauLobby",
            include_str!("../maps/IndigoPlateauLobby/map.json"),
        ),
        ("LancesRoom", include_str!("../maps/LancesRoom/map.json")),
        (
            "LavenderCuboneHouse",
            include_str!("../maps/LavenderCuboneHouse/map.json"),
        ),
        (
            "LavenderMart",
            include_str!("../maps/LavenderMart/map.json"),
        ),
        (
            "LavenderPokecenter",
            include_str!("../maps/LavenderPokecenter/map.json"),
        ),
        (
            "LavenderTown",
            include_str!("../maps/LavenderTown/map.json"),
        ),
        (
            "LoreleisRoom",
            include_str!("../maps/LoreleisRoom/map.json"),
        ),
        (
            "MrFujisHouse",
            include_str!("../maps/MrFujisHouse/map.json"),
        ),
        (
            "MrPsychicsHouse",
            include_str!("../maps/MrPsychicsHouse/map.json"),
        ),
        ("MtMoon1F", include_str!("../maps/MtMoon1F/map.json")),
        ("MtMoonB1F", include_str!("../maps/MtMoonB1F/map.json")),
        ("MtMoonB2F", include_str!("../maps/MtMoonB2F/map.json")),
        (
            "MtMoonPokecenter",
            include_str!("../maps/MtMoonPokecenter/map.json"),
        ),
        ("Museum1F", include_str!("../maps/Museum1F/map.json")),
        ("Museum2F", include_str!("../maps/Museum2F/map.json")),
        (
            "NameRatersHouse",
            include_str!("../maps/NameRatersHouse/map.json"),
        ),
        ("OaksLab", include_str!("../maps/OaksLab/map.json")),
        ("PalletTown", include_str!("../maps/PalletTown/map.json")),
        ("PewterCity", include_str!("../maps/PewterCity/map.json")),
        ("PewterGym", include_str!("../maps/PewterGym/map.json")),
        ("PewterMart", include_str!("../maps/PewterMart/map.json")),
        (
            "PewterNidoranHouse",
            include_str!("../maps/PewterNidoranHouse/map.json"),
        ),
        (
            "PewterPokecenter",
            include_str!("../maps/PewterPokecenter/map.json"),
        ),
        (
            "PewterSpeechHouse",
            include_str!("../maps/PewterSpeechHouse/map.json"),
        ),
        (
            "PokemonFanClub",
            include_str!("../maps/PokemonFanClub/map.json"),
        ),
        (
            "PokemonMansion1F",
            include_str!("../maps/PokemonMansion1F/map.json"),
        ),
        (
            "PokemonMansion2F",
            include_str!("../maps/PokemonMansion2F/map.json"),
        ),
        (
            "PokemonMansion3F",
            include_str!("../maps/PokemonMansion3F/map.json"),
        ),
        (
            "PokemonMansionB1F",
            include_str!("../maps/PokemonMansionB1F/map.json"),
        ),
        (
            "PokemonTower1F",
            include_str!("../maps/PokemonTower1F/map.json"),
        ),
        (
            "PokemonTower2F",
            include_str!("../maps/PokemonTower2F/map.json"),
        ),
        (
            "PokemonTower3F",
            include_str!("../maps/PokemonTower3F/map.json"),
        ),
        (
            "PokemonTower4F",
            include_str!("../maps/PokemonTower4F/map.json"),
        ),
        (
            "PokemonTower5F",
            include_str!("../maps/PokemonTower5F/map.json"),
        ),
        (
            "PokemonTower6F",
            include_str!("../maps/PokemonTower6F/map.json"),
        ),
        (
            "PokemonTower7F",
            include_str!("../maps/PokemonTower7F/map.json"),
        ),
        ("PowerPlant", include_str!("../maps/PowerPlant/map.json")),
        ("RedsHouse1F", include_str!("../maps/RedsHouse1F/map.json")),
        ("RedsHouse2F", include_str!("../maps/RedsHouse2F/map.json")),
        (
            "RockTunnel1F",
            include_str!("../maps/RockTunnel1F/map.json"),
        ),
        (
            "RockTunnelB1F",
            include_str!("../maps/RockTunnelB1F/map.json"),
        ),
        (
            "RockTunnelPokecenter",
            include_str!("../maps/RockTunnelPokecenter/map.json"),
        ),
        (
            "RocketHideoutB1F",
            include_str!("../maps/RocketHideoutB1F/map.json"),
        ),
        (
            "RocketHideoutB2F",
            include_str!("../maps/RocketHideoutB2F/map.json"),
        ),
        (
            "RocketHideoutB3F",
            include_str!("../maps/RocketHideoutB3F/map.json"),
        ),
        (
            "RocketHideoutB4F",
            include_str!("../maps/RocketHideoutB4F/map.json"),
        ),
        (
            "RocketHideoutElevator",
            include_str!("../maps/RocketHideoutElevator/map.json"),
        ),
        ("Route1", include_str!("../maps/Route1/map.json")),
        ("Route10", include_str!("../maps/Route10/map.json")),
        ("Route11", include_str!("../maps/Route11/map.json")),
        (
            "Route11Gate1F",
            include_str!("../maps/Route11Gate1F/map.json"),
        ),
        (
            "Route11Gate2F",
            include_str!("../maps/Route11Gate2F/map.json"),
        ),
        ("Route12", include_str!("../maps/Route12/map.json")),
        (
            "Route12Gate1F",
            include_str!("../maps/Route12Gate1F/map.json"),
        ),
        (
            "Route12Gate2F",
            include_str!("../maps/Route12Gate2F/map.json"),
        ),
        (
            "Route12SuperRodHouse",
            include_str!("../maps/Route12SuperRodHouse/map.json"),
        ),
        ("Route13", include_str!("../maps/Route13/map.json")),
        ("Route14", include_str!("../maps/Route14/map.json")),
        ("Route15", include_str!("../maps/Route15/map.json")),
        (
            "Route15Gate1F",
            include_str!("../maps/Route15Gate1F/map.json"),
        ),
        (
            "Route15Gate2F",
            include_str!("../maps/Route15Gate2F/map.json"),
        ),
        ("Route16", include_str!("../maps/Route16/map.json")),
        (
            "Route16FlyHouse",
            include_str!("../maps/Route16FlyHouse/map.json"),
        ),
        (
            "Route16Gate1F",
            include_str!("../maps/Route16Gate1F/map.json"),
        ),
        (
            "Route16Gate2F",
            include_str!("../maps/Route16Gate2F/map.json"),
        ),
        ("Route17", include_str!("../maps/Route17/map.json")),
        ("Route18", include_str!("../maps/Route18/map.json")),
        (
            "Route18Gate1F",
            include_str!("../maps/Route18Gate1F/map.json"),
        ),
        (
            "Route18Gate2F",
            include_str!("../maps/Route18Gate2F/map.json"),
        ),
        ("Route19", include_str!("../maps/Route19/map.json")),
        ("Route2", include_str!("../maps/Route2/map.json")),
        ("Route20", include_str!("../maps/Route20/map.json")),
        ("Route21", include_str!("../maps/Route21/map.json")),
        ("Route22", include_str!("../maps/Route22/map.json")),
        ("Route22Gate", include_str!("../maps/Route22Gate/map.json")),
        ("Route23", include_str!("../maps/Route23/map.json")),
        ("Route24", include_str!("../maps/Route24/map.json")),
        ("Route25", include_str!("../maps/Route25/map.json")),
        ("Route2Gate", include_str!("../maps/Route2Gate/map.json")),
        (
            "Route2TradeHouse",
            include_str!("../maps/Route2TradeHouse/map.json"),
        ),
        ("Route3", include_str!("../maps/Route3/map.json")),
        ("Route4", include_str!("../maps/Route4/map.json")),
        ("Route5", include_str!("../maps/Route5/map.json")),
        ("Route5Gate", include_str!("../maps/Route5Gate/map.json")),
        ("Route6", include_str!("../maps/Route6/map.json")),
        ("Route6Gate", include_str!("../maps/Route6Gate/map.json")),
        ("Route7", include_str!("../maps/Route7/map.json")),
        ("Route7Gate", include_str!("../maps/Route7Gate/map.json")),
        ("Route8", include_str!("../maps/Route8/map.json")),
        ("Route8Gate", include_str!("../maps/Route8Gate/map.json")),
        ("Route9", include_str!("../maps/Route9/map.json")),
        ("SSAnne1F", include_str!("../maps/SSAnne1F/map.json")),
        (
            "SSAnne1FRooms",
            include_str!("../maps/SSAnne1FRooms/map.json"),
        ),
        ("SSAnne2F", include_str!("../maps/SSAnne2F/map.json")),
        (
            "SSAnne2FRooms",
            include_str!("../maps/SSAnne2FRooms/map.json"),
        ),
        ("SSAnne3F", include_str!("../maps/SSAnne3F/map.json")),
        ("SSAnneB1F", include_str!("../maps/SSAnneB1F/map.json")),
        (
            "SSAnneB1FRooms",
            include_str!("../maps/SSAnneB1FRooms/map.json"),
        ),
        ("SSAnneBow", include_str!("../maps/SSAnneBow/map.json")),
        (
            "SSAnneCaptainsRoom",
            include_str!("../maps/SSAnneCaptainsRoom/map.json"),
        ),
        (
            "SSAnneKitchen",
            include_str!("../maps/SSAnneKitchen/map.json"),
        ),
        (
            "SafariZoneCenter",
            include_str!("../maps/SafariZoneCenter/map.json"),
        ),
        (
            "SafariZoneCenterRestHouse",
            include_str!("../maps/SafariZoneCenterRestHouse/map.json"),
        ),
        (
            "SafariZoneEast",
            include_str!("../maps/SafariZoneEast/map.json"),
        ),
        (
            "SafariZoneEastRestHouse",
            include_str!("../maps/SafariZoneEastRestHouse/map.json"),
        ),
        (
            "SafariZoneGate",
            include_str!("../maps/SafariZoneGate/map.json"),
        ),
        (
            "SafariZoneNorth",
            include_str!("../maps/SafariZoneNorth/map.json"),
        ),
        (
            "SafariZoneNorthRestHouse",
            include_str!("../maps/SafariZoneNorthRestHouse/map.json"),
        ),
        (
            "SafariZoneSecretHouse",
            include_str!("../maps/SafariZoneSecretHouse/map.json"),
        ),
        (
            "SafariZoneWest",
            include_str!("../maps/SafariZoneWest/map.json"),
        ),
        (
            "SafariZoneWestRestHouse",
            include_str!("../maps/SafariZoneWestRestHouse/map.json"),
        ),
        ("SaffronCity", include_str!("../maps/SaffronCity/map.json")),
        ("SaffronGym", include_str!("../maps/SaffronGym/map.json")),
        ("SaffronMart", include_str!("../maps/SaffronMart/map.json")),
        (
            "SaffronPidgeyHouse",
            include_str!("../maps/SaffronPidgeyHouse/map.json"),
        ),
        (
            "SaffronPokecenter",
            include_str!("../maps/SaffronPokecenter/map.json"),
        ),
        (
            "SeafoamIslands1F",
            include_str!("../maps/SeafoamIslands1F/map.json"),
        ),
        (
            "SeafoamIslandsB1F",
            include_str!("../maps/SeafoamIslandsB1F/map.json"),
        ),
        (
            "SeafoamIslandsB2F",
            include_str!("../maps/SeafoamIslandsB2F/map.json"),
        ),
        (
            "SeafoamIslandsB3F",
            include_str!("../maps/SeafoamIslandsB3F/map.json"),
        ),
        (
            "SeafoamIslandsB4F",
            include_str!("../maps/SeafoamIslandsB4F/map.json"),
        ),
        ("SilphCo10F", include_str!("../maps/SilphCo10F/map.json")),
        ("SilphCo11F", include_str!("../maps/SilphCo11F/map.json")),
        ("SilphCo1F", include_str!("../maps/SilphCo1F/map.json")),
        ("SilphCo2F", include_str!("../maps/SilphCo2F/map.json")),
        ("SilphCo3F", include_str!("../maps/SilphCo3F/map.json")),
        ("SilphCo4F", include_str!("../maps/SilphCo4F/map.json")),
        ("SilphCo5F", include_str!("../maps/SilphCo5F/map.json")),
        ("SilphCo6F", include_str!("../maps/SilphCo6F/map.json")),
        ("SilphCo7F", include_str!("../maps/SilphCo7F/map.json")),
        ("SilphCo8F", include_str!("../maps/SilphCo8F/map.json")),
        ("SilphCo9F", include_str!("../maps/SilphCo9F/map.json")),
        (
            "SilphCoElevator",
            include_str!("../maps/SilphCoElevator/map.json"),
        ),
        ("TradeCenter", include_str!("../maps/TradeCenter/map.json")),
        (
            "UndergroundPathNorthSouth",
            include_str!("../maps/UndergroundPathNorthSouth/map.json"),
        ),
        (
            "UndergroundPathRoute5",
            include_str!("../maps/UndergroundPathRoute5/map.json"),
        ),
        (
            "UndergroundPathRoute6",
            include_str!("../maps/UndergroundPathRoute6/map.json"),
        ),
        (
            "UndergroundPathRoute6Copy",
            include_str!("../maps/UndergroundPathRoute6Copy/map.json"),
        ),
        (
            "UndergroundPathRoute7",
            include_str!("../maps/UndergroundPathRoute7/map.json"),
        ),
        (
            "UndergroundPathRoute7Copy",
            include_str!("../maps/UndergroundPathRoute7Copy/map.json"),
        ),
        (
            "UndergroundPathRoute8",
            include_str!("../maps/UndergroundPathRoute8/map.json"),
        ),
        (
            "UndergroundPathWestEast",
            include_str!("../maps/UndergroundPathWestEast/map.json"),
        ),
        ("UnusedMap0B", include_str!("../maps/UnusedMap0B/map.json")),
        ("UnusedMap69", include_str!("../maps/UnusedMap69/map.json")),
        ("UnusedMap6A", include_str!("../maps/UnusedMap6A/map.json")),
        ("UnusedMap6B", include_str!("../maps/UnusedMap6B/map.json")),
        ("UnusedMap6D", include_str!("../maps/UnusedMap6D/map.json")),
        ("UnusedMap6E", include_str!("../maps/UnusedMap6E/map.json")),
        ("UnusedMap6F", include_str!("../maps/UnusedMap6F/map.json")),
        ("UnusedMap70", include_str!("../maps/UnusedMap70/map.json")),
        ("UnusedMap72", include_str!("../maps/UnusedMap72/map.json")),
        ("UnusedMap73", include_str!("../maps/UnusedMap73/map.json")),
        ("UnusedMap74", include_str!("../maps/UnusedMap74/map.json")),
        ("UnusedMap75", include_str!("../maps/UnusedMap75/map.json")),
        ("UnusedMapCC", include_str!("../maps/UnusedMapCC/map.json")),
        ("UnusedMapCD", include_str!("../maps/UnusedMapCD/map.json")),
        ("UnusedMapCE", include_str!("../maps/UnusedMapCE/map.json")),
        ("UnusedMapE7", include_str!("../maps/UnusedMapE7/map.json")),
        ("UnusedMapED", include_str!("../maps/UnusedMapED/map.json")),
        ("UnusedMapEE", include_str!("../maps/UnusedMapEE/map.json")),
        ("UnusedMapF1", include_str!("../maps/UnusedMapF1/map.json")),
        ("UnusedMapF2", include_str!("../maps/UnusedMapF2/map.json")),
        ("UnusedMapF3", include_str!("../maps/UnusedMapF3/map.json")),
        ("UnusedMapF4", include_str!("../maps/UnusedMapF4/map.json")),
        (
            "VermilionCity",
            include_str!("../maps/VermilionCity/map.json"),
        ),
        (
            "VermilionDock",
            include_str!("../maps/VermilionDock/map.json"),
        ),
        (
            "VermilionGym",
            include_str!("../maps/VermilionGym/map.json"),
        ),
        (
            "VermilionMart",
            include_str!("../maps/VermilionMart/map.json"),
        ),
        (
            "VermilionOldRodHouse",
            include_str!("../maps/VermilionOldRodHouse/map.json"),
        ),
        (
            "VermilionPidgeyHouse",
            include_str!("../maps/VermilionPidgeyHouse/map.json"),
        ),
        (
            "VermilionPokecenter",
            include_str!("../maps/VermilionPokecenter/map.json"),
        ),
        (
            "VermilionTradeHouse",
            include_str!("../maps/VermilionTradeHouse/map.json"),
        ),
        (
            "VictoryRoad1F",
            include_str!("../maps/VictoryRoad1F/map.json"),
        ),
        (
            "VictoryRoad2F",
            include_str!("../maps/VictoryRoad2F/map.json"),
        ),
        (
            "VictoryRoad3F",
            include_str!("../maps/VictoryRoad3F/map.json"),
        ),
        (
            "ViridianCity",
            include_str!("../maps/ViridianCity/map.json"),
        ),
        (
            "ViridianForest",
            include_str!("../maps/ViridianForest/map.json"),
        ),
        (
            "ViridianForestNorthGate",
            include_str!("../maps/ViridianForestNorthGate/map.json"),
        ),
        (
            "ViridianForestSouthGate",
            include_str!("../maps/ViridianForestSouthGate/map.json"),
        ),
        ("ViridianGym", include_str!("../maps/ViridianGym/map.json")),
        (
            "ViridianMart",
            include_str!("../maps/ViridianMart/map.json"),
        ),
        (
            "ViridianNicknameHouse",
            include_str!("../maps/ViridianNicknameHouse/map.json"),
        ),
        (
            "ViridianPokecenter",
            include_str!("../maps/ViridianPokecenter/map.json"),
        ),
        (
            "ViridianSchoolHouse",
            include_str!("../maps/ViridianSchoolHouse/map.json"),
        ),
        (
            "WardensHouse",
            include_str!("../maps/WardensHouse/map.json"),
        ),
    ]
}

#[cfg(feature = "embedded-map-data")]
fn embedded_blk_sources() -> &'static [(&'static str, &'static [u8])] {
    &[
        ("AgathasRoom", include_bytes!("../maps/AgathasRoom/map.blk")),
        ("BikeShop", include_bytes!("../maps/BikeShop/map.blk")),
        ("BillsHouse", include_bytes!("../maps/BillsHouse/map.blk")),
        ("BluesHouse", include_bytes!("../maps/BluesHouse/map.blk")),
        ("BrunosRoom", include_bytes!("../maps/BrunosRoom/map.blk")),
        (
            "CeladonChiefHouse",
            include_bytes!("../maps/CeladonChiefHouse/map.blk"),
        ),
        ("CeladonCity", include_bytes!("../maps/CeladonCity/map.blk")),
        (
            "CeladonDiner",
            include_bytes!("../maps/CeladonDiner/map.blk"),
        ),
        ("CeladonGym", include_bytes!("../maps/CeladonGym/map.blk")),
        (
            "CeladonHotel",
            include_bytes!("../maps/CeladonHotel/map.blk"),
        ),
        (
            "CeladonMansion1F",
            include_bytes!("../maps/CeladonMansion1F/map.blk"),
        ),
        (
            "CeladonMansion2F",
            include_bytes!("../maps/CeladonMansion2F/map.blk"),
        ),
        (
            "CeladonMansion3F",
            include_bytes!("../maps/CeladonMansion3F/map.blk"),
        ),
        (
            "CeladonMansionRoof",
            include_bytes!("../maps/CeladonMansionRoof/map.blk"),
        ),
        (
            "CeladonMansionRoofHouse",
            include_bytes!("../maps/CeladonMansionRoofHouse/map.blk"),
        ),
        (
            "CeladonMart1F",
            include_bytes!("../maps/CeladonMart1F/map.blk"),
        ),
        (
            "CeladonMart2F",
            include_bytes!("../maps/CeladonMart2F/map.blk"),
        ),
        (
            "CeladonMart3F",
            include_bytes!("../maps/CeladonMart3F/map.blk"),
        ),
        (
            "CeladonMart4F",
            include_bytes!("../maps/CeladonMart4F/map.blk"),
        ),
        (
            "CeladonMart5F",
            include_bytes!("../maps/CeladonMart5F/map.blk"),
        ),
        (
            "CeladonMartElevator",
            include_bytes!("../maps/CeladonMartElevator/map.blk"),
        ),
        (
            "CeladonMartRoof",
            include_bytes!("../maps/CeladonMartRoof/map.blk"),
        ),
        (
            "CeladonPokecenter",
            include_bytes!("../maps/CeladonPokecenter/map.blk"),
        ),
        (
            "CeruleanBadgeHouse",
            include_bytes!("../maps/CeruleanBadgeHouse/map.blk"),
        ),
        (
            "CeruleanCave1F",
            include_bytes!("../maps/CeruleanCave1F/map.blk"),
        ),
        (
            "CeruleanCave2F",
            include_bytes!("../maps/CeruleanCave2F/map.blk"),
        ),
        (
            "CeruleanCaveB1F",
            include_bytes!("../maps/CeruleanCaveB1F/map.blk"),
        ),
        (
            "CeruleanCity",
            include_bytes!("../maps/CeruleanCity/map.blk"),
        ),
        ("CeruleanGym", include_bytes!("../maps/CeruleanGym/map.blk")),
        (
            "CeruleanMart",
            include_bytes!("../maps/CeruleanMart/map.blk"),
        ),
        (
            "CeruleanPokecenter",
            include_bytes!("../maps/CeruleanPokecenter/map.blk"),
        ),
        (
            "CeruleanTradeHouse",
            include_bytes!("../maps/CeruleanTradeHouse/map.blk"),
        ),
        (
            "CeruleanTrashedHouse",
            include_bytes!("../maps/CeruleanTrashedHouse/map.blk"),
        ),
        (
            "ChampionsRoom",
            include_bytes!("../maps/ChampionsRoom/map.blk"),
        ),
        ("CinnabarGym", include_bytes!("../maps/CinnabarGym/map.blk")),
        (
            "CinnabarIsland",
            include_bytes!("../maps/CinnabarIsland/map.blk"),
        ),
        ("CinnabarLab", include_bytes!("../maps/CinnabarLab/map.blk")),
        (
            "CinnabarLabFossilRoom",
            include_bytes!("../maps/CinnabarLabFossilRoom/map.blk"),
        ),
        (
            "CinnabarLabMetronomeRoom",
            include_bytes!("../maps/CinnabarLabMetronomeRoom/map.blk"),
        ),
        (
            "CinnabarLabTradeRoom",
            include_bytes!("../maps/CinnabarLabTradeRoom/map.blk"),
        ),
        (
            "CinnabarMart",
            include_bytes!("../maps/CinnabarMart/map.blk"),
        ),
        (
            "CinnabarPokecenter",
            include_bytes!("../maps/CinnabarPokecenter/map.blk"),
        ),
        ("Colosseum", include_bytes!("../maps/Colosseum/map.blk")),
        (
            "CopycatsHouse1F",
            include_bytes!("../maps/CopycatsHouse1F/map.blk"),
        ),
        (
            "CopycatsHouse2F",
            include_bytes!("../maps/CopycatsHouse2F/map.blk"),
        ),
        ("Daycare", include_bytes!("../maps/Daycare/map.blk")),
        (
            "DiglettsCave",
            include_bytes!("../maps/DiglettsCave/map.blk"),
        ),
        (
            "DiglettsCaveRoute11",
            include_bytes!("../maps/DiglettsCaveRoute11/map.blk"),
        ),
        (
            "DiglettsCaveRoute2",
            include_bytes!("../maps/DiglettsCaveRoute2/map.blk"),
        ),
        (
            "FightingDojo",
            include_bytes!("../maps/FightingDojo/map.blk"),
        ),
        (
            "FuchsiaBillsGrandpasHouse",
            include_bytes!("../maps/FuchsiaBillsGrandpasHouse/map.blk"),
        ),
        ("FuchsiaCity", include_bytes!("../maps/FuchsiaCity/map.blk")),
        (
            "FuchsiaGoodRodHouse",
            include_bytes!("../maps/FuchsiaGoodRodHouse/map.blk"),
        ),
        ("FuchsiaGym", include_bytes!("../maps/FuchsiaGym/map.blk")),
        ("FuchsiaMart", include_bytes!("../maps/FuchsiaMart/map.blk")),
        (
            "FuchsiaMeetingRoom",
            include_bytes!("../maps/FuchsiaMeetingRoom/map.blk"),
        ),
        (
            "FuchsiaPokecenter",
            include_bytes!("../maps/FuchsiaPokecenter/map.blk"),
        ),
        ("GameCorner", include_bytes!("../maps/GameCorner/map.blk")),
        (
            "GameCornerPrizeRoom",
            include_bytes!("../maps/GameCornerPrizeRoom/map.blk"),
        ),
        ("HallOfFame", include_bytes!("../maps/HallOfFame/map.blk")),
        (
            "IndigoPlateau",
            include_bytes!("../maps/IndigoPlateau/map.blk"),
        ),
        (
            "IndigoPlateauLobby",
            include_bytes!("../maps/IndigoPlateauLobby/map.blk"),
        ),
        ("LancesRoom", include_bytes!("../maps/LancesRoom/map.blk")),
        (
            "LavenderCuboneHouse",
            include_bytes!("../maps/LavenderCuboneHouse/map.blk"),
        ),
        (
            "LavenderMart",
            include_bytes!("../maps/LavenderMart/map.blk"),
        ),
        (
            "LavenderPokecenter",
            include_bytes!("../maps/LavenderPokecenter/map.blk"),
        ),
        (
            "LavenderTown",
            include_bytes!("../maps/LavenderTown/map.blk"),
        ),
        (
            "LoreleisRoom",
            include_bytes!("../maps/LoreleisRoom/map.blk"),
        ),
        (
            "MrFujisHouse",
            include_bytes!("../maps/MrFujisHouse/map.blk"),
        ),
        (
            "MrPsychicsHouse",
            include_bytes!("../maps/MrPsychicsHouse/map.blk"),
        ),
        ("MtMoon1F", include_bytes!("../maps/MtMoon1F/map.blk")),
        ("MtMoonB1F", include_bytes!("../maps/MtMoonB1F/map.blk")),
        ("MtMoonB2F", include_bytes!("../maps/MtMoonB2F/map.blk")),
        (
            "MtMoonPokecenter",
            include_bytes!("../maps/MtMoonPokecenter/map.blk"),
        ),
        ("Museum1F", include_bytes!("../maps/Museum1F/map.blk")),
        ("Museum2F", include_bytes!("../maps/Museum2F/map.blk")),
        (
            "NameRatersHouse",
            include_bytes!("../maps/NameRatersHouse/map.blk"),
        ),
        ("OaksLab", include_bytes!("../maps/OaksLab/map.blk")),
        ("PalletTown", include_bytes!("../maps/PalletTown/map.blk")),
        ("PewterCity", include_bytes!("../maps/PewterCity/map.blk")),
        ("PewterGym", include_bytes!("../maps/PewterGym/map.blk")),
        ("PewterMart", include_bytes!("../maps/PewterMart/map.blk")),
        (
            "PewterNidoranHouse",
            include_bytes!("../maps/PewterNidoranHouse/map.blk"),
        ),
        (
            "PewterPokecenter",
            include_bytes!("../maps/PewterPokecenter/map.blk"),
        ),
        (
            "PewterSpeechHouse",
            include_bytes!("../maps/PewterSpeechHouse/map.blk"),
        ),
        (
            "PokemonFanClub",
            include_bytes!("../maps/PokemonFanClub/map.blk"),
        ),
        (
            "PokemonMansion1F",
            include_bytes!("../maps/PokemonMansion1F/map.blk"),
        ),
        (
            "PokemonMansion2F",
            include_bytes!("../maps/PokemonMansion2F/map.blk"),
        ),
        (
            "PokemonMansion3F",
            include_bytes!("../maps/PokemonMansion3F/map.blk"),
        ),
        (
            "PokemonMansionB1F",
            include_bytes!("../maps/PokemonMansionB1F/map.blk"),
        ),
        (
            "PokemonTower1F",
            include_bytes!("../maps/PokemonTower1F/map.blk"),
        ),
        (
            "PokemonTower2F",
            include_bytes!("../maps/PokemonTower2F/map.blk"),
        ),
        (
            "PokemonTower3F",
            include_bytes!("../maps/PokemonTower3F/map.blk"),
        ),
        (
            "PokemonTower4F",
            include_bytes!("../maps/PokemonTower4F/map.blk"),
        ),
        (
            "PokemonTower5F",
            include_bytes!("../maps/PokemonTower5F/map.blk"),
        ),
        (
            "PokemonTower6F",
            include_bytes!("../maps/PokemonTower6F/map.blk"),
        ),
        (
            "PokemonTower7F",
            include_bytes!("../maps/PokemonTower7F/map.blk"),
        ),
        ("PowerPlant", include_bytes!("../maps/PowerPlant/map.blk")),
        ("RedsHouse1F", include_bytes!("../maps/RedsHouse1F/map.blk")),
        ("RedsHouse2F", include_bytes!("../maps/RedsHouse2F/map.blk")),
        (
            "RockTunnel1F",
            include_bytes!("../maps/RockTunnel1F/map.blk"),
        ),
        (
            "RockTunnelB1F",
            include_bytes!("../maps/RockTunnelB1F/map.blk"),
        ),
        (
            "RockTunnelPokecenter",
            include_bytes!("../maps/RockTunnelPokecenter/map.blk"),
        ),
        (
            "RocketHideoutB1F",
            include_bytes!("../maps/RocketHideoutB1F/map.blk"),
        ),
        (
            "RocketHideoutB2F",
            include_bytes!("../maps/RocketHideoutB2F/map.blk"),
        ),
        (
            "RocketHideoutB3F",
            include_bytes!("../maps/RocketHideoutB3F/map.blk"),
        ),
        (
            "RocketHideoutB4F",
            include_bytes!("../maps/RocketHideoutB4F/map.blk"),
        ),
        (
            "RocketHideoutElevator",
            include_bytes!("../maps/RocketHideoutElevator/map.blk"),
        ),
        ("Route1", include_bytes!("../maps/Route1/map.blk")),
        ("Route10", include_bytes!("../maps/Route10/map.blk")),
        ("Route11", include_bytes!("../maps/Route11/map.blk")),
        (
            "Route11Gate1F",
            include_bytes!("../maps/Route11Gate1F/map.blk"),
        ),
        (
            "Route11Gate2F",
            include_bytes!("../maps/Route11Gate2F/map.blk"),
        ),
        ("Route12", include_bytes!("../maps/Route12/map.blk")),
        (
            "Route12Gate1F",
            include_bytes!("../maps/Route12Gate1F/map.blk"),
        ),
        (
            "Route12Gate2F",
            include_bytes!("../maps/Route12Gate2F/map.blk"),
        ),
        (
            "Route12SuperRodHouse",
            include_bytes!("../maps/Route12SuperRodHouse/map.blk"),
        ),
        ("Route13", include_bytes!("../maps/Route13/map.blk")),
        ("Route14", include_bytes!("../maps/Route14/map.blk")),
        ("Route15", include_bytes!("../maps/Route15/map.blk")),
        (
            "Route15Gate1F",
            include_bytes!("../maps/Route15Gate1F/map.blk"),
        ),
        (
            "Route15Gate2F",
            include_bytes!("../maps/Route15Gate2F/map.blk"),
        ),
        ("Route16", include_bytes!("../maps/Route16/map.blk")),
        (
            "Route16FlyHouse",
            include_bytes!("../maps/Route16FlyHouse/map.blk"),
        ),
        (
            "Route16Gate1F",
            include_bytes!("../maps/Route16Gate1F/map.blk"),
        ),
        (
            "Route16Gate2F",
            include_bytes!("../maps/Route16Gate2F/map.blk"),
        ),
        ("Route17", include_bytes!("../maps/Route17/map.blk")),
        ("Route18", include_bytes!("../maps/Route18/map.blk")),
        (
            "Route18Gate1F",
            include_bytes!("../maps/Route18Gate1F/map.blk"),
        ),
        (
            "Route18Gate2F",
            include_bytes!("../maps/Route18Gate2F/map.blk"),
        ),
        ("Route19", include_bytes!("../maps/Route19/map.blk")),
        ("Route2", include_bytes!("../maps/Route2/map.blk")),
        ("Route20", include_bytes!("../maps/Route20/map.blk")),
        ("Route21", include_bytes!("../maps/Route21/map.blk")),
        ("Route22", include_bytes!("../maps/Route22/map.blk")),
        ("Route22Gate", include_bytes!("../maps/Route22Gate/map.blk")),
        ("Route23", include_bytes!("../maps/Route23/map.blk")),
        ("Route24", include_bytes!("../maps/Route24/map.blk")),
        ("Route25", include_bytes!("../maps/Route25/map.blk")),
        ("Route2Gate", include_bytes!("../maps/Route2Gate/map.blk")),
        (
            "Route2TradeHouse",
            include_bytes!("../maps/Route2TradeHouse/map.blk"),
        ),
        ("Route3", include_bytes!("../maps/Route3/map.blk")),
        ("Route4", include_bytes!("../maps/Route4/map.blk")),
        ("Route5", include_bytes!("../maps/Route5/map.blk")),
        ("Route5Gate", include_bytes!("../maps/Route5Gate/map.blk")),
        ("Route6", include_bytes!("../maps/Route6/map.blk")),
        ("Route6Gate", include_bytes!("../maps/Route6Gate/map.blk")),
        ("Route7", include_bytes!("../maps/Route7/map.blk")),
        ("Route7Gate", include_bytes!("../maps/Route7Gate/map.blk")),
        ("Route8", include_bytes!("../maps/Route8/map.blk")),
        ("Route8Gate", include_bytes!("../maps/Route8Gate/map.blk")),
        ("Route9", include_bytes!("../maps/Route9/map.blk")),
        ("SSAnne1F", include_bytes!("../maps/SSAnne1F/map.blk")),
        (
            "SSAnne1FRooms",
            include_bytes!("../maps/SSAnne1FRooms/map.blk"),
        ),
        ("SSAnne2F", include_bytes!("../maps/SSAnne2F/map.blk")),
        (
            "SSAnne2FRooms",
            include_bytes!("../maps/SSAnne2FRooms/map.blk"),
        ),
        ("SSAnne3F", include_bytes!("../maps/SSAnne3F/map.blk")),
        ("SSAnneB1F", include_bytes!("../maps/SSAnneB1F/map.blk")),
        (
            "SSAnneB1FRooms",
            include_bytes!("../maps/SSAnneB1FRooms/map.blk"),
        ),
        ("SSAnneBow", include_bytes!("../maps/SSAnneBow/map.blk")),
        (
            "SSAnneCaptainsRoom",
            include_bytes!("../maps/SSAnneCaptainsRoom/map.blk"),
        ),
        (
            "SSAnneKitchen",
            include_bytes!("../maps/SSAnneKitchen/map.blk"),
        ),
        (
            "SafariZoneCenter",
            include_bytes!("../maps/SafariZoneCenter/map.blk"),
        ),
        (
            "SafariZoneCenterRestHouse",
            include_bytes!("../maps/SafariZoneCenterRestHouse/map.blk"),
        ),
        (
            "SafariZoneEast",
            include_bytes!("../maps/SafariZoneEast/map.blk"),
        ),
        (
            "SafariZoneEastRestHouse",
            include_bytes!("../maps/SafariZoneEastRestHouse/map.blk"),
        ),
        (
            "SafariZoneGate",
            include_bytes!("../maps/SafariZoneGate/map.blk"),
        ),
        (
            "SafariZoneNorth",
            include_bytes!("../maps/SafariZoneNorth/map.blk"),
        ),
        (
            "SafariZoneNorthRestHouse",
            include_bytes!("../maps/SafariZoneNorthRestHouse/map.blk"),
        ),
        (
            "SafariZoneSecretHouse",
            include_bytes!("../maps/SafariZoneSecretHouse/map.blk"),
        ),
        (
            "SafariZoneWest",
            include_bytes!("../maps/SafariZoneWest/map.blk"),
        ),
        (
            "SafariZoneWestRestHouse",
            include_bytes!("../maps/SafariZoneWestRestHouse/map.blk"),
        ),
        ("SaffronCity", include_bytes!("../maps/SaffronCity/map.blk")),
        ("SaffronGym", include_bytes!("../maps/SaffronGym/map.blk")),
        ("SaffronMart", include_bytes!("../maps/SaffronMart/map.blk")),
        (
            "SaffronPidgeyHouse",
            include_bytes!("../maps/SaffronPidgeyHouse/map.blk"),
        ),
        (
            "SaffronPokecenter",
            include_bytes!("../maps/SaffronPokecenter/map.blk"),
        ),
        (
            "SeafoamIslands1F",
            include_bytes!("../maps/SeafoamIslands1F/map.blk"),
        ),
        (
            "SeafoamIslandsB1F",
            include_bytes!("../maps/SeafoamIslandsB1F/map.blk"),
        ),
        (
            "SeafoamIslandsB2F",
            include_bytes!("../maps/SeafoamIslandsB2F/map.blk"),
        ),
        (
            "SeafoamIslandsB3F",
            include_bytes!("../maps/SeafoamIslandsB3F/map.blk"),
        ),
        (
            "SeafoamIslandsB4F",
            include_bytes!("../maps/SeafoamIslandsB4F/map.blk"),
        ),
        ("SilphCo10F", include_bytes!("../maps/SilphCo10F/map.blk")),
        ("SilphCo11F", include_bytes!("../maps/SilphCo11F/map.blk")),
        ("SilphCo1F", include_bytes!("../maps/SilphCo1F/map.blk")),
        ("SilphCo2F", include_bytes!("../maps/SilphCo2F/map.blk")),
        ("SilphCo3F", include_bytes!("../maps/SilphCo3F/map.blk")),
        ("SilphCo4F", include_bytes!("../maps/SilphCo4F/map.blk")),
        ("SilphCo5F", include_bytes!("../maps/SilphCo5F/map.blk")),
        ("SilphCo6F", include_bytes!("../maps/SilphCo6F/map.blk")),
        ("SilphCo7F", include_bytes!("../maps/SilphCo7F/map.blk")),
        ("SilphCo8F", include_bytes!("../maps/SilphCo8F/map.blk")),
        ("SilphCo9F", include_bytes!("../maps/SilphCo9F/map.blk")),
        (
            "SilphCoElevator",
            include_bytes!("../maps/SilphCoElevator/map.blk"),
        ),
        ("TradeCenter", include_bytes!("../maps/TradeCenter/map.blk")),
        (
            "UndergroundPathNorthSouth",
            include_bytes!("../maps/UndergroundPathNorthSouth/map.blk"),
        ),
        (
            "UndergroundPathRoute5",
            include_bytes!("../maps/UndergroundPathRoute5/map.blk"),
        ),
        (
            "UndergroundPathRoute6",
            include_bytes!("../maps/UndergroundPathRoute6/map.blk"),
        ),
        (
            "UndergroundPathRoute7",
            include_bytes!("../maps/UndergroundPathRoute7/map.blk"),
        ),
        (
            "UndergroundPathRoute8",
            include_bytes!("../maps/UndergroundPathRoute8/map.blk"),
        ),
        (
            "UndergroundPathWestEast",
            include_bytes!("../maps/UndergroundPathWestEast/map.blk"),
        ),
        (
            "VermilionCity",
            include_bytes!("../maps/VermilionCity/map.blk"),
        ),
        (
            "VermilionDock",
            include_bytes!("../maps/VermilionDock/map.blk"),
        ),
        (
            "VermilionGym",
            include_bytes!("../maps/VermilionGym/map.blk"),
        ),
        (
            "VermilionMart",
            include_bytes!("../maps/VermilionMart/map.blk"),
        ),
        (
            "VermilionOldRodHouse",
            include_bytes!("../maps/VermilionOldRodHouse/map.blk"),
        ),
        (
            "VermilionPidgeyHouse",
            include_bytes!("../maps/VermilionPidgeyHouse/map.blk"),
        ),
        (
            "VermilionPokecenter",
            include_bytes!("../maps/VermilionPokecenter/map.blk"),
        ),
        (
            "VermilionTradeHouse",
            include_bytes!("../maps/VermilionTradeHouse/map.blk"),
        ),
        (
            "VictoryRoad1F",
            include_bytes!("../maps/VictoryRoad1F/map.blk"),
        ),
        (
            "VictoryRoad2F",
            include_bytes!("../maps/VictoryRoad2F/map.blk"),
        ),
        (
            "VictoryRoad3F",
            include_bytes!("../maps/VictoryRoad3F/map.blk"),
        ),
        (
            "ViridianCity",
            include_bytes!("../maps/ViridianCity/map.blk"),
        ),
        (
            "ViridianForest",
            include_bytes!("../maps/ViridianForest/map.blk"),
        ),
        (
            "ViridianForestNorthGate",
            include_bytes!("../maps/ViridianForestNorthGate/map.blk"),
        ),
        (
            "ViridianForestSouthGate",
            include_bytes!("../maps/ViridianForestSouthGate/map.blk"),
        ),
        ("ViridianGym", include_bytes!("../maps/ViridianGym/map.blk")),
        (
            "ViridianMart",
            include_bytes!("../maps/ViridianMart/map.blk"),
        ),
        (
            "ViridianNicknameHouse",
            include_bytes!("../maps/ViridianNicknameHouse/map.blk"),
        ),
        (
            "ViridianPokecenter",
            include_bytes!("../maps/ViridianPokecenter/map.blk"),
        ),
        (
            "ViridianSchoolHouse",
            include_bytes!("../maps/ViridianSchoolHouse/map.blk"),
        ),
        (
            "WardensHouse",
            include_bytes!("../maps/WardensHouse/map.blk"),
        ),
    ]
}

#[cfg(feature = "embedded-map-data")]
fn init_map_data() -> MapDataStore {
    let mut maps = HashMap::new();
    for (name, json_str) in embedded_json_sources() {
        match serde_json::from_str::<MapJson>(json_str) {
            Ok(map_json) => {
                maps.insert(name.to_string(), map_json);
            }
            Err(e) => {
                panic!("Failed to parse embedded map.json for {}: {}", name, e);
            }
        }
    }

    let mut blocks = HashMap::new();
    for (name, blk_data) in embedded_blk_sources() {
        blocks.insert(name.to_string(), blk_data.to_vec());
    }

    MapDataStore { maps, blocks }
}

// ── Filesystem mode ────────────────────────────────────────────────────────

#[cfg(not(feature = "embedded-map-data"))]
fn init_map_data() -> MapDataStore {
    let maps_dir = find_maps_directory();
    let mut maps = HashMap::new();
    let mut blocks = HashMap::new();

    if let Some(dir) = &maps_dir {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                let map_name = match path.file_name().and_then(|n| n.to_str()) {
                    Some(n) => n.to_string(),
                    None => continue,
                };

                let json_path = path.join("map.json");
                if json_path.exists() {
                    match std::fs::read_to_string(&json_path) {
                        Ok(json_str) => match serde_json::from_str::<MapJson>(&json_str) {
                            Ok(map_json) => {
                                maps.insert(map_name.clone(), map_json);
                            }
                            Err(e) => {
                                log::warn!("Failed to parse {}: {}", json_path.display(), e);
                            }
                        },
                        Err(e) => {
                            log::warn!("Failed to read {}: {}", json_path.display(), e);
                        }
                    }
                }

                let blk_path = path.join("map.blk");
                if blk_path.exists() {
                    match std::fs::read(&blk_path) {
                        Ok(data) => {
                            blocks.insert(map_name, data);
                        }
                        Err(e) => {
                            log::warn!("Failed to read {}: {}", blk_path.display(), e);
                        }
                    }
                }
            }
        }
    }

    log::info!(
        "MapDataLoader: loaded {} maps, {} block files from filesystem",
        maps.len(),
        blocks.len()
    );
    MapDataStore { maps, blocks }
}

#[cfg(not(feature = "embedded-map-data"))]
fn find_maps_directory() -> Option<std::path::PathBuf> {
    // Try relative to the workspace root: crates/pokered-data/maps/
    let candidates = [
        std::path::PathBuf::from("crates/pokered-data/maps"),
        std::path::PathBuf::from("../pokered-data/maps"),
        std::path::PathBuf::from("pokered-data/maps"),
    ];
    for candidate in &candidates {
        if candidate.is_dir() {
            log::info!("MapDataLoader: using maps directory {:?}", candidate);
            return Some(candidate.clone());
        }
    }

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let from_manifest = std::path::PathBuf::from(manifest_dir).join("maps");
        if from_manifest.is_dir() {
            return Some(from_manifest);
        }
    }

    log::warn!("MapDataLoader: could not find maps directory");
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_to_map_id_roundtrip() {
        let id_map = name_to_map_id();
        assert_eq!(id_map.get("PalletTown"), Some(&MapId::PalletTown));
        assert_eq!(id_map.get("Route1"), Some(&MapId::Route1));
        assert_eq!(id_map.get("OaksLab"), Some(&MapId::OaksLab));
        assert_eq!(id_map.get("NonExistent"), None);
    }

    #[test]
    fn test_resolve_map_id() {
        assert_eq!(resolve_map_id("PalletTown"), Some(MapId::PalletTown));
        assert_eq!(resolve_map_id("CeruleanCity"), Some(MapId::CeruleanCity));
        assert_eq!(resolve_map_id("Bogus"), None);
    }
}
