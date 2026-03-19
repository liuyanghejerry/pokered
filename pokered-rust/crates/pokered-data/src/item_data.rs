use crate::items::ItemId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemData {
    pub id: ItemId,
    pub name: &'static str,
    pub price: u16,
    pub is_key_item: bool,
}

/// Get item data by ItemId. Returns None for NoItem.
pub fn get_item_data(id: ItemId) -> Option<&'static ItemData> {
    let idx = id as usize;
    if idx == 0 || idx > 83 {
        None
    } else {
        Some(&ITEM_DATA[idx - 1])
    }
}

pub const ITEM_DATA: [ItemData; 83] = [
    ItemData {
        id: ItemId::MasterBall,
        name: "MASTER BALL",
        price: 0,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::UltraBall,
        name: "ULTRA BALL",
        price: 1200,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::GreatBall,
        name: "GREAT BALL",
        price: 600,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::PokeBall,
        name: "POKé BALL",
        price: 200,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::TownMap,
        name: "TOWN MAP",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::Bicycle,
        name: "BICYCLE",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::Surfboard,
        name: "?????",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::SafariBall,
        name: "SAFARI BALL",
        price: 1000,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::Pokedex,
        name: "POKéDEX",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::MoonStone,
        name: "MOON STONE",
        price: 0,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::Antidote,
        name: "ANTIDOTE",
        price: 100,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::BurnHeal,
        name: "BURN HEAL",
        price: 250,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::IceHeal,
        name: "ICE HEAL",
        price: 250,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::Awakening,
        name: "AWAKENING",
        price: 200,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::ParlyzHeal,
        name: "PARLYZ HEAL",
        price: 200,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::FullRestore,
        name: "FULL RESTORE",
        price: 3000,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::MaxPotion,
        name: "MAX POTION",
        price: 2500,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::HyperPotion,
        name: "HYPER POTION",
        price: 1500,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::SuperPotion,
        name: "SUPER POTION",
        price: 700,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::Potion,
        name: "POTION",
        price: 300,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::BoulderBadge,
        name: "BOULDERBADGE",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::CascadeBadge,
        name: "CASCADEBADGE",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::ThunderBadge,
        name: "THUNDERBADGE",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::RainbowBadge,
        name: "RAINBOWBADGE",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::SoulBadge,
        name: "SOULBADGE",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::MarshBadge,
        name: "MARSHBADGE",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::VolcanoBadge,
        name: "VOLCANOBADGE",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::EarthBadge,
        name: "EARTHBADGE",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::EscapeRope,
        name: "ESCAPE ROPE",
        price: 550,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::Repel,
        name: "REPEL",
        price: 350,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::OldAmber,
        name: "OLD AMBER",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::FireStone,
        name: "FIRE STONE",
        price: 2100,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::ThunderStone,
        name: "THUNDERSTONE",
        price: 2100,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::WaterStone,
        name: "WATER STONE",
        price: 2100,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::HpUp,
        name: "HP UP",
        price: 9800,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::Protein,
        name: "PROTEIN",
        price: 9800,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::Iron,
        name: "IRON",
        price: 9800,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::Carbos,
        name: "CARBOS",
        price: 9800,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::Calcium,
        name: "CALCIUM",
        price: 9800,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::RareCandy,
        name: "RARE CANDY",
        price: 4800,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::DomeFossil,
        name: "DOME FOSSIL",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::HelixFossil,
        name: "HELIX FOSSIL",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::SecretKey,
        name: "SECRET KEY",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::Unused2C,
        name: "?????",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::BikeVoucher,
        name: "BIKE VOUCHER",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::XAccuracy,
        name: "X ACCURACY",
        price: 950,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::LeafStone,
        name: "LEAF STONE",
        price: 2100,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::CardKey,
        name: "CARD KEY",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::Nugget,
        name: "NUGGET",
        price: 10000,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::Unused32,
        name: "PP UP",
        price: 9800,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::PokeDoll,
        name: "POKé DOLL",
        price: 1000,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::FullHeal,
        name: "FULL HEAL",
        price: 600,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::Revive,
        name: "REVIVE",
        price: 1500,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::MaxRevive,
        name: "MAX REVIVE",
        price: 4000,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::GuardSpec,
        name: "GUARD SPEC.",
        price: 700,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::SuperRepel,
        name: "SUPER REPEL",
        price: 500,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::MaxRepel,
        name: "MAX REPEL",
        price: 700,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::DireHit,
        name: "DIRE HIT",
        price: 650,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::Coin,
        name: "COIN",
        price: 10,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::FreshWater,
        name: "FRESH WATER",
        price: 200,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::SodaPop,
        name: "SODA POP",
        price: 300,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::Lemonade,
        name: "LEMONADE",
        price: 350,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::SsTicket,
        name: "S.S.TICKET",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::GoldTeeth,
        name: "GOLD TEETH",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::XAttack,
        name: "X ATTACK",
        price: 500,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::XDefend,
        name: "X DEFEND",
        price: 550,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::XSpeed,
        name: "X SPEED",
        price: 350,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::XSpecial,
        name: "X SPECIAL",
        price: 350,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::CoinCase,
        name: "COIN CASE",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::OaksParcel,
        name: "OAK's PARCEL",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::Itemfinder,
        name: "ITEMFINDER",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::SilphScope,
        name: "SILPH SCOPE",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::PokeFlute,
        name: "POKé FLUTE",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::LiftKey,
        name: "LIFT KEY",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::ExpAll,
        name: "EXP.ALL",
        price: 0,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::OldRod,
        name: "OLD ROD",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::GoodRod,
        name: "GOOD ROD",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::SuperRod,
        name: "SUPER ROD",
        price: 0,
        is_key_item: true,
    },
    ItemData {
        id: ItemId::PpUp,
        name: "PP UP",
        price: 0,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::Ether,
        name: "ETHER",
        price: 0,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::MaxEther,
        name: "MAX ETHER",
        price: 0,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::Elixer,
        name: "ELIXER",
        price: 0,
        is_key_item: false,
    },
    ItemData {
        id: ItemId::MaxElixer,
        name: "MAX ELIXER",
        price: 0,
        is_key_item: false,
    },
];

/// TM prices in Pokédollars (thousands * 1000)
pub const TM_PRICES: [u16; 50] = [
    3000, // TM01
    2000, // TM02
    2000, // TM03
    1000, // TM04
    3000, // TM05
    4000, // TM06
    2000, // TM07
    4000, // TM08
    3000, // TM09
    4000, // TM10
    2000, // TM11
    1000, // TM12
    4000, // TM13
    5000, // TM14
    5000, // TM15
    5000, // TM16
    3000, // TM17
    2000, // TM18
    3000, // TM19
    2000, // TM20
    5000, // TM21
    5000, // TM22
    5000, // TM23
    2000, // TM24
    5000, // TM25
    4000, // TM26
    5000, // TM27
    2000, // TM28
    4000, // TM29
    1000, // TM30
    2000, // TM31
    1000, // TM32
    1000, // TM33
    2000, // TM34
    4000, // TM35
    2000, // TM36
    2000, // TM37
    5000, // TM38
    2000, // TM39
    4000, // TM40
    2000, // TM41
    2000, // TM42
    5000, // TM43
    2000, // TM44
    2000, // TM45
    4000, // TM46
    3000, // TM47
    4000, // TM48
    4000, // TM49
    2000, // TM50
];
