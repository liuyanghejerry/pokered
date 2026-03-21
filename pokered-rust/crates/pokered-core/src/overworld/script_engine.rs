use pokered_data::event_flags::EventFlag;
use pokered_data::maps::MapId;
use pokered_data::species::Species;

use super::Direction;

#[derive(Debug, Clone, PartialEq)]
pub enum ScriptAction {
    DisplayText {
        text_id: u8,
    },
    DisplayTextAtId {
        text_id: u8,
    },
    GiveItem {
        item_id: u8,
        quantity: u8,
    },
    GivePokemon {
        species: Species,
        level: u8,
    },
    TakeItem {
        item_id: u8,
        quantity: u8,
    },
    SetEventFlag {
        flag: EventFlag,
    },
    ResetEventFlag {
        flag: EventFlag,
    },
    ShowObject {
        object_index: u8,
    },
    HideObject {
        object_index: u8,
    },
    MoveNpc {
        npc_index: u8,
        movements: Vec<NpcMovementStep>,
    },
    FaceNpc {
        npc_index: u8,
        direction: Direction,
    },
    FacePlayer {
        direction: Direction,
    },
    PlayMusic {
        music_id: u8,
    },
    PlaySound {
        sound_id: u8,
    },
    StopMusic,
    FadeOutMusic,
    SetJoyIgnore {
        mask: u8,
    },
    ClearJoyIgnore,
    StartTrainerBattle {
        trainer_index: u8,
    },
    Delay {
        frames: u8,
    },
    WarpTo {
        map: MapId,
        warp_id: u8,
    },
    Heal,
    SetMapScript {
        script_index: u8,
    },
    EndScript,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpcMovementStep {
    Walk(Direction),
    Turn(Direction),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptPhase {
    Idle,
    Executing,
    WaitingForText,
    WaitingForMovement,
    WaitingForBattle,
    WaitingForDelay { frames_remaining: u8 },
}

#[derive(Debug, Clone)]
pub struct MapScriptState {
    pub map: MapId,
    pub current_script: u8,
    pub phase: ScriptPhase,
    pub action_queue: Vec<ScriptAction>,
    pub action_index: usize,
}

impl MapScriptState {
    pub fn new(map: MapId) -> Self {
        Self {
            map,
            current_script: 0,
            phase: ScriptPhase::Idle,
            action_queue: Vec::new(),
            action_index: 0,
        }
    }

    pub fn set_script(&mut self, index: u8) {
        self.current_script = index;
        self.phase = ScriptPhase::Idle;
        self.action_queue.clear();
        self.action_index = 0;
    }

    pub fn queue_actions(&mut self, actions: Vec<ScriptAction>) {
        self.action_queue = actions;
        self.action_index = 0;
        if !self.action_queue.is_empty() {
            self.phase = ScriptPhase::Executing;
        }
    }

    pub fn next_action(&mut self) -> Option<&ScriptAction> {
        if self.action_index < self.action_queue.len() {
            let action = &self.action_queue[self.action_index];
            self.action_index += 1;
            Some(action)
        } else {
            self.phase = ScriptPhase::Idle;
            None
        }
    }

    pub fn is_active(&self) -> bool {
        self.phase != ScriptPhase::Idle
    }

    pub fn signal_text_done(&mut self) {
        if self.phase == ScriptPhase::WaitingForText {
            self.phase = ScriptPhase::Executing;
        }
    }

    pub fn signal_movement_done(&mut self) {
        if self.phase == ScriptPhase::WaitingForMovement {
            self.phase = ScriptPhase::Executing;
        }
    }

    pub fn signal_battle_done(&mut self) {
        if self.phase == ScriptPhase::WaitingForBattle {
            self.phase = ScriptPhase::Executing;
        }
    }

    pub fn tick_delay(&mut self) {
        if let ScriptPhase::WaitingForDelay { frames_remaining } = &mut self.phase {
            if *frames_remaining > 0 {
                *frames_remaining -= 1;
            }
            if *frames_remaining == 0 {
                self.phase = ScriptPhase::Executing;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordEvent {
    pub x: u8,
    pub y: u8,
    pub script_index: u8,
}

impl CoordEvent {
    pub const fn new(x: u8, y: u8, script_index: u8) -> Self {
        Self { x, y, script_index }
    }

    pub fn matches(&self, player_x: u8, player_y: u8) -> bool {
        self.x == player_x && self.y == player_y
    }
}

pub fn check_coord_events(events: &[CoordEvent], player_x: u8, player_y: u8) -> Option<u8> {
    events
        .iter()
        .find(|e| e.matches(player_x, player_y))
        .map(|e| e.script_index)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScriptResult {
    Continue,
    WaitForText,
    WaitForMovement,
    WaitForBattle,
    WaitForDelay { frames: u8 },
    ScriptChanged { new_index: u8 },
    ScriptEnded,
}

pub fn execute_next_action(state: &mut MapScriptState) -> ScriptResult {
    let action = match state.next_action() {
        Some(a) => a.clone(),
        None => return ScriptResult::ScriptEnded,
    };

    match action {
        ScriptAction::DisplayText { .. } | ScriptAction::DisplayTextAtId { .. } => {
            state.phase = ScriptPhase::WaitingForText;
            ScriptResult::WaitForText
        }
        ScriptAction::MoveNpc { .. } => {
            state.phase = ScriptPhase::WaitingForMovement;
            ScriptResult::WaitForMovement
        }
        ScriptAction::StartTrainerBattle { .. } => {
            state.phase = ScriptPhase::WaitingForBattle;
            ScriptResult::WaitForBattle
        }
        ScriptAction::Delay { frames } => {
            state.phase = ScriptPhase::WaitingForDelay {
                frames_remaining: frames,
            };
            ScriptResult::WaitForDelay { frames }
        }
        ScriptAction::SetMapScript { script_index } => {
            state.set_script(script_index);
            ScriptResult::ScriptChanged {
                new_index: script_index,
            }
        }
        ScriptAction::EndScript => {
            state.phase = ScriptPhase::Idle;
            ScriptResult::ScriptEnded
        }
        _ => ScriptResult::Continue,
    }
}
