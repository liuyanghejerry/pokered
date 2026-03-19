use serde::{Deserialize, Serialize};
use crate::species::{Species, GrowthRate};
use crate::types::PokemonType;
use crate::moves::MoveId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseStats {
    pub species: Species,
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub speed: u8,
    pub special: u8,
    pub type1: PokemonType,
    pub type2: PokemonType,
    pub catch_rate: u8,
    pub base_exp: u8,
    pub initial_moves: [MoveId; 4],
    pub growth_rate: GrowthRate,
    pub tm_hm_flags: [u8; 7],
}
