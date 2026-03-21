#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum MusicId {
    PalletTown = 0,
    Pokecenter = 1,
    Gym = 2,
    Cities1 = 3,
    Cities2 = 4,
    Celadon = 5,
    Cinnabar = 6,
    Vermilion = 7,
    Lavender = 8,
    SSAnne = 9,
    MeetProfOak = 10,
    MeetRival = 11,
    MuseumGuy = 12,
    SafariZone = 13,
    PkmnHealed = 14,
    Routes1 = 15,
    Routes2 = 16,
    Routes3 = 17,
    Routes4 = 18,
    IndigoPlateau = 19,
    GymLeaderBattle = 20,
    TrainerBattle = 21,
    WildBattle = 22,
    FinalBattle = 23,
    DefeatedTrainer = 24,
    DefeatedWildMon = 25,
    DefeatedGymLeader = 26,
    TitleScreen = 27,
    Credits = 28,
    HallOfFame = 29,
    OaksLab = 30,
    JigglypuffSong = 31,
    BikeRiding = 32,
    Surfing = 33,
    GameCorner = 34,
    IntroBattle = 35,
    Dungeon1 = 36,
    Dungeon2 = 37,
    Dungeon3 = 38,
    CinnabarMansion = 39,
    PokemonTower = 40,
    SilphCo = 41,
    MeetEvilTrainer = 42,
    MeetFemaleTrainer = 43,
    MeetMaleTrainer = 44,
}

pub const NUM_MUSIC_TRACKS: usize = 45;
pub const SFX_STOP_ALL_MUSIC: u8 = 0xFF;

impl MusicId {
    pub fn from_u8(value: u8) -> Option<MusicId> {
        if (value as usize) < NUM_MUSIC_TRACKS {
            Some(unsafe { core::mem::transmute(value) })
        } else {
            None
        }
    }
}
