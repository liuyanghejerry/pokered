#[cfg(test)]
mod tests {
    use pokered_data::moves::MoveId;
    use pokered_data::species::Species;
    use pokered_data::trainer_data::TrainerClass;
    use pokered_data::types::PokemonType;

    use crate::battle::settlement::settle::settle_battle;
    use crate::battle::settlement::BattleOutcome;
    use crate::battle::state::*;

    fn make_test_pokemon(species: Species, level: u8) -> Pokemon {
        Pokemon {
            species,
            level,
            hp: 50,
            max_hp: 50,
            attack: 30,
            defense: 30,
            speed: 30,
            special: 30,
            type1: PokemonType::Normal,
            type2: PokemonType::Normal,
            moves: [MoveId::Tackle, MoveId::None, MoveId::None, MoveId::None],
            pp: [35, 0, 0, 0],
            status: StatusCondition::None,
            dv_bytes: [0xFF, 0xFF],
            stat_exp: [0; 5],
            total_exp: 0,
            is_traded: false,
        }
    }

    #[test]
    fn win_trainer_battle_gains_money() {
        let player_party = vec![make_test_pokemon(Species::Pikachu, 25)];
        let enemy_party = vec![make_test_pokemon(Species::Geodude, 14)];
        let mut state = new_battle_state(BattleType::Trainer, player_party, enemy_party);

        let result = settle_battle(
            &mut state,
            BattleOutcome::Win,
            Some(TrainerClass::Brock),
            5000,
        );

        assert_eq!(result.outcome, BattleOutcome::Win);
        assert_eq!(result.money_gained, 9900 * 14);
        assert_eq!(result.money_lost, 0);
    }

    #[test]
    fn win_wild_battle_no_prize_money() {
        let player_party = vec![make_test_pokemon(Species::Pikachu, 25)];
        let enemy_party = vec![make_test_pokemon(Species::Rattata, 5)];
        let mut state = new_battle_state(BattleType::Wild, player_party, enemy_party);

        let result = settle_battle(&mut state, BattleOutcome::Win, None, 5000);

        assert_eq!(result.money_gained, 0);
    }

    #[test]
    fn win_with_payday_adds_bonus() {
        let player_party = vec![make_test_pokemon(Species::Pikachu, 25)];
        let enemy_party = vec![make_test_pokemon(Species::Rattata, 5)];
        let mut state = new_battle_state(BattleType::Wild, player_party, enemy_party);
        state.total_payday_money = 500;

        let result = settle_battle(&mut state, BattleOutcome::Win, None, 5000);

        assert_eq!(result.money_gained, 500);
        assert_eq!(result.payday_bonus, 500);
    }

    #[test]
    fn loss_halves_money() {
        let player_party = vec![make_test_pokemon(Species::Pikachu, 25)];
        let enemy_party = vec![make_test_pokemon(Species::Rattata, 5)];
        let mut state = new_battle_state(BattleType::Wild, player_party, enemy_party);

        let result = settle_battle(&mut state, BattleOutcome::Loss, None, 10_000);

        assert_eq!(result.outcome, BattleOutcome::Loss);
        assert_eq!(result.money_lost, 5_000);
        assert_eq!(result.money_gained, 0);
    }

    #[test]
    fn evolution_triggers_on_win() {
        let player_party = vec![make_test_pokemon(Species::Bulbasaur, 16)];
        let enemy_party = vec![make_test_pokemon(Species::Rattata, 5)];
        let mut state = new_battle_state(BattleType::Wild, player_party, enemy_party);

        let result = settle_battle(&mut state, BattleOutcome::Win, None, 0);

        assert_eq!(result.evolutions.len(), 1);
        assert_eq!(result.evolutions[0].old_species, Species::Bulbasaur);
        assert_eq!(result.evolutions[0].new_species, Species::Ivysaur);
        assert_eq!(state.player.party[0].species, Species::Ivysaur);
    }

    #[test]
    fn no_evolution_if_fainted() {
        let mut mon = make_test_pokemon(Species::Bulbasaur, 16);
        mon.hp = 0;
        let player_party = vec![mon];
        let enemy_party = vec![make_test_pokemon(Species::Rattata, 5)];
        let mut state = new_battle_state(BattleType::Wild, player_party, enemy_party);

        let result = settle_battle(&mut state, BattleOutcome::Win, None, 0);

        assert!(result.evolutions.is_empty());
        assert_eq!(state.player.party[0].species, Species::Bulbasaur);
    }

    #[test]
    fn no_evolution_on_loss() {
        let player_party = vec![make_test_pokemon(Species::Bulbasaur, 16)];
        let enemy_party = vec![make_test_pokemon(Species::Rattata, 5)];
        let mut state = new_battle_state(BattleType::Wild, player_party, enemy_party);

        let result = settle_battle(&mut state, BattleOutcome::Loss, None, 0);

        assert!(result.evolutions.is_empty());
    }

    #[test]
    fn escaped_still_checks_evolution() {
        let player_party = vec![make_test_pokemon(Species::Bulbasaur, 16)];
        let enemy_party = vec![make_test_pokemon(Species::Rattata, 5)];
        let mut state = new_battle_state(BattleType::Wild, player_party, enemy_party);

        let result = settle_battle(&mut state, BattleOutcome::Escaped, None, 0);

        assert_eq!(result.evolutions.len(), 1);
    }

    #[test]
    fn trainer_battle_prize_with_payday() {
        let player_party = vec![make_test_pokemon(Species::Pikachu, 25)];
        let enemy_party = vec![make_test_pokemon(Species::Geodude, 14)];
        let mut state = new_battle_state(BattleType::Trainer, player_party, enemy_party);
        state.total_payday_money = 200;

        let result = settle_battle(
            &mut state,
            BattleOutcome::Win,
            Some(TrainerClass::Brock),
            5000,
        );

        let expected = 9900u32 * 14 + 200;
        assert_eq!(result.money_gained, expected);
    }
}
