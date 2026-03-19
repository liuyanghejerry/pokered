//! Evolution and level-up learnset data for all 151 Pokemon.
//! Auto-generated from data/pokemon/evos_moves.asm - do not edit manually.

use crate::items::ItemId;
use crate::moves::MoveId;
use crate::species::Species;
use serde::{Deserialize, Serialize};

/// Method by which a Pokemon can evolve
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvolutionMethod {
    /// Evolves at a specific level
    Level { level: u8, species: Species },
    /// Evolves when a specific item is used
    Item {
        item: ItemId,
        min_level: u8,
        species: Species,
    },
    /// Evolves when traded
    Trade { min_level: u8, species: Species },
}

/// A move learned at a specific level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LevelUpMove {
    pub level: u8,
    pub move_id: MoveId,
}

/// Evolution and learnset data for a single species
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvosMovesEntry {
    pub species: Species,
    pub evolutions: Vec<EvolutionMethod>,
    pub learnset: Vec<LevelUpMove>,
}

/// Get evolution and learnset data for all 151 Pokemon, ordered by dex number.
pub fn evos_moves_data() -> Vec<EvosMovesEntry> {
    vec![
        // #1 Bulbasaur
        EvosMovesEntry {
            species: Species::Bulbasaur,
            evolutions: vec![EvolutionMethod::Level {
                level: 16,
                species: Species::Ivysaur,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 7,
                    move_id: MoveId::LeechSeed,
                },
                LevelUpMove {
                    level: 13,
                    move_id: MoveId::VineWhip,
                },
                LevelUpMove {
                    level: 20,
                    move_id: MoveId::Poisonpowder,
                },
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::RazorLeaf,
                },
                LevelUpMove {
                    level: 34,
                    move_id: MoveId::Growth,
                },
                LevelUpMove {
                    level: 41,
                    move_id: MoveId::SleepPowder,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::Solarbeam,
                },
            ],
        },
        // #2 Ivysaur
        EvosMovesEntry {
            species: Species::Ivysaur,
            evolutions: vec![EvolutionMethod::Level {
                level: 32,
                species: Species::Venusaur,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 7,
                    move_id: MoveId::LeechSeed,
                },
                LevelUpMove {
                    level: 13,
                    move_id: MoveId::VineWhip,
                },
                LevelUpMove {
                    level: 22,
                    move_id: MoveId::Poisonpowder,
                },
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::RazorLeaf,
                },
                LevelUpMove {
                    level: 38,
                    move_id: MoveId::Growth,
                },
                LevelUpMove {
                    level: 46,
                    move_id: MoveId::SleepPowder,
                },
                LevelUpMove {
                    level: 54,
                    move_id: MoveId::Solarbeam,
                },
            ],
        },
        // #3 Venusaur
        EvosMovesEntry {
            species: Species::Venusaur,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 7,
                    move_id: MoveId::LeechSeed,
                },
                LevelUpMove {
                    level: 13,
                    move_id: MoveId::VineWhip,
                },
                LevelUpMove {
                    level: 22,
                    move_id: MoveId::Poisonpowder,
                },
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::RazorLeaf,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::Growth,
                },
                LevelUpMove {
                    level: 55,
                    move_id: MoveId::SleepPowder,
                },
                LevelUpMove {
                    level: 65,
                    move_id: MoveId::Solarbeam,
                },
            ],
        },
        // #4 Charmander
        EvosMovesEntry {
            species: Species::Charmander,
            evolutions: vec![EvolutionMethod::Level {
                level: 16,
                species: Species::Charmeleon,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 9,
                    move_id: MoveId::Ember,
                },
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 22,
                    move_id: MoveId::Rage,
                },
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::Slash,
                },
                LevelUpMove {
                    level: 38,
                    move_id: MoveId::Flamethrower,
                },
                LevelUpMove {
                    level: 46,
                    move_id: MoveId::FireSpin,
                },
            ],
        },
        // #5 Charmeleon
        EvosMovesEntry {
            species: Species::Charmeleon,
            evolutions: vec![EvolutionMethod::Level {
                level: 36,
                species: Species::Charizard,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 9,
                    move_id: MoveId::Ember,
                },
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::Rage,
                },
                LevelUpMove {
                    level: 33,
                    move_id: MoveId::Slash,
                },
                LevelUpMove {
                    level: 42,
                    move_id: MoveId::Flamethrower,
                },
                LevelUpMove {
                    level: 56,
                    move_id: MoveId::FireSpin,
                },
            ],
        },
        // #6 Charizard
        EvosMovesEntry {
            species: Species::Charizard,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 9,
                    move_id: MoveId::Ember,
                },
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::Rage,
                },
                LevelUpMove {
                    level: 36,
                    move_id: MoveId::Slash,
                },
                LevelUpMove {
                    level: 46,
                    move_id: MoveId::Flamethrower,
                },
                LevelUpMove {
                    level: 55,
                    move_id: MoveId::FireSpin,
                },
            ],
        },
        // #7 Squirtle
        EvosMovesEntry {
            species: Species::Squirtle,
            evolutions: vec![EvolutionMethod::Level {
                level: 16,
                species: Species::Wartortle,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 8,
                    move_id: MoveId::Bubble,
                },
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::WaterGun,
                },
                LevelUpMove {
                    level: 22,
                    move_id: MoveId::Bite,
                },
                LevelUpMove {
                    level: 28,
                    move_id: MoveId::Withdraw,
                },
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::SkullBash,
                },
                LevelUpMove {
                    level: 42,
                    move_id: MoveId::HydroPump,
                },
            ],
        },
        // #8 Wartortle
        EvosMovesEntry {
            species: Species::Wartortle,
            evolutions: vec![EvolutionMethod::Level {
                level: 36,
                species: Species::Blastoise,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 8,
                    move_id: MoveId::Bubble,
                },
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::WaterGun,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::Bite,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::Withdraw,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::SkullBash,
                },
                LevelUpMove {
                    level: 47,
                    move_id: MoveId::HydroPump,
                },
            ],
        },
        // #9 Blastoise
        EvosMovesEntry {
            species: Species::Blastoise,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 8,
                    move_id: MoveId::Bubble,
                },
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::WaterGun,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::Bite,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::Withdraw,
                },
                LevelUpMove {
                    level: 42,
                    move_id: MoveId::SkullBash,
                },
                LevelUpMove {
                    level: 52,
                    move_id: MoveId::HydroPump,
                },
            ],
        },
        // #10 Caterpie
        EvosMovesEntry {
            species: Species::Caterpie,
            evolutions: vec![EvolutionMethod::Level {
                level: 7,
                species: Species::Metapod,
            }],
            learnset: vec![],
        },
        // #11 Metapod
        EvosMovesEntry {
            species: Species::Metapod,
            evolutions: vec![EvolutionMethod::Level {
                level: 10,
                species: Species::Butterfree,
            }],
            learnset: vec![],
        },
        // #12 Butterfree
        EvosMovesEntry {
            species: Species::Butterfree,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 12,
                    move_id: MoveId::Confusion,
                },
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::Poisonpowder,
                },
                LevelUpMove {
                    level: 16,
                    move_id: MoveId::StunSpore,
                },
                LevelUpMove {
                    level: 17,
                    move_id: MoveId::SleepPowder,
                },
                LevelUpMove {
                    level: 21,
                    move_id: MoveId::Supersonic,
                },
                LevelUpMove {
                    level: 26,
                    move_id: MoveId::Whirlwind,
                },
                LevelUpMove {
                    level: 32,
                    move_id: MoveId::Psybeam,
                },
            ],
        },
        // #13 Weedle
        EvosMovesEntry {
            species: Species::Weedle,
            evolutions: vec![EvolutionMethod::Level {
                level: 7,
                species: Species::Kakuna,
            }],
            learnset: vec![],
        },
        // #14 Kakuna
        EvosMovesEntry {
            species: Species::Kakuna,
            evolutions: vec![EvolutionMethod::Level {
                level: 10,
                species: Species::Beedrill,
            }],
            learnset: vec![],
        },
        // #15 Beedrill
        EvosMovesEntry {
            species: Species::Beedrill,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 12,
                    move_id: MoveId::FuryAttack,
                },
                LevelUpMove {
                    level: 16,
                    move_id: MoveId::FocusEnergy,
                },
                LevelUpMove {
                    level: 20,
                    move_id: MoveId::Twineedle,
                },
                LevelUpMove {
                    level: 25,
                    move_id: MoveId::Rage,
                },
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::PinMissile,
                },
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::Agility,
                },
            ],
        },
        // #16 Pidgey
        EvosMovesEntry {
            species: Species::Pidgey,
            evolutions: vec![EvolutionMethod::Level {
                level: 18,
                species: Species::Pidgeotto,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 5,
                    move_id: MoveId::SandAttack,
                },
                LevelUpMove {
                    level: 12,
                    move_id: MoveId::QuickAttack,
                },
                LevelUpMove {
                    level: 19,
                    move_id: MoveId::Whirlwind,
                },
                LevelUpMove {
                    level: 28,
                    move_id: MoveId::WingAttack,
                },
                LevelUpMove {
                    level: 36,
                    move_id: MoveId::Agility,
                },
                LevelUpMove {
                    level: 44,
                    move_id: MoveId::MirrorMove,
                },
            ],
        },
        // #17 Pidgeotto
        EvosMovesEntry {
            species: Species::Pidgeotto,
            evolutions: vec![EvolutionMethod::Level {
                level: 36,
                species: Species::Pidgeot,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 5,
                    move_id: MoveId::SandAttack,
                },
                LevelUpMove {
                    level: 12,
                    move_id: MoveId::QuickAttack,
                },
                LevelUpMove {
                    level: 21,
                    move_id: MoveId::Whirlwind,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::WingAttack,
                },
                LevelUpMove {
                    level: 40,
                    move_id: MoveId::Agility,
                },
                LevelUpMove {
                    level: 49,
                    move_id: MoveId::MirrorMove,
                },
            ],
        },
        // #18 Pidgeot
        EvosMovesEntry {
            species: Species::Pidgeot,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 5,
                    move_id: MoveId::SandAttack,
                },
                LevelUpMove {
                    level: 12,
                    move_id: MoveId::QuickAttack,
                },
                LevelUpMove {
                    level: 21,
                    move_id: MoveId::Whirlwind,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::WingAttack,
                },
                LevelUpMove {
                    level: 44,
                    move_id: MoveId::Agility,
                },
                LevelUpMove {
                    level: 54,
                    move_id: MoveId::MirrorMove,
                },
            ],
        },
        // #19 Rattata
        EvosMovesEntry {
            species: Species::Rattata,
            evolutions: vec![EvolutionMethod::Level {
                level: 20,
                species: Species::Raticate,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 7,
                    move_id: MoveId::QuickAttack,
                },
                LevelUpMove {
                    level: 14,
                    move_id: MoveId::HyperFang,
                },
                LevelUpMove {
                    level: 23,
                    move_id: MoveId::FocusEnergy,
                },
                LevelUpMove {
                    level: 34,
                    move_id: MoveId::SuperFang,
                },
            ],
        },
        // #20 Raticate
        EvosMovesEntry {
            species: Species::Raticate,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 7,
                    move_id: MoveId::QuickAttack,
                },
                LevelUpMove {
                    level: 14,
                    move_id: MoveId::HyperFang,
                },
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::FocusEnergy,
                },
                LevelUpMove {
                    level: 41,
                    move_id: MoveId::SuperFang,
                },
            ],
        },
        // #21 Spearow
        EvosMovesEntry {
            species: Species::Spearow,
            evolutions: vec![EvolutionMethod::Level {
                level: 20,
                species: Species::Fearow,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 9,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::FuryAttack,
                },
                LevelUpMove {
                    level: 22,
                    move_id: MoveId::MirrorMove,
                },
                LevelUpMove {
                    level: 29,
                    move_id: MoveId::DrillPeck,
                },
                LevelUpMove {
                    level: 36,
                    move_id: MoveId::Agility,
                },
            ],
        },
        // #22 Fearow
        EvosMovesEntry {
            species: Species::Fearow,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 9,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::FuryAttack,
                },
                LevelUpMove {
                    level: 25,
                    move_id: MoveId::MirrorMove,
                },
                LevelUpMove {
                    level: 34,
                    move_id: MoveId::DrillPeck,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::Agility,
                },
            ],
        },
        // #23 Ekans
        EvosMovesEntry {
            species: Species::Ekans,
            evolutions: vec![EvolutionMethod::Level {
                level: 22,
                species: Species::Arbok,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 10,
                    move_id: MoveId::PoisonSting,
                },
                LevelUpMove {
                    level: 17,
                    move_id: MoveId::Bite,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::Glare,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::Screech,
                },
                LevelUpMove {
                    level: 38,
                    move_id: MoveId::Acid,
                },
            ],
        },
        // #24 Arbok
        EvosMovesEntry {
            species: Species::Arbok,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 10,
                    move_id: MoveId::PoisonSting,
                },
                LevelUpMove {
                    level: 17,
                    move_id: MoveId::Bite,
                },
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::Glare,
                },
                LevelUpMove {
                    level: 36,
                    move_id: MoveId::Screech,
                },
                LevelUpMove {
                    level: 47,
                    move_id: MoveId::Acid,
                },
            ],
        },
        // #25 Pikachu
        EvosMovesEntry {
            species: Species::Pikachu,
            evolutions: vec![EvolutionMethod::Item {
                item: ItemId::ThunderStone,
                min_level: 1,
                species: Species::Raichu,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 9,
                    move_id: MoveId::ThunderWave,
                },
                LevelUpMove {
                    level: 16,
                    move_id: MoveId::QuickAttack,
                },
                LevelUpMove {
                    level: 26,
                    move_id: MoveId::Swift,
                },
                LevelUpMove {
                    level: 33,
                    move_id: MoveId::Agility,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::Thunder,
                },
            ],
        },
        // #26 Raichu
        EvosMovesEntry {
            species: Species::Raichu,
            evolutions: vec![],
            learnset: vec![],
        },
        // #27 Sandshrew
        EvosMovesEntry {
            species: Species::Sandshrew,
            evolutions: vec![EvolutionMethod::Level {
                level: 22,
                species: Species::Sandslash,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 10,
                    move_id: MoveId::SandAttack,
                },
                LevelUpMove {
                    level: 17,
                    move_id: MoveId::Slash,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::PoisonSting,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::Swift,
                },
                LevelUpMove {
                    level: 38,
                    move_id: MoveId::FurySwipes,
                },
            ],
        },
        // #28 Sandslash
        EvosMovesEntry {
            species: Species::Sandslash,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 10,
                    move_id: MoveId::SandAttack,
                },
                LevelUpMove {
                    level: 17,
                    move_id: MoveId::Slash,
                },
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::PoisonSting,
                },
                LevelUpMove {
                    level: 36,
                    move_id: MoveId::Swift,
                },
                LevelUpMove {
                    level: 47,
                    move_id: MoveId::FurySwipes,
                },
            ],
        },
        // #29 NidoranF
        EvosMovesEntry {
            species: Species::NidoranF,
            evolutions: vec![EvolutionMethod::Level {
                level: 16,
                species: Species::Nidorina,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 8,
                    move_id: MoveId::Scratch,
                },
                LevelUpMove {
                    level: 14,
                    move_id: MoveId::PoisonSting,
                },
                LevelUpMove {
                    level: 21,
                    move_id: MoveId::TailWhip,
                },
                LevelUpMove {
                    level: 29,
                    move_id: MoveId::Bite,
                },
                LevelUpMove {
                    level: 36,
                    move_id: MoveId::FurySwipes,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::DoubleKick,
                },
            ],
        },
        // #30 Nidorina
        EvosMovesEntry {
            species: Species::Nidorina,
            evolutions: vec![EvolutionMethod::Item {
                item: ItemId::MoonStone,
                min_level: 1,
                species: Species::Nidoqueen,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 8,
                    move_id: MoveId::Scratch,
                },
                LevelUpMove {
                    level: 14,
                    move_id: MoveId::PoisonSting,
                },
                LevelUpMove {
                    level: 23,
                    move_id: MoveId::TailWhip,
                },
                LevelUpMove {
                    level: 32,
                    move_id: MoveId::Bite,
                },
                LevelUpMove {
                    level: 41,
                    move_id: MoveId::FurySwipes,
                },
                LevelUpMove {
                    level: 50,
                    move_id: MoveId::DoubleKick,
                },
            ],
        },
        // #31 Nidoqueen
        EvosMovesEntry {
            species: Species::Nidoqueen,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 8,
                    move_id: MoveId::Scratch,
                },
                LevelUpMove {
                    level: 14,
                    move_id: MoveId::PoisonSting,
                },
                LevelUpMove {
                    level: 23,
                    move_id: MoveId::BodySlam,
                },
            ],
        },
        // #32 NidoranM
        EvosMovesEntry {
            species: Species::NidoranM,
            evolutions: vec![EvolutionMethod::Level {
                level: 16,
                species: Species::Nidorino,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 8,
                    move_id: MoveId::HornAttack,
                },
                LevelUpMove {
                    level: 14,
                    move_id: MoveId::PoisonSting,
                },
                LevelUpMove {
                    level: 21,
                    move_id: MoveId::FocusEnergy,
                },
                LevelUpMove {
                    level: 29,
                    move_id: MoveId::FuryAttack,
                },
                LevelUpMove {
                    level: 36,
                    move_id: MoveId::HornDrill,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::DoubleKick,
                },
            ],
        },
        // #33 Nidorino
        EvosMovesEntry {
            species: Species::Nidorino,
            evolutions: vec![EvolutionMethod::Item {
                item: ItemId::MoonStone,
                min_level: 1,
                species: Species::Nidoking,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 8,
                    move_id: MoveId::HornAttack,
                },
                LevelUpMove {
                    level: 14,
                    move_id: MoveId::PoisonSting,
                },
                LevelUpMove {
                    level: 23,
                    move_id: MoveId::FocusEnergy,
                },
                LevelUpMove {
                    level: 32,
                    move_id: MoveId::FuryAttack,
                },
                LevelUpMove {
                    level: 41,
                    move_id: MoveId::HornDrill,
                },
                LevelUpMove {
                    level: 50,
                    move_id: MoveId::DoubleKick,
                },
            ],
        },
        // #34 Nidoking
        EvosMovesEntry {
            species: Species::Nidoking,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 8,
                    move_id: MoveId::HornAttack,
                },
                LevelUpMove {
                    level: 14,
                    move_id: MoveId::PoisonSting,
                },
                LevelUpMove {
                    level: 23,
                    move_id: MoveId::Thrash,
                },
            ],
        },
        // #35 Clefairy
        EvosMovesEntry {
            species: Species::Clefairy,
            evolutions: vec![EvolutionMethod::Item {
                item: ItemId::MoonStone,
                min_level: 1,
                species: Species::Clefable,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 13,
                    move_id: MoveId::Sing,
                },
                LevelUpMove {
                    level: 18,
                    move_id: MoveId::Doubleslap,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::Minimize,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::Metronome,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::DefenseCurl,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::LightScreen,
                },
            ],
        },
        // #36 Clefable
        EvosMovesEntry {
            species: Species::Clefable,
            evolutions: vec![],
            learnset: vec![],
        },
        // #37 Vulpix
        EvosMovesEntry {
            species: Species::Vulpix,
            evolutions: vec![EvolutionMethod::Item {
                item: ItemId::FireStone,
                min_level: 1,
                species: Species::Ninetales,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 16,
                    move_id: MoveId::QuickAttack,
                },
                LevelUpMove {
                    level: 21,
                    move_id: MoveId::Roar,
                },
                LevelUpMove {
                    level: 28,
                    move_id: MoveId::ConfuseRay,
                },
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::Flamethrower,
                },
                LevelUpMove {
                    level: 42,
                    move_id: MoveId::FireSpin,
                },
            ],
        },
        // #38 Ninetales
        EvosMovesEntry {
            species: Species::Ninetales,
            evolutions: vec![],
            learnset: vec![],
        },
        // #39 Jigglypuff
        EvosMovesEntry {
            species: Species::Jigglypuff,
            evolutions: vec![EvolutionMethod::Item {
                item: ItemId::MoonStone,
                min_level: 1,
                species: Species::Wigglytuff,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 9,
                    move_id: MoveId::Pound,
                },
                LevelUpMove {
                    level: 14,
                    move_id: MoveId::Disable,
                },
                LevelUpMove {
                    level: 19,
                    move_id: MoveId::DefenseCurl,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::Doubleslap,
                },
                LevelUpMove {
                    level: 29,
                    move_id: MoveId::Rest,
                },
                LevelUpMove {
                    level: 34,
                    move_id: MoveId::BodySlam,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::DoubleEdge,
                },
            ],
        },
        // #40 Wigglytuff
        EvosMovesEntry {
            species: Species::Wigglytuff,
            evolutions: vec![],
            learnset: vec![],
        },
        // #41 Zubat
        EvosMovesEntry {
            species: Species::Zubat,
            evolutions: vec![EvolutionMethod::Level {
                level: 22,
                species: Species::Golbat,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 10,
                    move_id: MoveId::Supersonic,
                },
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::Bite,
                },
                LevelUpMove {
                    level: 21,
                    move_id: MoveId::ConfuseRay,
                },
                LevelUpMove {
                    level: 28,
                    move_id: MoveId::WingAttack,
                },
                LevelUpMove {
                    level: 36,
                    move_id: MoveId::Haze,
                },
            ],
        },
        // #42 Golbat
        EvosMovesEntry {
            species: Species::Golbat,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 10,
                    move_id: MoveId::Supersonic,
                },
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::Bite,
                },
                LevelUpMove {
                    level: 21,
                    move_id: MoveId::ConfuseRay,
                },
                LevelUpMove {
                    level: 32,
                    move_id: MoveId::WingAttack,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::Haze,
                },
            ],
        },
        // #43 Oddish
        EvosMovesEntry {
            species: Species::Oddish,
            evolutions: vec![EvolutionMethod::Level {
                level: 21,
                species: Species::Gloom,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::Poisonpowder,
                },
                LevelUpMove {
                    level: 17,
                    move_id: MoveId::StunSpore,
                },
                LevelUpMove {
                    level: 19,
                    move_id: MoveId::SleepPowder,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::Acid,
                },
                LevelUpMove {
                    level: 33,
                    move_id: MoveId::PetalDance,
                },
                LevelUpMove {
                    level: 46,
                    move_id: MoveId::Solarbeam,
                },
            ],
        },
        // #44 Gloom
        EvosMovesEntry {
            species: Species::Gloom,
            evolutions: vec![EvolutionMethod::Item {
                item: ItemId::LeafStone,
                min_level: 1,
                species: Species::Vileplume,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::Poisonpowder,
                },
                LevelUpMove {
                    level: 17,
                    move_id: MoveId::StunSpore,
                },
                LevelUpMove {
                    level: 19,
                    move_id: MoveId::SleepPowder,
                },
                LevelUpMove {
                    level: 28,
                    move_id: MoveId::Acid,
                },
                LevelUpMove {
                    level: 38,
                    move_id: MoveId::PetalDance,
                },
                LevelUpMove {
                    level: 52,
                    move_id: MoveId::Solarbeam,
                },
            ],
        },
        // #45 Vileplume
        EvosMovesEntry {
            species: Species::Vileplume,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::Poisonpowder,
                },
                LevelUpMove {
                    level: 17,
                    move_id: MoveId::StunSpore,
                },
                LevelUpMove {
                    level: 19,
                    move_id: MoveId::SleepPowder,
                },
            ],
        },
        // #46 Paras
        EvosMovesEntry {
            species: Species::Paras,
            evolutions: vec![EvolutionMethod::Level {
                level: 24,
                species: Species::Parasect,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 13,
                    move_id: MoveId::StunSpore,
                },
                LevelUpMove {
                    level: 20,
                    move_id: MoveId::LeechLife,
                },
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::Spore,
                },
                LevelUpMove {
                    level: 34,
                    move_id: MoveId::Slash,
                },
                LevelUpMove {
                    level: 41,
                    move_id: MoveId::Growth,
                },
            ],
        },
        // #47 Parasect
        EvosMovesEntry {
            species: Species::Parasect,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 13,
                    move_id: MoveId::StunSpore,
                },
                LevelUpMove {
                    level: 20,
                    move_id: MoveId::LeechLife,
                },
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::Spore,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::Slash,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::Growth,
                },
            ],
        },
        // #48 Venonat
        EvosMovesEntry {
            species: Species::Venonat,
            evolutions: vec![EvolutionMethod::Level {
                level: 31,
                species: Species::Venomoth,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::Poisonpowder,
                },
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::LeechLife,
                },
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::StunSpore,
                },
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::Psybeam,
                },
                LevelUpMove {
                    level: 38,
                    move_id: MoveId::SleepPowder,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::PsychicM,
                },
            ],
        },
        // #49 Venomoth
        EvosMovesEntry {
            species: Species::Venomoth,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::Poisonpowder,
                },
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::LeechLife,
                },
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::StunSpore,
                },
                LevelUpMove {
                    level: 38,
                    move_id: MoveId::Psybeam,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::SleepPowder,
                },
                LevelUpMove {
                    level: 50,
                    move_id: MoveId::PsychicM,
                },
            ],
        },
        // #50 Diglett
        EvosMovesEntry {
            species: Species::Diglett,
            evolutions: vec![EvolutionMethod::Level {
                level: 26,
                species: Species::Dugtrio,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::Growl,
                },
                LevelUpMove {
                    level: 19,
                    move_id: MoveId::Dig,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::SandAttack,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::Slash,
                },
                LevelUpMove {
                    level: 40,
                    move_id: MoveId::Earthquake,
                },
            ],
        },
        // #51 Dugtrio
        EvosMovesEntry {
            species: Species::Dugtrio,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::Growl,
                },
                LevelUpMove {
                    level: 19,
                    move_id: MoveId::Dig,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::SandAttack,
                },
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::Slash,
                },
                LevelUpMove {
                    level: 47,
                    move_id: MoveId::Earthquake,
                },
            ],
        },
        // #52 Meowth
        EvosMovesEntry {
            species: Species::Meowth,
            evolutions: vec![EvolutionMethod::Level {
                level: 28,
                species: Species::Persian,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 12,
                    move_id: MoveId::Bite,
                },
                LevelUpMove {
                    level: 17,
                    move_id: MoveId::PayDay,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::Screech,
                },
                LevelUpMove {
                    level: 33,
                    move_id: MoveId::FurySwipes,
                },
                LevelUpMove {
                    level: 44,
                    move_id: MoveId::Slash,
                },
            ],
        },
        // #53 Persian
        EvosMovesEntry {
            species: Species::Persian,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 12,
                    move_id: MoveId::Bite,
                },
                LevelUpMove {
                    level: 17,
                    move_id: MoveId::PayDay,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::Screech,
                },
                LevelUpMove {
                    level: 37,
                    move_id: MoveId::FurySwipes,
                },
                LevelUpMove {
                    level: 51,
                    move_id: MoveId::Slash,
                },
            ],
        },
        // #54 Psyduck
        EvosMovesEntry {
            species: Species::Psyduck,
            evolutions: vec![EvolutionMethod::Level {
                level: 33,
                species: Species::Golduck,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 28,
                    move_id: MoveId::TailWhip,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::Disable,
                },
                LevelUpMove {
                    level: 36,
                    move_id: MoveId::Confusion,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::FurySwipes,
                },
                LevelUpMove {
                    level: 52,
                    move_id: MoveId::HydroPump,
                },
            ],
        },
        // #55 Golduck
        EvosMovesEntry {
            species: Species::Golduck,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 28,
                    move_id: MoveId::TailWhip,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::Disable,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::Confusion,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::FurySwipes,
                },
                LevelUpMove {
                    level: 59,
                    move_id: MoveId::HydroPump,
                },
            ],
        },
        // #56 Mankey
        EvosMovesEntry {
            species: Species::Mankey,
            evolutions: vec![EvolutionMethod::Level {
                level: 28,
                species: Species::Primeape,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::KarateChop,
                },
                LevelUpMove {
                    level: 21,
                    move_id: MoveId::FurySwipes,
                },
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::FocusEnergy,
                },
                LevelUpMove {
                    level: 33,
                    move_id: MoveId::SeismicToss,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::Thrash,
                },
            ],
        },
        // #57 Primeape
        EvosMovesEntry {
            species: Species::Primeape,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::KarateChop,
                },
                LevelUpMove {
                    level: 21,
                    move_id: MoveId::FurySwipes,
                },
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::FocusEnergy,
                },
                LevelUpMove {
                    level: 37,
                    move_id: MoveId::SeismicToss,
                },
                LevelUpMove {
                    level: 46,
                    move_id: MoveId::Thrash,
                },
            ],
        },
        // #58 Growlithe
        EvosMovesEntry {
            species: Species::Growlithe,
            evolutions: vec![EvolutionMethod::Item {
                item: ItemId::FireStone,
                min_level: 1,
                species: Species::Arcanine,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 18,
                    move_id: MoveId::Ember,
                },
                LevelUpMove {
                    level: 23,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::TakeDown,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::Agility,
                },
                LevelUpMove {
                    level: 50,
                    move_id: MoveId::Flamethrower,
                },
            ],
        },
        // #59 Arcanine
        EvosMovesEntry {
            species: Species::Arcanine,
            evolutions: vec![],
            learnset: vec![],
        },
        // #60 Poliwag
        EvosMovesEntry {
            species: Species::Poliwag,
            evolutions: vec![EvolutionMethod::Level {
                level: 25,
                species: Species::Poliwhirl,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 16,
                    move_id: MoveId::Hypnosis,
                },
                LevelUpMove {
                    level: 19,
                    move_id: MoveId::WaterGun,
                },
                LevelUpMove {
                    level: 25,
                    move_id: MoveId::Doubleslap,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::BodySlam,
                },
                LevelUpMove {
                    level: 38,
                    move_id: MoveId::Amnesia,
                },
                LevelUpMove {
                    level: 45,
                    move_id: MoveId::HydroPump,
                },
            ],
        },
        // #61 Poliwhirl
        EvosMovesEntry {
            species: Species::Poliwhirl,
            evolutions: vec![EvolutionMethod::Item {
                item: ItemId::WaterStone,
                min_level: 1,
                species: Species::Poliwrath,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 16,
                    move_id: MoveId::Hypnosis,
                },
                LevelUpMove {
                    level: 19,
                    move_id: MoveId::WaterGun,
                },
                LevelUpMove {
                    level: 26,
                    move_id: MoveId::Doubleslap,
                },
                LevelUpMove {
                    level: 33,
                    move_id: MoveId::BodySlam,
                },
                LevelUpMove {
                    level: 41,
                    move_id: MoveId::Amnesia,
                },
                LevelUpMove {
                    level: 49,
                    move_id: MoveId::HydroPump,
                },
            ],
        },
        // #62 Poliwrath
        EvosMovesEntry {
            species: Species::Poliwrath,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 16,
                    move_id: MoveId::Hypnosis,
                },
                LevelUpMove {
                    level: 19,
                    move_id: MoveId::WaterGun,
                },
            ],
        },
        // #63 Abra
        EvosMovesEntry {
            species: Species::Abra,
            evolutions: vec![EvolutionMethod::Level {
                level: 16,
                species: Species::Kadabra,
            }],
            learnset: vec![],
        },
        // #64 Kadabra
        EvosMovesEntry {
            species: Species::Kadabra,
            evolutions: vec![EvolutionMethod::Trade {
                min_level: 1,
                species: Species::Alakazam,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 16,
                    move_id: MoveId::Confusion,
                },
                LevelUpMove {
                    level: 20,
                    move_id: MoveId::Disable,
                },
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::Psybeam,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::Recover,
                },
                LevelUpMove {
                    level: 38,
                    move_id: MoveId::PsychicM,
                },
                LevelUpMove {
                    level: 42,
                    move_id: MoveId::Reflect,
                },
            ],
        },
        // #65 Alakazam
        EvosMovesEntry {
            species: Species::Alakazam,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 16,
                    move_id: MoveId::Confusion,
                },
                LevelUpMove {
                    level: 20,
                    move_id: MoveId::Disable,
                },
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::Psybeam,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::Recover,
                },
                LevelUpMove {
                    level: 38,
                    move_id: MoveId::PsychicM,
                },
                LevelUpMove {
                    level: 42,
                    move_id: MoveId::Reflect,
                },
            ],
        },
        // #66 Machop
        EvosMovesEntry {
            species: Species::Machop,
            evolutions: vec![EvolutionMethod::Level {
                level: 28,
                species: Species::Machoke,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 20,
                    move_id: MoveId::LowKick,
                },
                LevelUpMove {
                    level: 25,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 32,
                    move_id: MoveId::FocusEnergy,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::SeismicToss,
                },
                LevelUpMove {
                    level: 46,
                    move_id: MoveId::Submission,
                },
            ],
        },
        // #67 Machoke
        EvosMovesEntry {
            species: Species::Machoke,
            evolutions: vec![EvolutionMethod::Trade {
                min_level: 1,
                species: Species::Machamp,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 20,
                    move_id: MoveId::LowKick,
                },
                LevelUpMove {
                    level: 25,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 36,
                    move_id: MoveId::FocusEnergy,
                },
                LevelUpMove {
                    level: 44,
                    move_id: MoveId::SeismicToss,
                },
                LevelUpMove {
                    level: 52,
                    move_id: MoveId::Submission,
                },
            ],
        },
        // #68 Machamp
        EvosMovesEntry {
            species: Species::Machamp,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 20,
                    move_id: MoveId::LowKick,
                },
                LevelUpMove {
                    level: 25,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 36,
                    move_id: MoveId::FocusEnergy,
                },
                LevelUpMove {
                    level: 44,
                    move_id: MoveId::SeismicToss,
                },
                LevelUpMove {
                    level: 52,
                    move_id: MoveId::Submission,
                },
            ],
        },
        // #69 Bellsprout
        EvosMovesEntry {
            species: Species::Bellsprout,
            evolutions: vec![EvolutionMethod::Level {
                level: 21,
                species: Species::Weepinbell,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 13,
                    move_id: MoveId::Wrap,
                },
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::Poisonpowder,
                },
                LevelUpMove {
                    level: 18,
                    move_id: MoveId::SleepPowder,
                },
                LevelUpMove {
                    level: 21,
                    move_id: MoveId::StunSpore,
                },
                LevelUpMove {
                    level: 26,
                    move_id: MoveId::Acid,
                },
                LevelUpMove {
                    level: 33,
                    move_id: MoveId::RazorLeaf,
                },
                LevelUpMove {
                    level: 42,
                    move_id: MoveId::Slam,
                },
            ],
        },
        // #70 Weepinbell
        EvosMovesEntry {
            species: Species::Weepinbell,
            evolutions: vec![EvolutionMethod::Item {
                item: ItemId::LeafStone,
                min_level: 1,
                species: Species::Victreebel,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 13,
                    move_id: MoveId::Wrap,
                },
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::Poisonpowder,
                },
                LevelUpMove {
                    level: 18,
                    move_id: MoveId::SleepPowder,
                },
                LevelUpMove {
                    level: 23,
                    move_id: MoveId::StunSpore,
                },
                LevelUpMove {
                    level: 29,
                    move_id: MoveId::Acid,
                },
                LevelUpMove {
                    level: 38,
                    move_id: MoveId::RazorLeaf,
                },
                LevelUpMove {
                    level: 49,
                    move_id: MoveId::Slam,
                },
            ],
        },
        // #71 Victreebel
        EvosMovesEntry {
            species: Species::Victreebel,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 13,
                    move_id: MoveId::Wrap,
                },
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::Poisonpowder,
                },
                LevelUpMove {
                    level: 18,
                    move_id: MoveId::SleepPowder,
                },
            ],
        },
        // #72 Tentacool
        EvosMovesEntry {
            species: Species::Tentacool,
            evolutions: vec![EvolutionMethod::Level {
                level: 30,
                species: Species::Tentacruel,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 7,
                    move_id: MoveId::Supersonic,
                },
                LevelUpMove {
                    level: 13,
                    move_id: MoveId::Wrap,
                },
                LevelUpMove {
                    level: 18,
                    move_id: MoveId::PoisonSting,
                },
                LevelUpMove {
                    level: 22,
                    move_id: MoveId::WaterGun,
                },
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::Constrict,
                },
                LevelUpMove {
                    level: 33,
                    move_id: MoveId::Barrier,
                },
                LevelUpMove {
                    level: 40,
                    move_id: MoveId::Screech,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::HydroPump,
                },
            ],
        },
        // #73 Tentacruel
        EvosMovesEntry {
            species: Species::Tentacruel,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 7,
                    move_id: MoveId::Supersonic,
                },
                LevelUpMove {
                    level: 13,
                    move_id: MoveId::Wrap,
                },
                LevelUpMove {
                    level: 18,
                    move_id: MoveId::PoisonSting,
                },
                LevelUpMove {
                    level: 22,
                    move_id: MoveId::WaterGun,
                },
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::Constrict,
                },
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::Barrier,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::Screech,
                },
                LevelUpMove {
                    level: 50,
                    move_id: MoveId::HydroPump,
                },
            ],
        },
        // #74 Geodude
        EvosMovesEntry {
            species: Species::Geodude,
            evolutions: vec![EvolutionMethod::Level {
                level: 25,
                species: Species::Graveler,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 11,
                    move_id: MoveId::DefenseCurl,
                },
                LevelUpMove {
                    level: 16,
                    move_id: MoveId::RockThrow,
                },
                LevelUpMove {
                    level: 21,
                    move_id: MoveId::Selfdestruct,
                },
                LevelUpMove {
                    level: 26,
                    move_id: MoveId::Harden,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::Earthquake,
                },
                LevelUpMove {
                    level: 36,
                    move_id: MoveId::Explosion,
                },
            ],
        },
        // #75 Graveler
        EvosMovesEntry {
            species: Species::Graveler,
            evolutions: vec![EvolutionMethod::Trade {
                min_level: 1,
                species: Species::Golem,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 11,
                    move_id: MoveId::DefenseCurl,
                },
                LevelUpMove {
                    level: 16,
                    move_id: MoveId::RockThrow,
                },
                LevelUpMove {
                    level: 21,
                    move_id: MoveId::Selfdestruct,
                },
                LevelUpMove {
                    level: 29,
                    move_id: MoveId::Harden,
                },
                LevelUpMove {
                    level: 36,
                    move_id: MoveId::Earthquake,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::Explosion,
                },
            ],
        },
        // #76 Golem
        EvosMovesEntry {
            species: Species::Golem,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 11,
                    move_id: MoveId::DefenseCurl,
                },
                LevelUpMove {
                    level: 16,
                    move_id: MoveId::RockThrow,
                },
                LevelUpMove {
                    level: 21,
                    move_id: MoveId::Selfdestruct,
                },
                LevelUpMove {
                    level: 29,
                    move_id: MoveId::Harden,
                },
                LevelUpMove {
                    level: 36,
                    move_id: MoveId::Earthquake,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::Explosion,
                },
            ],
        },
        // #77 Ponyta
        EvosMovesEntry {
            species: Species::Ponyta,
            evolutions: vec![EvolutionMethod::Level {
                level: 40,
                species: Species::Rapidash,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::TailWhip,
                },
                LevelUpMove {
                    level: 32,
                    move_id: MoveId::Stomp,
                },
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::Growl,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::FireSpin,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::TakeDown,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::Agility,
                },
            ],
        },
        // #78 Rapidash
        EvosMovesEntry {
            species: Species::Rapidash,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::TailWhip,
                },
                LevelUpMove {
                    level: 32,
                    move_id: MoveId::Stomp,
                },
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::Growl,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::FireSpin,
                },
                LevelUpMove {
                    level: 47,
                    move_id: MoveId::TakeDown,
                },
                LevelUpMove {
                    level: 55,
                    move_id: MoveId::Agility,
                },
            ],
        },
        // #79 Slowpoke
        EvosMovesEntry {
            species: Species::Slowpoke,
            evolutions: vec![EvolutionMethod::Level {
                level: 37,
                species: Species::Slowbro,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 18,
                    move_id: MoveId::Disable,
                },
                LevelUpMove {
                    level: 22,
                    move_id: MoveId::Headbutt,
                },
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::Growl,
                },
                LevelUpMove {
                    level: 33,
                    move_id: MoveId::WaterGun,
                },
                LevelUpMove {
                    level: 40,
                    move_id: MoveId::Amnesia,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::PsychicM,
                },
            ],
        },
        // #80 Slowbro
        EvosMovesEntry {
            species: Species::Slowbro,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 18,
                    move_id: MoveId::Disable,
                },
                LevelUpMove {
                    level: 22,
                    move_id: MoveId::Headbutt,
                },
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::Growl,
                },
                LevelUpMove {
                    level: 33,
                    move_id: MoveId::WaterGun,
                },
                LevelUpMove {
                    level: 37,
                    move_id: MoveId::Withdraw,
                },
                LevelUpMove {
                    level: 44,
                    move_id: MoveId::Amnesia,
                },
                LevelUpMove {
                    level: 55,
                    move_id: MoveId::PsychicM,
                },
            ],
        },
        // #81 Magnemite
        EvosMovesEntry {
            species: Species::Magnemite,
            evolutions: vec![EvolutionMethod::Level {
                level: 30,
                species: Species::Magneton,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 21,
                    move_id: MoveId::Sonicboom,
                },
                LevelUpMove {
                    level: 25,
                    move_id: MoveId::Thundershock,
                },
                LevelUpMove {
                    level: 29,
                    move_id: MoveId::Supersonic,
                },
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::ThunderWave,
                },
                LevelUpMove {
                    level: 41,
                    move_id: MoveId::Swift,
                },
                LevelUpMove {
                    level: 47,
                    move_id: MoveId::Screech,
                },
            ],
        },
        // #82 Magneton
        EvosMovesEntry {
            species: Species::Magneton,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 21,
                    move_id: MoveId::Sonicboom,
                },
                LevelUpMove {
                    level: 25,
                    move_id: MoveId::Thundershock,
                },
                LevelUpMove {
                    level: 29,
                    move_id: MoveId::Supersonic,
                },
                LevelUpMove {
                    level: 38,
                    move_id: MoveId::ThunderWave,
                },
                LevelUpMove {
                    level: 46,
                    move_id: MoveId::Swift,
                },
                LevelUpMove {
                    level: 54,
                    move_id: MoveId::Screech,
                },
            ],
        },
        // #83 Farfetchd
        EvosMovesEntry {
            species: Species::Farfetchd,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 7,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::FuryAttack,
                },
                LevelUpMove {
                    level: 23,
                    move_id: MoveId::SwordsDance,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::Agility,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::Slash,
                },
            ],
        },
        // #84 Doduo
        EvosMovesEntry {
            species: Species::Doduo,
            evolutions: vec![EvolutionMethod::Level {
                level: 31,
                species: Species::Dodrio,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 20,
                    move_id: MoveId::Growl,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::FuryAttack,
                },
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::DrillPeck,
                },
                LevelUpMove {
                    level: 36,
                    move_id: MoveId::Rage,
                },
                LevelUpMove {
                    level: 40,
                    move_id: MoveId::TriAttack,
                },
                LevelUpMove {
                    level: 44,
                    move_id: MoveId::Agility,
                },
            ],
        },
        // #85 Dodrio
        EvosMovesEntry {
            species: Species::Dodrio,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 20,
                    move_id: MoveId::Growl,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::FuryAttack,
                },
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::DrillPeck,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::Rage,
                },
                LevelUpMove {
                    level: 45,
                    move_id: MoveId::TriAttack,
                },
                LevelUpMove {
                    level: 51,
                    move_id: MoveId::Agility,
                },
            ],
        },
        // #86 Seel
        EvosMovesEntry {
            species: Species::Seel,
            evolutions: vec![EvolutionMethod::Level {
                level: 34,
                species: Species::Dewgong,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::Growl,
                },
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::AuroraBeam,
                },
                LevelUpMove {
                    level: 40,
                    move_id: MoveId::Rest,
                },
                LevelUpMove {
                    level: 45,
                    move_id: MoveId::TakeDown,
                },
                LevelUpMove {
                    level: 50,
                    move_id: MoveId::IceBeam,
                },
            ],
        },
        // #87 Dewgong
        EvosMovesEntry {
            species: Species::Dewgong,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::Growl,
                },
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::AuroraBeam,
                },
                LevelUpMove {
                    level: 44,
                    move_id: MoveId::Rest,
                },
                LevelUpMove {
                    level: 50,
                    move_id: MoveId::TakeDown,
                },
                LevelUpMove {
                    level: 56,
                    move_id: MoveId::IceBeam,
                },
            ],
        },
        // #88 Grimer
        EvosMovesEntry {
            species: Species::Grimer,
            evolutions: vec![EvolutionMethod::Level {
                level: 38,
                species: Species::Muk,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::PoisonGas,
                },
                LevelUpMove {
                    level: 33,
                    move_id: MoveId::Minimize,
                },
                LevelUpMove {
                    level: 37,
                    move_id: MoveId::Sludge,
                },
                LevelUpMove {
                    level: 42,
                    move_id: MoveId::Harden,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::Screech,
                },
                LevelUpMove {
                    level: 55,
                    move_id: MoveId::AcidArmor,
                },
            ],
        },
        // #89 Muk
        EvosMovesEntry {
            species: Species::Muk,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::PoisonGas,
                },
                LevelUpMove {
                    level: 33,
                    move_id: MoveId::Minimize,
                },
                LevelUpMove {
                    level: 37,
                    move_id: MoveId::Sludge,
                },
                LevelUpMove {
                    level: 45,
                    move_id: MoveId::Harden,
                },
                LevelUpMove {
                    level: 53,
                    move_id: MoveId::Screech,
                },
                LevelUpMove {
                    level: 60,
                    move_id: MoveId::AcidArmor,
                },
            ],
        },
        // #90 Shellder
        EvosMovesEntry {
            species: Species::Shellder,
            evolutions: vec![EvolutionMethod::Item {
                item: ItemId::WaterStone,
                min_level: 1,
                species: Species::Cloyster,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 18,
                    move_id: MoveId::Supersonic,
                },
                LevelUpMove {
                    level: 23,
                    move_id: MoveId::Clamp,
                },
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::AuroraBeam,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 50,
                    move_id: MoveId::IceBeam,
                },
            ],
        },
        // #91 Cloyster
        EvosMovesEntry {
            species: Species::Cloyster,
            evolutions: vec![],
            learnset: vec![LevelUpMove {
                level: 50,
                move_id: MoveId::SpikeCannon,
            }],
        },
        // #92 Gastly
        EvosMovesEntry {
            species: Species::Gastly,
            evolutions: vec![EvolutionMethod::Level {
                level: 25,
                species: Species::Haunter,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::Hypnosis,
                },
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::DreamEater,
                },
            ],
        },
        // #93 Haunter
        EvosMovesEntry {
            species: Species::Haunter,
            evolutions: vec![EvolutionMethod::Trade {
                min_level: 1,
                species: Species::Gengar,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 29,
                    move_id: MoveId::Hypnosis,
                },
                LevelUpMove {
                    level: 38,
                    move_id: MoveId::DreamEater,
                },
            ],
        },
        // #94 Gengar
        EvosMovesEntry {
            species: Species::Gengar,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 29,
                    move_id: MoveId::Hypnosis,
                },
                LevelUpMove {
                    level: 38,
                    move_id: MoveId::DreamEater,
                },
            ],
        },
        // #95 Onix
        EvosMovesEntry {
            species: Species::Onix,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::Bind,
                },
                LevelUpMove {
                    level: 19,
                    move_id: MoveId::RockThrow,
                },
                LevelUpMove {
                    level: 25,
                    move_id: MoveId::Rage,
                },
                LevelUpMove {
                    level: 33,
                    move_id: MoveId::Slam,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::Harden,
                },
            ],
        },
        // #96 Drowzee
        EvosMovesEntry {
            species: Species::Drowzee,
            evolutions: vec![EvolutionMethod::Level {
                level: 26,
                species: Species::Hypno,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 12,
                    move_id: MoveId::Disable,
                },
                LevelUpMove {
                    level: 17,
                    move_id: MoveId::Confusion,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::Headbutt,
                },
                LevelUpMove {
                    level: 29,
                    move_id: MoveId::PoisonGas,
                },
                LevelUpMove {
                    level: 32,
                    move_id: MoveId::PsychicM,
                },
                LevelUpMove {
                    level: 37,
                    move_id: MoveId::Meditate,
                },
            ],
        },
        // #97 Hypno
        EvosMovesEntry {
            species: Species::Hypno,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 12,
                    move_id: MoveId::Disable,
                },
                LevelUpMove {
                    level: 17,
                    move_id: MoveId::Confusion,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::Headbutt,
                },
                LevelUpMove {
                    level: 33,
                    move_id: MoveId::PoisonGas,
                },
                LevelUpMove {
                    level: 37,
                    move_id: MoveId::PsychicM,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::Meditate,
                },
            ],
        },
        // #98 Krabby
        EvosMovesEntry {
            species: Species::Krabby,
            evolutions: vec![EvolutionMethod::Level {
                level: 28,
                species: Species::Kingler,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 20,
                    move_id: MoveId::Vicegrip,
                },
                LevelUpMove {
                    level: 25,
                    move_id: MoveId::Guillotine,
                },
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::Stomp,
                },
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::Crabhammer,
                },
                LevelUpMove {
                    level: 40,
                    move_id: MoveId::Harden,
                },
            ],
        },
        // #99 Kingler
        EvosMovesEntry {
            species: Species::Kingler,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 20,
                    move_id: MoveId::Vicegrip,
                },
                LevelUpMove {
                    level: 25,
                    move_id: MoveId::Guillotine,
                },
                LevelUpMove {
                    level: 34,
                    move_id: MoveId::Stomp,
                },
                LevelUpMove {
                    level: 42,
                    move_id: MoveId::Crabhammer,
                },
                LevelUpMove {
                    level: 49,
                    move_id: MoveId::Harden,
                },
            ],
        },
        // #100 Voltorb
        EvosMovesEntry {
            species: Species::Voltorb,
            evolutions: vec![EvolutionMethod::Level {
                level: 30,
                species: Species::Electrode,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 17,
                    move_id: MoveId::Sonicboom,
                },
                LevelUpMove {
                    level: 22,
                    move_id: MoveId::Selfdestruct,
                },
                LevelUpMove {
                    level: 29,
                    move_id: MoveId::LightScreen,
                },
                LevelUpMove {
                    level: 36,
                    move_id: MoveId::Swift,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::Explosion,
                },
            ],
        },
        // #101 Electrode
        EvosMovesEntry {
            species: Species::Electrode,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 17,
                    move_id: MoveId::Sonicboom,
                },
                LevelUpMove {
                    level: 22,
                    move_id: MoveId::Selfdestruct,
                },
                LevelUpMove {
                    level: 29,
                    move_id: MoveId::LightScreen,
                },
                LevelUpMove {
                    level: 40,
                    move_id: MoveId::Swift,
                },
                LevelUpMove {
                    level: 50,
                    move_id: MoveId::Explosion,
                },
            ],
        },
        // #102 Exeggcute
        EvosMovesEntry {
            species: Species::Exeggcute,
            evolutions: vec![EvolutionMethod::Item {
                item: ItemId::LeafStone,
                min_level: 1,
                species: Species::Exeggutor,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 25,
                    move_id: MoveId::Reflect,
                },
                LevelUpMove {
                    level: 28,
                    move_id: MoveId::LeechSeed,
                },
                LevelUpMove {
                    level: 32,
                    move_id: MoveId::StunSpore,
                },
                LevelUpMove {
                    level: 37,
                    move_id: MoveId::Poisonpowder,
                },
                LevelUpMove {
                    level: 42,
                    move_id: MoveId::Solarbeam,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::SleepPowder,
                },
            ],
        },
        // #103 Exeggutor
        EvosMovesEntry {
            species: Species::Exeggutor,
            evolutions: vec![],
            learnset: vec![LevelUpMove {
                level: 28,
                move_id: MoveId::Stomp,
            }],
        },
        // #104 Cubone
        EvosMovesEntry {
            species: Species::Cubone,
            evolutions: vec![EvolutionMethod::Level {
                level: 28,
                species: Species::Marowak,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 25,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::FocusEnergy,
                },
                LevelUpMove {
                    level: 38,
                    move_id: MoveId::Thrash,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::Bonemerang,
                },
                LevelUpMove {
                    level: 46,
                    move_id: MoveId::Rage,
                },
            ],
        },
        // #105 Marowak
        EvosMovesEntry {
            species: Species::Marowak,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 25,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 33,
                    move_id: MoveId::FocusEnergy,
                },
                LevelUpMove {
                    level: 41,
                    move_id: MoveId::Thrash,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::Bonemerang,
                },
                LevelUpMove {
                    level: 55,
                    move_id: MoveId::Rage,
                },
            ],
        },
        // #106 Hitmonlee
        EvosMovesEntry {
            species: Species::Hitmonlee,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 33,
                    move_id: MoveId::RollingKick,
                },
                LevelUpMove {
                    level: 38,
                    move_id: MoveId::JumpKick,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::FocusEnergy,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::HiJumpKick,
                },
                LevelUpMove {
                    level: 53,
                    move_id: MoveId::MegaKick,
                },
            ],
        },
        // #107 Hitmonchan
        EvosMovesEntry {
            species: Species::Hitmonchan,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 33,
                    move_id: MoveId::FirePunch,
                },
                LevelUpMove {
                    level: 38,
                    move_id: MoveId::IcePunch,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::Thunderpunch,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::MegaPunch,
                },
                LevelUpMove {
                    level: 53,
                    move_id: MoveId::Counter,
                },
            ],
        },
        // #108 Lickitung
        EvosMovesEntry {
            species: Species::Lickitung,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 7,
                    move_id: MoveId::Stomp,
                },
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::Disable,
                },
                LevelUpMove {
                    level: 23,
                    move_id: MoveId::DefenseCurl,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::Slam,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::Screech,
                },
            ],
        },
        // #109 Koffing
        EvosMovesEntry {
            species: Species::Koffing,
            evolutions: vec![EvolutionMethod::Level {
                level: 35,
                species: Species::Weezing,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 32,
                    move_id: MoveId::Sludge,
                },
                LevelUpMove {
                    level: 37,
                    move_id: MoveId::Smokescreen,
                },
                LevelUpMove {
                    level: 40,
                    move_id: MoveId::Selfdestruct,
                },
                LevelUpMove {
                    level: 45,
                    move_id: MoveId::Haze,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::Explosion,
                },
            ],
        },
        // #110 Weezing
        EvosMovesEntry {
            species: Species::Weezing,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 32,
                    move_id: MoveId::Sludge,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::Smokescreen,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::Selfdestruct,
                },
                LevelUpMove {
                    level: 49,
                    move_id: MoveId::Haze,
                },
                LevelUpMove {
                    level: 53,
                    move_id: MoveId::Explosion,
                },
            ],
        },
        // #111 Rhyhorn
        EvosMovesEntry {
            species: Species::Rhyhorn,
            evolutions: vec![EvolutionMethod::Level {
                level: 42,
                species: Species::Rhydon,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::Stomp,
                },
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::TailWhip,
                },
                LevelUpMove {
                    level: 40,
                    move_id: MoveId::FuryAttack,
                },
                LevelUpMove {
                    level: 45,
                    move_id: MoveId::HornDrill,
                },
                LevelUpMove {
                    level: 50,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 55,
                    move_id: MoveId::TakeDown,
                },
            ],
        },
        // #112 Rhydon
        EvosMovesEntry {
            species: Species::Rhydon,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::Stomp,
                },
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::TailWhip,
                },
                LevelUpMove {
                    level: 40,
                    move_id: MoveId::FuryAttack,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::HornDrill,
                },
                LevelUpMove {
                    level: 55,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 64,
                    move_id: MoveId::TakeDown,
                },
            ],
        },
        // #113 Chansey
        EvosMovesEntry {
            species: Species::Chansey,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::Sing,
                },
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::Growl,
                },
                LevelUpMove {
                    level: 38,
                    move_id: MoveId::Minimize,
                },
                LevelUpMove {
                    level: 44,
                    move_id: MoveId::DefenseCurl,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::LightScreen,
                },
                LevelUpMove {
                    level: 54,
                    move_id: MoveId::DoubleEdge,
                },
            ],
        },
        // #114 Tangela
        EvosMovesEntry {
            species: Species::Tangela,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 29,
                    move_id: MoveId::Absorb,
                },
                LevelUpMove {
                    level: 32,
                    move_id: MoveId::Poisonpowder,
                },
                LevelUpMove {
                    level: 36,
                    move_id: MoveId::StunSpore,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::SleepPowder,
                },
                LevelUpMove {
                    level: 45,
                    move_id: MoveId::Slam,
                },
                LevelUpMove {
                    level: 49,
                    move_id: MoveId::Growth,
                },
            ],
        },
        // #115 Kangaskhan
        EvosMovesEntry {
            species: Species::Kangaskhan,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 26,
                    move_id: MoveId::Bite,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::TailWhip,
                },
                LevelUpMove {
                    level: 36,
                    move_id: MoveId::MegaPunch,
                },
                LevelUpMove {
                    level: 41,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 46,
                    move_id: MoveId::DizzyPunch,
                },
            ],
        },
        // #116 Horsea
        EvosMovesEntry {
            species: Species::Horsea,
            evolutions: vec![EvolutionMethod::Level {
                level: 32,
                species: Species::Seadra,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 19,
                    move_id: MoveId::Smokescreen,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::WaterGun,
                },
                LevelUpMove {
                    level: 37,
                    move_id: MoveId::Agility,
                },
                LevelUpMove {
                    level: 45,
                    move_id: MoveId::HydroPump,
                },
            ],
        },
        // #117 Seadra
        EvosMovesEntry {
            species: Species::Seadra,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 19,
                    move_id: MoveId::Smokescreen,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::WaterGun,
                },
                LevelUpMove {
                    level: 41,
                    move_id: MoveId::Agility,
                },
                LevelUpMove {
                    level: 52,
                    move_id: MoveId::HydroPump,
                },
            ],
        },
        // #118 Goldeen
        EvosMovesEntry {
            species: Species::Goldeen,
            evolutions: vec![EvolutionMethod::Level {
                level: 33,
                species: Species::Seaking,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 19,
                    move_id: MoveId::Supersonic,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::HornAttack,
                },
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::FuryAttack,
                },
                LevelUpMove {
                    level: 37,
                    move_id: MoveId::Waterfall,
                },
                LevelUpMove {
                    level: 45,
                    move_id: MoveId::HornDrill,
                },
                LevelUpMove {
                    level: 54,
                    move_id: MoveId::Agility,
                },
            ],
        },
        // #119 Seaking
        EvosMovesEntry {
            species: Species::Seaking,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 19,
                    move_id: MoveId::Supersonic,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::HornAttack,
                },
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::FuryAttack,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::Waterfall,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::HornDrill,
                },
                LevelUpMove {
                    level: 54,
                    move_id: MoveId::Agility,
                },
            ],
        },
        // #120 Staryu
        EvosMovesEntry {
            species: Species::Staryu,
            evolutions: vec![EvolutionMethod::Item {
                item: ItemId::WaterStone,
                min_level: 1,
                species: Species::Starmie,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 17,
                    move_id: MoveId::WaterGun,
                },
                LevelUpMove {
                    level: 22,
                    move_id: MoveId::Harden,
                },
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::Recover,
                },
                LevelUpMove {
                    level: 32,
                    move_id: MoveId::Swift,
                },
                LevelUpMove {
                    level: 37,
                    move_id: MoveId::Minimize,
                },
                LevelUpMove {
                    level: 42,
                    move_id: MoveId::LightScreen,
                },
                LevelUpMove {
                    level: 47,
                    move_id: MoveId::HydroPump,
                },
            ],
        },
        // #121 Starmie
        EvosMovesEntry {
            species: Species::Starmie,
            evolutions: vec![],
            learnset: vec![],
        },
        // #122 MrMime
        EvosMovesEntry {
            species: Species::MrMime,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 15,
                    move_id: MoveId::Confusion,
                },
                LevelUpMove {
                    level: 23,
                    move_id: MoveId::LightScreen,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::Doubleslap,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::Meditate,
                },
                LevelUpMove {
                    level: 47,
                    move_id: MoveId::Substitute,
                },
            ],
        },
        // #123 Scyther
        EvosMovesEntry {
            species: Species::Scyther,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 17,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 20,
                    move_id: MoveId::FocusEnergy,
                },
                LevelUpMove {
                    level: 24,
                    move_id: MoveId::DoubleTeam,
                },
                LevelUpMove {
                    level: 29,
                    move_id: MoveId::Slash,
                },
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::SwordsDance,
                },
                LevelUpMove {
                    level: 42,
                    move_id: MoveId::Agility,
                },
            ],
        },
        // #124 Jynx
        EvosMovesEntry {
            species: Species::Jynx,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 18,
                    move_id: MoveId::Lick,
                },
                LevelUpMove {
                    level: 23,
                    move_id: MoveId::Doubleslap,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::IcePunch,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::BodySlam,
                },
                LevelUpMove {
                    level: 47,
                    move_id: MoveId::Thrash,
                },
                LevelUpMove {
                    level: 58,
                    move_id: MoveId::Blizzard,
                },
            ],
        },
        // #125 Electabuzz
        EvosMovesEntry {
            species: Species::Electabuzz,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 34,
                    move_id: MoveId::Thundershock,
                },
                LevelUpMove {
                    level: 37,
                    move_id: MoveId::Screech,
                },
                LevelUpMove {
                    level: 42,
                    move_id: MoveId::Thunderpunch,
                },
                LevelUpMove {
                    level: 49,
                    move_id: MoveId::LightScreen,
                },
                LevelUpMove {
                    level: 54,
                    move_id: MoveId::Thunder,
                },
            ],
        },
        // #126 Magmar
        EvosMovesEntry {
            species: Species::Magmar,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 36,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::ConfuseRay,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::FirePunch,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::Smokescreen,
                },
                LevelUpMove {
                    level: 52,
                    move_id: MoveId::Smog,
                },
                LevelUpMove {
                    level: 55,
                    move_id: MoveId::Flamethrower,
                },
            ],
        },
        // #127 Pinsir
        EvosMovesEntry {
            species: Species::Pinsir,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 25,
                    move_id: MoveId::SeismicToss,
                },
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::Guillotine,
                },
                LevelUpMove {
                    level: 36,
                    move_id: MoveId::FocusEnergy,
                },
                LevelUpMove {
                    level: 43,
                    move_id: MoveId::Harden,
                },
                LevelUpMove {
                    level: 49,
                    move_id: MoveId::Slash,
                },
                LevelUpMove {
                    level: 54,
                    move_id: MoveId::SwordsDance,
                },
            ],
        },
        // #128 Tauros
        EvosMovesEntry {
            species: Species::Tauros,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 21,
                    move_id: MoveId::Stomp,
                },
                LevelUpMove {
                    level: 28,
                    move_id: MoveId::TailWhip,
                },
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 44,
                    move_id: MoveId::Rage,
                },
                LevelUpMove {
                    level: 51,
                    move_id: MoveId::TakeDown,
                },
            ],
        },
        // #129 Magikarp
        EvosMovesEntry {
            species: Species::Magikarp,
            evolutions: vec![EvolutionMethod::Level {
                level: 20,
                species: Species::Gyarados,
            }],
            learnset: vec![LevelUpMove {
                level: 15,
                move_id: MoveId::Tackle,
            }],
        },
        // #130 Gyarados
        EvosMovesEntry {
            species: Species::Gyarados,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 20,
                    move_id: MoveId::Bite,
                },
                LevelUpMove {
                    level: 25,
                    move_id: MoveId::DragonRage,
                },
                LevelUpMove {
                    level: 32,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 41,
                    move_id: MoveId::HydroPump,
                },
                LevelUpMove {
                    level: 52,
                    move_id: MoveId::HyperBeam,
                },
            ],
        },
        // #131 Lapras
        EvosMovesEntry {
            species: Species::Lapras,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 16,
                    move_id: MoveId::Sing,
                },
                LevelUpMove {
                    level: 20,
                    move_id: MoveId::Mist,
                },
                LevelUpMove {
                    level: 25,
                    move_id: MoveId::BodySlam,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::ConfuseRay,
                },
                LevelUpMove {
                    level: 38,
                    move_id: MoveId::IceBeam,
                },
                LevelUpMove {
                    level: 46,
                    move_id: MoveId::HydroPump,
                },
            ],
        },
        // #132 Ditto
        EvosMovesEntry {
            species: Species::Ditto,
            evolutions: vec![],
            learnset: vec![],
        },
        // #133 Eevee
        EvosMovesEntry {
            species: Species::Eevee,
            evolutions: vec![
                EvolutionMethod::Item {
                    item: ItemId::FireStone,
                    min_level: 1,
                    species: Species::Flareon,
                },
                EvolutionMethod::Item {
                    item: ItemId::ThunderStone,
                    min_level: 1,
                    species: Species::Jolteon,
                },
                EvolutionMethod::Item {
                    item: ItemId::WaterStone,
                    min_level: 1,
                    species: Species::Vaporeon,
                },
            ],
            learnset: vec![
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::QuickAttack,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::TailWhip,
                },
                LevelUpMove {
                    level: 37,
                    move_id: MoveId::Bite,
                },
                LevelUpMove {
                    level: 45,
                    move_id: MoveId::TakeDown,
                },
            ],
        },
        // #134 Vaporeon
        EvosMovesEntry {
            species: Species::Vaporeon,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::QuickAttack,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::WaterGun,
                },
                LevelUpMove {
                    level: 37,
                    move_id: MoveId::TailWhip,
                },
                LevelUpMove {
                    level: 40,
                    move_id: MoveId::Bite,
                },
                LevelUpMove {
                    level: 42,
                    move_id: MoveId::AcidArmor,
                },
                LevelUpMove {
                    level: 44,
                    move_id: MoveId::Haze,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::Mist,
                },
                LevelUpMove {
                    level: 54,
                    move_id: MoveId::HydroPump,
                },
            ],
        },
        // #135 Jolteon
        EvosMovesEntry {
            species: Species::Jolteon,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::QuickAttack,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::Thundershock,
                },
                LevelUpMove {
                    level: 37,
                    move_id: MoveId::TailWhip,
                },
                LevelUpMove {
                    level: 40,
                    move_id: MoveId::ThunderWave,
                },
                LevelUpMove {
                    level: 42,
                    move_id: MoveId::DoubleKick,
                },
                LevelUpMove {
                    level: 44,
                    move_id: MoveId::Agility,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::PinMissile,
                },
                LevelUpMove {
                    level: 54,
                    move_id: MoveId::Thunder,
                },
            ],
        },
        // #136 Flareon
        EvosMovesEntry {
            species: Species::Flareon,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 27,
                    move_id: MoveId::QuickAttack,
                },
                LevelUpMove {
                    level: 31,
                    move_id: MoveId::Ember,
                },
                LevelUpMove {
                    level: 37,
                    move_id: MoveId::TailWhip,
                },
                LevelUpMove {
                    level: 40,
                    move_id: MoveId::Bite,
                },
                LevelUpMove {
                    level: 42,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 44,
                    move_id: MoveId::FireSpin,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::Rage,
                },
                LevelUpMove {
                    level: 54,
                    move_id: MoveId::Flamethrower,
                },
            ],
        },
        // #137 Porygon
        EvosMovesEntry {
            species: Species::Porygon,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 23,
                    move_id: MoveId::Psybeam,
                },
                LevelUpMove {
                    level: 28,
                    move_id: MoveId::Recover,
                },
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::Agility,
                },
                LevelUpMove {
                    level: 42,
                    move_id: MoveId::TriAttack,
                },
            ],
        },
        // #138 Omanyte
        EvosMovesEntry {
            species: Species::Omanyte,
            evolutions: vec![EvolutionMethod::Level {
                level: 40,
                species: Species::Omastar,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 34,
                    move_id: MoveId::HornAttack,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 46,
                    move_id: MoveId::SpikeCannon,
                },
                LevelUpMove {
                    level: 53,
                    move_id: MoveId::HydroPump,
                },
            ],
        },
        // #139 Omastar
        EvosMovesEntry {
            species: Species::Omastar,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 34,
                    move_id: MoveId::HornAttack,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 44,
                    move_id: MoveId::SpikeCannon,
                },
                LevelUpMove {
                    level: 49,
                    move_id: MoveId::HydroPump,
                },
            ],
        },
        // #140 Kabuto
        EvosMovesEntry {
            species: Species::Kabuto,
            evolutions: vec![EvolutionMethod::Level {
                level: 40,
                species: Species::Kabutops,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 34,
                    move_id: MoveId::Absorb,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::Slash,
                },
                LevelUpMove {
                    level: 44,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 49,
                    move_id: MoveId::HydroPump,
                },
            ],
        },
        // #141 Kabutops
        EvosMovesEntry {
            species: Species::Kabutops,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 34,
                    move_id: MoveId::Absorb,
                },
                LevelUpMove {
                    level: 39,
                    move_id: MoveId::Slash,
                },
                LevelUpMove {
                    level: 46,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 53,
                    move_id: MoveId::HydroPump,
                },
            ],
        },
        // #142 Aerodactyl
        EvosMovesEntry {
            species: Species::Aerodactyl,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 33,
                    move_id: MoveId::Supersonic,
                },
                LevelUpMove {
                    level: 38,
                    move_id: MoveId::Bite,
                },
                LevelUpMove {
                    level: 45,
                    move_id: MoveId::TakeDown,
                },
                LevelUpMove {
                    level: 54,
                    move_id: MoveId::HyperBeam,
                },
            ],
        },
        // #143 Snorlax
        EvosMovesEntry {
            species: Species::Snorlax,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::BodySlam,
                },
                LevelUpMove {
                    level: 41,
                    move_id: MoveId::Harden,
                },
                LevelUpMove {
                    level: 48,
                    move_id: MoveId::DoubleEdge,
                },
                LevelUpMove {
                    level: 56,
                    move_id: MoveId::HyperBeam,
                },
            ],
        },
        // #144 Articuno
        EvosMovesEntry {
            species: Species::Articuno,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 51,
                    move_id: MoveId::Blizzard,
                },
                LevelUpMove {
                    level: 55,
                    move_id: MoveId::Agility,
                },
                LevelUpMove {
                    level: 60,
                    move_id: MoveId::Mist,
                },
            ],
        },
        // #145 Zapdos
        EvosMovesEntry {
            species: Species::Zapdos,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 51,
                    move_id: MoveId::Thunder,
                },
                LevelUpMove {
                    level: 55,
                    move_id: MoveId::Agility,
                },
                LevelUpMove {
                    level: 60,
                    move_id: MoveId::LightScreen,
                },
            ],
        },
        // #146 Moltres
        EvosMovesEntry {
            species: Species::Moltres,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 51,
                    move_id: MoveId::Leer,
                },
                LevelUpMove {
                    level: 55,
                    move_id: MoveId::Agility,
                },
                LevelUpMove {
                    level: 60,
                    move_id: MoveId::SkyAttack,
                },
            ],
        },
        // #147 Dratini
        EvosMovesEntry {
            species: Species::Dratini,
            evolutions: vec![EvolutionMethod::Level {
                level: 30,
                species: Species::Dragonair,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 10,
                    move_id: MoveId::ThunderWave,
                },
                LevelUpMove {
                    level: 20,
                    move_id: MoveId::Agility,
                },
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::Slam,
                },
                LevelUpMove {
                    level: 40,
                    move_id: MoveId::DragonRage,
                },
                LevelUpMove {
                    level: 50,
                    move_id: MoveId::HyperBeam,
                },
            ],
        },
        // #148 Dragonair
        EvosMovesEntry {
            species: Species::Dragonair,
            evolutions: vec![EvolutionMethod::Level {
                level: 55,
                species: Species::Dragonite,
            }],
            learnset: vec![
                LevelUpMove {
                    level: 10,
                    move_id: MoveId::ThunderWave,
                },
                LevelUpMove {
                    level: 20,
                    move_id: MoveId::Agility,
                },
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::Slam,
                },
                LevelUpMove {
                    level: 45,
                    move_id: MoveId::DragonRage,
                },
                LevelUpMove {
                    level: 55,
                    move_id: MoveId::HyperBeam,
                },
            ],
        },
        // #149 Dragonite
        EvosMovesEntry {
            species: Species::Dragonite,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 10,
                    move_id: MoveId::ThunderWave,
                },
                LevelUpMove {
                    level: 20,
                    move_id: MoveId::Agility,
                },
                LevelUpMove {
                    level: 35,
                    move_id: MoveId::Slam,
                },
                LevelUpMove {
                    level: 45,
                    move_id: MoveId::DragonRage,
                },
                LevelUpMove {
                    level: 60,
                    move_id: MoveId::HyperBeam,
                },
            ],
        },
        // #150 Mewtwo
        EvosMovesEntry {
            species: Species::Mewtwo,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 63,
                    move_id: MoveId::Barrier,
                },
                LevelUpMove {
                    level: 66,
                    move_id: MoveId::PsychicM,
                },
                LevelUpMove {
                    level: 70,
                    move_id: MoveId::Recover,
                },
                LevelUpMove {
                    level: 75,
                    move_id: MoveId::Mist,
                },
                LevelUpMove {
                    level: 81,
                    move_id: MoveId::Amnesia,
                },
            ],
        },
        // #151 Mew
        EvosMovesEntry {
            species: Species::Mew,
            evolutions: vec![],
            learnset: vec![
                LevelUpMove {
                    level: 10,
                    move_id: MoveId::Transform,
                },
                LevelUpMove {
                    level: 20,
                    move_id: MoveId::MegaPunch,
                },
                LevelUpMove {
                    level: 30,
                    move_id: MoveId::Metronome,
                },
                LevelUpMove {
                    level: 40,
                    move_id: MoveId::PsychicM,
                },
            ],
        },
    ]
}

/// Get evolution and learnset data for a specific species.
pub fn get_evos_moves(species: Species) -> Option<&'static EvosMovesEntry> {
    use std::sync::LazyLock;
    static DATA: LazyLock<Vec<EvosMovesEntry>> = LazyLock::new(evos_moves_data);
    let dex = species as u8;
    if dex >= 1 && dex <= 151 {
        Some(&DATA[(dex - 1) as usize])
    } else {
        None
    }
}
