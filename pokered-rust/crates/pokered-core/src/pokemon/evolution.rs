use crate::battle::settlement::evolution::{
    apply_evolution, check_item_evolution, check_level_evolution, check_trade_evolution,
};
use crate::battle::state::Pokemon;
use pokered_data::items::ItemId;
use pokered_data::species::Species;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvolutionTrigger {
    LevelUp,
    Trade,
    Item(ItemId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvolutionResult {
    pub from: Species,
    pub to: Species,
    pub trigger: EvolutionTrigger,
}

pub fn check_evolution(pokemon: &Pokemon, trigger: EvolutionTrigger) -> Option<Species> {
    match trigger {
        EvolutionTrigger::LevelUp => check_level_evolution(pokemon.species, pokemon.level),
        EvolutionTrigger::Trade => check_trade_evolution(pokemon.species, pokemon.level),
        EvolutionTrigger::Item(item) => check_item_evolution(pokemon.species, pokemon.level, item),
    }
}

pub fn try_evolve(pokemon: &mut Pokemon, trigger: EvolutionTrigger) -> Option<EvolutionResult> {
    let from = pokemon.species;
    let to = check_evolution(pokemon, trigger)?;
    apply_evolution(pokemon, to);
    Some(EvolutionResult { from, to, trigger })
}

pub fn evolve_party_after_battle(party: &mut [Pokemon]) -> Vec<EvolutionResult> {
    let mut results = Vec::new();
    for mon in party.iter_mut() {
        if mon.hp > 0 {
            if let Some(result) = try_evolve(mon, EvolutionTrigger::LevelUp) {
                results.push(result);
            }
        }
    }
    results
}
