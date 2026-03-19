use pokered_data::moves::MoveId;
use pokered_data::types::PokemonType;

use super::stat_stages::apply_stage;
use super::types::{apply_type_effectiveness, TypeMultiplier};

pub fn is_physical(move_type: PokemonType) -> bool {
    matches!(
        move_type,
        PokemonType::Normal
            | PokemonType::Fighting
            | PokemonType::Flying
            | PokemonType::Ground
            | PokemonType::Rock
            | PokemonType::Bug
            | PokemonType::Ghost
            | PokemonType::Poison
    )
}

pub fn is_special(move_type: PokemonType) -> bool {
    !is_physical(move_type)
}

pub fn is_high_crit_move(move_id: MoveId) -> bool {
    matches!(
        move_id,
        MoveId::KarateChop | MoveId::RazorLeaf | MoveId::Crabhammer | MoveId::Slash
    )
}

/// Gen 1 crit threshold. Focus Energy BUG preserved: divides by 4 instead of multiplying.
pub fn crit_chance(base_speed: u8, is_high_crit: bool, is_focus_energy: bool) -> u8 {
    let threshold = if is_high_crit {
        let val = (base_speed as u16) * 8;
        if is_focus_energy {
            val / 4
        } else {
            val
        }
    } else {
        let val = base_speed as u16 / 2;
        if is_focus_energy {
            val / 4
        } else {
            val
        }
    };
    threshold.min(255) as u8
}

pub struct DamageParams {
    pub attacker_level: u8,
    pub move_power: u8,
    pub move_type: PokemonType,
    pub move_id: MoveId,
    pub attack_stat: u16,
    pub defense_stat: u16,
    pub attack_stage: i8,
    pub defense_stage: i8,
    pub attacker_type1: PokemonType,
    pub attacker_type2: PokemonType,
    pub defender_type1: PokemonType,
    pub defender_type2: PokemonType,
    pub is_critical: bool,
    pub random_value: u8,
    pub has_reflect_or_light_screen: bool,
    pub is_explode_effect: bool,
}

pub struct DamageResult {
    pub damage: u16,
    pub type_effectiveness: TypeMultiplier,
    pub is_miss: bool,
}

/// Scale stats when either exceeds 255: both divided by 4, min 1.
/// ASM bug: defense could become 0 causing a freeze — we prevent that here.
fn scale_stats(attack: u16, defense: u16) -> (u16, u16) {
    if attack > 255 || defense > 255 {
        let a = (attack >> 2).max(1);
        let d = (defense >> 2).max(1);
        (a, d)
    } else {
        (attack, defense)
    }
}

pub fn calculate_damage(params: &DamageParams) -> DamageResult {
    if params.move_power == 0 {
        let eff = super::types::get_type_effectiveness(
            params.move_type,
            params.defender_type1,
            params.defender_type2,
        );
        return DamageResult {
            damage: 0,
            type_effectiveness: eff,
            is_miss: false,
        };
    }

    let level = if params.is_critical {
        (params.attacker_level as u32) * 2
    } else {
        params.attacker_level as u32
    };

    let (mut attack, mut defense) = if params.is_critical {
        (params.attack_stat as u32, params.defense_stat as u32)
    } else {
        (
            apply_stage(params.attack_stat, params.attack_stage) as u32,
            apply_stage(params.defense_stat, params.defense_stage) as u32,
        )
    };

    if params.has_reflect_or_light_screen {
        defense = (defense as u16).wrapping_mul(2) as u32;
    }

    if params.is_explode_effect {
        defense = (defense / 2).max(1);
    }

    let (attack_scaled, defense_scaled) = scale_stats(attack as u16, defense as u16);
    attack = attack_scaled as u32;
    defense = defense_scaled as u32;

    let defense = defense.max(1);

    let base = (2u32 * level / 5 + 2)
        .wrapping_mul(params.move_power as u32)
        .wrapping_mul(attack)
        / defense
        / 50;

    let base = base.min(997) + 2;

    let type_result = apply_type_effectiveness(
        base as u16,
        params.move_type,
        params.attacker_type1,
        params.attacker_type2,
        params.defender_type1,
        params.defender_type2,
    );

    if type_result.damage == 0 {
        return DamageResult {
            damage: 0,
            type_effectiveness: type_result.multiplier,
            is_miss: type_result.caused_miss,
        };
    }

    let random = params.random_value.max(1) as u32;
    let final_damage = (type_result.damage as u32) * random / 255;

    DamageResult {
        damage: final_damage.max(1) as u16,
        type_effectiveness: type_result.multiplier,
        is_miss: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_params() -> DamageParams {
        DamageParams {
            attacker_level: 50,
            move_power: 80,
            move_type: PokemonType::Normal,
            move_id: MoveId::Strength,
            attack_stat: 100,
            defense_stat: 100,
            attack_stage: 0,
            defense_stage: 0,
            attacker_type1: PokemonType::Normal,
            attacker_type2: PokemonType::Normal,
            defender_type1: PokemonType::Normal,
            defender_type2: PokemonType::Normal,
            is_critical: false,
            random_value: 255,
            has_reflect_or_light_screen: false,
            is_explode_effect: false,
        }
    }

    #[test]
    fn basic_damage_nonzero() {
        let result = calculate_damage(&make_params());
        assert!(result.damage > 0);
        assert!(!result.is_miss);
    }

    #[test]
    fn zero_power_zero_damage() {
        let mut p = make_params();
        p.move_power = 0;
        let result = calculate_damage(&p);
        assert_eq!(result.damage, 0);
    }

    #[test]
    fn immunity_zero_damage() {
        let mut p = make_params();
        p.move_type = PokemonType::Normal;
        p.attacker_type1 = PokemonType::Fire;
        p.attacker_type2 = PokemonType::Fire;
        p.defender_type1 = PokemonType::Ghost;
        p.defender_type2 = PokemonType::Ghost;
        let result = calculate_damage(&p);
        assert_eq!(result.damage, 0);
        assert!(result.is_miss);
    }

    #[test]
    fn stab_increases_damage() {
        let mut no_stab = make_params();
        no_stab.attacker_type1 = PokemonType::Fire;
        no_stab.attacker_type2 = PokemonType::Fire;
        let r1 = calculate_damage(&no_stab);

        let r2 = calculate_damage(&make_params());

        assert!(r2.damage > r1.damage, "STAB should increase damage");
    }

    #[test]
    fn super_effective_increases_damage() {
        let mut p = make_params();
        p.move_type = PokemonType::Water;
        p.attacker_type1 = PokemonType::Water;
        p.defender_type1 = PokemonType::Fire;
        p.defender_type2 = PokemonType::Fire;
        let se = calculate_damage(&p);

        let mut p2 = make_params();
        p2.move_type = PokemonType::Water;
        p2.attacker_type1 = PokemonType::Water;
        p2.defender_type1 = PokemonType::Normal;
        p2.defender_type2 = PokemonType::Normal;
        let neutral = calculate_damage(&p2);

        assert!(se.damage > neutral.damage);
    }

    #[test]
    fn critical_hit_increases_damage() {
        let normal = calculate_damage(&make_params());
        let mut p = make_params();
        p.is_critical = true;
        let crit = calculate_damage(&p);
        assert!(crit.damage > normal.damage);
    }

    #[test]
    fn random_value_affects_damage() {
        let mut low = make_params();
        low.random_value = 217;
        let mut high = make_params();
        high.random_value = 255;
        let r_low = calculate_damage(&low);
        let r_high = calculate_damage(&high);
        assert!(r_high.damage >= r_low.damage);
    }

    #[test]
    fn physical_special_split() {
        assert!(is_physical(PokemonType::Normal));
        assert!(is_physical(PokemonType::Fighting));
        assert!(is_physical(PokemonType::Rock));
        assert!(is_physical(PokemonType::Ground));
        assert!(is_physical(PokemonType::Ghost));
        assert!(is_physical(PokemonType::Poison));
        assert!(is_physical(PokemonType::Bug));
        assert!(is_physical(PokemonType::Flying));

        assert!(is_special(PokemonType::Fire));
        assert!(is_special(PokemonType::Water));
        assert!(is_special(PokemonType::Grass));
        assert!(is_special(PokemonType::Electric));
        assert!(is_special(PokemonType::Psychic));
        assert!(is_special(PokemonType::Ice));
        assert!(is_special(PokemonType::Dragon));
    }

    #[test]
    fn high_crit_moves() {
        assert!(is_high_crit_move(MoveId::KarateChop));
        assert!(is_high_crit_move(MoveId::RazorLeaf));
        assert!(is_high_crit_move(MoveId::Crabhammer));
        assert!(is_high_crit_move(MoveId::Slash));
        assert!(!is_high_crit_move(MoveId::Tackle));
        assert!(!is_high_crit_move(MoveId::Thunder));
    }

    #[test]
    fn crit_chance_normal_move() {
        let chance = crit_chance(100, false, false);
        assert_eq!(chance, 50);
    }

    #[test]
    fn crit_chance_high_crit_move() {
        let chance = crit_chance(100, true, false);
        assert_eq!(chance, 255);
    }

    #[test]
    fn crit_chance_with_focus_energy_bug() {
        let normal = crit_chance(100, false, false);
        let focus = crit_chance(100, false, true);
        assert!(
            focus < normal,
            "Gen 1 Focus Energy bug: should reduce crit rate"
        );
    }

    #[test]
    fn stat_scaling_both_high() {
        let (a, d) = scale_stats(512, 256);
        assert_eq!(a, 128);
        assert_eq!(d, 64);
    }

    #[test]
    fn stat_scaling_not_needed() {
        let (a, d) = scale_stats(200, 150);
        assert_eq!(a, 200);
        assert_eq!(d, 150);
    }

    #[test]
    fn stat_scaling_prevents_zero_attack() {
        let (a, _d) = scale_stats(3, 300);
        assert!(a >= 1, "scaled attack should be at least 1");
    }

    #[test]
    fn reflect_doubles_defense() {
        let normal = calculate_damage(&make_params());
        let mut p = make_params();
        p.has_reflect_or_light_screen = true;
        let reflected = calculate_damage(&p);
        assert!(reflected.damage < normal.damage);
    }

    #[test]
    fn explode_halves_defense() {
        let normal = calculate_damage(&make_params());
        let mut p = make_params();
        p.is_explode_effect = true;
        let exploded = calculate_damage(&p);
        assert!(exploded.damage > normal.damage);
    }

    #[test]
    fn miss_on_zero_damage_after_type_effectiveness() {
        let mut p = make_params();
        p.attacker_level = 2;
        p.move_power = 10;
        p.attack_stat = 10;
        p.defense_stat = 200;
        p.move_type = PokemonType::Fire;
        p.attacker_type1 = PokemonType::Normal;
        p.attacker_type2 = PokemonType::Normal;
        p.defender_type1 = PokemonType::Water;
        p.defender_type2 = PokemonType::Rock;
        let result = calculate_damage(&p);
        if result.damage == 0 {
            assert!(result.is_miss);
        }
    }
}
