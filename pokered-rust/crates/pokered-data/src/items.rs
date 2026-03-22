use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Display,
    EnumCount,
    EnumIter,
    EnumString,
    FromPrimitive,
    ToPrimitive,
)]
#[repr(u8)]
pub enum ItemId {
    NoItem = 0x00,
    MasterBall = 0x01,
    UltraBall = 0x02,
    GreatBall = 0x03,
    PokeBall = 0x04,
    TownMap = 0x05,
    Bicycle = 0x06,
    Surfboard = 0x07,
    SafariBall = 0x08,
    Pokedex = 0x09,
    MoonStone = 0x0A,
    Antidote = 0x0B,
    BurnHeal = 0x0C,
    IceHeal = 0x0D,
    Awakening = 0x0E,
    ParlyzHeal = 0x0F,
    FullRestore = 0x10,
    MaxPotion = 0x11,
    HyperPotion = 0x12,
    SuperPotion = 0x13,
    Potion = 0x14,
    BoulderBadge = 0x15,
    CascadeBadge = 0x16,
    ThunderBadge = 0x17,
    RainbowBadge = 0x18,
    SoulBadge = 0x19,
    MarshBadge = 0x1A,
    VolcanoBadge = 0x1B,
    EarthBadge = 0x1C,
    EscapeRope = 0x1D,
    Repel = 0x1E,
    OldAmber = 0x1F,
    FireStone = 0x20,
    ThunderStone = 0x21,
    WaterStone = 0x22,
    HpUp = 0x23,
    Protein = 0x24,
    Iron = 0x25,
    Carbos = 0x26,
    Calcium = 0x27,
    RareCandy = 0x28,
    DomeFossil = 0x29,
    HelixFossil = 0x2A,
    SecretKey = 0x2B,
    Unused2C = 0x2C,
    BikeVoucher = 0x2D,
    XAccuracy = 0x2E,
    LeafStone = 0x2F,
    CardKey = 0x30,
    Nugget = 0x31,
    Unused32 = 0x32,
    PokeDoll = 0x33,
    FullHeal = 0x34,
    Revive = 0x35,
    MaxRevive = 0x36,
    GuardSpec = 0x37,
    SuperRepel = 0x38,
    MaxRepel = 0x39,
    DireHit = 0x3A,
    Coin = 0x3B,
    FreshWater = 0x3C,
    SodaPop = 0x3D,
    Lemonade = 0x3E,
    SsTicket = 0x3F,
    GoldTeeth = 0x40,
    XAttack = 0x41,
    XDefend = 0x42,
    XSpeed = 0x43,
    XSpecial = 0x44,
    CoinCase = 0x45,
    OaksParcel = 0x46,
    Itemfinder = 0x47,
    SilphScope = 0x48,
    PokeFlute = 0x49,
    LiftKey = 0x4A,
    ExpAll = 0x4B,
    OldRod = 0x4C,
    GoodRod = 0x4D,
    SuperRod = 0x4E,
    PpUp = 0x4F,
    Ether = 0x50,
    MaxEther = 0x51,
    Elixer = 0x52,
    MaxElixer = 0x53,
}

pub const NUM_ITEMS: u8 = 0x53;

impl ItemId {
    pub fn from_id(id: u8) -> Self {
        num_traits::FromPrimitive::from_u8(id).unwrap_or(ItemId::NoItem)
    }

    pub fn is_badge(self) -> bool {
        let v = self as u8;
        v >= 0x15 && v <= 0x1C
    }

    pub fn is_key_item(self) -> bool {
        matches!(
            self,
            ItemId::TownMap
                | ItemId::Bicycle
                | ItemId::Surfboard
                | ItemId::Pokedex
                | ItemId::OldAmber
                | ItemId::SecretKey
                | ItemId::BikeVoucher
                | ItemId::CardKey
                | ItemId::SsTicket
                | ItemId::GoldTeeth
                | ItemId::CoinCase
                | ItemId::OaksParcel
                | ItemId::Itemfinder
                | ItemId::SilphScope
                | ItemId::PokeFlute
                | ItemId::LiftKey
                | ItemId::OldRod
                | ItemId::GoodRod
                | ItemId::SuperRod
                | ItemId::DomeFossil
                | ItemId::HelixFossil
        )
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, FromPrimitive, ToPrimitive,
)]
#[repr(u8)]
pub enum HmId {
    Hm01Cut = 1,
    Hm02Fly = 2,
    Hm03Surf = 3,
    Hm04Strength = 4,
    Hm05Flash = 5,
}

pub const NUM_HMS: u8 = 5;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, FromPrimitive, ToPrimitive,
)]
#[repr(u8)]
pub enum TmId {
    Tm01 = 1,
    Tm02 = 2,
    Tm03 = 3,
    Tm04 = 4,
    Tm05 = 5,
    Tm06 = 6,
    Tm07 = 7,
    Tm08 = 8,
    Tm09 = 9,
    Tm10 = 10,
    Tm11 = 11,
    Tm12 = 12,
    Tm13 = 13,
    Tm14 = 14,
    Tm15 = 15,
    Tm16 = 16,
    Tm17 = 17,
    Tm18 = 18,
    Tm19 = 19,
    Tm20 = 20,
    Tm21 = 21,
    Tm22 = 22,
    Tm23 = 23,
    Tm24 = 24,
    Tm25 = 25,
    Tm26 = 26,
    Tm27 = 27,
    Tm28 = 28,
    Tm29 = 29,
    Tm30 = 30,
    Tm31 = 31,
    Tm32 = 32,
    Tm33 = 33,
    Tm34 = 34,
    Tm35 = 35,
    Tm36 = 36,
    Tm37 = 37,
    Tm38 = 38,
    Tm39 = 39,
    Tm40 = 40,
    Tm41 = 41,
    Tm42 = 42,
    Tm43 = 43,
    Tm44 = 44,
    Tm45 = 45,
    Tm46 = 46,
    Tm47 = 47,
    Tm48 = 48,
    Tm49 = 49,
    Tm50 = 50,
}

pub const NUM_TMS: u8 = 50;

use crate::moves::MoveId;

pub const TM_MOVES: [MoveId; 50] = [
    MoveId::MegaPunch,
    MoveId::RazorWind,
    MoveId::SwordsDance,
    MoveId::Whirlwind,
    MoveId::MegaKick,
    MoveId::Toxic,
    MoveId::HornDrill,
    MoveId::BodySlam,
    MoveId::TakeDown,
    MoveId::DoubleEdge,
    MoveId::Bubblebeam,
    MoveId::WaterGun,
    MoveId::IceBeam,
    MoveId::Blizzard,
    MoveId::HyperBeam,
    MoveId::PayDay,
    MoveId::Submission,
    MoveId::Counter,
    MoveId::SeismicToss,
    MoveId::Rage,
    MoveId::MegaDrain,
    MoveId::Solarbeam,
    MoveId::DragonRage,
    MoveId::Thunderbolt,
    MoveId::Thunder,
    MoveId::Earthquake,
    MoveId::Fissure,
    MoveId::Dig,
    MoveId::PsychicM,
    MoveId::Teleport,
    MoveId::Mimic,
    MoveId::DoubleTeam,
    MoveId::Reflect,
    MoveId::Bide,
    MoveId::Metronome,
    MoveId::Selfdestruct,
    MoveId::EggBomb,
    MoveId::FireBlast,
    MoveId::Swift,
    MoveId::SkullBash,
    MoveId::Softboiled,
    MoveId::DreamEater,
    MoveId::SkyAttack,
    MoveId::Rest,
    MoveId::ThunderWave,
    MoveId::Psywave,
    MoveId::Explosion,
    MoveId::RockSlide,
    MoveId::TriAttack,
    MoveId::Substitute,
];

pub const HM_MOVES: [MoveId; 5] = [
    MoveId::Cut,
    MoveId::Fly,
    MoveId::Surf,
    MoveId::Strength,
    MoveId::Flash,
];
