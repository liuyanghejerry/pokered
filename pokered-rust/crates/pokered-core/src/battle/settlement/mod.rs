pub mod evolution;
pub mod money;
pub mod settle;

#[cfg(test)]
mod evolution_tests;
#[cfg(test)]
mod money_tests;
#[cfg(test)]
mod settle_tests;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BattleOutcome {
    Win,
    Loss,
    Draw,
    Escaped,
    Captured,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionEvent {
    pub party_index: usize,
    pub old_species: pokered_data::species::Species,
    pub new_species: pokered_data::species::Species,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleSettlement {
    pub outcome: BattleOutcome,
    pub money_gained: u32,
    pub money_lost: u32,
    pub payday_bonus: u32,
    pub evolutions: Vec<EvolutionEvent>,
}
