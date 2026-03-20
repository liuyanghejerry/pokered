use pokered_data::trainer_data::TrainerClass;

use super::evolution::{apply_evolution, check_level_evolution};
use super::money::{calc_blackout_penalty, calc_prize_money, calc_total_winnings};
use super::{BattleOutcome, BattleSettlement, EvolutionEvent};
use crate::battle::state::{BattleState, BattleType};

pub fn settle_battle(
    state: &mut BattleState,
    outcome: BattleOutcome,
    trainer_class: Option<TrainerClass>,
    player_money: u32,
) -> BattleSettlement {
    let mut settlement = BattleSettlement {
        outcome,
        money_gained: 0,
        money_lost: 0,
        payday_bonus: 0,
        evolutions: Vec::new(),
    };

    match outcome {
        BattleOutcome::Win => {
            let prize = if state.battle_type == BattleType::Trainer {
                if let Some(class) = trainer_class {
                    let last_level = last_enemy_level(state);
                    calc_prize_money(class, last_level)
                } else {
                    0
                }
            } else {
                0
            };

            let payday = state.total_payday_money;
            settlement.payday_bonus = payday;
            settlement.money_gained = calc_total_winnings(prize, payday);

            process_evolutions(&mut state.player.party, &mut settlement.evolutions);
        }
        BattleOutcome::Loss => {
            settlement.money_lost = calc_blackout_penalty(player_money);
        }
        BattleOutcome::Escaped | BattleOutcome::Captured | BattleOutcome::Draw => {
            process_evolutions(&mut state.player.party, &mut settlement.evolutions);
        }
    }

    settlement
}

fn last_enemy_level(state: &BattleState) -> u8 {
    state.enemy.party.last().map(|m| m.level).unwrap_or(1)
}

fn process_evolutions(
    party: &mut [crate::battle::state::Pokemon],
    events: &mut Vec<EvolutionEvent>,
) {
    for (i, mon) in party.iter_mut().enumerate() {
        if mon.hp == 0 {
            continue;
        }
        if let Some(new_species) = check_level_evolution(mon.species, mon.level) {
            let old_species = mon.species;
            apply_evolution(mon, new_species);
            events.push(EvolutionEvent {
                party_index: i,
                old_species,
                new_species,
            });
        }
    }
}
