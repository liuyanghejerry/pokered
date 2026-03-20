use pokered_data::moves::MoveId;

use super::state::{BattleState, BattlerState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnOrder {
    PlayerFirst,
    EnemyFirst,
}

/// Determine who moves first this turn.
///
/// Gen 1 priority rules (from ASM):
/// 1. Quick Attack has +1 priority (explicit check, not a generic system)
/// 2. Counter always goes second (priority -1)
/// 3. If both have same priority bracket: compare speed stats
/// 4. Equal speed: 50/50 coin flip
pub fn determine_order(state: &BattleState, random_byte: u8) -> TurnOrder {
    let player_move = state.player.selected_move;
    let enemy_move = state.enemy.selected_move;

    let player_priority = move_priority(player_move);
    let enemy_priority = move_priority(enemy_move);

    if player_priority > enemy_priority {
        return TurnOrder::PlayerFirst;
    }
    if enemy_priority > player_priority {
        return TurnOrder::EnemyFirst;
    }

    let player_speed = effective_speed(&state.player);
    let enemy_speed = effective_speed(&state.enemy);

    if player_speed > enemy_speed {
        TurnOrder::PlayerFirst
    } else if enemy_speed > player_speed {
        TurnOrder::EnemyFirst
    } else {
        // Equal speed: coin flip (ASM: random, compare with $80)
        if random_byte < 128 {
            TurnOrder::PlayerFirst
        } else {
            TurnOrder::EnemyFirst
        }
    }
}

fn move_priority(move_id: MoveId) -> i8 {
    match move_id {
        MoveId::QuickAttack => 1,
        MoveId::Counter => -1,
        _ => 0,
    }
}

fn effective_speed(battler: &BattlerState) -> u16 {
    use super::stat_stages::apply_stage;
    let base = battler.active_mon().speed;
    let staged = apply_stage(base, battler.stat_stages.speed);
    // Paralysis quarters speed (ASM: QuarterSpeedDueToParalysis)
    if battler.active_mon().status == super::state::StatusCondition::Paralysis {
        (staged / 4).max(1)
    } else {
        staged
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::state::*;
    use pokered_data::species::Species;
    use pokered_data::types::PokemonType;

    fn pokemon_with_speed(speed: u16) -> Pokemon {
        Pokemon {
            species: Species::Pikachu,
            level: 50,
            hp: 100,
            max_hp: 100,
            attack: 80,
            defense: 60,
            speed,
            special: 70,
            type1: PokemonType::Electric,
            type2: PokemonType::Electric,
            moves: [
                MoveId::Thundershock,
                MoveId::QuickAttack,
                MoveId::None,
                MoveId::None,
            ],
            pp: [30, 30, 0, 0],
            status: StatusCondition::None,
            dv_bytes: [0xFF, 0xFF],
            stat_exp: [0; 5],
            total_exp: 0,
            is_traded: false,
        }
    }

    fn make_battle(player_speed: u16, enemy_speed: u16) -> BattleState {
        let p = vec![pokemon_with_speed(player_speed)];
        let e = vec![pokemon_with_speed(enemy_speed)];
        let mut state = new_battle_state(BattleType::Wild, p, e);
        state.player.selected_move = MoveId::Thundershock;
        state.enemy.selected_move = MoveId::Thundershock;
        state
    }

    #[test]
    fn faster_pokemon_goes_first() {
        let state = make_battle(100, 50);
        assert_eq!(determine_order(&state, 0), TurnOrder::PlayerFirst);

        let state = make_battle(50, 100);
        assert_eq!(determine_order(&state, 0), TurnOrder::EnemyFirst);
    }

    #[test]
    fn equal_speed_coin_flip() {
        let state = make_battle(80, 80);
        assert_eq!(determine_order(&state, 0), TurnOrder::PlayerFirst);
        assert_eq!(determine_order(&state, 127), TurnOrder::PlayerFirst);
        assert_eq!(determine_order(&state, 128), TurnOrder::EnemyFirst);
        assert_eq!(determine_order(&state, 255), TurnOrder::EnemyFirst);
    }

    #[test]
    fn quick_attack_priority() {
        let mut state = make_battle(50, 100);
        state.player.selected_move = MoveId::QuickAttack;
        state.enemy.selected_move = MoveId::Thundershock;
        assert_eq!(determine_order(&state, 0), TurnOrder::PlayerFirst);
    }

    #[test]
    fn counter_goes_last() {
        let mut state = make_battle(100, 50);
        state.player.selected_move = MoveId::Counter;
        state.enemy.selected_move = MoveId::Thundershock;
        assert_eq!(determine_order(&state, 0), TurnOrder::EnemyFirst);
    }

    #[test]
    fn both_quick_attack_uses_speed() {
        let mut state = make_battle(100, 50);
        state.player.selected_move = MoveId::QuickAttack;
        state.enemy.selected_move = MoveId::QuickAttack;
        assert_eq!(determine_order(&state, 0), TurnOrder::PlayerFirst);
    }

    #[test]
    fn paralysis_quarters_speed() {
        let mut state = make_battle(100, 50);
        state.player.active_mon_mut().status = StatusCondition::Paralysis;
        assert_eq!(determine_order(&state, 0), TurnOrder::EnemyFirst);
    }
}
