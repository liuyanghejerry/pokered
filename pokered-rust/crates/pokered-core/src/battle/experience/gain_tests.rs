#[cfg(test)]
mod tests {
    use super::super::gain::*;
    use crate::battle::state::*;
    use pokered_data::moves::MoveId;
    use pokered_data::species::Species;
    use pokered_data::types::PokemonType;

    fn make_mon(species: Species, level: u8, total_exp: u32, is_traded: bool) -> Pokemon {
        Pokemon {
            species,
            level,
            hp: 100,
            max_hp: 100,
            attack: 50,
            defense: 50,
            speed: 50,
            special: 50,
            type1: PokemonType::Normal,
            type2: PokemonType::Normal,
            moves: [MoveId::Tackle, MoveId::None, MoveId::None, MoveId::None],
            pp: [35, 0, 0, 0],
            pp_ups: [0; 4],
            status: StatusCondition::None,
            dv_bytes: [0xFF, 0xFF],
            stat_exp: [0; 5],
            total_exp,
            is_traded,
        }
    }

    #[test]
    fn calc_exp_gain_basic() {
        // base_exp=64, enemy_level=10: 64*10/7 = 91
        assert_eq!(calc_exp_gain(64, 10, false, false), 91);
    }

    #[test]
    fn calc_exp_gain_traded_boost() {
        // 91 * 3/2 = 136
        assert_eq!(calc_exp_gain(64, 10, true, false), 136);
    }

    #[test]
    fn calc_exp_gain_trainer_boost() {
        // 91 * 3/2 = 136
        assert_eq!(calc_exp_gain(64, 10, false, true), 136);
    }

    #[test]
    fn calc_exp_gain_both_boosts() {
        // 91 * 3/2 = 136, then 136 * 3/2 = 204
        assert_eq!(calc_exp_gain(64, 10, true, true), 204);
    }

    #[test]
    fn add_stat_exp_accumulates() {
        let mut mon = make_mon(Species::Pikachu, 25, 0, false);
        let base = pokered_data::pokemon_data::get_base_stats(Species::Pikachu).unwrap();
        add_stat_exp(&mut mon, base);
        assert_eq!(mon.stat_exp[0], 35); // hp
        assert_eq!(mon.stat_exp[1], 55); // atk
        assert_eq!(mon.stat_exp[2], 30); // def
        assert_eq!(mon.stat_exp[3], 90); // spd
        assert_eq!(mon.stat_exp[4], 50); // spc
    }

    #[test]
    fn add_stat_exp_saturates_at_max() {
        let mut mon = make_mon(Species::Pikachu, 25, 0, false);
        mon.stat_exp = [0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF];
        let base = pokered_data::pokemon_data::get_base_stats(Species::Pikachu).unwrap();
        add_stat_exp(&mut mon, base);
        assert_eq!(mon.stat_exp[0], 0xFFFF);
    }

    #[test]
    fn gain_experience_single_mon() {
        // Bulbasaur at level 10 defeat a level 10 Pidgey (base_exp=55)
        // 55*10/7=78 EXP gained, not enough to level from 10 to 11
        let mon = make_mon(Species::Bulbasaur, 10, 560, false);
        let mut state = new_battle_state(
            BattleType::Wild,
            vec![mon],
            vec![make_mon(Species::Pidgey, 10, 0, false)],
        );
        state.party_gain_exp_flags[0] = true;

        let result = gain_experience(&mut state, Species::Pidgey, 10, false);
        // 560 + 78 = 638, level 11 needs 742, so no level up
        assert_eq!(state.player.party[0].total_exp, 638);
        assert!(result.leveled_up.is_empty());
    }

    #[test]
    fn gain_experience_triggers_level_up() {
        // Bulbasaur at level 5 with 215 EXP. Gaining 78 → 293.
        // Level 7 needs 236 ≤ 293, so levels up from 5 to 7.
        let mon = make_mon(Species::Bulbasaur, 5, 215, false);
        let mut state = new_battle_state(
            BattleType::Wild,
            vec![mon],
            vec![make_mon(Species::Pidgey, 10, 0, false)],
        );
        state.party_gain_exp_flags[0] = true;

        let result = gain_experience(&mut state, Species::Pidgey, 10, false);
        assert_eq!(result.leveled_up, vec![0]);
        assert!(state.player.party[0].level > 5);
    }

    #[test]
    fn no_gainers_no_exp() {
        let mon = make_mon(Species::Bulbasaur, 5, 0, false);
        let mut state = new_battle_state(
            BattleType::Wild,
            vec![mon],
            vec![make_mon(Species::Pidgey, 10, 0, false)],
        );
        // No flags set
        let result = gain_experience(&mut state, Species::Pidgey, 10, false);
        assert!(result.leveled_up.is_empty());
        assert_eq!(state.player.party[0].total_exp, 0);
    }
}
