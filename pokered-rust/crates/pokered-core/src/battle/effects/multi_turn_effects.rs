use pokered_data::move_data::MoveData;

use crate::battle::state::{status1, status2, BattleState};

use super::{EffectRandoms, EffectResult};

pub fn apply_charge(state: &mut BattleState, move_data: &MoveData) -> EffectResult {
    let attacker = state.attacker_mut();
    if attacker.has_status1(status1::CHARGING_UP) {
        attacker.clear_status1(status1::CHARGING_UP);
        return EffectResult::NoEffect;
    }
    attacker.set_status1(status1::CHARGING_UP);
    let _ = move_data;
    EffectResult::MultiTurnContinue
}

pub fn apply_fly(state: &mut BattleState, move_data: &MoveData) -> EffectResult {
    let attacker = state.attacker_mut();
    if attacker.has_status1(status1::CHARGING_UP) {
        attacker.clear_status1(status1::CHARGING_UP);
        attacker.clear_status1(status1::INVULNERABLE);
        return EffectResult::NoEffect;
    }
    attacker.set_status1(status1::CHARGING_UP);
    attacker.set_status1(status1::INVULNERABLE);
    let _ = move_data;
    EffectResult::MultiTurnContinue
}

pub fn apply_trapping(state: &mut BattleState, randoms: &EffectRandoms) -> EffectResult {
    let attacker = state.attacker_mut();
    if attacker.has_status1(status1::USING_TRAPPING_MOVE) {
        attacker.num_attacks_left -= 1;
        if attacker.num_attacks_left == 0 {
            attacker.clear_status1(status1::USING_TRAPPING_MOVE);
        }
        return EffectResult::NoEffect;
    }
    // Gen 1: 2-5 turns (same distribution as multi-hit)
    let turns = match randoms.multi_hit_roll & 0x03 {
        0 => 2,
        1 => 3,
        2 => 4,
        _ => 5,
    };
    attacker.num_attacks_left = turns - 1;
    attacker.set_status1(status1::USING_TRAPPING_MOVE);
    EffectResult::NoEffect
}

pub fn apply_bide(state: &mut BattleState) -> EffectResult {
    let attacker = state.attacker_mut();
    if attacker.has_status1(status1::STORING_ENERGY) {
        attacker.num_attacks_left -= 1;
        if attacker.num_attacks_left == 0 {
            attacker.clear_status1(status1::STORING_ENERGY);
            let unleash_damage = attacker.bide_accumulated_damage * 2;
            attacker.bide_accumulated_damage = 0;
            state.damage = unleash_damage;
            return EffectResult::SpecialDamageDealt {
                damage: unleash_damage,
            };
        }
        return EffectResult::MultiTurnContinue;
    }
    attacker.set_status1(status1::STORING_ENERGY);
    attacker.num_attacks_left = 2;
    attacker.bide_accumulated_damage = 0;
    EffectResult::MultiTurnContinue
}

pub fn apply_thrash(state: &mut BattleState, randoms: &EffectRandoms) -> EffectResult {
    let attacker = state.attacker_mut();
    if attacker.has_status1(status1::THRASHING_ABOUT) {
        attacker.num_attacks_left -= 1;
        if attacker.num_attacks_left == 0 {
            attacker.clear_status1(status1::THRASHING_ABOUT);
            // Confusion after thrashing: 2-5 turns
            let conf_turns = (randoms.duration_roll & 0x03) + 2;
            attacker.set_status1(status1::CONFUSED);
            attacker.confused_turns_left = conf_turns;
            return EffectResult::ConfusionApplied;
        }
        return EffectResult::NoEffect;
    }
    // 3-4 turns of thrashing (Gen 1: random & 1 + 2 = 2 or 3, but total = 3 or 4 including first)
    let turns = (randoms.duration_roll & 0x01) + 2;
    attacker.num_attacks_left = turns;
    attacker.num_attacks_left -= 1;
    attacker.set_status1(status1::THRASHING_ABOUT);
    EffectResult::NoEffect
}

pub fn apply_rage(state: &mut BattleState) -> EffectResult {
    let attacker = state.attacker_mut();
    attacker.set_status2(status2::USING_RAGE);
    EffectResult::RageActivated
}

pub fn apply_hyper_beam(state: &mut BattleState) -> EffectResult {
    let defender = state.defender();
    if defender.active_mon().hp == 0 {
        return EffectResult::NoEffect;
    }
    let attacker = state.attacker_mut();
    attacker.set_status2(status2::NEEDS_TO_RECHARGE);
    EffectResult::MustRecharge
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::state::*;
    use pokered_data::moves::{MoveEffect, MoveId};
    use pokered_data::species::Species;
    use pokered_data::types::PokemonType;

    fn make_pokemon() -> Pokemon {
        Pokemon {
            species: Species::Pikachu,
            level: 50,
            hp: 200,
            max_hp: 200,
            attack: 100,
            defense: 80,
            speed: 110,
            special: 80,
            type1: PokemonType::Electric,
            type2: PokemonType::Electric,
            moves: [MoveId::Tackle, MoveId::None, MoveId::None, MoveId::None],
            pp: [35, 0, 0, 0],
            status: StatusCondition::None,
            dv_bytes: [0xFF, 0xFF],
            stat_exp: [0; 5],
            total_exp: 0,
            is_traded: false,
        }
    }

    fn make_state() -> BattleState {
        new_battle_state(BattleType::Wild, vec![make_pokemon()], vec![make_pokemon()])
    }

    fn solarbeam_data() -> MoveData {
        MoveData {
            id: MoveId::Solarbeam,
            effect: MoveEffect::ChargeEffect,
            power: 120,
            move_type: PokemonType::Grass,
            accuracy: 100,
            pp: 10,
        }
    }

    fn fly_data() -> MoveData {
        MoveData {
            id: MoveId::Fly,
            effect: MoveEffect::FlyEffect,
            power: 70,
            move_type: PokemonType::Flying,
            accuracy: 95,
            pp: 15,
        }
    }

    fn default_randoms() -> EffectRandoms {
        EffectRandoms {
            side_effect_roll: 0,
            duration_roll: 0,
            multi_hit_roll: 0,
        }
    }

    #[test]
    fn charge_first_turn_sets_charging() {
        let mut state = make_state();
        let result = apply_charge(&mut state, &solarbeam_data());
        assert_eq!(result, EffectResult::MultiTurnContinue);
        assert!(state.player.has_status1(status1::CHARGING_UP));
    }

    #[test]
    fn charge_second_turn_clears_charging() {
        let mut state = make_state();
        state.player.set_status1(status1::CHARGING_UP);
        let result = apply_charge(&mut state, &solarbeam_data());
        assert_eq!(result, EffectResult::NoEffect);
        assert!(!state.player.has_status1(status1::CHARGING_UP));
    }

    #[test]
    fn fly_sets_invulnerable() {
        let mut state = make_state();
        let result = apply_fly(&mut state, &fly_data());
        assert_eq!(result, EffectResult::MultiTurnContinue);
        assert!(state.player.has_status1(status1::INVULNERABLE));
        assert!(state.player.has_status1(status1::CHARGING_UP));
    }

    #[test]
    fn fly_second_turn_clears_both() {
        let mut state = make_state();
        state.player.set_status1(status1::CHARGING_UP);
        state.player.set_status1(status1::INVULNERABLE);
        let result = apply_fly(&mut state, &fly_data());
        assert_eq!(result, EffectResult::NoEffect);
        assert!(!state.player.has_status1(status1::INVULNERABLE));
        assert!(!state.player.has_status1(status1::CHARGING_UP));
    }

    #[test]
    fn trapping_sets_turns() {
        let mut state = make_state();
        let randoms = EffectRandoms {
            side_effect_roll: 0,
            duration_roll: 0,
            multi_hit_roll: 1,
        };
        apply_trapping(&mut state, &randoms);
        assert!(state.player.has_status1(status1::USING_TRAPPING_MOVE));
        assert_eq!(state.player.num_attacks_left, 2);
    }

    #[test]
    fn trapping_decrements_and_clears() {
        let mut state = make_state();
        state.player.set_status1(status1::USING_TRAPPING_MOVE);
        state.player.num_attacks_left = 1;
        let randoms = default_randoms();
        apply_trapping(&mut state, &randoms);
        assert!(!state.player.has_status1(status1::USING_TRAPPING_MOVE));
        assert_eq!(state.player.num_attacks_left, 0);
    }

    #[test]
    fn bide_stores_then_unleashes() {
        let mut state = make_state();
        let result = apply_bide(&mut state);
        assert_eq!(result, EffectResult::MultiTurnContinue);
        assert!(state.player.has_status1(status1::STORING_ENERGY));

        state.player.bide_accumulated_damage = 30;
        let result = apply_bide(&mut state);
        assert_eq!(result, EffectResult::MultiTurnContinue);

        let result = apply_bide(&mut state);
        assert_eq!(result, EffectResult::SpecialDamageDealt { damage: 60 });
        assert!(!state.player.has_status1(status1::STORING_ENERGY));
    }

    #[test]
    fn thrash_confuses_after_end() {
        let mut state = make_state();
        let randoms = EffectRandoms {
            side_effect_roll: 0,
            duration_roll: 0,
            multi_hit_roll: 0,
        };
        apply_thrash(&mut state, &randoms);
        assert!(state.player.has_status1(status1::THRASHING_ABOUT));

        apply_thrash(&mut state, &randoms);
        assert!(state.player.has_status1(status1::CONFUSED));
        assert!(!state.player.has_status1(status1::THRASHING_ABOUT));
    }

    #[test]
    fn rage_sets_flag() {
        let mut state = make_state();
        let result = apply_rage(&mut state);
        assert_eq!(result, EffectResult::RageActivated);
        assert!(state.player.has_status2(status2::USING_RAGE));
    }

    #[test]
    fn hyper_beam_sets_recharge() {
        let mut state = make_state();
        let result = apply_hyper_beam(&mut state);
        assert_eq!(result, EffectResult::MustRecharge);
        assert!(state.player.has_status2(status2::NEEDS_TO_RECHARGE));
    }

    #[test]
    fn hyper_beam_no_recharge_if_target_fainted() {
        let mut state = make_state();
        state.enemy.active_mon_mut().hp = 0;
        let result = apply_hyper_beam(&mut state);
        assert_eq!(result, EffectResult::NoEffect);
        assert!(!state.player.has_status2(status2::NEEDS_TO_RECHARGE));
    }
}
