use super::evolution::*;
use crate::battle::state::{Pokemon, StatusCondition};
use pokered_data::items::ItemId;
use pokered_data::moves::MoveId;
use pokered_data::species::Species;
use pokered_data::types::PokemonType;

fn make_pokemon(species: Species, level: u8) -> Pokemon {
    Pokemon {
        species,
        level,
        hp: 50,
        max_hp: 50,
        attack: 40,
        defense: 40,
        speed: 40,
        special: 40,
        type1: PokemonType::Normal,
        type2: PokemonType::Normal,
        moves: [MoveId::Tackle, MoveId::None, MoveId::None, MoveId::None],
        pp: [35, 0, 0, 0],
        pp_ups: [0; 4],
        status: StatusCondition::None,
        dv_bytes: [0xAA, 0xAA],
        stat_exp: [0; 5],
        total_exp: 0,
        is_traded: false,
    }
}

#[test]
fn level_evolution_bulbasaur_at_16() {
    let mut mon = make_pokemon(Species::Bulbasaur, 16);
    let result = try_evolve(&mut mon, EvolutionTrigger::LevelUp);
    assert!(result.is_some());
    let r = result.unwrap();
    assert_eq!(r.from, Species::Bulbasaur);
    assert_eq!(r.to, Species::Ivysaur);
    assert_eq!(r.trigger, EvolutionTrigger::LevelUp);
    assert_eq!(mon.species, Species::Ivysaur);
}

#[test]
fn level_evolution_bulbasaur_at_15_no_evolve() {
    let mut mon = make_pokemon(Species::Bulbasaur, 15);
    let result = try_evolve(&mut mon, EvolutionTrigger::LevelUp);
    assert!(result.is_none());
    assert_eq!(mon.species, Species::Bulbasaur);
}

#[test]
fn trade_evolution_kadabra() {
    let mut mon = make_pokemon(Species::Kadabra, 30);
    let result = try_evolve(&mut mon, EvolutionTrigger::Trade);
    assert!(result.is_some());
    let r = result.unwrap();
    assert_eq!(r.from, Species::Kadabra);
    assert_eq!(r.to, Species::Alakazam);
    assert_eq!(mon.species, Species::Alakazam);
}

#[test]
fn trade_evolution_pikachu_no_evolve() {
    let mut mon = make_pokemon(Species::Pikachu, 50);
    let result = try_evolve(&mut mon, EvolutionTrigger::Trade);
    assert!(result.is_none());
    assert_eq!(mon.species, Species::Pikachu);
}

#[test]
fn item_evolution_pikachu_thunder_stone() {
    let mut mon = make_pokemon(Species::Pikachu, 25);
    let result = try_evolve(&mut mon, EvolutionTrigger::Item(ItemId::ThunderStone));
    assert!(result.is_some());
    let r = result.unwrap();
    assert_eq!(r.from, Species::Pikachu);
    assert_eq!(r.to, Species::Raichu);
    assert_eq!(mon.species, Species::Raichu);
}

#[test]
fn item_evolution_wrong_item() {
    let mut mon = make_pokemon(Species::Pikachu, 25);
    let result = try_evolve(&mut mon, EvolutionTrigger::Item(ItemId::FireStone));
    assert!(result.is_none());
    assert_eq!(mon.species, Species::Pikachu);
}

#[test]
fn no_evolution_for_mew() {
    let mut mon = make_pokemon(Species::Mew, 100);
    assert!(try_evolve(&mut mon, EvolutionTrigger::LevelUp).is_none());
    assert!(try_evolve(&mut mon, EvolutionTrigger::Trade).is_none());
    assert!(try_evolve(&mut mon, EvolutionTrigger::Item(ItemId::MoonStone)).is_none());
}

#[test]
fn check_evolution_without_applying() {
    let mon = make_pokemon(Species::Bulbasaur, 16);
    let target = check_evolution(&mon, EvolutionTrigger::LevelUp);
    assert_eq!(target, Some(Species::Ivysaur));
}

#[test]
fn evolve_party_after_battle_only_alive() {
    let mut party = vec![
        make_pokemon(Species::Bulbasaur, 16),
        make_pokemon(Species::Charmander, 16),
        make_pokemon(Species::Squirtle, 16),
    ];
    party[1].hp = 0;

    let results = evolve_party_after_battle(&mut party);
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].from, Species::Bulbasaur);
    assert_eq!(results[0].to, Species::Ivysaur);
    assert_eq!(results[1].from, Species::Squirtle);
    assert_eq!(results[1].to, Species::Wartortle);

    assert_eq!(party[0].species, Species::Ivysaur);
    assert_eq!(party[1].species, Species::Charmander);
    assert_eq!(party[2].species, Species::Wartortle);
}

#[test]
fn evolve_party_no_evolutions() {
    let mut party = vec![
        make_pokemon(Species::Bulbasaur, 10),
        make_pokemon(Species::Mew, 100),
    ];
    let results = evolve_party_after_battle(&mut party);
    assert!(results.is_empty());
}

#[test]
fn evolution_updates_stats() {
    let mut mon = make_pokemon(Species::Bulbasaur, 16);
    let old_max_hp = mon.max_hp;
    try_evolve(&mut mon, EvolutionTrigger::LevelUp);
    assert_ne!(mon.max_hp, old_max_hp);
}

#[test]
fn eevee_fire_stone_evolution() {
    let mut mon = make_pokemon(Species::Eevee, 25);
    let result = try_evolve(&mut mon, EvolutionTrigger::Item(ItemId::FireStone));
    assert!(result.is_some());
    assert_eq!(mon.species, Species::Flareon);
}

#[test]
fn eevee_water_stone_evolution() {
    let mut mon = make_pokemon(Species::Eevee, 25);
    let result = try_evolve(&mut mon, EvolutionTrigger::Item(ItemId::WaterStone));
    assert!(result.is_some());
    assert_eq!(mon.species, Species::Vaporeon);
}

#[test]
fn eevee_thunder_stone_evolution() {
    let mut mon = make_pokemon(Species::Eevee, 25);
    let result = try_evolve(&mut mon, EvolutionTrigger::Item(ItemId::ThunderStone));
    assert!(result.is_some());
    assert_eq!(mon.species, Species::Jolteon);
}
