pub mod accuracy;
pub mod capture;
pub mod damage;
pub mod effects;
pub mod escape;
pub mod experience;
pub mod menu;
pub mod move_execution;
pub mod residual;
pub mod settlement;
pub mod stat_stages;
pub mod state;
pub mod status_checks;
pub mod trainer_ai;
pub mod turn;
pub mod turn_order;
pub mod types;
pub mod wild;

#[cfg(test)]
mod menu_tests;

// ── BattleScreen (frame-loop adapter) ─────────────────────────────

use crate::game_state::{GameScreen, ScreenAction};
use crate::main_menu::MenuInput;
use effects::EffectRandoms;
use escape::{try_run_from_battle, RunResult};
use menu::{
    BattleMenuAction, BattleMenuInput, BattleMenuState, MoveMenuResult, MoveMenuState, MoveSlot,
};
use move_execution::MoveRandoms;
use pokered_data::move_data::MoveData;
use pokered_data::moves::MoveId;
use pokered_data::species::Species;
use pokered_data::trainer_data::TrainerClass;
use state::{BattleState, BattleType, Side, StatusCondition};
use trainer_ai::move_choice::{choose_moves, MoveChoiceResult};
use trainer_ai::move_choice_layers;
use turn::{execute_turn, TurnRandoms};
use types::TypeMultiplier;

/// High-level battle phase (frame-loop granularity).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BattlePhase {
    /// Intro animation / "Wild X appeared!" / "Trainer wants to fight!"
    Intro { wait_frames: u16 },
    /// Player picks FIGHT / BAG / POKéMON / RUN.
    PlayerMenu,
    /// Player picks a move from the move list.
    MoveSelect,
    /// Displaying sequential text messages (turn results, status, etc.).
    /// Advances on A press. After all messages → next phase.
    ShowingText {
        messages: Vec<String>,
        current: usize,
        /// Frames to auto-wait before accepting input (brief pause).
        wait_frames: u16,
        /// Phase to transition to after all messages are shown.
        next_phase: Box<BattlePhase>,
    },
    /// Player chooses which party member to switch to.
    PartySelect,
    /// Enemy trainer sends out next Pokémon after one faints.
    EnemySendingNext { wait_frames: u16 },
    /// Player must choose replacement after their Pokémon faints.
    PlayerFaintSwitch,
    /// Battle is over — display final message then exit.
    BattleOver { won: bool, wait_frames: u16 },
}

/// Input forwarded to the battle screen each frame.
#[derive(Debug, Clone, Copy)]
pub struct BattleInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
}

impl BattleInput {
    pub fn none() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            a: false,
            b: false,
        }
    }
}

use status_checks::CannotMoveReason;

/// Convert a PascalCase MoveId Debug name to game-style uppercase with spaces.
/// e.g. "QuickAttack" → "QUICK ATTACK", "Thunderbolt" → "THUNDERBOLT",
///      "HiJumpKick" → "HI JUMP KICK", "ThunderWave" → "THUNDER WAVE"
fn move_display_name(move_id: MoveId) -> String {
    let raw = format!("{:?}", move_id);
    let mut result = String::with_capacity(raw.len() + 4);
    for (i, c) in raw.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            // Insert space before uppercase letter unless previous char was also uppercase
            let prev = raw.as_bytes()[i - 1] as char;
            if prev.is_lowercase() {
                result.push(' ');
            }
        }
        result.push(c);
    }
    result.to_uppercase()
}

fn format_cannot_move(reason: &CannotMoveReason) -> &'static str {
    match reason {
        CannotMoveReason::Asleep => "is fast asleep!",
        CannotMoveReason::WokeUpButLostTurn => "woke up!",
        CannotMoveReason::Frozen => "is frozen solid!",
        CannotMoveReason::TrappedByEnemy => "can't move!",
        CannotMoveReason::Flinched => "flinched!",
        CannotMoveReason::MustRecharge => "must recharge!",
        CannotMoveReason::ConfusedSelfHit => "hurt itself in confusion!",
        CannotMoveReason::MoveDisabled => "is disabled!",
        CannotMoveReason::FullyParalyzed => "is fully paralyzed!",
    }
}

pub struct BattleScreen {
    pub phase: BattlePhase,
    pub battle_menu: BattleMenuState,
    pub is_wild: bool,
    /// Trainer class for AI move selection (None for wild battles).
    pub trainer_class: Option<TrainerClass>,

    // Display fields (synced from battle_state after every action)
    pub enemy_species: Species,
    pub enemy_level: u8,
    pub enemy_hp: u16,
    pub enemy_max_hp: u16,
    pub enemy_status: StatusCondition,
    pub player_species: Species,
    pub player_level: u8,
    pub player_hp: u16,
    pub player_max_hp: u16,
    pub player_status: StatusCondition,
    pub player_party_size: usize,
    pub enemy_party_size: usize,

    // Real battle engine state
    pub battle_state: Option<BattleState>,
    pub move_menu: Option<MoveMenuState>,
    pub current_message: Option<String>,
    pub party_cursor: usize,
}

impl BattleScreen {
    pub fn new(is_wild: bool) -> Self {
        Self {
            phase: BattlePhase::Intro { wait_frames: 90 },
            battle_menu: BattleMenuState::new(),
            is_wild,
            trainer_class: None,
            enemy_species: Species::Pikachu,
            enemy_level: 25,
            enemy_hp: 55,
            enemy_max_hp: 55,
            enemy_status: StatusCondition::None,
            player_species: Species::Charmander,
            player_level: 5,
            player_hp: 19,
            player_max_hp: 20,
            player_status: StatusCondition::None,
            player_party_size: 1,
            enemy_party_size: 1,
            battle_state: None,
            move_menu: None,
            current_message: None,
            party_cursor: 0,
        }
    }

    pub fn from_parties(
        is_wild: bool,
        player_party: &[state::Pokemon],
        enemy_party: &[state::Pokemon],
        trainer_class: Option<TrainerClass>,
    ) -> Self {
        let player = &player_party[0];
        let enemy = &enemy_party[0];
        let battle_type = if is_wild {
            BattleType::Wild
        } else {
            BattleType::Trainer
        };
        let bs = state::new_battle_state(battle_type, player_party.to_vec(), enemy_party.to_vec());
        Self {
            phase: BattlePhase::Intro { wait_frames: 90 },
            battle_menu: BattleMenuState::new(),
            is_wild,
            trainer_class,
            enemy_species: enemy.species,
            enemy_level: enemy.level,
            enemy_hp: enemy.hp,
            enemy_max_hp: enemy.max_hp,
            enemy_status: enemy.status,
            player_species: player.species,
            player_level: player.level,
            player_hp: player.hp,
            player_max_hp: player.max_hp,
            player_status: player.status,
            player_party_size: player_party.len(),
            enemy_party_size: enemy_party.len(),
            battle_state: Some(bs),
            move_menu: None,
            current_message: None,
            party_cursor: 0,
        }
    }

    fn sync_display_from_state(&mut self) {
        if let Some(ref bs) = self.battle_state {
            let p = bs.player.active_mon();
            self.player_species = p.species;
            self.player_level = p.level;
            self.player_hp = p.hp;
            self.player_max_hp = p.max_hp;
            self.player_status = p.status;
            self.player_party_size = bs.player.party.len();

            let e = bs.enemy.active_mon();
            self.enemy_species = e.species;
            self.enemy_level = e.level;
            self.enemy_hp = e.hp;
            self.enemy_max_hp = e.max_hp;
            self.enemy_status = e.status;
            self.enemy_party_size = bs.enemy.party.len();
        }
    }

    fn generate_move_randoms() -> MoveRandoms {
        MoveRandoms {
            confusion_roll: rand::random(),
            paralysis_roll: rand::random(),
            crit_roll: rand::random(),
            accuracy_roll: rand::random(),
            damage_roll: rand::random(),
            effect_randoms: EffectRandoms {
                side_effect_roll: rand::random(),
                duration_roll: rand::random(),
                multi_hit_roll: rand::random(),
            },
        }
    }

    fn generate_turn_randoms() -> TurnRandoms {
        TurnRandoms {
            order_random: rand::random(),
            first_mover: Self::generate_move_randoms(),
            second_mover: Self::generate_move_randoms(),
        }
    }

    fn pick_enemy_move(bs: &BattleState, trainer_class: Option<TrainerClass>) -> (MoveId, u8) {
        let mon = bs.enemy.active_mon();
        let available: Vec<(MoveId, u8)> = mon
            .moves
            .iter()
            .enumerate()
            .filter(|(i, m)| **m != MoveId::None && mon.pp[*i] > 0)
            .map(|(i, m)| (*m, i as u8))
            .collect();
        if available.is_empty() {
            return (MoveId::Struggle, 0);
        }

        if let Some(tc) = trainer_class {
            let layers = move_choice_layers(tc);
            if !layers.is_empty() {
                let result = choose_moves(layers, &bs.enemy, &bs.player, 0);
                if let Some(slot) = result.pick_move(rand::random::<u8>()) {
                    let move_id = mon.moves[slot];
                    if move_id != MoveId::None && mon.pp[slot] > 0 {
                        return (move_id, slot as u8);
                    }
                }
            }
        }

        let idx: usize = rand::random::<usize>() % available.len();
        available[idx]
    }

    fn build_move_menu_from_state(bs: &BattleState) -> MoveMenuState {
        let mon = bs.player.active_mon();
        let slots: Vec<MoveSlot> = mon
            .moves
            .iter()
            .enumerate()
            .filter(|(_, m)| **m != MoveId::None)
            .map(|(i, m)| {
                let max_pp = MoveData::get(*m).map_or(0, |d| d.pp);
                MoveSlot {
                    move_id: *m,
                    current_pp: mon.pp[i],
                    max_pp,
                    is_disabled: bs.player.disabled_move > 0
                        && bs.player.disabled_move == (i as u8 + 1),
                }
            })
            .collect();
        MoveMenuState::new(slots)
    }

    fn format_move_outcome(
        side_name: &str,
        move_name: &str,
        outcome: &move_execution::MoveOutcome,
        _target_name: &str,
    ) -> Vec<String> {
        let mut msgs = vec![format!("{} used {}!", side_name, move_name)];
        match outcome {
            move_execution::MoveOutcome::Success {
                is_critical,
                type_effectiveness,
                ..
            } => {
                if *is_critical {
                    msgs.push("Critical hit!".to_string());
                }
                if type_effectiveness.is_super_effective() {
                    msgs.push("It's super effective!".to_string());
                } else if type_effectiveness.is_not_very_effective() {
                    msgs.push("It's not very effective...".to_string());
                } else if type_effectiveness.is_no_effect() {
                    msgs.push("It doesn't affect the enemy!".to_string());
                }
            }

            move_execution::MoveOutcome::Missed => {
                msgs.push(format!("{}'s attack missed!", side_name));
            }
            move_execution::MoveOutcome::CannotMove(reason) => {
                msgs.clear();
                msgs.push(format!("{} {}", side_name, format_cannot_move(reason)));
            }
            move_execution::MoveOutcome::NoDamageMove { .. } => {}
        }
        msgs
    }

    pub fn update_frame(&mut self, input: BattleInput) -> ScreenAction {
        match self.phase.clone() {
            BattlePhase::Intro { mut wait_frames } => {
                if input.a || input.b {
                    wait_frames = 0;
                }
                if wait_frames > 0 {
                    self.phase = BattlePhase::Intro {
                        wait_frames: wait_frames - 1,
                    };
                    return ScreenAction::Continue;
                }
                self.battle_menu = BattleMenuState::new();
                self.phase = BattlePhase::PlayerMenu;
                ScreenAction::Continue
            }
            BattlePhase::PlayerMenu => {
                let menu_input = BattleMenuInput {
                    up: input.up,
                    down: input.down,
                    left: input.left,
                    right: input.right,
                    a: input.a,
                    b: input.b,
                };
                if let Some(action) = self.battle_menu.update_frame(menu_input) {
                    match action {
                        BattleMenuAction::Fight => {
                            if let Some(ref bs) = self.battle_state {
                                self.move_menu = Some(Self::build_move_menu_from_state(bs));
                            }
                            self.phase = BattlePhase::MoveSelect;
                        }
                        BattleMenuAction::Run => {
                            self.handle_run();
                        }
                        BattleMenuAction::Pokemon => {
                            if let Some(ref bs) = self.battle_state {
                                if bs.player.party.len() > 1 {
                                    self.party_cursor = 0;
                                    self.phase = BattlePhase::PartySelect;
                                } else {
                                    self.show_text_then(
                                        vec!["No other POKeMON!".to_string()],
                                        BattlePhase::PlayerMenu,
                                    );
                                }
                            }
                        }
                        BattleMenuAction::Bag => {
                            self.show_text_then(
                                vec!["No items!".to_string()],
                                BattlePhase::PlayerMenu,
                            );
                        }
                    }
                }
                ScreenAction::Continue
            }
            BattlePhase::MoveSelect => {
                let menu_input = MenuInput {
                    up: input.up,
                    down: input.down,
                    a: input.a,
                    b: input.b,
                };
                if let Some(ref mut mm) = self.move_menu {
                    if let Some(result) = mm.update_frame(menu_input) {
                        match result {
                            MoveMenuResult::Selected(idx) => {
                                self.execute_turn_with_move(idx);
                            }
                            MoveMenuResult::Cancelled => {
                                self.move_menu = None;
                                self.battle_menu = BattleMenuState::new();
                                self.phase = BattlePhase::PlayerMenu;
                            }
                            MoveMenuResult::NoPP(_) => {
                                self.current_message = Some("No PP left!".to_string());
                            }
                            MoveMenuResult::Disabled(_) => {
                                self.current_message = Some("Move is disabled!".to_string());
                            }
                        }
                    }
                }
                ScreenAction::Continue
            }
            BattlePhase::ShowingText {
                messages,
                current,
                wait_frames,
                next_phase,
            } => {
                if wait_frames > 0 {
                    self.phase = BattlePhase::ShowingText {
                        messages: messages.clone(),
                        current,
                        wait_frames: wait_frames - 1,
                        next_phase,
                    };
                    return ScreenAction::Continue;
                }
                self.current_message = Some(messages[current].clone());
                if input.a || input.b {
                    let next_idx = current + 1;
                    if next_idx >= messages.len() {
                        self.current_message = None;
                        self.phase = *next_phase;
                        self.post_text_transition();
                    } else {
                        self.phase = BattlePhase::ShowingText {
                            messages,
                            current: next_idx,
                            wait_frames: 0,
                            next_phase,
                        };
                    }
                }
                ScreenAction::Continue
            }
            BattlePhase::PartySelect => {
                if input.b {
                    self.battle_menu = BattleMenuState::new();
                    self.phase = BattlePhase::PlayerMenu;
                    return ScreenAction::Continue;
                }
                if let Some(ref bs) = self.battle_state {
                    let party_len = bs.player.party.len();
                    if input.down {
                        self.party_cursor = (self.party_cursor + 1) % party_len;
                    } else if input.up {
                        self.party_cursor = if self.party_cursor == 0 {
                            party_len - 1
                        } else {
                            self.party_cursor - 1
                        };
                    }
                    if input.a {
                        let chosen = self.party_cursor;
                        let active = bs.player.active_pokemon_index;
                        if chosen == active {
                            self.current_message = Some("Already out!".to_string());
                        } else if bs.player.party[chosen].hp == 0 {
                            self.current_message = Some("No energy left!".to_string());
                        } else {
                            self.switch_player_pokemon(chosen);
                        }
                    }
                }
                ScreenAction::Continue
            }
            BattlePhase::EnemySendingNext { wait_frames } => {
                if wait_frames > 0 {
                    self.phase = BattlePhase::EnemySendingNext {
                        wait_frames: wait_frames - 1,
                    };
                    return ScreenAction::Continue;
                }
                self.sync_display_from_state();
                self.battle_menu = BattleMenuState::new();
                self.phase = BattlePhase::PlayerMenu;
                ScreenAction::Continue
            }
            BattlePhase::PlayerFaintSwitch => {
                if let Some(ref bs) = self.battle_state {
                    let party_len = bs.player.party.len();
                    if input.down {
                        self.party_cursor = (self.party_cursor + 1) % party_len;
                    } else if input.up {
                        self.party_cursor = if self.party_cursor == 0 {
                            party_len - 1
                        } else {
                            self.party_cursor - 1
                        };
                    }
                    if input.a {
                        let chosen = self.party_cursor;
                        if bs.player.party[chosen].hp == 0 {
                            self.current_message = Some("No energy left!".to_string());
                        } else {
                            self.force_switch_player(chosen);
                        }
                    }
                }
                ScreenAction::Continue
            }
            BattlePhase::BattleOver {
                won,
                mut wait_frames,
            } => {
                if input.a || input.b {
                    wait_frames = 0;
                }
                if wait_frames > 0 {
                    self.phase = BattlePhase::BattleOver {
                        won,
                        wait_frames: wait_frames - 1,
                    };
                    return ScreenAction::Continue;
                }
                ScreenAction::Transition(GameScreen::Overworld)
            }
        }
    }

    fn show_text_then(&mut self, messages: Vec<String>, next: BattlePhase) {
        if messages.is_empty() {
            self.phase = next;
            return;
        }
        self.current_message = Some(messages[0].clone());
        self.phase = BattlePhase::ShowingText {
            messages,
            current: 0,
            wait_frames: 10,
            next_phase: Box::new(next),
        };
    }

    fn handle_run(&mut self) {
        if let Some(ref mut bs) = self.battle_state {
            let result = try_run_from_battle(bs, rand::random());
            match result {
                RunResult::Escaped => {
                    self.show_text_then(
                        vec!["Got away safely!".to_string()],
                        BattlePhase::BattleOver {
                            won: false,
                            wait_frames: 30,
                        },
                    );
                }
                RunResult::CannotRun => {
                    self.show_text_then(
                        vec!["No! There's no running from a trainer battle!".to_string()],
                        BattlePhase::PlayerMenu,
                    );
                }
                RunResult::FailedToEscape => {
                    self.execute_enemy_free_turn();
                }
            }
        } else {
            self.phase = BattlePhase::BattleOver {
                won: false,
                wait_frames: 30,
            };
        }
    }

    fn execute_enemy_free_turn(&mut self) {
        if let Some(ref mut bs) = self.battle_state {
            let (enemy_move_id, enemy_move_idx) = Self::pick_enemy_move(bs, self.trainer_class);
            let enemy_move = match MoveData::get(enemy_move_id) {
                Some(m) => m,
                None => return,
            };
            bs.enemy.selected_move = enemy_move_id;
            bs.enemy.selected_move_index = enemy_move_idx;
            bs.whose_turn = Side::Enemy;
            let randoms = Self::generate_move_randoms();
            let outcome = move_execution::execute_move(bs, enemy_move, &randoms);

            let enemy_name = format!("{}", bs.enemy.active_mon().species).to_uppercase();
            let move_name = move_display_name(enemy_move_id);

            let mut msgs = vec!["Can't escape!".to_string()];
            msgs.extend(Self::format_move_outcome(
                &format!("Enemy {}", enemy_name),
                &move_name,
                &outcome,
                &format!("{}", bs.player.active_mon().species).to_uppercase(),
            ));

            self.sync_display_from_state();
            let next = self.check_faint_after_turn();
            self.show_text_then(msgs, next);
        }
    }

    fn execute_turn_with_move(&mut self, move_index: usize) {
        let (player_move_id, enemy_move_id, enemy_move_idx, player_name, enemy_name);

        if let Some(ref bs) = self.battle_state {
            let mon = bs.player.active_mon();
            player_move_id = mon.moves[move_index];
            let (eid, eidx) = Self::pick_enemy_move(bs, self.trainer_class);
            enemy_move_id = eid;
            enemy_move_idx = eidx;
            player_name = format!("{}", mon.species).to_uppercase();
            enemy_name = format!("{}", bs.enemy.active_mon().species).to_uppercase();
        } else {
            return;
        }

        let player_move = match MoveData::get(player_move_id) {
            Some(m) => m,
            None => return,
        };
        let enemy_move = match MoveData::get(enemy_move_id) {
            Some(m) => m,
            None => return,
        };

        let randoms = Self::generate_turn_randoms();

        if let Some(ref mut bs) = self.battle_state {
            bs.player.selected_move = player_move_id;
            bs.player.selected_move_index = move_index as u8;
            bs.enemy.selected_move = enemy_move_id;
            bs.enemy.selected_move_index = enemy_move_idx;

            let result = execute_turn(bs, player_move, enemy_move, &randoms);

            let mut msgs = Vec::new();

            let (first_name, first_move_name, second_name, second_move_name) =
                if result.first == Side::Player {
                    (
                        player_name.clone(),
                        move_display_name(player_move_id),
                        format!("Enemy {}", enemy_name),
                        move_display_name(enemy_move_id),
                    )
                } else {
                    (
                        format!("Enemy {}", enemy_name),
                        move_display_name(enemy_move_id),
                        player_name.clone(),
                        move_display_name(player_move_id),
                    )
                };

            let first_target = if result.first == Side::Player {
                format!("Enemy {}", enemy_name)
            } else {
                player_name.clone()
            };
            msgs.extend(Self::format_move_outcome(
                &first_name,
                &first_move_name,
                &result.first_outcome,
                &first_target,
            ));

            if let Some(side) = result.first_fainted {
                let fainted_name = if side == Side::Player {
                    player_name.clone()
                } else {
                    format!("Enemy {}", enemy_name)
                };
                msgs.push(format!("{} fainted!", fainted_name));
            }

            if let Some(ref second_outcome) = result.second_outcome {
                let second_target = if result.first == Side::Player {
                    player_name.clone()
                } else {
                    format!("Enemy {}", enemy_name)
                };
                msgs.extend(Self::format_move_outcome(
                    &second_name,
                    &second_move_name,
                    second_outcome,
                    &second_target,
                ));

                if let Some(side) = result.second_fainted {
                    let fainted_name = if side == Side::Player {
                        player_name.clone()
                    } else {
                        format!("Enemy {}", enemy_name)
                    };
                    msgs.push(format!("{} fainted!", fainted_name));
                }
            }

            self.move_menu = None;
            self.sync_display_from_state();
            let next = self.check_faint_after_turn();
            self.show_text_then(msgs, next);
        }
    }

    fn check_faint_after_turn(&self) -> BattlePhase {
        if let Some(ref bs) = self.battle_state {
            let player_fainted = bs.player.active_mon().hp == 0;
            let enemy_fainted = bs.enemy.active_mon().hp == 0;

            if enemy_fainted {
                let alive_enemies = bs
                    .enemy
                    .party
                    .iter()
                    .any(|p| p.hp > 0 && !std::ptr::eq(p, bs.enemy.active_mon()));
                if !alive_enemies {
                    return BattlePhase::BattleOver {
                        won: true,
                        wait_frames: 60,
                    };
                }
                return BattlePhase::EnemySendingNext { wait_frames: 30 };
            }

            if player_fainted {
                let alive_player = bs
                    .player
                    .party
                    .iter()
                    .any(|p| p.hp > 0 && !std::ptr::eq(p, bs.player.active_mon()));
                if !alive_player {
                    return BattlePhase::BattleOver {
                        won: false,
                        wait_frames: 60,
                    };
                }
                return BattlePhase::PlayerFaintSwitch;
            }
        }
        BattlePhase::PlayerMenu
    }

    fn post_text_transition(&mut self) {
        match &self.phase {
            BattlePhase::PlayerMenu => {
                self.battle_menu = BattleMenuState::new();
            }
            BattlePhase::EnemySendingNext { .. } => {
                self.send_next_enemy();
            }
            _ => {}
        }
    }

    fn send_next_enemy(&mut self) {
        if let Some(ref mut bs) = self.battle_state {
            let next_idx = bs.enemy.party.iter().position(|p| p.hp > 0);
            if let Some(idx) = next_idx {
                bs.enemy.active_pokemon_index = idx;
                bs.enemy.reset_volatile_status();
                bs.enemy.refresh_unmodified_stats();
                self.sync_display_from_state();
            }
        }
    }

    fn switch_player_pokemon(&mut self, new_index: usize) {
        if let Some(ref mut bs) = self.battle_state {
            let old_name = format!("{}", bs.player.active_mon().species).to_uppercase();
            bs.player.active_pokemon_index = new_index;
            bs.player.reset_volatile_status();
            bs.player.refresh_unmodified_stats();
            let new_name = format!("{}", bs.player.active_mon().species).to_uppercase();

            self.sync_display_from_state();

            let msgs = vec![
                format!("{}, come back!", old_name),
                format!("Go! {}!", new_name),
            ];

            self.execute_enemy_free_turn_after_switch(msgs);
        }
    }

    fn execute_enemy_free_turn_after_switch(&mut self, mut msgs: Vec<String>) {
        if let Some(ref mut bs) = self.battle_state {
            let (enemy_move_id, enemy_move_idx) = Self::pick_enemy_move(bs, self.trainer_class);
            if let Some(enemy_move) = MoveData::get(enemy_move_id) {
                bs.enemy.selected_move = enemy_move_id;
                bs.enemy.selected_move_index = enemy_move_idx;
                bs.whose_turn = Side::Enemy;
                let randoms = Self::generate_move_randoms();
                let outcome = move_execution::execute_move(bs, enemy_move, &randoms);

                let enemy_name = format!("{}", bs.enemy.active_mon().species).to_uppercase();
                let move_name = move_display_name(enemy_move_id);
                let player_name = format!("{}", bs.player.active_mon().species).to_uppercase();

                msgs.extend(Self::format_move_outcome(
                    &format!("Enemy {}", enemy_name),
                    &move_name,
                    &outcome,
                    &player_name,
                ));
            }

            self.sync_display_from_state();
            let next = self.check_faint_after_turn();
            self.show_text_then(msgs, next);
        }
    }

    fn force_switch_player(&mut self, new_index: usize) {
        if let Some(ref mut bs) = self.battle_state {
            bs.player.active_pokemon_index = new_index;
            bs.player.reset_volatile_status();
            bs.player.refresh_unmodified_stats();
            let new_name = format!("{}", bs.player.active_mon().species).to_uppercase();

            self.sync_display_from_state();
            self.show_text_then(vec![format!("Go! {}!", new_name)], BattlePhase::PlayerMenu);
        }
    }
}
