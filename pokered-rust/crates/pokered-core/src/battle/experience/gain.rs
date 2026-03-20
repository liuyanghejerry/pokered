use pokered_data::pokemon_data::{get_base_stats, BaseStats};
use pokered_data::species::Species;

use crate::battle::state::{BattleState, BattleType, Pokemon};

use super::growth::max_exp;
use super::level_up::process_level_up;

pub fn calc_exp_gain(base_exp: u8, enemy_level: u8, is_traded: bool, is_trainer: bool) -> u32 {
    let raw = (base_exp as u32 * enemy_level as u32) / 7;
    let mut exp = raw;
    if is_traded {
        exp = (exp * 3) / 2;
    }
    if is_trainer {
        exp = (exp * 3) / 2;
    }
    exp
}

pub fn add_stat_exp(mon: &mut Pokemon, enemy_base: &BaseStats) {
    mon.stat_exp[0] = mon.stat_exp[0].saturating_add(enemy_base.hp as u16);
    mon.stat_exp[1] = mon.stat_exp[1].saturating_add(enemy_base.attack as u16);
    mon.stat_exp[2] = mon.stat_exp[2].saturating_add(enemy_base.defense as u16);
    mon.stat_exp[3] = mon.stat_exp[3].saturating_add(enemy_base.speed as u16);
    mon.stat_exp[4] = mon.stat_exp[4].saturating_add(enemy_base.special as u16);
}

pub struct GainExpResult {
    pub leveled_up: Vec<usize>,
    pub new_moves: Vec<(usize, pokered_data::moves::MoveId)>,
}

pub fn gain_experience(
    state: &mut BattleState,
    defeated_species: Species,
    defeated_level: u8,
    has_exp_all: bool,
) -> GainExpResult {
    let enemy_base = match get_base_stats(defeated_species) {
        Some(b) => b,
        None => {
            return GainExpResult {
                leveled_up: vec![],
                new_moves: vec![],
            }
        }
    };

    let is_trainer = state.battle_type == BattleType::Trainer;

    let num_gainers = state.party_gain_exp_flags.iter().filter(|&&f| f).count() as u32;
    if num_gainers == 0 && !has_exp_all {
        return GainExpResult {
            leveled_up: vec![],
            new_moves: vec![],
        };
    }

    let mut leveled_up = vec![];
    let mut new_moves = vec![];

    if has_exp_all {
        let halved_base_exp = enemy_base.base_exp / 2;
        let halved_base = make_halved_base(enemy_base);

        // Pass 1: fighters only (with halved values)
        for i in 0..state.player.party.len() {
            if state.party_gain_exp_flags[i] {
                let mon = &mut state.player.party[i];
                add_stat_exp(mon, &halved_base);
                let exp = calc_exp_gain(halved_base_exp, defeated_level, mon.is_traded, is_trainer);
                let growth_rate = get_base_stats(mon.species).map(|b| b.growth_rate).unwrap();
                let max = max_exp(growth_rate);
                mon.total_exp = (mon.total_exp + exp).min(max);

                let result = process_level_up(mon);
                if result.leveled_up {
                    leveled_up.push(i);
                }
                for m in result.learned_moves {
                    new_moves.push((i, m));
                }
            }
        }

        // Pass 2: all party members (with halved values)
        for i in 0..state.player.party.len() {
            let mon = &mut state.player.party[i];
            add_stat_exp(mon, &halved_base);
            let exp = calc_exp_gain(halved_base_exp, defeated_level, mon.is_traded, is_trainer);
            let growth_rate = get_base_stats(mon.species).map(|b| b.growth_rate).unwrap();
            let max = max_exp(growth_rate);
            mon.total_exp = (mon.total_exp + exp).min(max);

            let result = process_level_up(mon);
            if result.leveled_up {
                if !leveled_up.contains(&i) {
                    leveled_up.push(i);
                }
            }
            for m in result.learned_moves {
                new_moves.push((i, m));
            }
        }
    } else {
        // Normal: divide EXP among gainers
        let effective_base_exp = if num_gainers > 1 {
            enemy_base.base_exp / num_gainers as u8
        } else {
            enemy_base.base_exp
        };

        for i in 0..state.player.party.len() {
            if !state.party_gain_exp_flags[i] {
                continue;
            }
            let mon = &mut state.player.party[i];
            add_stat_exp(mon, enemy_base);
            let exp = calc_exp_gain(
                effective_base_exp,
                defeated_level,
                mon.is_traded,
                is_trainer,
            );
            let growth_rate = get_base_stats(mon.species).map(|b| b.growth_rate).unwrap();
            let max = max_exp(growth_rate);
            mon.total_exp = (mon.total_exp + exp).min(max);

            let result = process_level_up(mon);
            if result.leveled_up {
                leveled_up.push(i);
            }
            for m in result.learned_moves {
                new_moves.push((i, m));
            }
        }
    }

    GainExpResult {
        leveled_up,
        new_moves,
    }
}

fn make_halved_base(base: &BaseStats) -> BaseStats {
    BaseStats {
        species: base.species,
        hp: base.hp / 2,
        attack: base.attack / 2,
        defense: base.defense / 2,
        speed: base.speed / 2,
        special: base.special / 2,
        type1: base.type1,
        type2: base.type2,
        catch_rate: base.catch_rate,
        base_exp: base.base_exp / 2,
        initial_moves: base.initial_moves,
        growth_rate: base.growth_rate,
        tm_hm_flags: base.tm_hm_flags,
    }
}
