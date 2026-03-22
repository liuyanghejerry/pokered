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
use menu::{BattleMenuAction, BattleMenuInput, BattleMenuState};
use pokered_data::species::Species;

/// High-level battle phase (frame-loop granularity).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BattlePhase {
    /// Intro animation / "Wild X appeared!" / "Trainer wants to fight!"
    Intro { wait_frames: u16 },
    /// Player picks FIGHT / BAG / POKéMON / RUN.
    PlayerMenu,
    /// Player picks a move from the move list.
    MoveSelect,
    /// Turn is being executed (animation/text playback).
    TurnExecution { wait_frames: u16 },
    /// A Pokémon fainted — show message, check for next.
    FaintCheck { wait_frames: u16 },
    /// Battle ended — victory / loss / ran away.
    Finished { won: bool, wait_frames: u16 },
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

/// Top-level battle screen adapter that wraps the battle subsystems
/// into a frame-by-frame update loop compatible with ScreenAction.
pub struct BattleScreen {
    pub phase: BattlePhase,
    pub battle_menu: BattleMenuState,
    pub is_wild: bool,
    pub enemy_species: Species,
    pub enemy_level: u8,
    pub enemy_hp: u16,
    pub enemy_max_hp: u16,
    pub player_species: Species,
    pub player_level: u8,
    pub player_hp: u16,
    pub player_max_hp: u16,
}

impl BattleScreen {
    pub fn new(is_wild: bool) -> Self {
        Self {
            phase: BattlePhase::Intro { wait_frames: 90 },
            battle_menu: BattleMenuState::new(),
            is_wild,
            enemy_species: Species::Pikachu,
            enemy_level: 25,
            enemy_hp: 55,
            enemy_max_hp: 55,
            player_species: Species::Charmander,
            player_level: 5,
            player_hp: 19,
            player_max_hp: 20,
        }
    }

    pub fn update_frame(&mut self, input: BattleInput) -> ScreenAction {
        match &mut self.phase {
            BattlePhase::Intro { wait_frames } => {
                if input.a || input.b {
                    *wait_frames = 0;
                }
                if *wait_frames > 0 {
                    *wait_frames -= 1;
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
                            // Simplified: go straight to turn execution
                            self.phase = BattlePhase::TurnExecution { wait_frames: 60 };
                        }
                        BattleMenuAction::Run => {
                            if self.is_wild {
                                self.phase = BattlePhase::Finished {
                                    won: false,
                                    wait_frames: 30,
                                };
                            }
                            // Can't run from trainer battles — stay in menu
                        }
                        BattleMenuAction::Bag | BattleMenuAction::Pokemon => {
                            // TODO: open sub-menus
                        }
                    }
                }
                ScreenAction::Continue
            }
            BattlePhase::MoveSelect => {
                // TODO: integrate MoveMenuState
                self.phase = BattlePhase::TurnExecution { wait_frames: 60 };
                ScreenAction::Continue
            }
            BattlePhase::TurnExecution { wait_frames } => {
                if *wait_frames > 0 {
                    *wait_frames -= 1;
                    return ScreenAction::Continue;
                }
                self.phase = BattlePhase::FaintCheck { wait_frames: 30 };
                ScreenAction::Continue
            }
            BattlePhase::FaintCheck { wait_frames } => {
                if *wait_frames > 0 {
                    *wait_frames -= 1;
                    return ScreenAction::Continue;
                }
                // Simplified: after faint check, go back to menu or end
                self.battle_menu = BattleMenuState::new();
                self.phase = BattlePhase::PlayerMenu;
                ScreenAction::Continue
            }
            BattlePhase::Finished { wait_frames, .. } => {
                if *wait_frames > 0 {
                    *wait_frames -= 1;
                    return ScreenAction::Continue;
                }
                ScreenAction::Transition(GameScreen::Overworld)
            }
        }
    }
}
