use pokered_data::event_flags::EventFlag;
use pokered_data::maps::MapId;

use super::event_flags::EventFlags;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrainerHeader {
    pub event_flag: EventFlag,
    pub sight_range: u8,
    pub before_battle_text_id: u8,
    pub end_battle_text_id: u8,
    pub after_battle_text_id: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrainerBattleState {
    NotEngaged,
    Spotted,
    WalkingToPlayer,
    ShowBeforeBattleText,
    InBattle,
    ShowEndBattleText,
    Defeated,
}

#[derive(Debug, Clone)]
pub struct TrainerEncounter {
    pub map: MapId,
    pub trainer_index: u8,
    pub npc_index: u8,
    pub state: TrainerBattleState,
}

impl TrainerEncounter {
    pub fn new(map: MapId, trainer_index: u8, npc_index: u8) -> Self {
        Self {
            map,
            trainer_index,
            npc_index,
            state: TrainerBattleState::NotEngaged,
        }
    }
}

pub fn is_trainer_defeated(flags: &EventFlags, header: &TrainerHeader) -> bool {
    flags.check(header.event_flag)
}

pub fn mark_trainer_defeated(flags: &mut EventFlags, header: &TrainerHeader) {
    flags.set(header.event_flag);
}

pub fn can_trainer_see_player(
    trainer_x: u8,
    trainer_y: u8,
    trainer_facing_dx: i8,
    trainer_facing_dy: i8,
    player_x: u8,
    player_y: u8,
    sight_range: u8,
) -> bool {
    if trainer_facing_dx != 0 {
        if trainer_y != player_y {
            return false;
        }
        let dx = player_x as i16 - trainer_x as i16;
        if trainer_facing_dx > 0 {
            dx > 0 && dx <= sight_range as i16
        } else {
            dx < 0 && dx >= -(sight_range as i16)
        }
    } else if trainer_facing_dy != 0 {
        if trainer_x != player_x {
            return false;
        }
        let dy = player_y as i16 - trainer_y as i16;
        if trainer_facing_dy > 0 {
            dy > 0 && dy <= sight_range as i16
        } else {
            dy < 0 && dy >= -(sight_range as i16)
        }
    } else {
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TrainerPosition {
    pub x: u8,
    pub y: u8,
    pub facing_dx: i8,
    pub facing_dy: i8,
}

pub fn check_all_trainers(
    headers: &[TrainerHeader],
    flags: &EventFlags,
    trainer_positions: &[TrainerPosition],
    player_x: u8,
    player_y: u8,
) -> Option<usize> {
    for (i, header) in headers.iter().enumerate() {
        if is_trainer_defeated(flags, header) {
            continue;
        }
        if let Some(pos) = trainer_positions.get(i) {
            if can_trainer_see_player(
                pos.x,
                pos.y,
                pos.facing_dx,
                pos.facing_dy,
                player_x,
                player_y,
                header.sight_range,
            ) {
                return Some(i);
            }
        }
    }
    None
}

pub fn advance_trainer_battle(encounter: &mut TrainerEncounter) -> TrainerBattleState {
    encounter.state = match encounter.state {
        TrainerBattleState::NotEngaged => TrainerBattleState::Spotted,
        TrainerBattleState::Spotted => TrainerBattleState::WalkingToPlayer,
        TrainerBattleState::WalkingToPlayer => TrainerBattleState::ShowBeforeBattleText,
        TrainerBattleState::ShowBeforeBattleText => TrainerBattleState::InBattle,
        TrainerBattleState::InBattle => TrainerBattleState::ShowEndBattleText,
        TrainerBattleState::ShowEndBattleText => TrainerBattleState::Defeated,
        TrainerBattleState::Defeated => TrainerBattleState::Defeated,
    };
    encounter.state
}
