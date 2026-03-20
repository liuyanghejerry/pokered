use crate::species::Species;
use serde::{Deserialize, Serialize};

/// A trainer class identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum TrainerClass {
    Nobody = 0,
    Youngster = 1,
    BugCatcher = 2,
    Lass = 3,
    Sailor = 4,
    JrTrainerM = 5,
    JrTrainerF = 6,
    Pokemaniac = 7,
    SuperNerd = 8,
    Hiker = 9,
    Biker = 10,
    Burglar = 11,
    Engineer = 12,
    UnusedJuggler = 13,
    Fisher = 14,
    Swimmer = 15,
    CueBall = 16,
    Gambler = 17,
    Beauty = 18,
    PsychicTr = 19,
    Rocker = 20,
    Juggler = 21,
    Tamer = 22,
    BirdKeeper = 23,
    Blackbelt = 24,
    Rival1 = 25,
    ProfOak = 26,
    Chief = 27,
    Scientist = 28,
    Giovanni = 29,
    Rocket = 30,
    CooltrainerM = 31,
    CooltrainerF = 32,
    Bruno = 33,
    Brock = 34,
    Misty = 35,
    LtSurge = 36,
    Erika = 37,
    Koga = 38,
    Blaine = 39,
    Sabrina = 40,
    Gentleman = 41,
    Rival2 = 42,
    Rival3 = 43,
    Lorelei = 44,
    Channeler = 45,
    Agatha = 46,
    Lance = 47,
}

pub const NUM_TRAINER_CLASSES: u8 = 47;

/// A single Pokémon in a trainer's party
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainerMon {
    pub level: u8,
    pub species: Species,
}

/// A trainer party (one encounter instance)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainerParty {
    pub pokemon: Vec<TrainerMon>,
}

/// All parties for a trainer class (multiple trainers of the same class)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainerClassData {
    pub class: TrainerClass,
    pub parties: Vec<TrainerParty>,
}

/// Get all trainer class data
pub fn trainer_data() -> Vec<TrainerClassData> {
    vec![
        TrainerClassData {
            class: TrainerClass::Youngster,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 11,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 11,
                            species: Species::Ekans,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 14,
                        species: Species::Spearow,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 10,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 10,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 10,
                            species: Species::Zubat,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 14,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 14,
                            species: Species::Ekans,
                        },
                        TrainerMon {
                            level: 14,
                            species: Species::Zubat,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 15,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 15,
                            species: Species::Spearow,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 17,
                        species: Species::Slowpoke,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 14,
                            species: Species::Ekans,
                        },
                        TrainerMon {
                            level: 14,
                            species: Species::Sandshrew,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 21,
                        species: Species::NidoranM,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 21,
                        species: Species::Ekans,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 19,
                            species: Species::Sandshrew,
                        },
                        TrainerMon {
                            level: 19,
                            species: Species::Zubat,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 17,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Raticate,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 18,
                            species: Species::NidoranM,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Nidorino,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 17,
                            species: Species::Spearow,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Spearow,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::BugCatcher,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 6,
                            species: Species::Weedle,
                        },
                        TrainerMon {
                            level: 6,
                            species: Species::Caterpie,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 7,
                            species: Species::Weedle,
                        },
                        TrainerMon {
                            level: 7,
                            species: Species::Kakuna,
                        },
                        TrainerMon {
                            level: 7,
                            species: Species::Weedle,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 9,
                        species: Species::Weedle,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 10,
                            species: Species::Caterpie,
                        },
                        TrainerMon {
                            level: 10,
                            species: Species::Weedle,
                        },
                        TrainerMon {
                            level: 10,
                            species: Species::Caterpie,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 9,
                            species: Species::Weedle,
                        },
                        TrainerMon {
                            level: 9,
                            species: Species::Kakuna,
                        },
                        TrainerMon {
                            level: 9,
                            species: Species::Caterpie,
                        },
                        TrainerMon {
                            level: 9,
                            species: Species::Metapod,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 11,
                            species: Species::Caterpie,
                        },
                        TrainerMon {
                            level: 11,
                            species: Species::Metapod,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 11,
                            species: Species::Weedle,
                        },
                        TrainerMon {
                            level: 11,
                            species: Species::Kakuna,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 10,
                            species: Species::Caterpie,
                        },
                        TrainerMon {
                            level: 10,
                            species: Species::Metapod,
                        },
                        TrainerMon {
                            level: 10,
                            species: Species::Caterpie,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 14,
                            species: Species::Caterpie,
                        },
                        TrainerMon {
                            level: 14,
                            species: Species::Weedle,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 16,
                            species: Species::Weedle,
                        },
                        TrainerMon {
                            level: 16,
                            species: Species::Caterpie,
                        },
                        TrainerMon {
                            level: 16,
                            species: Species::Weedle,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 20,
                        species: Species::Butterfree,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 18,
                            species: Species::Metapod,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Caterpie,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Venonat,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 19,
                            species: Species::Beedrill,
                        },
                        TrainerMon {
                            level: 19,
                            species: Species::Beedrill,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 20,
                            species: Species::Caterpie,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Weedle,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Venonat,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Lass,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 9,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 9,
                            species: Species::Pidgey,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 10,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 10,
                            species: Species::NidoranM,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 14,
                        species: Species::Jigglypuff,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 31,
                            species: Species::Paras,
                        },
                        TrainerMon {
                            level: 31,
                            species: Species::Paras,
                        },
                        TrainerMon {
                            level: 31,
                            species: Species::Parasect,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 11,
                            species: Species::Oddish,
                        },
                        TrainerMon {
                            level: 11,
                            species: Species::Bellsprout,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 14,
                        species: Species::Clefairy,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 16,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 16,
                            species: Species::NidoranF,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 14,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 14,
                            species: Species::NidoranF,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 15,
                            species: Species::NidoranM,
                        },
                        TrainerMon {
                            level: 15,
                            species: Species::NidoranF,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 13,
                            species: Species::Oddish,
                        },
                        TrainerMon {
                            level: 13,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 13,
                            species: Species::Oddish,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 18,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::NidoranF,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 18,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Pikachu,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 23,
                            species: Species::NidoranF,
                        },
                        TrainerMon {
                            level: 23,
                            species: Species::Nidorina,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 24,
                            species: Species::Meowth,
                        },
                        TrainerMon {
                            level: 24,
                            species: Species::Meowth,
                        },
                        TrainerMon {
                            level: 24,
                            species: Species::Meowth,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 19,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 19,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 19,
                            species: Species::NidoranM,
                        },
                        TrainerMon {
                            level: 19,
                            species: Species::Meowth,
                        },
                        TrainerMon {
                            level: 19,
                            species: Species::Pikachu,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 22,
                            species: Species::Clefairy,
                        },
                        TrainerMon {
                            level: 22,
                            species: Species::Clefairy,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 23,
                            species: Species::Bellsprout,
                        },
                        TrainerMon {
                            level: 23,
                            species: Species::Weepinbell,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 23,
                            species: Species::Oddish,
                        },
                        TrainerMon {
                            level: 23,
                            species: Species::Gloom,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Sailor,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 18,
                            species: Species::Machop,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Shellder,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 17,
                            species: Species::Machop,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Tentacool,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 21,
                        species: Species::Shellder,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 17,
                            species: Species::Horsea,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Shellder,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Tentacool,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 18,
                            species: Species::Tentacool,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Staryu,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 17,
                            species: Species::Horsea,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Horsea,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Horsea,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 20,
                        species: Species::Machop,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 21,
                            species: Species::Pikachu,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Pikachu,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::JrTrainerM,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 11,
                            species: Species::Diglett,
                        },
                        TrainerMon {
                            level: 11,
                            species: Species::Sandshrew,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 14,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 14,
                            species: Species::Ekans,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 18,
                        species: Species::Mankey,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 20,
                        species: Species::Squirtle,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 16,
                            species: Species::Spearow,
                        },
                        TrainerMon {
                            level: 16,
                            species: Species::Raticate,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 18,
                            species: Species::Diglett,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Diglett,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Sandshrew,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 21,
                            species: Species::Growlithe,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Charmander,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 19,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 19,
                            species: Species::Diglett,
                        },
                        TrainerMon {
                            level: 19,
                            species: Species::Ekans,
                        },
                        TrainerMon {
                            level: 19,
                            species: Species::Sandshrew,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::NidoranM,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Nidorino,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::JrTrainerF,
            parties: vec![
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 19,
                        species: Species::Goldeen,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 16,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 16,
                            species: Species::Pikachu,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 16,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 16,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 16,
                            species: Species::Pidgey,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 22,
                        species: Species::Bulbasaur,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 18,
                            species: Species::Oddish,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Bellsprout,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Oddish,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Bellsprout,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 23,
                        species: Species::Meowth,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 20,
                            species: Species::Pikachu,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Clefairy,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 21,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Pidgeotto,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 21,
                            species: Species::Jigglypuff,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Meowth,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 22,
                            species: Species::Oddish,
                        },
                        TrainerMon {
                            level: 22,
                            species: Species::Bulbasaur,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 24,
                            species: Species::Bulbasaur,
                        },
                        TrainerMon {
                            level: 24,
                            species: Species::Ivysaur,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 24,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 24,
                            species: Species::Meowth,
                        },
                        TrainerMon {
                            level: 24,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 24,
                            species: Species::Pikachu,
                        },
                        TrainerMon {
                            level: 24,
                            species: Species::Meowth,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 30,
                            species: Species::Poliwag,
                        },
                        TrainerMon {
                            level: 30,
                            species: Species::Poliwag,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 27,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Meowth,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Pidgeotto,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Goldeen,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Poliwag,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Horsea,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 31,
                            species: Species::Goldeen,
                        },
                        TrainerMon {
                            level: 31,
                            species: Species::Seaking,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 22,
                            species: Species::Bellsprout,
                        },
                        TrainerMon {
                            level: 22,
                            species: Species::Clefairy,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 20,
                            species: Species::Meowth,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Oddish,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Pidgey,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 19,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 19,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 19,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 19,
                            species: Species::Bellsprout,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Gloom,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Oddish,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Oddish,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Pikachu,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Raichu,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 33,
                        species: Species::Clefairy,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Bellsprout,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Oddish,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Tangela,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 30,
                            species: Species::Tentacool,
                        },
                        TrainerMon {
                            level: 30,
                            species: Species::Horsea,
                        },
                        TrainerMon {
                            level: 30,
                            species: Species::Seel,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Pokemaniac,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 30,
                            species: Species::Rhyhorn,
                        },
                        TrainerMon {
                            level: 30,
                            species: Species::Lickitung,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 20,
                            species: Species::Cubone,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Slowpoke,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 20,
                            species: Species::Slowpoke,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Slowpoke,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Slowpoke,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 22,
                            species: Species::Charmander,
                        },
                        TrainerMon {
                            level: 22,
                            species: Species::Cubone,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 25,
                        species: Species::Slowpoke,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 40,
                            species: Species::Charmeleon,
                        },
                        TrainerMon {
                            level: 40,
                            species: Species::Lapras,
                        },
                        TrainerMon {
                            level: 40,
                            species: Species::Lickitung,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 23,
                            species: Species::Cubone,
                        },
                        TrainerMon {
                            level: 23,
                            species: Species::Slowpoke,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::SuperNerd,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 11,
                            species: Species::Magnemite,
                        },
                        TrainerMon {
                            level: 11,
                            species: Species::Voltorb,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 12,
                            species: Species::Grimer,
                        },
                        TrainerMon {
                            level: 12,
                            species: Species::Voltorb,
                        },
                        TrainerMon {
                            level: 12,
                            species: Species::Koffing,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 20,
                            species: Species::Voltorb,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Voltorb,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Magnemite,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 22,
                            species: Species::Grimer,
                        },
                        TrainerMon {
                            level: 22,
                            species: Species::Muk,
                        },
                        TrainerMon {
                            level: 22,
                            species: Species::Grimer,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 26,
                        species: Species::Koffing,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 22,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 22,
                            species: Species::Magnemite,
                        },
                        TrainerMon {
                            level: 22,
                            species: Species::Weezing,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 20,
                            species: Species::Magnemite,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Magnemite,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Magnemite,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 24,
                            species: Species::Magnemite,
                        },
                        TrainerMon {
                            level: 24,
                            species: Species::Voltorb,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 36,
                            species: Species::Vulpix,
                        },
                        TrainerMon {
                            level: 36,
                            species: Species::Vulpix,
                        },
                        TrainerMon {
                            level: 36,
                            species: Species::Ninetales,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 34,
                            species: Species::Ponyta,
                        },
                        TrainerMon {
                            level: 34,
                            species: Species::Charmander,
                        },
                        TrainerMon {
                            level: 34,
                            species: Species::Vulpix,
                        },
                        TrainerMon {
                            level: 34,
                            species: Species::Growlithe,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 41,
                        species: Species::Rapidash,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 37,
                            species: Species::Growlithe,
                        },
                        TrainerMon {
                            level: 37,
                            species: Species::Vulpix,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Hiker,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 10,
                            species: Species::Geodude,
                        },
                        TrainerMon {
                            level: 10,
                            species: Species::Geodude,
                        },
                        TrainerMon {
                            level: 10,
                            species: Species::Onix,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 15,
                            species: Species::Machop,
                        },
                        TrainerMon {
                            level: 15,
                            species: Species::Geodude,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 13,
                            species: Species::Geodude,
                        },
                        TrainerMon {
                            level: 13,
                            species: Species::Geodude,
                        },
                        TrainerMon {
                            level: 13,
                            species: Species::Machop,
                        },
                        TrainerMon {
                            level: 13,
                            species: Species::Geodude,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 17,
                        species: Species::Onix,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 21,
                            species: Species::Geodude,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Onix,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 20,
                            species: Species::Geodude,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Machop,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Geodude,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 21,
                            species: Species::Geodude,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Onix,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 19,
                            species: Species::Onix,
                        },
                        TrainerMon {
                            level: 19,
                            species: Species::Graveler,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 21,
                            species: Species::Geodude,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Geodude,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Graveler,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 25,
                        species: Species::Geodude,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 20,
                            species: Species::Machop,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Onix,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 19,
                            species: Species::Geodude,
                        },
                        TrainerMon {
                            level: 19,
                            species: Species::Machop,
                        },
                        TrainerMon {
                            level: 19,
                            species: Species::Geodude,
                        },
                        TrainerMon {
                            level: 19,
                            species: Species::Geodude,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 20,
                            species: Species::Onix,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Onix,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Geodude,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 21,
                            species: Species::Geodude,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Graveler,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Biker,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Koffing,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Grimer,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 25,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Weezing,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Grimer,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Grimer,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Weezing,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Grimer,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Koffing,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 33,
                        species: Species::Weezing,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 26,
                            species: Species::Grimer,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Grimer,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Grimer,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Grimer,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Weezing,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Weezing,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 33,
                        species: Species::Muk,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Voltorb,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Voltorb,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Weezing,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Muk,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 25,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Weezing,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Weezing,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 26,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Grimer,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Koffing,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Grimer,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Grimer,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Koffing,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Muk,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Burglar,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Growlithe,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Vulpix,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 33,
                        species: Species::Growlithe,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Vulpix,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Charmander,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Ponyta,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 36,
                            species: Species::Growlithe,
                        },
                        TrainerMon {
                            level: 36,
                            species: Species::Vulpix,
                        },
                        TrainerMon {
                            level: 36,
                            species: Species::Ninetales,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 41,
                        species: Species::Ponyta,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 37,
                            species: Species::Vulpix,
                        },
                        TrainerMon {
                            level: 37,
                            species: Species::Growlithe,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 34,
                            species: Species::Charmander,
                        },
                        TrainerMon {
                            level: 34,
                            species: Species::Charmeleon,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 38,
                        species: Species::Ninetales,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 34,
                            species: Species::Growlithe,
                        },
                        TrainerMon {
                            level: 34,
                            species: Species::Ponyta,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Engineer,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 21,
                            species: Species::Voltorb,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Magnemite,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 21,
                        species: Species::Magnemite,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 18,
                            species: Species::Magnemite,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Magnemite,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Magneton,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::UnusedJuggler,
            parties: vec![],
        },
        TrainerClassData {
            class: TrainerClass::Fisher,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 17,
                            species: Species::Goldeen,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Tentacool,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Goldeen,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 17,
                            species: Species::Tentacool,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Staryu,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Shellder,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 22,
                            species: Species::Goldeen,
                        },
                        TrainerMon {
                            level: 22,
                            species: Species::Poliwag,
                        },
                        TrainerMon {
                            level: 22,
                            species: Species::Goldeen,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 24,
                            species: Species::Tentacool,
                        },
                        TrainerMon {
                            level: 24,
                            species: Species::Goldeen,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 27,
                        species: Species::Goldeen,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 21,
                            species: Species::Poliwag,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Shellder,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Goldeen,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Horsea,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Seaking,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Goldeen,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Seaking,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Seaking,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 31,
                            species: Species::Shellder,
                        },
                        TrainerMon {
                            level: 31,
                            species: Species::Cloyster,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 27,
                            species: Species::Magikarp,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Magikarp,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Magikarp,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Magikarp,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Magikarp,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Magikarp,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 33,
                            species: Species::Seaking,
                        },
                        TrainerMon {
                            level: 33,
                            species: Species::Goldeen,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 24,
                            species: Species::Magikarp,
                        },
                        TrainerMon {
                            level: 24,
                            species: Species::Magikarp,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Swimmer,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 16,
                            species: Species::Horsea,
                        },
                        TrainerMon {
                            level: 16,
                            species: Species::Shellder,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 30,
                            species: Species::Tentacool,
                        },
                        TrainerMon {
                            level: 30,
                            species: Species::Shellder,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Goldeen,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Horsea,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Staryu,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 30,
                            species: Species::Poliwag,
                        },
                        TrainerMon {
                            level: 30,
                            species: Species::Poliwhirl,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 27,
                            species: Species::Horsea,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Tentacool,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Tentacool,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Goldeen,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Goldeen,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Shellder,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Seaking,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 30,
                            species: Species::Horsea,
                        },
                        TrainerMon {
                            level: 30,
                            species: Species::Horsea,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 27,
                            species: Species::Tentacool,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Tentacool,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Staryu,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Horsea,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Tentacruel,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 31,
                            species: Species::Shellder,
                        },
                        TrainerMon {
                            level: 31,
                            species: Species::Cloyster,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 35,
                        species: Species::Staryu,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Horsea,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Horsea,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Seadra,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Horsea,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 33,
                            species: Species::Seadra,
                        },
                        TrainerMon {
                            level: 33,
                            species: Species::Tentacruel,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 37,
                        species: Species::Starmie,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 33,
                            species: Species::Staryu,
                        },
                        TrainerMon {
                            level: 33,
                            species: Species::Wartortle,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 32,
                            species: Species::Poliwhirl,
                        },
                        TrainerMon {
                            level: 32,
                            species: Species::Tentacool,
                        },
                        TrainerMon {
                            level: 32,
                            species: Species::Seadra,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::CueBall,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Machop,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Mankey,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Machop,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Mankey,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Machop,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 33,
                        species: Species::Machop,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Mankey,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Primeape,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Machop,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Machoke,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 33,
                        species: Species::Machoke,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 26,
                            species: Species::Mankey,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Mankey,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Machoke,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Machop,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Primeape,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Machoke,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 31,
                            species: Species::Tentacool,
                        },
                        TrainerMon {
                            level: 31,
                            species: Species::Tentacool,
                        },
                        TrainerMon {
                            level: 31,
                            species: Species::Tentacruel,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Gambler,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 18,
                            species: Species::Poliwag,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Horsea,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 18,
                            species: Species::Bellsprout,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Oddish,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 18,
                            species: Species::Voltorb,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Magnemite,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 18,
                            species: Species::Growlithe,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Vulpix,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 22,
                            species: Species::Poliwag,
                        },
                        TrainerMon {
                            level: 22,
                            species: Species::Poliwag,
                        },
                        TrainerMon {
                            level: 22,
                            species: Species::Poliwhirl,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 22,
                            species: Species::Onix,
                        },
                        TrainerMon {
                            level: 22,
                            species: Species::Geodude,
                        },
                        TrainerMon {
                            level: 22,
                            species: Species::Graveler,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 24,
                            species: Species::Growlithe,
                        },
                        TrainerMon {
                            level: 24,
                            species: Species::Vulpix,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Beauty,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 21,
                            species: Species::Oddish,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Bellsprout,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Oddish,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Bellsprout,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 24,
                            species: Species::Bellsprout,
                        },
                        TrainerMon {
                            level: 24,
                            species: Species::Bellsprout,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 26,
                        species: Species::Exeggcute,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 27,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Pikachu,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Rattata,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Clefairy,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Meowth,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 35,
                        species: Species::Seaking,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 30,
                            species: Species::Shellder,
                        },
                        TrainerMon {
                            level: 30,
                            species: Species::Shellder,
                        },
                        TrainerMon {
                            level: 30,
                            species: Species::Cloyster,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 31,
                            species: Species::Poliwag,
                        },
                        TrainerMon {
                            level: 31,
                            species: Species::Seaking,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Pidgeotto,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Wigglytuff,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Bulbasaur,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Ivysaur,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 33,
                            species: Species::Weepinbell,
                        },
                        TrainerMon {
                            level: 33,
                            species: Species::Bellsprout,
                        },
                        TrainerMon {
                            level: 33,
                            species: Species::Weepinbell,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 27,
                            species: Species::Poliwag,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Goldeen,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Seaking,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Goldeen,
                        },
                        TrainerMon {
                            level: 27,
                            species: Species::Poliwag,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 30,
                            species: Species::Goldeen,
                        },
                        TrainerMon {
                            level: 30,
                            species: Species::Seaking,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Staryu,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Staryu,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Staryu,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 30,
                            species: Species::Seadra,
                        },
                        TrainerMon {
                            level: 30,
                            species: Species::Horsea,
                        },
                        TrainerMon {
                            level: 30,
                            species: Species::Seadra,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::PsychicTr,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 31,
                            species: Species::Kadabra,
                        },
                        TrainerMon {
                            level: 31,
                            species: Species::Slowpoke,
                        },
                        TrainerMon {
                            level: 31,
                            species: Species::MrMime,
                        },
                        TrainerMon {
                            level: 31,
                            species: Species::Kadabra,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 34,
                            species: Species::MrMime,
                        },
                        TrainerMon {
                            level: 34,
                            species: Species::Kadabra,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 33,
                            species: Species::Slowpoke,
                        },
                        TrainerMon {
                            level: 33,
                            species: Species::Slowpoke,
                        },
                        TrainerMon {
                            level: 33,
                            species: Species::Slowbro,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 38,
                        species: Species::Slowbro,
                    }],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Rocker,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 20,
                            species: Species::Voltorb,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Magnemite,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Voltorb,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Voltorb,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Electrode,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Juggler,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Kadabra,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::MrMime,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 41,
                            species: Species::Drowzee,
                        },
                        TrainerMon {
                            level: 41,
                            species: Species::Hypno,
                        },
                        TrainerMon {
                            level: 41,
                            species: Species::Kadabra,
                        },
                        TrainerMon {
                            level: 41,
                            species: Species::Kadabra,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 31,
                            species: Species::Drowzee,
                        },
                        TrainerMon {
                            level: 31,
                            species: Species::Drowzee,
                        },
                        TrainerMon {
                            level: 31,
                            species: Species::Kadabra,
                        },
                        TrainerMon {
                            level: 31,
                            species: Species::Drowzee,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 34,
                            species: Species::Drowzee,
                        },
                        TrainerMon {
                            level: 34,
                            species: Species::Hypno,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 48,
                        species: Species::MrMime,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 33,
                        species: Species::Hypno,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 38,
                        species: Species::Hypno,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 34,
                            species: Species::Drowzee,
                        },
                        TrainerMon {
                            level: 34,
                            species: Species::Kadabra,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Tamer,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 34,
                            species: Species::Sandslash,
                        },
                        TrainerMon {
                            level: 34,
                            species: Species::Arbok,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 33,
                            species: Species::Arbok,
                        },
                        TrainerMon {
                            level: 33,
                            species: Species::Sandslash,
                        },
                        TrainerMon {
                            level: 33,
                            species: Species::Arbok,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 43,
                        species: Species::Rhyhorn,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 39,
                            species: Species::Arbok,
                        },
                        TrainerMon {
                            level: 39,
                            species: Species::Tauros,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 44,
                            species: Species::Persian,
                        },
                        TrainerMon {
                            level: 44,
                            species: Species::Golduck,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 42,
                            species: Species::Rhyhorn,
                        },
                        TrainerMon {
                            level: 42,
                            species: Species::Primeape,
                        },
                        TrainerMon {
                            level: 42,
                            species: Species::Arbok,
                        },
                        TrainerMon {
                            level: 42,
                            species: Species::Tauros,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::BirdKeeper,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Pidgeotto,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 25,
                            species: Species::Spearow,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Spearow,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Spearow,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 26,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Pidgeotto,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Spearow,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Fearow,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 33,
                        species: Species::Farfetchd,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Spearow,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Fearow,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 26,
                            species: Species::Pidgeotto,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Farfetchd,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Doduo,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Pidgey,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Dodrio,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Doduo,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Doduo,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Spearow,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Fearow,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 34,
                        species: Species::Dodrio,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 26,
                            species: Species::Spearow,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Spearow,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Fearow,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Spearow,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 30,
                            species: Species::Fearow,
                        },
                        TrainerMon {
                            level: 30,
                            species: Species::Fearow,
                        },
                        TrainerMon {
                            level: 30,
                            species: Species::Pidgeotto,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 39,
                            species: Species::Pidgeotto,
                        },
                        TrainerMon {
                            level: 39,
                            species: Species::Pidgeotto,
                        },
                        TrainerMon {
                            level: 39,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 39,
                            species: Species::Pidgeotto,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 42,
                            species: Species::Farfetchd,
                        },
                        TrainerMon {
                            level: 42,
                            species: Species::Fearow,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Doduo,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Pidgeotto,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 26,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Spearow,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Fearow,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Pidgeotto,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Fearow,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Spearow,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Doduo,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Fearow,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Blackbelt,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 37,
                            species: Species::Hitmonlee,
                        },
                        TrainerMon {
                            level: 37,
                            species: Species::Hitmonchan,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 31,
                            species: Species::Mankey,
                        },
                        TrainerMon {
                            level: 31,
                            species: Species::Mankey,
                        },
                        TrainerMon {
                            level: 31,
                            species: Species::Primeape,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 32,
                            species: Species::Machop,
                        },
                        TrainerMon {
                            level: 32,
                            species: Species::Machoke,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 36,
                        species: Species::Primeape,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 31,
                            species: Species::Machop,
                        },
                        TrainerMon {
                            level: 31,
                            species: Species::Mankey,
                        },
                        TrainerMon {
                            level: 31,
                            species: Species::Primeape,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 40,
                            species: Species::Machop,
                        },
                        TrainerMon {
                            level: 40,
                            species: Species::Machoke,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 43,
                        species: Species::Machoke,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 38,
                            species: Species::Machoke,
                        },
                        TrainerMon {
                            level: 38,
                            species: Species::Machop,
                        },
                        TrainerMon {
                            level: 38,
                            species: Species::Machoke,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 43,
                            species: Species::Machoke,
                        },
                        TrainerMon {
                            level: 43,
                            species: Species::Machop,
                        },
                        TrainerMon {
                            level: 43,
                            species: Species::Machoke,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Rival1,
            parties: vec![
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 5,
                        species: Species::Squirtle,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 5,
                        species: Species::Bulbasaur,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 5,
                        species: Species::Charmander,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 9,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 8,
                            species: Species::Squirtle,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 9,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 8,
                            species: Species::Bulbasaur,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 9,
                            species: Species::Pidgey,
                        },
                        TrainerMon {
                            level: 8,
                            species: Species::Charmander,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 18,
                            species: Species::Pidgeotto,
                        },
                        TrainerMon {
                            level: 15,
                            species: Species::Abra,
                        },
                        TrainerMon {
                            level: 15,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Squirtle,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 18,
                            species: Species::Pidgeotto,
                        },
                        TrainerMon {
                            level: 15,
                            species: Species::Abra,
                        },
                        TrainerMon {
                            level: 15,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Bulbasaur,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 18,
                            species: Species::Pidgeotto,
                        },
                        TrainerMon {
                            level: 15,
                            species: Species::Abra,
                        },
                        TrainerMon {
                            level: 15,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Charmander,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::ProfOak,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 66,
                            species: Species::Tauros,
                        },
                        TrainerMon {
                            level: 67,
                            species: Species::Exeggutor,
                        },
                        TrainerMon {
                            level: 68,
                            species: Species::Arcanine,
                        },
                        TrainerMon {
                            level: 69,
                            species: Species::Blastoise,
                        },
                        TrainerMon {
                            level: 70,
                            species: Species::Gyarados,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 66,
                            species: Species::Tauros,
                        },
                        TrainerMon {
                            level: 67,
                            species: Species::Exeggutor,
                        },
                        TrainerMon {
                            level: 68,
                            species: Species::Arcanine,
                        },
                        TrainerMon {
                            level: 69,
                            species: Species::Venusaur,
                        },
                        TrainerMon {
                            level: 70,
                            species: Species::Gyarados,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 66,
                            species: Species::Tauros,
                        },
                        TrainerMon {
                            level: 67,
                            species: Species::Exeggutor,
                        },
                        TrainerMon {
                            level: 68,
                            species: Species::Arcanine,
                        },
                        TrainerMon {
                            level: 69,
                            species: Species::Charizard,
                        },
                        TrainerMon {
                            level: 70,
                            species: Species::Gyarados,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Chief,
            parties: vec![],
        },
        TrainerClassData {
            class: TrainerClass::Scientist,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 34,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 34,
                            species: Species::Voltorb,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 26,
                            species: Species::Grimer,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Weezing,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Weezing,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Magnemite,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Voltorb,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Magneton,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Electrode,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Weezing,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 33,
                        species: Species::Electrode,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 26,
                            species: Species::Magneton,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Weezing,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Magnemite,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 25,
                            species: Species::Voltorb,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Magneton,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Magnemite,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Koffing,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Electrode,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Muk,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Grimer,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Electrode,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Voltorb,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Magneton,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Magnemite,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Koffing,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 33,
                            species: Species::Magnemite,
                        },
                        TrainerMon {
                            level: 33,
                            species: Species::Magneton,
                        },
                        TrainerMon {
                            level: 33,
                            species: Species::Voltorb,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 34,
                            species: Species::Magnemite,
                        },
                        TrainerMon {
                            level: 34,
                            species: Species::Electrode,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Giovanni,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 25,
                            species: Species::Onix,
                        },
                        TrainerMon {
                            level: 24,
                            species: Species::Rhyhorn,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Kangaskhan,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 37,
                            species: Species::Nidorino,
                        },
                        TrainerMon {
                            level: 35,
                            species: Species::Kangaskhan,
                        },
                        TrainerMon {
                            level: 37,
                            species: Species::Rhyhorn,
                        },
                        TrainerMon {
                            level: 41,
                            species: Species::Nidoqueen,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 45,
                            species: Species::Rhyhorn,
                        },
                        TrainerMon {
                            level: 42,
                            species: Species::Dugtrio,
                        },
                        TrainerMon {
                            level: 44,
                            species: Species::Nidoqueen,
                        },
                        TrainerMon {
                            level: 45,
                            species: Species::Nidoking,
                        },
                        TrainerMon {
                            level: 50,
                            species: Species::Rhydon,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Rocket,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 13,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 13,
                            species: Species::Zubat,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 11,
                            species: Species::Sandshrew,
                        },
                        TrainerMon {
                            level: 11,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 11,
                            species: Species::Zubat,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 12,
                            species: Species::Zubat,
                        },
                        TrainerMon {
                            level: 12,
                            species: Species::Ekans,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 16,
                        species: Species::Raticate,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 17,
                            species: Species::Machop,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Drowzee,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 15,
                            species: Species::Ekans,
                        },
                        TrainerMon {
                            level: 15,
                            species: Species::Zubat,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 20,
                            species: Species::Raticate,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Zubat,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 21,
                            species: Species::Drowzee,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Machop,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 21,
                            species: Species::Raticate,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Raticate,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 20,
                            species: Species::Grimer,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Koffing,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 19,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 19,
                            species: Species::Raticate,
                        },
                        TrainerMon {
                            level: 19,
                            species: Species::Raticate,
                        },
                        TrainerMon {
                            level: 19,
                            species: Species::Rattata,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 22,
                            species: Species::Grimer,
                        },
                        TrainerMon {
                            level: 22,
                            species: Species::Koffing,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 17,
                            species: Species::Zubat,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Grimer,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Zubat,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Raticate,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 20,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Raticate,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Drowzee,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 21,
                            species: Species::Machop,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Machop,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 23,
                            species: Species::Sandshrew,
                        },
                        TrainerMon {
                            level: 23,
                            species: Species::Ekans,
                        },
                        TrainerMon {
                            level: 23,
                            species: Species::Sandslash,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 23,
                            species: Species::Ekans,
                        },
                        TrainerMon {
                            level: 23,
                            species: Species::Sandshrew,
                        },
                        TrainerMon {
                            level: 23,
                            species: Species::Arbok,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 21,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 21,
                            species: Species::Zubat,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 25,
                            species: Species::Zubat,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Zubat,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Golbat,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 26,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Drowzee,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 23,
                            species: Species::Zubat,
                        },
                        TrainerMon {
                            level: 23,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 23,
                            species: Species::Raticate,
                        },
                        TrainerMon {
                            level: 23,
                            species: Species::Zubat,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 26,
                            species: Species::Drowzee,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Koffing,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Cubone,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Zubat,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 25,
                            species: Species::Golbat,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Zubat,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Zubat,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Raticate,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Zubat,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Raticate,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Hypno,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Raticate,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Machop,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Drowzee,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Ekans,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Zubat,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Cubone,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 33,
                        species: Species::Arbok,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 33,
                        species: Species::Hypno,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Machop,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Machoke,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Zubat,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Zubat,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Golbat,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 26,
                            species: Species::Raticate,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Arbok,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Koffing,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Golbat,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Cubone,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Cubone,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 29,
                            species: Species::Sandshrew,
                        },
                        TrainerMon {
                            level: 29,
                            species: Species::Sandslash,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 26,
                            species: Species::Raticate,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Zubat,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Golbat,
                        },
                        TrainerMon {
                            level: 26,
                            species: Species::Rattata,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Weezing,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Golbat,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Koffing,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Drowzee,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Grimer,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Machop,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 28,
                            species: Species::Golbat,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Drowzee,
                        },
                        TrainerMon {
                            level: 28,
                            species: Species::Hypno,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 33,
                        species: Species::Machoke,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 25,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Zubat,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Rattata,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Ekans,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 32,
                            species: Species::Cubone,
                        },
                        TrainerMon {
                            level: 32,
                            species: Species::Drowzee,
                        },
                        TrainerMon {
                            level: 32,
                            species: Species::Marowak,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::CooltrainerM,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 39,
                            species: Species::Nidorino,
                        },
                        TrainerMon {
                            level: 39,
                            species: Species::Nidoking,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 43,
                            species: Species::Exeggutor,
                        },
                        TrainerMon {
                            level: 43,
                            species: Species::Cloyster,
                        },
                        TrainerMon {
                            level: 43,
                            species: Species::Arcanine,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 43,
                            species: Species::Kingler,
                        },
                        TrainerMon {
                            level: 43,
                            species: Species::Tentacruel,
                        },
                        TrainerMon {
                            level: 43,
                            species: Species::Blastoise,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 45,
                            species: Species::Kingler,
                        },
                        TrainerMon {
                            level: 45,
                            species: Species::Starmie,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 42,
                            species: Species::Ivysaur,
                        },
                        TrainerMon {
                            level: 42,
                            species: Species::Wartortle,
                        },
                        TrainerMon {
                            level: 42,
                            species: Species::Charmeleon,
                        },
                        TrainerMon {
                            level: 42,
                            species: Species::Charizard,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 44,
                            species: Species::Ivysaur,
                        },
                        TrainerMon {
                            level: 44,
                            species: Species::Wartortle,
                        },
                        TrainerMon {
                            level: 44,
                            species: Species::Charmeleon,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 49,
                        species: Species::Nidoking,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 44,
                            species: Species::Kingler,
                        },
                        TrainerMon {
                            level: 44,
                            species: Species::Cloyster,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 39,
                            species: Species::Sandslash,
                        },
                        TrainerMon {
                            level: 39,
                            species: Species::Dugtrio,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 43,
                        species: Species::Rhyhorn,
                    }],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::CooltrainerF,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 24,
                            species: Species::Weepinbell,
                        },
                        TrainerMon {
                            level: 24,
                            species: Species::Gloom,
                        },
                        TrainerMon {
                            level: 24,
                            species: Species::Ivysaur,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 43,
                            species: Species::Bellsprout,
                        },
                        TrainerMon {
                            level: 43,
                            species: Species::Weepinbell,
                        },
                        TrainerMon {
                            level: 43,
                            species: Species::Victreebel,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 43,
                            species: Species::Parasect,
                        },
                        TrainerMon {
                            level: 43,
                            species: Species::Dewgong,
                        },
                        TrainerMon {
                            level: 43,
                            species: Species::Chansey,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 46,
                            species: Species::Vileplume,
                        },
                        TrainerMon {
                            level: 46,
                            species: Species::Butterfree,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 44,
                            species: Species::Persian,
                        },
                        TrainerMon {
                            level: 44,
                            species: Species::Ninetales,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 45,
                            species: Species::Ivysaur,
                        },
                        TrainerMon {
                            level: 45,
                            species: Species::Venusaur,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 45,
                            species: Species::Nidorina,
                        },
                        TrainerMon {
                            level: 45,
                            species: Species::Nidoqueen,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 43,
                            species: Species::Persian,
                        },
                        TrainerMon {
                            level: 43,
                            species: Species::Ninetales,
                        },
                        TrainerMon {
                            level: 43,
                            species: Species::Raichu,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Bruno,
            parties: vec![TrainerParty {
                pokemon: vec![
                    TrainerMon {
                        level: 53,
                        species: Species::Onix,
                    },
                    TrainerMon {
                        level: 55,
                        species: Species::Hitmonchan,
                    },
                    TrainerMon {
                        level: 55,
                        species: Species::Hitmonlee,
                    },
                    TrainerMon {
                        level: 56,
                        species: Species::Onix,
                    },
                    TrainerMon {
                        level: 58,
                        species: Species::Machamp,
                    },
                ],
            }],
        },
        TrainerClassData {
            class: TrainerClass::Brock,
            parties: vec![TrainerParty {
                pokemon: vec![
                    TrainerMon {
                        level: 12,
                        species: Species::Geodude,
                    },
                    TrainerMon {
                        level: 14,
                        species: Species::Onix,
                    },
                ],
            }],
        },
        TrainerClassData {
            class: TrainerClass::Misty,
            parties: vec![TrainerParty {
                pokemon: vec![
                    TrainerMon {
                        level: 18,
                        species: Species::Staryu,
                    },
                    TrainerMon {
                        level: 21,
                        species: Species::Starmie,
                    },
                ],
            }],
        },
        TrainerClassData {
            class: TrainerClass::LtSurge,
            parties: vec![TrainerParty {
                pokemon: vec![
                    TrainerMon {
                        level: 21,
                        species: Species::Voltorb,
                    },
                    TrainerMon {
                        level: 18,
                        species: Species::Pikachu,
                    },
                    TrainerMon {
                        level: 24,
                        species: Species::Raichu,
                    },
                ],
            }],
        },
        TrainerClassData {
            class: TrainerClass::Erika,
            parties: vec![TrainerParty {
                pokemon: vec![
                    TrainerMon {
                        level: 29,
                        species: Species::Victreebel,
                    },
                    TrainerMon {
                        level: 24,
                        species: Species::Tangela,
                    },
                    TrainerMon {
                        level: 29,
                        species: Species::Vileplume,
                    },
                ],
            }],
        },
        TrainerClassData {
            class: TrainerClass::Koga,
            parties: vec![TrainerParty {
                pokemon: vec![
                    TrainerMon {
                        level: 37,
                        species: Species::Koffing,
                    },
                    TrainerMon {
                        level: 39,
                        species: Species::Muk,
                    },
                    TrainerMon {
                        level: 37,
                        species: Species::Koffing,
                    },
                    TrainerMon {
                        level: 43,
                        species: Species::Weezing,
                    },
                ],
            }],
        },
        TrainerClassData {
            class: TrainerClass::Blaine,
            parties: vec![TrainerParty {
                pokemon: vec![
                    TrainerMon {
                        level: 42,
                        species: Species::Growlithe,
                    },
                    TrainerMon {
                        level: 40,
                        species: Species::Ponyta,
                    },
                    TrainerMon {
                        level: 42,
                        species: Species::Rapidash,
                    },
                    TrainerMon {
                        level: 47,
                        species: Species::Arcanine,
                    },
                ],
            }],
        },
        TrainerClassData {
            class: TrainerClass::Sabrina,
            parties: vec![TrainerParty {
                pokemon: vec![
                    TrainerMon {
                        level: 38,
                        species: Species::Kadabra,
                    },
                    TrainerMon {
                        level: 37,
                        species: Species::MrMime,
                    },
                    TrainerMon {
                        level: 38,
                        species: Species::Venomoth,
                    },
                    TrainerMon {
                        level: 43,
                        species: Species::Alakazam,
                    },
                ],
            }],
        },
        TrainerClassData {
            class: TrainerClass::Gentleman,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 18,
                            species: Species::Growlithe,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Growlithe,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 19,
                            species: Species::NidoranM,
                        },
                        TrainerMon {
                            level: 19,
                            species: Species::NidoranF,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 23,
                        species: Species::Pikachu,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 48,
                        species: Species::Primeape,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 17,
                            species: Species::Growlithe,
                        },
                        TrainerMon {
                            level: 17,
                            species: Species::Ponyta,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Rival2,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 19,
                            species: Species::Pidgeotto,
                        },
                        TrainerMon {
                            level: 16,
                            species: Species::Raticate,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Kadabra,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Wartortle,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 19,
                            species: Species::Pidgeotto,
                        },
                        TrainerMon {
                            level: 16,
                            species: Species::Raticate,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Kadabra,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Ivysaur,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 19,
                            species: Species::Pidgeotto,
                        },
                        TrainerMon {
                            level: 16,
                            species: Species::Raticate,
                        },
                        TrainerMon {
                            level: 18,
                            species: Species::Kadabra,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Charmeleon,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 25,
                            species: Species::Pidgeotto,
                        },
                        TrainerMon {
                            level: 23,
                            species: Species::Growlithe,
                        },
                        TrainerMon {
                            level: 22,
                            species: Species::Exeggcute,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Kadabra,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Wartortle,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 25,
                            species: Species::Pidgeotto,
                        },
                        TrainerMon {
                            level: 23,
                            species: Species::Gyarados,
                        },
                        TrainerMon {
                            level: 22,
                            species: Species::Growlithe,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Kadabra,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Ivysaur,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 25,
                            species: Species::Pidgeotto,
                        },
                        TrainerMon {
                            level: 23,
                            species: Species::Exeggcute,
                        },
                        TrainerMon {
                            level: 22,
                            species: Species::Gyarados,
                        },
                        TrainerMon {
                            level: 20,
                            species: Species::Kadabra,
                        },
                        TrainerMon {
                            level: 25,
                            species: Species::Charmeleon,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 37,
                            species: Species::Pidgeot,
                        },
                        TrainerMon {
                            level: 38,
                            species: Species::Growlithe,
                        },
                        TrainerMon {
                            level: 35,
                            species: Species::Exeggcute,
                        },
                        TrainerMon {
                            level: 35,
                            species: Species::Alakazam,
                        },
                        TrainerMon {
                            level: 40,
                            species: Species::Blastoise,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 37,
                            species: Species::Pidgeot,
                        },
                        TrainerMon {
                            level: 38,
                            species: Species::Gyarados,
                        },
                        TrainerMon {
                            level: 35,
                            species: Species::Growlithe,
                        },
                        TrainerMon {
                            level: 35,
                            species: Species::Alakazam,
                        },
                        TrainerMon {
                            level: 40,
                            species: Species::Venusaur,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 37,
                            species: Species::Pidgeot,
                        },
                        TrainerMon {
                            level: 38,
                            species: Species::Exeggcute,
                        },
                        TrainerMon {
                            level: 35,
                            species: Species::Gyarados,
                        },
                        TrainerMon {
                            level: 35,
                            species: Species::Alakazam,
                        },
                        TrainerMon {
                            level: 40,
                            species: Species::Charizard,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 47,
                            species: Species::Pidgeot,
                        },
                        TrainerMon {
                            level: 45,
                            species: Species::Rhyhorn,
                        },
                        TrainerMon {
                            level: 45,
                            species: Species::Growlithe,
                        },
                        TrainerMon {
                            level: 47,
                            species: Species::Exeggcute,
                        },
                        TrainerMon {
                            level: 50,
                            species: Species::Alakazam,
                        },
                        TrainerMon {
                            level: 53,
                            species: Species::Blastoise,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 47,
                            species: Species::Pidgeot,
                        },
                        TrainerMon {
                            level: 45,
                            species: Species::Rhyhorn,
                        },
                        TrainerMon {
                            level: 45,
                            species: Species::Gyarados,
                        },
                        TrainerMon {
                            level: 47,
                            species: Species::Growlithe,
                        },
                        TrainerMon {
                            level: 50,
                            species: Species::Alakazam,
                        },
                        TrainerMon {
                            level: 53,
                            species: Species::Venusaur,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 47,
                            species: Species::Pidgeot,
                        },
                        TrainerMon {
                            level: 45,
                            species: Species::Rhyhorn,
                        },
                        TrainerMon {
                            level: 45,
                            species: Species::Exeggcute,
                        },
                        TrainerMon {
                            level: 47,
                            species: Species::Gyarados,
                        },
                        TrainerMon {
                            level: 50,
                            species: Species::Alakazam,
                        },
                        TrainerMon {
                            level: 53,
                            species: Species::Charizard,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Rival3,
            parties: vec![
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 61,
                            species: Species::Pidgeot,
                        },
                        TrainerMon {
                            level: 59,
                            species: Species::Alakazam,
                        },
                        TrainerMon {
                            level: 61,
                            species: Species::Rhydon,
                        },
                        TrainerMon {
                            level: 61,
                            species: Species::Arcanine,
                        },
                        TrainerMon {
                            level: 63,
                            species: Species::Exeggutor,
                        },
                        TrainerMon {
                            level: 65,
                            species: Species::Blastoise,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 61,
                            species: Species::Pidgeot,
                        },
                        TrainerMon {
                            level: 59,
                            species: Species::Alakazam,
                        },
                        TrainerMon {
                            level: 61,
                            species: Species::Rhydon,
                        },
                        TrainerMon {
                            level: 61,
                            species: Species::Gyarados,
                        },
                        TrainerMon {
                            level: 63,
                            species: Species::Arcanine,
                        },
                        TrainerMon {
                            level: 65,
                            species: Species::Venusaur,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 61,
                            species: Species::Pidgeot,
                        },
                        TrainerMon {
                            level: 59,
                            species: Species::Alakazam,
                        },
                        TrainerMon {
                            level: 61,
                            species: Species::Rhydon,
                        },
                        TrainerMon {
                            level: 61,
                            species: Species::Exeggutor,
                        },
                        TrainerMon {
                            level: 63,
                            species: Species::Gyarados,
                        },
                        TrainerMon {
                            level: 65,
                            species: Species::Charizard,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Lorelei,
            parties: vec![TrainerParty {
                pokemon: vec![
                    TrainerMon {
                        level: 54,
                        species: Species::Dewgong,
                    },
                    TrainerMon {
                        level: 53,
                        species: Species::Cloyster,
                    },
                    TrainerMon {
                        level: 54,
                        species: Species::Slowbro,
                    },
                    TrainerMon {
                        level: 56,
                        species: Species::Jynx,
                    },
                    TrainerMon {
                        level: 56,
                        species: Species::Lapras,
                    },
                ],
            }],
        },
        TrainerClassData {
            class: TrainerClass::Channeler,
            parties: vec![
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 22,
                        species: Species::Gastly,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 24,
                        species: Species::Gastly,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 23,
                            species: Species::Gastly,
                        },
                        TrainerMon {
                            level: 23,
                            species: Species::Gastly,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 24,
                        species: Species::Gastly,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 23,
                        species: Species::Gastly,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 24,
                        species: Species::Gastly,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 24,
                        species: Species::Haunter,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 22,
                        species: Species::Gastly,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 24,
                        species: Species::Gastly,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 23,
                            species: Species::Gastly,
                        },
                        TrainerMon {
                            level: 23,
                            species: Species::Gastly,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 24,
                        species: Species::Gastly,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 22,
                        species: Species::Gastly,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 24,
                        species: Species::Gastly,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 23,
                        species: Species::Haunter,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 24,
                        species: Species::Gastly,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 22,
                        species: Species::Gastly,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 24,
                        species: Species::Gastly,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 22,
                        species: Species::Haunter,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 22,
                            species: Species::Gastly,
                        },
                        TrainerMon {
                            level: 22,
                            species: Species::Gastly,
                        },
                        TrainerMon {
                            level: 22,
                            species: Species::Gastly,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 24,
                        species: Species::Gastly,
                    }],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 24,
                        species: Species::Gastly,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 34,
                            species: Species::Gastly,
                        },
                        TrainerMon {
                            level: 34,
                            species: Species::Haunter,
                        },
                    ],
                },
                TrainerParty {
                    pokemon: vec![TrainerMon {
                        level: 38,
                        species: Species::Haunter,
                    }],
                },
                TrainerParty {
                    pokemon: vec![
                        TrainerMon {
                            level: 33,
                            species: Species::Gastly,
                        },
                        TrainerMon {
                            level: 33,
                            species: Species::Gastly,
                        },
                        TrainerMon {
                            level: 33,
                            species: Species::Haunter,
                        },
                    ],
                },
            ],
        },
        TrainerClassData {
            class: TrainerClass::Agatha,
            parties: vec![TrainerParty {
                pokemon: vec![
                    TrainerMon {
                        level: 56,
                        species: Species::Gengar,
                    },
                    TrainerMon {
                        level: 56,
                        species: Species::Golbat,
                    },
                    TrainerMon {
                        level: 55,
                        species: Species::Haunter,
                    },
                    TrainerMon {
                        level: 58,
                        species: Species::Arbok,
                    },
                    TrainerMon {
                        level: 60,
                        species: Species::Gengar,
                    },
                ],
            }],
        },
        TrainerClassData {
            class: TrainerClass::Lance,
            parties: vec![TrainerParty {
                pokemon: vec![
                    TrainerMon {
                        level: 58,
                        species: Species::Gyarados,
                    },
                    TrainerMon {
                        level: 56,
                        species: Species::Dragonair,
                    },
                    TrainerMon {
                        level: 56,
                        species: Species::Dragonair,
                    },
                    TrainerMon {
                        level: 60,
                        species: Species::Aerodactyl,
                    },
                    TrainerMon {
                        level: 62,
                        species: Species::Dragonite,
                    },
                ],
            }],
        },
    ]
}

/// Base money reward per trainer class (in yen / currency units).
/// Prize money = base_money × level of last enemy Pokémon.
/// Data from data/trainers/pic_pointers_money.asm.
pub fn get_base_money(class: TrainerClass) -> u16 {
    match class {
        TrainerClass::Nobody => 0,
        TrainerClass::Youngster => 1500,
        TrainerClass::BugCatcher => 1000,
        TrainerClass::Lass => 1500,
        TrainerClass::Sailor => 3000,
        TrainerClass::JrTrainerM => 2000,
        TrainerClass::JrTrainerF => 2000,
        TrainerClass::Pokemaniac => 5000,
        TrainerClass::SuperNerd => 2500,
        TrainerClass::Hiker => 3500,
        TrainerClass::Biker => 2000,
        TrainerClass::Burglar => 9000,
        TrainerClass::Engineer => 5000,
        TrainerClass::UnusedJuggler => 3500,
        TrainerClass::Fisher => 3500,
        TrainerClass::Swimmer => 500,
        TrainerClass::CueBall => 2500,
        TrainerClass::Gambler => 7000,
        TrainerClass::Beauty => 7000,
        TrainerClass::PsychicTr => 1000,
        TrainerClass::Rocker => 2500,
        TrainerClass::Juggler => 3500,
        TrainerClass::Tamer => 4000,
        TrainerClass::BirdKeeper => 2500,
        TrainerClass::Blackbelt => 2500,
        TrainerClass::Rival1 => 3500,
        TrainerClass::ProfOak => 9900,
        TrainerClass::Chief => 3000,
        TrainerClass::Scientist => 5000,
        TrainerClass::Giovanni => 9900,
        TrainerClass::Rocket => 3000,
        TrainerClass::CooltrainerM => 3500,
        TrainerClass::CooltrainerF => 3500,
        TrainerClass::Bruno => 9900,
        TrainerClass::Brock => 9900,
        TrainerClass::Misty => 9900,
        TrainerClass::LtSurge => 9900,
        TrainerClass::Erika => 9900,
        TrainerClass::Koga => 9900,
        TrainerClass::Blaine => 9900,
        TrainerClass::Sabrina => 9900,
        TrainerClass::Gentleman => 7000,
        TrainerClass::Rival2 => 6500,
        TrainerClass::Rival3 => 9900,
        TrainerClass::Lorelei => 9900,
        TrainerClass::Channeler => 3000,
        TrainerClass::Agatha => 9900,
        TrainerClass::Lance => 9900,
    }
}
