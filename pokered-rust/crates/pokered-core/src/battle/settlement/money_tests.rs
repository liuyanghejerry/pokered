#[cfg(test)]
mod tests {
    use pokered_data::trainer_data::TrainerClass;

    use crate::battle::settlement::money::*;

    #[test]
    fn prize_money_basic() {
        // Youngster base=1500, level 10 => 15000
        assert_eq!(calc_prize_money(TrainerClass::Youngster, 10), 15_000);
    }

    #[test]
    fn prize_money_gym_leader() {
        // Brock base=9900, level 14 => 138600
        assert_eq!(calc_prize_money(TrainerClass::Brock, 14), 138_600);
    }

    #[test]
    fn prize_money_lance_level_62() {
        // Lance base=9900, level 62 => 613800
        assert_eq!(calc_prize_money(TrainerClass::Lance, 62), 613_800);
    }

    #[test]
    fn prize_money_nobody_is_zero() {
        assert_eq!(calc_prize_money(TrainerClass::Nobody, 50), 0);
    }

    #[test]
    fn blackout_penalty_halves() {
        assert_eq!(calc_blackout_penalty(10_000), 5_000);
    }

    #[test]
    fn blackout_penalty_odd_rounds_down() {
        assert_eq!(calc_blackout_penalty(10_001), 5_000);
    }

    #[test]
    fn blackout_penalty_zero() {
        assert_eq!(calc_blackout_penalty(0), 0);
    }

    #[test]
    fn total_winnings_sum() {
        assert_eq!(calc_total_winnings(50_000, 1_000), 51_000);
    }

    #[test]
    fn total_winnings_capped_at_999999() {
        assert_eq!(calc_total_winnings(999_000, 2_000), 999_999);
    }

    #[test]
    fn total_winnings_no_overflow() {
        assert_eq!(calc_total_winnings(u32::MAX, 1), 999_999);
    }
}
